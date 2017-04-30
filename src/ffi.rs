#![allow(dead_code, non_camel_case_types)]
extern crate libc;

use ::c_w_string::CWcharString;

use std::ffi::CString;

type uint64 = libc::uint64_t;
type uint8  = libc::uint8_t;
type size_t = libc::size_t;
type wchar  = libc::wchar_t;
type c_char = libc::c_char;
type c_int  = libc::c_int;
type void   = libc::c_void;

type c_MediaInfoStream = libc::c_int;
type c_MediaInfoInfo   = libc::c_int;

const LC_CTYPE: c_int = 0;

pub enum MediaInfoStream {
    General = 0,
    Video,
    Audio,
    Text,
    Other,
    Image,
    Menu,
    Max,
}

impl MediaInfoStream {
    fn c_compatible(self) -> c_MediaInfoStream {
        self as libc::c_int
    }
}

pub enum MediaInfoInfo {
    Name = 0,
    Text,
    Measure,
    Options,
    Name_Text,
    Measure_Text,
    Info,
    HowTo,
    Max,
}

impl MediaInfoInfo {
    fn c_compatible(self) -> c_MediaInfoInfo {
        self as libc::c_int
    }
}

pub struct MediaInfo {
    handle: *mut void,
}

impl MediaInfo {
    pub fn new() -> MediaInfo {
        unsafe {
            // NOTE(erick): Setting the locale so we can
            // work properly with c wide strings.
            let empty_c_str = CString::new("").unwrap();
            setlocale(LC_CTYPE, empty_c_str.as_ptr());
            MediaInfo {
                handle : MediaInfo_New(),
            }
        }
    }

    // NOTE(erick): We could receive a Path instead of a &str.
    pub fn open(&mut self, path: &str) -> usize {
        unsafe {
            let path_w_string = CWcharString::from_str(path);
            let path_ptr = path_w_string.as_raw();

            let result = MediaInfo_Open(self.handle, path_ptr);

            result as usize
        }
    }

    pub fn close(&mut self) {
        unsafe {
            MediaInfo_Close(self.handle);
        }
    }

    pub fn open_buffer_init(&mut self, buffer_size: u64, offset: u64) -> usize {
        unsafe { MediaInfo_Open_Buffer_Init(self.handle, buffer_size, offset) as usize }
    }

    pub fn open_buffer_continue(&mut self, data: &[u8]) -> usize {
        unsafe {
            let bytes_ptr = &data[0] as *const uint8;
            let result = MediaInfo_Open_Buffer_Continue(self.handle,
                                                        bytes_ptr,
                                                        data.len() as uint64);
            result as usize
        }
    }

    pub fn open_buffer_finalize(&mut self) -> usize {
        unsafe { MediaInfo_Open_Buffer_Finalize(self.handle) as usize }
    }

    pub fn option(&mut self, parameter: &str, value: &str) -> String {
        unsafe {
            let param_w_string = CWcharString::from_str(parameter);
            let value_w_string = CWcharString::from_str(value);

            let param_ptr = param_w_string.as_raw();
            let value_ptr = value_w_string.as_raw();

            // TODO(erick): Do we need to free this memory? I could not
            // find this information on the documentation.
            let result_ptr = MediaInfo_Option(self.handle, param_ptr, value_ptr);
            let result_c_string = CWcharString::from_raw_to_c_string(result_ptr);

            result_c_string.into_string().expect("Could not convert c string")
        }
    }

    pub fn inform(&mut self, reserved: usize) -> String {
        unsafe {
            // TODO(erick): Do we need to free this memory? I could not
            // find this information on the documentation.
            let result_ptr = MediaInfo_Inform(self.handle, reserved as size_t);
            let result_c_string = CWcharString::from_raw_to_c_string(result_ptr);

            result_c_string.into_string().expect("Could not convert c string")
        }
    }

    pub fn get(&mut self, info_stream: MediaInfoStream,
               stream_number: usize, parameter: &str,
               info_kind: MediaInfoInfo, search_kind: MediaInfoInfo) -> String {
        unsafe {
            let param_w_string = CWcharString::from_str(parameter);
            let param_ptr = param_w_string.as_raw();

            // TODO(erick): Do we need to free this memory? I could not
            // find this information on the documentation.
            let result_ptr = MediaInfo_Get(self.handle, info_stream.c_compatible(),
                                           stream_number as size_t, param_ptr,
                                           info_kind.c_compatible(),
                                           search_kind.c_compatible());
            let result_c_string = CWcharString::from_raw_to_c_string(result_ptr);

            result_c_string.into_string().expect("Could not convert c string")
        }
    }
}

impl Drop for MediaInfo {
    fn drop(&mut self) {
        unsafe {
            MediaInfo_Delete(self.handle);
        }
    }
}

#[link(name="mediainfo")]
extern "C" {
    fn MediaInfo_New() -> *mut void;
    fn MediaInfo_Delete(handle: *mut void);

    fn MediaInfo_Open_Buffer_Init(handle: *mut void,
                                  buffer_size: uint64,
                                  offset: uint64) -> size_t;
    fn MediaInfo_Open_Buffer_Continue(handle: *mut void,
                                      bytes: *const uint8,
                                      length: size_t) -> size_t;
    fn MediaInfo_Open_Buffer_Finalize(handle: *mut void) -> size_t;

    fn MediaInfo_Open(handle: *mut void, path: *const wchar) -> size_t;
    fn MediaInfo_Close(handle: *mut void);

    fn MediaInfo_Option(handle: *mut void,
                        parameter: *const wchar,
                        value: *const wchar) -> *const wchar;
    fn MediaInfo_Inform(handle: *mut void, reserved: size_t) -> *const wchar;

    fn MediaInfo_Get(handle: *mut void, info_stream: c_MediaInfoStream,
                     stream_number: size_t, parameter: *const wchar,
                     info_kind: c_MediaInfoInfo, search_kind: c_MediaInfoInfo)
                     -> *const wchar;

    fn setlocale(category: c_int, locale: *const c_char) -> *const c_char;
}