// original: 0x00890F10 
// 00890F10 audSound pool attach: resolve the slot pool, notify the old
// owner, then attach the pool.
export!(thiscall, rw_00890f10(this: *mut u8) -> () {
    unsafe {
        let sel = *this.add(4);
        let pool = if sel == 0xFF {
            0u32
        } else {
            let row = (*this.add(0x40) as u32).wrapping_mul(0x6F40);
            let entry = (*global::<u32>(0x115D968))
                .wrapping_mul(sel as u32)
                .wrapping_add(
                    *((row.wrapping_add(*global::<u32>(0x115D988)).wrapping_add(0x6F14))
                        as *const u32),
                );
            entry
        };
        let owner = *(this.add(0x74) as *const u32);
        if owner != 0 {
            let vtable = *(owner as *const u32);
            let target = *((vtable as *const u8).add(4) as *const u32);
            let drop_old: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let prev = drop_old(owner, 0);
            callee_thiscall!(2, u32, pool, prev);
        }
        callee_thiscall!(3, u32, pool, 1);
    }
});
