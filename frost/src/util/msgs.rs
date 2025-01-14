use std::borrow::Cow;
use std::io::{Read, Seek};

use serde;
use serde::de;
use serde_rosmsg;

use crate::errors::{Error, ErrorKind};
use crate::{Bag, ChunkHeaderLoc};

pub trait Msg {}

pub struct MessageView<'a, R: Read + Seek> {
    pub topic: &'a str,
    pub(crate) bag: &'a Bag<R>,
    pub(crate) chunk_loc: ChunkHeaderLoc,
    pub(crate) start_index: usize,
    pub(crate) end_index: usize,
}

impl<'a, R: Read + Seek> MessageView<'a, R> {
    pub fn instantiate<'de, T>(&self) -> Result<T, Error>
    where
        T: Msg,
        T: de::Deserialize<'de>,
    {
        let Some(bytes) = self.bag.chunk_bytes.get(&self.chunk_loc) else {
            return Err(Error::new(ErrorKind::InvalidBag(Cow::Borrowed(
                "Supplied chunk loc for msg view doesn't exist",
            ))))
        };
        serde_rosmsg::from_slice(&bytes[self.start_index..self.end_index]).map_err(|e| e.into())
    }
}
