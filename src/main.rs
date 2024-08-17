use graveler::bits;

const TOTAL_ROLLS: u32 = 1_000_000_000;

fn main() {
    let now = std::time::Instant::now();

    let max = bits(TOTAL_ROLLS);

    println!("Highest Ones Roll: {}", max);
    println!("Number of Roll Sessions: {}", TOTAL_ROLLS);
    println!("Time Elapsed: {:.2}", now.elapsed().as_secs_f32());
}