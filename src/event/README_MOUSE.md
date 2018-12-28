# Events

## Mouse Events

### MouseEvent

`MouseEvent` is a container struct that contains an origin point of a mouse event.
A origin point only contains the X and Y coordinates.

### MouseMoveEvent

This event is triggered when a mouse moves.  Only contains an origin point.  If any button modifiers
are handled, those are triggered by click/up/down events, and must be tracked by the application.

