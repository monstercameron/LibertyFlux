// original: 0x00877430 copy_orientation_record_fields

/// Initialize an orientation output record from its source fields.
///
/// First invoke the orientation setup helper with the output pointer in ECX
/// and a pointer to the source's embedded data at `+0x10` in EDX. Then copy
/// four raw words from source offsets `+0x00`, `+0x04`, `+0x08`, and `+0x0c`
/// to destination offsets `+0x30`, `+0x34`, `+0x38`, and `+0x3c`. The two
/// middle words are float payloads copied without arithmetic. Return the
/// destination pointer.
///
/// Original calling convention: thiscall with one stack destination pointer.
lf_checker_rt::export!(thiscall, rw_00877430(this: u32, destination: u32) -> u32 {
    const SOURCE_OFFSETS: [u32; 4] = [0x00, 0x04, 0x08, 0x0c];
    const DESTINATION_OFFSETS: [u32; 4] = [0x30, 0x34, 0x38, 0x3c];
    unsafe {
        let _setup_result = lf_checker_rt::callee_fastcall!(1, u32, destination, this.wrapping_add(0x10));
        for (source_offset, destination_offset) in SOURCE_OFFSETS.into_iter().zip(DESTINATION_OFFSETS) {
            let raw_word = (this.wrapping_add(source_offset) as *const u32).read_unaligned();
            (destination.wrapping_add(destination_offset) as *mut u32).write_unaligned(raw_word);
        }
    }
    destination
});
