// original: 0x008B1110 rage::audNullEffect::vf5 (merged symbol)

/// Advance the effect's 3-step phase counter after running the base update.
///
/// Calls the base-class update (`0x8a9370`, thiscall, no arguments), then
/// stores `([this+0x30] + 1) mod 3` back at `+0x30` (the original uses a
/// `div` by 3 and keeps the remainder). No meaningful return value.
/// Original is thiscall with no stack words (plain `ret`).
lf_checker_rt::export!(thiscall, rw_008B1110(this: u32) -> u32 {
    const BASE_UPDATE: u32 = 1;
    const PHASE: u32 = 0x30;
    const STEPS: u32 = 3;
    unsafe {
        lf_checker_rt::callee_thiscall!(BASE_UPDATE, u32, this);
        let cur = ((this + PHASE) as *const u32).read_unaligned();
        ((this + PHASE) as *mut u32).write_unaligned(cur.wrapping_add(1) % STEPS);
    }
    0
});
