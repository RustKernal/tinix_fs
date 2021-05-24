use super::flags::*;
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;

#[derive(Debug, Clone, Copy)]
pub struct FileDescriptor(u32);
#[derive(Debug, Clone, Copy)]
pub struct FileFlags(u8);
#[derive(Debug, Clone, Copy)]
pub struct File {
    name    : &'static String,
    handle  : FileDescriptor,
    flags   : FileFlags,
}

pub trait FileIO<T> {
    fn read (&mut self) -> T;
    fn write(&mut self, item : T);
}
#[derive(Debug)]
pub struct NullFile;

impl FileIO<u8> for NullFile {
    fn read     (&mut self               ) -> u8 {0xFF}
    fn write    (&mut self,     item : u8) {}
}

pub fn open_file(path : &str) -> Option<FileDescriptor> {
    if path == "/dev/null" {
        Some(FileDescriptor(0))
    } else {
        None
    }
}

pub fn get_file_from_handle(handle : FileDescriptor) -> Option<NullFile> {
    if handle.0 != 0 {
        return None;
    } else {
        return Some(NullFile);
    }
}

pub fn get_file(file : File) -> Option<NullFile> {
    get_file_from_handle(file.handle)
}

pub fn raw_handle(file : File) -> u32 {
    file.handle.0
}

pub fn raw_flags(file : File) -> u8 {
    file.flags.0
}

pub fn name(file : File) -> &'static String {
    file.name
}

