// original: 0x009DF7C0 CPortalVisTracker::~CPortalVisTracker__deleting

//! Reset the tracker and conditionally send its detach notification.
//!
//! This thiscall deleting destructor receives the tracker in ECX and a
//! flags word on the stack. It calls the reset-detach helper, sends the
//! notification with the tracker pointer when flags bit 0 is set, and
//! returns the tracker pointer. The notification target is intercepted by
//! the contract, including when its original code lies in the encrypted
//! portion of the executable.

const NOTIFY_FLAG_MASK: u32 = 0x1;
const RESET_DETACH_CALLEE: u32 = 1;
const SEND_NOTIFICATION_CALLEE: u32 = 2;

lf_checker_rt::export!(thiscall, rw_009df7c0(tracker: u32, flags: u32) -> u32 {
    let _: u32 = lf_checker_rt::callee_thiscall!(RESET_DETACH_CALLEE, u32, tracker);
    if flags & NOTIFY_FLAG_MASK != 0 {
        let _: u32 = lf_checker_rt::callee_cdecl!(SEND_NOTIFICATION_CALLEE, u32, tracker);
    }
    tracker
});
