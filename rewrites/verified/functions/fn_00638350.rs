// original: 0x00638350 rage::ptxEffectRuleStd::vf33

/// Invoke each eligible record's callback, optionally release its resource,
/// then clear the callback pointer stored in the query result.
///
/// `this` has a record pointer array at `+0x130` and an unsigned 16-bit count
/// at `+0x134`. Records whose word at `+0x14` is nonzero are skipped. Each
/// other record is queried through the helper with `(record, 0, name_a,
/// name_b, 0)`. If the returned object's `+0x48` field is non-null, its
/// virtual method at `+0x08` is called. When the low byte of
/// `release_requested` is nonzero and the callback object's word at `+0x04`
/// equals one, the manager's virtual method at `+0x18` receives the callback,
/// one, and zero. The helper result's `+0x48` field is then cleared. This is a
/// thiscall method with one 32-bit stack argument and no semantic return.
lf_checker_rt::export!(thiscall, rw_00638350(this: u32, release_requested: u32) -> () {
    unsafe {
        const RECORD_ARRAY: u32 = 0x130;
        const RECORD_COUNT: u32 = 0x134;
        const RECORD_SKIP_WORD: u32 = 0x14;
        const CALLBACK_RESULT: u32 = 0x48;
        const CALLBACK_KIND: u32 = 0x04;
        const CALLBACK_SLOT: u32 = 0x08;
        const MANAGER_SLOT: u32 = 0x18;
        const RELEASE_MANAGER_FILE_VA: u32 = 0x01BB_6674;
        const HELPER_NAME_A: u32 = 0x0107_07D8;
        const HELPER_NAME_B: u32 = 0x00F9_8458;
        const QUERY_HELPER: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let records = unsafe { read_word(this.wrapping_add(RECORD_ARRAY)) };
        let count = unsafe { (this.wrapping_add(RECORD_COUNT) as *const u16).read_unaligned() } as u32;
        let release_requested = (release_requested & 0xFF) != 0;
        let mut index = 0_u32;
        while index < count {
            let record = unsafe { read_word(records.wrapping_add(index.wrapping_mul(4))) };
            index = index.wrapping_add(1);
            if unsafe { read_word(record.wrapping_add(RECORD_SKIP_WORD)) } != 0 {
                continue;
            }

            let query = lf_checker_rt::callee_cdecl!(
                QUERY_HELPER,
                u32,
                record,
                0_u32,
                lf_checker_rt::relocated(HELPER_NAME_A),
                lf_checker_rt::relocated(HELPER_NAME_B),
                0_u32
            );
            let callback = unsafe { read_word(query.wrapping_add(CALLBACK_RESULT)) };
            if callback == 0 {
                continue;
            }

            let callback_vtable = unsafe { read_word(callback) };
            let callback_address = unsafe {
                read_word(callback_vtable.wrapping_add(CALLBACK_SLOT))
            };
            let callback_method: unsafe extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(callback_address as usize) };
            let _callback_result = unsafe { callback_method(callback) };

            if release_requested
                && unsafe { read_word(callback.wrapping_add(CALLBACK_KIND)) } == 1
            {
                let manager_global = lf_checker_rt::global::<u32>(RELEASE_MANAGER_FILE_VA) as u32;
                let manager = unsafe { read_word(manager_global) };
                let manager_vtable = unsafe { read_word(manager) };
                let release_address = unsafe {
                    read_word(manager_vtable.wrapping_add(MANAGER_SLOT))
                };
                let release_method: unsafe extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(release_address as usize) };
                let _release_result = unsafe { release_method(manager, callback, 1, 0) };
            }

            unsafe { write_word(query.wrapping_add(CALLBACK_RESULT), 0) };
        }
    }
});
