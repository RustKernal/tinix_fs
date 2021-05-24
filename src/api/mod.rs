pub struct File {
    handle  :   usize,
    pos     :   usize,
    size    :   usize
}

pub trait FileInteractor {
    fn set_pos      (pos  : usize, file : File)           ;
    fn is_eof       (file : File              ) -> bool   ;
    fn get_pos      (file : File              ) -> usize  ;
    fn close        (file : File              )           ;
    fn open         (path : &str              ) -> File   ;
    fn get_handle   (file : File              ) -> usize  ;
}

pub trait FileReader<T> : FileInteractor {
    fn open(file : File) -> Self;
    fn read(file : File) -> T   ;
}

pub trait FileWriter<T> : FileInteractor {
    fn open (file : File          ) -> Self;
    fn write(data : T, file : File)        ;
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

    fn open(path : &str) -> File {
        File {handle : 0, pos : 0, size : 0}
    }

    fn close(file : File) {}
        
}