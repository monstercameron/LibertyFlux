// original: 0x00a504a0 vehicle_dispatch_kind3_b (proposed)

/// Twin of the kind-3 dispatcher with a float-returning position callee.
///
/// Like its twin it positions through a scratch area, reads the kind from
/// bits 6-9 of the word at +0x28 and dispatches kinds 3, 2, 4 (anything
/// else calls nothing), returning 1 on every path. Here the position
/// callee's scratch also yields the float operand the operators take (zero
/// under the stubbed callee and zeroed scratch, read from our own scratch
/// like the original reads its own): kinds 3 and 4 pass (object, scratch,
/// float, 0) and kind 2 passes (object, scratch, float, 0, 1).
/// The scratch addresses are skipped (see contract). Cdecl, two stack
/// words, four callees, 1 in eax.
lf_checker_rt::export!(cdecl, rw_00a504a0(obj: u32, slot: u32) -> u32 {
    unsafe {
        const POS: u32 = 1;
        const OP3: u32 = 2;
        const OP2: u32 = 3;
        const OP4: u32 = 4;
        const KIND_OFF: u32 = 0x28;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut scratch = [0u32; 8];
        let frame = (&mut scratch as *mut u32) as u32;
        lf_checker_rt::callee_thiscall!(POS, u32, slot, frame);
        let f = scratch[0];
        let kind = (rd32(obj.wrapping_add(KIND_OFF)) >> 6) & 0xf;
        if kind == 3 {
            lf_checker_rt::callee_cdecl!(OP3, u32, obj, frame, f, 0);
        } else if kind == 2 {
            lf_checker_rt::callee_cdecl!(OP2, u32, obj, frame, f, 0, 1);
        } else if kind == 4 {
            lf_checker_rt::callee_cdecl!(OP4, u32, obj, frame, f, 0);
        }
        1
    }
});
