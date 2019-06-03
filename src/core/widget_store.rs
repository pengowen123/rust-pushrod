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

use gl::types::GLuint;
use opengl_graphics::GlGraphics;
use std::cell::RefCell;

use crate::core::callbacks::CallbackEvent;
use crate::core::drawing_texture::DrawingTexture;
use crate::core::point::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// This is a container object that stores a `Widget`, assigns a name, a `Widget` ID, and its
/// parent.
pub struct WidgetContainer {
    /// The `Widget` being stored.
    pub widget: RefCell<Box<dyn Widget>>,
    pub drawing_texture: RefCell<DrawingTexture>,

    widget_name: String,

    /// The `Widget`'s automatically assigned ID.
    pub widget_id: i32,

    parent_id: i32,
}

/// This is the `WidgetStore`, which contains a list of `Widget` objects for a GUI window.
pub struct WidgetStore {
    pub widgets: Vec<WidgetContainer>,
}

impl WidgetStore {
    /// Constructor, creates a new `WidgetStore`, assigning a top-level `CanvasWidget` as the
    /// very top-level widget.  All `Widget` objects added will be a parent to this `Widget`,
    /// which is stored at ID 0.  If this `Widget` object ever becomes invalidated, the entire
    /// window is force refreshed.
    pub fn new() -> Self {
        let mut widgets_list: Vec<WidgetContainer> = Vec::new();
        let mut base_widget = CanvasWidget::new();

        base_widget.config().set(
            CONFIG_BODY_SIZE,
            Config::Size(crate::core::point::Size { w: 800, h: 600 }),
        );
        widgets_list.push(WidgetContainer {
            widget: RefCell::new(Box::new(base_widget)),
            drawing_texture: RefCell::new(DrawingTexture::new()),
            widget_name: String::from("_WidgetStoreBase"),
            widget_id: 0,
            parent_id: 0,
        });

        Self {
            widgets: widgets_list,
        }
    }

    /// Invalidates all `Widget`s in the GUI stack, forcing a redraw.
    pub fn invalidate_all_widgets(&mut self) {
        self.widgets
            .iter_mut()
            .for_each(|x| x.widget.borrow_mut().invalidate());
    }

    /// Indicates whether or not a widget in the store has been invalidated.
    pub fn needs_repaint(&mut self) -> bool {
        self.widgets
            .iter_mut()
            .map(|x| x.widget.borrow_mut().is_invalidated())
            .find(|x| x == &true)
            .unwrap_or(false)
    }

    /// Adds a `Widget` to the stack by name.
    pub fn add_widget(&mut self, name: &str, widget: Box<dyn Widget>) -> i32 {
        let widget_size = self.widgets.len() as i32;
        let container = WidgetContainer {
            widget: RefCell::new(widget),
            drawing_texture: RefCell::new(DrawingTexture::new()),
            widget_name: String::from(name),
            widget_id: widget_size,
            parent_id: 0,
        };

        // #117 - assigns widget ID to itself
        container
            .widget
            .borrow_mut()
            .config()
            .set_numeric(CONFIG_WIDGET_ID, widget_size as u64);

        self.widgets.push(container);

        widget_size
    }

    /// Adds a `Widget` object to the parent specified by ID.
    pub fn add_widget_to_parent(
        &mut self,
        name: &str,
        widget: Box<dyn Widget>,
        parent_id: i32,
    ) -> i32 {
        // TODO Validate parent_id
        let widget_size = self.widgets.len() as i32;
        let container = WidgetContainer {
            widget: RefCell::new(widget),
            drawing_texture: RefCell::new(DrawingTexture::new()),
            widget_name: String::from(name),
            widget_id: widget_size,
            parent_id,
        };

        // #117 - assigns widget ID to itself
        container
            .widget
            .borrow_mut()
            .config()
            .set_numeric(CONFIG_WIDGET_ID, widget_size as u64);

        self.widgets.push(container);

        widget_size
    }

    /// Gets the parent of the child `Widget` by ID.  If the child has no assigned parent, the
    /// top-level `CanvasWidget` is returned (ID 0).
    pub fn get_parent_of(&mut self, widget_id: i32) -> i32 {
        if widget_id <= 0 {
            0
        } else {
            self.widgets[widget_id as usize].parent_id
        }
    }

    /// Returns a list of the children that are owned by a parent ID.  Does not return a list of
    /// siblings, only the first-level children.
    pub fn get_children_of(&self, parent_id: i32) -> Vec<i32> {
        self.widgets
            .iter()
            .filter(|x| x.parent_id == parent_id)
            .map(|x| x.widget_id)
            .collect()
    }

