// original: 0x00976400 audio_entity_loudness_resolve
/// Resolve an audio entity's loudness pair (integer and filtered level).
///
/// `entity` selects through the shared sample object: when null, both outputs
/// keep their defaults (0 and -100.0). Otherwise the input level is scaled by
/// a global divisor, clamped below by a global floor, and resolved twice
/// through one of two evaluator objects: attached sub-evaluators on `this`
/// when the global selector is set, else a temporary stack evaluator keyed by
/// words from the entity (+0x2f, then +0x33). The first resolution is mapped
/// through a transfer helper and scaled into the integer output; the second
/// is filtered into the float output (directly on the attached path, through
/// one more filter stage on the temporary path). Returns nothing meaningful
/// (the original leaves eax incidental; verified with ret:none).
export!(thiscall, rb37_976400(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        *(a3 as *mut u32) = 0;
        *(a4 as *mut u32) = 0xc2c80000;
        if a2 == 0 {
            cookie();
            return 0;
        }
        let div = f32::from_bits(*global::<u32>(0x10388e0));
        let floor = f32::from_bits(*global::<u32>(0xfe88e8));
        let scaled = f32::from_bits(a1) / div;
        let v = if floor > scaled { scaled } else { floor };
        let vbits = v.to_bits();
        let flag = *global::<u8>(0x10388dc);
        // First resolution -> integer output.
        if flag != 0 {
            let r: f32 = callee_thiscall!(1, f32, this.wrapping_add(0x10f38), vbits);
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
        // Second resolution -> float output.
        if flag != 0 {
            let r: f32 = callee_thiscall!(1, f32, this.wrapping_add(0x10f10), vbits);
            *(a4 as *mut u32) = r.to_bits();
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
            let r: f32 = callee_thiscall!(2, f32, obj, vbits);
            let f: f32 = callee_cdecl!(6, f32, r.to_bits());
            *(a4 as *mut u32) = f.to_bits();
        }
        cookie();
        0
    }
});
