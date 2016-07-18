#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate libc;
use libc::{c_void, c_ulong, c_char, c_long, c_uchar, c_ushort, c_short};


#[repr(C)]
pub struct SCardIORequest(c_ulong, c_ulong);

#[repr(C)]
pub struct SCARD_READERSTATE {
    pub szReader: *const c_char,
    pub pvUserData: *mut c_void,
    pub dwCurrentState: DWORD,
    pub dwEventState: DWORD,
    pub cbAtr: DWORD,
    pub rgbAtr: [c_uchar; MAX_ATR_SIZE],
}

pub type LPSCARD_READERSTATE = *mut SCARD_READERSTATE;

#[repr(C)]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: c_ulong,
    pub cbPciLength: c_ulong,
}

pub type PSCARD_IO_REQUEST = *mut SCARD_IO_REQUEST;
pub type LPSCARD_IO_REQUEST = *mut SCARD_IO_REQUEST;
pub type LPCSCARD_IO_REQUEST = *const SCARD_IO_REQUEST;

#[repr(C)]
pub struct DEVICE_CAPABILITIES {
    Vendor_Name: LPSTR,
    IFD_Type: LPSTR,
    IFD_Version: DWORD,
    IFD_Serial: LPSTR,
    IDF_Channel_ID: DWORD,
    Asynch_Supported: DWORD,
    Default_Clock: DWORD,
    Max_Clock: DWORD,
    Default_Data_Rate: DWORD,
    Max_Data_Rate: DWORD,
    Max_IFSD: DWORD,
    Synch_Supported: DWORD,
    Power_Mgmt: DWORD,
    Card_Auth_Devices: DWORD,
    User_Auth_Device: DWORD,
    Mechanics_Supported: DWORD,
    Vendor_Features: DWORD,
}

pub type PDEVICE_CAPABILITIES = *mut DEVICE_CAPABILITIES;

#[repr(C)]
pub struct ICC_STATE {
    ICC_Presence: UCHAR,
    ICC_Interface_Status: UCHAR,
    ATR: [UCHAR; MAX_ATR_SIZE],
    ICC_Type: UCHAR,
}

pub type PICC_STATE = *mut ICC_STATE;

#[repr(C)]
pub struct PROTOCOL_OPTIONS {
    Protocol_Type: DWORD,
    Current_Clock: DWORD,
    Current_F: DWORD,
    Current_D: DWORD,
    Current_N: DWORD,
    Current_W: DWORD,
    Current_IFSC: DWORD,
    Current_IFSD: DWORD,
    Current_BWT: DWORD,
    Current_CWT: DWORD,
    Current_EBC: DWORD,
}

pub type PPROTOCOL_OPTIONS = *mut PROTOCOL_OPTIONS;

#[repr(C)]
pub struct SCARD_IO_HEADER {
    Protocol: DWORD,
    Length: DWORD,
}

pub type PSCARD_IO_HEADER = *mut SCARD_IO_HEADER;

#[cfg(target_os="macos")]
pub type BYTE = u8;
#[cfg(target_os="macos")]
pub type UCHAR = u8;
#[cfg(target_os="macos")]
pub type USHORT = u16;
#[cfg(target_os="macos")]
pub type ULONG = u32;
#[cfg(target_os="macos")]
pub type BOOL = i16;
#[cfg(target_os="macos")]
pub type WORD = u16;
#[cfg(target_os="macos")]
pub type DWORD = u32;
#[cfg(target_os="macos")]
pub type LONG = i32;
#[cfg(not(target_os="macos"))]
pub type BYTE = c_uchar;
#[cfg(not(target_os="macos"))]
pub type UCHAR = c_uchar;
#[cfg(not(target_os="macos"))]
pub type USHORT = c_ushort;
#[cfg(not(target_os="macos"))]
pub type ULONG = c_ulong;
#[cfg(not(target_os="macos"))]
pub type BOOL = c_short;
#[cfg(not(target_os="macos"))]
pub type WORD = c_ushort;
#[cfg(not(target_os="macos"))]
pub type DWORD = c_ulong;
#[cfg(not(target_os="macos"))]
pub type LONG = c_long;

