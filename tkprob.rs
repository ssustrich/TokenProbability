use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..9);

    // A counter variable
    let mut n = 1;
    //let triples = 0;
    let mut pin_size_counter = 0;
    // Loop while `n` is less than 101
    while n < 100 {
        print!("run {}   ", n );
        let mut pinDigits: [i32; 10] = [0,0,0,0,0,0,0,0,0,0];
        let mut pinMeta: [i32; 5] = [0,0,0,0,0];
        while pin_size_counter <6 {
            let throw = die.sample(&mut rng);
            pinDigits[throw] = pinDigits[throw] +1; 
            print!("{}", throw);
            //let s: String = throw.to_string();
            //pin.push_str(&s);
            pin_size_counter+=1;
            }
          print!("  ");
        for i in 0..pinDigits.len() {
           if pinDigits[i] == 2 {
                pinMeta[0] = pinMeta[0] + 1;
           }
           if pinDigits[i] == 3 {
                pinMeta[1] = pinMeta[1] + 1;
           }
           if pinDigits[i] == 4 {
                pinMeta[2] = pinMeta[2] + 1;
           }
           if pinDigits[i] == 5 {
                pinMeta[3] = pinMeta[3] + 1;
           }
           if pinDigits[i] == 6 {
                pinMeta[4] = pinMeta[4] + 1;
           }
        }
        
        for j in 0..pinMeta.len() {
            print!("{} ", pinMeta[j] )
        }
        println!("-----",);
        pin_size_counter = 0;
        n+=1;
        }
    }
