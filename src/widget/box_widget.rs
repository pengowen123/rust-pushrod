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

use graphics::*;
use opengl_graphics::GlGraphics;

use crate::core::callbacks::*;
use crate::core::point::Size;
use crate::widget::config::*;
use crate::widget::widget::*;
use crate::core::callbacks::CallbackEvent::WidgetMoved;

/// A `BoxWidget` is a `CanvasWidget` with a bounding box.  Takes two additional options:
/// * `CONFIG_BORDER_WIDTH` specifies the width of the border to be drawn in pixels.
/// * `CONFIG_BORDER_COLOR` specifies the color of the border to be drawn.
pub struct BoxWidget {
    config: Configurable,
    event_list: Vec<CallbackEvent>,
    widget_id: i32,
}

impl BoxWidget {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            event_list: vec![],
            widget_id: 0,
        }
    }

    fn draw_box(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);
        let border: f64 = self.config().get_numeric(CONFIG_BORDER_WIDTH) as f64;
        let color: types::Color = self.config().get_color(CONFIG_BORDER_COLOR);
        let fill_color: types::Color = self.config().get_color(CONFIG_MAIN_COLOR);

        g.rectangle(
            &Rectangle::new(fill_color),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform.clone(),
        );

        Rectangle::new_border(color, border).draw(
            [
                0.0 as f64 + border as f64,
                0.0 as f64 + border as f64,
                size.w as f64 - (border as f64 * 2.0),
                size.h as f64 - (border as f64 * 2.0),
            ],
            clip,
            c.transform.clone(),
            g,
        );
    }
}

impl Widget for BoxWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value);
        self.invalidate();
    }

    fn set_widget_id(&mut self, widget_id: i32) {
        self.widget_id = widget_id;
    }

    fn get_widget_id(&mut self) -> i32 {
        self.widget_id
    }

    fn inject_system_event(&mut self) -> Option<CallbackEvent> {
        self.event_list.pop().clone()
    }

    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        // Paint the box.
        self.draw_box(c, g, &clip);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
