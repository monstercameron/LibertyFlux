// original: 0x00b710b0 CTaskSimpleCarSlowBeDraggedOut::vf5

/// Task event handler: drive the drag-out on event 2, release on event 1.
///
/// Event 2 mirrors the sibling drag handler: with no target at `this+0x18`
/// reports done; else calls release (thiscall on the target, -1000.0f) and,
/// unless the vehicle at `this+0x24` is null, notify (thiscall on the
/// vehicle; pushed 0, target, `this+0x28`, first word). Event 1 takes the
/// object from the third word: null, or a type query through its vtable
/// (slot 1) answering anything but 0x82, is ignored; with no target reports
/// done; else calls release, then detach (thiscall on the target, one stack
/// word: `this`), clears the target slot and reports done. Any other event
/// is ignored. Every return path sets eax from a fully known value (the
/// event id is loaded whole up front, later paths from call answers), so
/// the full dword is compared with no pinning.
///
/// Original: 0x00b710b0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00b710b0(this: u32, a1: u32, event: u32, a3: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x18;
        const VEHICLE_OFF: u32 = 0x24;
        const EXTRA_OFF: u32 = 0x28;
        const DRAG_EVENT: u32 = 2;
        const RELEASE_EVENT: u32 = 1;
        const WANT_TYPE: u32 = 0x82;
        const EVENT_SPEED: u32 = 0xc47a0000; // -1000.0f
        const RELEASE: u32 = 1;
        const NOTIFY: u32 = 2;
        const TYPE_QUERY: u32 = 3;
        const DETACH: u32 = 4;
        if event == DRAG_EVENT {
            let target = ((this + TARGET_OFF) as *const u32).read_unaligned();
            if target == 0 {
                return 1;
            }
            let ans1: u32 =
                lf_checker_rt::callee_thiscall!(RELEASE, u32, target, EVENT_SPEED);
            let vehicle = ((this + VEHICLE_OFF) as *const u32).read_unaligned();
            if vehicle == 0 {
                return (ans1 & 0xffff_ff00) | 1;
            }
            let extra = ((this + EXTRA_OFF) as *const u32).read_unaligned();
            let ans2: u32 =
                lf_checker_rt::callee_thiscall!(NOTIFY, u32, vehicle, a1, extra, target, 0);
            return (ans2 & 0xffff_ff00) | 1;
        }
        if event != RELEASE_EVENT {
            return event & 0xffff_ff00;
        }
        if a3 == 0 {
            return 0;
        }
        // Type query through the object's vtable slot 1 (thiscall on the
        // object, no stack words); the stub is planted in a fabricated
        // vtable by the contract.
        let ty: u32 = lf_checker_rt::callee_thiscall!(TYPE_QUERY, u32, a3);
        if ty != WANT_TYPE {
            return ty & 0xffff_ff00;
        }
        let target = ((this + TARGET_OFF) as *const u32).read_unaligned();
        if target == 0 {
            return 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, target, EVENT_SPEED);
        let ans: u32 = lf_checker_rt::callee_thiscall!(DETACH, u32, target, this);
        ((this + TARGET_OFF) as *mut u32).write_unaligned(0);
        (ans & 0xffff_ff00) | 1
    }
});
