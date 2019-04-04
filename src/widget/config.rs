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

use crate::core::point::Point;
use crate::core::point::Size;

/// Powerful macro that automatically creates a configuration object from a specified struct.
/// Each struct has its own getter, setter, removal of a key (by its value), and checking to see
/// if a key exists (by its value.)
macro_rules! impl_configurable {
    ($($name:ty => $field:ident,)*) => {
        pub trait ConfigKey: private::ConfigKeyInner {}
        $( impl ConfigKey for $name {} )*

        mod private {
            use super::*;

            pub trait ConfigKeyInner: Sized {
                fn field(config: &Configurable) -> &Option<Self>;
                fn field_mut(config: &mut Configurable) -> &mut Option<Self>;
            }

            $(
            impl ConfigKeyInner for $name {
                fn field(config: &Configurable) -> &Option<Self> {
                    &config.$field
                }
                fn field_mut(config: &mut Configurable) -> &mut Option<Self> {
                    &mut config.$field
                }
            }
            )*
        }

        /// Default Configurable object, created for each struct represented in the
        /// `impl_configurable!` macro.
        #[derive(Default)]
        pub struct Configurable {
            $( $field: Option<$name>, )*
        }
    }
}

/// Existence of this object indicates that a `Widget` needs to be redrawn.
pub struct Invalidate;

/// Origin `Point` at which a `Widget` exists on the display window.
pub struct Origin(pub Point);

/// Physical size of the `Widget`.
pub struct BodySize(pub Size);

/// Color of the body of the `Widget`.
pub struct MainColor(pub Color);

/// Color of the border for the `BoxWidget` and any `Widget` objects that contain a border.
pub struct BorderColor(pub Color);

/// Width (in pixels) of the border for the `BoxWidget` or any `Widget` objects that contain a border.
pub struct BorderWidth(pub u8);

/// `Color` of text to be displayed in a `TextWidget`.
pub struct TextColor(pub Color);

/// This macro implements the availability of configuration items.  The first value is the name
/// of the `struct` that the configuration object applies, and the second value is the name of the
/// private inner trait that is responsible for setting and getting values for that `struct`
impl_configurable! {
    Invalidate => invalidate,
    Origin => origin,
    BodySize => body_size,
    MainColor => main_color,
    BorderColor => border_color,
    BorderWidth => border_width,
    TextColor => text_color,
}

/// Implementation of the default `Configurable` object.
///
/// There are two ways in which configuration objects can be used:
/// ```
/// # use pushrod::widget::config::*;
/// # use pushrod::core::point::Point;
/// # use pushrod::core::point::Size;
/// fn main() {
///   let mut config: Configurable = Configurable::new();
///
///   config.set(Origin(Point { x: 0, y: 100 }));
///   config.set(BodySize(Size { w: 150, h: 150 }));
///
///   // To get the value of the Origin, you can use type inference:
///   let main_origin: &Origin = config.get().unwrap();
///
///   // Or you can use declared types with ::<> as such:
///   let body_size = &config.get::<BodySize>().unwrap().0;
/// }
/// ```
impl Configurable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T: ConfigKey>(&mut self, value: T) {
        *T::field_mut(self) = Some(value);
    }

    pub fn get<T: ConfigKey>(&self) -> Option<&T> {
        T::field(self).as_ref()
    }

    pub fn remove<T: ConfigKey>(&mut self) {
        *T::field_mut(self) = None;
    }

    pub fn contains_key<T: ConfigKey>(&self) -> bool {
        T::field(self).is_some()
    }
}
