use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(Health)]
pub fn player_input(
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Space => Point::new(0, 0),
            _ => return,
        };
        let mut players = <(&Point, Entity)>::query().filter(component::<Player>());
        let (destination, player_entity) = players
            .iter(ecs)
            .find_map(|(pos, entity)| Some((*pos + delta, *entity)))
            .unwrap();
        if delta != Point::zero() {
            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });
            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        destination,
                        entity: player_entity,
                    },
                ));
            }
        } else {
            if let Ok(mut health) = ecs.entry_mut(player_entity).unwrap().get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + 1);
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
