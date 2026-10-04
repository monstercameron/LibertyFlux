// original: 0x00bdf8b0 audio_dtor_retag_tail
/// Re-tag the audio sub-object with vtable 0xEB90F0 and forward to the
/// shared sub-object teardown (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf8b0(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB90F0;
        *(this as *mut u32) = relocated(VTABLE);
        callee_thiscall!(9, u32, this as u32)
    }
});
