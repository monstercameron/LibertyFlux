// original: 0x0088D900 rage::audVoiceSoft::vf0

/// Deleting destructor for a software voice: run the base destructor, then
/// release the voice pool slot when the delete flag is set.
///
/// `this` points to the voice. Its table pointer is reset to the software
/// voice table and the base destructor (callee 1) runs with the voice as
/// `this`. When bit 0 of `delete` is set, the voice's slot in the voice
/// pool (reached through the pool pointer global) is released: the slot
/// index is the object address minus the pool base, divided by the slot
/// stride with a magic multiply; the pool lock (callee 2) is taken, the
/// slot's free bit (0x80) is set in the pool bitmap, the pool's free count
/// rises by one, and the lock (callee 3) is released. Returns `this`.
///
/// Original: 0x0088D900 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0088D900(this: u32, delete: u32) -> u32 {
    unsafe {
        const VTABLE_SOFT_VOICE: u32 = 0x00e7_8250;
        const POOL_GLOBAL: u32 = 0x0115_a520;
        const POOL_BASE: u32 = 0x00;
        const POOL_BITMAP: u32 = 0x04;
        const POOL_LOCK: u32 = 0x08;
        const POOL_FREE_COUNT: u32 = 0x2c;
        const SLOT_MAGIC: u32 = 0x67b2_3a55;
        const SLOT_SHIFT: u32 = 9;
        const SLOT_FREE_BIT: u8 = 0x80;
        const BASE_DTOR: u32 = 1;
        const POOL_LOCK_CALLEE: u32 = 2;
        const POOL_UNLOCK_CALLEE: u32 = 3;

        // The table address is an absolute immediate with a relocation entry,
        // so the original stores the relocated address, not the file value.
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_SOFT_VOICE));
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        if delete & 1 != 0 {
            let pool =
                (lf_checker_rt::relocated(POOL_GLOBAL) as *const u32).read_unaligned();
            let base = (pool as *const u32).read_unaligned();
            let diff = this.wrapping_sub(base);
            // Signed divide of `diff` by the slot stride: high word of the
            // magic product, shifted, rounded toward zero.
            let high = ((SLOT_MAGIC as u64 * diff as u64) >> 32) as u32 as i32;
            let quot = high >> SLOT_SHIFT;
            let index = quot + ((quot >> 31) & 1);
            let lock = pool + POOL_LOCK;
            lf_checker_rt::callee_thiscall!(POOL_LOCK_CALLEE, u32, lock);
            let bitmap = ((pool + POOL_BITMAP) as *const u32).read_unaligned();
            let slot = (bitmap + index as u32) as *mut u8;
            slot.write(slot.read() | SLOT_FREE_BIT);
            let count = (pool + POOL_FREE_COUNT) as *mut u32;
            count.write_unaligned(count.read_unaligned().wrapping_add(1));
            lf_checker_rt::callee_thiscall!(POOL_UNLOCK_CALLEE, u32, lock);
        }
        this
    }
});
