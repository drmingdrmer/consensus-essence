use rand::prelude::SliceRandom;

use config::Config;
use point::Point;
use scene::Scene;

use crate::contour::Contour;

pub mod config;
pub mod contour;
pub mod display_slice;
pub mod draw;
pub mod point;
pub mod scene;

pub struct Evolution {
    config: Config,

    scene: Scene,

    /// A normalized by each point of the scene
    normalized: Vec<Scene>,

    /// Best found contour and its conflicts
    contours: Vec<Contour>,
}

impl Evolution {
    pub fn new(config: Config, scene: Scene, build_contour: impl Fn(&Point) -> Contour) -> Self {
        let normalized = scene
            .points
            .iter()
            .map(|point| scene.normalize(*point))
            .collect::<Vec<_>>();

        let contours = scene.points.iter().map(build_contour).collect::<Vec<_>>();

        Self {
            config,
            scene,
            normalized,
            contours,
        }
    }

    pub fn points_len(&self) -> usize {
        self.scene.points.len()
    }

    /// Get a scene that is normalized by the given point.
    pub fn get_normalized_scene(&self, i: usize) -> &Scene {
        &self.normalized[i]
    }

    /// Given two points in the scene, return `a` is below `b`,
    /// by checking if `b` is above the contour,
    /// in the reference frame of `a`.
    pub fn is_below(&self, a: usize, b: usize, contour: &Contour) -> bool {
        self.product(a, b, contour) > 0f64
    }

    pub fn is_above(&self, a: usize, b: usize, contour: &Contour) -> bool {
        self.product(a, b, contour) < 0f64
    }

    pub fn product(&self, unit: usize, p: usize, contour: &Contour) -> f64 {
        let b_in_normalized_by_a = self.normalized[unit].points[p];
        contour.cross_product_x(&b_in_normalized_by_a)
    }

    /// Count the number of conflicting point pairs.
    ///
    /// Return the total number of conflicts found for point `p`, using contour `contour`.
    pub fn count_conflict(&self, p: usize, contour: &Contour) -> usize {
        contour.validate();

        let mut conflicts = 0;
        for i in 0..self.points_len() {
            if i == p {
                continue;
            }

            if self.is_below(p, i, contour) && self.is_below(i, p, &self.contours[i])
                || self.is_above(p, i, contour) && self.is_above(i, p, &self.contours[i])
            {
                conflicts += 1;
            }
        }
        conflicts
    }

    /// Update the contour for the given point, find a better contour that has less conflicts with other points.
    ///
    /// Return the best contour found and the number of conflicts.
    pub fn find_better_contour(&self, p: usize) -> (Contour, usize) {
        let w = self.config.variant_weight;
        let contour = self.contours[p].clone();
        let conflict = self.count_conflict(p, &contour);

        let mut best = (contour.clone(), conflict);

        for _i in 0..self.config.n_spawn {
            let (new, _action) = contour.rand_update(w.0, w.1, w.2);
            let conflict = self.count_conflict(p, &new);

            // Find the first better solution
            if conflict < best.1 {
                best = (new, conflict);
                break;
            }
        }

        best
    }

    pub fn evolve_one_round(&mut self) {
        let mut next_generation = vec![];

        for p in 0..self.contours.len() {
            let (new_contour, _new_conflict) = self.find_better_contour(p);
            next_generation.push(new_contour);
        }

        next_generation.shuffle(&mut rand::thread_rng());

        self.contours = next_generation;
    }

    pub fn evolve(&mut self) {
        // make dir `output`:
        std::fs::create_dir_all("output/").unwrap();

        let pref = "output/cc";

        draw::draw_contour(format!("{pref}-0000.png"), &self.scene, &self.contours).unwrap();

        for i in 1..=self.config.n_round {
            println!("round {}", i);
            self.evolve_one_round();

            draw::draw_contour(format!("{pref}-{i:0>4}.png"), &self.scene, &self.contours).unwrap();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fn contour(points: impl IntoIterator<Item = (f64, f64)>) -> Contour {
        Contour::new(points.into_iter().map(Point::from))
    }

    let config = Config {
        n_points_per_scene: 1_000,
        n_round: 100,
        n_spawn: 10,
        variant_weight: (5.0, 5.0, 5.0),
    };
    let scene = Scene::rand_scene(10f64, 10f64, config.n_points_per_scene);

    let mut evolution = Evolution::new(config, scene, |_point| {
        let x0 = rand::random::<f64>();
        let y0 = 1.0 + rand::random::<f64>();

        let x2 = 1.0 + rand::random::<f64>();
        let y2 = rand::random::<f64>();
        contour([(x0, y0), (1.0, 1.0), (x2, y2)])
    });
    evolution.evolve();

    Ok(())
}
