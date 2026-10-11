// original: 0x00ABC290 input_ui_find_highest_record

/// Return the record with the greatest selected float field in a sorted
/// 0x20-byte record range.
///
/// The three cdecl arguments are the first record address, the exclusive end
/// address, and a field index. The selected f32 lives at record offset
/// 0x10 + index * 4. Empty and one-record ranges return the first address.
/// Larger ranges are scanned in ascending address order; only an ordered,
/// strictly greater value replaces the current choice, so ties and NaNs keep
/// the earlier record. The returned value is a pointer, not a float.
lf_checker_rt::export!(cdecl, rw_00abc290(start: u32, end: u32, field_index: u32) -> u32 {
    const RECORD_BYTES: u32 = 0x20;
    const VALUE_OFFSET: u32 = 0x10;

    let mut best_record = start;
    if start == end {
        return best_record;
    }
    let mut candidate = start.wrapping_add(RECORD_BYTES);
    if candidate == end {
        return best_record;
    }

    let field_offset = VALUE_OFFSET.wrapping_add(field_index.wrapping_mul(4));
    let mut best = unsafe {
        f32::from_bits((best_record.wrapping_add(field_offset) as *const u32).read_unaligned())
    };
    while candidate != end {
        let value = unsafe {
            f32::from_bits((candidate.wrapping_add(field_offset) as *const u32).read_unaligned())
        };
        if core::hint::black_box(value) > core::hint::black_box(best) {
            best_record = candidate;
            best = value;
        }
        candidate = candidate.wrapping_add(RECORD_BYTES);
    }
    best_record
});
