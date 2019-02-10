// Window Container
// Contains a PistonWindow and a list of widgets
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

use crate::core::point::*;
use crate::widget::widget::*;

use piston_window::*;

/// This is a container object, used for storing the `Widget` trait object, and the parent-child
/// relationship for the added `Widget`.  Only the `widget` is public.
pub struct WidgetContainer {
    /// The `Widget` trait object being stored.
    pub widget: Box<dyn Widget>,

    /// This `Widget`'s assigned ID.
    widget_id: i32,

    /// The parent ID.
    parent_id: i32,
}

/// This structure contains a window and its corresponding onscreen widgets.  These objects
/// are stored in the Pushrod main loop.
pub struct PushrodWindow {
    /// A `piston_window::PistonWindow` object.
    pub window: PistonWindow,

    /// A vector list of Boxed `PushrodWidget` trait objects.
    pub widgets: Vec<WidgetContainer>,
}

/// Implementation for a new `PushrodWindow`.  When a new `PushrodWindow` is added to the
/// managed window stack in the Pushrod main loop, this object is created to store its
/// components.
impl PushrodWindow {
    /// Constructor, takes a managed `PistonWindow` from the `piston_window` crate.  Adds a top-level
    /// widget to the list that is a white container widget.  This is the base for all other widgets
    /// tht will be added to the window.
    pub fn new(window: PistonWindow) -> Self {
        let mut widgets_list: Vec<WidgetContainer> = Vec::new();
        let mut base_widget = BaseWidget::new();

        base_widget.set_size(800, 600);
        widgets_list.push(WidgetContainer {
            widget: Box::new(base_widget),
            widget_id: 0,
            parent_id: 0,
        });

        Self {
            window,
            widgets: widgets_list,
        }
    }

    /// Adds a UI `Widget` to this window.  `Widget` objects that are added using this method will
    /// be part of the base widget (`id = 0`), and will be force-redrawn when the parent is
    /// invalidated.
    ///
    /// After adding a widget, the ID of the widget is returned.
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) -> i32 {
        let widget_size = self.widgets.len() as i32;

        self.widgets.push(WidgetContainer {
            widget,
            widget_id: widget_size,
            parent_id: 0,
        });

        widget_size
    }

    /// Adds a UI `Widget` to the parent of a window, specified by the `parent_id`.  The `parent_id`
    /// must be an object that already exists in the stack.
    ///
    /// After adding a widget, the ID of the widget is returned.
    pub fn add_widget_to_parent(&mut self, widget: Box<dyn Widget>, parent_id: i32) -> i32 {
        // TODO Validate parent_id
        let widget_size = self.widgets.len() as i32;

        self.widgets.push(WidgetContainer { widget, widget_id: widget_size, parent_id });

        widget_size
    }

    /// Retrieves the parent of the widget requested.  Parent of 0 or -1 will always return 0.
    pub fn get_parent_of(&mut self, widget_id: i32) -> i32 {
        if widget_id <= 0 {
            0
        } else {
            self.widgets[widget_id as usize].parent_id
        }
    }

    /// Retrieves a list of all of the child IDs that list the `parent_id` as its parent.  This
    /// can be used recursively to determine the widget ownership tree, or the redraw order in which
    /// repaint should take place.
    pub fn get_children_of(&self, parent_id: i32) -> Vec<i32> {
        let mut return_list = vec![];

        for (pos, obj) in self
            .widgets
            .iter()
            .enumerate()
        {
            if obj.parent_id == parent_id {
                return_list.push(pos as i32);
            }
        }

        return_list
    }

    /// Retrieves a `PushrodWidget` ID for a specified `Point`.  If no ID could be found,
    /// defaults to a -1.
    pub fn get_widget_id_for_point(&mut self, point: Point) -> i32 {
        let mut found_id: i32 = -1;

        for (pos, obj) in self.widgets.iter_mut().enumerate() {
            let widget_point = &obj.widget.get_origin();
            let widget_size: crate::core::point::Size = obj.widget.get_size();

            if point.x >= widget_point.x
                && point.x <= widget_point.x + widget_size.w
                && point.y >= widget_point.y
                && point.y <= widget_point.y + widget_size.h
            {
                found_id = pos as i32;
            }
        }

        found_id
    }

    /// Callback to `mouse_entered` for a `Widget` by ID.
    pub fn mouse_entered_for_id(&mut self, id: i32) {
        &self.widgets[id as usize].widget.mouse_entered(id);
    }

    /// Callback to `mouse_exited` for a `Widget` by ID.
    pub fn mouse_exited_for_id(&mut self, id: i32) {
        &self.widgets[id as usize].widget.mouse_exited(id);
    }

    /// Callback to `mouse_scrolled` for a `Widget` by ID, with the mouse scroll `Point`.
    pub fn mouse_scrolled_for_id(&mut self, id: i32, point: Point) {
        &self.widgets[id as usize].widget.mouse_scrolled(id, point);
    }

    /// Retrieves a reference to the `Box`ed `Widget` object by its ID.
    pub fn get_widget_for_id(&mut self, id: i32) -> &Box<dyn Widget> {
        &self.widgets[id as usize].widget
    }
}
