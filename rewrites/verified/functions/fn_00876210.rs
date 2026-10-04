// original: 0x00876210 rage::crmtRequestAddSubtract::vf1
/// Release the combiner child and clear the request fields.
///
/// thiscall/0 (`rage::crmtRequestAddSubtract::vf1`). If a child is
/// attached, releases it through slot 2 of its vtable, then clears the
/// child slot and the two parameter fields.
export!(thiscall, rw_00876210(this: *mut u8) -> () {
    unsafe {
        let child = *(this.add(0x20) as *const u32);
        if child != 0 {
            let vtable = *(child as *const u32);
            let release: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *((vtable as *const u8).add(8) as *const u32) as usize,
            );
            release(child);
        }
        *(this.add(0x20) as *mut u32) = 0;
        *(this.add(0x18) as *mut u32) = 0;
        *(this.add(0x14) as *mut u32) = 0;
    }
});
