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

extern crate graphics;

use piston_window::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::character::CharacterCache;

use crate::widget::config::*;
use crate::widget::widget::*;

/// Enumeration identifying the justification of the text to be drawn, as long as the bounds
/// of the object allow for it.
pub enum TextJustify {
    /// Left-justified text.
    Left,

    /// Center-justified text: `(total width - text width) / 2`
    Center,

    /// Right-justified text: `(total width - text width)`
    Right,
}

/// Draws a block of text.
pub struct TextWidget {
    config: Configurable,
    font_cache: GlyphCache<'static>,    // YUCK - I do not like this!
    font_size: u32,
    justify: TextJustify,
    desired_size: i32,
}

impl TextWidget {
    /// Constructor.  Requires the name of the font, the text to display, the size of the font,
    /// and the font justification when rendered.
    pub fn new(
        font_name: String,
        text: String,
        font_size: u32,
        justify: TextJustify,
    ) -> Self {
        let mut configurable = Configurable::new();
        let mut cache = GlyphCache::new(font_name.clone(), (), TextureSettings::new()).unwrap();
        let size = cache.width(font_size, &text).unwrap();

        configurable.set(CONFIG_DISPLAY_TEXT, Config::Text(text.clone()));

        Self {
            config: configurable,
            font_cache: cache,
            font_size,
            justify,
            desired_size: size as i32,
        }
    }

    fn draw_text(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);

        // Modify transform here based on the width of the text being drawn, which is element 0 of
        // self.desired_size
        let start_x = match self.justify {
            TextJustify::Left => 0.0,
            TextJustify::Center => ((size.w - self.desired_size) / 2) as f64,
            TextJustify::Right => (size.w - self.desired_size) as f64,
        };

        // Vertically justify the text as default.
        let start_y = (self.font_size - 2 + size.h as u32) / 2 - 1;

        // And draw the remaining text based on the starting point adjusted by the text justification.
        //
        // IMPORTANT NOTE:
        // The provided transform from the run loop must be modified, as Piston's text drawing
        // routines treats the top "y" value specified as the _baseline_ for the image drawing
        // start point.  We want to treat the _inside_ of the box as the baseline, so we simply
        // add the size of the font (in pixels), which adjusts the baseline to the desired area.
        text::Text::new_color(self.config().get_color(CONFIG_TEXT_COLOR), self.font_size)
            .draw(
                self.config().get_text(CONFIG_DISPLAY_TEXT).as_str(),
                &mut self.font_cache,
                clip,
                c.transform.trans(start_x, start_y as f64),
                g,
            )
            .unwrap();
    }
}

impl Widget for TextWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.invalidate();
    }

    /// Draws the contents of the widget.
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        // Draw the text.
        self.draw_text(c, g, &clip);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
