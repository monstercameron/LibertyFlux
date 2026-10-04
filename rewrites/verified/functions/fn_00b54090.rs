// original: 0x00b54090 crFrameFilterBoneMask::vf0
/// Scalar deleting destructor: stamps the filter vtable, destroys
/// the base, and returns `this` to its pool when bit 0 of the flag is
/// set. Returns `this`.
export!(thiscall, rw_00b54090(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xEAF2FC);
        callee_thiscall!(1, u32, this as u32);
        if flag & 1 != 0 {
            let pool = *global::<u32>(0x1669D74);
            callee_thiscall!(2, u32, pool, this as u32);
        }
        this as u32
    }
});
