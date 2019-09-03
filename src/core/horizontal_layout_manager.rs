// Horizontal Layout Manager
// Lays out Widgets Horizontally in a Bounding Box
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

pub struct HorizontalLayoutManager {
    container_widget_id: i32,
    padding: LayoutManagerPadding,
}

impl HorizontalLayoutManager {
    pub fn new(widget_id: i32, padding: LayoutManagerPadding) -> Self {
        Self {
            container_widget_id: widget_id,
            padding,
        }
    }
}

impl LayoutManager for HorizontalLayoutManager {
    fn do_layout(
        &mut self,
        origin: Point,
        size: Size,
        coordinates: LayoutManagerCoordinates,
    ) -> LayoutManagerCoordinates {
        let num_widgets = coordinates.widget_sizes.len() as i32;
        let drawing_width = size.w - self.padding.left - self.padding.right;

        let width_per_widget = drawing_width / num_widgets;
        let height_per_width = size.h - self.padding.top - self.padding.bottom;
        let current_y: i32 = origin.y + self.padding.top;

        let mut widget_origins: Vec<Point> = vec![];
        let mut widget_sizes: Vec<Size> = vec![];
        let mut current_x: i32 = origin.x + self.padding.left;
        let width_spacing_difference = drawing_width - (width_per_widget * num_widgets);

        for x in 0..num_widgets {
            if x > 0 {
                current_x += width_per_widget;
            }

            widget_origins.push(Point {
                x: current_x,
                y: current_y,
            });

            if x == num_widgets - 1 {
                widget_sizes.push(Size {
                    w: width_per_widget + width_spacing_difference,
                    h: height_per_width,
                });
            } else {
                widget_sizes.push(Size {
                    w: width_per_widget - self.padding.spacing,
                    h: height_per_width,
                });
            }
        }

        LayoutManagerCoordinates {
            widget_origins: widget_origins.clone(),
            widget_sizes: widget_sizes.clone(),
            widget_positions: coordinates.widget_positions.clone(),
        }
    }

    fn adjust_layout(&mut self, coordinates: LayoutManagerPadding) {
        self.padding = coordinates.clone()
    }

    fn get_widget_id(&self) -> i32 {
        return self.container_widget_id;
    }
}
