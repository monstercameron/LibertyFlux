// original: 0x00BDE9E0 ped_task_pool_release_entry
/// Release the indexed pool record, clear its table slot, and return the
/// release helper's answer.
///
/// The global pool header provides a record-pointer table at +0, marker bytes
/// at +4, and the table stride at +12. If the selected marker has bit 7 set,
/// the original faults through a null read. Otherwise it passes the record
/// pointer to the thiscall release helper, clears the selected table word, and
/// returns the helper's EAX value.
lf_checker_rt::export!(cdecl, rw_00bde9e0(index: u32) -> u32 {
    const PED_TASK_POOL_GLOBAL: u32 = 0x0167_F594;
    unsafe {
        let pool = *lf_checker_rt::global::<u32>(PED_TASK_POOL_GLOBAL);
        let data_base = *(pool as *const u32);
        let marker_base = *(pool.wrapping_add(4) as *const u32);
        let stride = *(pool.wrapping_add(12) as *const u32);
        let marker = *((marker_base.wrapping_add(index)) as *const u8);
        if marker & 0x80 != 0 {
            let fault_address = core::hint::black_box(0usize);
            core::ptr::read_volatile(fault_address as *const u32);
        }
        let slot = data_base.wrapping_add(stride.wrapping_mul(index));
        let record = *(slot as *const u32);
        let result = lf_checker_rt::callee_thiscall!(1, u32, record);
        *(slot as *mut u32) = 0;
        result
    }
});
