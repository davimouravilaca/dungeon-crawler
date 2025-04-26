use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld, // Subworlds only have access to the components requested
    #[resource] map : &Map,
    #[resource] key : &Option<VirtualKeyCode>,
    #[resource] camera : &mut Camera
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Rigth => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y {
            let mut players = <&mut Point>::Query().filter(component::<Player>());
            player.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map .can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}