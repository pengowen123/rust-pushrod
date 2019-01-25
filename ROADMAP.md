# rush-pushrod Roadmap

# 0.1.0

- [x] Event Skeleton
  - [x] Mouse Movement
  - [x] Mouse Button Press
  - [x] Mouse Button Release
  - [x] Mouse Button Scroll
- [x] Widget Library Skeleton
  - [x] Base Widget Framework
  - [x] Base Widget Point of Origin
  - [x] Base Widget Size
  - [x] Base Widget Fill Color (with opacity)
  - [x] Base Widget Draw Function
  - [x] Base Widget Invalidate (to indicate a refresh/redraw)
  - [x] Update test case to show simple widgets
  - [x] Update draw function to walk all widgets and draw them
  - [ ] Document Widget Library and how to extend
  - [x] Assign Widget ID when added to Window Container
- [x] Base Widget Callback Framework
  - [x] Callback for Mouse Enter
  - [x] Callback for Mouse Exit
  - [x] Callback for Mouse Scroll
  - [x] Implement Widget Lookup based on mouse position (done by Widget ID)
    - [x] Get Widget at Point (returns Widget ID)
    - [x] Get Widget by ID (returns Widget reference)
  - [x] Callback in Base Widgets for each event type (done by Widget ID)
- [ ] Documentation

# 0.2.0

- [ ] Optimize main run loop
  - [ ] Mouse move - if mouse point doesn't change, do not call mouse move dispatch
- [ ] Improve Widget Library
  - [ ] Improve mutability in Pushrod Window for triggering events
  - [ ] Add parent/child relationship
  - [ ] Walk children when parent signals an invalidate
  - [ ] Invalidated child should draw subchildren as well, but not parent
  - [ ] Extend Widget Store to keep track of (parent, child)
- [ ] Improve Signal Events
  - [ ] Implement for Mouse Click (Single click)
  - [ ] Implement Double Click
- [ ] Implement Graphics Library
  - [ ] Implement a graphics library that is uniform and not independent of any Rust library
  - [ ] Clip
  - [ ] Draw Box
  - [ ] Draw Rounded Rectangle
  - [ ] Draw Ellipse
  - [ ] Draw Arc
  - [ ] Draw Circle
  - [ ] Draw Poly
  - [ ] Fill Area
  - [ ] Translate Points based on Origin/Size
- [ ] Implement Standard Widget Library (Extends from Base Widget)
  - [ ] Box Widget with Border Width and Color
  - [ ] Text box (use Google Font Library, as it's the most uniform/generic)
  - [ ] Button
  - [ ] Progress Indicator

# 0.3.0

- [ ] Complicated Widget Library
  - [ ] Scrollbox (Horizontal and Vertical)
  - [ ] Editable Text Box
  - [ ] Scrollable Viewing Area
  - [ ] Toggle Button
- [ ] Main loop
  - [ ] Object focus
  - [ ] Window focus
  - [ ] Window loses focus
  - [ ] Window resize

# TBD

- [ ] Convert Widget Library to OpenGL 3D Objects
  - [ ] All drawing and widget libraries remain the same
  - [ ] Translate graphics to draw to a canvas/texture
