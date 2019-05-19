// Box Widget
// Extensible widget for the widget library - handles drawing a box with a border and a fill color
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

use crate::widget::config::*;
use crate::widget::widget::*;

/// This is the `BoxWidget`, which contains a top-level widget for display, overriding the
/// draw method to draw the base widget and the border for this box.
///
/// Example usage:
/// NEEDS_DOCUMENTATION
pub struct BoxWidget {
    config: Configurable,
    base_widget: CanvasWidget,
}

/// Implementation of the constructor for the `BoxWidget`.
impl BoxWidget {
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            base_widget: CanvasWidget::new(),
        }
    }

    /// Function to draw a box for the point and size of this box.  Automatically draws the border
    /// along with the width of the border.  This is automatically determined by the origin, so the
    /// box is automatically drawn for the bounds of the `Widget`.
    fn draw_box(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);
        let border: f64 = self.config().get_numeric(CONFIG_BORDER_WIDTH) as f64;
        let color: types::Color = self.config().get_color(CONFIG_BORDER_COLOR);

        // Upper left to upper right
        Line::new(color, border).draw(
            [0.0 as f64, border, size.w as f64, border],
            clip,
            c.transform,
            g,
        );

        // Upper left to lower right
        Line::new(color, border).draw(
            [
                size.w as f64 - border,
                border,
                size.w as f64 - border,
                size.h as f64,
            ],
            clip,
            c.transform,
            g,
        );

        // Upper left to lower left
        Line::new(color, border).draw(
            [border, border, border, size.h as f64],
            clip,
            c.transform,
            g,
        );

        // Lower left to lower right
        Line::new(color, border).draw(
            [
                0.0 as f64,
                size.h as f64 - border,
                size.w as f64,
                size.h as f64 - border,
            ],
            clip,
            c.transform,
            g,
        );
    }
}

/// Implementation of the `BoxWidget` object with the `Widget` traits implemented.
/// This implementation is similar to the `CanvasWidget`, but incorporates a drawable box inside
/// the widget.  Base widget is the `CanvasWidget`.
///
/// This is basically just a box with a fill color.  Use this to draw other things like buttons,
/// text widgets, and so on, if you need anything with a drawable border.
impl Widget for BoxWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.base_widget.config().set(config, config_value.clone());
        self.invalidate();
    }

    /// Draws the contents of the widget in this order:
    ///
    /// - Base widget first
    /// - Box graphic for the specified width
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c, g, &clip);

        // Paint the box.
        self.draw_box(c, g, &clip);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
