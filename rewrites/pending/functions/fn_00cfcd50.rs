// original: 0x00cfcd50 CTaskComplexCombat::~CTaskComplexCombat
/// Destructor: stamps the transitional virtual table, clears a flag byte,
/// releases each of the four guarded handles at offsets 0x3c, 0x40, 0x48
/// and 0x4c when set (releasing clears the slot), then tails into the
/// shared base destructor and returns its result.
lf_rs85_rt::export!(thiscall, rw_00cfcd50(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = lf_rs85_rt::relocated(0x00EDFC7C);
        *(this.add(0x38) as *mut u8) = 0;
        for off in [0x3Cusize, 0x40, 0x48, 0x4C] {
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
