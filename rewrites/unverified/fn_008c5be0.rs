// original: 0x008C5BE0 text_label_to_string
/// Return a resolved label or the shared UTF-16 scratch table. Empty labels
/// clear all 64 entries; nonempty labels are hashed and forwarded with their
/// original pointer. The final cookie helper preserves the selected EAX value.
lf_checker_rt::export!(thiscall, rw_008c5be0(this: u32, label: u32) -> u32 {
    const HASH_HELPER: u32 = 1;
    const LABEL_RESOLVER: u32 = 2;
    const NULL_LABEL_HANDLER: u32 = 3;
    const COOKIE_CHECK: u32 = 4;
    const SCRATCH_TABLE_FILE_VA: u32 = 0x116B_F68;
    unsafe {
        if label == 0 {
            let _ = lf_checker_rt::callee_thiscall!(NULL_LABEL_HANDLER, u32, this);
            for index in 0..64 {
                (lf_checker_rt::relocated(SCRATCH_TABLE_FILE_VA).wrapping_add(index * 2)
                    as *mut u16).write_unaligned(0);
            }
            let result = lf_checker_rt::relocated(SCRATCH_TABLE_FILE_VA);
            let _ = lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, 0);
            return result;
        }
        let first = (label as *const u8).read();
        let result = if first == 0 || first == b' ' {
            for index in 0..64 {
                (lf_checker_rt::relocated(SCRATCH_TABLE_FILE_VA).wrapping_add(index * 2)
                    as *mut u16).write_unaligned(0);
            }
            lf_checker_rt::relocated(SCRATCH_TABLE_FILE_VA)
        } else {
            let hash = lf_checker_rt::callee_cdecl!(HASH_HELPER, u32, label, 0);
            lf_checker_rt::callee_thiscall!(LABEL_RESOLVER, u32, this, hash, label)
        };
        let _ = lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, 0);
        result
    }
});
