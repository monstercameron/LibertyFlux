// original: 0x00B55090 crmtManagerPriority::vf7

/// Notify channel `idx`, then close the gap by sliding entries down.
///
/// The worker at callee 1 runs first with (`this`, `idx`); its answer is
/// ignored. Then entries above `idx` in the 32-entry table at `this` +
/// 0x828 slide one slot towards lower indices (entry `i` moves to `i-1`
/// for `i` from `idx+1` through 31), closing the gap. When `idx+1`
/// (wrapping) is 32 or more (compared unsigned) nothing slides. Returns
/// `idx+1`.
///
/// Original: 0x00B55090 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b55090(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x828;
        const ENTRIES: u32 = 32;
        const NOTIFY: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, idx);
        let next = idx.wrapping_add(1);
        if next < ENTRIES {
            let dst = (this + TABLE - 4).wrapping_add(next.wrapping_mul(4));
            let src = dst.wrapping_add(4);
            let count = (ENTRIES - next) as usize;
            core::ptr::copy(src as *const u32, dst as *mut u32, count);
        }
        next
    }
});
