// Text Widget
// Draws text in a specified bounding area.
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

use crate::core::point::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// This is the `BoxWidget`, which contains a top-level widget for display, overriding the
/// draw method to draw the base widget and the border for this box.
pub struct TextWidget {
    config: Configurable,
    font_cache: Glyphs,
    text: String,
    font_size: u32,
}

/// Implementation of the constructor for the `BaseWidget`.  Creates a new base widget
/// that can be positioned anywhere on the screen.
impl TextWidget {
    pub fn new(factory: GfxFactory, font_name: String, text: String, font_size: u32) -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let ref font = assets.join(font_name.clone());
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        Self {
            config: Configurable::new(),
            font_cache: glyphs,
            text,
            font_size,
        }
    }

    /// Sets the thickness of the border for this widget.
    pub fn set_text_color(&mut self, color: types::Color) {
        self.config()
            .set(CONFIG_TEXT_COLOR, WidgetConfig::TextColor { color });
        self.invalidate();
    }

    /// Retrieves the border thickness of this widget.
    /// Defaults to 1 if not set.
    pub fn get_text_color(&mut self) -> types::Color {
        match self.config().get(CONFIG_TEXT_COLOR) {
            Some(WidgetConfig::TextColor { color }) => color.clone(),
            _ => [1.0; 4],
        }
    }

    /// Changes the text, redraws after change.
    pub fn set_text(&mut self, text: String) {
        self.text = text.clone();
        self.invalidate();
    }

    /// Function to draw a box for the point and size of this box.  Automatically draws the border
    /// along with the width of the border.  This is automatically determined by the origin, so the
    /// box is automatically drawn for the bounds of the `Widget`.
    pub fn draw_text(&mut self, c: Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);
        text(
            [0.0, 0.0, 1.0, 1.0],
            self.font_size,
            &self.text,
            &mut self.font_cache,
            c.transform,
            g,
        );
    }
}

/// Implementation of the `BoxWidget` object with the `Widget` traits implemented.
/// This implementation is similar to the `BaseWidget`, but incorporates a drawable box inside
/// the widget.  Base widget is the `BaseWidget`.
///
/// This is basically just a box with a fill color.  Use this to draw other things like buttons,
/// text widgets, and so on, if you need anything with a drawable border.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::core::window::*;
/// # use pushrod::widget::widget::*;
/// # use pushrod::widget::box_widget::*;
/// # fn main() {
/// #   let opengl = OpenGL::V3_2;
/// #   let mut pushrod_window: PushrodWindow = PushrodWindow::new(
/// #       WindowSettings::new("Pushrod Window", [640, 480])
/// #           .opengl(opengl)
/// #           .build()
/// #           .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)),
/// #   );
/// #
///    let mut box_widget = BoxWidget::new();
///
///    box_widget.set_origin(100, 100);
///    box_widget.set_size(200, 200);
///    box_widget.set_color([0.5, 0.5, 0.5, 1.0]);
///    box_widget.set_border_color([0.0, 0.0, 0.0, 1.0]);
///    box_widget.set_border_thickness(3);
///
///    // (OR)
///
///    box_widget.set_border([0.0, 0.0, 0.0, 1.0], 3);
/// # }
/// ```
impl Widget for TextWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn mouse_entered(&mut self, widget_id: i32) {
        eprintln!("[Text] Mouse entered: id={}", widget_id);
    }

    fn mouse_exited(&mut self, widget_id: i32) {
        eprintln!("[Text] Mouse exited: id={}", widget_id);
    }

    fn mouse_scrolled(&mut self, widget_id: i32, point: Point) {
        eprintln!(
            "[Text] Mouse scrolled: x={} y={}: id={}",
            point.x, point.y, widget_id
        );
    }

    /// Draws the contents of the widget in this order:
    ///
    /// - Base widget first
    /// - Box graphic for the specified width
    fn draw(&mut self, c: Context, g: &mut G2d) {
        // Paint the box.
        self.draw_text(c, g);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
