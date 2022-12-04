use crate::{util::{random_int}, vec3::{Point3, Vec3, dot}};

const POINT_COUNT: usize = 256;

#[derive(Debug, Clone, Copy)]
pub struct Perlin {
  ranvec: [Vec3; POINT_COUNT],
  perm_x: [i32; POINT_COUNT],
  perm_y: [i32; POINT_COUNT],
  perm_z: [i32; POINT_COUNT]
}

impl Perlin {
  pub fn new() -> Self {
    let mut ranvec = [Vec3::zero(); POINT_COUNT];
    for i in 0..POINT_COUNT { ranvec[i] = Vec3::random_unit_vector() };

    Self {
      ranvec,
      perm_x: perlin_generate_perm(),
      perm_y: perlin_generate_perm(),
      perm_z: perlin_generate_perm(),
    }
  }

  pub fn noise(&self, p: &Point3) -> f64 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = p.x().floor() as i32;
    let j = p.y().floor() as i32;
    let k = p.z().floor() as i32;
    let mut c = [[[Vec3::zero();2];2];2];

    for di in 0..2 {
      for dj in 0..2 {
        for dk in 0..2 {
          c[di][dj][dk] = self.ranvec[
            (self.perm_x[((i+di as i32) & 255) as usize] ^
            self.perm_y[((j+dj as i32) & 255) as usize] ^
            self.perm_z[((k+dk as i32) & 255) as usize]) as usize
          ];
        }
      }
    }

    perlin_interp(c, u, v, w)
  }

  pub fn turb(&self, p: &Point3) -> f64 {
    self.turbd(p, 7)
  }

  pub fn turbd(&self, p: &Point3, depth: i32) -> f64 {
    let mut accum = 0.0;
    let mut temp_p = *p;
    let mut weight = 1.0;

    for _ in 0..depth {
      accum += weight*self.noise(&temp_p);
      weight *= 0.5;
      temp_p *= 2.0;
    }

    accum.abs()
  }
}

fn perlin_generate_perm() -> [i32; POINT_COUNT] {
  let mut p = [0; POINT_COUNT];

  for i in 0..POINT_COUNT { p[i] = i as i32 };

  permute(&mut p);

  p
}

fn permute(p: &mut [i32; POINT_COUNT]) {
  for i in (1..POINT_COUNT).rev() {
    let target = random_int(0, i as i32) as usize;
    (p[i], p[target]) = (p[target], p[i])
  }
}

fn perlin_interp(c: [[[Vec3;2];2];2], u: f64, v: f64, w: f64) -> f64 {
  let uu = u*u*(3.0-2.0*u);
  let vv = v*v*(3.0-2.0*v);
  let ww = w*w*(3.0-2.0*w);
  let mut accum = 0.0;

  for i in 0..2 {
    for j in 0..2 {
      for k in 0..2 {
        let weight_v = Vec3::new(u-i as f64, v-j as f64, w-k as f64);
        accum += (i as f64*uu + (1.0-i as f64)*(1.0-uu)) *
                (j as f64*vv + (1.0-j as f64)*(1.0-vv)) *
                (k as f64*ww + (1.0-k as f64)*(1.0-ww))*dot(&c[i][j][k], &weight_v);
      }
    }
  }
  accum
}