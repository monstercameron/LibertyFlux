// original: 0x009B1B30 audPoliceScanner::acquire_noise_loop

/// Acquires the shared noise-loop handle when its global ready bit is clear,
/// then initializes the selected scanner slot and clears its 16 report
/// records. The helper is called with the loop hash and a zero flag; its
/// returned handle is cached globally and stored in the slot.
lf_checker_rt::export!(thiscall, rw_009b1b30(this: u32, slot_index: u32) -> u32 {
    const SLOT_STRIDE: u32 = 0x1a0;
    const READY_FLAG: u32 = 0x0128e204;
    const NOISE_HANDLE: u32 = 0x0128e200;
    const NOISE_CONFIG: u32 = 0x0128aa84;
    const LOOP_HASH: u32 = 0x00e93728;
    const RECORD_COUNT: u32 = 16;
    const RECORD_BASE: u32 = 0xa4;
    const RECORD_STRIDE: u32 = 0x14;

    unsafe {
        let ready = lf_checker_rt::global::<u32>(READY_FLAG);
        let flags = ready.read_unaligned();
        let loop_handle = if flags & 1 == 0 {
            ready.write_unaligned(flags | 1);
            let handle = lf_checker_rt::callee_cdecl!(1, u32, LOOP_HASH, 0);
            lf_checker_rt::global::<u32>(NOISE_HANDLE).write_unaligned(handle);
            handle
        } else {
            lf_checker_rt::global::<u32>(NOISE_HANDLE).read_unaligned()
        };

        let slot = this.wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE));
        (slot.wrapping_add(0x1e8) as *mut u32).write_unaligned(loop_handle);
        (slot.wrapping_add(0x1ec) as *mut u32).write_unaligned(0x2ee);
        (slot.wrapping_add(0x098) as *mut u32).write_unaligned(0);
        (slot.wrapping_add(0x200) as *mut u16).write_unaligned(0x100);
        (slot.wrapping_add(0x1e0) as *mut u32).write_unaligned(0);
        (slot.wrapping_add(0x090) as *mut u32).write_unaligned(0);
        (slot.wrapping_add(0x094) as *mut u32).write_unaligned(0);
        (slot.wrapping_add(0x080) as *mut u32).write_unaligned(0);
        let config = lf_checker_rt::global::<u32>(NOISE_CONFIG).read_unaligned();
        (slot.wrapping_add(0x1e4) as *mut u32).write_unaligned(config);
        (slot.wrapping_add(0x1f0) as *mut u32).write_unaligned(0);
        (slot.wrapping_add(0x203) as *mut u8).write(1);
        (slot.wrapping_add(0x1f4) as *mut u32).write_unaligned(0);
        (slot.wrapping_add(0x1fb) as *mut u16).write_unaligned(0);
        (slot.wrapping_add(0x1f8) as *mut u16).write_unaligned(0);
        (slot.wrapping_add(0x1fa) as *mut u8).write(0);

        for index in 0..RECORD_COUNT {
            let record = slot.wrapping_add(RECORD_BASE).wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            (record as *mut u32).write_unaligned(0);
            (record.wrapping_add(4) as *mut u32).write_unaligned(0);
        }
        loop_handle
    }
});
