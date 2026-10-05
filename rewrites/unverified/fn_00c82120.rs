// original: 0x00c82120 scenario_scratch_store (proposed) — UNVERIFIED (deferred)

// NOTE: deferred with reason `other`: after aligning the stack pointer the
// function loads a float from *below* its own entry ESP (caller scratch the
// contract cannot name) and stores it at `this+0xac`. That value cannot be
// expressed as a rewrite input, so the rewrite below is structural only: the
// loaded value is marked unrepresentable. Never passed, not verified.

/// Store the caller's scratch float into `this+0xac`; return 0.
///
/// Original: thiscall, no stack words (plain `ret`).
lf_checker_rt::export!(thiscall, rw_00c82120(this: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 0xac;
        // UNREPRESENTABLE: the original reads this word from below its entry
        // ESP (out-of-bounds caller scratch); 0 is a placeholder, not behaviour.
        let v: u32 = 0;
        ((this + SLOT_OFF) as *mut u32).write_unaligned(v);
        0
    }
});
