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

use opengl_graphics::GlGraphics;
use piston_window::*;

use std::cell::RefCell;
use std::collections::HashMap;

use crate::core::point::*;

/// Config entry key for invalidated object (invalidated means "requires screen refresh")
pub const CONFIG_INVALIDATE: u8 = 0;

/// Config entry key for retrieving the `Point` of origin.
pub const CONFIG_ORIGIN: u8 = 1;

/// Config entry key for retrieving the `Size` of the widget.
pub const CONFIG_SIZE: u8 = 2;

/// Config entry key for retrieving the widget's color.
pub const CONFIG_COLOR: u8 = 3;

/// Algebraic data type containing storage areas for each configuration object.
pub enum PushrodWidgetConfig {
    Invalidate {},
    Origin { point: Point },
    Size { size: crate::core::point::Size },
    Color { color: types::Color },
}

/// Implementable trait that is used by every `PushrodWidget`.  These are the public methods,
/// and a function _may_ override them.
///
/// You _must_ implement the following methods:
///
/// - get_config
/// - mouse_entered
/// - mouse_exited
/// - mouse_scrolled
///
/// You _should_ override `draw`, but you are not required to.
pub trait PushrodWidget {
    /// Retrieves the configuration HashMap that stores the configuration list of settings
    /// for this widget.
    ///
    /// To implement this, the following code could be used in your object's structure:
    ///
    /// ```
    ///   fn new() -> Self {
    ///     Self {
    ///       config: RefCell::new(HashMap::new()),
    ///     }
    ///   }
    /// ```
    ///
    /// And in the overridden function for get_config in your implementation, use:
    ///
    /// ```
    ///   fn get_config(&mut self) -> ... {
    ///     &self.config
    ///   }
    /// ```
    ///
    /// This uses a `RefCell`, since configurations require a mutable reference to the HashMap
    /// that stores the configs.
    fn get_config(&mut self) -> &RefCell<HashMap<u8, PushrodWidgetConfig>>;

    /// Sets a configuration object by its key.
    fn set_config(&mut self, key: u8, value: PushrodWidgetConfig) {
        self.get_config().borrow_mut().insert(key, value);
    }

    /// Indicates that a widget needs to be redrawn/refreshed.
    fn invalidate(&mut self) {
        self.set_config(CONFIG_INVALIDATE, PushrodWidgetConfig::Invalidate {});
    }

    /// Clears the invalidation flag.
    fn clear_invalidate(&mut self) {
        self.get_config().borrow_mut().remove(&CONFIG_INVALIDATE);
    }

    /// Checks to see whether or not the widget needs to be redrawn/refreshed.
    fn is_invalidated(&mut self) -> bool {
        self.get_config().borrow().contains_key(&CONFIG_INVALIDATE)
    }

    /// Sets the `Point` of origin for this widget.
    fn set_origin(&mut self, point: Point) {
        self.set_config(CONFIG_ORIGIN, PushrodWidgetConfig::Origin { point });
    }

    /// Retrieves the `Point` of origin for this object.
    fn get_origin(&mut self) -> Point {
        match self.get_config().borrow()[&CONFIG_ORIGIN] {
            PushrodWidgetConfig::Origin { ref point } => point.clone(),
            _ => make_origin_point(),
        }
    }

    /// Sets the `Size` for this widget.
    fn set_size(&mut self, size: crate::core::point::Size) {
        self.set_config(CONFIG_SIZE, PushrodWidgetConfig::Size { size });
    }

    /// Retrieves the `Size` bounds for this widget.
    fn get_size(&mut self) -> crate::core::point::Size {
        match self.get_config().borrow()[&CONFIG_SIZE] {
            PushrodWidgetConfig::Size { ref size } => size.clone(),
            _ => crate::core::point::Size { w: 0, h: 0 },
        }
    }

    /// Sets the color for this widget.
    fn set_color(&mut self, color: types::Color) {
        self.set_config(CONFIG_COLOR, PushrodWidgetConfig::Color { color });
    }

    /// Retrieves the color of this widget.
    fn get_color(&mut self) -> types::Color {
        match self.get_config().borrow()[&CONFIG_COLOR] {
            PushrodWidgetConfig::Color { color } => color,
            _ => [1.0; 4],
        }
    }

    // Events

    /// Called when a mouse enters the bounds of the widget.
    fn mouse_entered(&mut self);

    /// Called when a mouse exits the bounds of the widget.
    fn mouse_exited(&mut self);

    /// Called when a scroll event is called within the bounds of the widget.
    fn mouse_scrolled(&mut self, point: Point);

    // Draw routines

    /// Draws the contents of the widget, provided a `piston2d` `Context` and `GlGraphics` object.
    fn draw(&mut self, context: Context, graphics: &mut GlGraphics) {
        let origin: Point = self.get_origin();
        let size: crate::core::point::Size = self.get_size();

        rectangle(
            self.get_color(),
            [
                origin.x as f64,
                origin.y as f64,
                size.w as f64,
                size.h as f64,
            ],
            context.transform,
            graphics,
        );
        context.reset();
    }
}

/// Base widget structure.
pub struct PushrodBaseWidget {
    config: RefCell<HashMap<u8, PushrodWidgetConfig>>,
}

/// Implementation of the PushrodBaseWidget.  Creates a new base widget object.
impl PushrodBaseWidget {
    pub fn new() -> Self {
        Self {
            config: RefCell::new(HashMap::new()),
        }
    }
}

/// Implementation of the PushrodBaseWidget.
impl PushrodWidget for PushrodBaseWidget {
    fn get_config(&mut self) -> &RefCell<HashMap<u8, PushrodWidgetConfig>> {
        &self.config
    }

    fn mouse_entered(&mut self) {
        eprintln!("Mouse entered");
    }

    fn mouse_exited(&mut self) {
        eprintln!("Mouse exited");
    }

    fn mouse_scrolled(&mut self, point: Point) {
        eprintln!("Mouse scrolled: x={} y={}", point.x, point.y);
    }
}
