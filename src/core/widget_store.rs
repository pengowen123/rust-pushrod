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

use opengl_graphics::GlGraphics;
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

pub struct WidgetStore {
    /// A vector list of Boxed `PushrodWidget` trait objects.
    pub widgets: Vec<WidgetContainer>,
}

impl WidgetStore {
    pub fn new() -> Self {
        let mut widgets_list: Vec<WidgetContainer> = Vec::new();
        let mut base_widget = BaseWidget::new();

        base_widget.set_size(800, 600);
        widgets_list.push(WidgetContainer {
            widget: Box::new(base_widget),
            widget_id: 0,
            parent_id: 0,
        });

        Self {
            widgets: widgets_list,
        }
    }

    /// Handles the resizing of the texture buffer after the window resize has taken place.  The
    /// behavior should be processed before drawing is rendered, so the sequence of events should
    /// be `event` -> `handle_resize` -> `invalidate` -> `draw`.  This is mainly handled by the
    /// `pushrod::core::main` loop, but it can be handled programmatically if required.
    pub fn handle_resize(&mut self, width: u32, height: u32) {
        eprintln!("[Resize] W={} H={}", width, height);
    }

    /// Invalidates all widgets in the window.  This is used to force a complete refresh of the
    /// window's contents, usually based on a timer expiration, or a window resize.  Use with
    /// care, as this is an expensive operation.
    pub fn invalidate_all_widgets(&mut self) {
        self.widgets.iter_mut().for_each(|x| x.widget.invalidate());
    }

    pub fn needs_repaint(&mut self) -> bool {
        self.widgets
            .iter_mut()
            .map(|x| x.widget.is_invalidated())
            .find(|x| x == &true)
            .unwrap_or(false)
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

        self.widgets.push(WidgetContainer {
            widget,
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
            let widget_point = &obj.widget.get_origin();
            let widget_size: crate::core::point::Size = obj.widget.get_size();

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

    pub fn draw(&mut self, widget_id: i32, c: Context, g: &mut GlGraphics) {
        let parents_of_widget = self.get_children_of(widget_id);

        if parents_of_widget.len() == 0 {
            return;
        }

        for pos in 0..parents_of_widget.len() {
            let paint_id = parents_of_widget[pos];
            let paint_widget = &mut self.widgets[paint_id as usize];

            if &paint_widget.widget.is_invalidated() == &true {
                if paint_widget.widget.get_autoclip() {
                    let trans = c.transform.trans(
                        paint_widget.widget.get_origin().x as f64,
                        paint_widget.widget.get_origin().y as f64,
                    );
                    let viewport = c.viewport.unwrap();
                    let scale_x = viewport.draw_size[0] as f64 / viewport.window_size[0];
                    let scale_y = viewport.draw_size[1] as f64 / viewport.window_size[1];

                    let clip_rect = [
                        ((paint_widget.widget.get_origin().x as f64 + viewport.rect[0] as f64)
                            * scale_x) as u32,
                        ((paint_widget.widget.get_origin().y as f64 + viewport.rect[1] as f64)
                            * scale_y) as u32,
                        (paint_widget.widget.get_size().w as f64 * scale_x) as u32,
                        (paint_widget.widget.get_size().h as f64 * scale_y) as u32,
                    ];

                    let vp = Viewport {
                        rect: [
                            paint_widget.widget.get_origin().x as i32 + viewport.rect[0],
                            paint_widget.widget.get_origin().y as i32 + viewport.rect[1],
                            paint_widget.widget.get_size().w as i32,
                            paint_widget.widget.get_size().h as i32,
                        ],
                        draw_size: viewport.draw_size,
                        window_size: viewport.window_size,
                    };

                    let clipped = Context {
                        viewport: Some(vp),
                        view: c.view,
                        transform: trans,
                        draw_state: c.draw_state.scissor(clip_rect),
                    };

                    &paint_widget.widget.draw(clipped, g);
                } else {
                    &paint_widget.widget.draw(c, g);
                }
            }

            if parents_of_widget[pos] != widget_id {
                self.draw(paint_id, c, g);
            }
        }
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
