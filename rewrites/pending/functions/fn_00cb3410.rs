// original: 0x00CB3410 CTaskComplexGoToAttractor::vf18

/// Run one attractor check and clear a stale attraction flag.
///
/// `this` is the complex task, `ped` the ped. The attractor handle stored
/// at `this+0x14` and the ped are passed to the attractor lookup (callee 1,
/// two stack words, callee cleans up); its result selects the attractor
/// (callee 2). If the ped's flag byte (`ped+0x26c`) has bit 0x40 set, the
/// current owner of the attractor (callee 3) is compared with the ped and
/// the bit is cleared when they differ. The result is always zero.
///
/// Original: 0x00CB3410 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB3410(this: u32, ped: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const SELECT: u32 = 2;
        const OWNER_OF: u32 = 3;
        const ATTRACTOR: u32 = 0x14;
        const PED_FLAGS: u32 = 0x26c;
        const ATTRACTED_BIT: u8 = 0x40;
        let handle = (this.wrapping_add(ATTRACTOR) as *const u32).read_unaligned();
        let found: u32 = lf_checker_rt::callee_stdcall!(LOOKUP, u32, ped, handle);
        lf_checker_rt::callee_thiscall!(SELECT, u32, found);
        let flags = (ped.wrapping_add(PED_FLAGS) as *mut u8).read_unaligned();
        if flags & ATTRACTED_BIT != 0 {
            let owner: u32 = lf_checker_rt::callee_thiscall!(OWNER_OF, u32, handle);
            if owner != ped {
                (ped.wrapping_add(PED_FLAGS) as *mut u8)
                    .write_unaligned(flags & !ATTRACTED_BIT);
            }
        }
        0
    }
});
