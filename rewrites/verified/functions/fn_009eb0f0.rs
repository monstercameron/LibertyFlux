// original: 0x009eb0f0 ped_guarded_notify_direct
/// Guarded notify with a range-checked mode argument.
///
/// Returns early with the state word when the `0x210` flag is set and the
/// state at `0xa74` is 1 or 2, or with 0 (entry `eax`, fixed to 0 by the
/// contract) when the link at `0x224` is null while the flag is clear, or
/// with the state word when the link is null while the flag is set. A mode
/// outside 0..=2 reports through the shared reporter and continues. Then it
/// resolves the slot at `0xb9c` with tag `0x1bd`; a null resolution returns
/// 0, otherwise the inner object at `0x78` is notified with
/// (slot, 0x1bd, 0x48000, 6, 30.0, -1) and its answer is returned.
export!(thiscall, rw_009eb0f0(this_ptr: u32, mode: u32) -> u32 {
    unsafe {
        let flag = *((this_ptr + 0x210) as *const u8);
        if flag != 0 {
            let state = *((this_ptr + 0xa74) as *const u32);
            if state == 1 || state == 2 {
                return state;
            }
            if *((this_ptr + 0x224) as *const u32) == 0 {
                return state;
            }
        } else if *((this_ptr + 0x224) as *const u32) == 0 {
            return 0;
        }
        if (mode as i32) < 0 || mode > 2 {
            let _: u32 = callee_cdecl!(2, u32,);
        }
        let slot = *((this_ptr + 0xb9c) as *const u32);
        let resolved: u32 = callee_cdecl!(3, u32, slot, 0x1bd);
        if resolved == 0 {
            return 0;
        }
        let inner = *((this_ptr + 0x78) as *const u32);
        callee_thiscall!(
            4, u32, inner,
            slot, 0x1bd, 0x48000, 6, 30.0f32.to_bits(), 0xffffffff
        )
    }
});
