// original: 0x005C22F0 release_active_control_slot

/// Release the caller's control slot, release the indexed active record when
/// one exists, then notify the current control context. The active index is an
/// unsigned word; u32::MAX means no record. The record address uses wrapping
/// 32-bit arithmetic with a 0xBC-byte stride, and a wrapped null address is
/// skipped. The final helper receives the original object as its first stack
/// argument and a zero second stack argument; its low-byte flag is zero.
/// The entry object is in ECX and the function has no stack arguments.
lf_checker_rt::export!(fastcall, rw_005c22f0(this_ptr: u32, incoming_edx: u32) -> () {
    const RELEASE_SLOT_CALLEE: u32 = 1;
    const RELEASE_RECORD_CALLEE: u32 = 2;
    const NOTIFY_CONTEXT_CALLEE: u32 = 3;
    const ACTIVE_RECORD_INDEX_VA: u32 = 0x010330F8;
    const ACTIVE_RECORD_TABLE_VA: u32 = 0x0118D470;
    const ACTIVE_RECORD_STRIDE: u32 = 0xBC;
    const CONTROL_CONTEXT_VA: u32 = 0x011D6FE0;
    const NO_ACTIVE_RECORD: u32 = u32::MAX;
    const LOW_BYTE_MASK: u32 = 0xFF;

    let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE_SLOT_CALLEE, u32, 1u32);

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
        this_ptr,
        0u32
    );
});
