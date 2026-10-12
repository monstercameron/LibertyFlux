// original: 0x00A2FD00 EntityScannerConstruct

/// Initializes an entity scanner record from its constructor arguments.
///
/// The thiscall entry receives the record in ECX and four stack arguments:
/// option bits, a stored value, an optional four-word payload pointer, and a
/// radius. It clears the base payload fields, stores the squared radius using
/// the original single-precision operand order, and copies the optional
/// payload only when its pointer is non-null. In that case it also sets the
/// payload-present option bit. The record pointer is returned unchanged.
lf_checker_rt::export!(thiscall, rw_00a2fd00(
    record: u32,
    options: u32,
    stored_value: u32,
    payload: u32,
    radius: f32
) -> u32 {
    const TYPE_WORD_OFFSET: usize = 0x00;
    const OPTIONS_WORD_OFFSET: usize = 0x04;
    const ZERO_WORD_OFFSET: usize = 0x08;
    const STORED_VALUE_OFFSET: usize = 0x0C;
    const RADIUS_SQUARED_OFFSET: usize = 0x10;
    const PAYLOAD_FIRST_OFFSET: usize = 0x20;
    const PAYLOAD_SECOND_OFFSET: usize = 0x24;
    const PAYLOAD_THIRD_OFFSET: usize = 0x28;
    const PAYLOAD_FOURTH_OFFSET: usize = 0x2C;
    const PAYLOAD_PRESENT: u32 = 0x80;
    const TYPE_SENTINEL: u32 = u32::MAX;

    unsafe {
        let record_ptr = record as *mut u8;
        record_ptr
            .add(PAYLOAD_FIRST_OFFSET)
            .cast::<u32>()
            .write_unaligned(0);
        record_ptr
            .add(PAYLOAD_SECOND_OFFSET)
            .cast::<u32>()
            .write_unaligned(0);
        record_ptr
            .add(PAYLOAD_THIRD_OFFSET)
            .cast::<u32>()
            .write_unaligned(0);

        let squared_radius =
            core::hint::black_box(radius) * core::hint::black_box(radius);

        record_ptr
            .add(OPTIONS_WORD_OFFSET)
            .cast::<u32>()
            .write_unaligned(options);
        record_ptr
            .add(ZERO_WORD_OFFSET)
            .cast::<u32>()
            .write_unaligned(0);
        record_ptr
            .add(TYPE_WORD_OFFSET)
            .cast::<u32>()
            .write_unaligned(TYPE_SENTINEL);
        record_ptr
            .add(STORED_VALUE_OFFSET)
            .cast::<u32>()
            .write_unaligned(stored_value);
        record_ptr
            .add(RADIUS_SQUARED_OFFSET)
            .cast::<f32>()
            .write_unaligned(squared_radius);

        if payload != 0 {
            let updated_options = options | PAYLOAD_PRESENT;
            record_ptr
                .add(OPTIONS_WORD_OFFSET)
                .cast::<u32>()
                .write_unaligned(updated_options);

            let source = payload as *const u8;
            let first = source.cast::<u32>().read_unaligned();
            let second = source.add(4).cast::<u32>().read_unaligned();
            let third = source.add(8).cast::<u32>().read_unaligned();
            record_ptr
                .add(PAYLOAD_FIRST_OFFSET)
                .cast::<u32>()
                .write_unaligned(first);
            record_ptr
                .add(PAYLOAD_SECOND_OFFSET)
                .cast::<u32>()
                .write_unaligned(second);
            record_ptr
                .add(PAYLOAD_THIRD_OFFSET)
                .cast::<u32>()
                .write_unaligned(third);
            let fourth = source.add(12).cast::<u32>().read_unaligned();
            record_ptr
                .add(PAYLOAD_FOURTH_OFFSET)
                .cast::<u32>()
                .write_unaligned(fourth);
        }
        record
    }
});
