// original: 0x008b5870 device_table_setup
/// Input device table setup for one slot.
///
/// Builds the device record for handle-table slot `index`: fetches the band
/// record and its sample float, configures the slot, creates the device
/// handle from the parameter block (storing it in the table), resolves three
/// extra values, assembles the record, then finalises and publishes the slot.
/// Returns the publish call's answer.
export!(cdecl, rw_008b5870(index: u32) -> u32 {
    unsafe {
        const HANDLES: u32 = 0x01160C0C;
        const PARAM_CREATE: u32 = 0x01161854;
        const PARAM_VALUE: u32 = 0x01161850;
        const PARAM_ASSEMBLE: u32 = 0x0116185C;
        const PARAM_FLOAT: u32 = 0x01161864;
        let slot3 = index.wrapping_mul(3);
        let stride = slot3.wrapping_mul(8);
        let tmp_a = 0u32;
        let rec: u32 = callee_cdecl!(1, u32, &tmp_a as *const u32 as u32);
        let sample = (rec as *const u32).read();
        let tmp_b = 0u32;
        let _: u32 = callee_cdecl!(2, u32, 2, 0, &tmp_b as *const u32 as u32, 0);
        let create_params = relocated(PARAM_CREATE).wrapping_add(stride);
        let handle: u32 = callee_cdecl!(3, u32, 0, 0, create_params, sample, 2, 1, 0, 1);
        (relocated(HANDLES).wrapping_add(index.wrapping_mul(4)) as *mut u32).write(handle);
        let tmp_c = 0u32;
        let r1: u32 = callee_cdecl!(4, u32, &tmp_c as *const u32 as u32, 0x42);
        let first = (r1 as *const u32).read();
        let tmp_d = 0u32;
        let r2: u32 = callee_cdecl!(4, u32, &tmp_d as *const u32 as u32, 0x3E);
        let second = (r2 as *const u32).read();
        let tmp_e = 0u32;
        let r3: u32 = callee_cdecl!(4, u32, &tmp_e as *const u32 as u32, 0x3B);
        let extra = (r3 as *const u32).read();
        let tune = (relocated(PARAM_FLOAT).wrapping_add(stride) as *const u32).read();
        let value = (relocated(PARAM_VALUE).wrapping_add(stride) as *const u32).read();
        let assemble = relocated(PARAM_ASSEMBLE).wrapping_add(stride);
        let _: u32 = callee_cdecl!(5, u32, handle, value, assemble, tune, extra, second, first);
        let _: u32 = callee_cdecl!(6, u32, index);
        callee_cdecl!(7, u32, handle, 0, 1)
    }
});
