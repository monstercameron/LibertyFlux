// original: 0x00cc7580 fragInstNMGta::fragInstNMGta
/// Construct a natural-motion fragment instance: base-construct, stamp the
/// vtables, construct the two embedded sub-objects, store the leading
/// argument and the back-link, and initialise the tag fields.
/// Returns the object pointer.
export!(thiscall, rw_00cc7580(this: *mut u8, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this as u32, a1, a2, a3);
        *(this as *mut u32) = relocated(0xED994C);
        *(this.add(0x50) as *mut u32) = relocated(0xED9A58);
        let _: u32 = callee_thiscall!(2, u32, (this as u32).wrapping_add(0x100));
        let _: u32 = callee_thiscall!(3, u32, (this as u32).wrapping_add(0x2A0));
        *(this.add(0xB0) as *mut u32) = a0;
        *(this.add(0xB4) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0xB8) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0xC) as *mut u32) = 0;
        *(this.add(0x290) as *mut u32) = this as u32;
        *(this.add(0x2E0) as *mut u16) = 0;
        this as u32
    }
});
