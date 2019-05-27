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

use crate::core::callbacks::CallbackEvent;
use crate::widget::config::*;
use crate::widget::widget::*;

/// Creates a timer that can be used to generate callbacks based on a timeout.  When a timeout
/// has been reached, a `TimerTriggered` event is generated.  Set the timer timeout in
/// milliseconds by setting `CONFIG_TIMER_TIMEOUT` values.
///
/// Timer timeouts are triggered on a per-refresh basis, so if the FPS of the screen for the
/// main window is set to 30 FPS, the timer will only receive a tick every 1/30th of a second.  If
/// you need higher resolution timers, consider using threads.
pub struct TimerWidget {
    config: Configurable,
    enabled: bool,
    initiated: u64,
    triggered: bool,
}

fn time_ms() -> u64 {
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    (since_the_epoch.as_secs() * 1_000) + (since_the_epoch.subsec_nanos() / 1_000_000) as u64
}

impl TimerWidget {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            enabled: true,
            initiated: time_ms(),
            triggered: false,
        }
    }

    // Called to check the time since initiation, and call the timeout function when a timer has
    // been triggered.
    fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        let elapsed = time_ms() - self.initiated;

        if elapsed > self.config().get_numeric(CONFIG_TIMER_TIMEOUT) {
            self.initiated = time_ms();
            self.triggered = true;
        }
    }
}

impl Widget for TimerWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn is_invalidated(&mut self) -> bool {
        true
    }

    fn injects_events(&mut self) -> bool {
        true
    }

    fn inject_event(&mut self, widget_id: i32) -> Option<CallbackEvent> {
        if self.triggered {
            self.triggered = false;
            Some(CallbackEvent::TimerTriggered { widget_id })
        } else {
            None
        }
    }

    fn draw(&mut self, _context: Context, _graphics: &mut G2d, _clip: &DrawState) {
        self.tick();
    }
}
