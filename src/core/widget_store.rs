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

use piston_window::*;

use std::cell::RefCell;

use crate::core::callbacks::CallbackEvent;
use crate::core::point::*;
use crate::widget::widget::*;

/// This is a container object, used for storing the `Widget` trait object, and the parent
/// relationship for the added `Widget`.  Only the `widget` is public.  `Widget` objects do not
/// need to have a child relationship, only parent objects are traversed.  A parent object of 0, or
/// itself, indicates that the parent is self.
pub struct WidgetContainer {
    /// The `Widget` trait object being stored.
    pub widget: RefCell<Box<dyn Widget>>,

    /// This is the `Widget`'s assigned name.  These IDs are assigned at the time they are added
    /// to the `WidgetStore`.
    widget_name: String,

    /// This `Widget`'s assigned ID.  These IDs are auto-assigned.
    widget_id: i32,

    /// The parent ID.
    parent_id: i32,
}

/// This is the `WidgetStore`, which is used to store `Widget` objects for a `Pushrod`
/// management object.
pub struct WidgetStore {
    /// A vector list of `WidgetContainer` objects.
    pub widgets: Vec<WidgetContainer>,
}

/// Implementation of the `WidgetStore`.
impl WidgetStore {
    /// Creates a new `WidgetStore`.
    pub fn new() -> Self {
        let mut widgets_list: Vec<WidgetContainer> = Vec::new();
        let mut base_widget = CanvasWidget::new();

        base_widget.set_size(800, 600);
        widgets_list.push(WidgetContainer {
            widget: RefCell::new(Box::new(base_widget)),
            widget_name: String::from("_WidgetStoreBase"),
            widget_id: 0,
            parent_id: 0,
        });

        Self {
            widgets: widgets_list,
        }
    }

    /// Invalidates all widgets in the window.  This is used to force a complete refresh of the
    /// window's contents, usually based on a timer expiration, or a window resize.  Use with
    /// care, as this is an expensive operation.
    pub fn invalidate_all_widgets(&mut self) {
        self.widgets
            .iter_mut()
            .for_each(|x| x.widget.borrow_mut().invalidate());
    }

    /// Indicates whether or not any `Widget`s in the `WidgetStore` have been invalidated and need
    /// to be repainted.
    pub fn needs_repaint(&mut self) -> bool {
        self.widgets
            .iter_mut()
            .map(|x| x.widget.borrow_mut().is_invalidated())
            .find(|x| x == &true)
            .unwrap_or(false)
    }

    /// Adds a UI `Widget` to this window.  `Widget` objects that are added using this method will
    /// be part of the base widget (`id = 0`), and will be force-redrawn when the parent is
    /// invalidated.
    ///
    /// After adding a widget, the ID of the widget is returned.
    pub fn add_widget(&mut self, name: &str, widget: Box<dyn Widget>) -> i32 {
        let widget_size = self.widgets.len() as i32;

        self.widgets.push(WidgetContainer {
            widget: RefCell::new(widget),
            widget_name: String::from(name),
            widget_id: widget_size,
            parent_id: 0,
        });

        widget_size
    }

    /// Adds a UI `Widget` to the parent of a window, specified by the `parent_id`.  The `parent_id`
    /// must be an object that already exists in the stack.
    ///
    /// After adding a widget, the ID of the widget is returned.
    pub fn add_widget_to_parent(&mut self, name: &str, widget: Box<dyn Widget>, parent_id: i32) -> i32 {
        // TODO Validate parent_id
        let widget_size = self.widgets.len() as i32;

        self.widgets.push(WidgetContainer {
            widget: RefCell::new(widget),
            widget_name: String::from(name),
            widget_id: widget_size,
            parent_id,
        });

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
        self.widgets
            .iter()
            .filter(|x| x.parent_id == parent_id)
            .map(|x| x.widget_id)
            .collect()
    }

