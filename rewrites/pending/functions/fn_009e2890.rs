// original: 0x009e2890 audio_record_notify_if_set
/// If the record tag is set (not -1), notify through the helper.
/// (thiscall/0; no defined return: EAX keeps its entry value on the skip path,
/// so the contract compares calls and memory, not EAX.)
export!(thiscall, rw_009e2890(this: *mut u8) -> u32 {
    unsafe {
        if *(this as *const u32) != 0xFFFF_FFFF {
            callee_cdecl!(1, u32, this as u32);
        }
        0
    }
});
