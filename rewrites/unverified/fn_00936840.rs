// original: 0x00936840 net_msg_build_send (proposed)

/// Build a 13-entry message table on the stack and hand it to the sender.
///
/// The table has a leading status dword followed by 14 entries of 12 bytes
/// (kind dword, value dword, flag word). The entries start as
/// (-1, 0, 0); callee 1 classifies the `level` float and stores its code in
/// the status dword. Entry 0's kind is then always written, the remaining
/// entries depend on the mode byte, on whether `limit` exceeds its
/// threshold, on the status code (9 and 10 take the short form), on the
/// `flag` byte and on `target` (-1 skips the last two entries, 0 versus
/// non-zero picks their kind). Finally callee 2 sends the entries with a
/// routing byte, and callee 3 (the security-cookie check, which preserves
/// every register) runs; the send call's answer is the return value.
///
/// Original: 0x00936840 (thiscall shape with four stack words; the incoming
/// `this` is never read, only the stack words). The entries' padding bytes
/// are stack the original never writes: the contract's zero stack fill,
/// matched here by building the table over a zeroed buffer.
lf_checker_rt::export!(thiscall, rw_00936840(
    _this: u32,
    flag: u32,
    level: u32,
    limit: u32,
    target: u32,
) -> u32 {
    unsafe {
        const MODE: u32 = 0x0103_6EE0;
        const THRESH: u32 = 0x0103_6EBC;
        const PARAM: u32 = 0x0103_6EB0;
        const ROUTE: u32 = 0x011A_2EA9;
        const SENDER: u32 = 0x0117_6888;
        const ENTRIES: u32 = 14;
        const ENTRY_LEN: u32 = 12;
        const CAL_CLASSIFY: u32 = 1;
        const CAL_SEND: u32 = 2;
        const CAL_COOKIE: u32 = 3;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        // Leading status dword plus the entries, all zero first.
        let mut tab = [0u8; 172];
        #[inline(always)]
        fn wr32(tab: &mut [u8; 172], at: u32, v: u32) {
            tab[at as usize..at as usize + 4].copy_from_slice(&v.to_le_bytes());
        }
        #[inline(always)]
        fn wr16(tab: &mut [u8; 172], at: u32, v: u16) {
            tab[at as usize..at as usize + 2].copy_from_slice(&v.to_le_bytes());
        }
        let base = |i: u32| 4 + i * ENTRY_LEN;
        for k in 0..ENTRIES {
            wr32(&mut tab, base(k), 0xFFFF_FFFF);
            wr32(&mut tab, base(k) + 4, 0);
            wr16(&mut tab, base(k) + 8, 0);
        }
        wr32(&mut tab, 0, 0xFFFF_FFFF);

        let status_ptr = tab.as_mut_ptr() as u32;
        let _: u32 =
            lf_checker_rt::callee_stdcall!(CAL_CLASSIFY, u32, level, status_ptr);
        let status = u32::from_le_bytes([tab[0], tab[1], tab[2], tab[3]]);

        let thresh = rd32(lf_checker_rt::relocated(THRESH));
        if rd8(lf_checker_rt::relocated(MODE)) != 0 {
            let mut idx = 0u32;
            if limit > thresh {
                wr32(&mut tab, base(0), 0x0E);
                wr32(&mut tab, base(1), status);
                tab[(base(1) + 9) as usize] = 1;
                if status == 9 || status == 10 {
                    idx = 2;
                } else {
                    wr32(&mut tab, base(2), 0x18);
                    idx = 3;
                }
                wr32(
                    &mut tab,
                    base(idx) + 4,
                    rd32(lf_checker_rt::relocated(PARAM)),
                );
            }
            wr32(&mut tab, base(idx), 0x15);
            idx += 1;
            wr32(
                &mut tab,
                base(idx),
                if (flag as u8) != 0 { 0x12 } else { 0x11 },
            );
            idx += 1;
            if target != 0xFFFF_FFFF {
                wr32(&mut tab, base(idx), 0x14);
                idx += 1;
                wr32(
                    &mut tab,
                    base(idx),
                    if target != 0 { 0x12 } else { 0x11 },
                );
            }
        } else {
            wr32(&mut tab, base(0), 0x15);
            let mut idx = 2u32;
            wr32(&mut tab, base(1), 0x12);
            if (flag as u8) == 0 {
                wr32(&mut tab, base(1), 0x11);
            }
            if target != 0xFFFF_FFFF {
                wr32(&mut tab, base(2), 0x14);
                idx = 4;
                wr32(&mut tab, base(3), 0x11);
                if target != 0 {
                    wr32(&mut tab, base(3), 0x12);
                }
            }
            if limit > thresh {
                wr32(&mut tab, base(idx), 0x0E);
                idx += 1;
                wr32(&mut tab, base(idx), status);
                tab[(base(idx) + 9) as usize] = 1;
                if status != 9 && status != 10 {
                    wr32(&mut tab, base(idx + 1), 0x18);
                }
            }
        }

        let route = rd8(lf_checker_rt::relocated(ROUTE)) as u32;
        let entries_ptr = (tab.as_mut_ptr() as u32).wrapping_add(4);
        let answer: u32 = lf_checker_rt::callee_thiscall!(
            CAL_SEND,
            u32,
            lf_checker_rt::relocated(SENDER),
            entries_ptr,
            route
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        answer
    }
});