pub type PUCHAR = *mut UCHAR;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type PULONG = *mut ULONG;
pub type PDWORD = *mut DWORD;
pub type LPCSTR = *const c_char;
pub type LPCBYTE = *const BYTE;
pub type LPBYTE = *mut BYTE;
pub type LPDWORD = *mut DWORD;
pub type LPSTR = *mut c_char;

pub type SCARDCONTEXT = LONG;
pub type PSCARDCONTEXT = *mut SCARDCONTEXT;
pub type LPSCARDCONTEXT = *mut SCARDCONTEXT;
pub type SCARDHANDLE = LONG;
pub type PSCARDHANDLE = *mut SCARDHANDLE;
pub type LPSCARDHANDLE = *mut SCARDHANDLE;

pub type RESPONSECODE = c_long;

pub const MAX_ATR_SIZE: usize = 33;

pub const SCARD_S_SUCCESS: LONG                 = 0x00000000;
pub const SCARD_F_INTERNAL_ERROR: LONG          = 0x80100001;
pub const SCARD_E_CANCELLED: LONG               = 0x80100002;
pub const SCARD_E_INVALID_HANDLE: LONG          = 0x80100003;
pub const SCARD_E_INVALID_PARAMETER: LONG       = 0x80100004;
pub const SCARD_E_INVALID_TARGET: LONG          = 0x80100005;
pub const SCARD_E_NO_MEMORY: LONG               = 0x80100006;
pub const SCARD_F_WAITED_TOO_LONG: LONG         = 0x80100007;
pub const SCARE_E_INSUFFICIENT_BUFFER: LONG     = 0x80100008;
pub const SCARD_E_UNKNOWN_READER: LONG          = 0x80100009;
pub const SCARD_E_TIMEOUT: LONG                 = 0x8010000a;
pub const SCARD_E_SHARING_VIOLATION: LONG       = 0x8010000b;
pub const SCARD_E_NO_SMARTCARD: LONG            = 0x8010000c;
pub const SCARD_E_UNKNOWN_CARD: LONG            = 0x8010000d;
pub const SCARD_E_CANT_DISPOSE: LONG            = 0x8010000e;
pub const SCARD_E_PROTO_MISMATCH: LONG          = 0x8010000f;
pub const SCARD_E_NOT_READY: LONG               = 0x80100010;
pub const SCARD_E_INVALID_VALUE: LONG           = 0x80100011;
pub const SCARD_E_SYSTEM_CANCELLED: LONG        = 0x80100012;
pub const SCARD_F_COMM_ERROR: LONG              = 0x80100013;
pub const SCARD_F_UNKNOWN_ERROR: LONG           = 0x80100014;
pub const SCARD_E_INVALID_ATR: LONG             = 0x80100015;
pub const SCARD_E_NOT_TRANSACTED: LONG          = 0x80100016;
pub const SCARD_E_READER_UNAVAILABLE: LONG      = 0x80100017;
pub const SCARD_P_SHUTDOWN: LONG                = 0x80100018;
pub const SCARD_E_PCI_TOO_SMALL: LONG           = 0x80100019;
pub const SCARD_E_READER_UNSUPPORTED: LONG      = 0x8010001a;
pub const SCARD_E_DUPLICATE_READER: LONG        = 0x8010001b;
pub const SCARD_E_CARD_UNSUPPORTED: LONG        = 0x8010001c;
pub const SCARD_E_NO_SERVICE: LONG              = 0x8010001d;
pub const SCARD_E_SERVICE_STOPPED: LONG         = 0x8010001e;
pub const SCARD_E_UNEXPECTED: LONG              = 0x8010001f;
pub const SCARD_E_UNSUPPORTED_FEATURE: LONG     = 0x8010001f;
pub const SCARD_E_ICC_INSTALLATION: LONG        = 0x80100020;
pub const SCARD_E_ICC_CREATEORDER: LONG         = 0x80100021;
pub const SCARD_E_DIR_NOT_FOUND: LONG           = 0x80100023;
pub const SCARD_E_FILE_NOT_FOUND: LONG          = 0x80100024;
pub const SCARD_E_NO_DIR: LONG                  = 0x80100025;
pub const SCARD_E_NO_FILE: LONG                 = 0x80100026;
pub const SCARD_E_NO_ACCESS: LONG               = 0x80100027;
pub const SCARD_E_WRITE_TOO_MANY: LONG          = 0x80100028;
pub const SCARD_E_BAD_SEEK: LONG                = 0x80100029;
pub const SCARD_E_INVALID_CHV: LONG             = 0x8010002a;
pub const SCARD_E_UNKNOWN_RES_MNG: LONG         = 0x8010002b;
pub const SCARD_E_NO_SUCH_CERTIFICATE: LONG     = 0x8010002c;
pub const SCARD_E_CERTIFICATE_UNAVAILABLE: LONG = 0x8010002d;
pub const SCARD_E_NO_READERS_AVAILABLE: LONG    = 0x8010002e;
pub const SCARD_E_COMM_DATA_LOST: LONG          = 0x8010002f;
pub const SCARD_E_NO_KEY_CONTAINER: LONG        = 0x80100030;
pub const SCARD_E_SERVER_TOO_BUSY: LONG         = 0x80100031;
pub const SCARD_W_UNSUPPORTED_CARD: LONG        = 0x80100065;
pub const SCARD_W_UNRESPONSIVE_CARD: LONG       = 0x80100066;
pub const SCARD_W_UNPOWERED_CARD: LONG          = 0x80100067;
pub const SCARD_W_RESET_CARD: LONG              = 0x80100068;
pub const SCARD_W_REMOVED_CARD: LONG            = 0x80100069;
pub const SCARD_W_SECURITY_VIOLATION: LONG      = 0x8010006a;
pub const SCARD_W_WRONG_CHV: LONG               = 0x8010006b;
pub const SCARD_W_CHV_BLOCKED: LONG             = 0x8010006c;
pub const SCARD_W_EOF: LONG                     = 0x8010006d;
pub const SCARD_W_CANCELLED_BY_USER: LONG       = 0x8010006e;
pub const SCARD_W_CARD_NOT_AUTHENTICATED: LONG  = 0x8010006f;

