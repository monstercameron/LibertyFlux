// original: 0x00a4f690 vehicle_dispatch_kind3 (proposed)

/// Position the slot, then run the operator for the object's kind.
///
/// The slot-position callee runs first with a scratch word; then bits 6-9
/// of the word at +0x28 select the operator: kinds 3 and 4 call their
/// operator with (object, scratch, rate, slot word), kind 2 with an extra
/// zero word, any other kind calls nothing. Every path returns 1 (eax is
/// fully determined: the kind is under 16 or the stub answered zero before
/// al is set). The scratch word is callee output, so its address is skipped
/// and only the calls are compared (see contract). Cdecl, two stack words,
/// four callees, 1 in eax.
lf_checker_rt::export!(cdecl, rw_00a4f690(obj: u32, slot: u32) -> u32 {
    unsafe {
        const POS: u32 = 1;
        const OP3: u32 = 2;
        const OP2: u32 = 3;
        const OP4: u32 = 4;
        const RATE: u32 = 0x3e800000;
        const KIND_OFF: u32 = 0x28;
        const SLOT_OFF: u32 = 0x44;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut scratch = [0u32; 8];
        let frame = (&mut scratch as *mut u32) as u32;
        lf_checker_rt::callee_thiscall!(POS, u32, slot, frame);
        let kind = (rd32(obj.wrapping_add(KIND_OFF)) >> 6) & 0xf;
        if kind == 3 {
            let extra = rd32(slot.wrapping_add(SLOT_OFF));
            lf_checker_rt::callee_cdecl!(OP3, u32, obj, frame, RATE, extra);
        } else if kind == 2 {
            let extra = rd32(slot.wrapping_add(SLOT_OFF));
            lf_checker_rt::callee_cdecl!(OP2, u32, obj, frame, RATE, extra, 0);
        } else if kind == 4 {
            let extra = rd32(slot.wrapping_add(SLOT_OFF));
            lf_checker_rt::callee_cdecl!(OP4, u32, obj, frame, RATE, extra);
        }
        1
    }
});
