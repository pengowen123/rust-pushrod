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

/// Config entry key for retrieving the widget's border color.
pub const CONFIG_COLOR_BORDER: u8 = 4;

/// Config entry key for retrieving the widget's border width.
pub const CONFIG_BORDER_WIDTH: u8 = 5;

/// Enumeration data type containing storage areas for each configuration object.
pub enum PushrodWidgetConfig {
    /// Indicates that a widget's paint contents have become invalidated, and need to be redrawn.
    Invalidate {},

    /// `Point` of origin of this Widget.
    Origin { point: Point },

    /// `Size` of this widget.
    Size { size: crate::core::point::Size },

    /// The `types::Color` of this widget: `[f64; 4]` where the values are
    /// `[red, green, blue, transparency]`, values between 0 and 1.0.
    Color { color: types::Color },

    /// The `types::Color` of the border of this widget: `[f64; 4]` where the values are
    /// `[red, green, blue, transparency]`, values between 0 and 1.0.
    BorderColor { color: types::Color },

    /// Indicates the thickness of the border width to be drawn inside widgets that draw a
    /// border.  (See `BorderColor`.)
    BorderWidth { thickness: u8 },
}

/// Implementable trait that is used by every `Widget`.  These are the public methods,
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
///
/// If you want a blank base widget, refer to the `BaseWidget`, which will create a
/// base widget that paints the contents of its bounds with whatever color has been
/// specified with `set_color`.
pub trait Widget {
    /// Retrieves the configuration HashMap that stores the configuration list of settings
    /// for this widget.
    ///
    /// To implement this, the following code could be used in your object's structure:
    ///
    /// ```
    /// # use pushrod::widget::widget::Widget;
    /// # use pushrod::widget::widget::PushrodWidgetConfig;
    /// # use std::collections::HashMap;
    /// # use std::cell::RefCell;
    /// struct MyWidget {
    ///   config: RefCell<HashMap<u8, PushrodWidgetConfig>>
    /// }
    ///
    /// impl MyWidget {
    ///   fn new() -> Self {
    ///     Self {
    ///       config: RefCell::new(HashMap::new()),
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// And in the overridden function for get_config in your implementation, use:
    ///
    /// ```
    /// # use pushrod::widget::widget::Widget;
    /// # use pushrod::widget::widget::PushrodWidgetConfig;
    /// # use std::collections::HashMap;
    /// # use std::cell::RefCell;
    /// # use pushrod::core::point::Point;
    /// struct MyWidget {
    ///   config: RefCell<HashMap<u8, PushrodWidgetConfig>>
    /// }
    ///
    /// impl Widget for MyWidget {
    ///
    ///   fn get_config(&mut self) -> &RefCell<HashMap<u8, PushrodWidgetConfig>> {
    ///     &self.config
    ///   }
    ///
    ///  fn mouse_entered(&mut self, widget_id: i32) {}
    ///  fn mouse_exited(&mut self, widget_id: i32) {}
    ///  fn mouse_scrolled(&mut self, widget_id: i32, point: Point) {}
    /// }
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

    /// Sets the `Point` of origin for this widget.  Invalidates the widget afterward.
    fn set_origin(&mut self, point: Point) {
        self.set_config(CONFIG_ORIGIN, PushrodWidgetConfig::Origin { point });
        self.invalidate();
    }

    /// Retrieves the `Point` of origin for this object.
    /// Defaults to origin (0, 0) if not set.
    fn get_origin(&mut self) -> Point {
        match self.get_config().borrow().get(&CONFIG_ORIGIN) {
            Some(PushrodWidgetConfig::Origin { ref point }) => point.clone(),
            None => make_origin_point(),
            _ => make_origin_point(),
        }
    }

    /// Sets the `Size` for this widget.  Invalidates the widget afterward.
    fn set_size(&mut self, size: crate::core::point::Size) {
        self.set_config(CONFIG_SIZE, PushrodWidgetConfig::Size { size });
        self.invalidate();
    }

