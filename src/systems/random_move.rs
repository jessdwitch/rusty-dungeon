use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
) {
    let mut rng = RandomNumberGenerator::new();
    <&mut Point>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(ecs)
        .for_each(|p| {
            let proposed = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *p;
            if map.can_enter_tile(proposed) {
                *p = proposed;
            }
        })
}
