use piston_window::*;
use fps_counter::FPSCounter;

mod libs;
mod sprite;
mod scene;
mod camera;
mod player;
mod object;
mod collider;
mod weapon;

use scene::Scene;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Demo platformer", (600, 600))
        .exit_on_esc(true)
        .build()
        .unwrap();

    // update per second
    window.set_ups(60);

    // frame per second counter
    let mut fps_counter = FPSCounter::new();

    // assets folder
    let assets = find_folder::Search::Kids(1).for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    let mut scene = Scene::new(assets, &mut window); 

    while let Some(event) = window.next() {

        scene.update(&event, &mut window);

        // print on screen fps status
        let fps = format!("{} fps", fps_counter.tick().to_string());

        window.draw_2d(&event, |context, graphics, device| {

            let transform = context.transform.trans(10.0, 25.0);
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 24)
                .draw(&fps, &mut glyphs, &context.draw_state, transform, graphics)
                .unwrap();

                glyphs.factory.encoder.flush(device);
        });
    }
}