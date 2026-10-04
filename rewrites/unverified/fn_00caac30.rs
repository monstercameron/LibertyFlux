// original: 0x00CAAC30 task_slot_update (proposed)

/// Update the handler's task slots from an event, by kind and freshness.
///
/// Asks slot 4 of the event's (first stack argument) virtual table for its
/// kind; kind 0x20 needs no update. Otherwise asks slot 0x4C whether the
/// event is fresh: a fresh event with a zero freshness byte (low byte of the
/// second stack argument) takes the live path, anything else the stale path.
/// The live path shifts the current task out (running the shift helper when
/// the current slot is non-null, then clearing it), releases the spare slot
/// when non-null, and installs a clone made by the event's slot 0x10 into
/// the spare slot. The stale path releases the current slot when non-null,
/// installs the event's slot-0x10 clone into the current slot, releases the
/// auxiliary slot when non-null and clears it, then releases the spare slot
/// when non-null and clears it. The first stack argument is not read. No
/// return value.
///
/// Original: 0x00CAAC30 (thiscall, three stack words, the last two read).
lf_checker_rt::export!(thiscall, rw_00caac30(this: u32, _a0: u32, event: u32, fresh: u32) -> u32 {
    unsafe {
        const SHIFTER: u32 = 5;
        const CURRENT: u32 = 0x04;
        const SPARE: u32 = 0x08;
        const AUX: u32 = 0x0C;
        const KIND_SLOT: u32 = 0x04;
        const FRESH_SLOT: u32 = 0x4C;
        const CLONE_SLOT: u32 = 0x10;
        const RELEASE_SLOT: u32 = 0x00;
        const RELEASE_ARG: u32 = 1;
        const NO_UPDATE_KIND: u32 = 0x20;
        unsafe fn vcall(obj: u32, slot_off: u32) -> u32 {
            unsafe {
                let vtable = (obj as *const u32).read_unaligned();
                let slot = (vtable.wrapping_add(slot_off) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj)
            }
        }
        unsafe fn release(obj: u32) {
            unsafe {
                let vtable = (obj as *const u32).read_unaligned();
                let slot = (vtable as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj, RELEASE_ARG);
            }
        }
        if vcall(event, KIND_SLOT) == NO_UPDATE_KIND {
            return 0;
        }
        if (vcall(event, FRESH_SLOT) as u8) != 0 && (fresh & 0xFF) == 0 {
            if (this.wrapping_add(CURRENT) as *const u32).read_unaligned() != 0 {
                lf_checker_rt::callee_thiscall!(SHIFTER, u32, this);
                (this.wrapping_add(CURRENT) as *mut u32).write_unaligned(0);
            }
            let spare = (this.wrapping_add(SPARE) as *const u32).read_unaligned();
            if spare != 0 {
                release(spare);
            }
            let task = vcall(event, CLONE_SLOT);
            (this.wrapping_add(SPARE) as *mut u32).write_unaligned(task);
            return 0;
        }
        let current = (this.wrapping_add(CURRENT) as *const u32).read_unaligned();
        if current != 0 {
            release(current);
        }
        let task = vcall(event, CLONE_SLOT);
        (this.wrapping_add(CURRENT) as *mut u32).write_unaligned(task);
        let aux = (this.wrapping_add(AUX) as *const u32).read_unaligned();
        if aux != 0 {
            release(aux);
        }
        (this.wrapping_add(AUX) as *mut u32).write_unaligned(0);
        let spare = (this.wrapping_add(SPARE) as *const u32).read_unaligned();
        if spare != 0 {
            release(spare);
        }
        (this.wrapping_add(SPARE) as *mut u32).write_unaligned(0);
        0
    }
});
