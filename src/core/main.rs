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

struct Config {
    window_width: u32,
    window_height: u32,
    window_fps: u64,
}

impl Config {
    fn default() -> Self {
        Self {
            window_width: 1024,
            window_height: 768,
            window_fps: 60,
        }
    }
}

struct Pushrod {
    config: Config,
}

// Once threads are supported, and objects can be opened as separate windows,
// Pushrod should be able to run multiple instances of self.run() in a threaded
// loop.
impl Pushrod {
    fn new(config: Config) -> Self {
        Self {
            config,
        }
    }

    fn run(&self) {
        let opengl = OpenGL::V3_2;

        let mut window: PistonWindow = WindowSettings::new(
            "Pushrod Window",
            [self.config.window_width, self.config.window_height]
        )
            .opengl(opengl)
//            .samples(4)
//            .exit_on_esc(true)
//            .fullscreen(true)
            .build()
            .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

        window.set_max_fps(self.config.window_fps);
        window.set_ups(self.config.window_fps);

        let mut gl = GlGraphics::new(opengl);

        while let Some(event) = window.next() {
            if let Some(args) = event.render_args() {
                // Draw loop
//                self.handle_draw_loop(&gl);
            }
        }
    }
}