// Configurable Implementation
// New configuration module, as described by u/JayDepp on Reddit - THANKS!!!
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

use piston_window::types::Color;

use std::collections::HashMap;

use crate::core::point::Point;
use crate::core::point::Size;

#[derive(Clone, Debug)]
pub enum Config {
    Point(Point),
    Size(Size),
    Color(Color),
    Numeric(u64),
    Text(String),
    Toggle(bool),
}

pub const CONFIG_INVALIDATE: u8 = 1;
pub const CONFIG_ORIGIN: u8 = 2;
pub const CONFIG_BODY_SIZE: u8 = 3;
pub const CONFIG_MAIN_COLOR: u8 = 4;
pub const CONFIG_BORDER_COLOR: u8 = 5;
pub const CONFIG_TEXT_COLOR: u8 = 6;
pub const CONFIG_SECONDARY_COLOR: u8 = 7;
pub const CONFIG_BORDER_WIDTH: u8 = 8;
pub const CONFIG_DISPLAY_TEXT: u8 = 9;
pub const CONFIG_PROGRESS: u8 = 10;
pub const CONFIG_TIMER_ENABLED: u8 = 11;
pub const CONFIG_TIMER_TIMEOUT: u8 = 12;
pub const CONFIG_WIDGET_HIDDEN: u8 = 13;
pub const CONFIG_WIDGET_DISABLED: u8 = 14;

pub struct Configurable {
    configs: HashMap<u8, Config>,
}

/// Implementation of the default `Configurable` object.
///
/// There are two ways in which configuration objects can be used:
/// NEEDS_DOCS
impl Configurable {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    pub fn set(&mut self, config: u8, config_value: Config) {
        self.configs.insert(config, config_value.clone());
    }

    pub fn remove(&mut self, config: u8) {
        self.configs.remove(&config);
    }

    pub fn contains(&self, config: u8) -> bool {
        self.configs.contains_key(&config)
    }

    pub fn get(&self, config: u8) -> Option<&Config> {
        self.configs.get(&config)
    }

    pub fn set_point(&mut self, config: u8, x: i32, y: i32) {
        self.set(config, Config::Point(Point { x, y }));
    }

    pub fn set_size(&mut self, config: u8, w: i32, h: i32) {
        self.set(config, Config::Size(Size { w, h }));
    }

    pub fn set_color(&mut self, config: u8, color: Color) {
        self.set(config, Config::Color(color));
    }

    pub fn set_numeric(&mut self, config: u8, value: u64) {
        self.set(config, Config::Numeric(value));
    }

    pub fn set_text(&mut self, config: u8, text: String) {
        self.set(config, Config::Text(text.clone()));
    }

    pub fn set_toggle(&mut self, config: u8, flag: bool) {
        self.set(config, Config::Toggle(flag));
    }

    pub fn get_point(&self, config: u8) -> Point {
        match self.configs.get(&config) {
            Some(Config::Point(point)) => point.clone(),
            _ => Point::default(),
        }
    }

    pub fn get_size(&self, config: u8) -> crate::core::point::Size {
        match self.configs.get(&config) {
            Some(Config::Size(size)) => size.clone(),
            _ => Size::default(),
        }
    }

    pub fn get_color(&self, config: u8) -> Color {
        match self.configs.get(&config) {
            Some(Config::Color(color)) => *color,
            _ => [1.0; 4],
        }
    }

    pub fn get_numeric(&self, config: u8) -> u64 {
        match self.configs.get(&config) {
            Some(Config::Numeric(numeric)) => *numeric,
            _ => 0,
        }
    }

    pub fn get_text(&self, config: u8) -> String {
        match self.configs.get(&config) {
            Some(Config::Text(text)) => text.clone(),
            _ => String::from(""),
        }
    }

    pub fn get_toggle(&self, config: u8) -> bool {
        match self.configs.get(&config) {
            Some(Config::Toggle(toggle)) => *toggle,
            _ => false,
        }
    }
}
