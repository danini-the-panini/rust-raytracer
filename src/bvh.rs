use std::cmp::Ordering;

use crate::{hittable::{Hittable, HitRecord}, aabb::AABB, util::random_int};

enum BVHNode {
  Branch { left: Box<BVH>, right: Box<BVH> },
  Leaf(Box<dyn Hittable>)
}

pub struct BVH {
  tree: BVHNode,
  bbox: AABB
}

impl BVH {
  pub fn new(mut objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
    let len = objects.len();

    let comparator = match random_int(0, 2) {
      0 => box_x_compare,
      1 => box_y_compare,
      _ => box_z_compare
    };

    match len {
      0 => panic!("no elements in BVH constructor"),
      1 => {
        let leaf = objects.pop().unwrap();
        if let Some(bbox) = leaf.bounding_box(time0, time1) {
          BVH { tree: BVHNode::Leaf(leaf), bbox }
        } else {
          panic!("no bounding box in BVH constructor");
        }
      },
      _ => {
        objects.sort_unstable_by(comparator);

        let left = BVH::new(objects.drain(len/2..).collect(), time0, time1);
        let right = BVH::new(objects, time0, time1);
        let bbox = AABB::surrounding_box(&left.bbox, &right.bbox);
        BVH { tree: BVHNode::Branch { left: Box::new(left), right: Box::new(right) }, bbox }
      }
    }
  }
}

impl Hittable for BVH {
  fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    if !self.bbox.hit(&r, t_min, t_max) { return None };

    match &self.tree {
      BVHNode::Leaf(leaf) => leaf.hit(&r, t_min, t_max),
      BVHNode::Branch { left, right } => {
        let hit_left = left.hit(&r, t_min, t_max);
        let t_max = if let Some(rec) = &hit_left { rec.t } else { t_max };
        let hit_right = right.hit(&r, t_min, t_max);

        hit_right.or(hit_left)
      }
    }
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
    Some(self.bbox)
  }
}

fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: i32) -> Ordering {
  let box_a = a.bounding_box(0.0, 0.0);
  let box_b = b.bounding_box(0.0, 0.0);

  match (box_a, box_b) {
    (Some(box_a), Some(box_b)) => {
      box_a.min()[axis as usize].total_cmp(&box_b.min()[axis as usize])
    },
    _ => panic!("No bounding box in BVH constructor.")
  }
}

fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering { box_compare(a, b, 0) }
fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering { box_compare(a, b, 1) }
fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering { box_compare(a, b, 2) }