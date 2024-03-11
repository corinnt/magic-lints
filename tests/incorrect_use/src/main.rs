use trait_def::AlohomoraType; 

#[derive(Debug)]
pub struct EveStruct {
    _f1: u8,
}

pub struct EveStruct2{
    _f1: u8,
}

pub struct EveStructOut {
    _f1: u8,
}

impl AlohomoraType for EveStruct {
    type Out = EveStructOut; 
}

impl AlohomoraType for EveStruct2 {
    type Out = EveStructOut; 
}

fn main() {
    let mystruct = EveStruct{ _f1: 42 };
    println!("hi from {:?}", mystruct); 
}

