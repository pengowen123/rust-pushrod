# Pushrod Widget Callbacks

Callbacks are used to indicate when an action takes place that affects an onscreen UI component.  These can
be actions when a mouse enters or exits a region, when a user clicks on a widget, or types inside it, etc.

## Mouse Enter Event

When a mouse enters the region of a widget (its own x, y, w, h area), this event is triggered.  The method to
override is ```fn mouse_entered(&mut self)```

## Mouse Exit Event

When a mouse exits the region of a widget, this event is triggered.  The method to override is
```fn mouse_exited(&mut self)```

## Mouse Scroll Event

When a mouse scroll button is moved (horizontally or vertically), this is represented with a `Point` structure.
The method to override is ```fn mouse_scrolled(&mut self, point: Point)```

