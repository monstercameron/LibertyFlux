// original: 0x00976a30 audio_entity_loudness_resolve_gated
/// Resolve an audio entity's loudness pair, threshold-gated variant.
///
/// Same family as `rb37_976400`: the input level and entity first pass a
/// threshold helper, and levels at or below the global threshold (or
/// unordered) keep the default outputs. Above threshold, two evaluator
/// resolutions follow (attached +0x10e98/+0x10e70 when the global selector is
/// set, else temporary stack evaluators keyed by entity words +0x27/+0x2b).
/// The first resolution goes through a magic-constant rounding sequence and
/// an x87 truncation to the integer output; the second shares one call site
/// across both paths and a filter stage into the float output. a3 takes the
/// integer, a4 the float. Returns nothing meaningful (ret:none).
export!(thiscall, rb37_976a30(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        *(a3 as *mut u32) = 0;
        *(a4 as *mut u32) = 0xc2c80000;
        if a2 == 0 {
            cookie();
            return 0;
        }
        let v0: f32 = callee_stdcall!(9, f32, a1, a2);
        let thresh = f32::from_bits(*global::<u32>(0xfe8628));
        if !(v0 > thresh) {
            cookie();
            return 0;
        }
        let flag = *global::<u8>(0x10388dc);
        if flag != 0 {
            let r: f32 = callee_thiscall!(1, f32, this.wrapping_add(0x10e98), v0.to_bits());
            *(a3 as *mut i32) = round_trunc(r);
        } else {
            let mut buf1 = [0u32; 12];
            let obj = buf1.as_mut_ptr() as u32;
            let _: u32 = callee_thiscall!(3, u32, obj);
            let key = core::ptr::read_unaligned((a2 as *const u8).add(0x27) as *const u32);
            let ok: u32 = callee_thiscall!(4, u32, obj, key);
            if ok != 0 {
                let r: f32 = callee_thiscall!(2, f32, obj, v0.to_bits());
                *(a3 as *mut i32) = round_trunc(r);
            }
        }
        if flag != 0 {
            let r: f32 = callee_thiscall!(8, f32, this.wrapping_add(0x10e70), v0.to_bits());
            let f: f32 = callee_cdecl!(6, f32, r.to_bits());
            *(a4 as *mut u32) = f.to_bits();
        } else {
            let mut buf2 = [0u32; 12];
            let obj = buf2.as_mut_ptr() as u32;
            let _: u32 = callee_thiscall!(3, u32, obj);
            let key = core::ptr::read_unaligned((a2 as *const u8).add(0x2b) as *const u32);
            let ok: u32 = callee_thiscall!(4, u32, obj, key);
            if ok == 0 {
                cookie();
                return 0;
            }
            let r: f32 = callee_thiscall!(8, f32, obj, v0.to_bits());
            let f: f32 = callee_cdecl!(6, f32, r.to_bits());
            *(a4 as *mut u32) = f.to_bits();
        }
        cookie();
        0
    }
});
