use crate::prelude::*;

#[system]
#[read_component(Render)]
#[read_component(Point)]
pub fn entity_render(ecs: &mut SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(DRAW_CONSOLE_ENTITY);
    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Render, &Point)>::query()
        .iter(ecs)
        .for_each(|(render, p)| {
            draw_batch.set(*p - offset, render.color, render.glyph);
        });
    draw_batch.submit(5000).expect("Batch error");
}
