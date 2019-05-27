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

/// A `BoxWidget` is a `CanvasWidget` with a bounding box.  Takes two additional options:
/// * `CONFIG_BORDER_WIDTH` specifies the width of the border to be drawn in pixels.
/// * `CONFIG_BORDER_COLOR` specifies the color of the border to be drawn.
pub struct BoxWidget {
    config: Configurable,
    base_widget: CanvasWidget,
}

impl BoxWidget {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            base_widget: CanvasWidget::new(),
        }
    }

    fn draw_box(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);
        let border: f64 = self.config().get_numeric(CONFIG_BORDER_WIDTH) as f64;
        let color: types::Color = self.config().get_color(CONFIG_BORDER_COLOR);

        Rectangle::new_border(color, border).draw(
            [0.0 as f64, 0.0 as f64, size.w as f64, size.h as f64],
            clip, c.transform, g);
    }
}

impl Widget for BoxWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.base_widget.config().set(config, config_value.clone());
        self.invalidate();
    }

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
