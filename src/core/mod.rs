// Core module
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

/// Main module for Pushrod, contains the run loop code responsible for translating
/// OS-level events to well known structures, maintaining a list of active windows and
/// their widgets, and performing callbacks for events where appropriate.
pub mod main;

/// Contains geometric shape representations: `Point` and `Size`, representing a point on
/// the screen within a window, and the size of an object.
pub mod point;

/// Stores a `PistonWindow` and accompanying `PushrodWidget` trait objects.  Allows the
/// main run loop to interact with widgets by signalling when a UI event triggers a
/// callback that is appropriate for that widget.  Also contains helper functions
/// to determine which widget is at a specific point in the `PistonWindow`.
pub mod window;
