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

/// Pushrod Event Mask type (`u32`).  Used to define an event mask.  Event masks are hexadecimal masks
/// identifying which events to listen for, in binary order.  When defining a new event that
/// is triggered by the OS, they should be defined in this list, with this type.
pub type EventMask = u32;

/// No events mask.
pub const MASK_EVENT_NONE: EventMask = 0x00000000;

/// Mouse movement event mask.
pub const MASK_EVENT_MOUSE_MOVED: EventMask = 0x00000001;

/// Mouse button press event mask.
pub const MASK_EVENT_MOUSE_DOWN: EventMask = 0x00000002;

/// Mouse button release event mask.
pub const MASK_EVENT_MOUSE_UP: EventMask = 0x00000004;

/// Mouse scroll event mask.
pub const MASK_EVENT_MOUSE_SCROLL: EventMask = 0x00000008;

/// All mouse events mask.
pub const MASK_EVENT_MOUSE_ALL: EventMask = 0x0000000F;

/// All events mask.  (Use this carefully)
pub const MASK_EVENT_ALL: EventMask = MASK_EVENT_MOUSE_ALL;

/// Enumeration types for different `PushrodEvents`, translated from the underlying OS.
pub enum PushrodEvent {
    /// Mouse event contains a point to where the mouse was moved.
    MouseEvent { point: Point },

    /// Contains the button that was pressed.
    MouseDownEvent { button: MouseButton },

    /// COntains the button that was released.
    MouseUpEvent { button: MouseButton },

    /// Contains the direction in which the mouse scroll event took place.  X movement in a
    /// positive direction indicates movement to the right, where negative is to the left.
    /// Y movement in a positive direction indicates downward movement, negative is upward.
    MouseScrollEvent { point: Point },
}

/// Implement this trait to register for system-wide events.  Only implement this if you plan
/// to implement event trackers on your own.  Make sure to specify the type of event mask
/// you wish to use, otherwise, you will receive all events as they occur.
///
/// Example:
/// ```
/// # use pushrod::event::event::*;
/// # use pushrod::core::point::*;
/// # struct EventListener { }
/// #
/// # impl EventListener {
/// #     fn new() -> Self {
/// #         Self { }
/// #     }
/// # }
/// #
/// impl PushrodEventListener for EventListener {
///     fn event_mask(&self) -> EventMask {
///         MASK_EVENT_MOUSE_MOVED
///     }
///
///     fn handle_event(&self, event: &PushrodEvent) {
///         match event {
///             PushrodEvent::MouseEvent { point: _point } => eprintln!("Mouse moved!"),
///             _ => (),
///         }
///     }
/// }
/// ```
///
/// Programmers who use this event system are encouraged to override `event_mask` so that
/// they only receive the events that pertain to your application.  If this is strictly set
/// to `MASK_EVENT_ALL`, all wrapped events will be sent to `handle_event`.
pub trait PushrodEventListener {
    /// Identifies which events to receive in the `handle_event` function.  Any events that
    /// do not match the masks (defined in constants) will not trigger a `handle_event` callback.
    fn event_mask(&self) -> EventMask {
        MASK_EVENT_ALL
    }

    /// Called when an event matching a masked type (in `event_mask`) occurs.
    fn handle_event(&self, event: &PushrodEvent);
}
