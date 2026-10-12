// original: 0x0086E4D0 Script_NativeBind
/// Scan a fastcall byte stream by its signed remaining length. Prefix-marked
/// native identifiers are looked up through the script table, failed lookups
/// are reported through the error helper, and every returned identifier is
/// written back in little-endian order. The instruction-length helper advances
/// the cursor after each item; the function returns whether every lookup was
/// successful in AL.
lf_checker_rt::export!(fastcall, rw_0086E4D0(buffer_start: u32, byte_length: u32) -> u8 {
    const NATIVE_PREFIX: u8 = b'-';
    const NATIVE_ID_OFFSET_BYTES: usize = 3;
    const ERROR_FORMAT_VA: u32 = 0x00FC_71D8;
    const LOOKUP_CALLEE: u32 = 1;
    const REPORT_CALLEE: u32 = 2;
    const INSN_LENGTH_CALLEE: u32 = 3;

    let mut cursor = buffer_start;
    let mut remaining = byte_length as i32;
    let mut all_lookups_succeeded = true;

    while remaining > 0 {
        unsafe {
            let instruction = cursor as *mut u8;
            if instruction.read_unaligned() == NATIVE_PREFIX {
                let native_id = (instruction.add(NATIVE_ID_OFFSET_BYTES) as *const u32)
                    .read_unaligned();
                let resolved_id = lf_checker_rt::callee_stdcall!(
                    LOOKUP_CALLEE,
                    u32,
                    native_id
                );
                if resolved_id == 0 {
                    let _ = lf_checker_rt::callee_cdecl!(
                        REPORT_CALLEE,
                        u32,
                        lf_checker_rt::global::<u8>(ERROR_FORMAT_VA) as u32,
                        native_id,
                        0
                    );
                    all_lookups_succeeded = false;
                }
                (instruction.add(NATIVE_ID_OFFSET_BYTES) as *mut u32)
                    .write_unaligned(resolved_id);
            }

            let instruction_bytes = lf_checker_rt::callee_thiscall!(
                INSN_LENGTH_CALLEE,
                u32,
                cursor
            );
            remaining = remaining.wrapping_sub(instruction_bytes as i32);
            cursor = cursor.wrapping_add(instruction_bytes);
        }
    }

    u8::from(all_lookups_succeeded)
});
