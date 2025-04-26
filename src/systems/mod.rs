mod systems;
mod map_render;
mod player_input;

mod prelude {
    pub use crate::systems::*;
}

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_builder::map_render_system())
        .build()
}