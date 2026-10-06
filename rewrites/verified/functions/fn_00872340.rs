// original: 0x00872340 rage::crExpressionProcessor::ExpressionFilter::vf3
/// Test whether any filter entry accepts the given kind and flags.
///
/// The entry table pointer lives at `this + 0x0C`; the word at table `+4`
/// is the entry count (compared SIGNED: a non-positive count accepts
/// nothing) and entries are consecutive dwords from the table base, so a
/// count of 2 or more re-reads the count word itself as the second entry
/// pointer. Each entry is examined in order; an entry whose first byte or
/// whose byte at `+2` is clear is skipped. Otherwise, with `kind` the low
/// byte of the first argument and `flags` the low word of the second, the
/// entry accepts when the flags word is zero and one of: the byte at
/// `+0x0A` is set with kind 1; the byte at `+0x0B` is clear with kind 0 or
/// 1; the kind is 5 or 6. Returns 1 on the first accepting entry, else 0.
/// The third argument is unread. Only the low byte of the result is
/// meaningful.
///
/// Original: 0x00872340 (thiscall, three stack arguments).
lf_checker_rt::export!(thiscall, rw_00872340(this: u32, kind: u32, flags: u32, _unused: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x0C;
        const COUNT_OFF: u32 = 0x04;
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        if table == 0 {
            return 0;
        }
        let count = ((table + COUNT_OFF) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let al = (kind & 0xFF) as u8;
        let di_zero = (flags & 0xFFFF) == 0;
        let mut i: i32 = 0;
        let mut slot = table;
        // Volatile loads: a count of 2 or more re-reads the count word as
        // an entry pointer, so the original faults on those trials. The
        // optimizer versions this loop on the (invariant) arguments and
        // would otherwise skip loads it proves irrelevant to the result,
        // surviving reads the original faults on. Volatile keeps every
        // read in order (still a plain byte load on x86).
        while i < count {
            let entry = (slot as *const u32).read_unaligned();
            let b0 = core::ptr::read_volatile(entry as *const u8);
            if b0 != 0 {
                let b2 = core::ptr::read_volatile((entry + 0x02) as *const u8);
                if b2 != 0 {
                    let b0a = core::ptr::read_volatile((entry + 0x0A) as *const u8);
                    if b0a != 0 && al == 1 && di_zero {
                        return 1;
                    }
                    let b0b = core::ptr::read_volatile((entry + 0x0B) as *const u8);
                    if b0b == 0 {
                        if al == 1 && di_zero {
                            return 1;
                        }
                        if al == 0 && di_zero {
                            return 1;
                        }
                    }
                    if (al == 6 || al == 5) && di_zero {
                        return 1;
                    }
                }
            }
            i += 1;
            slot = slot.wrapping_add(4);
        }
        0
    }
});
