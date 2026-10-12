// original: 0x006349F0 frag_parse_skeletontype_all

/// Read a 512-byte skeleton-type record from the object at `input + 4` into
/// a local buffer, compare that buffer with the image string `none`, and, for
/// any nonzero comparison result, pass the buffer to the initializer at
/// `this + 0xE0`. The security-cookie check receives the image's current
/// cookie. The local buffer is observed through snapshots of the first 64
/// words at the scripted calls; the supplied records terminate in their
/// first two words, so later buffer words do not affect the tested string.
lf_checker_rt::export!(thiscall, rw_006349F0(this: u32, input: u32) -> u32 {
    const READ_RECORD: u32 = 1;
    const COMPARE_TEXT: u32 = 2;
    const INITIALIZE_RECORD: u32 = 3;
    const CHECK_COOKIE: u32 = 4;
    const INPUT_OBJECT: u32 = 4;
    const INITIALIZER: u32 = 0xE0;
    const SECURITY_COOKIE: u32 = 0x01057FB4;
    const NO_SKELETON_TYPE: u32 = 0x00F97E3C;
    const RECORD_BYTES: u32 = 0x200;

    unsafe {
        let interface = (input.wrapping_add(INPUT_OBJECT) as *const u32).read_unaligned();
        let mut record = core::mem::MaybeUninit::<[u32; 128]>::uninit();
        let record_pointer = core::ptr::addr_of_mut!(record).cast::<u32>() as u32;

        let _read_count = lf_checker_rt::callee_thiscall!(
            READ_RECORD,
            u32,
            interface,
            record_pointer,
            RECORD_BYTES,
        );

        let compare_result = lf_checker_rt::callee_cdecl!(
            COMPARE_TEXT,
            u32,
            record_pointer,
            lf_checker_rt::relocated(NO_SKELETON_TYPE),
        );

        let result = if compare_result == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(
                INITIALIZE_RECORD,
                u32,
                this.wrapping_add(INITIALIZER),
                record_pointer,
            )
        };

        let cookie = lf_checker_rt::global::<u32>(SECURITY_COOKIE).read_unaligned();
        lf_checker_rt::callee_thiscall!(CHECK_COOKIE, (), cookie);
        result
    }
});
