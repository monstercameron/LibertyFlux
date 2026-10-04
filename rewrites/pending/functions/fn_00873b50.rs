// original: 0x00873b50 crmt_release_child_slot
// Release the child held in slot 4: publish the base vtable first, then ask
// a live child to detach itself and clear the slot. (thiscall/0)
export!(thiscall, rw_00873b50(this_ptr: u32) -> () {
    unsafe {
        let base = this_ptr as *mut u32;
        let child = base.add(1).read();
        base.write(relocated(0x00FE7F10));
        if child != 0 {
            let vt = (child as *const u32).read();
            let detach: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(((vt + 8) as *const u32).read() as usize);
            detach(child);
            base.add(1).write(0);
        }
    }
});
