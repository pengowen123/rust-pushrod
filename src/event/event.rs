// Events
//
// Scale factor is not taken into account here, as the widget
// does not need to be aware of the screen scale.  That should
// be up to the hardware to take care of that.
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
use piston_window::*;

/// Pushrod Event Mask type.
pub type PushrodEventMask = u32;

/// No events mask.
pub const PUSHROD_EVENT_NONE: PushrodEventMask = 0x00000000;

/// Mouse movement event mask.
pub const PUSHROD_EVENT_MOUSE_MOVED: PushrodEventMask = 0x00000001;

/// Mouse button press event mask.
pub const PUSHROD_EVENT_MOUSE_DOWN: PushrodEventMask = 0x00000002;

/// Mouse button release event mask.
pub const PUSHROD_EVENT_MOUSE_UP: PushrodEventMask = 0x00000004;

/// Mouse scroll event mask.
pub const PUSHROD_EVENT_MOUSE_SCROLL: PushrodEventMask = 0x00000008;

/// All mouse events mask.
pub const PUSHROD_EVENT_MOUSE_ALL: PushrodEventMask = 0x0000000F;

/// All events mask.
pub const PUSHROD_EVENT_ALL: PushrodEventMask = PUSHROD_EVENT_MOUSE_ALL;

/// Enumerations for different `PushrodEvents`, translated from the underlying OS.
pub enum PushrodEvent {
    MouseEvent { point: Point },
    MouseDownEvent { button: MouseButton },
    MouseUpEvent { button: MouseButton },
    MouseScrollEvent { point: Point },
}

/// Implement this trait to register for system-wide events.  Only implement this if you plan
/// to implement event trackers on your own.  Make sure to specify the type of event mask
/// you wish to use, otherwise, you will receive a duplicate of all events that have occurred.
pub trait PushrodEventListener {
    fn event_mask(&self) -> PushrodEventMask {
        PUSHROD_EVENT_ALL
    }

    fn handle_event(&self, event: &PushrodEvent);
}
