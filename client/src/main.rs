mod app;
mod camera;

mod player;
mod scene;
mod ui;

#[cfg(target_arch = "wasm32")]
fn main() {
    app::web::run();
}

#[cfg(not(target_arch = "wasm32"))]
mod net;
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    app::desktop::run();
}
