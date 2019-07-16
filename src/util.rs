type Point2 = nalgebra::Point2<f32>;

/// Translates the world coordinate system, which
/// has Y pointing up and the origin at the center,
/// to the screen coordinate system, which has Y
/// pointing downward and the origin at the top-left,
pub fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: Point2) -> Point2 {
    let x = point.x + screen_width / 2.0;
    let y = screen_height - (point.y + screen_height / 2.0);
    Point2::new(x, y)
}

/// Translates the world coordinate system to
/// coordinates suitable for the audio system.
pub fn world_to_audio_coords(screen_width: f32, screen_height: f32, point: Point2) -> [f32; 3] {
    let x = point.x * 2.0 / screen_width;
    let y = point.y * 2.0 / screen_height;
    let z = 0.0;
    [x, y, z]
}
