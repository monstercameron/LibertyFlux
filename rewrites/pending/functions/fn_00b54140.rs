// original: 0x00b54140 crmtManagerPriority::vf0
/// Scalar deleting destructor: destroys the embedded manager, then
/// frees `this` when bit 0 of the flag is set. Returns `this`.
export!(thiscall, rw_00b54140(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        if flag & 1 != 0 {
            callee_cdecl!(2, u32, this as u32);
        }
        this as u32
    }
});
