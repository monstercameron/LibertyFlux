// original: 0x00627780 network_record_read_list_outputs

/// Read two 32-byte record lists through distinct stack-local output words and copy them into the
/// caller's two-word result. The owner and result pointer arrive in ECX and EDX. The first helper
/// writes the word used for result[0]; the second helper, which runs only if the first succeeds,
/// writes the word used for result[1]. Both result words are stored even when a helper fails.
/// Success requires both helpers and a nonzero OR of their output words. The stack pointer passed
/// to each helper is skipped in the call log and its one pointed-to word is compared by a call-time
/// snapshot.
lf_checker_rt::export!(fastcall, rw_00627780(owner: u32, result: u32) -> u32 {
    unsafe {
        const FIRST_LIST_READ: u32 = 1;
        const SECOND_LIST_READ: u32 = 2;
        const LIST_BYTES: u32 = 0x20;

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let mut first_output = 0u32;
        let mut second_output = 0u32;
        let first_ok: u32 = lf_checker_rt::callee_thiscall!(
            FIRST_LIST_READ,
            u32,
            owner,
            &mut first_output as *mut u32 as usize as u32,
            LIST_BYTES
        );
        let both_reads_ok = first_ok & 0xFF != 0
            && (lf_checker_rt::callee_thiscall!(
                SECOND_LIST_READ,
                u32,
                owner,
                &mut second_output as *mut u32 as usize as u32,
                LIST_BYTES
            ) & 0xFF != 0);

        write_u32(result, first_output);
        write_u32(result.wrapping_add(4), second_output);

        let any_output = (first_output | second_output) != 0;
        if both_reads_ok && any_output { 1 } else { 0 }
    }
});
