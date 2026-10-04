// original: 0x00947820 scale_pair_apply_gated
/// Scale a gated filtered value and apply it to two adjacent records.
///
/// Takes an object pointer, two float parameters and five opaque words.
/// The second float parameter is run through a scripted unary filter
/// helper and clamped from below at a small positive floor (a NaN filter
/// answer takes the floor). When the tag byte at object+0x1918 equals the
/// scripted data tag, the clamped value is divided by a scripted divisor:
/// a gate pointer selects the scale (scripted data scale when the gate is
/// non-null with the word at gate+6 equal to 2, else the constant 1.0)
/// and a truncated integer key derived from a scripted data factor times
/// 1000.0 (out-of-range or NaN products yield 0, matching the original's
/// truncating x87 conversion) goes with the scale to a scripted binary
/// helper; otherwise the clamped value passes through unchanged. The
/// result goes through a second scripted unary helper, a scripted data
/// addend and the first float parameter are added in that order, and the
/// final value is passed to a scripted six-word helper twice - once for
/// the object and once for the record 0xBD0 bytes past it - together with
/// the five opaque words in the order (a0, a4, a5, a3, a6). Returns
/// nothing meaningful (the original leaves the last helper answer in eax,
/// which is not behaviour).
export!(thiscall, rw_00947820(
    obj: u32,
    a0: u32,
    a1: f32,
    a2: f32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
) -> u32 {
    unsafe {
        const FILTER_ID: u32 = 1;
        const FILTER_THIS: u32 = 0x11D7650;
        const FLOOR_GLOB: u32 = 0xFE8670;
        const TAG_OFF: u32 = 0x1918;
        const TAG_GLOB: u32 = 0x1284644;
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
        const I64_LIMIT: f32 = 9223372036854775808.0;

        let filtered: f32 =
            callee_thiscall!(FILTER_ID, f32, relocated(FILTER_THIS), a2.to_bits());
        let floor = *(relocated(FLOOR_GLOB) as *const f32);
        // Original is comiss+ja: only a strictly greater ordered value is
        // kept, so NaN takes the floor.
        let clamped = if filtered > floor { filtered } else { floor };
        let tag = *((obj.wrapping_add(TAG_OFF)) as *const u8);
        let want = *(global::<u32>(TAG_GLOB));
        let base = if (tag as u32) == want {
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
            let product = *(global::<f32>(FACTOR_GLOB))
                * *(relocated(THOUSAND_GLOB) as *const f32);
            let key: u32 = if product > -I64_LIMIT && product < I64_LIMIT {
                (product as i64) as i32 as u32
            } else {
                0
            };
            let divisor: f32 = callee_thiscall!(
                COMBINE_ID,
                f32,
                relocated(COMBINE_THIS),
                scale.to_bits(),
                key
            );
            clamped / divisor
        } else {
            clamped
        };
        let adjusted: f32 = callee_cdecl!(ADJUST_ID, f32, base.to_bits());
        // Addition order matches the original: addend first, then a1.
        let value = (adjusted + *(global::<f32>(ADDEND_GLOB))) + a1;
        let mut this = obj;
        let mut i = 0;
        while i < 2 {
            let _: u32 = callee_thiscall!(
                APPLY_ID,
                u32,
                this,
                a0,
                value.to_bits(),
                a4,
                a5,
                a3,
                a6
            );
            this = this.wrapping_add(RECORD_STRIDE);
            i += 1;
        }
        0
    }
});
