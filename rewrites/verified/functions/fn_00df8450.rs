// original: 0x00DF8450 save_settings_match (proposed)

/// Match a path against a table of fixed-size name records (stdcall, one
/// stack arg, byte result).
///
/// `path` points at a NUL-terminated string. After a guard check on the
/// path, two 8-byte constants are copied into a frame buffer and handed
/// with a key to a lookup callee, twice with different keys; a null answer
/// from either fails with 0. Otherwise a count callee yields `edi`
/// (compared SIGNED: 0 or less skips the loop; the count must stay small
/// or the loop below would not terminate) and a table callee yields the
/// record base. Each of the `(edi - 1) >> 5 + 1` records (0x20 bytes apart)
/// is compared byte-wise (unsigned strcmp) with the path; a mismatch is
/// reported through a store callee. A final release/notify pair runs on
/// success, returning 1.
///
/// The entry ECX is never read by the original (it flows only into the
/// guard callee, whose answer is scripted), so the contract pins it to a
/// constant the rewrite passes explicitly. The CRT security-cookie check
/// runs natively on the original side and is not modelled.
lf_checker_lf_checker_rt::export!(stdcall, rb586_fn3(path: u32) -> u32 {
    unsafe {
        const PINNED_ECX: u32 = 0x12345678;
        const K_LOOKUP: u32 = 0x00f01ee0;
        const B_LO: u32 = 0x00f01ef0;
        const B_HI: u32 = 0x00f01ef8;
        const K_FIRST: u32 = 0x00f01f00;
        const K_SECOND: u32 = 0x00f01f04;
        const ENTRY_STRIDE: u32 = 0x20;
        const C_GUARD: u32 = 1;
        const C_SETUP: u32 = 2;
        const C_LOOKUP1: u32 = 3;
        const C_COUNT: u32 = 4;
        const C_TABLE: u32 = 5;
        const C_BIND: u32 = 6;
        const C_NOTIFY: u32 = 7;
        const C_LOOKUP2: u32 = 8;
        const C_STORE: u32 = 9;
        const C_DONE: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        /// Unsigned byte-wise strcmp, -1/0/+1 like the original's sbb/or tail.
        unsafe fn strcmp(mut a: u32, mut b: u32) -> i32 {
            unsafe {
                loop {
                    let x = rd8(a);
                    let y = rd8(b);
                    if x != y {
                        return if (x as u32) < (y as u32) { -1 } else { 1 };
                    }
                    if x == 0 {
                        return 0;
                    }
                    a = a.wrapping_add(1);
                    b = b.wrapping_add(1);
                }
            }
        }

        let g: u32 = lf_checker_rt::callee_thiscall!(C_GUARD, u32, PINNED_ECX, path);
        if (g as u8) == 0 {
            return g;
        }
        lf_checker_rt::callee_cdecl!(C_SETUP, u32, lf_checker_rt::relocated(K_LOOKUP), 0);
        let mut buf = [
            rd32(lf_checker_rt::relocated(B_LO)),
            rd32(lf_checker_rt::relocated(B_LO).wrapping_add(4)),
            rd32(lf_checker_rt::relocated(B_HI)),
            rd32(lf_checker_rt::relocated(B_HI).wrapping_add(4)),
        ];
        let bufp = buf.as_mut_ptr() as u32;
        let esi: u32 = lf_checker_rt::callee_cdecl!(C_LOOKUP1, u32, bufp, lf_checker_rt::relocated(K_FIRST));
        if esi == 0 {
            return 0;
        }
        let edi = lf_checker_rt::callee_cdecl!(C_COUNT, u32, esi) as i32;
        let table: u32 = lf_checker_rt::callee_cdecl!(C_TABLE, u32, edi as u32);
        lf_checker_rt::callee_cdecl!(C_BIND, u32, esi, table, edi as u32);
        lf_checker_rt::callee_cdecl!(C_NOTIFY, u32, esi);
        let ebx: u32 = lf_checker_rt::callee_cdecl!(C_LOOKUP2, u32, bufp, lf_checker_rt::relocated(K_SECOND));
        if ebx == 0 {
            return 0;
        }
        if edi > 0 {
            let mut n = ((edi as u32).wrapping_sub(1) >> 5) + 1;
            let mut ent = table;
            while n != 0 {
                if strcmp(path, ent) != 0 {
                    lf_checker_rt::callee_cdecl!(C_STORE, u32, ebx, ent, ENTRY_STRIDE);
                }
                ent = ent.wrapping_add(ENTRY_STRIDE);
                n -= 1;
            }
        }
        lf_checker_rt::callee_cdecl!(C_NOTIFY, u32, ebx);
        lf_checker_rt::callee_cdecl!(C_DONE, u32, table);
        1
    }
});
