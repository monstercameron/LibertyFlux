// original: 0x005dd120 CTaskSimpleAssessInjuredPed::~CTaskSimpleAssessInjuredPed__deleting
/// Deleting destructor of an assess task: run the member destructor, then
/// free the pool slot when the delete flag is set.
export!(thiscall, rw_005dd120(this_ptr: *mut u8, flag: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr as u32);
        if flag & 1 != 0 {
                    {
            // Free this slot back to the task pool (see pool_free in the lane crate).
                let pool = *global::<u32>(0x167E2A0) as *mut u32;
                let base = *pool;
                let diff = (this_ptr as u32).wrapping_sub(base);
                let div = *pool.add(3);
                let idx = (diff as i32 as i64 / div as i32 as i64) as u32;
                let bitmap = *pool.add(1) as *mut u8;
                *bitmap.add(idx as usize) |= 0x80;
                if (idx as i32) < (*pool.add(4) as i32) {
                    *pool.add(4) = idx;
                }
                *pool.add(5) = (*pool.add(5)).wrapping_sub(1);
            }
        }
        this_ptr as u32
    }
});
