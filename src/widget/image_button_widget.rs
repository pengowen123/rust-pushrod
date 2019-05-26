// Image Button Widget
// Extensible widget for the widget library - handles an image button object.
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

use crate::core::callbacks::CallbackEvent::WidgetClicked;
use crate::core::callbacks::*;
use crate::core::point::Point;
use crate::widget::box_widget::*;
use crate::widget::config::*;
use crate::widget::image_widget::*;
use crate::widget::text_widget::*;
use crate::widget::widget::*;

/// This is the `ImageButtonWidget`, which contains a top-level widget for display, overriding the
/// draw method to draw the base widget and the border for this box.
///
/// Example usage:
/// IN PROGRESS
pub struct ImageButtonWidget {
    config: Configurable,
    base_widget: BoxWidget,
    text_widget: TextWidget,
    image_widget: ImageWidget,
    active: bool,
}

/// Implementation of the constructor for the `ImageButtonWidget`.
impl ImageButtonWidget {
    pub fn new(
        factory: &mut GfxFactory,
        font_name: String,
        text: String,
        image_name: String,
        font_size: u32,
        justify: TextJustify,
    ) -> Self {
        let mut image_widget = ImageWidget::new(factory, image_name.to_string());

        image_widget.set_point(CONFIG_ORIGIN, 2, 2);

        Self {
            config: Configurable::new(),
            base_widget: BoxWidget::new(),
            text_widget: TextWidget::new(
                factory,
                font_name.to_string(),
                text.to_string(),
                font_size,
                justify,
            ),
            image_widget,
            active: false,
        }
    }

    fn draw_hovered(&mut self) {
        self.base_widget
            .set_color(CONFIG_MAIN_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.text_widget.set_color(CONFIG_TEXT_COLOR, [1.0; 4]);
    }

    fn draw_unhovered(&mut self) {
        self.base_widget.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
        self.text_widget
            .set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    }
}

/// Implementation of the `ImageButtonWidget` object with the `Widget` traits implemented.
/// The base widget is a `BoxWidget`, which overlays a `TextWidget` over the top.  This `Widget`
/// responds to the button down/up callbacks internally, and generates an `on_clicked` callback
/// when appropriate.
impl Widget for ImageButtonWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());

        self.base_widget.set_config(config, config_value.clone());
        self.text_widget.set_config(config, config_value.clone());
        self.image_widget.set_config(config, config_value.clone());
    }

    fn handle_event(&mut self, injected: bool, event: CallbackEvent) -> Option<CallbackEvent> {
        if !injected {
            match event {
                CallbackEvent::MouseEntered { widget_id: _ } => {
                    if self.active {
                        self.draw_hovered();
                    }
                }

                CallbackEvent::MouseExited { widget_id: _ } => {
                    if self.active {
                        self.draw_unhovered();
                    }
                }

                CallbackEvent::MouseButtonDown {
                    widget_id: _,
                    button,
                } => match button {
                    Button::Mouse(mouse_button) => {
                        if mouse_button == MouseButton::Left {
                            self.draw_hovered();
                            self.active = true;
                        }
                    }
                    _ => (),
                },

                CallbackEvent::MouseButtonUpInside { widget_id, button } => match button {
                    Button::Mouse(mouse_button) => {
                        if mouse_button == MouseButton::Left {
                            self.draw_unhovered();
                            self.active = false;

                            return Some(WidgetClicked { widget_id, button });
                        }
                    }
                    _ => (),
                },

                CallbackEvent::MouseButtonUpOutside {
                    widget_id: _,
                    button,
                } => match button {
                    Button::Mouse(mouse_button) => {
                        if mouse_button == MouseButton::Left {
                            self.draw_unhovered();
                            self.active = false;
                        }
                    }
                    _ => (),
                },

                _ => (),
            }
        }

        None
    }

    /// Draws the contents of the widget in this order:
    ///
    /// - Base widget first
    /// - Box graphic for the specified width
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        let size = self.base_widget.config().get_size(CONFIG_BODY_SIZE);
        let border = (self.base_widget.config().get_numeric(CONFIG_BORDER_WIDTH) * 2) as i32;

        self.image_widget.config().set_size(
            CONFIG_BODY_SIZE,
            size.h - border as i32,
            size.h - (border * 2) as i32,
        );

        self.base_widget.draw(c, g, &clip);
        self.image_widget
            .draw_with_offset(c, g, &clip, Point { x: 2, y: 2 });
        self.text_widget.draw_with_offset(
            c,
            g,
            &clip,
            Point {
                x: size.h + border + 4,
                y: 0,
            },
        );

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
