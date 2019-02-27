# Pushrod Releases

## 0.1.13

- Removed texture creation and renewal code.

## 0.1.12

- Removed OpenGL specification in constructor for Pushrod runtime.
- Removing PushrodWindow, changing to a WidgetStore.
- Moved painting functions to appropriate areas, refactoring.
- Removed "Invalidated" messages.
- Optimized debugging messages in run loop.
- Implemented auto clipping when painting a widget.
- Added auto clipping.
- Changed rendering functionality to draw_2d, required a function in run loop to handle closures.
- Implemented text rendering.

## 0.1.11

- Changed code so that widgets of size 0x0 are skipped when searching for widget IDs by point.
- Removed resizable window in the demo for now.
- Added "time_ms()" function to timer_widget to pull time in milliseconds.
- Implemented tick function in timer.
- Implemented on_timeout as a closure (boxed)
- Added on_timeout example in simple example.

## 0.1.10

- Optimized window code to use iterator with filter and map.
- Optimized widget-for-point lookup code.
- Super-optimized the draw loop, now walks children and draws only when an invalidation is sensed before-hand.
- Optimized mouse cursor events in the main run loop.
- Starting work on the timer widget.
- Added "handle_resize" function to the PushrodWindow to recreate the texture buffer.
- Added resize handling in the main loop (doesn't work on Mac)
- Added OpenGL texture support for drawing screen on an OpenGL texture, then drawing its image on the screen in render loop.

## 0.1.9

- Further optimized config access for widget and base_widget.
- Updated configuration testing code.
- Moved configs to their own file, separate from the widget.
- Added `get_children_of` function to return all the child IDs for the requested parent ID.
- Updated main loop to show children of widget IDs.
- Added widget_id to the Widget store.

## 0.1.8

- Added simplified set_origin that now takes x, y coordinates rather than a `Point` object.
- Added simplified set_size that now takes w, h coordinates rather than a `Size` object.
- Changed code to only store the parent ID.
- Added `set_border` helper function to set both the border color and thickness at once.
- Added `Configurable` object so that widgets can now separate config logic for set/get.
- Modified all code to use `Configurable` object, so that the configuration logic is separated.

## 0.1.7

- Changed Widget to use ConfigKey type instead of u8, as this might change in a later release.
- Added parent/child relationship to window store.
- Added function to retrieve parent based on widget ID.
- Created widget store to store the widget (boxed) and parent/child relationship.
- Adding a widget now returns its widget ID.
- Fixed breaking test after changing to ConfigKey type.
- Updated examples to include widget ID and parent add.

## 0.1.6

- Modified so that you no longer have to add a base widget to the window
- Removed README markdown files, as they are not necessary
- Renamed event masking to start with "MASK" instead of "PUSHROD"
- Renamed PushrodBoxWidget to BoxWidget
- Renamed PushrodWidget to Widget
- Renamed PushrodEventMask to EventMask
- Renamed PushrodEventListener to EventListener
- Renamed PushrodWidgetConfig to WidgetConfig
- More corrections to documentation, removed Pushrod from class names where appropriate

## 0.1.5

- Modified Base Widget to clear its invalidation state after a draw.
- Set window `swap_buffers` flag to false when added.
- Run loop optimization:
  - Checks to see if widgets need to be drawn/refreshed (invalidated)
  - Upon invalidation, draw is called, and display buffer is swapped
- Added ability to draw a box using `line()` calls
- Created `PushrodBoxWidget` object to draw a box with a backing widget
  - Created as an extension of PushrodWidget
    - Added `get/set_border_color` to set get and set border color
    - Added `get/set_border_thickness` to set border width
    - Uses the PushrodBaseWidget as its top-level widget for drawing
    - set origin, color, and point all control the top level and base widget
- Added Timer as list of widgets to build before 0.2.0 release.

## 0.1.4

- Run loop optimization:
  - Optimized mouse movement - repetitive points are redundant.
  - Added invalidation to set origin, size, and color.
  - Added clear_invalidation flag to draw.
- Added tests for Points.
- Added example to show invalidation behavior in Piston.
- Updated callbacks to use widget_id when calling mouse enter, exit, scroll.
- Removed context reset from trait object default draw method.
- Renamed simple example to "simple"
- Adjusted Cargo.toml to include keywords and README.
- Got Travis.CI building the application.
- Shortened README.

## 0.1.3

- Got some assistance from @andygrove for documentation and testing.
- Completed documentation in the application
- Moved example away from the `src` directory, and into its own `examples` directory.

## 0.1.2

- Adjustments for Crates.

## 0.1.1

- Updated Cargo Crates to use latest piston library and associated graphics libraries.
- Fixed bug found with configs: now returns default values in default trait if not found.
- Added convenience method to create a width and height of 0x0

## 0.1.0

- First release
