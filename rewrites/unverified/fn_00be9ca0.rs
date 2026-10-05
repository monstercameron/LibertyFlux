// original: 0x00be9ca0 task_apply_flagged_ops (proposed)

/// Apply the flag-selected sub-operations of parameter block `b` to target `a`.
///
/// Does nothing when `a` is null. Otherwise each bit of the flag byte at
/// `b+0x2d` fires one operation: bit 0 sends the float at `b+0x08` to
/// callee 1, bit 1 the float at `b+0x0c` to callee 2, bit 3 the dword at
/// `b+0x1c` to callee 4, bit 4 the dword at `b+0x20` to callee 5, each with
/// the looked-up object as the `this` pointer; bit 2 passes the three
/// floats at `b+0x10..0x18` as an array to callee 3 with `a` as `this`;
/// bit 5 sends bit 2 of `b+0x2c` to callee 6 and bit 6 sends bit 3 to
/// callee 7, both with `a` as `this`.
///
/// The sub-selector at `b+0x2e` then fires callee 8 with the dword at
/// `b+0x24` for values 1 and 3, or callee 9 for value 2. Finally, when bit
/// 0x10 of `b+0x2c` is set, flag bit 3 is set on the looked-up object at
/// offset 0xe8.
///
/// The lookup reads the index byte at `a+4` (0xff means no object) and the
/// slot byte at `a+0x40`, and computes `stride*index + table[slot]`, where
/// the stride and the table base come from two shared globals and each
/// table entry sits `0x6f40` bytes apart starting at base `+0x6f14`. With
/// no object the flag write targets address 0xe8 and faults, exactly like
/// the original.
///
/// Returns nothing significant.
///
/// Original: 0x00be9ca0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00be9ca0(a: u32, b: u32) -> u32 {
    unsafe {
        const G_STRIDE: u32 = 0x0115d968;
        const G_TABLE: u32 = 0x0115d988;
        const SLOT_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f14;
        const OBJ_FLAG_OFF: u32 = 0xe8;
        const NO_OBJECT: u8 = 0xff;

        #[inline(always)]
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(x: u32) -> u8 {
            unsafe { (x as *const u8).read() }
        }

        if a == 0 {
            return 0;
        }
        let idx = rd8(a + 4);
        let slot = rd8(a + 0x40);
        let lookup = || -> u32 {
            if idx == NO_OBJECT {
                0
            } else {
                let stride = rd32(lf_checker_rt::relocated(G_STRIDE));
                let base = rd32(lf_checker_rt::relocated(G_TABLE));
                let entry = rd32(
                    base
                        .wrapping_add((slot as u32).wrapping_mul(SLOT_STRIDE))
                        .wrapping_add(TABLE_BIAS),
                );
                stride.wrapping_mul(idx as u32).wrapping_add(entry)
            }
        };

        let fb = rd8(b + 0x2d);
        if fb & 1 != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, lookup(), rd32(b + 0x08));
        }
        if fb & 2 != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, lookup(), rd32(b + 0x0c));
        }
        if fb & 4 != 0 {
            let mut buf = [rd32(b + 0x10), rd32(b + 0x14), rd32(b + 0x18)];
            lf_checker_rt::callee_thiscall!(3, u32, a, buf.as_mut_ptr() as u32);
        }
        if fb & 8 != 0 {
            lf_checker_rt::callee_thiscall!(4, u32, lookup(), rd32(b + 0x1c));
        }
        if fb & 0x10 != 0 {
            lf_checker_rt::callee_thiscall!(5, u32, lookup(), rd32(b + 0x20));
        }
        if fb & 0x20 != 0 {
            let bit = ((rd8(b + 0x2c) >> 2) & 1) as u32;
            lf_checker_rt::callee_thiscall!(6, u32, a, bit);
        }
        if fb & 0x40 != 0 {
            let bit = ((rd8(b + 0x2c) >> 3) & 1) as u32;
            lf_checker_rt::callee_thiscall!(7, u32, a, bit);
        }
        let sub = rd8(b + 0x2e);
        if sub == 1 || sub == 3 {
            lf_checker_rt::callee_thiscall!(8, u32, a, rd32(b + 0x24));
        } else if sub == 2 {
            lf_checker_rt::callee_thiscall!(9, u32, a, rd32(b + 0x24));
        }
        if rd8(b + 0x2c) & 0x10 != 0 {
            let p = lookup().wrapping_add(OBJ_FLAG_OFF);
            let v = (p as *const u8).read();
            (p as *mut u8).write(v | 8);
        }
        0
    }
});
