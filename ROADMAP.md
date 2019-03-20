# rush-pushrod Roadmap

## 0.1.x Accomplished

- [x] Optimize main run loop
  - [x] Mouse move - if mouse point doesn't change, do not call mouse move dispatch
- [x] Improve Widget Library
  - [x] ~Improve mutability in Pushrod Window for triggering events~
  - [x] Remove "Pushrod" from everything (if possible) - we know it's pushrod, it doesn't need to be vain!
  - [x] Implement invalidate in draw cycle
  - [x] Use window.window.swap_buffers() only after drawing the screen with invalidated items
  - [x] Add parent relationship
  - [x] Add "get children of" parent array of IDs
  - [x] Walk children when parent signals an invalidate: parent -> children draw order
  - [x] Create a Widget Store that stores the widget and the parent/child relationship
  - [x] Extend Widget Store to keep track of (parent, child)
  - [x] Need to store the widget_id in the widget store
  - [x] Modify iterator to use filter after implementing widget_id
  - [x] Add autoclip flag to widget to automatically clip a widget's drawing area if requested
- [ ] Widget/Run Loop Library Changes
  - [ ] ~Add chainable functions that return self, so functions can be chained~
  - [x] Create 3D textures are assigned on a per-window basis.
  - [x] Remove code to swap buffers; use OpenGL 3D draw loop in favor.
  - [x] Add buffer retrieval so that objects can be drawn to the borrowed texture from the active window.
  - [x] Refactor the OpenGL 3D library code into its own separate struct/impl for extending/generics.
  - [x] Add white base widget to Pushrod Window constructor
  - [x] Change Widget configs to its own struct instead of being part of Widget
  - [x] Move Configurable struct and impl to their own external file
  - [x] Ensure that widgets of width and height of 0x0 are not included in the widget_at_point search
  - [x] Refactor code to use a single window and single set of widgets in a container
  - [x] Implement callbacks for Pushrod and Widgets
- [ ] Improve Signal Events
  - [x] Implement Closure callback cache for widgets
- [ ] Implement Standard Widget Library (Extends from Base Widget)
  - [x] Box Widget with Border Width and Color
  - [x] Clip
- [ ] Implement Graphics Translation Library
  - [x] Draw Box
  - [x] Timer
  - [x] Image

## 0.2.x -> 0.3.0

- [ ] Complicated Widget Library
  - [ ] Scrollbox (Horizontal and Vertical)
  - [ ] Slider (Horizontal and Vertical)
  - [ ] Scrollable Viewing Area
  - [ ] Toggle/Push Button
  - [ ] Progress Indicator
  - [ ] Popup Menu
  - [ ] Editable Text Box
- [ ] Callbacks
  - [ ] Implement for Mouse Click (Single click)
  - [ ] Implement Double Click
  - [ ] Implement Apple-like Mouse-up-inside
  - [ ] Implement Apple-like Mouse-up-outside
  - [ ] Create callbacks with a single option, contains event information instead of multiple callback types
- [ ] Resource Manager
  - [ ] Store widgets in a centralized resource manager so that they can be (de)serialized to store
  - [ ] Allow for manipulation of widgets by ID through resource manager
  - [ ] Create dialog boxes (windows) with builder
- [ ] Widget States
  - [ ] Enabled/Disabled (disabled means no callback interactions from event loop)
  - [ ] (In)visible (invisible means skip draw, remove from get_widget_id_for_point)
- [ ] Widget/Run Loop Library Changes
  - [ ] Implement visibility
  - [ ] Improve callbacks to use enum to define input parameters for each callback type
  - [ ] Implement enum for different `Widget` types.
- [ ] Main loop
  - [ ] Object focus
  - [ ] Window focus
  - [ ] Window loses focus
  - [ ] Window resize (needs to trigger a window-wide invalidate)

## TBD

- [ ] Convert Widget Library to OpenGL 3D Objects
  - [ ] All drawing and widget libraries remain the same
  - [ ] Translate graphics to draw to a canvas/texture
  - [ ] Editable text area (all one uniform font)
