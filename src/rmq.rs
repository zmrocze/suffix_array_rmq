// Implements range minimum query (RMQ) structure, assuming in the sequence numbers differ at most by 1.

use std::{iter, vec};

use num_bigint::BigUint;
use unzip3::Unzip3;

/// Structure for range minimum queries on a sequence of numbers.
/// O(n) space, O(1) query time.
// ranges inclusive for both ends
pub struct RMQ {
  n : usize,
  b : usize, // ~ log n
  m : usize, // n / b
  logm : usize,
  ranges: Vec<Vec<usize>>, // answers for 2^k sized ranges
  starts: Vec<usize>,    // values for first elements of each block
  difftypes: Vec<usize>, // block types
  answers: Vec<Vec<usize>>, // precomputed answers for each block type
}

/// Defines how to get an index for a given range WITHIN a block
pub const fn query_index(left : usize, right : usize) -> usize {
  right*(right+1)/2 + left
}

impl RMQ {

  // minimum of the range [left, right]
  pub fn query(&self, left : usize, right : usize) -> usize {
    if left > right { return usize::MAX }
    let l_block = (left as f64 / self.b as f64).ceil() as usize; // starts in it
    let r_block = (right as f64 / self.b as f64).floor() as usize; // ends in it 
    if l_block >= r_block {
      // within block
      // checking answer in the precomputed answers table for the block
      let l = left % self.b;
      let r = right % self.b;
      self.starts[r_block] + self.answers[self.difftypes[r_block]][query_index(l, r)]
    } else {
      // across blocks: |left scraps|RANGE OF BLOCKS|right scraps|
      // Cover the range of blocks with two ranges of size 2^k, read answer,
      // read answers for the within block scraps - take min.
      let log = (((r_block - l_block) as f64).log2() - 1.0).ceil() as usize;
      // log=ceil(log(s)-1)
      // s = 2^k => log=k-1 and 2^(k-1) + 2^(k-1) >= s
      // 2^k < s < 2^(k+1) => log=k and 2^k + 2^k >= s
      let range_min = self.ranges[log][l_block].min( self.ranges[log][l_block + (1<<log) - r_block] );
      let left_scraps = self.starts[l_block] + self.answers[self.difftypes[l_block]][query_index(left % self.b, self.b - 1)];
      let right_scraps = self.starts[r_block] + self.answers[self.difftypes[r_block]][query_index(0, right % self.b)];
      left_scraps.min(range_min).min(right_scraps)
    }
  }
}

pub fn create_rmq( xs : &Vec<usize>, keys: &Vec<usize>  ) -> RMQ {

  let n = xs.len();
  let c = 4;
  let b = (n as f64).log2().ceil() as usize / c;
  println!("b: {:?}, c: {:?}, n: {:?}, xs: {:?}", b, c, n, xs);
  // chunk into b sized blocks, extend last block to match
  let chunks: Box<dyn Iterator<Item = Vec<usize>>> = {
    let chunks = xs.chunks_exact(b);
    let remainder = chunks.remainder();
    let chunks = chunks.map(|c| Vec::from(c));
    if ! remainder.is_empty() {
      let z = remainder.last().unwrap();
      let last = {
        let mut last: Vec<usize> = remainder.iter().map(Clone::clone).collect();
        last.extend(iter::repeat(z).take(b - last.len()));
        last
      };
      Box::new(chunks.chain(iter::once(last)))
    } else {
      Box::new(chunks)
    }
  };

  let difftype_index = |diffs: &Vec<i8>| {
    diffs.iter().fold(0, |acc, diff| acc * 3 + ((diff + 1) as usize))
  }; 
  let (mins, starts, difftypes): (Vec<usize>, Vec<usize>, Vec<Vec<i8>>) = chunks.map(|chunk| {
    let argmin = *chunk.iter().min_by_key(|&x| keys[*x]).unwrap();
    let first = keys[chunk[0]];
    let mut prev = keys[first];
    let mut difftype = vec![0];
    for a in &chunk[1..] {
      let diff = (( keys[*a] as i64) - (prev as i64)) as i8;
      assert!(-1 <= diff && diff <= 1, "diff by -1/0/+1");
      difftype.push(diff);
      prev = keys[*a];
    }
    (argmin, first, difftype)
  }).unzip3();
  let m = mins.len(); // n / b
  let logm = (m as f64).log2().ceil() as usize;

  // calculate ranges bottom up
  let mut ranges = vec![vec![0; m]; logm];
  for i in 0..m { ranges[0][i] = mins[i] }
  for i in 1..logm {
    for j in 0..m {
      // min of two smaller ranges
      ranges[i][j] = ranges[i-1][j].min( *ranges[i-1].get( j + (2<<(i-1)) ).unwrap_or(&usize::MAX) )
    }
  }

  // precompute answers for within block queries
  let r: usize = (3 as usize).pow(b as u32); // "Expecting <number of distinct blocks (somesmallpoly(n)> to fit in usize")
  let s = (b+1) * b / 2; // see query_index
  let mut answers = vec![vec![usize::MAX; s]; r];
  let mut done = vec![false; r]; 
  let mut difftype_indices = vec![0; m];
  println!("r : {}, s : {}, b : {}", r, s, b); 
  for i in 0..m {
    let dt = &difftypes[i]; 
    let dt_index = difftype_index(dt);
    if ! done[dt_index] {
      // wasteful in O(b^3), could be O(b^2)
      for x in 0..b {
        for y in x..b {
          let mut a: usize = 0;
          let mut min = usize::MAX;
          for z in x..=y {
            a += dt[z] as usize;
            min = min.min(a);
          }
          println!("answerslen: {}, answers[0].len: {}, quer: {}, dt_index: {:?}, x: {:?}, y: {:?}, min: {:?}", answers.len(), answers[0].len(), query_index(x, y), dt_index, x, y, min);
          answers[dt_index][query_index(x, y)] = min;
        }
      }
    }

    difftype_indices[i] = dt_index;
    done[dt_index] = true;
  }

  RMQ {
    n : n,
    b : b,
    m: m,
    logm : logm,
    ranges: ranges,
    starts: starts,
    difftypes: difftype_indices,
    answers: answers,
  }
}