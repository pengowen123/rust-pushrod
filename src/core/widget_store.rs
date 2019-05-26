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
use crate::widget::config::*;
use crate::widget::widget::*;

pub struct WidgetContainer {
    pub widget: RefCell<Box<dyn Widget>>,

    widget_name: String,

    pub widget_id: i32,

    parent_id: i32,
}

pub struct WidgetStore {
    pub widgets: Vec<WidgetContainer>,
}

impl WidgetStore {
    pub fn new() -> Self {
        let mut widgets_list: Vec<WidgetContainer> = Vec::new();
        let mut base_widget = CanvasWidget::new();

        base_widget.config().set(
            CONFIG_BODY_SIZE,
            Config::Size(crate::core::point::Size { w: 800, h: 600 }),
        );
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

    pub fn invalidate_all_widgets(&mut self) {
        self.widgets
            .iter_mut()
            .for_each(|x| x.widget.borrow_mut().invalidate());
    }

    pub fn needs_repaint(&mut self) -> bool {
        self.widgets
            .iter_mut()
            .map(|x| x.widget.borrow_mut().is_invalidated())
            .find(|x| x == &true)
            .unwrap_or(false)
    }

    pub fn add_widget(&mut self, name: &str, widget: Box<dyn Widget>) -> i32 {
        let widget_size = self.widgets.len() as i32;
        let container = WidgetContainer {
            widget: RefCell::new(widget),
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

    pub fn get_parent_of(&mut self, widget_id: i32) -> i32 {
        if widget_id <= 0 {
            0
        } else {
            self.widgets[widget_id as usize].parent_id
        }
    }

    pub fn get_children_of(&self, parent_id: i32) -> Vec<i32> {
        self.widgets
            .iter()
            .filter(|x| x.parent_id == parent_id)
            .map(|x| x.widget_id)
            .collect()
    }

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

    pub fn get_name_for_widget_id(&mut self, widget_id: i32) -> &str {
        self.widgets[widget_id as usize].widget_name.as_str()
    }

    pub fn handle_event(&mut self, widget_id: i32, event: CallbackEvent) -> Option<CallbackEvent> {
        self.widgets[widget_id as usize]
            .widget
            .borrow_mut()
            .handle_event(false, event)
    }

    pub fn inject_event(&mut self, event: CallbackEvent) {
        self.widgets.iter_mut().for_each(|x| {
            x.widget.borrow_mut().handle_event(true, event.clone());
        });
    }

    pub fn draw(&mut self, widget_id: i32, c: Context, g: &mut G2d) {
        let parents_of_widget = self.get_children_of(widget_id);

        if parents_of_widget.len() == 0 {
            return;
        }

        for pos in 0..parents_of_widget.len() {
            c.reset();

            let paint_id = parents_of_widget[pos];
            let paint_widget = &mut self.widgets[paint_id as usize];

            if !paint_widget
                .widget
                .borrow_mut()
                .config()
                .get_toggle(CONFIG_WIDGET_HIDDEN)
            {
                if &paint_widget.widget.borrow_mut().is_invalidated() == &true {
                    let origin: Point = paint_widget
                        .widget
                        .borrow_mut()
                        .config()
                        .get_point(CONFIG_ORIGIN);
//                    let size: crate::core::point::Size = paint_widget
//                        .widget
//                        .borrow_mut()
//                        .config()
//                        .get_size(CONFIG_BODY_SIZE);

                    let new_context: Context = Context {
                        viewport: c.viewport,
                        view: c.view,
                        transform: c.transform.trans(origin.x as f64, origin.y as f64),
                        draw_state: c.draw_state,
                    };

//                    let clip: DrawState = c.draw_state.scissor([
//                        origin.x as u32,
//                        origin.y as u32,
//                        size.w as u32,
//                        size.h as u32,
//                    ]);

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
            }

            if parents_of_widget[pos] != widget_id {
                self.draw(paint_id, c, g);
            }
        }
    }

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

    pub fn get_widget_for_id(&mut self, id: i32) -> &RefCell<Box<dyn Widget>> {
        &self.widgets[id as usize].widget
    }
}