pub const SCARD_AUTOALLOCATE: DWORD = !(0 as DWORD);

pub const SCARD_SCOPE_USER: DWORD     = 0x0000;
pub const SCARD_SCOPE_TERMINAL: DWORD = 0x0001;
pub const SCARD_SCOPE_SYSTEM: DWORD   = 0x0002;
pub const SCARD_SCOPE_GLOBAL: DWORD   = 0x0003;

pub const SCARD_PROTOCOL_UNDEFINED: DWORD = 0x0000;
pub const SCARD_PROTOCOL_T0: DWORD        = 0x0001;
pub const SCARD_PROTOCOL_T1: DWORD        = 0x0002;
pub const SCARD_PROTOCOL_RAW: DWORD       = 0x0004;
pub const SCARD_PROTOCOL_T15: DWORD       = 0x0008;

pub const SCARD_PROTOCOL_ANY: DWORD = SCARD_PROTOCOL_T0 | SCARD_PROTOCOL_T1;

pub const SCARD_SHARE_EXCLUSIVE: DWORD = 0x0001;
pub const SCARD_SHARE_SHARED: DWORD    = 0x0002;
pub const SCARD_SHARE_DIRECT: DWORD    = 0x0003;

pub const SCARD_LEAVE_CARD: DWORD   = 0x0000;
pub const SCARD_RESET_CARD: DWORD   = 0x0001;
pub const SCARD_UNPOWER_CARD: DWORD = 0x0002;
pub const SCARD_EJECT_CARD: DWORD   = 0x0003;

