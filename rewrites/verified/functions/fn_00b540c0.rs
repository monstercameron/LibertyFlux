// original: 0x00b540c0 crmtManagerPriority::crFrameFilterWeightCorrection::vf0
/// Scalar deleting destructor: stamps the filter vtable, destroys the
/// base, and frees `this` when bit 0 of the flag is set. Returns `this`.
export!(thiscall, rw_00b540c0(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xEAF350);
        callee_thiscall!(1, u32, this as u32);
        if flag & 1 != 0 {
            callee_cdecl!(2, u32, this as u32);
        }
        this as u32
    }
});
