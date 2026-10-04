// original: 0x00B6DA30 task_dtor_rel1c_gated_f_c47a
/// Destroy a task: release the owned handle, stamp the vtable, unlink the
/// link only while the peer still references it, run the float update, flag
/// the peer, then run the base destructor.
export!(thiscall, rw_00b6da30(this: *mut u8) -> u32 {
    unsafe {
        let field = *((this.add(0x1c)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB1CA4);
        if field != 0 {
            callee_thiscall!(1, u32, field, (this.add(0x1c)) as u32);
        }
        let link = *((this.add(0x18)) as *mut u32);
        if link != 0 {
            if *((link.wrapping_add(0x24)) as *const u32) != 0 {
        callee_thiscall!(3, u32, link, this as u32);
    }
            callee_thiscall!(2, u32, link, 0xC47A0000);
            *((link.wrapping_add(4)) as *mut u32) |= 0x4000;
            *((this.add(0x18)) as *mut u32) = 0;
        }
        callee_thiscall!(4, u32, this as u32)
    }
});
