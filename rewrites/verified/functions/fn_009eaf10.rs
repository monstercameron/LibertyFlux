// original: 0x009eaf10 ped_guarded_notify_dual
/// Guarded notify with two lazily initialised shared handles.
///
/// Same guards as its siblings, plus a mode argument valid over 0..=1
/// (anything else reports through the shared reporter and continues).
/// Resolves the slot at `0xb9c`, then initialises each shared handle on
/// first use under flag bits 1 and 2 at `0x12b60f4`, caching them at
/// `0x12b60f0` and `0x12b60f8` (the second initialisation re-reads the
/// first cache into the working register, which the rewrite reproduces).
/// A failed pair check returns 0, otherwise the inner object at `0x78` is
/// notified with (resolved, cached, 0x48000, 6, 30.0) and its answer is
/// returned.
export!(thiscall, rw_009eaf10(this_ptr: u32, mode: u32) -> u32 {
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
        if (mode as i32) < 0 || mode > 1 {
            let _: u32 = callee_cdecl!(2, u32,);
        }
        let slot = *((this_ptr + 0xb9c) as *const u32);
        let resolved: u32 = callee_cdecl!(3, u32, slot);
        let flags = global::<u32>(0x12b60f4);
        let cache_a = global::<u32>(0x12b60f0);
        let cache_b = global::<u32>(0x12b60f8);
        let mut cached = *cache_a;
        let mut f = *flags;
        if f & 1 == 0 {
            f |= 1;
            *flags = f;
            let fresh: u32 = callee_cdecl!(4, u32, relocated(0xe98474));
            *cache_a = fresh;
            cached = fresh;
        }
        f = *flags;
        if f & 2 == 0 {
            f |= 2;
            *flags = f;
            let fresh: u32 = callee_cdecl!(4, u32, relocated(0xe9847c));
            cached = *cache_a;
            *cache_b = fresh;
        }
        let ok: u32 = callee_cdecl!(5, u32, resolved, cached);
        if ok == 0 {
            return 0;
        }
        let inner = *((this_ptr + 0x78) as *const u32);
        callee_thiscall!(
            6, u32, inner,
            resolved, cached, 0x48000, 6, 30.0f32.to_bits()
        )
    }
});
