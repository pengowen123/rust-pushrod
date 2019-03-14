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

use piston_window::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::callbacks::*;
use crate::core::point::*;
use crate::widget::config::*;
use crate::widget::widget::*;

pub const CALLBACK_TIMER: u32 = 100;

/// This is the `TimerWidget`.  It contains no base widget, it only contains a start and end
/// time,
pub struct TimerWidget {
    config: Configurable,
    callbacks: CallbackStore,
    enabled: bool,
    initiated: u64,
    timeout: u64,
}

/// Helper function that returns the current time in milliseconds since the `UNIX_EPOCH`.  This
/// function is the equivalent of a `System.currentTimeMillis()` in Java.
fn time_ms() -> u64 {
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    (since_the_epoch.as_secs() * 1_000) + (since_the_epoch.subsec_nanos() / 1_000_000) as u64
}

/// Implementation of the constructor for the `TimerWidget`.  Timer widgets are not accessible
/// on the screen, so they have an origin of 0x0 and width of 0x0.
///
/// The timer provides a simple way to call a callback function after a certain amount of time
/// has passed.  Upon instantiation, the timer is enabled.
///
/// Here are a few limitations of the timer as it currently stands:
///
/// - Timer cannot be paused; it is enabled or disabled, and the timer resets when enabled.
/// - Timer is called when the screen refreshes, so slower FPS settings will affect the timer.
impl TimerWidget {
    /// Constructor, creates a new `TimerWidget` struct with an empty timeout function.
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            callbacks: CallbackStore::new(),
            enabled: true,
            initiated: time_ms(),
            timeout: 0,
        }
    }

    // Called to check the time since initiation, and call the timeout function when a timer has
    // been triggered.
    fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        let elapsed = time_ms() - self.initiated;

        if elapsed > self.timeout {
            self.initiated = time_ms();
            self.timeout();
        }
    }

    /// Enables or disables the timer.  When disabled, the timer will not initiate the callback
    /// function.  When re-enabled, the initiation time resets, so the timer will reset back to
    /// zero, effectively resetting the entire timer.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.initiated = time_ms();
    }

    /// Sets the closure function for the timer when a timeout has been triggered.  This closure
    /// needs to be `Boxed`.
    pub fn on_timeout(&mut self, callback: BlankCallback) {
        self.callbacks().put(
            CALLBACK_TIMER,
            CallbackTypes::BlankCallback { callback },
        );
    }

    /// Calls the timeout function.
    fn timeout(&mut self) {
        match self.callbacks().get(CALLBACK_TIMER) {
            CallbackTypes::BlankCallback { callback } => callback(),
            _ => (),
        }
    }

    /// Sets the timeout in milliseconds for this timer.  Will trigger a call to the function
    /// set in `on_timeout` when triggered, and will continue to call that function until this
    /// timer is disabled by using `self.set_enabled(false)`.
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }
}

/// Implementation of the `TimerWidget` object with the `Widget` traits implemented.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::widget::widget::*;
/// # use pushrod::widget::timer_widget::*;
/// # fn main() {
///    let mut timer_widget = TimerWidget::new();
///
///    timer_widget.set_timeout(60000);
///    timer_widget.on_timeout(Box::new( || eprintln!("Timer triggered.") ));
///    timer_widget.set_enabled(true);
/// # }
/// ```
impl Widget for TimerWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn callbacks(&mut self) -> &mut CallbackStore {
        &mut self.callbacks
    }

    /// Timer is always invalidated, this way, the tick function is always called on each
    /// screen refresh.
    fn is_invalidated(&mut self) -> bool {
        true
    }

    /// Origin is always set to X/Y at points 0x0.
    fn get_origin(&mut self) -> Point {
        make_origin_point()
    }

    /// Size is always unsized, as timers are invisible.
    fn get_size(&mut self) -> crate::core::point::Size {
        make_unsized()
    }

    /// Does not draw anything - only calls the timer `tick()` function to increment the
    /// timer.
    fn draw(&mut self, _context: Context, _graphics: &mut G2d) {
        self.tick();
    }
}
