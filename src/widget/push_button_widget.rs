// Push Button Widget
// Extensible widget for the widget library - handles a push button object.
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
use crate::core::point::*;
use crate::widget::box_widget::*;
use crate::widget::config::*;
use crate::widget::text_widget::*;
use crate::widget::widget::*;

/// This is the `PushButtonWidget`, which contains a top-level widget for display, overriding the
/// draw method to draw the base widget and the border for this box.
///
/// Example usage:
/// IN PROGRESS
pub struct PushButtonWidget {
    config: Configurable,
    base_widget: BoxWidget,
    text_widget: TextWidget,
}

/// Implementation of the constructor for the `PushButtonWidget`.
impl PushButtonWidget {
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
        }
    }

    /// Sets the color of the text for this `Widget`.
    pub fn set_text_color(&mut self, color: types::Color) {
        self.text_widget.set_text_color(color);
    }

    /// Retrieves the color of the text for this `Widget`.
    /// Defaults to black if not set.
    pub fn get_text_color(&mut self) -> types::Color {
        self.text_widget.get_text_color()
    }

    /// Sets the border color for this widget.
    pub fn set_border_color(&mut self, color: types::Color) {
        self.base_widget.set_border_color(color);
        self.text_widget.invalidate();
        self.invalidate();
    }

    /// Retrieves the border color of this widget.
    /// Defaults to black color `[0.0, 0.0, 0.0, 1.0]` if not set.
    pub fn get_border_color(&mut self) -> types::Color {
        self.base_widget.get_border_color()
    }

    /// Sets the thickness of the border for this widget.
    pub fn set_border_thickness(&mut self, thickness: u8) {
        self.base_widget.set_border_thickness(thickness);
        self.text_widget.invalidate();
        self.invalidate();
    }

    /// Retrieves the border thickness of this widget.
    /// Defaults to 1 if not set.
    pub fn get_border_thickness(&mut self) -> u8 {
        self.base_widget.get_border_thickness()
    }

    /// Helper function that sets both the color of the border and the thickness at the same time.
    pub fn set_border(&mut self, color: types::Color, thickness: u8) {
        self.set_border_color(color);
        self.set_border_thickness(thickness);
    }
}

/// Implementation of the `PushButtonWidget` object with the `Widget` traits implemented.
/// The base widget is a `BoxWidget`, which overlays a `TextWidget` over the top.  This `Widget`
/// responds to the button down/up callbacks internally, and generates an `on_clicked` callback
/// when appropriate.
impl Widget for PushButtonWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    /// Sets the `Point` of origin for this widget and the base widget, given the X and Y
    /// coordinates.  Invalidates the widget afterward.
    fn set_origin(&mut self, x: i32, y: i32) {
        let border_size = self.base_widget.get_border_thickness();

        self.config().set(Origin(Point { x, y }));
        self.base_widget.set_origin(x, y);
        self.text_widget
            .set_origin(x + border_size as i32, y + border_size as i32);
        self.invalidate();
    }

    /// Sets the `Size` for this widget and the base widget, given width and height.  Invalidates the widget afterward.
    fn set_size(&mut self, w: i32, h: i32) {
        let border_size = self.base_widget.get_border_thickness();

        self.config()
            .set(BodySize(crate::core::point::Size { w, h }));
        self.base_widget.set_size(w, h);
        self.text_widget
            .set_size(w - (border_size as i32 * 2), h - (border_size as i32 * 2));
        self.invalidate();
    }

    /// Sets the color for this widget.  Invalidates the widget afterward.
    fn set_color(&mut self, color: types::Color) {
        self.config().set(MainColor(color));
        self.base_widget.set_color(color);
        self.invalidate();
    }

    /// Retrieves the color of this widget.
    /// Defaults to white color `[1.0; 4]` if not set.
    fn get_color(&mut self) -> types::Color {
        self.base_widget.get_color()
    }

    /// Sets the border color for this widget.
    fn set_text(&mut self, text: &str) {
        self.text_widget.set_text(text);
        self.invalidate();
    }

    fn handle_event(&mut self, event: CallbackEvent) -> Option<CallbackEvent> {
        match event {
            CallbackEvent::MouseButtonDown {
                widget_id: _,
                button,
            } => match button {
                Button::Mouse(mouse_button) => {
                    if mouse_button == MouseButton::Left {
                        self.base_widget.set_color([0.0, 0.0, 0.0, 1.0]);
                        self.text_widget.set_text_color([1.0; 4]);
                    }
                }
                _ => (),
            },

            CallbackEvent::MouseButtonUpInside { widget_id, button } => match button {
                Button::Mouse(mouse_button) => {
                    if mouse_button == MouseButton::Left {
                        self.base_widget.set_color([1.0; 4]);
                        self.text_widget.set_text_color([0.0, 0.0, 0.0, 1.0]);

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
                        self.base_widget.set_color([1.0; 4]);
                        self.text_widget.set_text_color([0.0, 0.0, 0.0, 1.0]);
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
        self.text_widget.draw(c, g, &clip);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
