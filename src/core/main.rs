// Main Event Dispatcher
// Master of the Universe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::core::callbacks::*;
use crate::core::point::*;
use crate::core::widget_store::*;
use crate::widget::widget::*;

use glfw_window::GlfwWindow;

use piston_window::*;
use opengl_graphics::{Texture, GlGraphics};
use graphics::math::scale;
use gl::types::GLuint;

/// This structure is returned when instantiating a new Pushrod main object.
/// It stores the OpenGL configuration that is desired for drawing, a list of references
/// to a managed set of `PushrodWindow` objects, registered `EventListener`s, and
/// `PushrodEvent` objects that are pending dispatch.
///
/// The objects contained within this structure are used by the `Pushrod` run loop, and
/// are not intended to be modified except through methods in the `Pushrod` impl.
pub struct Pushrod {
    window: PistonWindow<GlfwWindow>,
    pub widget_store: RefCell<WidgetStore>,
    texture_buffer: Box<Vec<u8>>,
    pub texture: Texture,
    pub fbo: GLuint,
}

/// Pushrod implementation.  Create a `Pushrod::new( OpenGL )` object to create a new
/// main loop.  Only one of these should be set for the entire application runtime.
///
/// Example usage:
/// IN PROGRESS
impl Pushrod {
    /// Pushrod Object Constructor.  Takes in a single OpenGL configuration type.
    pub fn new(window: PistonWindow<GlfwWindow>) -> Self {
        Self {
            window,
            widget_store: RefCell::new(WidgetStore::new()),
            texture_buffer: Box::new(vec![0u8; 1]),
            texture: Texture::empty(&TextureSettings::new()).unwrap(),
            fbo: 0,
        }
    }

    /// Retrieves the window `GfxFactory` factory object for graphics textures.
    pub fn get_factory(&mut self) -> &mut GfxFactory {
        &mut self.window.factory
    }

    /// Helper method that adds a `Widget` to the `WidgetStore`, returning the ID of the `Widget`
    /// after it has been added.
    pub fn add_widget(&mut self, name: &str, widget: Box<dyn Widget>) -> i32 {
        self.widget_store.borrow_mut().add_widget(name, widget)
    }

    /// Helper method that adds a `Widget` to the `WidgetStore`, specifying the `parent_id` as the
    /// parent of which to add this object to.  Returns the new ID of the `Widget` after it has
    /// been added.
    pub fn add_widget_to_parent(
        &mut self,
        name: &str,
        widget: Box<dyn Widget>,
        parent_id: i32,
    ) -> i32 {
        self.widget_store
            .borrow_mut()
            .add_widget_to_parent(name, widget, parent_id)
    }

    fn handle_draw(&mut self, event: &Event) {
        let widgets = &mut self.widget_store.borrow_mut();

        self.window.draw_2d(event, |c, g| widgets.draw(0, c, g));
    }

