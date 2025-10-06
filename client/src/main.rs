mod app;
mod camera;
mod scene;
mod ui;

#[cfg(target_arch = "wasm32")]
fn main() {
    app::web::run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    app::desktop::run();
}
