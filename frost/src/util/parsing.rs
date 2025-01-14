use std::io::{self, Read};

#[inline(always)]
pub fn parse_u8(buf: &[u8]) -> io::Result<u8> {
    parse_u8_at(buf, 0)
}

#[inline(always)]
pub fn parse_u8_at(buf: &[u8], index: usize) -> io::Result<u8> {
    let bytes = buf.get(index..index + 1).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Buffer is not large enough to parse 1 bytes",
        )
    })?;
    Ok(u8::from_le_bytes(bytes.try_into().unwrap()))
}

#[inline(always)]
pub fn parse_le_u32(buf: &[u8]) -> io::Result<u32> {
    parse_le_u32_at(buf, 0)
}

#[inline(always)]
pub fn parse_le_u32_at(buf: &[u8], index: usize) -> io::Result<u32> {
    let bytes = buf.get(index..index + 4).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Buffer is not large enough to parse 4 bytes",
        )
    })?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

#[inline(always)]
pub fn parse_le_u64(buf: &[u8]) -> io::Result<u64> {
    parse_le_u64_at(buf, 0)
}

#[inline(always)]
pub fn parse_le_u64_at(buf: &[u8], index: usize) -> io::Result<u64> {
    let bytes = buf.get(index..index + 8).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Buffer is not large enough to parse 8 bytes",
        )
    })?;
    Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
}

#[inline(always)]
pub fn get_lengthed_bytes(reader: &mut impl Read) -> io::Result<Vec<u8>> {
    // Get a vector of bytes from a reader when the first 4 bytes are the length
    // Ex: with <header_len><header> or <data_len><data>, this function returns either header or data
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;

    let len = u32::from_le_bytes(len_buf);
    let mut bytes = vec![0u8; len as usize];
    reader.read_exact(&mut bytes)?;

    Ok(bytes)
}
