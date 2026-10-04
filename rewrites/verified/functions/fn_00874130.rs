// original: 0x00874130 crmt_conditional_slot_replace
// Conditional slot replace: when the dirty flag is clear just store the new
// child; otherwise detach the old child first and clear the flag.
// (thiscall/1)
export!(thiscall, rw_00874130(this_ptr: u32, child: u32) -> () {
    unsafe {
        let holder = (this_ptr as *const u32).add(3).read();
        let flag = holder.wrapping_add(8) as *mut u8;
        if flag.read() == 0 {
            (holder as *mut u32).add(1).write(child);
        } else if child != 0 {
            let old = (holder as *const u32).add(1).read();
            if old != 0 {
                let old_vt = (old as *const u32).read();
                let detach: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute((old_vt as *const u32).read() as usize);
                detach(old, 1);
            }
            (holder as *mut u32).add(1).write(child);
            flag.write(0);
        }
    }
});
