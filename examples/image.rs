extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("piston: image", [300, 300])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let rust_logo = assets.join("rust-512x512.jpg");
    let rust_logo: G2dTexture = Texture::from_path(
        &mut window.factory,
        &rust_logo,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.25; 4], g);

            let transform = c.transform.trans(25.0, 25.0).scale(0.50, 0.50);
            let (clip_x, clip_y, clip_w, clip_h) = (50, 50, 200, 200);
            let clipped = c.draw_state.scissor([clip_x, clip_y, clip_w, clip_h]);

            image(&rust_logo, transform, g);
        });
    }
}