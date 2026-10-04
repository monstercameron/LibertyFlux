// original: 0x00B6D900 task_dtor_rel24_unlink_f_c080_pedflag
/// Destroy a task: release the owned handle, stamp the vtable, unlink the
/// link before the float update, flag the peer, consume a pending
/// player-flag by marking the player object, then run the base destructor.
export!(thiscall, rw_00b6d900(this: *mut u8) -> u32 {
    unsafe {
        let field = *((this.add(0x24)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB1AEC);
        if field != 0 {
            callee_thiscall!(1, u32, field, (this.add(0x24)) as u32);
        }
        let link = *((this.add(0x18)) as *mut u32);
        if link != 0 {
            callee_thiscall!(3, u32, link, this as u32);
            callee_thiscall!(2, u32, link, 0xC0800000);
            *((link.wrapping_add(4)) as *mut u32) |= 0x4000;
            *((this.add(0x18)) as *mut u32) = 0;
        }
        if *this.add(0x2d) != 0 {
            *this.add(0x2d) = 0;
            let ped = callee_stdcall!(4, u32,);
            *((ped.wrapping_add(0x26c)) as *mut u32) |= 0x20000;
        }
        callee_thiscall!(5, u32, this as u32)
    }
});
