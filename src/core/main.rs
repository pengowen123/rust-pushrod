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

    pub fn next_window(&mut self) -> Option<Event> {
        let mut cur_window_position = self.window_position;

        cur_window_position += 1;

        if cur_window_position > self.windows.len() - 1 {
            cur_window_position = 0;
        }

        self.window_position = cur_window_position;
        self.windows[self.window_position].next()
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

    fn handle_mouse_event(&self) {

    }

    fn handle_window_event(&self) {

    }

    pub fn run(&self) {
        let mut gl: GlGraphics = GlGraphics::new(self.window_opengl);

        while let Some(event) = self.windows.borrow_mut().next_window() {
//            if let Some([x, y]) = event.mouse_cursor_args() {
//                let mut true_x = x;
//                let mut true_y = y;
//
//                if true_x > self.config.window_width as f64 {
//                    true_x = self.config.window_width as f64;
//                } else if true_x < 0.0 {
//                    true_x = 0.0;
//                }
//
//                if true_y > self.config.window_height as f64 {
//                    true_y = self.config.window_height as f64;
//                } else if true_y < 0.0 {
//                    true_y = 0.0;
//                }
//
//                println!("X: {} Y: {}", true_x as u32, true_y as u32);
//            }

            if let Some(args) = event.render_args() {
                gl.draw(args.viewport(), |_context, graphics| {
                    clear([1.0; 4], graphics);
                });
            }
        }
    }
}
