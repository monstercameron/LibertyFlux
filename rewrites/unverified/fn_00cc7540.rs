// original: 0x00cc7540 fragInstGta::fragInstGta_2
/// Construct a default fragment instance: run the parameterless base
/// constructor, stamp the vtables, set the kind tag at +0xA0 to 13 and zero
/// the state fields. Returns the object pointer.
export!(thiscall, rw_00cc7540(this: *mut u8) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(0xED9814);
        *(this.add(0x50) as *mut u32) = relocated(0xED991C);
        *(this.add(0xA0) as *mut u32) = 13;
        *(this.add(0xA4) as *mut u32) = 0;
        *(this.add(0xC) as *mut u32) = 0;
        this as u32
    }
});
