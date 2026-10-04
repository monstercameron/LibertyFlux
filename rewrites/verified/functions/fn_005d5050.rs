// original: 0x005d5050 CTaskComplexPlayerSettingsTask::~CTaskComplexPlayerSettingsTask__deleting
/// Run the base destructor, then free the pool slot when the deleting flag
/// (low bit of the argument) is set. The pool header reached through the
/// global gives the base address, the free bitmap, the element stride, the
/// lowest free index and the live count; freeing marks the bitmap, lowers
/// the minimum and drops the count. Returns the object pointer.
export!(thiscall, rw_005d5050(this_ptr: u32, flag: u32) -> u32 {
    let _: u32 = callee_thiscall!(0, u32, this_ptr);
    if flag & 1 == 0 {
        return this_ptr;
    }
    let pool = unsafe { global::<u32>(0x0167E2A0).read() };
    let base = unsafe { (pool as *const u32).read() };
    let bitmap = unsafe { ((pool + 4) as *const u32).read() };
    let stride = unsafe { ((pool + 0x0C) as *const u32).read() };
    // Original: sub/cdq/idiv, i.e. truncated signed division of the
    // pool-relative offset by the stride; inputs keep it non-faulting.
    let idx = (this_ptr as i32).wrapping_sub(base as i32) / stride as i32;
    unsafe {
        let cell = (bitmap.wrapping_add(idx as u32)) as *mut u8;
        cell.write(cell.read() | 0x80);
        let min_cell = (pool + 0x10) as *mut i32;
        if idx < min_cell.read() {
            min_cell.write(idx);
        }
        let count_cell = (pool + 0x14) as *mut u32;
        count_cell.write(count_cell.read().wrapping_sub(1));
    }
    this_ptr
});
