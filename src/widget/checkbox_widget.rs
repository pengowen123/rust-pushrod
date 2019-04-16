// Checkbox Widget
// Extensible widget for the widget library - handles a checkbox with text button.
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

use crate::core::callbacks::*;
use crate::widget::box_widget::*;
use crate::widget::config::*;
use crate::widget::text_widget::*;
use crate::widget::image_widget::*;
use crate::widget::widget::*;

/// This is the `CheckboxWidget`, which contains a top-level widget for display, overriding the
/// draw method to draw the base widget and the border for this box.
///
/// Example usage:
/// IN PROGRESS
pub struct CheckboxWidget {
    config: Configurable,
    base_widget: BoxWidget,
    text_widget: TextWidget,
    selected: bool,
    selected_widget: ImageWidget,
    unselected_widget: ImageWidget,
}

/// Implementation of the constructor for the `CheckboxWidget`.
impl CheckboxWidget {
    pub fn new(
        factory: &mut GfxFactory,
        font_name: String,
        text: String,
        font_size: u32,
        justify: TextJustify,
        selected: bool,
    ) -> Self {
        let mut selected_widget = ImageWidget::new(
            factory,
            "checkbox_selected.png".to_string(),
        );
        selected_widget.set_point(CONFIG_ORIGIN, 2, 2);
        selected_widget.set_size(CONFIG_BODY_SIZE, 32, 32);
        selected_widget.set_toggle(CONFIG_WIDGET_HIDDEN, true);

        let mut unselected_widget = ImageWidget::new(
            factory,
            "checkbox_unselected.png".to_string(),
        );
        unselected_widget.set_point(CONFIG_ORIGIN, 2, 2);
        unselected_widget.set_size(CONFIG_BODY_SIZE, 32, 32);
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
        }
    }
}

/// Implementation of the `CheckboxWidget` object with the `Widget` traits implemented.
/// The base widget is a `BoxWidget`, which overlays a `TextWidget` over the top.  This `Widget`
/// responds to the button down/up callbacks internally.
impl Widget for CheckboxWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.base_widget.set_config(config, config_value.clone());
        self.text_widget.set_config(config, config_value.clone());
    }

    fn handle_event(&mut self, event: CallbackEvent) -> Option<CallbackEvent> {
        match event {
            CallbackEvent::MouseButtonUpInside { widget_id, button } => match button {
                Button::Mouse(mouse_button) => {
                    if mouse_button == MouseButton::Left {
                        self.selected = !self.selected;

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

        None
    }

    /// Draws the contents of the widget in this order:
    ///
    /// - Base widget first
    /// - Box graphic for the specified width
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c, g, &clip);
        self.text_widget.draw(c.trans(38.0, 0.0), g, &clip);

        if self.selected {
            self.selected_widget.draw(c, g, &clip);
        } else {
            self.unselected_widget.draw(c, g, &clip);
        }

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
