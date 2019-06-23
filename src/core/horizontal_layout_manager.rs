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

pub struct HorizontalLayoutManager {}

impl LayoutManager for HorizontalLayoutManager {
    fn do_layout(
        &mut self,
        widget_ids: Vec<i32>,
        widget_positions: Vec<Point>,
        widget_store: &mut WidgetStore,
    ) {

    }

    fn resize(&mut self, size: Size, widget_store: &mut WidgetStore) {}
}
