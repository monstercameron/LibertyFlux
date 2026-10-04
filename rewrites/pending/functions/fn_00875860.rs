// original: 0x00875860 rage::crmtRequestFilter::vf1
/// Release the filter child and clear the request fields.
///
/// thiscall/0 (`rage::crmtRequestFilter::vf1`). If a child is attached,
/// releases it through slot 2 of its vtable, then clears both fields.
export!(thiscall, rw_00875860(this: *mut u8) -> () {
    unsafe {
        let child = *(this.add(0x18) as *const u32);
        if child != 0 {
            let vtable = *(child as *const u32);
            let release: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *((vtable as *const u8).add(8) as *const u32) as usize,
            );
            release(child);
        }
        *(this.add(0x18) as *mut u32) = 0;
        *(this.add(0x14) as *mut u32) = 0;
    }
});
