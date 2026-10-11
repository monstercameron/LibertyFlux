// original: 0x00a2d7f0 SaveHouse_RegisterInternal

/// Append a save-house record containing four raw words from a vector source,
/// one opaque context word, eight words produced by a scripted helper, a house
/// identifier, and a ready byte. The count selects a 64-byte slot; the previous
/// count is returned. The helper receives the vector pointer in ECX, the record
/// output pointer, a caller-supplied auxiliary word, and the fixed byte count.
lf_checker_rt::export!(cdecl, rw_a2d7f0(vector: u32, context_bits: u32, auxiliary: u32, house_id: u32) -> u32 {
    const COUNT_VA: u32 = 0x12dd738;
    const RECORDS_VA: u32 = 0x12ddb00;
    let count_ptr = lf_checker_rt::global::<u32>(COUNT_VA);
    let count = unsafe { count_ptr.read_unaligned() };
    let record = unsafe { lf_checker_rt::global::<u32>(RECORDS_VA).add(count.wrapping_mul(16) as usize) };
    let source = vector as *const u32;
    unsafe {
        for index in 0..4 {
            record.add(index).write_unaligned(source.add(index).read_unaligned());
        }
        record.add(4).write_unaligned(context_bits);
    }
    let output = unsafe { record.add(5) };
    let _ignored = unsafe { lf_checker_rt::callee_thiscall!(1, u32, vector, output as u32, auxiliary, 0x20u32) };
    unsafe {
        record.add(13).write_unaligned(house_id);
        (record.add(14) as *mut u8).write_unaligned(1);
        count_ptr.write_unaligned(count.wrapping_add(1));
    }
    count
});
