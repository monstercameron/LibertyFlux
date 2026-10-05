// original: 0x00A219B0 ped_task_state_select (proposed)

/// State-index selector: conditionally arms a sub-object and picks an
/// output word from one of two constant tables.
///
/// `this` is the task object (sub-object pointer at `+0x12C`, state index
/// at `+0x360`, output word at `+0x31C`). `a1` is a sensor block or null
/// (sense flag at `+0x328D`, key byte at `+0x269C`, probed bytes at
/// `+0x269E/+0x269F`). `a2` is a four-slot table or null (pointers at
/// `+0xD58/+0xD5C/+0xD60/+0xD64`, each with a tag word at `+0x2E`).
/// No floating-point arithmetic: the only float is a 1.0 stamp, moved as
/// bits. No outgoing calls.
///
/// Behaviour: when `a1` is non-null, two keyed bytes `x1 = b[0x269E] ^ key`
/// and `x2 = b[0x269F] ^ key` are formed and compared unsigned against
/// 0x7F. With the sense flag set the sub-object is armed (1.0 stamped at
/// its `+0x1460`, state index incremented) only when the first exceeds
/// 0x7F and the second does not; with it clear, only when the first is at
/// most 0x7F, the second exceeds it, and the mode flag global is zero.
/// The state index is then clamped
/// into 1..=3 (at most 0 becomes 3, at least 4 becomes 1, storing the
/// clamped value back) and the output word is loaded from the first
/// constant table at that index. When the selector global equals 2 and
/// `a2` is non-null, each live slot whose sign-extended tag word equals
/// the identity global reloads the output from the second table at the
/// same index (same value each time). Returns the last value written to
/// the output word. Original is thiscall with two stack words.
lf_checker_rt::export!(thiscall, rw_00A219B0(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const SENSE_OFF: u32 = 0x328D;
        const KEY_OFF: u32 = 0x269C;
        const X1_OFF: u32 = 0x269E;
        const X2_OFF: u32 = 0x269F;
        const ASCII_MAX: u8 = 0x7F;
        const SUB_OFF: u32 = 0x12C;
        const SUB_READY: u32 = 0x1460;
        const ONE_BITS: u32 = 0x3F80_0000;
        const STATE_OFF: u32 = 0x360;
        const OUT_OFF: u32 = 0x31C;
        const TAG_OFF: u32 = 0x2E;
        const MODE_FLAG: u32 = 0x011F_701F;
        const SELECTOR: u32 = 0x011D_6FD4;
        const IDENTITY: u32 = 0x012F_A4F4;
        const TABLE_MAIN: u32 = 0x00E9_B288;
        const TABLE_ALT: u32 = 0x00E9_B298;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if a1 != 0 {
            let flag = rd8(a1 + SENSE_OFF);
            let key = rd8(a1 + KEY_OFF);
            let x1 = rd8(a1 + X1_OFF) ^ key;
            let x2 = rd8(a1 + X2_OFF) ^ key;
            let store = if flag != 0 {
                x1 > ASCII_MAX && x2 <= ASCII_MAX
            } else {
                x1 <= ASCII_MAX && x2 > ASCII_MAX && rd8(lf_checker_rt::relocated(MODE_FLAG)) == 0
            };
            if store {
                let sub = rd32(this + SUB_OFF);
                wr32(sub + SUB_READY, ONE_BITS);
                wr32(this + STATE_OFF, rd32(this + STATE_OFF).wrapping_add(1));
            }
        }
        let mut idx = rd32(this + STATE_OFF) as i32;
        if idx <= 0 {
            idx = 3;
            wr32(this + STATE_OFF, 3);
        } else if idx >= 4 {
            idx = 1;
            wr32(this + STATE_OFF, 1);
        }
        let main = lf_checker_rt::relocated(TABLE_MAIN);
        let mut out = rd32(main + (idx as u32) * 4);
        wr32(this + OUT_OFF, out);
        if rd32(lf_checker_rt::relocated(SELECTOR)) == 2 && a2 != 0 {
            let identity = rd32(lf_checker_rt::relocated(IDENTITY)) as i32;
            let alt = lf_checker_rt::relocated(TABLE_ALT);
            for slot in [0xD58u32, 0xD5C, 0xD60, 0xD64] {
                let p = rd32(a2 + slot);
                if p != 0 && (rd16(p + TAG_OFF) as i16 as i32) == identity {
                    out = rd32(alt + (idx as u32) * 4);
                    wr32(this + OUT_OFF, out);
                }
            }
        }
        out
    }
});
