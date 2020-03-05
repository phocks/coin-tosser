use rand::prelude::*;

const HOW_MANY_TOSSES: u32 = 12;
const NUMBER_OF_ROUNDS: u32 = 16;

fn main() {
  let mut average = 0;

  for _x in 0..NUMBER_OF_ROUNDS {
    let mut iteration = 0;
    let mut done = false;

    while !done {
      println!("Iteration: {}", iteration);

      let mut heads = 0;
      let mut tails = 0;

      for _x in 0..HOW_MANY_TOSSES {
        if random() {
          // generates a boolean
          // println!("Heads!");
          heads += 1;
        } else {
          // println!("Tails!");
          tails += 1;
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
    average += iteration;
  }

  println!("Average: {}", average as f32 / NUMBER_OF_ROUNDS as f32);
}
