pub mod streams;

#[derive(Debug, Clone, Copy)]
pub struct File {
    handle  :   usize,
    pos     :   usize,
    size    :   usize
}

impl File {
    pub fn new(handle : usize, size : usize) -> File {
        File {handle : handle, size : size, pos : 0}
    }
}

pub trait FileInteractor {
    fn set_pos      (pos  : usize, file : File  )           ;
    fn is_eof       (file : File                ) -> bool   ;
    fn get_pos      (file : File                ) -> usize  ;
    fn close        (file : File                )           ;
    fn open         (path : &str                ) -> File   ;
    fn get_handle   (file : File                ) -> usize  ;

    fn at_end       (&self                      ) -> bool   ;
}

pub trait FileReader<T> : FileInteractor {
    fn open(file : &File) -> Self     ;
    fn read(&mut self)    -> Option<T>;
}

pub trait FileWriter<T> : FileInteractor {
    fn open (file : &File               ) -> Self;
    fn write(&mut self, data : T        )        ;
    fn flush(&mut self                  )        ;
}



//Primitive Interactors
pub struct ByteWriter (File);
pub struct ShortWriter(File);
pub struct IntWriter  (File);
pub struct LongWriter (File);
pub struct LLongWriter(File);

pub struct ByteReader (File);
pub struct ShortReader(File);
pub struct IntReader  (File);
pub struct LongReader (File);
pub struct LLongReader(File);



pub fn open(path : &str) -> File {
    File {handle : 0, pos : 0, size : 1}
}

impl FileInteractor for ByteReader {
    fn set_pos(pos : usize,mut file : File) {
        file.pos = pos;
    }

    fn get_pos(file : File) -> usize {
        file.pos
    }

    fn is_eof(file : File) -> bool {
        file.pos >= file.size
    }

    fn get_handle(file : File) -> usize {
        file.handle
    }

    fn open(_path : &str) -> File {
        File {handle : 1, pos : 0, size : 0}
    }

    fn close(_file : File) {}

    fn at_end(&self) -> bool {
        self.0.pos >= self.0.size
    }
        
}

impl FileInteractor for ByteWriter {
    fn set_pos(pos : usize,mut file : File) {
        file.pos = pos;
    }

    fn get_pos(file : File) -> usize {
        file.pos
    }

    fn is_eof(file : File) -> bool {
        file.pos >= file.size
    }

    fn get_handle(file : File) -> usize {
        file.handle
    }

    fn open(_path : &str) -> File {
        File {handle : 1, pos : 0, size : 0}
    }

    fn close(_file : File) {}

    fn at_end(&self) -> bool {
        self.0.pos >= self.0.size
    }
        
}



impl FileReader<u8> for ByteReader {
    fn open(file : &File) -> ByteReader {
        ByteReader(*file)
    }

    fn read(&mut self) -> Option<u8> {
        if self.0.handle == 0 {
            Some(0)
        } else {
            None
        }
    }
}

impl FileWriter<u8> for ByteWriter {
    fn open(file : &File) -> ByteWriter {
        ByteWriter(*file)
    }

    fn write(&mut self, data : u8) {
        
    }

    fn flush(&mut self) {
        
    }
}