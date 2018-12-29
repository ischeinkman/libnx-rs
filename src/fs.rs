use super::libnx::{
    FsFile, FsFileSystem, 
    fsFsOpenFile, fsFileGetSize, fsFileClose, fsFsDeleteFile,
    fsFileRead, fsFileWrite, 
    fsFileFlush, 
    fsdevGetDefaultFileSystem,
    fsdevGetDeviceFileSystem,
    lang_items
};
use super::error::{LibnxError };
use std::ffi::{CString};

pub struct File {
    inner : FsFile
}

impl File {
    pub fn size(&self) -> Result<usize, LibnxError> {
        let mut rval : u64 = 0;
        let res = unsafe { fsFileGetSize(&self.inner as *const FsFile as *mut FsFile, &mut rval as *mut u64)};
        if res != 0 {
            Err(LibnxError::from_raw(res))
        }
        else { 
            Ok(rval as usize)
        }
    }

    pub fn read(&mut self, offset : usize, buffer : &mut [u8]) -> Result<usize, LibnxError> {
        let mut rval : usize = 0; 
        let res = unsafe {
            fsFileRead(&mut self.inner as *mut FsFile, offset as u64, buffer.as_mut_ptr() as *mut lang_items::c_void, buffer.len(), &mut rval as *mut usize)
        };
        if res != 0 {
            Err(LibnxError::from_raw(res))
        }
        else {
            Ok(rval as usize)
        }
    }
    
    pub fn flush(&mut self) -> Result<(), LibnxError> {
        let res = unsafe {fsFileFlush(&mut self.inner as *mut FsFile)};
        if res != 0 {
            Err(LibnxError::from_raw(res))
        }
        else {
            Ok(())
        }

    }

}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { fsFileClose(&mut self.inner as *mut FsFile) }
    }
}

pub struct FileSystem {
    inner : *mut FsFileSystem
}

impl FileSystem {
    pub fn from_name(name : &str) -> Result<FileSystem, LibnxError> {
        unsafe {
            let c_str = CString::new(name).map_err(|e| LibnxError::from_msg(format!("CString create err: {:?}", e)))?;
            let inner : *mut  FsFileSystem = fsdevGetDeviceFileSystem(c_str.as_ptr() as *const u8);
            Ok(FileSystem {
                inner
            })
        }
    }

    pub fn open_file(&self, path : &str, flags : i32) -> Result<File, LibnxError> {
        let c_str = CString::new(path).map_err(|e| LibnxError::from_msg(format!("CString create err: {:?}", e)))?;
        let mut rval : File = unsafe {std::mem::zeroed()};
        unsafe {
            let res = fsFsOpenFile(self.inner, c_str.as_ptr() as *const u8, flags, &mut rval.inner as *mut FsFile);
            if res != 0 {
                Err(LibnxError::from_raw(res))
            }
            else {
                Ok(rval)
            }
        }
    }

}

impl Default for FileSystem {
    fn default() -> FileSystem {
        FileSystem {
            inner : unsafe { fsdevGetDefaultFileSystem() }
        }
    }
}