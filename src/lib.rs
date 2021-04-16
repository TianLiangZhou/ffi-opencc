use std::os::raw::c_char;
use opencc_rust::{OpenCC, DefaultConfig};
use std::ffi::{CStr, CString, c_void};
use std::path::Path;
use std::sync::RwLock;
use std::mem;


pub struct BoxOpenCC {
    s2t: OpenCC,
    t2s: OpenCC,
    s2tw: OpenCC,
    tw2s: OpenCC,
    s2hk: OpenCC,
    hk2s: OpenCC,
}

#[no_mangle]
pub extern "C" fn with_dict_path(str: *const c_char) -> *mut BoxOpenCC {
    let s = char2str(str);
    let p = Path::new(s);
    let box_opencc = BoxOpenCC{
        s2t: OpenCC::new(p.join(DefaultConfig::S2T)).unwrap(),
        t2s: OpenCC::new(p.join(DefaultConfig::T2S)).unwrap(),
        s2tw: OpenCC::new(p.join(DefaultConfig::S2TW)).unwrap(),
        tw2s: OpenCC::new(p.join(DefaultConfig::TW2S)).unwrap(),
        s2hk: OpenCC::new(p.join(DefaultConfig::S2HK)).unwrap(),
        hk2s: OpenCC::new(p.join(DefaultConfig::HK2S)).unwrap(),
    };
    return Box::into_raw(Box::new(box_opencc)) as *mut BoxOpenCC
}

#[no_mangle]
pub unsafe extern "C" fn s2t(str: *const c_char, oc: *mut BoxOpenCC) -> *mut c_char {
    let s = char2str(str);
    let string = (*oc).s2t.convert(s);
    return CString::new(string).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn t2s(str: *const c_char, oc: *mut BoxOpenCC) -> *mut c_char {
    let s = char2str(str);
    let string = (*oc).t2s.convert(s);
    return CString::new(string).unwrap().into_raw()
}
#[no_mangle]
pub unsafe extern "C" fn s2tw(str: *const c_char, oc: *mut BoxOpenCC) -> *mut c_char {
    let s = char2str(str);
    let string = (*oc).s2tw.convert(s);
    return CString::new(string).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn tw2s(str: *const c_char, oc: *mut BoxOpenCC) -> *mut c_char {
    let s = char2str(str);
    let string = (*oc).tw2s.convert(s);
    return CString::new(string).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn s2hk(str: *const c_char, oc: *mut BoxOpenCC) -> *mut c_char {
    let s = char2str(str);
    let string = (*oc).s2hk.convert(s);
    return CString::new(string).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn hk2s(str: *const c_char, oc: *mut BoxOpenCC) -> *mut c_char {
    let s = char2str(str);
    let string = (*oc).hk2s.convert(s);
    return CString::new(string).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn free_opencc(oc: *mut BoxOpenCC) {
    if !oc.is_null() {
        unsafe { Box::from_raw(oc); }
    }
}

//
#[no_mangle]
pub extern "C" fn free_pointer(ptr: *mut c_char) {
    unsafe {
        if ptr.is_null() {
            // No data there, already freed probably.
            return;
        }
        // Here we reclaim ownership of the data the pointer points to, to free the memory properly.
        CString::from_raw(ptr);
    }
}

fn char2str(str: *const c_char) -> &'static str {
    return unsafe { CStr::from_ptr(str) }.to_str().unwrap();
}



#[cfg(test)]
mod tests {
    use std::ffi::{CString, CStr};
    use std::os::raw::c_char;

    #[test]
    fn convert() {
        assert_eq!("我", "我");
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}