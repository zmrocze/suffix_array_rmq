// Implements range minimum query (RMQ) structure, assuming in the sequence numbers differ at most by 1.

use std::{iter, vec};

use num::Bounded;
use num_bigint::BigUint;
use unzip3::Unzip3;

#[derive(Debug, Clone, Copy)]
pub struct Min<A, B> {
  pub min: A,
  pub argmin: B,
}

impl<A : Ord, B> Min<A, B> {
  pub fn new(min: A, argmin: B) -> Self {
    Min { min, argmin }
  }
  // pub fn update_min(&mut self, new_min: usize, new_argmin: usize) {
  //   if new_min < self.min {
  //     self.min = new_min;
  //     self.argmin = new_argmin;
  //   }
  // }
  pub fn min(self, other: Self) -> Self {
    if self.min > other.min {
      other
    } else {
      self
    }
  }
}

impl<A : Bounded, B : Default> Default for Min<A, B> {
  fn default() -> Self {
    Min { min: A::max_value(), argmin: B::default() }
  }
}

/// Structure for range minimum queries (argmin) on a sequence of numbers.
/// O(n) space, O(1) query time.
// ranges inclusive for both ends
pub struct RMQ {
  // n : usize,
  b : usize, // ~ log n
  // m : usize, // n / b
  // logm : usize,
  ranges_answers: Vec<Vec<Min<usize, usize>>>, // answers for 2^k sized ranges
  block_firsts: Vec<usize>,    // values for first elements of each block
  block_types: Vec<usize>, // block types
  block_answers: Vec<Vec<Min<i64, usize>>>, // precomputed answers for each block type
}

/// Defines how to get an index for a given range WITHIN a block
pub const fn query_index(left : usize, right : usize) -> usize {
  right*(right+1)/2 + left
}

impl RMQ {

  /// argminimum of the range [left, right]
  pub fn query(&self, left : usize, right : usize) -> usize {
    assert!(left <= right, "Start of range is not on the left of the end of the range.");
    println!("[{:?}, {:?}], b={:?}", left, right, self.b);
    let l_block = left / self.b; // starts in it
    let r_block = right / self.b; // ends in it 
    if l_block >= r_block {
      // within block
      // checking answer in the precomputed answers table for the block
      let l = left % self.b;
      let r = right % self.b;
      let min = self.block_answers[self.block_types[r_block]][query_index(l, r)];
      println!("within block {:?}: [{:?}, {:?}]", r_block, l, r);
      return r_block * self.b + min.argmin;
    } else {
      // across blocks: |left scraps|RANGE OF BLOCKS|right scraps|
      // Cover the range of blocks with two ranges of size 2^k, read answer,
      // read answers for the within block scraps - take min.
      let range_min = {
        let log = (((r_block - (l_block+1)) as f64).log2() - 1.0).ceil() as usize;
        if r_block > 1 + l_block {
          // println!("[{:?} {:?}] of len {:?} = [{:?} {:?}] + [{:?} {:?}] of lens 2^{:?}", l_block + 1, r_block - 1 , r_block - (l_block+1), l_block + 1, l_block + (1<<log), r_block - (1<<log), r_block - 1, log);
          self.ranges_answers[log][l_block+1].min( self.ranges_answers[log][r_block - (1<<log)] )
        } else {
          Min::default()
        }
      };
      let left_scraps = self.block_answers[self.block_types[l_block]][query_index(left % self.b, self.b - 1)];
      let right_scraps = self.block_answers[self.block_types[r_block]][query_index(0, right % self.b)];
      let left_scraps = Min::new(((self.block_firsts[l_block] as i64) + left_scraps.min) as usize, left_scraps.argmin + l_block * self.b);
      let right_scraps = Min::new(((self.block_firsts[r_block] as i64) + right_scraps.min) as usize, right_scraps.argmin + r_block * self.b);
      // println!("left_scraps: {:?}, range_min: {:?} , right_scraps: {:?}", left_scraps, range_min, right_scraps);
      left_scraps.min(range_min).min(right_scraps).argmin
    }
  }

