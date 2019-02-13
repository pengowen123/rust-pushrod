// Timer Widget
// Timer-based widget that fires off a callback every time a certain time period is reached.
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

use opengl_graphics::GlGraphics;
use piston_window::*;
//use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::point::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// This is the `TimerWidget`.  It contains no base widget, it only contains a start and end
/// time,
pub struct TimerWidget {
    config: Configurable,
    enabled: bool,
    _initiated: u128,
    timeout: u128,
}

/// Implementation of the constructor for the `TimerWidget`.  Timer widgets are not accessible
/// on the screen, so they have an origin of 0x0 and width of 0x0.
impl TimerWidget {
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            enabled: true,
            _initiated: 0,
            timeout: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.enabled {
            //            let cur_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_millis();
            //            let elapsed = cur_time - self.initiated;
            //
            //            eprintln!("Elapsed: {}", elapsed);
            //
            //            if elapsed > self.timeout {
            //                eprintln!("Timeout!");
            //                self.initiated = cur_time;
            //            }
            //
            //            eprintln!("Tick!");
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_timeout(&mut self, timeout: u128) {
        self.timeout = timeout;
    }
}

/// Implementation of the `TimerWidget` object with the `Widget` traits implemented.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::core::window::*;
/// # use pushrod::widget::widget::*;
/// # use pushrod::widget::timer_widget::*;
/// # fn main() {
/// #   let opengl = OpenGL::V3_2;
/// #   let mut pushrod_window: PushrodWindow = PushrodWindow::new(
/// #       WindowSettings::new("Pushrod Window", [640, 480])
/// #           .opengl(opengl)
/// #           .build()
/// #           .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)),
/// #   );
/// #
///    let mut timer_widget = TimerWidget::new();
///
///    // (OR)
///
/// # }
/// ```
impl Widget for TimerWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn is_invalidated(&mut self) -> bool {
        true
    }

    fn get_origin(&mut self) -> Point {
        make_origin_point()
    }

    fn get_size(&mut self) -> crate::core::point::Size {
        make_unsized()
    }

    fn mouse_entered(&mut self, _widget_id: i32) {}

    fn mouse_exited(&mut self, _widget_id: i32) {}

    fn mouse_scrolled(&mut self, _widget_id: i32, _point: Point) {}

    /// Draws the contents of the widget in this order:
    ///
    /// - Base widget first
    /// - Box graphic for the specified width
    fn draw(&mut self, _context: Context, _graphics: &mut GlGraphics) {
        self.tick();
    }
}
