// original: 0x006f51c0 list_drain_teardown
/// Tears down the owned sub-objects and drains the pending list.
///
/// Runs only while bit 1 of the status byte is set, and clears it at the end.
/// Releases the slot-5 and slot-13 channels through the shared closers when
/// occupied, detaches the child object, then walks the pending list at slot 3,
/// handing each node to the owner's release handler. Slots holding zero are
/// skipped without a call.
export!(thiscall, rw_006f51c0(this: u32) -> () {
    unsafe {
        let obj = this as *mut u32;
        if *((this + 0x5c) as *const u8) & 2 == 0 {
            return;
        }
        let close_a: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(*global::<u32>(0xE731CC));
        let close_b: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(*global::<u32>(0xE731C8));
        if *obj.add(5) != 0 {
            close_a(this.wrapping_add(0x14));
        }
        let mut status = *((this + 0x5c) as *const u8);
        if status & 2 != 0 {
            status &= 0xFE;
            *((this + 0x5c) as *mut u8) = status;
        }
        let child = *obj.add(1);
        if child != 0 {
            callee_thiscall!(3, u32, child, this);
        }
        let owner = *obj.add(2);
        *obj.add(2) = 0;
        if *obj.add(5) != 0 {
            close_b(this.wrapping_add(0x14));
        }
        if owner != 0 {
            if *obj.add(0x34 / 4) != 0 {
                close_a(this.wrapping_add(0x34));
            }
            let mut node = *obj.add(3);
            while node != 0 {
                let next = *(node as *const u32).add(4);
                *obj.add(3) = next;
                let table = *(owner as *const u32);
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(*(table as *const u32).add(3));
                release(owner, node);
                node = *obj.add(3);
            }
            if *obj.add(0x34 / 4) != 0 {
                close_b(this.wrapping_add(0x34));
            }
        }
        *((this + 0x5c) as *mut u8) &= 0xFD;
    }
});
