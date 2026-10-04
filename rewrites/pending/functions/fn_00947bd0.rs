// original: 0x00947bd0 scale_pair_apply
/// Scale a filtered value and apply it to two adjacent records.
///
/// Takes an object pointer plus two float parameters. The first parameter
/// is run through a scripted unary filter helper, then clamped from below
/// at a small positive floor (a NaN filter answer takes the floor). A
/// gate pointer selects the divisor scale: when the gate is non-null and
/// the word at gate+6 equals 2 the scripted data scale is used, otherwise
/// the constant 1.0. A truncated integer key is derived from a scripted
/// data factor times 1000.0 (out-of-range or NaN products yield 0, matching
/// the original's truncating x87 conversion): the scale and the key go to
/// a scripted binary helper whose answer divides the clamped value, that
/// quotient goes through a second scripted unary helper, and a scripted
/// data addend is added. The final value, together with the second
/// parameter unchanged, is then passed to a scripted pair helper twice,
/// once for the object and once for the record 0xBD0 bytes past it.
/// Returns nothing meaningful (the original leaves the last helper answer
/// in eax, which is not behaviour).
export!(thiscall, rw_00947bd0(obj: u32, a0: f32, a1: f32) -> u32 {
    unsafe {
        const FILTER_ID: u32 = 1;
        const FILTER_THIS: u32 = 0x11D7650;
        const FLOOR_GLOB: u32 = 0xFE8670;
        const GATE_GLOB: u32 = 0x11D74FC;
        const GATE_WORD_OFF: u32 = 6;
        const GATE_WORD_WANT: u16 = 2;
        const SCALE_GLOB: u32 = 0x11D763C;
        const ONE_GLOB: u32 = 0xFE88E8;
        const FACTOR_GLOB: u32 = 0x115DBF4;
        const THOUSAND_GLOB: u32 = 0xFE8C58;
        const COMBINE_ID: u32 = 2;
        const COMBINE_THIS: u32 = 0x11D76B8;
        const ADJUST_ID: u32 = 3;
        const ADDEND_GLOB: u32 = 0x12845F4;
        const APPLY_ID: u32 = 4;
        const RECORD_STRIDE: u32 = 0xBD0;
        // 2^63 as f32: products outside (-2^63, +2^63) overflow the
        // original's 64-bit truncating conversion and yield 0, as do NaN
        // and infinities (the x87 indefinite value's low word is 0).
        const I64_LIMIT: f32 = 9223372036854775808.0;

        let filtered: f32 =
            callee_thiscall!(FILTER_ID, f32, relocated(FILTER_THIS), a0.to_bits());
        let floor = *(relocated(FLOOR_GLOB) as *const f32);
        // Original is comiss+ja: only a strictly greater ordered value is
        // kept, so NaN takes the floor.
        let clamped = if filtered > floor { filtered } else { floor };
        let gate = *(global::<u32>(GATE_GLOB));
        let scale = if gate != 0
            && core::ptr::read_unaligned(
                (gate.wrapping_add(GATE_WORD_OFF)) as *const u16,
            ) == GATE_WORD_WANT
        {
            *(global::<f32>(SCALE_GLOB))
        } else {
            *(relocated(ONE_GLOB) as *const f32)
        };
        let product =
            *(global::<f32>(FACTOR_GLOB)) * *(relocated(THOUSAND_GLOB) as *const f32);
        let key: u32 = if product > -I64_LIMIT && product < I64_LIMIT {
            (product as i64) as i32 as u32
        } else {
            0
        };
        // Argument order is (scale, key): the key is pushed first.
        let divisor: f32 =
            callee_thiscall!(COMBINE_ID, f32, relocated(COMBINE_THIS), scale.to_bits(), key);
        let quotient = clamped / divisor;
        let adjusted: f32 = callee_cdecl!(ADJUST_ID, f32, quotient.to_bits());
        let value = adjusted + *(global::<f32>(ADDEND_GLOB));
        let mut this = obj;
        let mut i = 0;
        while i < 2 {
            let _: u32 = callee_thiscall!(
                APPLY_ID,
                u32,
                this,
                value.to_bits(),
                a1.to_bits()
            );
            this = this.wrapping_add(RECORD_STRIDE);
            i += 1;
        }
        0
    }
});
