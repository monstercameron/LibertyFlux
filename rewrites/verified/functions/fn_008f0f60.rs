// original: 0x008f0f60 budget_remaining
/// Remaining budget: a global counter minus the object's consumed amount.
export!(thiscall, rw_008f0f60(this: u32) -> u32 {
    const USED_OFF: u32 = 0x3A6C;
    let total = unsafe { *global::<u32>(0x11735B4) };
    let used = unsafe { *((this.wrapping_add(USED_OFF)) as *const u32) };
    total.wrapping_sub(used)
});
