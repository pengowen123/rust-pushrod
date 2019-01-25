# Pushrod Widget Callbacks

Callbacks are used to indicate when an action takes place that affects an onscreen UI component.  These can
be actions when a mouse enters or exits a region, when a user clicks on a widget, or types inside it, etc.

Please keep in mind, this could all change.  The point of a callback mechanism of this nature is to make it easy
for a block of code to execute when an event is triggered.  This may be done with an `on_x_action` registration,
or something similar.  (Now that I write this, that actually sounds like a pretty good way to handle it.)

## Mouse Enter Event

When a mouse enters the region of a widget (its own x, y, w, h area), this event is triggered.  The method to
override is ```fn mouse_entered(&mut self)```

## Mouse Exit Event

When a mouse exits the region of a widget, this event is triggered.  The method to override is
```fn mouse_exited(&mut self)```

## Mouse Scroll Event

When a mouse scroll button is moved (horizontally or vertically), this is represented with a `Point` structure.
The method to override is ```fn mouse_scrolled(&mut self, point: Point)```

