// Progress Widget
// Handles the display of a progress bar
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

use crate::widget::box_widget::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// This is the `ProgressWidget`, which is used for showing a progress bar.
///
/// Example usage:
/// IN PROGRESS
pub struct ProgressWidget {
    config: Configurable,
    base_widget: BoxWidget,
}

/// Implementation of the constructor for the `ProgressWidget`.
impl ProgressWidget {
    pub fn new() -> Self {
        let mut base = BoxWidget::new();

        base.config().set(CONFIG_BORDER_WIDTH, Config::Numeric(1));
        base.config()
            .set(CONFIG_BORDER_COLOR, Config::Color([0.0, 0.0, 0.0, 1.0]));

        // Configurable set: set progress.

        Self {
            config: Configurable::new(),
            base_widget: base,
        }
    }
}

/// Implementation of the `ProgressWidget` object with the `Widget` traits implemented.
/// The base widget is a `BoxWidget`, which overlays a `TextWidget` over the top.  This `Widget`
/// responds to the button down/up callbacks internally, and generates an `on_clicked` callback
/// when appropriate.
impl Widget for ProgressWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.base_widget.config().set(config, config_value.clone());
        self.invalidate();
    }

    /// Draws the widget.  The progress bar is the secondary color.
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size = self.config().get_size(CONFIG_BODY_SIZE);

        self.base_widget.draw(c, g, clip);

        let draw_width =
            (size.w as f64 * (self.config().get_numeric(CONFIG_PROGRESS) as f64 / 100.0)) as f64;

        // Paint the secondary color to display the progress color.
        Rectangle::new(self.config().get_color(CONFIG_SECONDARY_COLOR)).draw(
            [1.0 as f64, 1.0 as f64, draw_width, (size.h - 2) as f64],
            clip,
            c.transform,
            g,
        );

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
