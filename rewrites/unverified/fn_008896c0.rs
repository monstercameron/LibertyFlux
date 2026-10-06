// original: 0x008896C0 rage::audVoicePhysical::vf0

/// Deleting destructor for a physical voice: reset its table pointer, then
/// release its voice pool slot when the delete flag is set.
///
/// `this` points to the voice. Its table pointer is reset to the physical
/// voice table. When bit 0 of `delete` is set, the voice's slot in the voice
/// pool (reached through the pool pointer global) is released: the slot index
/// is the object address minus the pool base, divided by the slot stride with
/// a magic multiply (signed, rounded toward zero); the pool lock (callee 1)
/// is taken with the lock object, the slot's free bit (0x80) is set in the
/// pool bitmap, the pool's free count rises by one, and the lock (callee 2)
/// is released. Returns `this`. Started from `rage::audVoiceDSound::vf0`
/// (lane r-s401), which additionally runs a base destructor this one lacks.
///
/// Original: 0x008896C0 (thiscall, one stack word, 2 calls).
lf_checker_rt::export!(thiscall, rw_008896C0(this: u32, delete: u32) -> u32 {
    unsafe {
        const VTABLE_PHYSICAL_VOICE: u32 = 0x00e7_79ec;
        const POOL_GLOBAL: u32 = 0x0115_a520;
        const POOL_BASE: u32 = 0x00;
        const POOL_BITMAP: u32 = 0x04;
        const POOL_LOCK: u32 = 0x08;
        const POOL_FREE_COUNT: u32 = 0x2c;
        const SLOT_MAGIC: u32 = 0x67b2_3a55;
        const SLOT_SHIFT: u32 = 9;
        const SLOT_FREE_BIT: u8 = 0x80;
        const POOL_LOCK_CALLEE: u32 = 1;
        const POOL_UNLOCK_CALLEE: u32 = 2;

        // The table address is an absolute immediate with a relocation entry,
        // so the original stores the relocated address, not the file value.
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_PHYSICAL_VOICE));
        if delete & 1 != 0 {
            let pool =
                (lf_checker_rt::relocated(POOL_GLOBAL) as *const u32).read_unaligned();
            let base = (pool as *const u32).read_unaligned();
            let diff = this.wrapping_sub(base);
            // Signed divide of `diff` by the slot stride: high word of the
            // SIGNED magic product (the original's imul is signed, so a
            // negative diff must sign-extend before multiplying), shifted,
            // rounded toward zero.
            let high = (((SLOT_MAGIC as i32 as i64) * (diff as i32 as i64)) >> 32) as i32;
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
