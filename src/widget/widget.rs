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

pub const MAX_CONFIGS: usize = 40;
pub const WIDGET_CONFIG_ORIGIN: u8 = 0;

pub enum PushrodWidgetConfig {
    Origin { point: Point },
    Size { size: crate::core::point::Size },
}

pub enum PushrodWidget {
    BaseWidget { configs: HashMap<u8, PushrodWidgetConfig>, }
}

//trait PushrodBaseWidgetTrait {
//    fn get_config<'a>(&'a self) -> &'a HashMap<u8, PushrodWidgetConfig>;
//    fn set_origin(&mut self, point: Point) {
//        self.get_config().entry(WIDGET_CONFIG_ORIGIN).set(point);
//    }
//}
//
//impl PushrodBaseWidgetTrait for PushrodWidget::BaseWidget {
//    fn get_config<'a>(&'a self) -> &'a HashMap<u8, PushrodWidgetConfig> {
//        &self.configs
//    }
//}
//
//
//pub trait PushrodWidgetEvents {
//    fn origin(&mut self) -> &Point;
//    fn size(&mut self) -> &crate::core::point::Size;
//
//    fn on_signal(&mut self, signal: PushrodWidgetSignal);
//    fn on_draw(&mut self, context: Context, graphics: &mut GlGraphics);
//}
//
//pub struct PushrodWidget {
//    origin: Point,
//    size: crate::core::point::Size,
//}
//
//impl PushrodWidget {
//    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
//        Self {
//            origin: Point { x, y },
//            size: crate::core::point::Size { w, h },
//        }
//    }
//}
//
//impl PushrodWidgetEvents for PushrodWidget {
//    fn origin(&mut self) -> &Point {
//        &self.origin
//    }
//
//    fn size(&mut self) -> &crate::core::point::Size {
//        &self.size
//    }
//
//    fn on_signal(&mut self, _signal: PushrodWidgetSignal) {
//        println!("On signal received.");
//    }
//
//    fn on_draw(&mut self, _context: Context, graphics: &mut GlGraphics) {
//        clear([1.0; 4], graphics);
//    }
//}
