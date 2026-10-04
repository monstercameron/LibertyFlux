// original: 0x00875830 crmt_request_retarget
/// Retarget a filter request: store the tag, attach the new child.
///
/// thiscall/2. Stores the tag word, acquires the incoming child through
/// slot 1 of its vtable, releases the previously attached child through
/// slot 2 of its vtable, and stores the incoming child. Either child may
/// be null, in which case its call is skipped.
export!(thiscall, rw_00875830(this: *mut u8, tag: u32, new_child: u32) -> () {
    unsafe {
        *(this.add(0x14) as *mut u32) = tag;
        if new_child != 0 {
            let vtable = *(new_child as *const u32);
            let acquire: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *((vtable as *const u8).add(4) as *const u32) as usize,
            );
            acquire(new_child);
        }
        let old = *(this.add(0x18) as *const u32);
        if old != 0 {
            let vtable = *(old as *const u32);
            let release: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *((vtable as *const u8).add(8) as *const u32) as usize,
            );
            release(old);
        }
        *(this.add(0x18) as *mut u32) = new_child;
    }
});
