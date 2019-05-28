// Toggle Button Widget
// Extensible widget for the widget library - handles a toggleable button object.
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
use opengl_graphics::GlGraphics;

use crate::core::callbacks::CallbackEvent::WidgetSelected;
use crate::core::callbacks::*;
use crate::widget::box_widget::*;
use crate::widget::config::*;
use crate::widget::text_widget::*;
use crate::widget::widget::*;

/// Draws a toggleable button on the screen.  The selected state of this object can be set
/// ahead of time by toggling `CONFIG_SELECTED`.  This `Widget` generates `WidgetSelected`
/// events when toggled.  The toggle state can be retrieved by reading the `CONFIG_SELECTED`
/// state of this `Widget`.
pub struct ToggleButtonWidget {
    config: Configurable,
    base_widget: BoxWidget,
    text_widget: TextWidget,
    selected: bool,
    active: bool,
}

impl ToggleButtonWidget {
    /// Constructor.  Requires a `GfxFactory` (retrievable from `Main::get_factory`),
    /// the name of the font, the text to display, the size of the font,
    /// and the font justification when rendered.  Fonts are loaded from the `assets/`
    /// directory.
    pub fn new(
        factory: &mut GfxFactory,
        font_name: String,
        text: String,
        font_size: u32,
        justify: TextJustify,
    ) -> Self {
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
            selected: false,
            active: false,
        }
    }

    fn draw_hovered(&mut self) {
        if !self.selected {
            self.base_widget
                .set_color(CONFIG_MAIN_COLOR, [0.0, 0.0, 0.0, 1.0]);
            self.text_widget.set_color(CONFIG_TEXT_COLOR, [1.0; 4]);
        } else {
            self.base_widget.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
            self.text_widget
                .set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        }
    }

    fn draw_unhovered(&mut self) {
        if !self.selected {
            self.base_widget.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
            self.text_widget
                .set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        } else {
            self.base_widget
                .set_color(CONFIG_MAIN_COLOR, [0.0, 0.0, 0.0, 1.0]);
            self.text_widget.set_color(CONFIG_TEXT_COLOR, [1.0; 4]);
        }
    }
}

impl Widget for ToggleButtonWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.base_widget.set_config(config, config_value.clone());
        self.text_widget.set_config(config, config_value.clone());

        if config == CONFIG_SELECTED {
            if self.config().get_toggle(CONFIG_SELECTED) {
                self.draw_hovered();
                self.selected = true;
            }
        }
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
                            self.selected = !self.selected;
                            self.draw_unhovered();
                            self.active = false;

                            return Some(WidgetSelected {
                                widget_id,
                                button,
                                selected: self.selected,
                            });
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

    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c, g, &clip);
        self.text_widget.draw(c, g, &clip);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
