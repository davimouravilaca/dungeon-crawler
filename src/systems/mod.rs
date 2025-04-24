mod systems;

mod prelude {
    pub use crate::systems::*;
}

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .build()
}