pub const SCARD_UNKNOWN: DWORD    = 0x0001;
pub const SCARD_ABSENT: DWORD     = 0x0002;
pub const SCARD_PRESENT: DWORD    = 0x0004;
pub const SCARD_SWALLOWED: DWORD  = 0x0008;
pub const SCARD_POWERED: DWORD    = 0x0010;
pub const SCARD_NEGOTIABLE: DWORD = 0x0020;
pub const SCARD_SPECIFIC: DWORD   = 0x0040;

pub const SCARD_STATE_UNAWARE: DWORD     = 0x0000;
pub const SCARD_STATE_IGNORE: DWORD      = 0x0001;
pub const SCARD_STATE_CHANGED: DWORD     = 0x0002;
pub const SCARD_STATE_UNKNOWN: DWORD     = 0x0004;
pub const SCARD_STATE_UNAVAILABLE: DWORD = 0x0008;
pub const SCARD_STATE_EMPTY: DWORD       = 0x0010;
pub const SCARD_STATE_PRESENT: DWORD     = 0x0020;
pub const SCARD_STATE_ATRMATCH: DWORD    = 0x0040;
pub const SCARD_STATE_EXCLUSIVE: DWORD   = 0x0080;
pub const SCARD_STATE_INUSE: DWORD       = 0x0100;
pub const SCARD_STATE_MUTE: DWORD        = 0x0200;
pub const SCARD_STATE_UNPOWERED: DWORD   = 0x0400;

pub const INFINITE: DWORD = 0xffffffff;

pub const PCSCLITE_MAX_READERS_CONTEXTS: usize = 16;
pub const MAX_READERNAME: usize = 128;
pub const SCARD_ATR_LENGTH: usize = MAX_ATR_SIZE;

pub const MAX_BUFFER_SIZE: usize = 264;
pub const MAX_BUFFER_SIZE_EXTENDED: usize = (4 + 3 + (1<<16) + 3 + 2);

pub const TAG_IFD_ATR: DWORD                         = 0x0303;
pub const TAG_IFD_SLOTNUM: DWORD                     = 0x0180;
pub const TAG_IFD_SLOT_THREAD_SAFE: DWORD            = 0x0fac;
pub const TAG_IFD_THREAD_SAFE: DWORD                 = 0x0fad;
pub const TAG_IFD_SLOTS_NUMBER: DWORD                = 0x0fae;
pub const TAG_IFD_SIMULTANEOUS_ACCESS: DWORD         = 0x0faf;
pub const TAG_IFD_POLLING_THREAD: DWORD              = 0x0fb0;
pub const TAG_IFD_POLLING_THREAD_KILLABLE: DWORD     = 0x0fb1;
pub const TAG_IFD_STOP_POLLING_THREAD: DWORD         = 0x0fb2;
pub const TAG_IFD_POLLING_THREAD_WITH_TIMEOUT: DWORD = 0x0fb3;

