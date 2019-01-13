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

use crate::core::point::Point;
use crate::event::event::*;
use opengl_graphics::GlGraphics;
use piston_window::*;

use std::cell::RefCell;

struct WindowList {
    windows: Vec<PistonWindow>,
    window_position: usize,
}

impl WindowList {
    pub fn new() -> WindowList {
        WindowList {
            windows: Vec::new(),
            window_position: 0,
        }
    }

    pub fn push(&mut self, window: PistonWindow) {
        self.windows.push(window);
    }

    pub fn next_window(&mut self) -> (Option<Event>, &PistonWindow) {
        let mut cur_window_position = self.window_position;

        cur_window_position += 1;

        if cur_window_position > self.windows.len() - 1 {
            cur_window_position = 0;
        }

        self.window_position = cur_window_position;

        (
            self.windows[self.window_position].next(),
            &self.windows[self.window_position],
        )
    }
}

pub struct Pushrod {
    window_opengl: OpenGL,
    windows: RefCell<WindowList>,
    event_listeners: RefCell<Vec<Box<PushrodEventListener>>>,
    event_list: RefCell<Vec<PushrodEvent>>,
}

impl Pushrod {
    pub fn new(config: OpenGL) -> Self {
        Self {
            window_opengl: config,
            windows: RefCell::new(WindowList::new()),
            event_listeners: RefCell::new(Vec::new()),
            event_list: RefCell::new(Vec::new()),
        }
    }

    pub fn add_window(&self, window: PistonWindow) {
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

    fn internal_derive_event_mask(&self, event: &PushrodEvent) -> PushrodEventMask {
        let mut event_mask = PUSHROD_EVENT_NONE;

        match event {
            PushrodEvent::MouseEvent { point: _ } => event_mask = PUSHROD_EVENT_MOUSE_MOVED,
            PushrodEvent::MouseDownEvent { button: _ } => event_mask = PUSHROD_EVENT_MOUSE_DOWN,
            PushrodEvent::MouseUpEvent { button: _ } => event_mask = PUSHROD_EVENT_MOUSE_UP,
            PushrodEvent::MouseScrollEvent { point: _ } => event_mask = PUSHROD_EVENT_MOUSE_SCROLL,
        }

        event_mask
    }

    pub fn run(&self) {
        let mut gl: GlGraphics = GlGraphics::new(self.window_opengl);

        while let (Some(event), _window) = self.windows.borrow_mut().next_window() {
            // UPS loop handling

            if let Some([x, y]) = event.mouse_cursor_args() {
                self.internal_handle_mouse_move(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }

            if let Some(button) = event.button_args() {
                self.internal_handle_mouse_button(button);
            }

            if let Some([x, y]) = event.mouse_scroll_args() {
                self.internal_handle_mouse_scroll(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }

            // Dispatch events here in the bus

            for event in self.event_list.borrow_mut().iter() {
                for listener in self.event_listeners.borrow_mut().iter() {
                    let event_mask = self.internal_derive_event_mask(event);

                    if listener.event_mask() & event_mask == event_mask {
                        listener.handle_event(event);
                    }
                }
            }

            self.event_list.borrow_mut().clear();

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
