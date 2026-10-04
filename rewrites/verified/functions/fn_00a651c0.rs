// original: 0x00a651c0 record_lookup_check_limit
/// Looks up the record and checks it against the shared limit.
///
/// Resolves the record through the lookup helper; a null record, a failed
/// check or a record value ordered-above the shared float limit all report
/// 0. Otherwise reports 1.
export!(cdecl, rw_00a651c0(a: u32, b: u32) -> u32 {
    unsafe {
        let rec = callee_thiscall!(1, u32, a, b);
        if rec == 0 {
            return 0;
        }
        let ok = callee_thiscall!(2, u32, rec, a);
        if (ok as u8) == 0 {
            return 0;
        }
        let value = *((rec + 0x1C) as *const f32);
        let limit = *global::<f32>(0xFE88D4);
        if value > limit {
            0
        } else {
            1
        }
    }
});
