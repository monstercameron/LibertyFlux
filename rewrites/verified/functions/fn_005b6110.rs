// original: 0x005B6110 txd_slot_retain
/// Retains a texture-dictionary pool slot: returns the slot address and bumps
/// its reference count. A slot whose flag byte is tagged faults exactly like
/// the original (reference write through a null slot pointer).
export!(cdecl, rw_005B6110(index: u32) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x011764C0) as *const u32;
        let flags = pool.add(1).read() as *const u8;
        let tagged = flags.add(index as usize).read() & 0x80 != 0;
        let base = pool.add(0).read();
        let stride = pool.add(3).read();
        let slot = if tagged {
            0u32
        } else {
            base.wrapping_add(index.wrapping_mul(stride))
        };
        let rc = slot.wrapping_add(4) as *mut u32;
        rc.write(rc.read().wrapping_add(1));
        slot
    }
});
