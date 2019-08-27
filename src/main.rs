use rand::prelude::*;

fn main() {
  let how_many_tosses = 15;

  let mut iteration = 0;
  let mut done = false;

  while !done {
    println!("Iteration: {}", iteration);

    let mut heads = 0;
    let mut tails = 0;

    for x in 0..how_many_tosses {
      if random() {
        // generates a boolean
        // println!("Heads!");
        heads = heads + 1;
      } else {
        // println!("Tails!");
        tails = tails + 1;
      }
    }

    println!("{} heads", heads);
    println!("{} tails", tails);

    if heads == 0 {
      done = true;
    }
    if tails == 0 {
      done = true;
    }
    iteration += 1;
  }

  println!("Took {} iterations to get zero result!", iteration);
}
