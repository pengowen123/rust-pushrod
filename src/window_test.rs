// Window Test
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

use pushrod::core::main::*;
use pushrod::event::event::*;
use piston_window::*;

struct TestMouseListener { }

impl EventListener for TestMouseListener {
    fn new() -> TestMouseListener {
        TestMouseListener { }
    }

    fn event_mask(&self) -> EventMask {
        EVENT_MOUSE_MOVEMENT
    }

    fn handle_event(&self) {
        println(!"Got a mouse movement event!");
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    println!("Starting Test.");

    let prod: Pushrod = Pushrod::new(opengl);

    let mut window: PistonWindow = WindowSettings::new(
        "Pushrod Window",
        [640, 480]
    )
        .opengl(opengl)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

    window.set_max_fps(60);
    window.set_ups(60);

    // Adds a window to the stack of watched events
    prod.add_window(window);
    prod.add_event_listener_for_window(TestMouseListener::new());

    // Runs the main event loop
    prod.run();
}