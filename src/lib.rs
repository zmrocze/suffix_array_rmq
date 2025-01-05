
mod rmq;
mod sa;
mod tests;

/// Creates a SA+RMQ structure which allows to query for any two indices in the sequence,
/// what is the length of the longest common prefix of the suffixes starting at these indices.
/// O(n) creation time, O(1) query time
pub fn create_sa(a: Vec<usize>) -> SARMQ {
  let sa = sa::SA::create_suffix_array(&a);
  let lcp = sa::lcp(&a, &sa);
  let cart_tree = sa::cartesian_tree(&lcp);
  let euler_walk = cart_tree.euler_walk();
  let rmq = rmq::create_rmq(&euler_walk.euler, &euler_walk.depth);
  SARMQ { euler_walk, rmq }
}

pub struct SARMQ {
  euler_walk: sa::Euler,
  rmq: rmq::RMQ,
}

impl SARMQ {
  /// Returns the length of the longest common prefix of the suffixes starting at the given indices.
  pub fn query(&self, i: usize, j: usize) -> usize {
    // 
    let argmin = self.rmq.query(self.euler_walk.first_occ[i], self.euler_walk.first_occ[j]);
    self.euler_walk.euler[argmin]
  }
}