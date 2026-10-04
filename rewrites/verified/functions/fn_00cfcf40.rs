// original: 0x00cfcf40 CTaskComplexCombatInvestigateSubtask::~CTaskComplexCombatInvestigateSubtask
/// Destructor: stamps the transitional virtual table, releases each of
/// the two guarded handles at offsets 0x18 and 0x4c when set (releasing
/// clears the slot), then tails into the shared base destructor and
/// returns its result.
lf_rs85_rt::export!(thiscall, rw_00cfcf40(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = lf_rs85_rt::relocated(0x00EDFFE4);
        for off in [0x18usize, 0x4C] {
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
