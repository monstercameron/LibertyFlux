// original: 0x006845E0 rage::crFrameDofInt::vf3

/// Select a frame row from the timeline using signed truncation of
/// `frame / frame_count`, capped only at the signed upper bound `length - 1`.
/// Negative indices remain negative and address the preceding table entries.
/// The remainder `frame - index * frame_count` is sent to the selected
/// object's virtual setter together with this record's value field, then the
/// record's update bit is cleared. This thiscall method takes a float frame
/// and a timeline pointer and has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_006845e0(this: u32, frame_bits: u32, timeline: u32) -> u32 {
    unsafe {
        const FRAME_COUNT_OFFSET: u32 = 4;
        const ROWS_OFFSET: u32 = 8;
        const LENGTH_OFFSET: u32 = 12;
        const FRAME_VALUE_OFFSET: u32 = 16;
        const ROW_OBJECT_OFFSET: u32 = 4;
        const INTEGER_SETTER_SLOT: u32 = 20;
        const UPDATE_BIT: u8 = 0x10;

        let frame = f32::from_bits(frame_bits);
        let frame_count = crate::read_halfword(timeline.wrapping_add(FRAME_COUNT_OFFSET)) as i32;
        let frame_count_float = frame_count as f32;
        let ratio = core::hint::black_box(frame) / core::hint::black_box(frame_count_float);
        let mut frame_index = k3probe(1, false, ratio) as i32;
        let length = crate::read_halfword(timeline.wrapping_add(LENGTH_OFFSET)) as i32;
        let maximum_index = length.wrapping_sub(1);
        if frame_index >= maximum_index {
            frame_index = maximum_index;
        }

        let rows = crate::read_word(timeline.wrapping_add(ROWS_OFFSET));
        let row_address = rows.wrapping_add((frame_index as u32).wrapping_mul(4));
        let row = crate::read_word(row_address);
        let target = crate::read_word(row.wrapping_add(ROW_OBJECT_OFFSET));
        let target_vtable = crate::read_word(target);
        let setter_slot = crate::read_word(target_vtable.wrapping_add(INTEGER_SETTER_SLOT));

        let stride = frame_count.wrapping_mul(frame_index) as f32;
        let remainder = core::hint::black_box(frame) - core::hint::black_box(stride);
        let value_output = this.wrapping_add(FRAME_VALUE_OFFSET);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, target, remainder.to_bits(), value_output);

        let flags = crate::read_byte(this.wrapping_add(4));
        crate::write_byte(this.wrapping_add(4), flags & !UPDATE_BIT);
        let _ = setter_slot;
        0
    }
});
