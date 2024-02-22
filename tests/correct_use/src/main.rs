use trait_derive:: AlohomoraType;
use trait_def::AlohomoraType; 

#[derive(AlohomoraType)]
#[out_type(name = "GoodStructOut", to_derive = [Debug])]
pub struct GoodStruct {
    _f1: u8,
}

impl GoodStructOut {
    pub fn new(num: u8) -> Self {
        Self {_f1: num}
    }
}

fn main() {
    let mystruct = <GoodStruct as AlohomoraType>::Out::new(42);
    println!("{:?}", mystruct); 
}
