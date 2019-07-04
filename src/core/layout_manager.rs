// Layout Manager Trait
// Describes how a Layout Manager behaves
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

use crate::core::point::{Point, Size};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutManagerPadding {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutManagerCoordinates {
    pub widget_origins: Vec<Point>,
    pub widget_sizes: Vec<Size>,
    pub widget_positions: Vec<Point>,
}

pub trait LayoutManager {
    fn do_layout(
        &mut self,
        origin: Point,
        size: Size,
        coordinates: LayoutManagerCoordinates,
    ) -> LayoutManagerCoordinates;

    fn get_widget_id(&self) -> i32;
}
