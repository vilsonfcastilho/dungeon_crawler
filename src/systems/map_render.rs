use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov: &FieldOfView = fov.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt: Point = Point::new(x, y);
            let offset: Point = Point::new(camera.left_x, camera.top_y);

            let idx: usize = map_idx(x, y);
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx]
            {
                let tint: (u8, u8, u8) = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GRAY
                };

                let glyph: u16 = theme.tile_to_render(map.tiles[idx]);
                draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
