#![allow(dead_code, non_camel_case_types)]
extern crate libc;

use std::ptr;

use std::ffi::CStr;
use std::ffi::CString;

type size_t = libc::size_t;
type wchar  = libc::wchar_t;
type c_char = libc::c_char;

pub struct CWcharString {
    pub data: Vec<wchar>,
    pub n_chars: usize,
}

impl CWcharString {
    pub unsafe fn from_c_string(c_string: &CString) -> CWcharString {
        let c_string_ptr =
            (&c_string.as_bytes_with_nul()[0] as *const u8) as *const c_char;

        let size_needed = mbstowcs(ptr::null_mut(), c_string_ptr, 0) + 1;

        let mut data = vec![0 as wchar; size_needed as usize];
        let wchar_ptr = &mut data.as_mut_slice()[0] as *mut wchar;

        let n_chars = mbstowcs(wchar_ptr, c_string_ptr, size_needed);

        CWcharString {
            data: data,
            n_chars: n_chars as usize,
        }
    }

    pub unsafe fn from_str(string: &str) -> CWcharString {
        let c_string = CString::new(string).expect("Invalid C string");
        CWcharString::from_c_string(&c_string)
    }

    pub unsafe fn as_raw(&self) -> *const wchar {
        &self.data[0] as *const wchar
    }

    pub unsafe fn from_raw_to_c_string(raw: *const wchar) -> CString {
        let n_bytes = wcstombs(ptr::null_mut(), raw, 0);

        let mut data = vec![0 as u8; (n_bytes + 1) as usize];
        let data_ptr = (&mut data[0] as *mut u8) as *mut c_char;

        wcstombs(data_ptr, raw, n_bytes + 1);
        let c_str = CStr::from_bytes_with_nul(data.as_slice()).expect("Invalid c str");
        c_str.to_owned()
    }
}

extern "C" {
    fn mbstowcs(__pwcs: *mut wchar, __s: *const c_char, __n: size_t) -> size_t;
    fn wcstombs(__s: *mut c_char, __pwcs: *const wchar, __n: size_t) -> size_t;
}