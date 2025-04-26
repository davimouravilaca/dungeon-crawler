mod systems;

mod prelude {
    pub use crate::systems::*;
}

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .build()
}