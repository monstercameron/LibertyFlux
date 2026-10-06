// original: 0x009763c0 audCollisionAudioEntity::vf0

/// Destroy the collision audio entity, freeing it when asked.
///
/// Frees the owned block at +0x8 (cdecl helper) and clears the slot, stamps
/// the base vtable, runs the base destructor, and when bit 0 of `flag` is
/// set frees the object itself. Returns the object pointer.
/// Original: 0x009763C0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_009763c0(this: u32, flag: u32) -> u32 {
    unsafe {
        const OWNED: u32 = 8;
        const VT_BASE: u32 = 0xE83134;
        const FREE_FN: u32 = 1;
        const BASE_DTOR: u32 = 2;
        let p = ((this.wrapping_add(OWNED)) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(FREE_FN, u32, p);
        ((this.wrapping_add(OWNED)) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT_BASE));
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        if flag & 1 != 0 {
            lf_checker_rt::callee_cdecl!(FREE_FN, u32, this);
        }
        this
    }
});
