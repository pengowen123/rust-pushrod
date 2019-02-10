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

use crate::core::point::*;
use crate::core::window::*;
use crate::event::event::*;

use opengl_graphics::GlGraphics;
use piston_window::*;

use std::cell::RefCell;

/// This structure is returned when instantiating a new Pushrod main object.
/// It stores the OpenGL configuration that is desired for drawing, a list of references
/// to a managed set of `PushrodWindow` objects, registered `EventListener`s, and
/// `PushrodEvent` objects that are pending dispatch.
///
/// The objects contained within this structure are used by the `Pushrod` run loop, and
/// are not intended to be modified except through methods in the `Pushrod` impl.
pub struct Pushrod {
    window_opengl: OpenGL,
    windows: RefCell<Vec<PushrodWindow>>,
    event_listeners: RefCell<Vec<Box<EventListener>>>,
    event_list: RefCell<Vec<PushrodEvent>>,
}

/// Pushrod implementation.  Create a `Pushrod::new( OpenGL )` object to create a new
/// main loop.  Only one of these should be set for the entire application runtime.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::main::*;
/// # use pushrod::core::window::*;
/// # fn main() {
///     // Define the version of OpenGL to use with the application
///     let opengl = OpenGL::V3_2;
///
///     // Create a new Pushrod object with the OpenGL version
///     let prod: Pushrod = Pushrod::new(opengl);
///
///     // Create a PushrodWindow container to store the PistonWindow
///     let mut pushrod_window: PushrodWindow = PushrodWindow::new(
///         WindowSettings::new("Pushrod Window", [640, 480])
///             .opengl(opengl)
///             .build()
///             .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)),
///     );
///
///     // Add the window to the managed stack.
///     prod.add_window(pushrod_window);
///
///     // Initiate the run loop.
///     prod.run();
/// # }
/// ```
impl Pushrod {
    /// Pushrod Object Constructor.  Takes in a single OpenGL configuration type.
    pub fn new(config: OpenGL) -> Self {
        Self {
            window_opengl: config,
            windows: RefCell::new(Vec::new()),
            event_listeners: RefCell::new(Vec::new()),
            event_list: RefCell::new(Vec::new()),
        }
    }

    /// Adds a managed window to the stack.  Changes the `swap_buffers` flag to `false`, as if
    /// this were set to `true`, the main draw loop would be extremely expensive.
    pub fn add_window(&self, mut window: PushrodWindow) {
        &window.window.set_swap_buffers(false);
        self.windows.borrow_mut().push(window);
    }

    /// Adds an event listener to the stack.  This should be an implementation of the
    /// `PushrodEventListener` trait.
    ///
    /// Example:
    /// ```no_run
    /// # use piston_window::*;
    /// # use pushrod::core::main::*;
    /// # use pushrod::core::window::*;
    /// # use pushrod::core::point::*;
    /// # use pushrod::event::event::*;
    /// struct ExampleListener {}
    ///
    /// impl ExampleListener {
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    ///
    ///     fn handle_mouse_move(&self, point: &Point) {
    ///         eprintln!("[Listener] X={} Y={}", point.x, point.y);
    ///     }
    ///
    ///     fn handle_mouse_down(&self, button: &MouseButton) {
    ///         match button {
    ///             MouseButton::Left => eprintln!("[Listener] Left mouse button pressed."),
    ///             _ => eprintln!("[Listener] Other mouse button pressed."),
    ///         }
    ///     }
    ///
    ///     fn handle_mouse_up(&self, button: &MouseButton) {
    ///         match button {
    ///             MouseButton::Left => eprintln!("[Listener] Left mouse button released."),
    ///             _ => eprintln!("[Listener] Other mouse button released."),
    ///         }
    ///     }
    ///
    ///     fn handle_mouse_scroll(&self, point: &Point) {
    ///         eprintln!("[Listener] Scroll: X={} Y={}", point.x, point.y);
    ///     }
    /// }
    ///
    /// impl EventListener for ExampleListener {
    ///     fn handle_event(&self, event: &PushrodEvent) {
    ///         match event {
    ///             PushrodEvent::MouseEvent { point } => self.handle_mouse_move(&point),
    ///             PushrodEvent::MouseDownEvent { button } => self.handle_mouse_down(&button),
    ///             PushrodEvent::MouseUpEvent { button } => self.handle_mouse_up(&button),
    ///             PushrodEvent::MouseScrollEvent { point } => self.handle_mouse_scroll(&point),
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    /// #    // Define the version of OpenGL to use with the application
    /// #    let opengl = OpenGL::V3_2;
    /// #
    ///     // Create a new Pushrod object with the OpenGL version
    ///     let prod: Pushrod = Pushrod::new(opengl);
    /// #
    /// #    // Create a PushrodWindow container to store the PistonWindow
    /// #    let mut pushrod_window: PushrodWindow = PushrodWindow::new(
    /// #        WindowSettings::new("Pushrod Window", [640, 480])
    /// #            .opengl(opengl)
    /// #            .build()
    /// #            .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)),
    /// #    );
    /// #
    /// #    // Add the window to the managed stack.
    /// #    prod.add_window(pushrod_window);
    ///
    ///     // Add the event listener to the run loop.
    ///     prod.add_event_listener_for_window(Box::new(ExampleListener::new()));
    ///
    ///     // Initiate the run loop.
    ///     prod.run();
    /// }
    /// ```
    pub fn add_event_listener_for_window(&self, listener: Box<EventListener>) {
        self.event_listeners.borrow_mut().push(listener);
    }

