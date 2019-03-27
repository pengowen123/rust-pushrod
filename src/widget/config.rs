// Configurable Implementation
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

/// A programmatic type identifying the type of key (and most importantly, size) that is used
/// for storing configuration values for a `Widget` in the config `HashMap`.
pub type ConfigKey = u8;

/// Config entry key for invalidated object (invalidated means "requires screen refresh")
pub const CONFIG_INVALIDATE: u8 = 0;

/// Config entry key for retrieving the `Point` of origin.
pub const CONFIG_ORIGIN: u8 = 1;

/// Config entry key for retrieving the `Size` of the widget.
pub const CONFIG_SIZE: u8 = 2;

/// Config entry key for retrieving the widget's color.
pub const CONFIG_COLOR: u8 = 3;

/// Config entry key for retrieving the widget's border color.
pub const CONFIG_COLOR_BORDER: u8 = 4;

/// Config entry key for retrieving the widget's border width.
pub const CONFIG_BORDER_WIDTH: u8 = 5;

/// Config entry key for retrieving the widget's text color.
pub const CONFIG_TEXT_COLOR: u8 = 6;

/// Enumeration data type containing storage areas for each configuration object.
pub enum WidgetConfig {
    /// Indicates that a widget's paint contents have become invalidated, and need to be redrawn.
    Invalidate {},

    /// `Point` of origin of this Widget.
    Origin { point: Point },

    /// `Size` of this widget.
    Size { size: crate::core::point::Size },

    /// The `types::Color` of this widget: `[f64; 4]` where the values are
    /// `[red, green, blue, transparency]`, values between 0 and 1.0.
    Color { color: Color },

    /// The `types::Color` of the border of this widget: `[f64; 4]` where the values are
    /// `[red, green, blue, transparency]`, values between 0 and 1.0.
    BorderColor { color: Color },

    /// Indicates the thickness of the border width to be drawn inside widgets that draw a
    /// border.  (See `BorderColor`.)
    BorderWidth { thickness: u8 },

    /// The `types::Color` of the text for thsi widget: `[f64; 4]` where the values are
    /// `[red, green, blue, transparency]`, values between 0 and 1.0.
    TextColor { color: Color },
}

/// This structure is used for the configuration store of `Widget` settings.  It contains its
/// own structure internally, so all that is used inside extended `Widget` objects is a simple
/// instantiation of a new `Configurable` object as part of your extension.
pub struct Configurable {
    config: HashMap<ConfigKey, WidgetConfig>,
}

/// Implementation of the `Configurable` object.  Contains methods to extend the `HashMap` that
/// is used for underlying storage.
impl Configurable {
    /// Creates a new instance of this object.
    pub fn new() -> Self {
        Self {
            config: HashMap::new(),
        }
    }

    /// Sets a configuration key by its `ConfigKey` ID, assigning a new `WidgetConfig` value
    /// to that key.
    pub fn set(&mut self, key: ConfigKey, value: WidgetConfig) {
        self.config.insert(key, value);
    }

    /// Retrieves an `Option<&WidgetConfig>` for the key specified.  If the key does not exist,
    /// a `None` is returned.
    pub fn get(&self, key: ConfigKey) -> Option<&WidgetConfig> {
        self.config.get(&key)
    }

    /// Removes the value for the specified key, if one exists.
    pub fn remove(&mut self, key: ConfigKey) {
        self.config.remove(&key);
    }

    /// Indicates whether or not a `Configurable` store contains a value for the specified key.
    /// Returns `true` if one is stored, `false` otherwise.
    pub fn contains_key(&self, key: ConfigKey) -> bool {
        self.config.contains_key(&key)
    }
}
