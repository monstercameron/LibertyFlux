// original: 0x00a64a30 PedDefensiveAreaSetup
/// Sets up the two defensive-area records and orders them.
///
/// Registers the flag through the area helpers, folds the mode byte's low
/// bit into flag bit 1 (bit 0 is set), copies the two source records and
/// the trailing float, then swaps the records when the first record's third
/// float is ordered-above the second's. The swapped-in low word comes from
/// uninitialized stack in the original, which reads as zero under the
/// checker's defined stack fill. Clears flag bit 2 last. Returns nothing.
export!(thiscall, rw_00a64a30(this: u32, p1: u32, p2: u32, farg: u32, flag: u32, mode: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        if flag != 0 {
            *((this + 0x24) as *mut u32) = flag;
            callee_thiscall!(2, u32, flag, this + 0x24);
        }
        let m = *((this + 0x28) as *const u32);
        *((this + 0x28) as *mut u32) = (m & !3) | (((mode & 1) << 1) | 1);
        *((this) as *mut u32) = *(p1 as *const u32);
        *((this + 4) as *mut u32) = *((p1 + 4) as *const u32);
        *((this + 8) as *mut u32) = *((p1 + 8) as *const u32);
        *((this + 0xC) as *mut u32) = *((p1 + 0xC) as *const u32);
        *((this + 0x10) as *mut u32) = *(p2 as *const u32);
        *((this + 0x14) as *mut u32) = *((p2 + 4) as *const u32);
        *((this + 0x18) as *mut u32) = *((p2 + 8) as *const u32);
        *((this + 0x1C) as *mut u32) = *((p2 + 0xC) as *const u32);
        *((this + 0x20) as *mut u32) = farg;
        let z0 = *((this + 8) as *const f32);
        let z1 = *((this + 0x18) as *const f32);
        if z0 > z1 {
            let a0 = *(this as *const u32);
            let a1 = *((this + 4) as *const u32);
            let a2 = *((this + 8) as *const u32);
            let a3 = *((this + 0xC) as *const u32);
            let b0 = *((this + 0x10) as *const u32);
            let b1 = *((this + 0x14) as *const u32);
            let b2 = *((this + 0x18) as *const u32);
            *((this + 0x10) as *mut u32) = a0;
            *((this + 0x14) as *mut u32) = a1;
            *((this + 0x18) as *mut u32) = a2;
            *((this + 0x1C) as *mut u32) = a3;
            *(this as *mut u32) = b0;
            *((this + 4) as *mut u32) = b1;
            *((this + 8) as *mut u32) = b2;
            *((this + 0xC) as *mut u32) = 0;
        }
        *((this + 0x28) as *mut u32) &= !4;
    }
    0
});
