// original: 0x00c3e460 train_slot_by_index (proposed)
/// Return a pointer to one of four slots selected by `idx`.
///
/// `this` (ECX) is the car. For idx 0..3 returns `this+0x10`, `this`,
/// `this+0x20`, `this+0x30` respectively (the original dispatches through
/// a jump table; the rewrite matches directly); any other idx returns
/// null. No calls.
///
/// Original: 0x00c3e460 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c3e460(this: u32, idx: u32) -> u32 {
    match idx {
        0 => this.wrapping_add(0x10),
        1 => this,
        2 => this.wrapping_add(0x20),
        3 => this.wrapping_add(0x30),
        _ => 0,
    }
});
