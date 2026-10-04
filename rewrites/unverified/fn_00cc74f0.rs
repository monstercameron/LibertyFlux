// original: 0x00cc74f0 fragInstGta::fragInstGta
/// Construct a fragment instance: run the base constructor with the trailing
/// arguments, store the leading argument at +0xA0, stamp the vtables and zero
/// the state fields. Returns the object pointer.
export!(thiscall, rw_00cc74f0(this: *mut u8, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this as u32, a1, a2, a3);
        *(this.add(0xA0) as *mut u32) = a0;
        *(this as *mut u32) = relocated(0xED9814);
        *(this.add(0x50) as *mut u32) = relocated(0xED991C);
        *(this.add(0xA4) as *mut u32) = 0;
        *(this.add(0xC) as *mut u32) = 0;
        this as u32
    }
});
