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

/// Structure identifying a point on the screen by X and Y coordinates.  X and Y coordinates
/// are represented from the upper left-hand corner of the base object.
#[derive(Clone, Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Structure identifying a size of an object by W (width) and H (height), respectively.
/// Other systems may use "width" and "height" as nomenclature, however, we wanted to keep
/// naming consistent.
#[derive(Clone, Debug, Default)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

/// Convenience method to create a new `Point`.
pub fn make_point_i32(x: i32, y: i32) -> Point {
    Point { x, y }
}

/// Convenience method to create a `Point` of origin, defined as an X and Y coordinate of 0.
///
/// Example:
/// ```
/// # use pushrod::core::point::*;
/// # fn main() {
///   let point = make_origin_point();
///   eprintln!("Point: {:?}", point);
/// # }
/// ```
pub fn make_origin_point() -> Point {
    Point { x: 0, y: 0 }
}

/// Convenience method to convert floating point X and Y positions to a graphical `Point`.
pub fn make_point_f64(x: f64, y: f64) -> Point {
    Point {
        x: x as i32,
        y: y as i32,
    }
}

/// Convenience method to create a non-existent size, defined as a width and height of 0.
///
/// Example:
/// ```
/// # use pushrod::core::point::*;
/// # fn main() {
///   let size = make_unsized();
///   eprintln!("Size: {:?}", size);
/// # }
/// ```
pub fn make_unsized() -> Size {
    Size { w: 0, h: 0 }
}
