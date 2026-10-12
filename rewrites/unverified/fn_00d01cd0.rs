// original: 0x00D01CD0 CTaskComplexBackOff::vf20

/// Poll a backing-off task's sub-task until it accepts or refuses.
///
/// Returns the sub-task word at `+0x8` when the active byte at `+0x18` is
/// clear. Otherwise queries the sub-task (planted stub callee through its
/// table slot `+0x14`) with kinds 1 then 2: a nonzero low byte, or bit 0 of
/// the sub-task flags at `+0xc`, sets bit 1 of those flags and returns zero.
/// Two zero answers return the sub-task word.
///
/// Original: 0x00D01CD0 (thiscall, one stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00d01cd0(this: u32, a1: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x14;
        let sub = ((this + 0x08) as *const u32).read_unaligned();
        if ((this + 0x18) as *const u8).read() == 0 {
            return sub;
        }
        let flags = ((sub + 0x0c) as *const u32).read_unaligned();
        if flags & 1 != 0 {
            ((sub + 0x0c) as *mut u32).write_unaligned(flags | 2);
            return 0;
        }
        let table = ((sub) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute((((table + SLOT) as *const u32).read_unaligned()) as usize);
        if (query(sub, a1, 1, 0) & 0xFF) != 0 {
            ((sub + 0x0c) as *mut u32).write_unaligned(
                ((sub + 0x0c) as *const u32).read_unaligned() | 2);
            return 0;
        }
        let sub = ((this + 0x08) as *const u32).read_unaligned();
        let flags = ((sub + 0x0c) as *const u32).read_unaligned();
        if flags & 1 != 0 {
            ((sub + 0x0c) as *mut u32).write_unaligned(flags | 2);
            return 0;
        }
        let table = ((sub) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute((((table + SLOT) as *const u32).read_unaligned()) as usize);
        if (query(sub, a1, 2, 0) & 0xFF) == 0 {
            return ((this + 0x08) as *const u32).read_unaligned();
        }
        ((sub + 0x0c) as *mut u32).write_unaligned(
            ((sub + 0x0c) as *const u32).read_unaligned() | 2);
        0
    }
});
