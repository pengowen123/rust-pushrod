// Widget Base Definition
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
use crate::core::point::{Point, Size};
use crate::widget::config::*;

pub trait Widget {
    fn config(&mut self) -> &mut Configurable;

    fn invalidate(&mut self) {
        if !self.is_invalidated() {
            self.set_config(CONFIG_INVALIDATE, Config::Toggle(true));
        }
    }

    fn clear_invalidate(&mut self) {
        self.config().remove(CONFIG_INVALIDATE);
    }

    fn is_invalidated(&mut self) -> bool {
        self.config().contains(CONFIG_INVALIDATE)
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.invalidate();
    }

    fn get_config(&mut self, config: u8) -> Option<&Config> {
        self.config().get(config)
    }

    // Property setters

    fn set_point(&mut self, config: u8, x: i32, y: i32) {
        self.set_config(config, Config::Point(Point { x, y }));
    }

    fn set_size(&mut self, config: u8, w: i32, h: i32) {
        self.set_config(config, Config::Size(Size { w, h }));
    }

    fn set_color(&mut self, config: u8, color: types::Color) {
        self.set_config(config, Config::Color(color));
    }

    fn set_numeric(&mut self, config: u8, value: u64) {
        self.set_config(config, Config::Numeric(value));
    }

    fn set_text(&mut self, config: u8, text: String) {
        self.set_config(config, Config::Text(text.clone()));
    }

    fn set_toggle(&mut self, config: u8, flag: bool) {
        self.set_config(config, Config::Toggle(flag));
    }

    fn handle_event(&mut self, _injected: bool, _event: CallbackEvent) -> Option<CallbackEvent> {
        None
    }

    fn inject_event(&mut self, _widget_id: i32) -> Option<CallbackEvent> {
        None
    }

    fn injects_events(&mut self) -> bool {
        false
    }

    // Draw routines

    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);

        g.rectangle(
            &Rectangle::new(self.config().get_color(CONFIG_MAIN_COLOR)),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
        );

        self.clear_invalidate();
    }

    fn draw_disabled(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);

        g.rectangle(
            &Rectangle::new([0.0, 0.0, 0.0, 0.8]),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
        );
    }

    fn draw_with_offset(
        &mut self,
        c: Context,
        g: &mut G2d,
        clip: &DrawState,
        point_offset: Point,
    ) {
        self.draw(
            c.trans(point_offset.x as f64, point_offset.y as f64),
            g,
            clip,
        );
    }
}

pub struct CanvasWidget {
    config: Configurable,
}

impl CanvasWidget {
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
        }
    }
}

impl Widget for CanvasWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }
}
