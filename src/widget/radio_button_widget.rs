// Radio Button Widget
// Extensible widget for the widget library - handles a radio button with text.
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

use graphics::*;
use opengl_graphics::GlGraphics;

use piston::input::*;

use crate::core::callbacks::*;
use crate::core::point::Point;
use crate::widget::box_widget::*;
use crate::widget::config::*;
use crate::widget::image_widget::*;
use crate::widget::text_widget::*;
use crate::widget::widget::*;

/// Draws a radio button with only one object that can be selected at any one time within a group.
/// Generates a `WidgetSelected` event when an object has been selected within the group.  Also
/// generates an internal `UnselectRadioButtons` event, which should not be handled - this is only
/// used by `RadioButtonWidget` objects.
pub struct RadioButtonWidget {
    config: Configurable,
    base_widget: BoxWidget,
    text_widget: TextWidget,
    selected: bool,
    selected_widget: ImageWidget,
    unselected_widget: ImageWidget,
    inject_event: bool,
    widget_id: i32,
}

impl RadioButtonWidget {
    /// Constructor.  Requires the name of the font, the text to display, the image name to display, the size of the font,
    /// the font justification when rendered, and a selected pre-set state.  Images are
    /// loaded from the `assets/` directory.
    pub fn new(
        font_name: String,
        text: String,
        font_size: u32,
        justify: TextJustify,
        selected: bool,
    ) -> Self {
        let mut selected_widget = ImageWidget::new("radio_selected.png".to_string());
        selected_widget.set_point(CONFIG_ORIGIN, 2, 2);
        selected_widget.set_toggle(CONFIG_WIDGET_HIDDEN, true);

        let mut unselected_widget = ImageWidget::new("radio_unselected.png".to_string());
        unselected_widget.set_point(CONFIG_ORIGIN, 2, 2);
        unselected_widget.set_toggle(CONFIG_WIDGET_HIDDEN, false);

        let mut text_widget =
            TextWidget::new(font_name.to_string(), text.to_string(), font_size, justify);
        text_widget.set_point(CONFIG_ORIGIN, 36, 0);

        Self {
            config: Configurable::new(),
            base_widget: BoxWidget::new(),
            text_widget,
            selected,
            selected_widget,
            unselected_widget,
            inject_event: false,
            widget_id: 0,
        }
    }
}

impl Widget for RadioButtonWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.base_widget.set_config(config, config_value.clone());
        self.text_widget.set_config(config, config_value.clone());

        if config == CONFIG_BODY_SIZE {
            let size = self.config().get_size(CONFIG_BODY_SIZE);

            if size.h < 32 {
                self.selected_widget
                    .set_size(CONFIG_BODY_SIZE, size.h, size.h);
                self.unselected_widget
                    .set_size(CONFIG_BODY_SIZE, size.h, size.h);
            } else {
                self.selected_widget
                    .set_point(CONFIG_ORIGIN, 0, (size.h - 32) / 2);
                self.selected_widget.set_size(CONFIG_BODY_SIZE, 32, 32);
                self.unselected_widget
                    .set_point(CONFIG_ORIGIN, 0, (size.h - 32) / 2);
                self.unselected_widget.set_size(CONFIG_BODY_SIZE, 32, 32);
            }
        }
    }

    fn handle_event(&mut self, injected: bool, event: CallbackEvent) -> Option<CallbackEvent> {
        if !injected {
            match event {
                CallbackEvent::MouseButtonUpInside { widget_id, button } => match button {
                    Button::Mouse(mouse_button) => {
                        if mouse_button == MouseButton::Left {
                            self.selected = true;
                            self.inject_event = true;

                            self.invalidate();

                            return Some(CallbackEvent::WidgetSelected {
                                widget_id,
                                button,
                                selected: self.selected,
                            });
                        }
                    }

                    _ => (),
                },

                _ => (),
            }
        } else {
            match event {
                CallbackEvent::UnselectRadioButtons {
                    widget_id,
                    group_id,
                } => {
                    if group_id == self.config().get_numeric(CONFIG_WIDGET_GROUP_ID) as i32 {
                        if widget_id != self.widget_id {
                            self.selected = false;
                        }
                    }

                    self.invalidate();
                }

                _ => (),
            }
        }

        None
    }

    fn injects_events(&mut self) -> bool {
        true
    }

    fn inject_event(&mut self, widget_id: i32) -> Option<CallbackEvent> {
        if self.inject_event {
            self.inject_event = false;

            Some(CallbackEvent::UnselectRadioButtons {
                widget_id,
                group_id: self.config().get_numeric(CONFIG_WIDGET_GROUP_ID) as i32,
            })
        } else {
            None
        }
    }

    fn set_widget_id(&mut self, widget_id: i32) {
        self.widget_id = widget_id;
    }

    fn get_widget_id(&mut self) -> i32 {
        self.widget_id
    }

    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c, g, &clip);

        let size = self.config().get_size(CONFIG_BODY_SIZE);

        // Clear the drawing backing
        g.rectangle(
            &Rectangle::new(self.config().get_color(CONFIG_MAIN_COLOR)),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
        );

        if self.selected {
            self.selected_widget
                .draw_with_offset(c, g, &clip, Point { x: 0, y: 0 });
        } else {
            self.unselected_widget
                .draw_with_offset(c, g, &clip, Point { x: 0, y: 0 });
        }

        self.text_widget
            .draw_with_offset(c, g, &clip, Point { x: 38, y: 0 });

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
