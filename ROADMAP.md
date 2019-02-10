# rush-pushrod Roadmap

## 0.1.x -> 0.2.0

- [ ] Widget Library Changes
  - [ ] ~Add chainable functions that return self, so functions can be chained~
  - [ ] Remove OpenGL use when creating a new window
  - [ ] Use glfw for window drawing instead of PistonWindow, as it seems to be more OS independent.
  - [ ] Change Widget object to be generic so that settings and calls are made against the widget, not the trait
  - [ ] Change Widget to be a Struct<> to include widget generics for drawing and re-entrant functions
  - [ ] Impl Widget<> should take generic widget and assign it internally so interactions are done against it as `widget.(x)`
  - [ ] Ensure that the widget library code is still super simple and easy to understand
- [ ] Improve Signal Events
  - [ ] Implement signal masks so widgets can decide which signals to receive
  - [ ] Implement for Mouse Click (Single click)
  - [ ] Implement Double Click
  - [ ] Implement possible callback registry for events, so code can be called after an event is triggered
- [ ] Implement Graphics Translation Library
  - [ ] Implement a graphics library that is uniform and not independent of any Rust library
  - [ ] Clip
  - [ ] Draw Rounded Rectangle
  - [ ] Draw Ellipse
  - [ ] Draw Arc
  - [ ] Draw Circle
  - [ ] Draw Poly
  - [ ] Draw Image
  - [ ] Fill Area (Rectangle)
  - [ ] Translate Points based on Origin/Size
- [ ] Implement Standard Widget Library (Extends from Base Widget)
  - [ ] Timer
  - [ ] Image
  - [ ] Text box (use Google Font Library, as it's the most uniform/generic)
  - [ ] Button
- [ ] More examples

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
- [ ] Widget Library Changes
  - [x] Add white base widget to Pushrod Window constructor
  - [x] Change Widget configs to its own struct instead of being part of Widget
  - [x] Move Configurable struct and impl to their own external file
- [ ] Implement Standard Widget Library (Extends from Base Widget)
  - [x] Box Widget with Border Width and Color
- [ ] Implement Graphics Translation Library
  - [x] Draw Box

## 0.2.x -> 0.3.0

- [ ] Complicated Widget Library
  - [ ] Scrollbox (Horizontal and Vertical)
  - [ ] Slider (Horizontal and Vertical)
  - [ ] Scrollable Viewing Area
  - [ ] Toggle/Push Button
  - [ ] Progress Indicator
  - [ ] Popup Menu
  - [ ] Editable Text Box
- [ ] Widget States
  - [ ] Enabled/Disabled (disabled means no callback interactions from event loop)
  - [ ] (In)visible (invisible means skip draw, remove from get_widget_id_for_point)
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
