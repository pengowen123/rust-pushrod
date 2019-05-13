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

use opengl_graphics::GlGraphics;
use piston_window::*;

use crate::core::callbacks::*;
use crate::core::point::Point;
use crate::widget::box_widget::*;
use crate::widget::config::*;
use crate::widget::image_widget::*;
use crate::widget::text_widget::*;
use crate::widget::widget::*;

/// This is the `RadioButtonWidget`, which contains a top-level widget for display, overriding the
/// draw method to draw the base widget and the border for this box.
///
/// Example usage:
/// IN PROGRESS
pub struct RadioButtonWidget {
    config: Configurable,
    base_widget: BoxWidget,
    text_widget: TextWidget,
    selected: bool,
    selected_widget: ImageWidget,
    unselected_widget: ImageWidget,
    inject_event: bool,
}

/// Implementation of the constructor for the `RadioButtonWidget`.
impl RadioButtonWidget {
    pub fn new(
        factory: &mut GfxFactory,
        font_name: String,
        text: String,
        font_size: u32,
        justify: TextJustify,
        selected: bool,
    ) -> Self {
        let mut selected_widget = ImageWidget::new(factory, "radio_selected.png".to_string());
        selected_widget.set_point(CONFIG_ORIGIN, 2, 2);
        selected_widget.set_toggle(CONFIG_WIDGET_HIDDEN, true);

        let mut unselected_widget = ImageWidget::new(factory, "radio_unselected.png".to_string());
        unselected_widget.set_point(CONFIG_ORIGIN, 2, 2);
        unselected_widget.set_toggle(CONFIG_WIDGET_HIDDEN, false);

        let mut text_widget = TextWidget::new(
            factory,
            font_name.to_string(),
            text.to_string(),
            font_size,
            justify,
        );
        text_widget.set_point(CONFIG_ORIGIN, 36, 0);

        Self {
            config: Configurable::new(),
            base_widget: BoxWidget::new(),
            text_widget,
            selected,
            selected_widget,
            unselected_widget,
            inject_event: false,
        }
    }
}

/// Implementation of the `RadioButtonWidget` object with the `Widget` traits implemented.
/// The base widget is a `BoxWidget`, which overlays a `TextWidget` over the top.  This `Widget`
/// responds to the button down/up callbacks internally.
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
                        if widget_id != self.config().get_numeric(CONFIG_WIDGET_ID) as i32 {
                            self.selected = false;
                        }
                    }
                }

                _ => (),
            }
        }

        None
    }

    /// This function injects events, as other radio buttons need to become invalidated that may
    /// be part of the same group ID.
    fn injects_events(&mut self) -> bool {
        true
    }

    /// Returns an injected event where appropriate.
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

    /// Draws the contents of the widget in this order:
    ///
    /// - Base widget first
    /// - Box graphic for the specified width
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c, g, &clip);

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
