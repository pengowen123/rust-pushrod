# rush-pushrod Roadmap

# 0.1.x -> 0.2.0

- [ ] Optimize main run loop
  - [x] Mouse move - if mouse point doesn't change, do not call mouse move dispatch
- [ ] Improve Widget Library
  - [ ] Implement invalidate in draw cycle
  - [ ] Improve mutability in Pushrod Window for triggering events
  - [ ] Add parent/child relationship
  - [ ] Walk children when parent signals an invalidate
  - [ ] Invalidated child should draw subchildren as well, but not parent
  - [ ] Extend Widget Store to keep track of (parent, child)
- [ ] Improve Signal Events
  - [ ] Implement signal masks so widgets can decide which signals to receive
  - [ ] Implement for Mouse Click (Single click)
  - [ ] Implement Double Click
  - [ ] Implement possible callback registry for events, so code can be called after an event is triggered
- [ ] Implement Graphics Translation Library
  - [ ] Implement a graphics library that is uniform and not independent of any Rust library
  - [ ] Clip
  - [ ] Draw Box
  - [ ] Draw Rounded Rectangle
  - [ ] Draw Ellipse
  - [ ] Draw Arc
  - [ ] Draw Circle
  - [ ] Draw Poly
  - [ ] Draw Image
  - [ ] Fill Area (Rectangle)
  - [ ] Translate Points based on Origin/Size
- [ ] Implement Standard Widget Library (Extends from Base Widget)
  - [ ] Box Widget with Border Width and Color
  - [ ] Text box (use Google Font Library, as it's the most uniform/generic)
  - [ ] Button

# 0.2.x -> 0.3.0

- [ ] Complicated Widget Library
  - [ ] Scrollbox (Horizontal and Vertical)
  - [ ] Editable Text Box
  - [ ] Scrollable Viewing Area
  - [ ] Toggle Button
  - [ ] Timer
  - [ ] Image
  - [ ] Progress Indicator
- [ ] Main loop
  - [ ] Object focus
  - [ ] Window focus
  - [ ] Window loses focus
  - [ ] Window resize (will trigger a window-wide invalidate)

# TBD

- [ ] Convert Widget Library to OpenGL 3D Objects
  - [ ] All drawing and widget libraries remain the same
  - [ ] Translate graphics to draw to a canvas/texture
