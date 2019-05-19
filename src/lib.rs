// Pushrod
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

//! Pushrod is a Cross Platform UI Widget Library for Rust.
//!
//! It is intended to be lightweight, easy to use, and easy to understand.  Pushrod draws
//! inspiration from 16-bit GUI-based systems and other GUI libraries over the years.
//!
//! # Dependencies
//! Pushrod uses the following dependencies:
//! ```ignore
//! [dependencies]
//! piston_window = "^0.89.0"
//! find_folder = "^0.3.0"
//! ```
//!
//! To use the crate in your project, add the following dependencies:
//! ```ignore
//! [dependencies]
//! rust-pushrod = "^0.2"
//! ```
//!
//! # Core Components
//! `pushrod::core` is the _core_ library components, representing the main run loop, the callback
//! store mechanism, and the widget store.
//!
//! `pushrod::event` is the event system, in the works for 0.2.x.
//!
//! `pushrod::widget` is the core `Widget` library.
//!
//! # Events
//! Coming soon.
//!
//! # Callbacks
//! Callbacks are introduced in the `Widget` libraries as a way to action upon an event that
//! was triggered.  If no callback is registered for a particular event, it is bypassed, and
//! no default action occurs.
//!
//! The following callbacks are available:
//! - Main window resizing
//! - Main window (un)focusing
//! - Mouse pointer entering a `Widget`
//! - Mouse pointer exiting a `Widget`
//! - Mouse pointer moving inside a `Widget`
//! - Mouse scrolling a mouse wheel inside a `Widget`
//! - A keyboard press event happening inside a `Widget`
//! - Mouse button click down inside the scope of a `Widget`
//! - Mouse button click release inside and outside of a `Widget`
//!
//! # Widgets
//! The following `Widget` objects are provided:
//!
//! `BaseWidget` is a top-level `Widget` object.  It can be configured with
//! an origin, size, and background color.  If you wish to design your own `Widget`, you would
//! want to either extend the `BaseWidget`, or use it as a backing in your own `Widget` object.
//! (See `BoxWidget` for an example of how to use it in tandem with another object.)
//!
//! `BoxWidget` incorporates a `BaseWidget`, extending it to include a border color and width.
//!
//! `TextWidget` provides a way to display text in the main window.  Fonts are in `ttf` format,
//! which can be downloaded from Google, so long as they are free/open source.  One has been
//! included with the source distribution in the `assets` directory.
//!
//! `TimerWidget` provides a rudimentary timer, which increases ticks based on the screen
//! refresh.  After a timer has expired, a callback is triggered, allowing an action to occur.
//!
//! `ImageWidget` draws an image on the screen.  `png` format has been tested, but `jpg` and `gif`
//! formats should also work, as they are part of the Piston library.
//!
//! `PushButtonWidget` draws a `BoxWidget`, overlaying text on the top of it, justified in any
//! direction you desired (Left, Center, Right)  It produces an `on_clicked` callback when a
//! click is detected (with the left mouse button) inside the bounds of the widget (ie. it
//! overrides the mouse button click inside event.)
//!
//! `ToggleButtonWidget` operates the same as a `PushButtonWidget`, but offers the ability to toggle
//! selected state.  `on_selected` is the callback that is triggered with the selected state
//! returned.
//!
//! `ProgressWidget` draws a progress bar on the screen, with a `BoxWidget` as its base.  The
//! color of the progress bar and its background can be changed at any time.

/// Main module containing the run loop for the UI components, containers for windows and
/// `Widget` trait objects, and so on.  Contains the core elements required to build
/// a UI.
pub mod core;

/// Companion module used to define and trigger system-wide events.  Uses an event masking
/// style similar to the Atari ST GEM series: event masks can be used to tell the Pushrod
/// run loop which events the programmer desires to receive.
pub mod event;

/// Widget library used for on-screen UI interaction.  This is a core set of `Widget`
/// objects that are used to allow users to interact with an application.  Contains a core set
/// of widgets that can be extended.
///
/// Currently contains:
/// - Base Widget (for drawing a plain background)
/// - Box Widget (for drawing a plain background with a box and a colored border)
/// - Text Widget (for drawing text)
/// - Timer Widget (for performing timer operations)
/// - Image Widget (for drawing images)
/// - Push Button Widget (for creating an interactive button that can be clicked)
/// - Toggle Button Widget (for creating an on/off button that can be toggled)
/// - Radio Button Widget (for creating a group of buttons where only one item can be selected at a time)
/// - Progress Widget (for displaying progress of an operation)
pub mod widget;
