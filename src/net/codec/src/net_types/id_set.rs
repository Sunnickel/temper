use crate::encode::errors::NetEncodeError;
use crate::encode::{NetEncode, NetEncodeOpts};
use crate::net_types::var_int::VarInt;
use std::io::Write;

pub enum IDSet {
    Indirect(String),
    Direct(Vec<VarInt>),
}

impl NetEncode for IDSet {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            IDSet::Indirect(tag_name) => {
                VarInt::new(0).encode(writer, opts)?;
                tag_name.encode(writer, opts)
            }
            IDSet::Direct(tags) => {
                VarInt::new((tags.len() - 1) as i32).encode(writer, opts)?;
                for tag in tags {
                    tag.encode(writer, opts)?;
                }
                Ok(())
            }
        }
    }
}
