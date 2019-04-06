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

use piston_window::*;

use crate::core::callbacks::*;
use crate::widget::config::*;
use crate::widget::widget::*;

mod private {
    use piston_window::character::CharacterCache;
    use piston_window::math::Matrix2d;
    use piston_window::types::{Color, FontSize};
    use piston_window::{color, DrawState, Graphics, Image, Transformed};

    pub struct TextHelper {
        /// The font size
        pub font_size: FontSize,
    }

    impl TextHelper {
        /// Creates a new text with black color
        pub fn new(font_size: FontSize) -> TextHelper {
            TextHelper { font_size }
        }

        /// Determines draw width and height with a character cache
        pub fn determine_size<C, G>(
            &self,
            text: &str,
            cache: &mut C,
            g: &mut G,
        ) -> Result<(i32, i32), C::Error>
            where
                C: CharacterCache,
                G: Graphics<Texture=<C as CharacterCache>::Texture>,
        {
            let mut x = 0.0;
            let mut y = 0.0;
            for ch in text.chars() {
                let character = cache.character(self.font_size, ch)?;
                x += character.width();
                y += character.height();
            }
            Ok((x as i32, y as i32))
        }
    }
}

/// This `enum` specifies the desired justification of the text to be drawn.
pub enum TextJustify {
    /// Left-justified text.
    Left,

    /// Center-justified text: `(total width - text width) / 2`
    Center,

    /// Right-justified text: `(total width - text width)`
    Right,
}

/// This is the `TextWidget`, which draws a line of text on the screen.  This structure contains
/// no accessable objects, they are all internal to `TextWidget`'s implementation.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::core::main::*;
/// # use pushrod::widget::widget::*;
/// # use pushrod::widget::text_widget::*;
/// # fn main() {
/// let mut window: PistonWindow = WindowSettings::new("Pushrod Window", [800, 600])
///       .opengl(OpenGL::V3_2)
///       .resizable(false)
///       .build()
///       .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));
///    let factory: GfxFactory = window.factory.clone();
///    let mut prod: Pushrod = Pushrod::new(window);
///    let mut text_widget = TextWidget::new(
///       prod.get_factory(),
///       "OpenSans-Regular.ttf".to_string(),
///       "Hello, World!".to_string(),
///       32,
///       TextJustify::Center,
///    );
///
///    text_widget.set_origin(8, 8);
///    text_widget.set_size(400, 48);
///    text_widget.set_color([0.75, 0.75, 1.0, 1.0]);
///    text_widget.set_text_color([0.0, 0.0, 1.0, 1.0]);
///    prod.widget_store.add_widget(Box::new(text_widget));
/// # }
/// ```
pub struct TextWidget {
    config: Configurable,
    callbacks: CallbackStore,
    font_cache: Glyphs,
    text: String,
    font_size: u32,
    justify: TextJustify,
    desired_size: (i32, i32),
}

/// Implementation of the constructor for the `TextWidget`.  Creates a new text object to be
/// displayed on the screen, given a font name, font size, and text message.
impl TextWidget {
    /// Creates a new `TextWidget` object, requiring the current `PistonWindow`'s factory object
    /// (which can be cloned), the name of the font (filename in the `assets` directory), the
    /// text to display, the font size in which to use, and the desired text justification
    /// strategy.
    pub fn new(
        factory: &mut GfxFactory,
        font_name: String,
        text: String,
        font_size: u32,
        justify: TextJustify,
    ) -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let ref font = assets.join(font_name.clone());
        let glyphs = Glyphs::new(font, factory.clone(), TextureSettings::new()).unwrap();

        Self {
            config: Configurable::new(),
            callbacks: CallbackStore::new(),
            font_cache: glyphs,
            text,
            font_size,
            justify,
            desired_size: (0, 0),
        }
    }

    /// Sets the color of the text for this `Widget`.
    pub fn set_text_color(&mut self, color: types::Color) {
        self.config().set(TextColor(color));
        self.invalidate();
    }

    /// Retrieves the color of the text for this `Widget`.
    /// Defaults to black if not set.
    pub fn get_text_color(&mut self) -> types::Color {
        self.config().get::<TextColor>().unwrap().0
    }

    /// Changes the text, recalculates the desired draw size, and redraws after change.
    pub fn set_text(&mut self, text: String) {
        self.text = text.clone();
        self.desired_size = (0, 0);
        self.invalidate();
    }

    /// Function to draw the text.  Generates a context transformation to display the text based on
    /// the point of origin's X and Y coordinates.  Since the text is drawn upwards from the point
    /// of origin, the starting point is the lower left-hand corner of the widget.  (This may change
    /// based on text justification, and other optional padding, once padding is introduced.)
    pub fn draw_text(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let origin = self.get_origin();
        let size: crate::core::point::Size = self.get_size();
        let mut font_cache = &mut self.font_cache;

        // This prevents the calculation from occurring at every single draw cycle.  It only needs
        // to occur once.
        if self.desired_size.0 == 0 {
            self.desired_size = private::TextHelper::new(self.font_size)
                .determine_size(&self.text, &mut self.font_cache, g)
                .unwrap();

            eprintln!("Desired size={:?} bounds={:?}", self.desired_size, size);
        }

        // Modify transform here based on the width of the text being drawn, which is element 0 of
        // self.desired_size
        let start_x = match self.justify {
            TextJustify::Left => 0.0,
            TextJustify::Center => ((size.w - self.desired_size.0) / 2) as f64,
            TextJustify::Right => (size.w - self.desired_size.0) as f64,
        };

        // And draw the remaining text based on the starting point adjusted by the text justification.
        //
        // IMPORTANT NOTE:
        // The provided transform from the run loop must be modified, as Piston's text drawing
        // routines treats the top "y" value specified as the _baseline_ for the image drawing
        // start point.  We want to treat the _inside_ of the box as the baseline, so we simply
        // add the size of the font (in pixels), which adjusts the baseline to the desired area.
        Text::new_color(self.get_text_color(), self.font_size)
            .draw(
                &self.text,
                &mut self.font_cache,
                clip,
                c.transform.trans(start_x, self.font_size as f64),
                g,
            )
            .unwrap();
    }
}

/// Implementation of the `TextWidget` object with the `Widget` traits implemented.
impl Widget for TextWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn callbacks(&mut self) -> &mut CallbackStore {
        &mut self.callbacks
    }

    /// Draws the contents of the widget.
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        // Draw the text.
        self.draw_text(c, g, &clip);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
