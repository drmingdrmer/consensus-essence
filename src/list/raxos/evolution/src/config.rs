pub struct Config {
    pub n_points_per_scene: usize,

    /// The number of round to evolve.
    pub n_round: usize,

    /// Number of variants to spawn for each contour
    pub n_spawn: usize,

    /// The probability of moving a point, adding a point, or removing a point
    pub variant_weight: (f64, f64, f64),
}
