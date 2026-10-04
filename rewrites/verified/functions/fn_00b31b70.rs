// original: 0x00b31b70 task_slot_lookup (proposed)

/// Find a task slot in a shared table and hand it to a worker callee.
///
/// `id` selects the slot (1 to 30; anything else returns `id - 1` at once).
/// The table holds 32 entries of 80 bytes: the id word at `+0x0`, three
/// coordinate words at `+0x10` to `+0x18`, and two link words at `+0x28` and
/// `+0x2c`. The first entry whose id word equals `id` is then qualified: when
/// `marker` is non-zero the link words must equal `marker` and `tag`, and
/// when `marker` is zero the three coordinate words must each equal the
/// three words at `vec` (a NaN never equals). The qualifying entry's address
/// goes to the worker callee, whose answer is returned. When nothing
/// qualifies, `id - 1` comes back, except that a zero `marker` with at least
/// one id hit leaves the last floating-point flag snapshot in the second
/// byte: `0x02` when the entry float was greater, `0x03` when smaller,
/// `0x47` when either side was NaN.
///
/// Original: 0x00b31b70 (cdecl, four stack words; one one-word callee).
lf_checker_rt::export!(cdecl, rw_00b31b70(id: u32, vec: u32, marker: u32, tag: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x01661a60;
        const ENTRIES: u32 = 32;
        const STRIDE: u32 = 0x50;
        const WORKER: u32 = 1;
        #[inline(always)]
        unsafe fn r32(o: u32, off: u32) -> u32 {
            unsafe { (o as *const u32).byte_add(off as usize).read_unaligned() }
        }
        let base = id.wrapping_sub(1);
        if base > 29 {
            return base;
        }
        let table = lf_checker_rt::relocated(TABLE);
        let mut saw_id = false;
        let mut snap = 0u32;
        for idx in 0..ENTRIES {
            let e = table.wrapping_add(idx.wrapping_mul(STRIDE));
            if r32(e, 0x00) != id {
                continue;
            }
            saw_id = true;
            if marker != 0 {
                if r32(e, 0x28) != marker || r32(e, 0x2c) != tag {
                    continue;
                }
            } else {
                let mut equal = true;
                for (w, v) in [(0x10u32, 0u32), (0x14, 4), (0x18, 8)] {
                    let a = f32::from_bits(r32(e, w));
                    let b = f32::from_bits(r32(vec, v));
                    if a != b {
                        snap = if a.is_nan() || b.is_nan() {
                            0x47
                        } else if a > b {
                            0x02
                        } else {
                            0x03
                        };
                        equal = false;
                        break;
                    }
                }
                if !equal {
                    continue;
                }
            }
            return lf_checker_rt::callee_cdecl!(WORKER, u32, e);
        }
        if marker == 0 && saw_id {
            base & !0xFF00 | snap << 8
        } else {
            base
        }
    }
});
