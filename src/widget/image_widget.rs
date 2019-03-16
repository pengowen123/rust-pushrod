// Image Widget
// Draws image in a specified bounding area.
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
use crate::core::point::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// This is the `ImageWidget`
pub struct ImageWidget {
    config: Configurable,
    callbacks: CallbackStore,
    image: G2dTexture,
}

impl ImageWidget {
    pub fn new(image: G2dTexture) -> Self {
        Self {
            config: Configurable::new(),
            callbacks: CallbackStore::new(),
            image,
        }
    }
}

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
///       factory,
///       "OpenSans-Regular.ttf".to_string(),
///       "Welcome to Pushrod!".to_string(),
///       32,
///    );
///
///    text_widget.set_origin(8, 8);
///    text_widget.set_size(400, 40);
///    text_widget.set_color([0.75, 0.75, 1.0, 1.0]);
///    text_widget.set_text_color([0.0, 0.0, 1.0, 1.0]);
///    prod.widget_store.add_widget(Box::new(text_widget));
/// # }
/// ```
impl Widget for ImageWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn callbacks(&mut self) -> &mut CallbackStore {
        &mut self.callbacks
    }

    /// Draws the contents of the widget.
    fn draw(&mut self, c: Context, g: &mut G2d) {
        // Draw the text.
        image(&self.image, c.transform, g);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
