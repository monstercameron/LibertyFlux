// original: 0x009edad0 ped_scaled_rate_store
/// Resolve a target, notify it, and store a scaled rate on it.
///
/// Resolves the slot at `0xb9c` through the shared resolver; a negative
/// answer or a null first argument returns that answer. A failed pair
/// check against the first argument returns 0. Otherwise the inner object
/// at `0x78` is notified with (resolved, arg0, 0x48000, 6, rate); a null
/// notification or a null second argument returns that answer. Finally it
/// reads a float from the target, scales it, divides by the second
/// argument, stores the quotient at target+`0x54`, and returns the float's
/// own bits (the value the original carries in `eax` there).
export!(thiscall, rw_009edad0(
    this_ptr: u32,
    arg0: u32,
    arg1: u32,
    _arg2: u32,
    _arg3: u32,
) -> u32 {
    unsafe {
        let shared = *global::<u32>(0x16dd63c);
        let slot = *((this_ptr + 0xb9c) as *const u32);
        let mut target: u32 = callee_thiscall!(2, u32, shared, slot);
        if (target as i32) < 0 {
            return target;
        }
        if arg0 == 0 {
            return target;
        }
        let ok: u32 = callee_cdecl!(3, u32, target, arg0);
        if ok == 0 {
            return 0;
        }
        let rate = f32::from_bits(*global::<u32>(0x103b394));
        let inner = *((this_ptr + 0x78) as *const u32);
        target = callee_thiscall!(4, u32, inner, target, arg0, 0x48000, 6, rate.to_bits());
        if target == 0 {
            return 0;
        }
        if arg1 == 0 {
            return target;
        }
        let sample: f32 = callee_thiscall!(5, f32, target);
        let scaled = sample * f32::from_bits(*global::<u32>(0xfe8c58));
        let quotient = scaled / (arg1 as f32);
        *((target + 0x54) as *mut u32) = quotient.to_bits();
        sample.to_bits()
    }
});
