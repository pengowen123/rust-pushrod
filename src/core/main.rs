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
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::core::callbacks::*;
use crate::core::drawing_texture::*;
use crate::core::layout_manager::*;
use crate::core::point::*;
use crate::core::widget_store::*;
use crate::widget::widget::*;

use glfw_window::GlfwWindow;

use graphics::math::scale;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::*;

/// This structure is returned when instantiating a new Pushrod main object.
pub struct Pushrod {
    window: GlfwWindow,
    events: Events,

    /// This is the `WidgetStore` object that is used to store the `Widget` list in the current
    /// display stack.
    pub widget_store: Rc<RefCell<WidgetStore>>,
    pub drawing_texture: DrawingTexture,
}

/// Pushrod implementation.  Create a `Pushrod::new( PistonWindow )` object to create a new
/// main loop.  Only one of these should be set for the entire application runtime.
impl Pushrod {
    /// Pushrod Object Constructor.  Takes in a single OpenGL configuration type.
    pub fn new(window: GlfwWindow) -> Self {
        let event_settings = EventSettings::new().max_fps(30);
        Self {
            window,
            events: Events::new(event_settings),
            widget_store: Rc::new(RefCell::new(WidgetStore::new())),
            drawing_texture: DrawingTexture::new(),
        }
    }

    /// Convenience method that adds a `Widget` to the GUI display stack.
    pub fn add_widget(&mut self, name: &str, widget: Box<dyn Widget>) -> i32 {
        self.widget_store.borrow_mut().add_widget(name, widget)
    }

    /// Convenience method that adds a `LayoutManager` to the layout management stack.
    pub fn add_layout_manager(&mut self, manager: Box<dyn LayoutManager>) -> i32 {
        self.widget_store.borrow_mut().add_layout_manager(manager)
    }

    /// Convenience method that adds a `Widget` to a `LayoutManager` by the manager's ID and
    /// the positioning of the `Widget`.
    pub fn add_widget_to_layout_manager(
        &mut self,
        name: &str,
        widget: Box<dyn Widget>,
        manager_id: i32,
        position: Point,
    ) -> i32 {
        let widget_id = self
            .widget_store
            .borrow_mut()
            .add_widget_to_layout_manager(name, widget, manager_id, position);

        self.widget_store
            .borrow_mut()
            .do_layout_for_manager(manager_id);

        widget_id
    }

    /// Convenience method that adds a `Widget` to a parent by its ID.  This guarantees a refresh
    /// if the top level parent becomes invalidated.
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

    /// Convenience method that adds a `Widget` to a parent by the parent's name.
    pub fn add_widget_to_parent_by_name(
        &mut self,
        parent_name: &str,
        name: &str,
        widget: Box<dyn Widget>,
    ) -> i32 {
        let parent_id = self
            .widget_store
            .borrow_mut()
            .get_widget_id_for_name(parent_name);

        self.widget_store
            .borrow_mut()
            .add_widget_to_parent(name, widget, parent_id)
    }

    fn broadcast_event(&mut self, event: CallbackEvent) {
        self.widget_store
            .borrow_mut()
            .widgets
            .iter_mut()
            .for_each(|container| {
                container
                    .widget
                    .borrow_mut()
                    .handle_event(false, event.clone());
            });
    }

