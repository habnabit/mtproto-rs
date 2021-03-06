use std::io::{Read, Write};
use tl::{self, Type};
use tl::parsing::{ConstructorId, ReadContext, WriteContext};
use tl::dynamic::{TLDynamic, ClassStore, TLObject};

#[derive(Copy, Clone, Debug)]
pub struct Null;

impl Null {
    pub const SIGNATURE: ConstructorId = ConstructorId(0x56730bcc);
}

impl Type for Null {
    fn bare_type() -> bool {
        false
    }
    
    fn type_id(&self) -> Option<ConstructorId> {
        Some(Null::SIGNATURE)
    }
    
    fn serialize<W: Write>(&self, _: &mut WriteContext<W>) -> tl::Result<()> {
        Ok(())
    }
    
    fn deserialize<R: Read>(_: &mut ReadContext<R>) -> tl::Result<Self> {
        Err(tl::Error::BoxedAsBare)
    }
    
    fn deserialize_boxed<R: Read>(id: ConstructorId, _: &mut ReadContext<R>) -> tl::Result<Self> {
        match id {
            Null::SIGNATURE => Ok(Null),
            _ => Err(tl::Error::InvalidData),
        }
    }
}

impl TLDynamic for Null {
    fn register_ctors(cstore: &mut ClassStore) {
        fn do_deser(_: ConstructorId, _: &mut ReadContext<&mut Read>) -> tl::Result<Box<TLObject>> {
            Ok(Box::new(Null))
        }
        cstore.add_ctor(Null::SIGNATURE, do_deser)
    }
}
