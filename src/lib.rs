use rand::prelude::*;
//use rand::rngs::SmallRng;
//got better results on my system using Xoshiro256Plus over SmallRng
use rand_xoshiro::Xoshiro256Plus;
use rayon::prelude::*;


const BIT_MASK: u64 = !0 << 25;// [u64; 4] has 256 bits and we are masking out 25 to get 231

/// A random bit can be 0 or 1 with even odds. Two random bit's 'AND' togther would then have a 1/4 chance to be 1.
/// So we generate 2 sets of 256 bits and 'AND' them together, masking out the extra bits.
/// Then we simply tally the '1' bits to get our procs for an iteration.

#[inline(never)]//not sure what black magic is going on here, but I get worse results when inlining
pub fn bits(total_rolls: u32) -> u32 {//returns max
    (0..total_rolls).into_par_iter().map_init(Xoshiro256Plus::from_entropy, |rng, _i| {
        let a = [rng.gen::<u64>(), rng.gen::<u64>(),rng.gen::<u64>(), rng.gen::<u64>()];
        let b = [rng.gen::<u64>(), rng.gen::<u64>(),rng.gen::<u64>(), rng.gen::<u64>()];

        let c = [a[0] & b[0], a[1] & b[1], a[2] & b[2], a[3] & b[3] & BIT_MASK];
        c[0].count_ones() + c[1].count_ones() + c[2].count_ones() + c[3].count_ones()
    }).max().unwrap()
}