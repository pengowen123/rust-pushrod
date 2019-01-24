// Window Container
// Contains a PistonWindow and a list of widgets
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

use crate::core::point::*;
use crate::widget::widget::*;

use piston_window::*;

pub struct PushrodWindow {
    pub window: PistonWindow,
    widgets: Vec<Box<dyn PushrodWidget>>,
}

impl PushrodWindow {
    pub fn new(window: PistonWindow) -> Self {
        Self {
            window,
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn PushrodWidget>) {
        self.widgets.push(widget);
    }

    pub fn get_widget_id_for_point(&mut self, point: Point) -> u32 {
        for pos in 0..self.widgets.len() {
            let widget_point = self.widgets[pos].get_origin();
            let widget_size: crate::core::point::Size = self.widgets[pos].get_size();
            let start_x: i32 = widget_point.x;
            let end_x: i32 = widget_point.x + widget_size.w;
            let start_y: i32 = widget_point.y;
            let end_y: i32 = widget_point.y + widget_size.h;

            if point.x >= start_x && point.x <= end_x && point.y >= start_y && point.y <= end_y {
                return pos as u32;
            }
        }

        0
    }

    pub fn get_widget_for_id(&mut self, id: u32) -> &Box<dyn PushrodWidget> {
        &self.widgets[id as usize]
    }
}
