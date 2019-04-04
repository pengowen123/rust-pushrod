// Image Widget
// Draws an image in a specified bounding area.
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

/// This is the `ImageWidget`, which draws an image on the screen.  The image is loaded into
/// heap memory (using a `Box`).  This way, larger image objects can be loaded.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::core::main::*;
/// # use pushrod::widget::widget::*;
/// # use pushrod::widget::image_widget::*;
/// # fn main() {
/// let mut window: PistonWindow = WindowSettings::new("Pushrod Window", [800, 600])
///       .opengl(OpenGL::V3_2)
///       .resizable(false)
///       .build()
///       .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));
///    let mut prod: Pushrod = Pushrod::new(window);
///    let mut image_widget = ImageWidget::new(prod.get_factory(),
///       "rust-512x512.jpg".to_string());
///
///    image_widget.set_origin(25, 25);
///    image_widget.set_size(200, 200);
///
///    prod.widget_store.add_widget(Box::new(image_widget));
/// # }
/// ```
pub struct ImageWidget {
    config: Configurable,
    callbacks: CallbackStore,
    image: Box<G2dTexture>,
    image_size: crate::core::point::Size,
}

/// Implementation of the constructor for the `ImageWidget`.  Creates a new image object to be
/// displayed on the screen, given the image filename.
impl ImageWidget {
    /// Creates a new `ImageWidget` object, requiring the current `PistonWindow`'s factory object
    /// (which can be cloned), and the name of the image to load.  The image should be in the
    /// project's local `assets` directory at the top level.
    pub fn new(factory: &mut GfxFactory, image_name: String) -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let texture = Texture::from_path(
            factory,
            &assets.join(image_name),
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();

        Self {
            config: Configurable::new(),
            callbacks: CallbackStore::new(),
            image: Box::new(texture.clone()),
            image_size: crate::core::point::Size {
                w: texture.clone().get_size().0 as i32,
                h: texture.clone().get_size().1 as i32,
            },
        }
    }
}

/// Implementation of the `ImageWidget` object.  Draws an image on the screen based on the
/// image file you specify.
impl Widget for ImageWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn callbacks(&mut self) -> &mut CallbackStore {
        &mut self.callbacks
    }

    /// Draws the contents of the widget.
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size = self.get_size();
        let transform = c.transform.scale(
            size.w as f64 / self.image_size.w as f64,
            size.h as f64 / self.image_size.h as f64,
        );

        Image::new().draw(&*self.image, clip, transform, g);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
