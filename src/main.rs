extern crate sdl2;

pub mod poly_ui {
    pub mod app;

    pub mod components;
    pub mod widgets;

    pub mod sdl2;
}
use crate::poly_ui::app::AppTrait;

fn main() -> Result<(), String> {
    // let args: Vec<_> = env::args().collect();

    // println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());

    // if args.len() < 2 {
    //     println!("Usage: ./demo font.[ttf|ttc|fon]")
    // } else {
    //     let path: &Path = Path::new(&args[1]);
    //     run(path)?;
    // }

    let mut app = poly_ui::sdl2::App::default();
    let _window = app.get_windows_manager().create_window("Test window", 800, 600);
    app.exec()
}
