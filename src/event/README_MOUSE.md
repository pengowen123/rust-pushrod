# Mouse Events

## PushrodMouseEvent

This event is triggered when a mouse moves.  Only contains an origin point.  If any button modifiers
are handled, those are triggered by click/up/down events, and must be tracked by the application.
This event matches the `PUSHROD_EVENT_MOUSE_MOVED` event mask.

## PushrodMouseDownEvent

Triggered when a mouse button is pushed down.  The mouse position is not sent as part of this event,
only the button state, which is the `piston_window` `ButtonState` enum.  This event matches the
`PUSHROD_EVENT_MOUSE_DOWN` event mask.

## PushrodMouseUpEvent

Triggered when a mouse button is released.  Contains the `piston_window::ButtonState`.  This event
matches the `PUSHROD_MOUSE_EVENT_UP` event mask.

## PushrodMouseScrollEvent

Triggered when a mouse scroll event takes place - whether horizontal or vertical.  Scrolling down or
to the right will cause X and Y values to be positive.  Anything scrolling up or to the left will
cause X and Y values to be negative.  This event matches the `PUSHROD_MOUSE_EVENT_SCROLL` event mask.
