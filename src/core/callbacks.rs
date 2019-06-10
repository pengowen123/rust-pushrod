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
use piston::input::*;

/// This is the callback event class that is used to handle events that are produced when a widget
/// is interacted with in the Pushrod Run Loop.  This callback is triggered when an event happens
/// either in the main GUI, or within a GUI object.
pub trait PushrodCallbackEvents {
    fn handle_event(&mut self, event: CallbackEvent, widget_store: &mut WidgetStore) {
        match event {
            CallbackEvent::MouseEntered { widget_id } => {
                self.mouse_entered(widget_id, widget_store)
            }
            CallbackEvent::MouseExited { widget_id } => self.mouse_exited(widget_id, widget_store),
            CallbackEvent::MouseScrolled { widget_id, point } => {
                self.mouse_scrolled(widget_id, point, widget_store)
            }
            CallbackEvent::MouseMoved { widget_id, point } => {
                self.mouse_moved(widget_id, point, widget_store)
            }
            CallbackEvent::KeyPressed {
                widget_id,
                key,
                state,
            } => self.key_pressed(widget_id, key, state, widget_store),
            CallbackEvent::WindowResized { size } => self.window_resized(size, widget_store),
            CallbackEvent::WindowFocused { flag } => self.window_focused(flag, widget_store),
            CallbackEvent::MouseButtonDown { widget_id, button } => {
                self.mouse_button_down(widget_id, button, widget_store)
            }
            CallbackEvent::MouseButtonUpInside { widget_id, button } => {
                self.mouse_button_up_inside(widget_id, button, widget_store)
            }
            CallbackEvent::MouseButtonUpOutside { widget_id, button } => {
                self.mouse_button_up_outside(widget_id, button, widget_store)
            }
            CallbackEvent::WidgetClicked { widget_id, button } => {
                self.widget_clicked(widget_id, button, widget_store)
            }
            CallbackEvent::WidgetSelected {
                widget_id,
                button,
                selected,
            } => self.widget_selected(widget_id, button, selected, widget_store),
            CallbackEvent::TimerTriggered { widget_id } => {
                self.timer_triggered(widget_id, widget_store)
            }
            CallbackEvent::UnselectRadioButtons {
                widget_id,
                group_id,
            } => self.unselect_radio_buttons(widget_id, group_id, widget_store),
            _ => (),
        }
    }

    fn mouse_entered(&mut self, widget_id: i32, widget_store: &mut WidgetStore) {}

    fn mouse_exited(&mut self, widget_id: i32, widget_store: &mut WidgetStore) {}

    fn mouse_scrolled(&mut self, widget_id: i32, point: Point, widget_store: &mut WidgetStore) {}

    fn mouse_moved(&mut self, widget_id: i32, point: Point, widget_store: &mut WidgetStore) {}

    fn key_pressed(
        &mut self,
        widget_id: i32,
        key: Key,
        state: ButtonState,
        widget_store: &mut WidgetStore,
    ) {
    }

    fn window_resized(&mut self, size: crate::core::point::Size, widget_store: &mut WidgetStore) {}

    fn window_focused(&mut self, flag: bool, widget_store: &mut WidgetStore) {}

    fn mouse_button_down(
        &mut self,
        widget_id: i32,
        button: Button,
        widget_store: &mut WidgetStore,
    ) {
    }

    fn mouse_button_up_inside(
        &mut self,
        widget_id: i32,
        button: Button,
        widget_store: &mut WidgetStore,
    ) {
    }

    fn mouse_button_up_outside(
        &mut self,
        widget_id: i32,
        button: Button,
        widget_store: &mut WidgetStore,
    ) {
    }

    fn widget_clicked(&mut self, widget_id: i32, button: Button, widget_store: &mut WidgetStore) {}

    fn widget_selected(
        &mut self,
        widget_id: i32,
        button: Button,
        selected: bool,
        widget_store: &mut WidgetStore,
    ) {
    }

    fn timer_triggered(&mut self, widget_id: i32, widget_store: &mut WidgetStore) {}

    fn unselect_radio_buttons(
        &mut self,
        widget_id: i32,
        group_id: i32,
        widget_store: &mut WidgetStore,
    ) {
    }
}

/// These are the different types of events that can be triggered.  Any other callback events
/// should be extended in this enum definition.
#[derive(Clone, Debug)]
pub enum CallbackEvent {
    /// Indicates a mouse entered the bounds of a `Widget`.  Contains the ID of the `Widget` that was
    /// affected.
    MouseEntered { widget_id: i32 },

    /// Indicates a mouse exited the bounds of a `Widget`.  Contains the ID of the `Widget` that was
    /// affected.
    MouseExited { widget_id: i32 },

    /// Indicates that the scroll wheel was moved inside a `Widget`.  Contains the ID of the
    /// `Widget` that had the mouse scroll action, and the point in the direction of the scroll,
    /// along with the amount of points the mouse scroll moved.
    MouseScrolled { widget_id: i32, point: Point },

    /// Indicates that a mouse moved within the bounds of a `Widget`.  Contains the ID of the
    /// `Widget` that was affected.
    MouseMoved { widget_id: i32, point: Point },

    /// Indicates that a keyboard key was pressed/released inside the bounds of a `Widget`.  Contains
    /// the ID of the `Widget` that received the keypress, along with the `Key` value, and any
    /// associated `Button` modifier states.
    KeyPressed {
        widget_id: i32,
        key: Key,
        state: ButtonState,
    },

    /// Indicates that the main application window was resized.  Contains the `Size` of the new
    /// bounds.
    WindowResized { size: crate::core::point::Size },

    /// Indicates whether or not focus was gained or lost for the main application.  Contains a
    /// boolean flag indicating focus: `true` is focused, `false` if lost.
    WindowFocused { flag: bool },

    /// Indicates that a mouse button was pressed within the bounds of a `Widget`.  Contains the
    /// ID of the `Widget`, along with the `Button` that was clicked.
    MouseButtonDown { widget_id: i32, button: Button },

    /// Indicates that a mouse button was released within the bounds of a `Widget`.  Contains
    /// the ID of the `Widget`, along with the `Button` that was released.
    MouseButtonUpInside { widget_id: i32, button: Button },

    /// Indicates that a mouse button was released outside of the bounds of a `Widget`.  Contains
    /// the ID of the `Widget`, along with the `Button` that was released.
    MouseButtonUpOutside { widget_id: i32, button: Button },

    /// Indicates that a mouse button triggered a click action within a `Widget`.  Contains the
    /// ID of the `Widget`, along with the `Button` that was used to indicate the click action.
    WidgetClicked { widget_id: i32, button: Button },

    /// Indicates that a `Widget`'s selected state has been toggled.  Contains the ID of the
    /// `Widget` that was toggled, along with the mouse `Button` that was clicked, and the
    /// final `selected` state of the widget.
    WidgetSelected {
        widget_id: i32,
        button: Button,
        selected: bool,
    },

    /// Indicates that a timer timeout has been triggered.  Contains the ID of the `Widget` that was
    /// affected.
    TimerTriggered { widget_id: i32 },

    /// Indicates that a group of `RadioButtonWidget` objects has been deselected.  Contains the
    /// ID of the `Widget` that was unselected, along with the group ID.
    UnselectRadioButtons { widget_id: i32, group_id: i32 },
}
