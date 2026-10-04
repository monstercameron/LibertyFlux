// original: 0x00cfcf10 CTaskComplexCombatFlankSubtask::~CTaskComplexCombatFlankSubtask
/// Destructor: stamps the transitional virtual table, destroys the
/// embedded members at offsets 0x4c and 0x40 in that order, then tails
/// into the shared base destructor and returns its result.
lf_rs85_rt::export!(thiscall, rw_00cfcf10(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = lf_rs85_rt::relocated(0x00EDFE24);
        lf_rs85_rt::callee_thiscall!(1, u32, this.add(0x4C) as u32);
        lf_rs85_rt::callee_thiscall!(2, u32, this.add(0x40) as u32);
        lf_rs85_rt::callee_thiscall!(9, u32, this as u32)
    }
});