extern "C" {
    pub static g_rgSCardT0Pci: SCARD_IO_REQUEST;
    pub static g_rgSCardT1Pci: SCARD_IO_REQUEST;
    pub static g_rgSCardRawPci: SCARD_IO_REQUEST;
    pub fn pcsc_stringify_error(e: LONG) -> *mut c_char;
    pub fn SCardEstablishContext(dwScope: DWORD, pvReserved1: LPCVOID, pvResserved2: LPCVOID, phContext: LPSCARDCONTEXT) -> LONG;
    pub fn SCardReleaseContext(hContext: SCARDCONTEXT) -> LONG;
    pub fn SCardIsValidContext(hContext: SCARDCONTEXT) -> LONG;
    pub fn SCardConnect(hContext: SCARDCONTEXT, szReader: LPCSTR, dwShareMode: DWORD, dwPreferredProtocols: DWORD, phCard: LPSCARDHANDLE, pdwActiveProtocol: LPDWORD) -> LONG;
    pub fn SCardReconnect(hCard: SCARDHANDLE, dwShareMode: DWORD, dwPreferredProtocols: DWORD, dwInitialization: DWORD, pdwActiveProtocol: LPDWORD) -> LONG;
    pub fn SCardDisconnect(hCard: SCARDHANDLE, dwDisposition: DWORD) -> LONG;
    pub fn SCardBeginTransaction(hCard: SCARDHANDLE) -> LONG;
    pub fn SCardEndTransaction(hCard: SCARDHANDLE, dwDisposition: DWORD) -> LONG;
    pub fn SCardStatus(hCard: SCARDHANDLE, mszReaderName: LPSTR, pcchReaderLen: LPDWORD, pdwState: LPDWORD, pdwProtocol: LPDWORD, pbAtr: LPBYTE, pcbAtrLen: LPDWORD) -> LONG;
    pub fn SCardGetStatusChange(hContext: SCARDCONTEXT, dwTimeout: DWORD, rgReaderStates: LPSCARD_READERSTATE, cReaders: DWORD) -> LONG;
    pub fn SCardControl(hCard: SCARDHANDLE, dwControlCode: DWORD, pbSendBuffer: LPCVOID, cbSendLength: DWORD, pbRecvBuffer: LPVOID, cbRecvLength: DWORD, lpBytesReturned: LPDWORD) -> LONG;
    pub fn SCardTransmit(hCard: SCARDHANDLE, pioSendPCI: *const SCARD_IO_REQUEST, pbSendBuffer: LPCBYTE, cbSendLength: DWORD, pioRecvPci: *mut SCARD_IO_REQUEST, pbRecvBuffer: LPBYTE, pcbRecvLength: LPDWORD) -> LONG;
    pub fn SCardListReaderGroups(hContext: SCARDCONTEXT, mszGroups: LPSTR, pcchGroups: LPDWORD) -> LONG;
    pub fn SCardListReaders(hContext: SCARDCONTEXT, mszGroups: LPCSTR, mszReaders: LPSTR, pcchReaders: LPDWORD) -> LONG;
    pub fn SCardFreeMemory(hContext: SCARDCONTEXT, pvMem: LPCVOID) -> LONG;
    pub fn SCardCancel(hContext: SCARDCONTEXT) -> LONG;
    pub fn SCardGetAttrib(hCard: SCARDHANDLE, dwAttrId: DWORD, pbAttr: LPBYTE, pcbAttrLen: LPDWORD) -> LONG;
    pub fn SCardSetAttrib(hCard: SCARDHANDLE, dwAttrId: DWORD, pbAttr: LPCBYTE, pcbAttrLen: DWORD) -> LONG;
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::c_void;

    #[test]
    fn test_linking() {
        let _: Vec<*const c_void> = vec![
            &g_rgSCardT0Pci as *const SCARD_IO_REQUEST as *const c_void,
            &g_rgSCardT1Pci as *const SCARD_IO_REQUEST as *const c_void,
            &g_rgSCardRawPci as *const SCARD_IO_REQUEST as *const c_void,
            pcsc_stringify_error as *const c_void,
            SCardEstablishContext as *const c_void,
            SCardReleaseContext as *const c_void,
            SCardIsValidContext as *const c_void,
            SCardConnect as *const c_void,
            SCardReconnect as *const c_void,
            SCardDisconnect as *const c_void,
            SCardBeginTransaction as *const c_void,
            SCardEndTransaction as *const c_void,
            SCardStatus as *const c_void,
            SCardGetStatusChange as *const c_void,
            SCardControl as *const c_void,
            SCardTransmit as *const c_void,
            SCardListReaderGroups as *const c_void,
            SCardListReaders as *const c_void,
            SCardFreeMemory as *const c_void,
            SCardCancel as *const c_void,
            SCardGetAttrib as *const c_void,
            SCardSetAttrib as *const c_void,
        ];
    }
}
