// original: 0x005B6140 txd_slot_release
/// Releases a texture-dictionary pool slot: drops its reference count and,
/// when it reaches zero and the slot is not resident, frees it through the
/// release helper. Tagged slots and live slots return immediately.
export!(cdecl, rw_005B6140(index: u32) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x011764C0) as *const u32;
        let flags = pool.add(1).read() as *const u8;
        if flags.add(index as usize).read() & 0x80 != 0 {
            return flags as u32;
        }
        let base = pool.add(0).read();
        let stride = pool.add(3).read();
        let slot = base.wrapping_add(index.wrapping_mul(stride));
        if slot == 0 {
            return 0;
        }
        let rc = (slot.wrapping_add(4)) as *mut u32;
        rc.write(rc.read().wrapping_sub(1));
        if rc.read() as i32 > 0 {
            return slot;
        }
        let arg = *global::<u32>(0x01032F58);
        let r = callee_cdecl!(1, u32, index, arg);
        if r as u8 != 0 {
            return r;
        }
        callee_cdecl!(2, u32, index)
    }
});
