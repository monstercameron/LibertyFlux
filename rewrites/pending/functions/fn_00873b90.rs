// original: 0x00873b90 crmtManagerPriority::vf2
// Tear down both child slots: unregister a linked child through the shared
// helper, ask each live child to detach, and clear the slots. (thiscall/0)
export!(thiscall, rw_00873b90(this_ptr: u32) -> () {
    unsafe {
        let base = this_ptr as *mut u32;
        let slot8 = base.add(2).read();
        if slot8 != 0 {
            let link = (slot8 as *const u32).add(2).read();
            if link != 0 {
                callee_thiscall!(1, u32, link, slot8);
            }
            let child = base.add(2).read();
            let child_vt = (child as *const u32).read();
            let detach: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(((child_vt + 8) as *const u32).read() as usize);
            detach(child);
            base.add(2).write(0);
        }
        let slot4 = base.add(1).read();
        if slot4 != 0 {
            let other_vt = (slot4 as *const u32).read();
            let detach: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(((other_vt + 8) as *const u32).read() as usize);
            detach(slot4);
            base.add(1).write(0);
        }
    }
});