    fn handle_event(
        &mut self,
        widget_id: i32,
        event_handler: &mut dyn PushrodCallbackEvents,
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

    fn handle_system_event(
        &mut self,
        event_handler: &mut dyn PushrodCallbackEvents,
        event: CallbackEvent,
    ) {
        eprintln!("Handling system event: {:?}", event.clone());
        event_handler.handle_event(event.clone(), &mut self.widget_store.borrow_mut());
    }

    fn handle_resize(&mut self, width: u32, height: u32) {
        eprintln!("[Resize] W={} H={}", width, height);
        self.rebuild_gl_buffers();
    }

    fn rebuild_gl_buffers(&mut self) {
        let draw_size = self.window.draw_size();

        self.drawing_texture.resize(crate::core::point::Size {
            w: draw_size.width as i32,
            h: draw_size.height as i32,
        });

        eprintln!("Rebuild of OpenGL buffers for rendering complete.");
    }

    fn get_system_events_list(&mut self) -> Vec<CallbackEvent> {
        let widgets = &self.widget_store.borrow().widgets;
        let mut return_list = vec![];

        for widget in widgets {
            let mut widget_container = widget.widget.borrow_mut();

            if widget_container.injects_system_events() {
                match widget_container
                    .get_injectable_system_events()
                    .inject_system_event()
                {
                    Some(ev) => return_list.push(ev.clone()),
                    _ => (),
                }
            }
        }

        return_list
    }

    /// This is the main run loop for `Pushrod`.  A run loop requires the use of an assigned
    /// `PushrodCallbackEvents` event handler.  This is how all communications take place when
    /// an action occurs within the GUI window.
    pub fn run(&mut self, event_handler: &mut dyn PushrodCallbackEvents) {
        let mut last_widget_id = -1;
        let mut previous_mouse_position: Point = make_origin_point();
        let mut button_map: HashMap<i32, HashSet<Button>> = HashMap::new();
        let injectable_map: Vec<i32> = self
            .widget_store
            .borrow_mut()
            .widgets
            .iter()
            .filter(|x| x.widget.borrow_mut().injects_custom_events())
            .map(|x| x.widget_id)
            .collect();
        let mut gl: GlGraphics = GlGraphics::new(OpenGL::V3_2);

        eprintln!("Injectable Map: {:?}", injectable_map);
        eprintln!("Window Size: {:?}", self.window.size());
        eprintln!("Draw Size: {:?}", self.window.draw_size());

        self.widget_store.borrow_mut().invalidate_all_widgets();
        self.rebuild_gl_buffers();

        while let Some(ref event) = self.events.next(&mut self.window) {
            let events_list = self.get_system_events_list();

            for event in events_list {
                self.handle_system_event(event_handler, event.clone());
            }

            event.mouse_cursor(|pos| {
                let mouse_point = make_point_f64(pos[0], pos[1]);

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

            event.mouse_scroll(|pos| {
                let mouse_point = make_point_f64(pos[0], pos[1]);

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

            event.resize(|args| {
                let w: u32 = args.window_size[0] as u32;
                let h: u32 = args.window_size[1] as u32;

                self.handle_resize(w, h);
                self.widget_store
                    .borrow_mut()
                    .resize_layout_managers(w as u32, h as u32);

                self.broadcast_event(CallbackEvent::WindowResized {
                    size: crate::core::point::Size {
                        w: w as i32,
                        h: h as i32,
                    },
                });

                self.widget_store.borrow_mut().invalidate_all_widgets();
            });

            event.focus(|focused| {
                self.handle_event(
                    last_widget_id,
                    event_handler,
                    CallbackEvent::WindowFocused { flag: focused },
                );
            });
            //
            //            match event {
            //                Event::Input(Input::Button(ButtonArgs {
            //                    state,
            //                    button: Button::Keyboard(key),
            //                    scancode: _,
            //                })) => {
            //                    self.handle_event(
            //                        last_widget_id,
            //                        event_handler,
            //                        CallbackEvent::KeyPressed {
            //                            widget_id: last_widget_id,
            //                            key: *key,
            //                            state: *state,
            //                        },
            //                    );
            //                }
            //                _ => {}
            //            };

            event.render(|args| {
                injectable_map.iter().for_each(|widget_id| {
                    let can_inject = self
                        .widget_store
                        .borrow_mut()
                        .get_widget_for_id(*widget_id)
                        .borrow_mut()
                        .injects_custom_events();

                    if can_inject {
                        let injectable_event = self
                            .widget_store
                            .borrow_mut()
                            .get_widget_for_id(*widget_id)
                            .borrow_mut()
                            .get_injectable_custom_events()
                            .inject_custom_event(*widget_id);

                        match injectable_event {
                            Some(x) => {
                                self.handle_event(*widget_id, event_handler, x.clone());
                                self.widget_store.borrow_mut().inject_event(x.clone());
                            }
                            None => (),
                        }
                    }
                });

                if self.widget_store.borrow_mut().needs_repaint() {
                    let widgets = &mut self.widget_store.borrow_mut();

                    self.drawing_texture.switch_to_texture();

                    gl.draw(args.viewport(), |c, g| {
                        widgets.draw(0, c, g, self.drawing_texture.fbo)
                    });

                    self.drawing_texture.switch_to_fb(0);
                }

                // Redraw the currently assigned drawing area.
                gl.draw(args.viewport(), |c, g| {
                    clear([1.0, 1.0, 1.0, 0.0], g);
                    let flipped = c.transform.prepend_transform(scale(1.0, -1.0));

                    // Enable zoom only if the draw size is larger than the window size.
                    let zoom_factor = (self.window.size().width + self.window.size().height)
                        / (self.window.draw_size().width + self.window.draw_size().height);

                    Image::new().draw(
                        &self.drawing_texture.texture,
                        &c.draw_state,
                        flipped.zoom(zoom_factor),
                        g,
                    );
                });
            });
        }
    }
}
