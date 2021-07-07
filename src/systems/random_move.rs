use crate::prelude::*;

#[system(for_each)]
pub fn random_move(
    entity: &Entity,
    pt: &Point,
    _rnd: &MovingRandomly,
    commands: &mut CommandBuffer,
) {
    let mut rng = RandomNumberGenerator::new();
    let destination = match rng.range(0, 4) {
        0 => Point::new(-1, 0),
        1 => Point::new(1, 0),
        2 => Point::new(0, -1),
        _ => Point::new(0, 1),
    } + *pt;
    commands.push(((), WantsToMove{destination, entity: *entity}));
}
