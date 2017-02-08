use SIZE_PER_TILE;
use BORDER_BETWEEN_TILES;

pub fn coord_to_pixel(world: u64, cam: u64) -> f64 {
    ((world - cam) * (SIZE_PER_TILE * BORDER_BETWEEN_TILES)) as f64
}