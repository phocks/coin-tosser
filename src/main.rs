use rand::prelude::*;

fn main() {
  let how_many_tosses = 10;
  let number_of_rounds = 100;

  let mut average = 0;

  for x in 0..number_of_rounds {
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

  println!("Average: {}", average / number_of_rounds);
}
