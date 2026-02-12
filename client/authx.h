
#ifndef __AUTHX_H__
#define __AUTHX_H__

#ifdef __cplusplus
#define AUTHX_EXTERN_C extern "C"
#else
#define AUTHX_EXTERN_C extern
#endif

#ifdef _WIN32
#define AUTHX_API AUTHX_EXTERN_C __declspec(dllexport)
#define AUTHX_DEPRECATED __declspec(deprecated)
#else
#define AUTHX_API AUTHX_EXTERN_C __attribute__((visibility("default")))
#define AUTHX_DEPRECATED __attribute__((deprecated))
#endif

typedef enum {
  AUTHX_OK = 0,
  AUTHX_ERROR = -1,
  AUTHX_INVALID_ARGUMENT = -2,
  AUTHX_INVALID_STATE = -3,
  AUTHX_TIMEOUT = -4,
  AUTHX_CANCELLED = -5,
  AUTHX_INTERNAL_ERROR = -6,
  AUTHX_NETWORK_ERROR = -7,
  AUTHX_SERVICE_UNAVAILABLE = -8,
  AUTHX_SERVER_ERROR = -9
} AUTHX_RET;

typedef enum {
  AUTHX_MODULE_A,
  AUTHX_MODULE_B,
  AUTHX_MODULE_C,
  AUTHX_MODULE_D,
  AUTHX_MODULE_E
} AUTHX_MODULE;

typedef char* AUTHX_VERSION;
typedef char* AUTHX_DEVICE_ID;
typedef char* AUTHX_TRACE_ID;

typedef struct {
  AUTHX_MODULE module;
  const AUTHX_VERSION version;
  const AUTHX_VERSION module_version;
  const AUTHX_DEVICE_ID device_id;
  const AUTHX_TRACE_ID trace_id;
  bool all_enabled_module;
  bool report;
} AUTHX_DATA;


AUTHX_API AUTHX_RET authx_get_device_id(AUTHX_DEVICE_ID* device_id);
AUTHX_API AUTHX_RET authx_get_trace_id(AUTHX_TRACE_ID* trace_id);

AUTHX_API AUTHX_RET authx_init(AUTHX_DATA* data);
AUTHX_API AUTHX_RET authx_get_version(AUTHX_VERSION* version);
AUTHX_API AUTHX_RET authx_shutdown();


#endif // __AUTHX_H__