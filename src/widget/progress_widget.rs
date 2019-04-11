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

use piston_window::*;

use crate::core::point::*;
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
    progress: u16,
}

/// Implementation of the constructor for the `ProgressWidget`.
impl ProgressWidget {
    pub fn new() -> Self {
        let mut base = BoxWidget::new();

        base.set_border_thickness(1);
        base.set_border_color([0.0, 0.0, 0.0, 1.0]);

        Self {
            config: Configurable::new(),
            base_widget: base,
            progress: 0,
        }
    }

    /// Sets the progress to be indicated.  This is a number between 0 and 100.  (Anything over 100
    /// will just fill the box.)
    pub fn set_progress(&mut self, progress: u16) {
        self.progress = progress;
        self.invalidate();
    }

    /// Returns the current progress shown.
    pub fn get_progress(&mut self) -> u16 {
        self.progress
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

    /// Sets the `Point` of origin for this widget and the base widget, given the X and Y
    /// coordinates.  Invalidates the widget afterward.
    fn set_origin(&mut self, x: i32, y: i32) {
        self.config().set(Origin(Point { x, y }));
        self.base_widget.set_origin(x, y);
        self.invalidate();
    }

    /// Sets the `Size` for this widget and the base widget, given width and height.  Invalidates the widget afterward.
    fn set_size(&mut self, w: i32, h: i32) {
        self.config()
            .set(BodySize(crate::core::point::Size { w, h }));
        self.base_widget.set_size(w, h);
        self.invalidate();
    }

    /// Sets the color for this widget.  Invalidates the widget afterward.
    fn set_color(&mut self, color: types::Color) {
        self.base_widget.set_color(color);
        self.invalidate();
    }

    /// Draws the widget.  The progress bar is the secondary color.
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size: crate::core::point::Size = self.get_size();

        self.base_widget.draw(c, g, clip);

        let draw_width = (size.w as f64 * (self.progress as f64 / 100.0)) as f64;

        // Paint the secondary color to display the progress color.
        Rectangle::new(self.get_secondary_color()).draw(
            [1.0 as f64, 1.0 as f64, draw_width, (size.h - 2) as f64],
            clip,
            c.transform,
            g,
        );

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