    /// Gets a `Widget` by ID for a point in the screen.  If the GUI object is hidden or disabled,
    /// the ID is not returned.  If no widget is found under the point specified, an ID of -1 is
    /// returned.
    pub fn get_widget_id_for_point(&mut self, point: Point) -> i32 {
        let mut found_id = -1;

        for (pos, obj) in self.widgets.iter_mut().enumerate() {
            let hidden = *&obj
                .widget
                .borrow_mut()
                .config()
                .get_toggle(CONFIG_WIDGET_HIDDEN);
            let disabled = *&obj
                .widget
                .borrow_mut()
                .config()
                .get_toggle(CONFIG_WIDGET_DISABLED);

            if !hidden && !disabled {
                let widget_point = &obj.widget.borrow_mut().config().get_point(CONFIG_ORIGIN);
                let widget_size: crate::core::point::Size =
                    obj.widget.borrow_mut().config().get_size(CONFIG_BODY_SIZE);

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
        }

        found_id
    }

    /// Returns the name of the `Widget` by specified ID.
    pub fn get_name_for_widget_id(&mut self, widget_id: i32) -> &str {
        self.widgets[widget_id as usize].widget_name.as_str()
    }

    /// Retrieves a reference to a `Widget` by its name.  Returns the top-level `CanvasWidget`
    /// object if not found.
    pub fn get_widget_for_name(&mut self, name: &str) -> &RefCell<Box<dyn Widget>> {
        let widget_id = match self
            .widgets
            .iter_mut()
            .find(|x| x.widget_name == String::from(name))
        {
            Some(x) => x.widget_id,
            None => 0,
        };

        self.get_widget_for_id(widget_id)
    }

    /// Retrieves a `Widget` by its ID.
    pub fn get_widget_for_id(&mut self, id: i32) -> &RefCell<Box<dyn Widget>> {
        &self.widgets[id as usize].widget
    }

    /// Handles a specific event generated by the OS or the GUI interaction.
    pub fn handle_event(&mut self, widget_id: i32, event: CallbackEvent) -> Option<CallbackEvent> {
        self.widgets[widget_id as usize]
            .widget
            .borrow_mut()
            .handle_event(false, event)
    }

    /// Injects an event to all widgets, allowing them to exhibit custom event handling behavior if
    /// required.  This is usually used in cases where special triggering needs to take place, like
    /// an indication of a timeout or transient error.
    pub fn inject_event(&mut self, event: CallbackEvent) {
        self.widgets.iter_mut().for_each(|x| {
            x.widget.borrow_mut().handle_event(true, event.clone());
        });
    }

    /// Draws a `Widget` by ID, and any children contained in that `Widget`.  Submitting a draw
    /// request from ID 0 will redraw the entire screen.
    pub fn draw(&mut self, widget_id: i32, c: Context, g: &mut GlGraphics, original_fbo: GLuint) {
        let parents_of_widget = self.get_children_of(widget_id);

        if parents_of_widget.len() == 0 {
            return;
        }

        for pos in 0..parents_of_widget.len() {
            c.reset();

            let paint_id = parents_of_widget[pos];
            let paint_widget = &mut self.widgets[paint_id as usize];
            let is_hidden = paint_widget
                .widget
                .borrow_mut()
                .config()
                .get_toggle(CONFIG_WIDGET_HIDDEN);
            let is_invalidated = &paint_widget.widget.borrow_mut().is_invalidated();

            if !is_hidden && is_invalidated == &true {
                let origin: Point = paint_widget
                    .widget
                    .borrow_mut()
                    .config()
                    .get_point(CONFIG_ORIGIN);

                let new_context: Context = Context {
                    viewport: c.viewport,
                    view: c.view,
                    transform: c.transform.trans(origin.x as f64, origin.y as f64),
                    draw_state: c.draw_state,
                };

                &paint_widget
                    .widget
                    .borrow_mut()
                    .draw(new_context, g, &c.draw_state);

                if paint_widget
                    .widget
                    .borrow_mut()
                    .config()
                    .get_toggle(CONFIG_WIDGET_DISABLED)
                {
                    &paint_widget
                        .widget
                        .borrow_mut()
                        .draw_disabled(new_context, g, &c.draw_state);
                }
            }

            if parents_of_widget[pos] != widget_id {
                self.draw(paint_id, c, g, original_fbo);
            }
        }
    }
}
