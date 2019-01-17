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
use pushrod::core::point::*;
use pushrod::core::window::*;
use pushrod::event::event::*;

struct TestMouseListener {}

impl TestMouseListener {
    fn new() -> Self {
        Self {}
    }

    fn handle_mouse_move(&self, point: &Point) {
        println!("X={} Y={}", point.x, point.y);
    }

    fn handle_mouse_down(&self, button: &MouseButton) {
        match button {
            MouseButton::Left => println!("Left mouse button pressed."),
            _ => println!("Other mouse button pressed."),
        }
    }

    fn handle_mouse_up(&self, button: &MouseButton) {
        match button {
            MouseButton::Left => println!("Left mouse button released."),
            _ => println!("Other mouse button released."),
        }
    }

    fn handle_mouse_scroll(&self, point: &Point) {
        println!("Scroll: X={} Y={}", point.x, point.y);
    }
}

impl PushrodEventListener for TestMouseListener {
    fn event_mask(&self) -> PushrodEventMask {
        PUSHROD_EVENT_MOUSE_ALL
    }

    fn handle_event(&self, event: &PushrodEvent) {
        match event {
            PushrodEvent::MouseEvent { point } => self.handle_mouse_move(&point),
            PushrodEvent::MouseDownEvent { button } => self.handle_mouse_down(&button),
            PushrodEvent::MouseUpEvent { button } => self.handle_mouse_up(&button),
            PushrodEvent::MouseScrollEvent { point } => self.handle_mouse_scroll(&point),
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let prod: Pushrod = Pushrod::new(opengl);
    let mut pushrod_window: PushrodWindow = PushrodWindow::new(
        WindowSettings::new("Pushrod Window", [640, 480])
            .opengl(opengl)
            .build()
            .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)),
    );

    pushrod_window.window.set_max_fps(60);
    pushrod_window.window.set_ups(60);

    // Adds a window to the stack of watched events
    prod.add_window(pushrod_window);
    prod.add_event_listener_for_window(Box::new(TestMouseListener::new()));

    // Runs the main event loop
    prod.run();
}
