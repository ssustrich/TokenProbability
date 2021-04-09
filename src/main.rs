use rand::distributions::{Distribution, Uniform};

fn main() {

     let mut pin = vec![];
     let mut spread: [usize; 10] = [0,0,0,0,0,0,0,0,0,0];
     let mut spreadPercent: [f32; 10] = [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
     let mut pin_size: usize = 6;
     let mut n = 100000000;
     let mut duplicates = 0;
    for i in 0..n {
     generateAPin(pin_size,&mut pin, &mut spread );
     if calculatePercentages(pin_size, pin){
          duplicates+=1;
     }
     pin = vec![];
    }
    validateDist(n, pin_size, &mut spread, &mut spreadPercent);

    println!("Over the course of {} pin generations I found {} pins with at least to repeated digits",n, duplicates);
    let percentOfDuplcates = duplicates as f32 /n as f32;
    let s = format!("{:.1}%", 100.0 * percentOfDuplcates);
    println!("{} of the pins had at least two repeated numbers", s);


}

    fn generateAPin(mut pin_length: usize, pin: &mut Vec<i32>, spread: &mut [usize]) {
         let mut rng = rand::thread_rng();
          let die = Uniform::from(0..10);

          for i in 0..pin_length{
               let throw = die.sample(&mut rng);
               let value: usize = throw as usize;
               pin.push(throw);
               spread[value] +=1;
  
          }
  }

     fn calculatePercentages(pin_length: usize, pin: Vec<i32>) -> bool {
          //println!("checking pin {:?} for duplicates ", pin);
          let mut duplicateFound = 0;
          for i in 0..(pin_length-1){
               for j in (i+1)..pin_length{
                   // println!("    checking pin to see if position {} equals position {}", i,j);
                    if pin[i] == pin[j]{
                     //    println!("        Yes {} equals {}", pin[i], pin[j]);
                         return true;
                    }
               }
            }

            return false;


     }
     fn validateDist(n: usize, mut pin_length: usize, spread: &mut [usize], spreadPercent: &mut [f32]) {
          for i in 0..10{
               spreadPercent[i] = (spread[i] as f32 /n as f32);
          }
          println!("The spread of digits in total was {:?}", spreadPercent);
     }