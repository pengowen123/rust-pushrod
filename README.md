# rust-pushrod

GEM-like Widget Library for Rust Piston library.

Incorporates Atari GEM/VDI ideas with callbacks and Objects (Widgets), which
are ideas of other GUI libraries - Qt, Atari, Amiga, Tk, and others.  It draws
inspiration from other libraries, letting you concentrate on code rather than
obsessing over the interface.

## Reason for Pushrod

The reason I created this library instead of extending another library was that
I wanted to keep these specific design ideas in mind:

- Maintainable with little effort
- Easily extensible
- Easy to use and understand

These design ideas are critical.  Keep it simple.  Keep it stupid simple.

## Timeline/Plans

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
       - Widget Contaioner
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
   - Layout Managers
       - Relative Layout
       - Grid Layout
       - Auto Layout
       - Wrap Layout
3. Themable Library
   - ONLY ONCE ALL WIDGETS ARE DESIGNED do we do themes.
   - Themes will be handled by the base widget

## Runnable Tests

To run the all-inclusive window event test, use:

```
cargo run --bin window_test
```

This will only test window-related events with mouse interaction: mouse enter, mouse exit, mouse click, mouse
pointer move, and mouse scroll.

## 1. Event Library

## 2. Widget Library

### Base Widget

### Uniform Font System

### Standard Widgets

### Extended Widgets

### Layout Managers

## 3. Themable Library


