// original: 0x00d8e650 audio_params_copy_and_notify
/// Copy five parameter slots from `arg` into `this`, then notify listeners.
///
/// Each of the five slots starting at `this + 0x2c` is initialised through
/// the shared initialiser with the caller's argument as its object. Every
/// nonzero slot among the last four is then reported to the slot listener,
/// and a nonzero head slot is reported to the head listener. Returns `this`.
lf_rs89_rt::export!(thiscall, rw_00d8e650(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let mut slot = 0x2Cu32;
        while slot < 0x40 {
            lf_rs89_rt::callee_thiscall!(1, u32, arg, this.wrapping_add(slot as usize) as u32);
            slot += 4;
        }
        let mut cur = this.wrapping_add(0x30) as *const u32;
        let end = this.wrapping_add(0x40) as *const u32;
        while cur < end {
            let v = *cur;
            if v != 0 {
                lf_rs89_rt::callee_cdecl!(2, u32, v, arg);
            }
            cur = cur.wrapping_add(1);
        }
        let head = *(this.wrapping_add(0x2C) as *const u32);
        if head != 0 {
            lf_rs89_rt::callee_cdecl!(3, u32, head, arg);
        }
        this as u32
    }
});