    fn handle_event(
        &mut self,
        widget_id: i32,
        event_handler: &mut PushrodCallbackEvents,
        event: CallbackEvent,
    ) {
        if widget_id == -1 {
            return;
        }

        let injectable_event = self
            .widget_store
            .borrow_mut()
            .handle_event(widget_id, event.clone());

        event_handler.handle_event(event.clone(), &mut self.widget_store.borrow_mut());

        match injectable_event {
            Some(new_event) => {
                event_handler.handle_event(new_event.clone(), &mut self.widget_store.borrow_mut())
            }
            None => (),
        }
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        eprintln!("[Resize] W={} H={}", width, height);
        self.texture_buffer = Box::new(vec![0u8; width as usize * height as usize]);
        self.texture = Texture::from_memory_alpha(&self.texture_buffer, width, height, &TextureSettings::new()).unwrap();

        unsafe {
            let mut fbos: [GLuint; 1] = [0];

            gl::GenFramebuffers(1, fbos.as_mut_ptr());
            self.fbo = fbos[0];

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D,
                self.texture.get_id(), 0);
        }
    }

    fn prepare_buffers(&mut self) {
        let draw_size = self.window.draw_size();
        self.handle_resize(draw_size.width as u32, draw_size.height as u32);
    }

    /// This is the main run loop that is called to process all UI events.  This loop is responsible
    /// for handling events from the OS, converting them to workable objects, and passing them off
    /// to quick callback dispatchers.
    ///
    /// The run loop handles events in the following order:
    ///
    /// - Mouse events
    ///   - Movement events
    ///   - Button events
    ///   - Scroll button events
    /// - Custom events are then dispatched to any registered event listeners
    /// - Draw loop
    ///   - Draw only widgets whose states have become invalidated
    ///   - Swap display buffers if required
    ///
    /// This event is handled window-by-window.  Once a window has processed all of its pending
    /// events, the next window is then processed.  No particular window takes precidence - any
    /// window that has events to process gets handled in order.
    pub fn run(&mut self, event_handler: &mut PushrodCallbackEvents) {
        let mut last_widget_id = -1;
        let mut previous_mouse_position: Point = make_origin_point();
        let mut button_map: HashMap<i32, HashSet<Button>> = HashMap::new();
        let injectable_map: Vec<i32> = self
            .widget_store
            .borrow_mut()
            .widgets
            .iter()
            .filter(|x| x.widget.borrow_mut().injects_events())
            .map(|x| x.widget_id)
            .collect();
        let mut gl: GlGraphics = GlGraphics::new(OpenGL::V3_2);

        eprintln!("Injectable Map: {:?}", injectable_map);
        eprintln!("Window Size: {:?}", self.window.size());
        eprintln!("Draw Size: {:?}", self.window.window.draw_size());

        self.window.set_max_fps(30);
        self.prepare_buffers();

        while let Some(ref event) = &self.window.next() {
            event.mouse_cursor(|x, y| {
                let mouse_point = make_point_f64(x, y);

                if mouse_point.x != previous_mouse_position.x
                    || mouse_point.y != previous_mouse_position.y
                {
                    previous_mouse_position = mouse_point.clone();

                    let current_widget_id = self
                        .widget_store
                        .borrow_mut()
                        .get_widget_id_for_point(mouse_point.clone());

                    // Handles the mouse move callback.
                    if current_widget_id != -1 {
                        self.handle_event(
                            current_widget_id,
                            event_handler,
                            CallbackEvent::MouseMoved {
                                widget_id: current_widget_id,
                                point: mouse_point.clone(),
                            },
                        );
                    }

                    if current_widget_id != last_widget_id {
                        if last_widget_id != -1 {
                            self.handle_event(
                                last_widget_id,
                                event_handler,
                                CallbackEvent::MouseExited {
                                    widget_id: last_widget_id,
                                },
                            );
                        }

                        last_widget_id = current_widget_id;

                        if last_widget_id != -1 {
                            self.handle_event(
                                last_widget_id,
                                event_handler,
                                CallbackEvent::MouseEntered {
                                    widget_id: last_widget_id,
                                },
                            );
                        }
                    }
                }
            });

            event.mouse_scroll(|x, y| {
                let mouse_point = make_point_f64(x, y);

                if last_widget_id != -1 {
                    self.handle_event(
                        last_widget_id,
                        event_handler,
                        CallbackEvent::MouseScrolled {
                            widget_id: last_widget_id,
                            point: mouse_point.clone(),
                        },
                    );
                }
            });

            event.button(|args| match args.state {
                ButtonState::Press => {
                    button_map
                        .entry(last_widget_id)
                        .or_insert(HashSet::new())
                        .insert(args.button);

                    self.handle_event(
                        last_widget_id,
                        event_handler,
                        CallbackEvent::MouseButtonDown {
                            widget_id: last_widget_id,
                            button: args.button,
                        },
                    );
                }

                ButtonState::Release => {
                    let button_set = button_map.entry(last_widget_id).or_insert(HashSet::new());

                    if button_set.contains(&args.button) {
                        button_set.remove(&args.button);

                        self.handle_event(
                            last_widget_id,
                            event_handler,
                            CallbackEvent::MouseButtonUpInside {
                                widget_id: last_widget_id,
                                button: args.button,
                            },
                        );
                    } else {
                        for (widget_id, button_set) in button_map.iter_mut() {
                            if button_set.contains(&args.button) {
                                self.handle_event(
                                    *widget_id,
                                    event_handler,
                                    CallbackEvent::MouseButtonUpOutside {
                                        widget_id: *widget_id,
                                        button: args.button,
                                    },
                                );

                                button_set.remove(&args.button);
                            }
                        }
                    }
                }
            });

            event.resize(|w, h| {
                self.handle_resize(w as u32, h as u32);

                event_handler.handle_event(
                    CallbackEvent::WindowResized {
                        size: crate::core::point::Size {
                            w: w as i32,
                            h: h as i32,
                        },
                    },
                    &mut self.widget_store.borrow_mut(),
                );
            });

            event.focus(|focused| {
                self.handle_event(
                    last_widget_id,
                    event_handler,
                    CallbackEvent::WindowFocused { flag: focused },
                );
            });

            match event {
                Event::Input(Input::Button(ButtonArgs {
                    state,
                    button: Button::Keyboard(key),
                    scancode: _,
                })) => {
                    self.handle_event(
                        last_widget_id,
                        event_handler,
                        CallbackEvent::KeyPressed {
                            widget_id: last_widget_id,
                            key: *key,
                            state: *state,
                        },
                    );
                }
                _ => {}
            };

            event.resize(|_, _| {
                self.widget_store.borrow_mut().invalidate_all_widgets();
            });

            // FPS loop handling

            event.render(|args| {
                injectable_map.iter().for_each(|widget_id| {
                    let injectable_event = self
                        .widget_store
                        .borrow_mut()
                        .get_widget_for_id(*widget_id)
                        .borrow_mut()
                        .inject_event(*widget_id);

                    match injectable_event {
                        Some(x) => {
                            self.handle_event(*widget_id, event_handler, x.clone());
                            self.widget_store.borrow_mut().inject_event(x.clone());
                        }
                        None => (),
                    }
                });

                let widgets = &mut self.widget_store.borrow_mut();

                unsafe {
                    gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
                }

//                gl.draw(args.viewport(), |c, g| widgets.draw(0, c, g));

                unsafe {
                    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                }

                gl.draw(args.viewport(), |c, g| {
                    clear([1.0, 1.0, 1.0, 0.0], g);
                    let flipped = c.transform.prepend_transform(scale(1.0, -1.0));
                    Image::new().draw(&self.texture, &c.draw_state, flipped, g);
                });


//                self.window.draw_2d(event, |c, g| widgets.draw(0, c, g));

//                self.handle_draw(&event);
//                self.widget_store.borrow_mut().invalidate_all_widgets();
            });
        }
    }
}
