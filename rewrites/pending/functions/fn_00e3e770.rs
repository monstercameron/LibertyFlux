// original: 0x00e3e770 StatsMeter_Restart
// 0x00E3E770: refresh a meter record when the stats gate is open, then
// tail-dispatch to the accumulator. (thiscall/0, tail jump)
//
// The probe buffers live on the caller's frame on both sides; only their
// addresses differ, so the contract skips the pointer arguments while the
// checked-in words still flow into the record identically.
export!(thiscall, rw_00e3e770(this: *mut u8) -> u32 {
    unsafe {
        if *global::<u8>(0x11609F6) == 0 {
            return 0;
        }
        *(this.add(0x20) as *mut u32) = 0x3F80_0000;
        *(this.add(8) as *mut u32) = 0;
        *(this.add(0x14)) = 0u8;
        *this = 0u8;
        *(this.add(0x0c) as *mut u32) = 0;
        *(this.add(0x10) as *mut u32) = 0;
        let mut probe: [u32; 4] = [0; 4];
        let info = callee_cdecl!(1, u32, probe.as_mut_ptr() as u32, 0x32);
        *(this.add(0x1c) as *mut u32) = *(info as *const u32);
        let mut sample: [u32; 8] = [0; 8];
        callee_cdecl!(2, u32, sample.as_mut_ptr() as u32);
        callee_cdecl!(3, u32, 2, sample.as_mut_ptr() as u32, 0, 0);
        *(this.add(0x24) as *mut u32) = sample[1];
        callee_thiscall!(4, u32, this as u32)
    }
});
