use crate::sdf;
use crate::sdf::{find_closest_point, sdf_curve};
use crate::threed::Vec3;
use crate::utils::{lerpf, exp_smin};

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
            lerpf(4., 3., s),
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

        distances.push(self.bottom_leaf(pt));
        distances.push(self.top_leaf(pt));
        distances.push(self.middle_leaf(pt));

        let mut sd: Option<f64> = None;
        for d in distances {
            sd = Some(match sd {
                None => d,
                Some(sd) => exp_smin(sd, d, 2.),
            });
        }
        sd.unwrap()
    }

    fn bottom_leaf(&self, pt: &Vec3) -> f64 {
        let branch_pt = self.stem_bezier(0.15);
        sdf_curve(&|s| Vec3::bezier2(
            &branch_pt,
            &(&branch_pt + &Vec3::new(-50., -60., 0.)),
            &(&branch_pt + &Vec3::new(-100., -80., 0.)),
            s,
        ), &|s| lerpf(4., 1., s), pt)
    }

    fn top_leaf(&self, pt: &Vec3) -> f64 {
        let branch_pt = self.stem_bezier(0.55);
        sdf_curve(&|s| Vec3::bezier2(
            &branch_pt,
            &(&branch_pt + &Vec3::new(-50., -60., 0.)),
            &(&branch_pt + &Vec3::new(-90., -90., 0.)),
            s,
        ), &|s| lerpf(4., 1., s), pt)
    }

    fn middle_leaf(&self, pt: &Vec3) -> f64 {
        let branch_pt = self.stem_bezier(0.45);
        let stem_pos = |s: f64| Vec3::bezier2(
            &branch_pt,
            &(&branch_pt + &Vec3::new(40., -120., 0.)),
            &(&branch_pt + &Vec3::new(20., -160., 0.)),
            s,
        );
        let stem_thickness = |s: f64| lerpf(4., 1., s);

        let branch_s = 0.23;

        exp_smin(
            sdf_curve(&stem_pos, &stem_thickness, pt),
            sdf_curve(
                &|s: f64| &stem_pos(branch_s) + &Vec3::bezier2(
                    &Vec3::zero(),
                    &Vec3::new(70., -50., 0.),
                    &Vec3::new(50., -90., 0.),
                    s
                ),
                &|s: f64| lerpf(stem_thickness(branch_s), 1., s),
                pt
            ),
            2.,
        )
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
