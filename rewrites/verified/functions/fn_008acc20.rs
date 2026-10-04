// original: 0x008acc20 rage::audOcclusionGroupInterface::vf0
/// Stamps the vtable; when `flag & 1`, releases `this` from the global pool:
/// `index = (this - items_base) / 384`, sets flag bit 0x80 under the pool
/// lock and bumps the live count. Returns `this`.
export!(thiscall, rw_008acc20(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00e7c88c);
        if flag & 1 != 0 {
            let pool = *global::<u32>(0x115fd54);
            let base = *(pool as *const u32);
            let idx = ((this as u32).wrapping_sub(base) as i32 / 384) as usize;
            callee_thiscall!(1, u32, pool.wrapping_add(8));
            let bytes = *((pool as *const u32).add(1));
            *((bytes as *mut u8).add(idx)) |= 0x80;
            let cnt = (pool as *mut u32).add(0x2c / 4);
            *cnt = (*cnt).wrapping_add(1);
            callee_thiscall!(2, u32, pool.wrapping_add(8));
        }
        this as u32
    }
});

