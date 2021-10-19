use crate::sdf;
use crate::sdf::{find_closest_point, sdf_curve};
use crate::threed::Vec3;
use crate::utils::lerpf;

pub struct Flower {
    control_points: Vec<Vec3>,
}

impl Flower {
    pub fn new() -> Self {
        Self {
            control_points: vec![
                Vec3::zero(),
                Vec3::zero(),
                Vec3::zero(),
                Vec3::zero(),
            ],
        }
    }

    pub fn update_controls(&mut self, points: &Vec<Vec3>) {
        assert_eq!(self.control_points.len(), points.len());
        for i in 0..points.len() {
            self.control_points[i] = points[i].clone();
        }
    }

    pub fn distance(&self, point: &Vec3) -> f64 {
        self.rose_sdf(point)
    }

    fn stem_thickness(&self, s: f64) -> f64 {
        lerpf(
            lerpf(0., 5., (s * 25.).min(1.)),
            4.,
            s,
        )
    }

    fn rose_sdf(&self, pt: &Vec3) -> f64 {
        let mut distances: Vec<f64> = vec![];

        // stem
        distances.push(sdf_curve(
            &|s| self.stem_bezier(s),
            &|s| self.stem_thickness(s),
            pt));

        // bottom leaf
        let branch_pt = self.stem_bezier(0.15);
        distances.push(sdf_curve(&|s| Vec3::bezier2(
            &branch_pt,
            &(&branch_pt + &Vec3::new(-50., -60., 0.)),
            &(&branch_pt + &Vec3::new(-100., -80., 0.)),
            s,
        ), &|s| lerpf(5., 1., s), pt));

        let mut sd: Option<f64> = None;
        for d in distances {
            if sd.is_none() || sd.unwrap() > d {
                sd = Some(d);
            }
        }
        sd.unwrap()
    }

    fn stem_bezier(&self, s: f64) -> Vec3 {
        Vec3::bezier3(
            &self.control_points[0],
            &self.control_points[1],
            &self.control_points[2],
            &self.control_points[3],
            s,
        )
    }
}
