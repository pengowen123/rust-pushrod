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

use std::collections::HashMap;
use std::collections::HashSet;

use crate::core::point::*;
use crate::core::widget_store::*;

use piston_window::*;

/// This structure is returned when instantiating a new Pushrod main object.
/// It stores the OpenGL configuration that is desired for drawing, a list of references
/// to a managed set of `PushrodWindow` objects, registered `EventListener`s, and
/// `PushrodEvent` objects that are pending dispatch.
///
/// The objects contained within this structure are used by the `Pushrod` run loop, and
/// are not intended to be modified except through methods in the `Pushrod` impl.
pub struct Pushrod {
    window: PistonWindow,
    pub widget_store: WidgetStore,
    //    event_listeners: RefCell<Vec<Box<EventListener>>>,
    //    event_list: RefCell<Vec<PushrodEvent>>,
}

/// Pushrod implementation.  Create a `Pushrod::new( OpenGL )` object to create a new
/// main loop.  Only one of these should be set for the entire application runtime.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::main::*;
/// # fn main() {
///     // Create a PushrodWindow container to store the PistonWindow
///     let mut prod: Pushrod = Pushrod::new(
///         WindowSettings::new("Pushrod Window", [640, 480])
///             .opengl(OpenGL::V3_2)
///             .build()
///             .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)));
///
///     // Initiate the run loop.
///     prod.run();
/// # }
/// ```
impl Pushrod {
    /// Pushrod Object Constructor.  Takes in a single OpenGL configuration type.
    pub fn new(window: PistonWindow) -> Self {
        Self {
            window,
            widget_store: WidgetStore::new(),
            //            event_listeners: RefCell::new(Vec::new()),
            //            event_list: RefCell::new(Vec::new()),
        }
    }

    /// Retrieves the window `GfxFactory` factory object for graphics textures.
    pub fn get_factory(&mut self) -> &mut GfxFactory {
        &mut self.window.factory
    }

    fn handle_draw(&mut self, event: &Event) {
        let widgets = &mut self.widget_store;

        self.window.draw_2d(event, |c, g| widgets.draw(0, c, g));
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
    pub fn run(&mut self) {
        let mut last_widget_id = -1;
        let mut previous_mouse_position: Point = make_origin_point();
        let mut button_map: HashMap<i32, HashSet<Button>> = HashMap::new();

        while let Some(ref event) = &self.window.next() {
            event.mouse_cursor(|x, y| {
                let mouse_point = make_point_f64(x, y);

                if mouse_point.x != previous_mouse_position.x
                    || mouse_point.y != previous_mouse_position.y
                {
                    previous_mouse_position = mouse_point.clone();

                    let current_widget_id = self
                        .widget_store
                        .get_widget_id_for_point(mouse_point.clone());
                    let current_parent_for_widget =
                        self.widget_store.get_parent_of(current_widget_id);

                    // Handles the mouse move callback.
                    if current_widget_id != -1 {
                        self.widget_store
                            .mouse_moved_for_id(current_widget_id, mouse_point.clone());
                    }

                    if current_widget_id != last_widget_id {
                        if last_widget_id != -1 {
                            self.widget_store.mouse_exited_for_id(last_widget_id);
                        }

                        last_widget_id = current_widget_id;

                        if last_widget_id != -1 {
                            self.widget_store.mouse_entered_for_id(last_widget_id);
                        }

                        eprintln!(
                            "Widget IDs: current={} parent={} children={:?}",
                            current_widget_id,
                            current_parent_for_widget,
                            self.widget_store.get_children_of(current_widget_id)
                        );
                    }
                }
            });

            event.mouse_scroll(|x, y| {
                let mouse_point = make_point_f64(x, y);

                if last_widget_id != -1 {
                    self.widget_store
                        .mouse_scrolled_for_id(last_widget_id, mouse_point.clone());
                }
            });

            event.button(|args| match args.state {
                ButtonState::Press => {
                    button_map
                        .entry(last_widget_id)
                        .or_insert(HashSet::new())
                        .insert(args.button);

                    self.widget_store.button_down(last_widget_id, args.button);
                }
                ButtonState::Release => {
                    let mut button_set = button_map.entry(last_widget_id)
                        .or_insert(HashSet::new());

                    if button_set.contains(&args.button) {
                        button_set.remove(&args.button);
                        self.widget_store.button_up_inside(last_widget_id, args.button);
                    } else {
                        // Find the button that was set for button down in the button map
                        // search each set, and check for the args.button
                        // The hash_map that matches that ID should be called with a
                        // button_up_outside callback.
                    }
                },
            });

            event.resize(|w, h| {
                self.widget_store.handle_resize(w as u32, h as u32);
            });

            event.focus(|focused| {
                self.widget_store.handle_focus(focused);
            });

            match event {
                Event::Input(Input::Button(ButtonArgs {
                    state,
                    button: Button::Keyboard(key),
                    scancode: _,
                })) => {
                    self.widget_store
                        .keypress_for_id(last_widget_id, &key, &state);
                }
                _ => {}
            };

            event.resize(|_, _| {
                self.widget_store.invalidate_all_widgets();
            });

            // FPS loop handling

            event.render(|_| {
                self.handle_draw(&event);
                self.widget_store.invalidate_all_widgets();
            });
        }
    }
}
