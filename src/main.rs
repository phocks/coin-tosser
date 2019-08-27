use rand::prelude::*;

fn main() {
  // We can use random() immediately. It can produce values of many common types:
  // let x: u8 = random();
  // println!("{}", x);

  let mut heads = 0;
  let mut tails = 0;

  for x in 0..10 {
    if random() {
      // generates a boolean
      println!("Heads!");
      heads = heads + 1;
    } else {
      println!("Tails!");
      tails = tails + 1;
    }
  }

  println!("{} heads", heads);
  println!("{} tails", tails);
}
