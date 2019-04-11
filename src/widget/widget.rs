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

use piston_window::*;

use crate::core::callbacks::*;
use crate::core::point::*;
use crate::widget::config::*;

/// Implementable trait that is used by every `Widget`.  These are the public methods,
/// and a function _may_ override them.
///
/// You _must_ implement the following methods:
///
/// - config
/// - callbacks
///
/// You _should_ override `draw`, but you are not required to.  (If you don't, however, your
/// widget won't really do much.)
///
/// If you want a blank base widget, refer to the `CanvasWidget`, which will create a
/// base widget that paints the contents of its bounds with whatever color has been
/// specified with `set_color`.
pub trait Widget {
    /// Retrieves the configuration HashMap that stores the configuration list of settings
    /// for this widget.
    ///
    /// To implement this, the following code could be used in your object's structure:
    ///
    /// IN PROGRESS
    ///
    /// And in the overridden function for get_config in your implementation, use:
    ///
    /// IN PROGRESS
    ///
    /// This uses a `RefCell`, since configurations require a mutable reference to the HashMap
    /// that stores the configs.
    fn config(&mut self) -> &mut Configurable;

    /// Indicates that a widget needs to be redrawn/refreshed.
    fn invalidate(&mut self) {
        self.config().set(Invalidate);
    }

    /// Clears the invalidation flag.
    fn clear_invalidate(&mut self) {
        self.config().remove::<Invalidate>();
    }

    /// Checks to see whether or not the widget needs to be redrawn/refreshed.
    fn is_invalidated(&mut self) -> bool {
        self.config().contains_key::<Invalidate>()
    }

    /// Sets the `Point` of origin for this widget, given the X and Y origin points.  Invalidates the widget afterward.
    fn set_origin(&mut self, x: i32, y: i32) {
        self.config().set(Origin(Point { x, y }));
        self.invalidate();
    }

    /// Retrieves the `Point` of origin for this object.
    /// Defaults to origin (0, 0) if not set.
    fn get_origin(&mut self) -> Point {
        self.config()
            .get::<Origin>()
            .unwrap_or(&Origin(Point { x: 0, y: 0 }))
            .0
            .clone()
    }

    /// Sets the `Size` for this widget, given a width and height.  Invalidates the widget afterward.
    fn set_size(&mut self, w: i32, h: i32) {
        self.config()
            .set(BodySize(crate::core::point::Size { w, h }));
        self.invalidate();
    }

    /// Retrieves the `Size` bounds for this widget.
    /// Defaults to size (0, 0) if not set.
    fn get_size(&mut self) -> crate::core::point::Size {
        self.config()
            .get::<BodySize>()
            .unwrap_or(&BodySize(crate::core::point::Size { w: 0, h: 0 }))
            .0
            .clone()
    }

    /// Sets the color for this widget.  Invalidates the widget afterward.
    fn set_color(&mut self, color: types::Color) {
        self.config().set(MainColor(color));
        self.invalidate();
    }

    /// Retrieves the color of this widget.
    /// Defaults to white color `[1.0; 4]` if not set.
    fn get_color(&mut self) -> types::Color {
        self.config()
            .get::<MainColor>()
            .unwrap_or(&MainColor([1.0; 4]))
            .0
    }

    /// Sets the secondary color for this widget.  Invalidates the widget afterward.
    fn set_secondary_color(&mut self, color: types::Color) {
        self.config().set(SecondaryColor(color));
        self.invalidate();
    }

    /// Retrieves the secondary color of this widget.
    /// Defaults to black color `[0.0, 0.0, 0.0, 1.0]` if not set.
    fn get_secondary_color(&mut self) -> types::Color {
        self.config()
            .get::<SecondaryColor>()
            .unwrap_or(&SecondaryColor([1.0; 4]))
            .0
    }

    /// Sets the displayable text in a `Widget`, if applicable.
    fn set_text(&mut self, text: &str) {
        self.config().set(DisplayText(String::from(text)));
        self.invalidate();
    }

    /// Retrieves a copy of the text that is set for the `Widget`.  If no text is set,
    /// defaults to a blank string.
    fn get_text(&mut self) -> String {
        self.config()
            .get::<DisplayText>()
            .unwrap_or(&DisplayText(String::from("")))
            .0
            .clone()
    }

    /// Handles an event that was sent by the event loop.  It is up to the `Widget` to handle the
    /// event, or to ignore it.  If this function is _not_ overridden, the event will be ignored,
    /// and no event will be returned as a result.  This function _returns_ an `Option<CallbackEvent>`,
    /// which can be injected into the run loop.  This can be things for `Widget` interaction that
    /// may generate an event that the application needs to respond to, like a button click, or
    /// a drag start/end event.
    fn handle_event(&mut self, _event: CallbackEvent) -> Option<CallbackEvent> {
        None
    }

    // Draw routines

    /// Draws the contents of the widget, provided a `piston2d` `Context` and `G2d` object.
    ///
    /// It is **highly recommended** that you call `clear_invalidate()` after the draw completes,
    /// otherwise, this will continue to be redrawn continuously (unless this is the desired
    /// behavior.)
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size: crate::core::point::Size = self.get_size();

        Rectangle::new(self.get_color()).draw(
            [0.0 as f64, 0.0 as f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
            g,
        );

        self.clear_invalidate();
    }
}

/// This is the `CanvasWidget`, which contains a top-level widget for display.  It does
/// not contain any special logic other than being a base for a display layer.
///
/// Example usage:
/// IN PROGRESS
pub struct CanvasWidget {
    config: Configurable,
}

/// Implementation of the constructor for the `CanvasWidget`.  Creates a new base widget
/// that can be positioned anywhere on the screen.
impl CanvasWidget {
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
        }
    }
}

/// Implementation of the `CanvasWidget` object with the `Widget` traits implemented.
/// This function only implements `config` and `callbacks`, which are used as a base for
/// all `Widget`s.
impl Widget for CanvasWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }
}