    /*
     * By handling events internally, we bypass the risk of the user having to interpret each
     * event, and having to figure out how to dispatch those events to any widgets that might be
     * in the display area.  Events will eventually be dispatched using a "dispatch all" method,
     * which will be done at the end of the event loop.  Any draw routines will be done within
     * the render_args() area, and a separate event will be sent out for that, as drawing
     * should be done at the end of all event processing, within the rendering loop, not the
     * updating loop (UPS vs. FPS)
     */

    fn internal_handle_mouse_move(&self, point: Point) {
        // Send the point movement to the widget event handler.

        self.event_list
            .borrow_mut()
            .push(PushrodEvent::MouseEvent { point });
    }

    fn internal_handle_mouse_button(&self, button: ButtonArgs) {
        // Send the button click to the widget event handler.

        if button.state == ButtonState::Press {
            match button.button {
                Button::Mouse(button) => {
                    self.event_list
                        .borrow_mut()
                        .push(PushrodEvent::MouseDownEvent { button });
                }
                _ => (),
            }
        } else if button.state == ButtonState::Release {
            match button.button {
                Button::Mouse(button) => {
                    self.event_list
                        .borrow_mut()
                        .push(PushrodEvent::MouseUpEvent { button });
                }
                _ => (),
            }
        }
    }

    fn internal_handle_mouse_scroll(&self, point: Point) {
        // Send the mouse scroll to the widget event handler.

        self.event_list
            .borrow_mut()
            .push(PushrodEvent::MouseScrollEvent { point });
    }

    fn internal_dispatch_events(&self) {
        for event in self.event_list.borrow_mut().iter() {
            for listener in self.event_listeners.borrow_mut().iter() {
                let event_mask = self.internal_derive_event_mask(event);

                if listener.event_mask() & event_mask == event_mask {
                    listener.handle_event(event);
                }
            }
        }

        self.event_list.borrow_mut().clear();
    }

    fn internal_derive_event_mask(&self, event: &PushrodEvent) -> EventMask {
        match event {
            PushrodEvent::MouseEvent { point: _ } => MASK_EVENT_MOUSE_MOVED,
            PushrodEvent::MouseDownEvent { button: _ } => MASK_EVENT_MOUSE_DOWN,
            PushrodEvent::MouseUpEvent { button: _ } => MASK_EVENT_MOUSE_UP,
            PushrodEvent::MouseScrollEvent { point: _ } => MASK_EVENT_MOUSE_SCROLL,
        }
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
    pub fn run(&self) {
        let mut gl: GlGraphics = GlGraphics::new(self.window_opengl);
        let mut last_widget_id = -1;
        let mut previous_mouse_position: Point = make_origin_point();

        for (_window_id, pushrod_window) in self.windows.borrow_mut().iter_mut().enumerate() {
            while let Some(event) = &pushrod_window.window.next() {
                if let Some([x, y]) = event.mouse_cursor_args() {
                    let mouse_point = make_point_f64(x, y);

                    if mouse_point.x != previous_mouse_position.x
                        || mouse_point.y != previous_mouse_position.y
                    {
                        previous_mouse_position = mouse_point.clone();

                        self.internal_handle_mouse_move(mouse_point.clone());

                        let current_widget_id = pushrod_window.get_widget_id_for_point(mouse_point);
                        let current_parent_for_widget =
                            pushrod_window.get_parent_of(current_widget_id);

                        if current_widget_id != last_widget_id {
                            if last_widget_id != -1 {
                                pushrod_window.mouse_exited_for_id(last_widget_id);
                            }

                            last_widget_id = current_widget_id;

                            if last_widget_id != -1 {
                                pushrod_window.mouse_entered_for_id(last_widget_id);
                            }
                        }

                        eprintln!(
                            "Widget IDs: current={} parent={} children={:?}",
                            current_widget_id, current_parent_for_widget,
                            pushrod_window.get_children_of(current_widget_id)
                        );
                    }
                }

                if let Some(button) = event.button_args() {
                    self.internal_handle_mouse_button(button);
                }

                if let Some([x, y]) = event.mouse_scroll_args() {
                    let mouse_point = make_point_f64(x, y);

                    self.internal_handle_mouse_scroll(mouse_point.clone());

                    if last_widget_id != -1 {
                        pushrod_window.mouse_scrolled_for_id(last_widget_id, mouse_point.clone());
                    }
                }

                // Dispatch events here in the bus
                self.internal_dispatch_events();

                // FPS loop handling

                if let Some(args) = event.render_args() {
                    let mut require_buffer_swap = false;

                    gl.draw(args.viewport(), |context, graphics| {
                        pushrod_window.widgets.iter_mut().for_each(|widget| {
                            if widget.widget.is_invalidated() {
                                widget.widget.draw(context, graphics);
                                require_buffer_swap = true;
                            }
                        });
                    });

                    if require_buffer_swap {
                        pushrod_window.window.window.swap_buffers();
                        eprintln!("[Run Loop] Display buffers swapped due to invalidation.");
                    }
                }
            }
        }
    }
}
