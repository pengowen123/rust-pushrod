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

use graphics::rectangle::Border;
use opengl_graphics::GlGraphics;
use piston_window::*;

use crate::widget::config::*;
use crate::widget::widget::*;

/// This is the `BoxWidget`, which contains a top-level widget for display, overriding the
/// draw method to draw the base widget and the border for this box.
///
/// Example usage:
/// IN PROGRESS
pub struct BoxWidget {
    config: Configurable,
}

/// Implementation of the constructor for the `BoxWidget`.
impl BoxWidget {
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
        }
    }

    /// Function to draw a box for the point and size of this box.  Automatically draws the border
    /// along with the width of the border.  This is automatically determined by the origin, so the
    /// box is automatically drawn for the bounds of the `Widget`.
    fn draw_box(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);
        let border: f64 = self.config().get_numeric(CONFIG_BORDER_WIDTH) as f64;
        let color: types::Color = self.config().get_color(CONFIG_BORDER_COLOR);

        g.rectangle(
            &Rectangle::new(self.config().get_color(CONFIG_MAIN_COLOR)).border(Border {
                color,
                radius: border,
            }),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
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
        self.invalidate();
    }

    /// Draws the contents of the widget in this order:
    ///
    /// - Base widget first
    /// - Box graphic for the specified width
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        // Paint the box.
        self.draw_box(c, g, &clip);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
