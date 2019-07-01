// Horizontal Layout Manager
// Lays out Widgets in a Horizontal Area
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

use crate::core::layout_manager::*;
use crate::core::point::{Point, Size};
use crate::core::widget_store::*;

pub struct HorizontalLayoutManager {
    container_widget_id: i32,
}

impl HorizontalLayoutManager {
    pub fn new(widget_id: i32) -> Self {
        Self {
            container_widget_id: widget_id,
        }
    }
}

impl LayoutManager for HorizontalLayoutManager {
    fn do_layout(
        &mut self,
        size: Size,
        coordinates: LayoutManagerCoordinates,
    ) -> LayoutManagerCoordinates {
        let num_widgets = coordinates.widget_sizes.len() as i32;
        let width_per_widget = size.w / num_widgets;
        let mut widget_origins: Vec<Point> = vec![];
        let mut widget_sizes: Vec<Size> = vec![];
        let mut current_x: i32 = 0;

        for x in 0..num_widgets {
            current_x = width_per_widget * x;
            widget_origins.push(Point {
                x: current_x,
                y: coordinates.widget_origins[x as usize].y,
            });
            widget_sizes.push(Size {
                w: width_per_widget,
                h: size.h,
            });
        }

        LayoutManagerCoordinates {
            widget_origins: widget_origins.clone(),
            widget_sizes: widget_sizes.clone(),
            widget_positions: coordinates.widget_positions.clone(),
        }
    }

    fn get_widget_id(&self) -> i32 {
        return self.container_widget_id;
    }
}
