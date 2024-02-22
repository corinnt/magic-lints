pub trait AlohomoraType {
    type Out;
    //fn to_enum(self) -> MagicUnboxEnum;
    //fn from_enum(e: MagicUnboxEnum) -> Result<Self::Out, ()>;
}

#[doc = "Library implementation of AlohomoraType. Test identifier: ALOHOMORA "]
impl AlohomoraType for u8 {
    type Out = u8; 
}
