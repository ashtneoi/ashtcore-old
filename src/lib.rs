#![no_std]

pub trait Write<T> { }

pub trait WriteText { }

pub struct Utf8TextWriter<W: Write<u8>> {
    writer: W,
}

impl<W: Write<u8>> WriteText for Utf8TextWriter<W> { }
