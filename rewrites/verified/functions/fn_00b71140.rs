// original: 0x00b71140 CTaskSimpleCarSlowDragPedOut::vf5

/// Task event handler: on event 2, or whenever the ped is flagged, drag out.
///
/// Reads the event id (second stack word) and the ped (first stack word).
/// Any event but 2 with the ped's flag byte at +0x211 clear is ignored
/// (returns 0). Otherwise, with no drag target at `this+0x18` reports done.
/// Else calls the release callee (thiscall on the target; -1000.0f for
/// event 2, -16.0f otherwise), flags the target with 0x4000, and unless the
/// vehicle at `this+0x24` is null also calls the notify callee (thiscall on
/// the vehicle; pushed 0, target, `this+0x28`, ped). Reports done; the low
/// return byte is the decision and the upper bytes are the target's on the
/// release-only path or the notify answer's on the full path (the contract
/// pins entry eax for the call-free paths).
///
/// Original: 0x00b71140 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00b71140(this: u32, ped: u32, event: u32, _a3: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x18;
        const VEHICLE_OFF: u32 = 0x24;
        const EXTRA_OFF: u32 = 0x28;
        const PED_FLAG_OFF: u32 = 0x211;
        const FLAG_WORD_OFF: u32 = 4;
        const RELEASED_FLAG: u32 = 0x4000;
        const DRAG_EVENT: u32 = 2;
        const EVENT_SPEED: u32 = 0xc47a0000; // -1000.0f
        const IDLE_SPEED: u32 = 0xc1000000; // -16.0f
        const RELEASE: u32 = 1;
        const NOTIFY: u32 = 2;
        if event != DRAG_EVENT && ((ped + PED_FLAG_OFF) as *const u8).read() == 0 {
            return 0;
        }
        let target = ((this + TARGET_OFF) as *const u32).read_unaligned();
        if target == 0 {
            return 1;
        }
        let speed = if event == DRAG_EVENT { EVENT_SPEED } else { IDLE_SPEED };
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, target, speed);
        let slot = (target + FLAG_WORD_OFF) as *mut u32;
        slot.write_unaligned(slot.read_unaligned() | RELEASED_FLAG);
        let vehicle = ((this + VEHICLE_OFF) as *const u32).read_unaligned();
        if vehicle == 0 {
            // The original reloads the target into the return register
            // here, so the upper return bytes are the target's (the release
            // answer is ignored).
            return (target & 0xffff_ff00) | 1;
        }
        let extra = ((this + EXTRA_OFF) as *const u32).read_unaligned();
        // Push order is 0, target, extra, ped, so C order is reversed.
        let ans2: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, vehicle, ped, extra, target, 0);
        (ans2 & 0xffff_ff00) | 1
    }
});
