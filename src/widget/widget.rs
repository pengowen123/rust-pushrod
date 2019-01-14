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

pub trait PushrodWidget {
    fn origin(&mut self) -> Point;
    fn size(&mut self) -> Size;

    fn on_mouse_enter(&mut self);
    fn on_mouse_exit(&mut self);
    fn on_click(&mut self, clicks: u8);
}

pub struct PushrodWidget {
    origin: Point,
    size: Size,
}

pub impl PushrodBaseWidget {

    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            origin: Point { x, y },
            size: Size { w, h },
        }
    }

}

pub impl PushrodWidget for PushrodBaseWidget {

    pub fn origin(&mut self) -> Point {
        self.origin
    }

    pub fn size(&mut self) -> Size {
        self.size
    }

    pub fn on_mouse_enter(&mut self) {
        println!("BaseWidget: on_mouse_enter");
    }

    pub fn on_mouse_exit(&mut self) {
        println!("BaseWidget: on_mouse_exit");
    }

    pub fn on_click(&mut self) {
        println!("BaseWidget: on_click");
    }

}