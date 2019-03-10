// Callback Store
// Callback Cache using FnMut Enumerations for storing closures
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

use crate::event::event::*;
use crate::core::point::Point;

use std::collections::HashMap;

pub enum CallbackTypes {
    SingleCallback { callback: Box<FnMut(u32) -> ()> },
    PointCallback  { callback: Box<FnMut(u32, Point) -> ()> },
    ButtonCallback { callback: Box<FnMut(u32, u32, Point) -> ()> },
}

pub struct CallbackStore {
    store: HashMap<u32, CallbackTypes>,
}

impl CallbackStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: u32, callback: CallbackTypes) {
        self.store.insert(key, callback);
    }

    pub fn get(&self, key: u32) -> Option<&CallbackTypes> {
        self.store.get(&key)
    }
}