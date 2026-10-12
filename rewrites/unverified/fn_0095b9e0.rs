// original: 0x0095B9E0 clear_stale_task_entries
/// Scan 512 twelve-byte task registry entries beginning at the registry's
/// pointer column. For a non-null object whose tag byte is 0x1b or 0x1c, call
/// the resource check with mode 1 and the object's key at byte offset 12. If
/// that check returns zero, call the task check with the object pointer. A
/// nonzero result from either check clears the preceding dword, the entry's
/// pointer dword, and the low byte of the following dword; otherwise the row
/// is retained. The function is a no-argument cdecl routine with no semantic
/// return value. All row accesses and object offsets are 32-bit game layout.
lf_checker_rt::export!(cdecl, rw_0095b9e0() -> () {
    unsafe {
        const ENTRY_TABLE_VA: u32 = 0x011FF074;
        const ENTRY_COUNT: u32 = 0x200;
        const ENTRY_STRIDE: u32 = 0x0c;
        const ENTRY_TAG: u32 = 0;
        const ENTRY_KEY: u32 = 0x0c;
        const RESOURCE_TAG_A: u8 = 0x1b;
        const RESOURCE_TAG_B: u8 = 0x1c;
        const CHECK_RESOURCE: u32 = 1;
        const CHECK_TASK: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        for index in 0..ENTRY_COUNT {
            let row = ENTRY_TABLE_VA.wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let task = read_u32(lf_checker_rt::relocated(row));
            if task == 0 {
                continue;
            }
            let tag = read_u8(task + ENTRY_TAG);
            if tag != RESOURCE_TAG_A && tag != RESOURCE_TAG_B {
                continue;
            }

            let key = read_u32(task + ENTRY_KEY);
            let accepted = lf_checker_rt::callee_cdecl!(CHECK_RESOURCE, u32, 1u32, key);
            let should_clear = if accepted != 0 {
                true
            } else {
                lf_checker_rt::callee_cdecl!(CHECK_TASK, u32, task) != 0
            };
            if !should_clear {
                continue;
            }

            lf_checker_rt::global::<u32>(row.wrapping_sub(4)).write_unaligned(0);
            lf_checker_rt::global::<u32>(row).write_unaligned(0);
            lf_checker_rt::global::<u8>(row.wrapping_add(4)).write(0);
        }
    }
});
