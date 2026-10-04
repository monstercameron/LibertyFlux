// original: 0x0091B9E0 input_binding_lookup
/// Look a binding name up in the static table and resolve it to an action.
///
/// Returns `0xFF` when the name pointer is null, the name is empty, or the
/// second argument is zero. Otherwise scans the 187-entry table at file
/// address `0x01034668` (44 bytes per entry) with the compare helper; with no
/// match it returns `0xFF`. On a match it calls the setup helper with the
/// entry's field at +`0x28`, the entry index and two out-slots (one of them
/// the incoming arg0 stack slot, used as scratch), feeds the setup answer to
/// the query helper, and repeats the pair while the second out-slot reads 9,
/// 8 or 6. Anything else finishes with the six-argument dispatch call whose
/// answer is returned; a `-1` from the query helper returns `0xFF` instead.
/// The table immediates carry high/low relocations, so entry addresses are
/// relocated to the worker's image base before use, exactly as the loaded
/// original sees them.
export!(cdecl, rw_0091b9e0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        if a0 == 0 || *(a0 as *const u8) == 0 || a1 == 0 {
            return 0xFF;
        }
        let mut entry: u32 = 0x1034668;
        let mut index: u32 = 0;
        let mut tabfield: u32 = 0;
        let mut matched = false;
        while entry < 0x103668C {
            let r: u32 = callee_cdecl!(1, u32, a0, relocated(entry));
            if r == 0 {
                tabfield = *((relocated(entry).wrapping_add(0x28)) as *const u32);
                matched = true;
                break;
            }
            entry = entry.wrapping_add(0x2C);
            index = index.wrapping_add(1);
        }
        if !matched {
            return 0xFF;
        }
        let mut s0: u32 = 0;
        let mut s1: u32 = 0x7FFFFFFF;
        let ans: u32 = callee_stdcall!(2, u32, 0, tabfield, index,
            &mut s0 as *mut u32 as u32, &mut s1 as *mut u32 as u32);
        let mut r: u32 = callee_thiscall!(4, u32, ans);
        if r == 0xFFFFFFFF {
            return 0xFF;
        }
        loop {
            if s1 != 9 && s1 != 8 && s1 != 6 {
                break;
            }
            let ans2: u32 = callee_stdcall!(3, u32, 0, tabfield, index,
                &mut s0 as *mut u32 as u32, &mut s1 as *mut u32 as u32);
            r = callee_thiscall!(4, u32, ans2);
            if r == 0xFFFFFFFF {
                return 0xFF;
            }
        }
        callee_cdecl!(5, u32, s1, s0, a1, a2, 0, 0)
    }
});
