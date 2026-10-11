// original: 0x00BDE840 ped_task_pool_record_word_at_index
/// Return the second dword of the indexed ped-task pool record.
///
/// The global pool pointer addresses a four-word header: record-pointer table
/// at +0, marker-byte base at +4, an unused count at +8, and pointer-table
/// stride at +12. A high marker bit faults through a null read. Otherwise the
/// selected record pointer is loaded and its dword at +4 is returned.
lf_checker_rt::export!(cdecl, rw_00bde840(index: u32) -> u32 {
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
        *((record.wrapping_add(4)) as *const u32)
    }
});
