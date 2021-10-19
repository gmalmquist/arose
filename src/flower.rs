use crate::sdf;
use crate::sdf::{find_closest_point, sdf_curve};
use crate::threed::{Vec3, Frame};
use crate::utils::{lerpf, exp_smin};

struct LeafGen {
    vein_pairs: usize,

    base_offset: f64,

    // in a basis where vein-tip to vein-tip is (0, 0, 0) to (1, 0, 0),
    // +y is away from the leaf
    // +z is above the top of the leaf
    margin_shape: Box<dyn Fn(f64) -> Vec3>,

    vein_shape: Box<dyn Fn(f64) -> Vec3>,
}

impl LeafGen {
    pub fn vein_distance<C: Fn(f64) -> (Vec3, f64)>(&self, midrib_curve: &C, pt: &Vec3) -> f64 {
        let mut distance: Option<f64> = None;

        for pair_no in 0..self.vein_pairs {
            let t = pair_no as f64 / self.vein_pairs as f64;
            let d = sdf_curve(
                &|s| self.rib_point(midrib_curve, pair_no, s),
                &|s| 1.,
                pt);
            distance = Some(match distance {
                None => d,
                Some(sd) => exp_smin(sd, d, 2.),
            });
        }

        distance.unwrap()
    }

    pub fn base_position<C: Fn(f64) -> (Vec3, f64)>(&self, midrib_curve: &C) -> Vec3 {
        midrib_curve(self.base_offset).0
    }

    pub fn rib_point<C: Fn(f64) -> (Vec3, f64)>(
        &self,
        midrib_curve: &C,
        index: usize,
        s: f64) -> Vec3 {
        let start_t = lerpf(self.base_offset, 1., index as f64 / self.vein_pairs as f64);
        let blade_frame = self.blade_frame(&midrib_curve, start_t);

        let local: Vec3 = (self.vein_shape)(s);

        blade_frame.project(&local)
    }

    pub fn blade_frame<C: Fn(f64) -> (Vec3, f64)>(&self, midrib_curve: &C, s: f64) -> Frame {
        let (origin, _rotation) = midrib_curve(s);
        let length: f64 = (&midrib_curve(1.).0 - &midrib_curve(self.base_offset).0).mag();
        let (next_pos, _) = midrib_curve(s + 0.01);
        let tipward: Vec3 = (&next_pos - &origin).unit();
        let (upward, sidward) = {
            let up = Vec3::new(0., 0., -1.);
            let side = tipward.cross(&up);
            if side.is_zero(0.0001) {
                (Vec3::right(), tipward.cross(&Vec3::right()).unit())
            } else {
                (up, side.unit())
            }
        };

        Frame::new(
            origin,
            sidward.scale_uniform_mut(length),
            tipward.scale_uniform_mut(length),
            upward.scale_uniform_mut(length)
        )
    }
}

pub struct Flower {
    control_points: Vec<Vec3>,
    leaf_gen: LeafGen,
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
            leaf_gen: LeafGen {
                vein_pairs: 6,
                base_offset: 0.1,
                margin_shape: Box::new(|s: f64| Vec3::lerp(&Vec3::zero(), &Vec3::right(), s)),
                vein_shape: Box::new(|s: f64| Vec3::lerp(&Vec3::zero(), &Vec3::right(), s)),
            },
        }
    }

    pub fn update_controls(&mut self, points: &Vec<Vec3>) {
        assert_eq!(self.control_points.len(), points.len());
        for i in 0..points.len() {
            self.control_points[i] = points[i].clone();
        }
    }

    pub fn distance(&self, point: &Vec3) -> f64 {
        self.vascular_sdf(point)
    }

    fn stem_thickness(&self, s: f64) -> f64 {
        lerpf(
            lerpf(0., 5., (s * 25.).min(1.)),
            lerpf(4., 3., s),
            s,
        )
    }

    fn vascular_sdf(&self, pt: &Vec3) -> f64 {
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

    fn leaf<C: Fn(f64) -> (Vec3, f64)>(&self, midrib: &C, pt: &Vec3) -> f64 {
        self.leaf_gen.vein_distance(&midrib, pt)
    }

    fn bottom_leaf(&self, pt: &Vec3) -> f64 {
        let branch_pt = self.stem_bezier(0.15);
        let midrib = |s: f64| Vec3::bezier2(
            &branch_pt,
            &(&branch_pt + &Vec3::new(-50., -60., 0.)),
            &(&branch_pt + &Vec3::new(-100., -80., 0.)),
            s,
        );
        exp_smin(
            sdf_curve(&midrib, &|s| lerpf(4., 1., s), pt),
            self.leaf_gen.vein_distance(&|s| (midrib(s), 0.), pt),
            2.,
        )
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
                    s,
                ),
                &|s: f64| lerpf(stem_thickness(branch_s), 1., s),
                pt,
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
