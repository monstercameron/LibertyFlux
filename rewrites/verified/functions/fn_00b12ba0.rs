// original: 0x00B12BA0 CSmashMan_ResetDrawBuckets::CSmashMan_ResetDrawBuckets
/// Initialize one draw-bucket record. The object arrives in ECX and the stack
/// argument is stored at byte offset eight. The low fourteen bits at offset
/// four are replaced with the current global bucket sequence; its high bits
/// are preserved. The global sequence increments with 32-bit wrapping, and
/// the object's final virtual table is installed before returning the object.
lf_checker_rt::export!(thiscall, rw_00b12ba0(object: u32, draw_bucket_value: u32) -> u32 {
    const BUCKET_FIELD: u32 = 4;
    const DRAW_VALUE_FIELD: u32 = 8;
    const BUCKET_SEQUENCE_FILE_VA: u32 = 0x010327A0;
    const BUCKET_MASK: u32 = 0x3FFF;
    const DRAW_BUCKET_VTABLE_FILE_VA: u32 = 0x00EAB0BC;

    unsafe {
        let bucket_field = object.wrapping_add(BUCKET_FIELD) as *mut u32;
        let old_value = bucket_field.read_volatile();
        let sequence = lf_checker_rt::global::<u32>(BUCKET_SEQUENCE_FILE_VA).read_volatile();
        let new_low_bits = (old_value ^ sequence) & BUCKET_MASK;
        bucket_field.write_volatile(old_value ^ new_low_bits);

        let counter = lf_checker_rt::global::<u32>(BUCKET_SEQUENCE_FILE_VA);
        counter.write_volatile(sequence.wrapping_add(1));
        (object.wrapping_add(DRAW_VALUE_FIELD) as *mut u32).write_volatile(draw_bucket_value);
        (object as *mut u32).write_volatile(lf_checker_rt::relocated(DRAW_BUCKET_VTABLE_FILE_VA));
        object
    }
});
