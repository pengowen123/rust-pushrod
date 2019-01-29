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

//! Pushrod is a Cross Platform UI Widget Library for Piston.
//!
//! It is intended to be lightweight, easy to use, and easy to understand.  Pushrod draws
//! inspiration from Atari ST GEM development, TrollTech Qt, and other libraries over
//! the years.
//!
//! Pushrod uses Piston as its main window drawing and event loop functionality.  It utilizes
//! piston2d-opengl graphics so that - eventually - the graphics can be represented as 3D
//! poly objects with textures as canvases.  It also utilizes the `gfx_core` library for
//! graphics functionality, as well as the `gfx_device_gl` support.
//!
//! Pushrod uses the following dependencies:
//! ```ignore
//! [dependencies]
//! piston_window = "^0.87.0"
//! piston2d-opengl_graphics = "^0.59.0"
//! gfx_core = "^0.8.3"
//! gfx_device_gl = "^0.15.5"
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
pub mod widget;
