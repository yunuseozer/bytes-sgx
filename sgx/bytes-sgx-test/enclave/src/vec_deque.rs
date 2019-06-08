//#[test]
use bytes::*;
use std::prelude::v1::*;
use std::collections::VecDeque;

pub fn hello_world() {
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(b"hello world");
    assert_eq!(11, buffer.remaining());
    assert_eq!(b"hello world", buffer.bytes());
    buffer.advance(6);
    assert_eq!(b"world", buffer.bytes());
    buffer.extend(b" piece");
    assert_eq!(b"world piece" as &[u8], &buffer.collect::<Vec<u8>>()[..]);
}
