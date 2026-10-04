// original: 0x00875880 rage::crmtRequestFilter::vf2
/// Clone a filter request: build the copy, rewire the child, link it in.
///
/// thiscall/2 (`rage::crmtRequestFilter::vf2`). Builds a fresh request
/// from the tag, acquires this request's child for the copy, releases the
/// copy's previous occupant, stores the child into the copy and links the
/// copy into the graph. Returns the fresh request.
export!(thiscall, rw_00875880(this: *mut u8, tag: u32, mode: u32) -> u32 {
    unsafe {
        let fresh: u32 = callee_cdecl!(1, u32, tag);
        let child = *(this.add(0x18) as *const u32);
        if child != 0 {
            let vtable = *(child as *const u32);
            let acquire: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *((vtable as *const u8).add(4) as *const u32) as usize,
            );
            acquire(child);
        }
        let occupant = *((fresh as *const u8).add(0x20) as *const u32);
        if occupant != 0 {
            let vtable = *(occupant as *const u32);
            let release: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *((vtable as *const u8).add(8) as *const u32) as usize,
            );
            release(occupant);
        }
        *((fresh as *mut u8).add(0x20) as *mut u32) = child;
        callee_thiscall!(4, u32, this as u32, tag, mode, fresh);
        fresh
    }
});
