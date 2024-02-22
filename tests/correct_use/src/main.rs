use trait_derive:: AlohomoraType;
use trait_def::AlohomoraType; 

#[derive(AlohomoraType, Clone, Debug)]
#[out_type(name = "GoodStructOut")]
pub struct GoodStruct {
    _f1: u8,
}

impl GoodStructOut {
    pub fn new(num: u8) -> Self {
        Self {_f1: num}
    }
}

fn main() {
    let mystruct = GoodStruct{ _f1: 42 };
    println!("{:?}", mystruct); 
}
