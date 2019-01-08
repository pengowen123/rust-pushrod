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

use piston_window::*;
use opengl_graphics::GlGraphics;
use crate::core::point::Point;
use crate::event::event::EventListener;

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
        (self.windows[self.window_position].next(), &self.windows[self.window_position])
    }
}

pub struct Pushrod {
    window_opengl: OpenGL,
    windows: RefCell<WindowList>,
}

impl Pushrod {
    pub fn new(config: OpenGL) -> Self {
        Self {
            window_opengl: config,
            windows: RefCell::new(WindowList::new()),
        }
    }

    pub fn add_window(&self, window: PistonWindow) {
        self.windows.borrow_mut().push(window);
    }

    pub fn add_event_listener_for_window(&self, listener: Box<EventListener>) {
        println!("Handle event listening addition to vector list here.");
    }

    // By handling events internally, we bypass the risk of the user having to interpret each
    // event, and having to figure out how to dispatch those events to any widgets that might be
    // in the display area.  Events will eventually be dispatched using a "dispatch all" method,
    // which will be done at the end of the event loop.  Any draw routines will be done within
    // the render_args() area, and a separate event will be sent out for that, as drawing
    // should be done at the end of all event processing, within the rendering loop, not the
    // updating loop (UPS vs. FPS)

    fn internal_handle_mouse_event(&self, point: Point) {
        println!("X={} Y={}", point.x, point.y);
    }

    pub fn run(&self) {
        let mut gl: GlGraphics = GlGraphics::new(self.window_opengl);

        while let (Some(event), _window) = self.windows.borrow_mut().next_window() {
//            let mut _event_list: Vec<Event> = Vec::new();

            // UPS loop handling

            if let Some([x, y]) = event.mouse_cursor_args() {
                self.internal_handle_mouse_event(Point {
                    x: x as i32,
                    y: y as i32
                });
            }

            // Dispatch events here in the bus

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
