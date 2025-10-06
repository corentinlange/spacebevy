pub mod perf;
pub mod styles;

use bevy::prelude::*;
use perf::PerfPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PerfPlugin);
    }
}
