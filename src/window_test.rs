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

use piston_window::*;
use pushrod::core::main::*;
use pushrod::event::event::*;

struct TestMouseListener {}

impl TestMouseListener {
    fn new() -> Self {
        Self {}
    }
}

impl PushrodEventListener for TestMouseListener {
    fn event_mask(&self) -> PushrodEventMask {
        PUSHROD_EVENT_MOUSE_MOVED
            | PUSHROD_EVENT_MOUSE_DOWN
            | PUSHROD_EVENT_MOUSE_UP
            | PUSHROD_EVENT_MOUSE_SCROLL
    }

    fn handle_event(&self, event: &PushrodEvent) {
        match event {
            PushrodEvent::PushrodMouseEvent { point } => {
                println!("[TEST CALLBACK] X={} Y={}", point.x, point.y)
            }
            PushrodEvent::PushrodMouseDownEvent { button } => match button {
                MouseButton::Left => println!("[TEST CALLBACK] Left mouse button pressed."),
                _ => (),
            },
            PushrodEvent::PushrodMouseUpEvent { button } => match button {
                MouseButton::Left => println!("[TEST CALLBACK] Left mouse button released."),
                _ => (),
            },
            PushrodEvent::PushrodMouseScrollEvent { point } => {
                println!("[TEST CALLBACK] Scroll X={} Y={}", point.x, point.y);
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    println!("Starting Test.");

    let prod: Pushrod = Pushrod::new(opengl);

    let mut window: PistonWindow = WindowSettings::new("Pushrod Window", [640, 480])
        .opengl(opengl)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

    window.set_max_fps(60);
    window.set_ups(60);

    // Adds a window to the stack of watched events
    prod.add_window(window);
    prod.add_event_listener_for_window(Box::new(TestMouseListener::new()));

    // Runs the main event loop
    prod.run();
}
