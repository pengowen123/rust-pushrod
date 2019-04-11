// Callback Event System
// Callback Events
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
use crate::core::widget_store::*;

use piston_window::*;

/// This is the callback event class that is used to handle events that are produced when a widget
/// is interacted with in the Pushrod Run Loop.
pub trait PushrodCallbackEvents {
    fn handle_event(&mut self, event: CallbackEvent, widget_store: &mut WidgetStore);
}

/// These are the different types of events that can be triggered.  Any other callback events
/// should be extended in this enum definition.
#[derive(Clone, Debug)]
pub enum CallbackEvent {
    MouseEntered {
        widget_id: i32,
    },
    MouseExited {
        widget_id: i32,
    },
    MouseScrolled {
        widget_id: i32,
        point: Point,
    },
    MouseMoved {
        widget_id: i32,
        point: Point,
    },
    KeyPressed {
        widget_id: i32,
        key: Key,
        state: ButtonState,
    },
    WindowResized {
        size: crate::core::point::Size,
    },
    WindowFocused {
        flag: bool,
    },
    MouseButtonDown {
        widget_id: i32,
        button: Button,
    },
    MouseButtonUpInside {
        widget_id: i32,
        button: Button,
    },
    MouseButtonUpOutside {
        widget_id: i32,
        button: Button,
    },

    WidgetClicked {
        widget_id: i32,
        button: Button,
    },
    TimerTriggered {
        widget_id: i32,
    }
}
