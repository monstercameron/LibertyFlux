// original: 0x008d6a20 resolve_index_or_fallback
// Resolves an index record through the active path, else falls back to a
// default lookup refined by a two-step slot query. Takes one flag word.
export!(cdecl, rw_008d6a20(flag: u32) -> u32 {
    unsafe {
        let ready: u32 = callee_cdecl!(1, u32,);
        if ready & 0xFF != 0
            && *global::<u8>(0x017F_5EB3) != 0
            && *global::<i32>(0x0103_E4B8) > 0
        {
            let count = *global::<u32>(0x0103_E4B8);
            let ctx = *global::<u32>(0x018B_6F1C);
            let rec: u32 = callee_thiscall!(2, u32, ctx, count);
            if rec != 0 {
                return *((rec as *const u32).wrapping_add(0x2CC));
            }
        }
        let cur: u32 = callee_cdecl!(3, u32, 0);
        if cur != 0 {
            return cur;
        }
        if flag & 0xFF == 0 {
            return 0;
        }
        let got: u32 = callee_cdecl!(4, u32,);
        if got == 0 {
            return 0;
        }
        let tab = *((got as *const u32).wrapping_add(0x89));
        if tab == 0 {
            return 0;
        }
        let slot = tab.wrapping_add(0x2E0);
        if slot == 0 {
            return 0;
        }
        let ok: u32 = callee_thiscall!(5, u32, slot, 0x2DE, cur);
        if ok & 0xFF == 0 {
            return 0;
        }
        callee_thiscall!(6, u32, slot, 0x2DE, 5)
    }
});
