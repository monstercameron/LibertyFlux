// original: 0x00872010 crmt_request_init
//! Initialize a motion-request object: install the shared vtable pointer and
//! clear the state words. No return value is produced (EAX keeps its entry
//! value on both sides, so the contract compares no return channel).

/// File VA of the shared motion-request vtable stamped by the initializer.
const CRMT_REQUEST_VTABLE: u32 = 0x00E86AFC;
export!(thiscall, rw_00872010(this: *mut u8) -> u32 {
    unsafe {
        *(this.add(0x0C) as *mut u32) = 0;
        *(this.add(0x6C) as *mut u32) = 0;
        *(this.add(0x60) as *mut u32) = relocated(CRMT_REQUEST_VTABLE);
        *(this as *mut u32) = relocated(CRMT_REQUEST_VTABLE);
        0
    }
});