    /// Retrieves the `Size` bounds for this widget.
    /// Defaults to size (0, 0) if not set.
    fn get_size(&mut self) -> crate::core::point::Size {
        match self.get_config().borrow().get(&CONFIG_SIZE) {
            Some(PushrodWidgetConfig::Size { ref size }) => size.clone(),
            None => make_unsized(),
            _ => make_unsized(),
        }
    }

    /// Sets the color for this widget.  Invalidates the widget afterward.
    fn set_color(&mut self, color: types::Color) {
        self.set_config(CONFIG_COLOR, PushrodWidgetConfig::Color { color });
        self.invalidate();
    }

    /// Retrieves the color of this widget.
    /// Defaults to white color `[1.0; 4]` if not set.
    fn get_color(&mut self) -> types::Color {
        if self.get_config().borrow().contains_key(&CONFIG_COLOR) {
            match self.get_config().borrow()[&CONFIG_COLOR] {
                PushrodWidgetConfig::Color { color } => color,
                _ => [1.0; 4],
            }
        } else {
            [1.0; 4]
        }
    }

    // Events

    /// Called when a mouse enters the bounds of the widget.  Includes the widget ID.
    fn mouse_entered(&mut self, widget_id: i32);

    /// Called when a mouse exits the bounds of the widget.  Includes the widget ID.
    fn mouse_exited(&mut self, widget_id: i32);

    /// Called when a scroll event is called within the bounds of the widget.  Includes the widget ID.
    fn mouse_scrolled(&mut self, widget_id: i32, point: Point);

    // Draw routines

    /// Draws the contents of the widget, provided a `piston2d` `Context` and `GlGraphics` object.
    ///
    /// It is **highly recommended** that you call `clear_invalidate()` after the draw completes,
    /// otherwise, this will continue to be redrawn continuously (unless this is the desired
    /// behavior.)
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

        self.clear_invalidate();
    }
}

/// This is the `BaseWidget`, which contains a top-level widget for display.  It does
/// not contain any special logic other than being a base for a display layer.
pub struct BaseWidget {
    config: RefCell<HashMap<u8, PushrodWidgetConfig>>,
}

/// Implementation of the constructor for the `PushrodBaseWidget`.  Creates a new base widget
/// that can be positioned anywhere on the screen.
impl BaseWidget {
    pub fn new() -> Self {
        Self {
            config: RefCell::new(HashMap::new()),
        }
    }
}

/// Implementation of the `BaseWidget` object with the `Widget` traits implemented.
/// This function only implements `get_config`, and samples of `mouse_entered`, `mouse_exited`,
/// and `mouse_scrolled`, which currently trigger messages to the screen.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::core::window::*;
/// # use pushrod::widget::widget::*;
/// # fn main() {
/// #   let opengl = OpenGL::V3_2;
/// #   let mut pushrod_window: PushrodWindow = PushrodWindow::new(
/// #       WindowSettings::new("Pushrod Window", [640, 480])
/// #           .opengl(opengl)
/// #           .build()
/// #           .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)),
/// #   );
/// #
///    let mut base_widget = BaseWidget::new();
///
///    base_widget.set_origin(Point { x: 100, y: 100 });
///    base_widget.set_size(pushrod::core::point::Size { w: 200, h: 200 });
///    base_widget.set_color([0.5, 0.5, 0.5, 1.0]);
///
///    // PushrodWidgets must be boxed, as they are trait objects.
///    pushrod_window.add_widget(Box::new(base_widget));
/// # }
/// ```
impl Widget for BaseWidget {
    fn get_config(&mut self) -> &RefCell<HashMap<u8, PushrodWidgetConfig>> {
        &self.config
    }

    fn mouse_entered(&mut self, widget_id: i32) {
        eprintln!("[Base] Mouse entered: id={}", widget_id);
    }

    fn mouse_exited(&mut self, widget_id: i32) {
        eprintln!("[Base] Mouse exited: id={}", widget_id);
    }

    fn mouse_scrolled(&mut self, widget_id: i32, point: Point) {
        eprintln!(
            "[Base] Mouse scrolled: x={} y={}: id={}",
            point.x, point.y, widget_id
        );
    }
}

