// original: 0x00e621e0 initialize_network_pool_state_records

/// Initialize eight network pool records spaced by 0x148 bytes. Clear the leading words, clear bit 0 in the flag byte, install the callback pointer, initialize each embedded critical section, reset the tail fields, then register the cleanup callback and return its result.
lf_checker_rt::export!(cdecl, rw_00e621e0() -> u32 {
    const FIRST_RECORD: u32 = 0x01b4991c;
    const RECORD_STRIDE: u32 = 0x148;
    const CALLBACK_VA: u32 = 0x00fe5c58;
    let mut record_va = FIRST_RECORD;
    for _record_index in 0..8 {
        unsafe {
            lf_checker_rt::global::<u32>(record_va).write_volatile(0);
            lf_checker_rt::global::<u32>(record_va.wrapping_add(4)).write_volatile(0);
            lf_checker_rt::global::<u32>(record_va.wrapping_add(8)).write_volatile(0);
            lf_checker_rt::global::<u32>(record_va.wrapping_add(12)).write_volatile(0);
            let flag = lf_checker_rt::global::<u8>(record_va.wrapping_add(0x10)).read_volatile();
            lf_checker_rt::global::<u8>(record_va.wrapping_add(0x10)).write_volatile(flag & 0xfe);
            lf_checker_rt::global::<u32>(record_va.wrapping_sub(4)).write_volatile(lf_checker_rt::relocated(CALLBACK_VA));
        }
        let _lock_result = lf_checker_rt::callee_stdcall!(1, u32, lf_checker_rt::relocated(record_va.wrapping_add(0x14)));
        unsafe {
            lf_checker_rt::global::<u32>(record_va.wrapping_add(0x134)).write_volatile(u32::MAX);
            lf_checker_rt::global::<u32>(record_va.wrapping_add(0x138)).write_volatile(0);
            lf_checker_rt::global::<u32>(record_va.wrapping_add(0x13c)).write_volatile(0);
            lf_checker_rt::global::<u32>(record_va.wrapping_add(0x140)).write_volatile(0);
            lf_checker_rt::global::<u8>(record_va.wrapping_add(0x34)).write_volatile(0);
        }
        record_va = record_va.wrapping_add(RECORD_STRIDE);
    }
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00e709a0))
});
