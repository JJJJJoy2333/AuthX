use std::ffi::CStr;
use std::os::raw::c_char;

// We declare the FFI functions locally for testing.
// We need to tell the linker to link against our local 'authx' library.
#[link(name = "authx")]
extern "C" {
    fn authx_get_version(version: *mut *const c_char) -> i32;
    #[allow(dead_code)]
    fn authx_init(data: *mut std::ffi::c_void) -> i32;
}

#[test]
fn test_get_version_ffi() {
    let mut version_ptr: *const c_char = std::ptr::null();
    let ret = unsafe { authx_get_version(&mut version_ptr) };
    
    assert_eq!(ret, 0, "authx_get_version should return AUTHX_OK (0)");
    assert!(!version_ptr.is_null(), "Version pointer should not be null");
    
    let version_cstr = unsafe { CStr::from_ptr(version_ptr) };
    let version_str = version_cstr.to_str().expect("Version should be valid UTF-8");
    
    println!("Retrieved Version via FFI: {}", version_str);
    
    // Basic structural checks
    assert!(version_str.contains(" ("), "Version string should contain build info");
    assert!(version_str.contains(")"), "Version string should contain build info");
}