  pub fn create_rmq(values: &Vec<usize>  ) -> RMQ {

    let n = values.len();
    let c = 4;
    let b = 1.max( (n as f64).log2().ceil() as usize / c );
    // chunk into b sized blocks, extend last block to match
    let vals = values.clone(); // note: would work with 0..n as iterator but with #![feature(iter_array_chunks)]
    let chunks: Box<dyn Iterator<Item = Vec<usize>>> = {
      let chunks = vals.chunks_exact(b);
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
  
    let block_type = |diffs: &Vec<i8>| {
      diffs.iter().fold(0, |acc, diff| acc * 3 + ((diff + 1) as usize))
    };



    let (block_mins, block_firsts, block_difftypes): (Vec<Min<usize, usize>>, Vec<usize>, Vec<Vec<i8>>) = chunks.map(|chunk| {
      let min = chunk.iter().enumerate().map(|(i, min)| Min::new(*min, i))
        .fold(Min::default(), Min::min);
      let first = chunk[0];
      let mut prev = first;
      let mut difftype = vec![0];
      for a in &chunk[1..] {
        let diff = (( *a as i64) - (prev as i64)) as i8;
        assert!(-1 <= diff && diff <= 1, "diff by -1/0/+1");
        difftype.push(diff);
        prev = *a;
      }
      (min, first, difftype)
    }).unzip3();
    let m = block_mins.len(); // n / b
    let logm = ((m as f64).log2().ceil() as usize).max(1 as usize);
    println!("m: {:?}, logm: {:?}", m , logm);
    
    let pow2 = |x: usize| 1 << x;
    // calculate ranges bottom up
    let mut range_answers = vec![vec![Min::default(); m]; logm];
    // let mut ranges_argmins = vec![vec![0; m]; logm];
    for i in 0..m { println!("blockmin at [{:?}] is {:?}", i, Min::new(block_mins[i].min, i * b + block_mins[i].argmin)); range_answers[0][i] = Min::new(block_mins[i].min, i * b + block_mins[i].argmin) }
    for i in 1..logm {
      // println!("2<<0={:?}", 1<<(0));
      // println!("2<<1={:?}", 1<<(1));
      for j in 0..m {
        // min of two smaller ranges_mins
        // println!("i={:?}, j={:?}", i, j);
        // if ( i <= 0 ) {
        //   println!("i=0")
        // } else {
        //   // println!("[{:?} {:?}] of len 2^{:?} = [{:?} {:?}] + [{:?} {:?}] of len 2^{:?}", j, j+ (1<<i) - 1 , i , j, j + (1<<(i-1)) - 1, j + (1<<(i-1)), j + (1<<(i-1)) + (1<<(i-1)) - 1, i-1);
        // }
        range_answers[i][j] = range_answers[i-1][j].min( *range_answers[i-1].get( j + pow2(i-1) ).unwrap_or(&Min::default()) );
      }
    }
  
    // precompute answers for within block queries
    let r: usize = (3 as usize).pow(b as u32); // "Expecting <number of distinct blocks (somesmallpoly(n)> to fit in usize")
    let s = (b+1) * b / 2; // see query_index
    let mut block_answers = vec![vec![Min::default(); s]; r];
    let mut types_done = vec![false; r];
    let mut block_types = vec![0; m];
    for i in 0..m {
      let bl_difftype = &block_difftypes[i]; 
      let bl_type = block_type(bl_difftype);
      if ! types_done[bl_type] {
        // O(b^2) computation of all range mins
        let mut val_at_x: i64 = 0;
        for x in 0..b { // start
          let mut val_at_y = val_at_x;
          val_at_x += bl_difftype[x] as i64;
          let mut min = Min::new(val_at_x, x);
          for y in x..b { // end
            val_at_y += bl_difftype[y] as i64;
            min = min.min(Min::new(val_at_y, y));
            block_answers[bl_type][query_index(x, y)] = min;
          }
        }
      }
      block_types[i] = bl_type;
      types_done[bl_type] = true;
    }
  
    RMQ {
      // n : n,
      b : b,
      // m: m,
      // logm : logm,
      ranges_answers: range_answers,
      block_firsts: block_firsts,
      block_types: block_types,
      block_answers: block_answers,
    }
  }

}