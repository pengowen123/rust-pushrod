# Pushrod Releases

## 0.3.0

- Reenabled use of OpenGL (using Molten's OpenGLES libraries - which are free.)
- Enabled zoom to handle zoom factor for HiDPI displays when displaying GL texture.
- Converted main graphics drawing routines to use GL drawing instead (#113)
- Converted text functions to use GlyphCache in GL rather than Piston.
- Converted images to use Texture instead of G2dTexture objects for GL.
- Uses CharacterCache trait in Piston to determine rendering width. (#129)

## 0.2.12

- Added use of Rectangle instead of drawing lines multiple times for box.  (#124)
- Made contribution to piston2d-graphics crate to provide size_hint when text is to be rendered.
- Now handles disable of objects properly (#127)
- Removed events code, as this is not in use.
- Code clean-up, documentation clean-up.

## 0.2.11

- Added image button (#80)
- Changed Rust Image into a Button
- Added ability to set button selected state (#89)
- Modified example application so that it starts the animation on the progress by default
- Removed debugging from Radio Button.
- Updated comments in Radio Button.

## 0.2.10

- Fixed bug with debugging: when selecting checkbox, now toggles hide/show rather than clearing out text. (#116)
- Added a third radio button to control speed of progress widget in example.
- Improved push button sensitivity, highlights when mouse is in bounds, deselects when out. (#119)
- Improved toggle button behavior, similar to push button. (#120)
- Just bought a huge improvement with set_lazy in the window event loop; need to modify timers.
- Modifying code so that GlfwWindow is now a requirement, as it has implemented window polling with timeout properly.
- Lessened load on the draw routines; switched max FPS to 30.
- Fixed drawing functionality on all platforms; issue with doubling clip, which is no longer necessary.

## 0.2.9

- Added radio button images.
- Adding a widget now assigns the widget ID to the configs (#117)
- Added support for injected events (#115)
- Added RadioButtonWidget (#45)
- Added example in the app for fast/slow timer in the progress widget.

## 0.2.8

- Added checkbox widget. (#46)
- Added checkbox to the main application, enable/disable debugging.
- Added additional draw function that takes origin point (#106)

## 0.2.7

- Adjusted mutability of getters in Config.
- Added hide/show button operations to the sample app.
- Removed event handling for widgets that are not visible (#103)
- Enable/Disable state on Widget implemented (#39)

## 0.2.6

- Includes widget_id when inject_event is called. (#102)
- Fixed timer widget to include widget_id when triggered.
- Added visibility of widget functionality via CONFIG_WIDGET_HIDDEN (#40)
- Now shows the progress (in percentage) of the progress widget.

## 0.2.5

- Added ability to modify Widget text (using set_text and get_text)
- Modified example to show debug: widget ID, name, and dimensions (#93)
- Re-added timer widget, added inject widget function
- Added ability to inject an event. (#97)
- Updated TimerWidget so that it injects a TimerEvent when triggered properly. (#96)
- Reimplemented ToggleButton code, generates WidgetSelected event on click. (#95)
- Added animate button, now enables/disables the timer callback on click.
- Updated configs to use get/set with enumerations. (#100)
- Moved config settings to Widget.
- Made the progress animate button work properly.

## 0.2.4

- Changed mutability of get_selected state in ToggleButtonWidget.
- Added PushrodCallbackEvents class to handle callback events.
- Modified Widget so that it now has its own handle_event function, returns an event to inject if desired.
- Fixed PushButtonWidget to use handle_event properly, injecting an event where appropriate.
- Added callback for random color button.
- Fixed callbacks so that they work appropriately via an event system. (#88)
- Changed mutability of widget_store, get_widget_for_id now returns a RefCell so widgets can be accessed directly.
- Added ability to look up a widget by name. (#94)
- Added ability to add a widget, and a widget with parent with an assigned name.
- Internally used "_WidgetStoreBase" as the internal first, or base widget.  Updated docs to reflect this.
- Added randomize button that randomizes color of progress widget.

## 0.2.3

- Modified simple test, increased spacing of Hide buttons.
- Implemented ProgressWidget (#32)
- Added `SecondaryColor` config to allow for progress widget color to be adjusted.
- Added `ToggleButtonWidget` (#31)

## 0.2.2

- Added PushButtonWidget. (#24)
- Fixed BoxWidget so that it returns the defaults in border_color and border_thickness as documented
- Added PushButtonWidget to the example
- Modified PushButtonWidget so that only the left mouse button triggers a click
- Made `add_widget` function reference widget_store internally, making it easier to add widgets.
- Modified example code so that it's in its own struct/impl (#78)
- Modified `TextWidget` so that text is vertically centered.
- Added buttons to hide related widgets

## 0.2.1

- Added TextHelper to TextWidget to help determine rendering size when drawing text.
- Added TextJustify enum to determine justification of text in regards to size of bounds.
- TextWidget now adjusts transformation based on font size, not re-adjusting origin.
- TextWidget now honors justification: left, right, and center are now shown in the demo. (#76)

## 0.1.21

- Code freeze
- Documentation updates and fixes
- Removal of dead code
- Added Debug to Point and Size

## 0.1.20

- Created button map to keep track of buttons down in run loop for button clicks.
- Added mouse click up inside widget. (#36)
- Added mouse click up outside widget. (#37)
- Changed on_mouse_up to on_button_up, which follows the naming convention everywhere else.

## 0.1.19

- Documentation cleanup
- Added CONTRIBUTING and issue templates
- Added handling of window resizing (#42)
- Re-enabled timer widget
- Fixed mouse movement in a widget based on its screen position (#34)
- Added window (un)focus event handling (#41)
- Added mouse button down inside a widget (#35)

## 0.1.18

- Fixing test code, problem is with image rendering; need to figure that out.
- Removed use of clear() call, it caused issues when drawing widgets on the screen. (#69)
- Modified image_widget so that it now stores the texture in heap.

## 0.1.17

- Warnings and unused code cleaned up.
- Added a much more uniform Config design based on u/RayDepp on Reddit (#25)
- Autoclip with scissors (#19)
- Fixed text widget so that it displays properly with clipping (#23)

## 0.1.16

- Keyboard events ticket (#43) completed by dannyfritz, merged to master.
- Modified code so that origin of drawing is 0x0 relative to the widget (#22)
- Changed Cargo.toml to only use major version of Piston, minor releases are automatically used.
- Updated README.
- Moved the context origin to 0x0 outside of the draw loop (#67)
- Renamed BaseWidget to CanvasWidget (#27)
- Removed autoclip config, as this will be automatic. (#20)
- Context reset automatically takes place before each draw (#21)

## 0.1.15

- Made a new `BlankCallback` type for timer
- Changed timer code to use the `BlankCallback` in the CallbackStore.
- Optimized Cargo.toml file to only include one dependency: piston.
- Modified run loop to include window factory object retrieval.
- Added Image widget, added official 512x512 logo.
- Added Image auto-scaling when drawing.
- Fixed text widget so it uses the text color when drawing text.
- Currently, text is causing issues with image display.

## 0.1.14

- Interim release to fix bugs in build and tests

## 0.1.13

- Removed texture creation and renewal code.
- Made `PushrodEvent` type cloneable.
- Changed mouse button callbacks to contain default functions.
- Added mouse moved, button down, and button up callbacks; changed all mouse calls to default empty function bodies.
- Optimized timer, box, and text widgets, removed functions that previously needed to be overridden.
- Added SingleCallback and PointCallback for closures.
- Added callbacks(&mut self) call to all widgets to allow for callback/closure storage.
- Added callbacks for mouse enter, exit, scroll, and move.
- More optimizations done to Widget class, only implementing config and callbacks.
- Added setters for callbacks in Widget.
- Implemented test case use of callbacks in example application.
- Code clean-up, removed unused code, commented event code temporarily.

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
