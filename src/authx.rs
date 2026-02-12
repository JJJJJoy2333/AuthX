extern crate libc;
use libc::{c_char};
// use AUTHX_RET::AUTHX_OK;
// use AUTHX_RET::AUTHX_ERROR;
// use AUTHX_RET::AUTHX_INVALID_ARGUMENT;
// use AUTHX_RET::AUTHX_INVALID_STATE;
// use AUTHX_RET::AUTHX_TIMEOUT;
// use AUTHX_RET::AUTHX_CANCELLED;
// use AUTHX_RET::AUTHX_INTERNAL_ERROR;
// use AUTHX_RET::AUTHX_NETWORK_ERROR;
// use AUTHX_RET::AUTHX_SERVICE_UNAVAILABLE;
// use AUTHX_RET::AUTHX_SERVER_ERROR;
// use AUTHX_MODULE::AUTHX_MODULE_A;
// use AUTHX_MODULE::AUTHX_MODULE_B;
// use AUTHX_MODULE::AUTHX_MODULE_C;
// use AUTHX_MODULE::AUTHX_MODULE_D;
// use AUTHX_MODULE::AUTHX_MODULE_E;
// use AUTHX_VERSION::AuthxVersion;
// use AUTHX_DEVICE_ID::AuthxDeviceId;
// use AUTHX_TRACE_ID::AuthxTraceId;

use std::ffi::CString;
// use std::ptr;

#[repr(C)]
pub enum AUTHX_RET {
    AuthxOk = 0,
    AuthxError = -1,
    AuthxInvalidArgument = -2,
    AuthxInvalidState = -3,
    AuthxTimeout = -4,
    AuthxCancelled = -5,
    AuthxInternalError = -6,
    AuthxNetworkError = -7,
    AuthxServiceUnavailable = -8,
    AuthxServerError = -9
}

#[repr(C)]
pub enum AUTHX_MODULE {
    AuthxModuleA,
    AuthxModuleB,
    AuthxModuleC,
    AuthxModuleD,
    AuthxModuleE
}

type AuthxVersion = *const c_char;
type AuthxDeviceId = *const c_char;
type AuthxTraceId = *const c_char;

#[repr(C)]
pub struct AUTHX_DATA {
    pub module: AUTHX_MODULE,
    pub version: AuthxVersion,
    pub module_version: AuthxVersion,
    pub device_id: AuthxDeviceId,
    pub trace_id: AuthxTraceId,
    pub all_enabled_module: bool,
    pub report: bool,
}

// Implement authx_get_device_id
#[no_mangle]
pub extern "C" fn authx_get_device_id(device_id: *mut *const c_char) -> AUTHX_RET {
    let device_id_str = CString::new("generated-device-id").unwrap();
    unsafe { *device_id = device_id_str.as_ptr() as *const c_char; }
    AUTHX_RET::AuthxOk
}

// Implement authx_get_trace_id
#[no_mangle]
pub extern "C" fn authx_get_trace_id(trace_id: *mut *const c_char) -> AUTHX_RET {
    let trace_id_str = CString::new("dummy-trace-id").unwrap();
    unsafe { *trace_id = trace_id_str.as_ptr() as *const c_char; }
    AUTHX_RET::AuthxOk
}

// Implement authx_init
#[no_mangle]
pub extern "C" fn authx_init(data: *mut AUTHX_DATA) -> AUTHX_RET {
    // Set default values
    unsafe {
        *data = AUTHX_DATA {
            module: AUTHX_MODULE::AuthxModuleA,
            version: std::ptr::null(),
            module_version: std::ptr::null(),
            device_id: std::ptr::null(),
            trace_id: std::ptr::null(),
            all_enabled_module: true,
            report: false,
        };
    }
    AUTHX_RET::AuthxOk
}

// Implement authx_get_version
#[no_mangle]
pub extern "C" fn authx_get_version(version: *mut *const c_char) -> AUTHX_RET {
    let version_str = CString::new("1.0.0").unwrap();
    unsafe { *version = version_str.as_ptr() as *const c_char; }
    AUTHX_RET::AuthxOk

}

// Implement authx_shutdown
#[no_mangle]
pub extern "C" fn authx_shutdown() -> AUTHX_RET {
    AUTHX_RET::AuthxOk
}