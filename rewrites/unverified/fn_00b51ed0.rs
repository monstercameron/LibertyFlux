// original: 0x00B51ED0 euphoria_slot_rebind

/// Rebind the slot's container handle at byte offset `0x430`. If its old
/// handle is nonzero, call the release routine with that handle in ECX and a
/// pointer to the handle field on the stack. Copy the relocated global value
/// into offset `0x08`, install the new handle, and call its initialization
/// routine only when nonzero. Then clear the slot's counters, flags, and four
/// 64-bit state blocks. This is a thiscall method with one stack argument;
/// its EAX return register is not meaningful.
lf_checker_rt::export!(thiscall, rw_b51ed0(this: *mut u8, new_handle: u32) -> () {
    const HANDLE_OFFSET: usize = 0x430;
    const GLOBAL_STATE_VA: u32 = 0x011735b4;
    unsafe {
        let old_handle = (this.add(HANDLE_OFFSET) as *const u32).read_unaligned();
        let handle_field = this.add(HANDLE_OFFSET) as u32;
        (this as *mut u32).write_unaligned(0);
        (this.add(0x0c) as *mut u32).write_unaligned(0);
        let global_state = lf_checker_rt::global::<u32>(GLOBAL_STATE_VA).read_unaligned();
        (this.add(0x08) as *mut u32).write_unaligned(global_state);
        if old_handle != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, old_handle, handle_field);
        }
        (this.add(HANDLE_OFFSET) as *mut u32).write_unaligned(new_handle);
        if new_handle != 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, new_handle, handle_field);
        }
        (this.add(0x10) as *mut u64).write_unaligned(0);
        (this.add(0x18) as *mut u64).write_unaligned(0);
        (this.add(0x20) as *mut u64).write_unaligned(0);
        (this.add(0x28) as *mut u64).write_unaligned(0);
        this.add(0x04).write(0);
        (this.add(0x434) as *mut u32).write_unaligned(0);
        (this.add(0x460) as *mut u32).write_unaligned(0);
        (this.add(0x464) as *mut u32).write_unaligned(0);
        (this.add(0x468) as *mut u32).write_unaligned(0);
        (this.add(0x46c) as *mut u32).write_unaligned(0);
        (this.add(0x470) as *mut u32).write_unaligned(0);
    }
});
