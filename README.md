# rust-pushrod

UI Widget Library for Rust Piston library.  Draws inspiration from Atari GEM,
TrollTech Qt, and others.

## Philosophy

The reason I created this library instead of extending another library was that
I wanted to keep these specific design ideas in mind:

- Maintainable with little effort
- Easily extensible
- **Easy to use and understand**

These design ideas are critical.  **Keep it simple.  Keep it stupid simple.**

## Prerequisites for Pushrod

Pushrod requires the following minimum versions:

| Library | Version |
| ------- | ------- |
| piston_window | 0.87.0 |
| piston2d-opengl_graphics | 0.59.0 |
| gfx_core | 0.8.3 |
| gfx_device_gl | 0.15.5 |
| rust | 2018 |

## Timeline/Plans

[View the ROADMAP here](ROADMAP.md)

1. Event Library
   - Callbacks for:
       - Mouse movement
       - Button clicks (with modifiers)
       - Keyboard
2. Widget Library
   - Base widget (PObject)
       - Base components: size, border size, border color, fill color
       - Events for:
           - Mouse Movement
           - Mouse Enter/Exit
           - Button click (with modifiers)
       - Storage objects for parent/child relationship
           - Z-Order
   - Uniform font renderer use
   - Standard widgets
       - Text
       - Button
       - Widget Container
       - Scroll bar
       - Progress bar
       - Shape/Poly
   - Extended widget set
       - TextEdit
       - Rendered text
       - Tab view
       - Split Pane (Horizontal/Vertical)
       - Video
       - Audio
       - Sprite
       - Drag-and-drop buffer
3. Layout Managers (possibly separate project)
   - Relative Layout
   - Grid Layout
   - Auto Layout
   - Wrap Layout
3. Themable Library
   - **ONLY ONCE ALL WIDGETS ARE DESIGNED do we do themes**.
   - Themes will be handled by the base widget

## Runnable Tests

To run the all-inclusive window event test, use:

```
cargo run
(or)
cargo run --bin window_test
```

This will only test window-related events with mouse interaction: mouse enter, mouse exit, mouse click, mouse
pointer move, and mouse scroll.

## 1. Event Library

See [Event Library README](src/event/README.md)

## 2. Widget Library

See [Widget Library README](src/widget/README.md)

