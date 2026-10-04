// original: 0x008acc90 audOcclusionPool_mark_item
/// `index = (ptr - items_base) / 384`; sets flag bit 0x80 under the pool
/// lock and bumps the live count. Returns the unlock answer.
export!(thiscall, rw_008acc90(this: u32, ptr: u32) -> u32 {
    unsafe {
        let base = *(this as *const u32);
        let idx = (ptr.wrapping_sub(base) as i32 / 384) as usize;
        callee_thiscall!(1, u32, this.wrapping_add(8));
        let bytes = *((this as *const u32).add(1));
        *((bytes as *mut u8).add(idx)) |= 0x80;
        let cnt = (this as *mut u32).add(0x2c / 4);
        *cnt = (*cnt).wrapping_add(1);
        callee_thiscall!(2, u32, this.wrapping_add(8))
    }
});

