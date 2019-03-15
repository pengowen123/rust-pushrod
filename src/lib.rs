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
//! inspiration from 16-bit GUI-based systems and other libraries over the years.
//!
//! Pushrod uses Piston as its main window drawing and event loop functionality.  It utilizes
//! piston2d-opengl graphics so that - eventually - the graphics can be represented as 3D
//! poly objects with textures as canvases.  It also utilizes the `gfx_core` library for
//! graphics functionality, as well as the `gfx_device_gl` support.
//!
//! Pushrod uses the following dependencies:
//! ```ignore
//! [dependencies]
//! piston_window = "^0.89.0"
//! find_folder = "^0.3.0"
//! ```

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
pub mod widget;
