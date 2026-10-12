// original: 0x005C2340 release_default_control_slot

/// Release the indexed active record when the unsigned global index is not
/// u32::MAX and its 0xBC-stride address does not wrap to null, then notify the
/// current control context with two zero stack arguments and a zero low-byte
/// flag. ECX is saved and restored by the native wrapper but is not otherwise
/// interpreted. EDX is caller-scratch state; the native code clears only DL.
/// The function has no stack arguments.
lf_checker_rt::export!(fastcall, rw_005c2340(_entry_ecx: u32, incoming_edx: u32) -> () {
    const RELEASE_RECORD_CALLEE: u32 = 1;
    const NOTIFY_CONTEXT_CALLEE: u32 = 2;
    const ACTIVE_RECORD_INDEX_VA: u32 = 0x010330F8;
    const ACTIVE_RECORD_TABLE_VA: u32 = 0x0118D470;
    const ACTIVE_RECORD_STRIDE: u32 = 0xBC;
    const CONTROL_CONTEXT_VA: u32 = 0x011D6FD8;
    const NO_ACTIVE_RECORD: u32 = u32::MAX;
    const LOW_BYTE_MASK: u32 = 0xFF;

    let active_index =
        unsafe { lf_checker_rt::global::<u32>(ACTIVE_RECORD_INDEX_VA).read_unaligned() };
    if active_index != NO_ACTIVE_RECORD {
        let record = lf_checker_rt::relocated(ACTIVE_RECORD_TABLE_VA)
            .wrapping_add(active_index.wrapping_mul(ACTIVE_RECORD_STRIDE));
        if record != 0 {
            let _: u32 =
                lf_checker_rt::callee_thiscall!(RELEASE_RECORD_CALLEE, u32, record, 1u32);
        }
    }

    let context =
        unsafe { lf_checker_rt::global::<u32>(CONTROL_CONTEXT_VA).read_unaligned() };
    let notify_flags = incoming_edx & !LOW_BYTE_MASK;
    let _: u32 = lf_checker_rt::callee_fastcall!(
        NOTIFY_CONTEXT_CALLEE,
        u32,
        context,
        notify_flags,
        0u32,
        0u32
    );
});
