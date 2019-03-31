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

        #[derive(Default)]
        pub struct Configurable {
            $( $field: Option<$name>, )*
        }
    }
}

pub struct Invalidate;
pub struct Origin(pub Point);
pub struct BodySize(pub Size);
pub struct MainColor(pub Color);
pub struct BorderColor(pub Color);
pub struct BorderWidth(pub u8);
pub struct TextColor(pub Color);

impl_configurable! {
    Invalidate => invalidate,
    Origin => origin,
    BodySize => body_size,
    MainColor => main_color,
    BorderColor => border_color,
    BorderWidth => border_width,
    TextColor => text_color,
}

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
