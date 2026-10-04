// original: 0x00a1e910 cam_slot_emit_stride_e0 (proposed)

/// Emits one slotted record through a worker, allocating the slot on -1.
///
/// Same shape as `rw_00a1e8b0` with a different slot table: the counter
/// lives at `+COUNT_OFF`, the callee gets
/// `this + BASE_OFF + idx * STRIDE` in `ecx` with `(a0, a1, a2, a3)` on
/// the stack, the byte at `this + idx + MARK_OFF` is set to 1, and the
/// callee's return value is returned. `a4` is the index or -1 to allocate.
///
/// Original: 0x00a1e910 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00a1e910(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        const COUNT_OFF: u32 = 0x408;
        const BASE_OFF: u32 = 0x200;
        const STRIDE: u32 = 0xe0;
        const MARK_OFF: u32 = 0x412;
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
