// original: 0x00b4e850 CDummyPed::vf13

/// Test whether the dummy ped's task holder is settled or idle.
///
/// The holder at `this + 0x6C` decides: null means settled (returns 1), a
/// non-zero flag byte at holder `+0x0E` means busy (returns 0). Otherwise
/// the holder's virtual slot at `+0x60` is asked; a zero answer means busy
/// (0), anything else settled (1).
///
/// Original: 0x00b4e850 (thiscall, no stack words; boolean in AL).
lf_checker_rt::export!(thiscall, rw_00b4e850(this: u32) -> u32 {
    unsafe {
        const HOLDER: u32 = 0x6c;
        const BUSY_FLAG: u32 = 0x0e;
        const READY_SLOT: u32 = 0x60;
        let holder = ((this + HOLDER) as *const u32).read_unaligned();
        if holder == 0 {
            return 1;
        }
        if ((holder + BUSY_FLAG) as *const u8).read() != 0 {
            return 0;
        }
        let vt = (holder as *const u32).read_unaligned();
        let ready: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + READY_SLOT) as *const u32).read_unaligned() as usize);
        u32::from((ready(holder) & 0xff) != 0)
    }
});
