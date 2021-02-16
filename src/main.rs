extern crate sdl2;

pub mod poly_ui {
    pub mod app;

    pub mod components;
    pub mod controls;
    pub mod events;
    pub mod layouts;
    pub mod widgets;

    pub mod sdl2;
}

use nalgebra::Point2;
use nalgebra::Vector2;
use crate::poly_ui::app::AppTrait;
use crate::poly_ui::components::Transform;
use crate::poly_ui::controls::PushButton;

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
    let window = app
        .get_windows_manager()
        .create_window("Test window", 800, 600);
    let button = PushButton::new();
    let transform = Transform::new(&Point2::<i32>::new(10, 10), &Vector2::<u32>::new(100, 100));
    window.borrow_mut().widget().borrow_mut().add_child_with_transform(button.make_ownerless(), &transform);
    app.exec()
}
