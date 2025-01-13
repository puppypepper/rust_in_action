use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

fn main() {
    let base = Rc::new(RefCell::new(GroundStation { radio_freq: 86.65 }));

    println!("base: {:?}", base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq = 22.65;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq = 33.65;
    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
