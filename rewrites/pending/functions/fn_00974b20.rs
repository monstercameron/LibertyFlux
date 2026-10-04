// original: 0x00974B20 audio_source_init
/// Initialise an audio source object with default parameters.
///
/// `base_arg` is forwarded to the base-class initialiser (callee 1) and kept
/// in the object; `level_a`/`level_b` are float gains whose squares seed two
/// envelope fields; `flags`/`extra` are stored as opaque words. The five
/// sub-objects hanging off the source are each initialised through callee 2
/// with `(rate, rate, 0.0, 1.0)` or `(0.01, 0.01, 0.0, 1.0)`, where each rate
/// is a live global scaled by the millisecond constant.
export!(thiscall, rw_00974B20(this_ptr: u32, base_arg: u32, level_a: u32, flags: u32, extra: u32, level_b: u32) -> u32 {
    unsafe {
        const UNIT: u32 = 0x3F80_0000; // 1.0f32
        const SMALL: u32 = 0x3C23_D70A; // 0.01f32
        // Base-class portion first; it consumes the first argument.
        let _: u32 = callee_thiscall!(1, u32, this_ptr, base_arg);
        let w32 = |off: u32, val: u32| {
            *((this_ptr.wrapping_add(off)) as *mut u32) = val;
        };
        // Default the gain triplets: 1.0 on the diagonal, 0.0 elsewhere.
        w32(0x40, UNIT);
        w32(0x44, 0);
        w32(0x48, 0);
        w32(0x50, 0);
        w32(0x54, UNIT);
        w32(0x58, 0);
        w32(0x60, 0);
        w32(0x64, 0);
        w32(0x68, UNIT);
        w32(0x78, 0);
        w32(0x74, 0);
        w32(0x70, 0);
        // First sub-object runs at the scaled live rate.
        let rate_a = *(global::<f32>(0x0103_8860)) * *(global::<f32>(0x00FE_86B4));
        let k = rate_a.to_bits();
        let _: u32 = callee_thiscall!(2, u32, this_ptr.wrapping_add(0x80), k, k, 0, UNIT);
        let amp_a = f32::from_bits(level_a);
        let amp_b = f32::from_bits(level_b);
        *((this_ptr.wrapping_add(0xA0)) as *mut u8) = 0;
        w32(0x9C, (amp_a * amp_a).to_bits());
        w32(0xAC, 0);
        w32(0xB8, 0);
        w32(0xB4, 0);
        w32(0xB0, 0);
        w32(0xC0, flags);
        w32(0xC4, extra);
        w32(0xC8, (amp_b * amp_b).to_bits());
        w32(0xCC, 0);
        // Two fixed-rate sub-objects.
        let _: u32 = callee_thiscall!(2, u32, this_ptr.wrapping_add(0xD0), SMALL, SMALL, 0, UNIT);
        let _: u32 = callee_thiscall!(2, u32, this_ptr.wrapping_add(0xEC), SMALL, SMALL, 0, UNIT);
        // Clear the tail state.
        w32(0x108, 0);
        w32(0x10C, 0);
        w32(0x110, 0);
        *((this_ptr.wrapping_add(0xA8)) as *mut u8) = 0;
        w32(0xA4, 0);
        w32(0x114, 0);
        w32(0x118, 0);
        *((this_ptr.wrapping_add(0x34)) as *mut u16) = 0;
        *((this_ptr.wrapping_add(0x11C)) as *mut u8) = 0;
        // Two sub-objects at the scaled secondary rate, re-read per site.
        let rate_b = *(global::<f32>(0x0103_8864)) * *(global::<f32>(0x00FE_86B4));
        let k2 = rate_b.to_bits();
        let _: u32 = callee_thiscall!(2, u32, this_ptr.wrapping_add(0x120), k2, k2, 0, UNIT);
        let rate_b = *(global::<f32>(0x0103_8864)) * *(global::<f32>(0x00FE_86B4));
        let k2 = rate_b.to_bits();
        let answer: u32 = callee_thiscall!(2, u32, this_ptr.wrapping_add(0x13C), k2, k2, 0, UNIT);
        *((this_ptr.wrapping_add(0xAA)) as *mut u8) = 0;
        answer
    }
});
