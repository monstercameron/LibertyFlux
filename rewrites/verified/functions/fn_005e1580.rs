// original: 0x005E1580 CRenderPhaseFrontEnd::vf8
/// Allocate two small phase-owned records, update the object's 14-bit tag
/// with a shared counter, initialize each record's type and callback fields,
/// and pass both nullable results to the release helper. The global counter
/// is a single 32-bit word; each successful allocation increments it once.
/// The allocator and release helper are external cdecl calls intercepted by
/// the checker, and the method returns the final helper's eax value.
lf_checker_rt::export!(cdecl, rw_005e1580() -> u32 {
    const COUNTER_FILE_VA: u32 = 0x0103_27a0;
    const TAG_MASK: u32 = 0x3fff;
    const PHASE_VTABLE_FILE_VA: u32 = 0x00e7_e080;
    const FIRST_CALLBACK_FILE_VA: u32 = 0x005a_90a0;
    const SECOND_CALLBACK_FILE_VA: u32 = 0x005a_a7d0;

    #[inline(always)]
    unsafe fn initialize_record(address: u32, callback: u32, counter: *mut u32) {
        let old_counter = unsafe { counter.read_unaligned() };
        let old_tag = unsafe { ((address + 4) as *const u32).read_unaligned() };
        let mixed_bits = (old_tag ^ old_counter) & TAG_MASK;
        unsafe {
            ((address + 4) as *mut u32).write_unaligned(old_tag ^ mixed_bits);
            counter.write_unaligned(old_counter.wrapping_add(1));
            (address as *mut u32)
                .write_unaligned(lf_checker_rt::relocated(PHASE_VTABLE_FILE_VA));
            ((address + 8) as *mut u32).write_unaligned(callback);
        }
    }

    let counter = lf_checker_rt::global::<u32>(COUNTER_FILE_VA);
    let first = lf_checker_rt::callee_cdecl!(1, u32, 12u32, 0u32);
    if first != 0 {
        unsafe {
            initialize_record(
                first,
                lf_checker_rt::relocated(FIRST_CALLBACK_FILE_VA),
                counter,
            )
        };
    }
    let _ = lf_checker_rt::callee_cdecl!(3, u32, first);

    let second = lf_checker_rt::callee_cdecl!(2, u32, 12u32, 0u32);
    if second != 0 {
        unsafe {
            initialize_record(
                second,
                lf_checker_rt::relocated(SECOND_CALLBACK_FILE_VA),
                counter,
            )
        };
    }
    lf_checker_rt::callee_cdecl!(4, u32, second)
});
