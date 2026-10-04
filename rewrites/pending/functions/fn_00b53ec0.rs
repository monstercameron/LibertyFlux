// original: 0x00b53ec0 rage::atDNode<CAnimChange, CAtdVirtualBase>::vf0
/// Scalar deleting destructor: stamps the node vtable, clears the key
/// words, releases the nullable child through its slot-2 virtual, stamps
/// the base vtable, and frees `this` when bit 0 of the flag is set.
/// Returns `this`.
export!(thiscall, rw_00b53ec0(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xEAF3B0);
        let child = *(this.add(0x0C) as *const u32);
        *(this.add(4) as *mut u16) = 0;
        *(this.add(6) as *mut u16) = 0xFFFF;
        *(this.add(8) as *mut u32) = 0;
        if child != 0 {
            let vt = *(child as *const u32);
            let tgt = *((vt as *const u8).add(8) as *const u32);
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            release(child);
            *(this.add(0x0C) as *mut u32) = 0;
        }
        *(this as *mut u32) = relocated(0xEAF398);
        if flag & 1 != 0 {
            callee_cdecl!(2, u32, this as u32);
        }
        this as u32
    }
});
