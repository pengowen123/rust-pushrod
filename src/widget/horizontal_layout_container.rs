// Horizontal Container Widget
// Horizontally lays out child widgets within its bounds
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

use crate::core::callbacks::*;
use crate::core::point::{Size, Point};
use crate::widget::config::*;
use crate::widget::widget::*;
use crate::widget::container_widget::*;

pub struct HorizontalContainerWidget {
    config: Configurable,
}

impl HorizontalContainerWidget {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
        }
    }

    fn reposition_children(&mut self) {

    }
}

impl ContainerWidgetTrait for HorizontalContainerWidget {

    fn on_resize(&mut self, _size: Size) {
        eprintln!("[HorizontalContainerWidget] Handle resize.");

        self.reposition_children();
    }

    fn draw_container(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        eprintln!("[HorizontalContainerWidget] Handle drawing of container here.");
    }

    fn add_widget(&mut self, _widget: Box<Widget>, _positioning: Point) -> i32 {
        self.reposition_children();
        0
    }


}
