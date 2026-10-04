// original: 0x00a1e8b0 cam_slot_emit_stride_c0 (proposed)

/// Emits one slotted record through a worker, allocating the slot on -1.
///
/// `this` points to a record with a slot counter at `+COUNT_OFF`. The
/// fifth argument `a4` is a slot index, or -1 to take the next counter
/// value (the counter is then incremented). The worker callee is invoked
/// with `this + BASE_OFF + idx * STRIDE` in `ecx` and `(a0, a1, a2, a3)`
/// on the stack (`a1` is a float carried as bits), the byte at
/// `this + idx + MARK_OFF` is set to 1, and the callee's return value is
/// returned.
///
/// Original: 0x00a1e8b0 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00a1e8b0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        const COUNT_OFF: u32 = 0x404;
        const BASE_OFF: u32 = 0x140;
        const STRIDE: u32 = 0xc0;
        const MARK_OFF: u32 = 0x411;
        let mut idx = a4;
        if idx == 0xffff_ffff {
            idx = ((this + COUNT_OFF) as *const u32).read_unaligned();
            ((this + COUNT_OFF) as *mut u32).write_unaligned(idx.wrapping_add(1));
        }
        let r = lf_checker_rt::callee_thiscall!(
            CALLEE,
            u32,
            this + BASE_OFF + idx * STRIDE,
            a0,
            a1,
            a2,
            a3
        );
        ((this + idx + MARK_OFF) as *mut u8).write(1);
        r
    }
});
