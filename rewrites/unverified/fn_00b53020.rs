// original: 0x00B53020 euphoria_step_live_slots

/// For each of three slot records spaced `0x520` bytes apart, call the
/// intercepted update routine only when the record's first dword is nonzero.
/// Each call receives the record in ECX, then the slot index and the elapsed
/// float's unchanged 32-bit word on the stack. This is a void thiscall method
/// with one float stack argument; its EAX value is unspecified.
lf_checker_rt::export!(thiscall, rw_b53020(this: *mut u8, elapsed: f32) -> () {
    const SLOT_STRIDE: usize = 0x520;
    const SLOT_COUNT: usize = 3;
    unsafe {
        for slot_index in 0..SLOT_COUNT {
            let slot = this.add(slot_index * SLOT_STRIDE);
            let active = (slot as *const u32).read_unaligned();
            if active != 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    1,
                    u32,
                    slot as u32,
                    slot_index as u32,
                    elapsed.to_bits(),
                );
            }
        }
    }
});