    /// Retrieves a `PushrodWidget` ID for a specified `Point`.  If no ID could be found,
    /// defaults to a -1.
    pub fn get_widget_id_for_point(&mut self, point: Point) -> i32 {
        let mut found_id = -1;

        for (pos, obj) in self.widgets.iter_mut().enumerate() {
            let widget_point = &obj.widget.borrow_mut().get_origin();
            let widget_size: crate::core::point::Size = obj.widget.borrow_mut().get_size();

            // Skip over item widgets that have a width and height of 0.
            if widget_size.w > 0 && widget_size.h > 0 {
                if point.x >= widget_point.x
                    && point.x <= widget_point.x + widget_size.w
                    && point.y >= widget_point.y
                    && point.y <= widget_point.y + widget_size.h
                {
                    found_id = pos as i32;
                }
            }
        }

        found_id
    }

    /// Returns the name of the widget by its ID.
    pub fn get_name_for_widget_id(&mut self, widget_id: i32) -> &str {
        self.widgets[widget_id as usize]
            .widget_name
            .as_str()
    }

    /// Handles event messages, returning an event if provided by the `Widget`.
    pub fn handle_event(&mut self, widget_id: i32, event: CallbackEvent) -> Option<CallbackEvent> {
        self.widgets[widget_id as usize]
            .widget
            .borrow_mut()
            .handle_event(event)
    }

    pub fn set_color(&mut self, widget_id: i32, color: types::Color) {
        self.widgets[widget_id as usize]
            .widget
            .borrow_mut()
            .set_color(color);
    }

    /// Recursive draw object: paints objects in order of appearance on the screen.  This does not
    /// account for object depth, but it is implied that objects' parents are displayed in stacking
    /// order.  Therefore, the parent is drawn first, then sibling, and other siblings.  This draw
    /// function is used by the `Pushrod` main loop, and is meant to be called in a `draw_2d`
    /// closure.
    pub fn draw(&mut self, widget_id: i32, c: Context, g: &mut G2d) {
        let parents_of_widget = self.get_children_of(widget_id);

        if parents_of_widget.len() == 0 {
            return;
        }

        for pos in 0..parents_of_widget.len() {
            c.reset();

            let paint_id = parents_of_widget[pos];
            let paint_widget = &mut self.widgets[paint_id as usize];

            if &paint_widget.widget.borrow_mut().is_invalidated() == &true {
                let origin: Point = paint_widget.widget.borrow_mut().get_origin().clone();
                let size: crate::core::point::Size =
                    paint_widget.widget.borrow_mut().get_size().clone();

                let new_context: Context = Context {
                    viewport: c.viewport,
                    view: c.view,
                    transform: c.transform.trans(origin.x as f64, origin.y as f64),
                    draw_state: c.draw_state,
                };

                let clip: DrawState = c.draw_state.scissor([
                    origin.x as u32 * 2,
                    origin.y as u32 * 2,
                    size.w as u32 * 2,
                    size.h as u32 * 2,
                ]);

                &paint_widget.widget.borrow_mut().draw(new_context, g, &clip);
            }

            if parents_of_widget[pos] != widget_id {
                self.draw(paint_id, c, g);
            }
        }
    }

    /// Retrieves a widget by the name when the widget was added.  To get the very top-level
    /// widget, refer to `_WidgetStoreBase`.
    pub fn get_widget_for_name(&mut self, name: &str) -> &RefCell<Box<dyn Widget>> {
        let widget_id = match self.widgets
            .iter_mut()
            .find(|x| x.widget_name == String::from(name)) {
            Some(x) => x.widget_id,
            None => 0,
        };

        self.get_widget_for_id(widget_id)
    }

    /// Retrieves a reference to the `Box`ed `Widget` object by its ID.  To get the very top-level
    /// widget, specify ID 0.
    pub fn get_widget_for_id(&mut self, id: i32) -> &RefCell<Box<dyn Widget>> {
        &self.widgets[id as usize].widget
    }
}
