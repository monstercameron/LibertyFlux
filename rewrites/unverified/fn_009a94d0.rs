// original: 0x009a94d0 conv_vote_bump
/// Find `want` in the three conversation entry lists and bump its votes.
///
/// Each stage asks one sub-table lookup (stubbed, stdcall/2) for its
/// entry list for `key`, with the entry count delivered through a
/// stack out-byte. Every entry is 8 bytes (id, votes): when an entry's
/// id equals `want` its vote word is incremented and the function
/// returns; otherwise the next stage runs. A null `want` returns at
/// once. Thiscall, two stack words, no result.
export!(thiscall, rw_009A94D0(this: u32, key: u32, want: u32) -> u32 {
    unsafe {
        if want == 0 {
            return 0;
        }
        let mut n: u8 = 0;
        let out = &mut n as *mut u8 as u32;
        let r1: u32 = callee_stdcall!(1, u32, key, out);
        if bump_if_found(r1, n, want) {
            return 0;
        }
        n = 0;
        let r2: u32 = callee_stdcall!(2, u32, key, out);
        if bump_if_found(r2, n, want) {
            return 0;
        }
        n = 0;
        let r3: u32 = callee_stdcall!(3, u32, key, out);
        bump_if_found(r3, n, want);
        0
    }
});

/// Scan `n` id/vote pairs at `list` for `want`; bump and report hit.
unsafe fn bump_if_found(list: u32, n: u8, want: u32) -> bool {
    unsafe {
        let mut k = 0u32;
        while k < n as u32 {
            let e = list + k * 8;
            if ((e) as *const u32).read_unaligned() == want {
                let v = (e.wrapping_add(4) as *const u32).read_unaligned();
                (e.wrapping_add(4) as *mut u32).write_unaligned(v.wrapping_add(1));
                return true;
            }
            k += 1;
        }
        false
    }
}
