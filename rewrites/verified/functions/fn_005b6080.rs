// original: 0x005B6080 txd_slot_acquire
/// Acquires a texture-dictionary pool slot: refreshes a live slot through the
/// release helper, tags the slot flag, drops the pool count, and tracks the
/// lowest live index. A tagged flag faults exactly like the original (null
/// slot read).
export!(cdecl, rw_005B6080(index: u32) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x011764C0) as *mut u32;
        let flags = pool.add(1).read() as *const u8;
        let base = pool.add(0).read();
        let stride = pool.add(3).read();
        let tagged = flags.add(index as usize).read() & 0x80 != 0;
        let slot = if tagged {
            0u32
        } else {
            base.wrapping_add(index.wrapping_mul(stride))
        };
        // Volatile: a plain read through null is undefined behaviour and
        // would let the optimizer fold this fault away; the original
        // performs the load and faults, so the rewrite must really load.
        if core::ptr::read_volatile(slot as *const u32) != 0 {
            callee_cdecl!(1, u32, index);
        }
        let pool = *global::<u32>(0x011764C0) as *mut u32;
        let flags = pool.add(1).read() as *mut u8;
        let base = pool.add(0).read();
        let stride = pool.add(3).read();
        let tagged = flags.add(index as usize).read() & 0x80 != 0;
        let slot = if tagged {
            0u32
        } else {
            base.wrapping_add(index.wrapping_mul(stride))
        };
        let quot = ((slot.wrapping_sub(base)) as i32).wrapping_div(stride as i32);
        *flags.add(quot as usize) |= 0x80;
        let cnt = pool.add(5);
        cnt.write(cnt.read().wrapping_sub(1));
        let lo = pool.add(4);
        if quot < lo.read() as i32 {
            lo.write(quot as u32);
        }
        quot as u32
    }
});
