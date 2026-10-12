// original: 0x008773B0 copy_frame_sample_vectors

/// Copy the three vector components of each 16-byte frame sample.
///
/// `start` and `end` are source pointers to 16-byte records. Each iteration
/// copies the first twelve bytes and leaves the fourth word untouched in the
/// destination. A null destination suppresses stores but still advances with
/// the source. The returned pointer is the destination advanced once per
/// record. The record count is determined by pointer equality, not ordering.
///
/// Original calling convention: fastcall, with start in ECX, end in EDX, and
/// the destination pointer on the stack.
lf_checker_rt::export!(fastcall, rw_008773b0(start: u32, end: u32, destination: u32) -> u32 {
    const RECORD_STRIDE: u32 = 16;
    const COMPONENT_OFFSETS: [u32; 3] = [0, 4, 8];
    let mut source_record = start;
    let mut destination_record = destination;
    unsafe {
        while source_record != end {
            if destination_record != 0 {
                for component_offset in COMPONENT_OFFSETS {
                    let component = (source_record.wrapping_add(component_offset) as *const u32)
                        .read_unaligned();
                    (destination_record.wrapping_add(component_offset) as *mut u32)
                        .write_unaligned(component);
                }
            }
            source_record = source_record.wrapping_add(RECORD_STRIDE);
            destination_record = destination_record.wrapping_add(RECORD_STRIDE);
        }
    }
    destination_record
});
