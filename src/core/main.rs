// Main Event Dispatcher
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
use crate::widget::widget::*;

use opengl_graphics::GlGraphics;
use piston_window::*;

use std::cell::RefCell;

pub struct Pushrod {
    window_opengl: OpenGL,
    windows: RefCell<Vec<PushrodWindow>>,
    event_listeners: RefCell<Vec<Box<PushrodEventListener>>>,
    event_list: RefCell<Vec<PushrodEvent>>,
}

impl Pushrod {
    pub fn new(config: OpenGL) -> Self {
        Self {
            window_opengl: config,
            windows: RefCell::new(Vec::new()),
            event_listeners: RefCell::new(Vec::new()),
            event_list: RefCell::new(Vec::new()),
        }
    }

    pub fn add_window(&self, window: PushrodWindow) {
        self.windows.borrow_mut().push(window);
    }

    pub fn add_event_listener_for_window(&self, listener: Box<PushrodEventListener>) {
        self.event_listeners.borrow_mut().push(listener);
    }

    // By handling events internally, we bypass the risk of the user having to interpret each
    // event, and having to figure out how to dispatch those events to any widgets that might be
    // in the display area.  Events will eventually be dispatched using a "dispatch all" method,
    // which will be done at the end of the event loop.  Any draw routines will be done within
    // the render_args() area, and a separate event will be sent out for that, as drawing
    // should be done at the end of all event processing, within the rendering loop, not the
    // updating loop (UPS vs. FPS)

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

    fn internal_derive_event_mask(&self, event: &PushrodEvent) -> PushrodEventMask {
        match event {
            PushrodEvent::MouseEvent { point: _ } => PUSHROD_EVENT_MOUSE_MOVED,
            PushrodEvent::MouseDownEvent { button: _ } => PUSHROD_EVENT_MOUSE_DOWN,
            PushrodEvent::MouseUpEvent { button: _ } => PUSHROD_EVENT_MOUSE_UP,
            PushrodEvent::MouseScrollEvent { point: _ } => PUSHROD_EVENT_MOUSE_SCROLL,
        }
    }

    pub fn run(&self) {
        let mut gl: GlGraphics = GlGraphics::new(self.window_opengl);

        for (_window_id, pushrod_window) in self.windows.borrow_mut().iter_mut().enumerate() {
            while let Some(event) = &pushrod_window.window.next() {
                if let Some([x, y]) = event.mouse_cursor_args() {
                    let point = make_point_f64(x, y);

                    self.internal_handle_mouse_move(point.clone());
                    let widget_id = pushrod_window.get_widget_id_for_point(point);

                    eprintln!("Widget ID: {}", widget_id);
                }

                if let Some(button) = event.button_args() {
                    self.internal_handle_mouse_button(button);
                }

                if let Some([x, y]) = event.mouse_scroll_args() {
                    self.internal_handle_mouse_scroll(make_point_f64(x, y));
                }

                // Dispatch events here in the bus
                self.internal_dispatch_events();

                // FPS loop handling

                if let Some(args) = event.render_args() {
                    gl.draw(args.viewport(), |_context, graphics| {
                        clear([1.0; 4], graphics);
                    });

                    // Reset GL drawing state

                    // Dispatch GL drawing to event loop
                }
            }
        }
    }
}
