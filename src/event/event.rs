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

pub type PushrodEventMask = u32;

pub const PUSHROD_EVENT_NONE: PushrodEventMask        = 0x00000000;
pub const PUSHROD_EVENT_MOUSE_MOVED: PushrodEventMask = 0x00000001;

pub enum PushrodEvent {
    PushrodMouseEvent {
        point: Point,
    },
}

pub trait PushrodEventListener {
    fn event_mask(&self) -> PushrodEventMask;
    fn handle_event(&self, event: &PushrodEvent);
}

//pub type EventMask = u64;
//
//pub const EVENT_MOUSE_MOVEMENT: EventMask = 0x00000001;
//
//pub trait EventListener {
//    fn event_mask(&self) -> EventMask;
//    fn handle_event(&self, event: &Box<PushrodEvent>);
//}
//
//pub trait PushrodEvent {
//    fn match_mask(&self) -> EventMask;
//}
//
//pub struct EventMouseMovement {
//    pub point: Point,
//}
//
//impl EventMouseMovement {
//    pub fn new(point: Point) -> Self {
//        Self { point }
//    }
//}
//
//impl PushrodEvent for EventMouseMovement {
//    fn match_mask(&self) -> EventMask {
//        EVENT_MOUSE_MOVEMENT
//    }
//}
