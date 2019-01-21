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

use opengl_graphics::GlGraphics;
use piston_window::*;
use std::collections::HashMap;

use crate::core::point::*;
use crate::widget::signal::*;

pub const CONFIG_ORIGIN: u8 = 0;
pub const CONFIG_SIZE: u8 = 1;
pub const CONFIG_COLOR: u8 = 2;

pub enum PushrodWidgetConfig {
    Origin { point: Point },
    Size { size: crate::core::point::Size },
    Color { color: types::Color },
}

pub trait PushrodWidget {
    fn get_config(&mut self) -> HashMap<u8, PushrodWidgetConfig>;

    fn get_origin(&mut self) -> Point {
        match self.get_config()[&CONFIG_ORIGIN] {
            PushrodWidgetConfig::Origin { ref point } => Point {
                x: point.x,
                y: point.y,
            },
            _ => Point { x: 0, y: 0 },
        }
    }

    fn get_size(&mut self) -> crate::core::point::Size {
        match self.get_config()[&CONFIG_SIZE] {
            PushrodWidgetConfig::Size { ref size } => crate::core::point::Size {
                w: size.w,
                h: size.h,
            },
            _ => crate::core::point::Size { w: 0, h: 0 },
        }
    }

    fn get_color(&mut self) -> types::Color {
        match self.get_config()[&CONFIG_COLOR] {
            PushrodWidgetConfig::Color { color } => color,
            _ => [1.0; 4],
        }
    }

    fn draw(&mut self, context: Context, graphics: &mut GlGraphics) {
        let origin: Point = self.get_origin();
        let size: crate::core::point::Size = self.get_size();

        context.draw_state.scissor([origin.x as u32, origin.y as u32, size.w as u32, size.h as u32]);
        clear(self.get_color(), graphics);
        context.reset();
    }
}
