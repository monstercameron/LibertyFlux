// original: 0x00bcd5b0 physics_notify_pending_action
/// Forward a damage-permission flag to the physics helper, or run the
/// disabled-path probe sequence. The single cdecl word is read low-byte-only:
/// when the low byte is nonzero the helper (cdecl/1) is called once with the
/// literal 1 and its return value is the function result. When the low byte
/// is zero the helper is called once with 0 (its answer discarded), then a
/// three-word stack scratch record is resolved through a second helper
/// (cdecl/1, writes all three words through the passed pointer), and a third
/// helper (cdecl/2) is called with the resolver's answer and the constant
/// limit bits 0x457a0000; the third helper's answer is the function result.
/// EAX is defined on both paths. The upper three bytes of the input word are
/// never read.
lf_checker_rt::export!(
    cdecl,
    rw_00bcd5b0(enabled_word: u32) -> u32 {
        const PROBE_LIMIT_BITS: u32 = 0x457a0000;
        const SCRATCH_WORDS: usize = 3;
        if enabled_word & 0xff == 0 {
            let _disabled = lf_checker_rt::callee_cdecl!(1, u32, 0u32);
            let mut scratch = [0u32; SCRATCH_WORDS];
            let probe = lf_checker_rt::callee_cdecl!(
                2,
                u32,
                scratch.as_mut_ptr() as usize as u32
            );
            lf_checker_rt::callee_cdecl!(3, u32, probe, PROBE_LIMIT_BITS)
        } else {
            lf_checker_rt::callee_cdecl!(1, u32, 1u32)
        }
    }
);
