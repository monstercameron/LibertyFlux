// original: 0x00cfd030 CTaskComplexCombatSeekCoverSubtask::~CTaskComplexCombatSeekCoverSubtask
/// Destructor: stamps the transitional virtual table, releases the
/// guarded handle at offset 0x18 when set (releasing clears the slot),
/// then tails into the shared base destructor and returns its result.
lf_rs85_rt::export!(thiscall, rw_00cfd030(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = lf_rs85_rt::relocated(0x00EDFE94);
        let slot = this.add(0x18) as *mut u32;
        let held = *slot;
        if held != 0 {
            lf_rs85_rt::callee_thiscall!(1, u32, held, slot as u32);
            *slot = 0;
        }
        lf_rs85_rt::callee_thiscall!(9, u32, this as u32)
    }
});
