// original: 0x008761b0 crmt_addsubtract_request_teardown
/// Tear down a combiner request: release the child, uninit the source.
///
/// thiscall/0. Releases the attached child through slot 2 of its vtable,
/// clears the fields, then tears down the embedded source object in two
/// steps through the shared teardown helper.
export!(thiscall, rw_008761b0(this: *mut u8) -> () {
    unsafe {
        *(this as *mut u32) = relocated(0x00fe81e4);
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
        *(this as *mut u32) = relocated(0x00fe7fc8);
        let sub = this.add(4);
        let member = *(this.add(0x0c) as *const u32);
        if member != 0 {
            callee_thiscall!(2, u32, member, sub as u32);
        }
        let follower = *(sub.add(8) as *const u32);
        *(sub as *mut u32) = relocated(0x00fe7fb4);
        if follower != 0 {
            callee_thiscall!(2, u32, follower, sub as u32);
        }
        *(sub as *mut u32) = relocated(0x00e86afc);
    }
});
