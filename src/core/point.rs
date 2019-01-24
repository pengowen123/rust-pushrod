// Geometric Point: X and Y positions
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

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub w: i32,
    pub h: i32,
}

// TODO: Add default Point (x, y = 0)
// TODO: Add default Size (w, h = 0)
// TODO: Add constructor impls for Point and Size, both of which take x, y, w, and h respectively as inputs.

pub fn make_point_i32(x: i32, y: i32) -> Point {
    Point { x, y }
}

pub fn make_point_f64(x: f64, y: f64) -> Point {
    Point {
        x: x as i32,
        y: y as i32,
    }
}
