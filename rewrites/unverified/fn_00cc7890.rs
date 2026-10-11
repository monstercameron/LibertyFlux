// original: 0x00CC7890 physics_feature_type_is_supported (proposed)

/// Returns whether a physics feature object has an allowed signed type code.
/// The receiver is an eight-byte vtable-backed object whose low flag at `+0x0a`
/// excludes one mode. The first stack argument points to a record: its byte at
/// `+0x24` excludes three record modes, and its signed 32-bit type at `+0x7b8`
/// must be in the inclusive range 2 through 6. The second stack word is unused.
/// The result is a one-byte boolean and the thiscall callee removes both stack
/// arguments.
lf_checker_rt::export!(thiscall, rw_00cc7890(this: u32, feature: u32, _unused: u32) -> u8 {
    const RECEIVER_FLAGS: u32 = 0x0a;
    const RECORD_FLAGS: u32 = 0x24;
    const SIGNED_TYPE: u32 = 0x7b8;
    const RECEIVER_EXCLUDED: u8 = 0x08;
    const RECORD_EXCLUDED: u8 = 0x1c;
    const MIN_TYPE: i32 = 2;
    const MAX_TYPE: i32 = 6;

    unsafe {
        let receiver_flags = (this.wrapping_add(RECEIVER_FLAGS) as *const u8).read();
        let record_flags = (feature.wrapping_add(RECORD_FLAGS) as *const u8).read();
        let type_code = (feature.wrapping_add(SIGNED_TYPE) as *const i32).read_unaligned();
        u8::from(
            receiver_flags & RECEIVER_EXCLUDED == 0
                && record_flags & RECORD_EXCLUDED == 0
                && (MIN_TYPE..=MAX_TYPE).contains(&type_code),
        )
    }
});
