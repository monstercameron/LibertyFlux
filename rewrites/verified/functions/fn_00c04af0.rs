// original: 0x00C04AF0 stream_emit_6

/// Returns without calls for a null parameter pointer. Otherwise, copies all
/// four words written by the first helper into the record, adds two owner
/// floats, and stores the normalized flag as one byte at record offset 40.
/// The second call skips its frame pointer and snapshots all eleven record
/// words, including the fourth helper output and the flag word. The defined
/// zero stack fill makes the three bytes beside the flag deterministic.
/// Both helper implementations are stubbed; their outputs are contract
/// controlled and the helper return values and wrapper's void return are
/// ignored.
unsafe fn emit_record(this: u32, params: u32) {
    let mut helper_output = [0u32; 4];
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, helper_output.as_mut_ptr() as u32);

    let mut record = [0u8; 44];
    let record_ptr = record.as_mut_ptr();
    record_ptr.cast::<u32>().write_unaligned(params);
    for (index, word) in helper_output.iter().copied().enumerate() {
        record_ptr.add(16 + index * 4).cast::<u32>().write_unaligned(word);
    }

    let owner_bytes = this as *const u8;
    let first_float = owner_bytes.add(0x18).cast::<u32>().read_unaligned();
    let second_float = owner_bytes.add(0x1c).cast::<u32>().read_unaligned();
    record_ptr.add(32).cast::<u32>().write_unaligned(first_float);
    record_ptr.add(36).cast::<u32>().write_unaligned(second_float);
    let flag_byte = owner_bytes.add(0x20).read();
    record_ptr.add(40).write((flag_byte >> 5) & 1);

    let _ = lf_checker_rt::callee_thiscall!(
        2,
        u32,
        lf_checker_rt::relocated(0x01305d30),
        record_ptr as u32,
        1
    );
}

lf_checker_rt::export!(thiscall, rw_00c04af0(this: u32, params: u32) -> () {
    if params == 0 {
        return;
    }
    unsafe { emit_record(this, params); }
});
