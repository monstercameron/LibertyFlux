// original: 0x00B6D7D0 task_dtor_rel28_float_f_c47a_noflag
/// Destroy a task: release the owned handle, stamp the vtable, retire the
/// link with a float update and an unlink without flagging the peer, then
/// run the base destructor.
export!(thiscall, rw_00b6d7d0(this: *mut u8) -> u32 {
    unsafe {
        let field = *((this.add(0x28)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB1BF4);
        if field != 0 {
            callee_thiscall!(1, u32, field, (this.add(0x28)) as u32);
        }
        let link = *((this.add(0x18)) as *mut u32);
        if link != 0 {
            callee_thiscall!(2, u32, link, 0xC47A0000);
            callee_thiscall!(3, u32, link, this as u32);
            *((this.add(0x18)) as *mut u32) = 0;
        }
        callee_thiscall!(4, u32, this as u32)
    }
});
