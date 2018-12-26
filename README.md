# rust-pushrod

GEM-like Widget Library for Rust Piston library.

Incorporates Atari GEM/VDI ideas with callbacks and Objects (Widgets), which
are ideas of other GUI libraries - Qt, Atari, Amiga, Tk, and others.  It draws
inspiration from other libraries, letting you concentrate on code rather than
obsessing over the interface.

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

## 1. Event Library

## 2. Widget Library

### Base Widget

### Uniform Font System

### Standard Widgets

### Extended Widgets

### Layout Managers

## 3. Themable Library


