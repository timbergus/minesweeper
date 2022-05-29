use rand::{thread_rng, Rng};

pub fn random_mine(width: usize, height: usize) -> (usize, usize) {
  let mut rng = thread_rng();
  let w: usize = rng.gen_range(0..width);
  let h: usize = rng.gen_range(0..height);
  return (w, h);
}
