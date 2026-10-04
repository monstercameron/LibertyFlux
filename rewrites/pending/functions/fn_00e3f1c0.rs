// original: 0x00e3f1c0 StatSlot_TrySeal
// 0x00E3F1C0: seal a stats slot when the readiness probe answers 1.
// Returns the probe answer. (thiscall/0)
export!(thiscall, rw_00e3f1c0(this: *mut u8) -> u32 {
    unsafe {
        let ready = callee_cdecl!(1, u32, this as u32, 4, 1);
        if ready == 1 {
            *(this.add(4) as *mut i32) = -1;
        }
        ready
    }
});
