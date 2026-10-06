// original: 0x00B54720 crmt_slot_refcount_bump (proposed)

/// Bump the reference count of slot `idx` in the global slot table.
///
/// The global at 0x16DD5D0 holds the table pointer: word +0x0 is the
/// slot array base, word +0x4 points at a per-slot flag byte array, and
/// word +0xC is the slot stride. When flag byte `idx` has bit 0x80 set
/// the slot is invalid and the original deliberately faults (an increment
/// through a null pointer); otherwise the count word at slot base plus
/// `idx` times the stride, offset by 4, is incremented and the slot
/// address is returned. The index arithmetic is unsigned wrapping.
///
/// Original: 0x00B54720 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b54720(idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x16dd5d0;
        const BASE: u32 = 0x0;
        const FLAGS: u32 = 0x4;
        const STRIDE: u32 = 0xc;
        const COUNT_OFF: u32 = 0x4;
        const INVALID: u8 = 0x80;
        let table = lf_checker_rt::global::<u32>(TABLE).read_unaligned();
        let flags = ((table + FLAGS) as *const u32).read_unaligned();
        let flag = ((flags.wrapping_add(idx)) as *const u8).read();
        if (flag & INVALID) != 0 {
            let trap = COUNT_OFF as *mut u32;
            let v = trap.read_unaligned();
            trap.write_unaligned(v.wrapping_add(1));
            return v;
        }
        let stride = ((table + STRIDE) as *const u32).read_unaligned();
        let base = ((table + BASE) as *const u32).read_unaligned();
        let slot = base.wrapping_add(idx.wrapping_mul(stride));
        let count = ((slot + COUNT_OFF) as *const u32).read_unaligned();
        ((slot + COUNT_OFF) as *mut u32).write_unaligned(count.wrapping_add(1));
        slot
    }
});
