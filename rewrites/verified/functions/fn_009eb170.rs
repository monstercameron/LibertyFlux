// original: 0x009eb170 ped_guarded_notify_single
/// Guarded notify with one lazily initialised shared handle.
///
/// Same guards as its siblings (early state return, null-link return of 0
/// or the state word). Resolves the slot at `0xb9c`, then takes the shared
/// handle: when flag bit 0 at `0x12b6100` is clear it resolves the shared
/// name, sets the bit and caches the handle at `0x12b60fc`, otherwise it
/// reuses the cache. A failed pair check returns 0, otherwise the inner
/// object at `0x78` is notified with (resolved, cached, 0x48000, 6, 30.0)
/// and its answer is returned.
export!(thiscall, rw_009eb170(this_ptr: u32) -> u32 {
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
        let slot = *((this_ptr + 0xb9c) as *const u32);
        let resolved: u32 = callee_cdecl!(2, u32, slot);
        let flags = global::<u32>(0x12b6100);
        let cached = global::<u32>(0x12b60fc);
        let handle = if *flags & 1 == 0 {
            *flags |= 1;
            let fresh: u32 = callee_cdecl!(3, u32, relocated(0xe98540), 0);
            *cached = fresh;
            fresh
        } else {
            *cached
        };
        let ok: u32 = callee_cdecl!(4, u32, resolved, handle);
        if ok == 0 {
            return 0;
        }
        let inner = *((this_ptr + 0x78) as *const u32);
        let cached_now = *cached;
        callee_thiscall!(
            5, u32, inner,
            resolved, cached_now, 0x48000, 6, 30.0f32.to_bits()
        )
    }
});
