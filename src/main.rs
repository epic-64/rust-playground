mod peano;

use crate::peano::Peano;

fn main() {
    let a = Peano::from_int(3);
    let b = Peano::from_int(5);
    let c = a.div(&b);

    match c {
        Ok(result) => println!("3 / 5 = {:?}", result.to_int()),
        Err(e) => println!("Error: {:?}", e),
    }
}

