// original: 0x00909070 radar_sweep_filters
/// Sweep the radar entry array through three engine filter steps.
///
/// Loops twice the configured count over the global entry array, running the
/// gate step per entry and, when its low answer byte is set, the two action
/// steps. Returns twice the configured count.
export!(cdecl, rw_00909070() -> u32 {
    unsafe {
        let count = *global::<i32>(0x10344EC);
        let total = count.wrapping_mul(2);
        if total <= 0 {
            return total as u32;
        }
        let arr = *global::<u32>(0x118F4E8) as *const u32;
        let tag = *global::<u32>(0x1032F58);
        let mut i = 0i32;
        while i < total {
            let v = *arr.add(i as usize);
            if callee_cdecl!(1, u32, v, tag) & 0xFF != 0 {
                callee_cdecl!(2, u32, v, tag);
                callee_cdecl!(3, u32, v, tag);
            }
            i = i.wrapping_add(1);
        }
        total as u32
    }
});
