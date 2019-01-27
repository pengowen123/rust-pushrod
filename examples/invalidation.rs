use piston_window::*;

fn main() {
    let mut win: PistonWindow = WindowSettings::new("Invalidation Window", [640, 480])
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));
    win.set_ups(60);
    win.set_max_fps(60);
    win.set_swap_buffers(false);

    let mut invalidated = true;
    let mut need_swap_buffers = true;

    while let Some(e) = win.next() {
        // Yes, draw will be called over and over again, but it doesn't need to do anything unless
        // an object has been invalidated.  If an invalidation occurs, the screen then needs to
        // have its drawing buffers swapped to represent the change.

        win.draw_2d(&e, |_c, g| {
            if invalidated {
                clear([1.0; 4], g);
                need_swap_buffers = true;
            }

            invalidated = false;
        });

        if need_swap_buffers {
            win.window.swap_buffers();
            need_swap_buffers = false;
        }
    }
}
