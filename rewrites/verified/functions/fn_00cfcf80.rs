// original: 0x00cfcf80 CTaskComplexCombatPersueInCarSubtask::~CTaskComplexCombatPersueInCarSubtask
/// Destructor: stamps the transitional virtual table, releases each of
/// the two guarded handles at offsets 0x40 and 0x18 when set (releasing
/// clears the slot), then tails into the shared base destructor and
/// returns its result.
lf_rs85_rt::export!(thiscall, rw_00cfcf80(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = lf_rs85_rt::relocated(0x00EE0054);
        for off in [0x40usize, 0x18] {
            let slot = this.add(off) as *mut u32;
            let held = *slot;
            if held != 0 {
                lf_rs85_rt::callee_thiscall!(1, u32, held, slot as u32);
                *slot = 0;
            }
        }
        lf_rs85_rt::callee_thiscall!(9, u32, this as u32)
    }
});
