// Container Widget
// Used to contain a set of widgets that can be displayed or hidden
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

use graphics::*;
use opengl_graphics::GlGraphics;

use crate::widget::config::*;
use crate::widget::widget::*;
use crate::core::callbacks::*;
use crate::core::point::Size;

/// Containers that handle resize events should implement this trait when extending
/// a `ContainerWidget`, as the container widget itself needs to resize its known
/// contained widgets according to the rules of the container bounds.
pub trait ContainerWidgetTrait {
    fn handle_resize(&mut self, _size: Size) { }
}

/// A `ContainerWidget` is a `CanvasWidget` that only contains a backing color.
pub struct ContainerWidget {
    config: Configurable,
}

impl ContainerWidget {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
        }
    }
}

impl ContainerWidgetTrait for ContainerWidget { }

impl Widget for ContainerWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn handle_event(&mut self, injected: bool, event: CallbackEvent) -> Option<CallbackEvent> {
        if injected {
            return None;
        }

        match event {
            CallbackEvent::WindowResized { size } => {
                eprintln!("Handle resize of container");
                self.handle_resize(size);

                eprintln!("Invalidate container");
                self.invalidate();
            },
            _ => (),
        }

        None
    }

    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);

        g.rectangle(
            &Rectangle::new(self.config().get_color(CONFIG_MAIN_COLOR)),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
        );

        self.clear_invalidate();
    }
}
