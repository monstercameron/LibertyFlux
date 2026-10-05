// original: 0x00982210 audio_emitter_lookup_and_forward
/// Convert-and-select: truncate a float, resolve the selector, hand off.
///
/// Takes the object pointer in ECX and one single-precision word on the
/// stack. Resolves the selector exactly like its table sibling (null when
/// the kind byte reads all set, otherwise scale times kind plus the
/// selected table word), truncates the float toward zero to a wide
/// integer exactly as the coprocessor store does (out-of-range positives
/// and the indefinite form all read back as zero in the low half), and
/// transfers control to the shared successor with both results. The
/// incoming slot is overwritten in place, so the stack comparison is off
/// and both results are compared through the call log instead.
export!(thiscall, rw_00982210(this: u32, fbits: u32) -> u32 {
    unsafe {
        let kind = *(this.wrapping_add(4) as *const u8);
        let sel = if kind == 0xFF {
            0
        } else {
            let row = *(this.wrapping_add(0x40) as *const u8);
            let scale = *(global::<u32>(0x0115D968) as *const u32);
            let base = *(global::<u32>(0x0115D988) as *const u32);
            let slot = (row as u32).wrapping_mul(0x6F40);
            let word = *((base.wrapping_add(slot).wrapping_add(0x6F14)) as *const u32);
            scale.wrapping_mul(kind as u32).wrapping_add(word)
        };
        let f = f32::from_bits(fbits);
        let conv = if f >= 9223372036854775808.0 {
            0u32
        } else {
            (f as i64) as u32
        };
        callee_thiscall!(1, u32, sel, conv)
    }
});
