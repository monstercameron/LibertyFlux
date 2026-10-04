// original: 0x00cfeab0 task_cover_gate_check (proposed)

/// Gate predicate over a task object and its linked inner object: returns 1
/// unless every gate passes, in which case it returns 0.
///
/// `obj` points to the task. The gates, in order: bit 0 of the byte at
/// `obj + 0x26f` must be set; the low nibble of the byte at `obj + 0x1e2`
/// must be below 2; bit 2 of the dword at `obj + 0x24` must be clear. Any
/// failure returns 0. Then the pointer at `obj + 0xab0` is loaded: a null
/// pointer returns 1, as does an inner dword at `+0x28` whose bits 6..9 are
/// anything but exactly bit 7 (`& 0x3c0 != 0x80`). Otherwise the dword at
/// inner `+0x1304` decides: equal to 2 returns 0, anything else 1.
/// Only AL carries the result (upper EAX bits are leftovers).
///
/// Original: 0x00cfeab0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00cfeab0(obj: u32) -> u32 {
    unsafe {
        const FLAG_A: u32 = 0x26F;
        const MODE: u32 = 0x1E2;
        const BITS: u32 = 0x24;
        const INNER: u32 = 0xAB0;
        const INNER_MASK_WORD: u32 = 0x28;
        const INNER_STATE: u32 = 0x1304;
        const MASK: u32 = 0x3C0;
        const WANT: u32 = 0x80;
        if (obj.wrapping_add(FLAG_A) as *const u8).read() & 1 == 0 {
            return 0;
        }
        if (obj.wrapping_add(MODE) as *const u8).read() & 0x0F >= 2 {
            return 0;
        }
        if (obj.wrapping_add(BITS) as *const u32).read_unaligned() & 0x04 != 0 {
            return 0;
        }
        let inner = (obj.wrapping_add(INNER) as *const u32).read_unaligned();
        if inner == 0 {
            return 1;
        }
        if (inner.wrapping_add(INNER_MASK_WORD) as *const u32).read_unaligned() & MASK != WANT {
            return 1;
        }
        if (inner.wrapping_add(INNER_STATE) as *const u32).read_unaligned() == 2 {
            0
        } else {
            1
        }
    }
});
