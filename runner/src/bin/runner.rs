use std::time::Instant;
fn main() {
  let now = Instant::now();
  runner::jobs().iter().for_each(|j| {
    println!("{}", j.1);
    j.0()
  });
  println!("total time: {}ms", now.elapsed().as_millis());
}
