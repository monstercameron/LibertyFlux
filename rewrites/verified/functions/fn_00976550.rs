// original: 0x00976550 audio_entity_loudness_resolve_scaled
/// Resolve an audio entity's loudness pair, byte-scaled variant.
///
/// Same shape as `rb37_976400` with three differences: the input level is
/// scaled by the entity's own byte at +0x1d (as a float divisor) instead of
/// the global divisor, the attached sub-evaluators sit at +0x10ee8/+0x10ec0,
/// and the second resolution always passes through the filter stage (both
/// paths converge on one evaluator call site). Out roles: a3 takes the
/// integer, a4 the filtered float. Returns nothing meaningful (ret:none).
export!(thiscall, rb37_976550(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        *(a3 as *mut u32) = 0;
        *(a4 as *mut u32) = 0xc2c80000;
        if a2 == 0 {
            cookie();
            return 0;
        }
        let div = *(a2 as *const u8).add(0x1d) as f32;
        let floor = f32::from_bits(*global::<u32>(0xfe88e8));
        let scaled = f32::from_bits(a1) / div;
        let v = if floor > scaled { scaled } else { floor };
        let vbits = v.to_bits();
        let flag = *global::<u8>(0x10388dc);
        if flag != 0 {
            let r: f32 = callee_thiscall!(1, f32, this.wrapping_add(0x10ee8), vbits);
            let m = map_level(r.to_bits());
            *(a3 as *mut i32) = cvttss2si(m);
        } else {
            let mut buf1 = [0u32; 12];
            let obj = buf1.as_mut_ptr() as u32;
            let _: u32 = callee_thiscall!(3, u32, obj);
            let key = core::ptr::read_unaligned((a2 as *const u8).add(0x2f) as *const u32);
            let ok: u32 = callee_thiscall!(4, u32, obj, key);
            if ok != 0 {
                let r: f32 = callee_thiscall!(2, f32, obj, vbits);
                let m = map_level(r.to_bits());
                *(a3 as *mut i32) = cvttss2si(m);
            }
        }
        // Second resolution: both paths share one call site (id 8) and the
        // filter stage; only the evaluator object differs.
        if flag != 0 {
            let r: f32 = callee_thiscall!(8, f32, this.wrapping_add(0x10ec0), vbits);
            let f: f32 = callee_cdecl!(6, f32, r.to_bits());
            *(a4 as *mut u32) = f.to_bits();
        } else {
            let mut buf2 = [0u32; 12];
            let obj = buf2.as_mut_ptr() as u32;
            let _: u32 = callee_thiscall!(3, u32, obj);
            let key = core::ptr::read_unaligned((a2 as *const u8).add(0x33) as *const u32);
            let ok: u32 = callee_thiscall!(4, u32, obj, key);
            if ok == 0 {
                cookie();
                return 0;
            }
            let r: f32 = callee_thiscall!(8, f32, obj, vbits);
            let f: f32 = callee_cdecl!(6, f32, r.to_bits());
            *(a4 as *mut u32) = f.to_bits();
        }
        cookie();
        0
    }
});
