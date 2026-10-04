// original: 0x00CB3BE0 CTaskComplexMoveGetToPointContinuous::vf18

/// Report arrival once the go-to subtask finishes.
///
/// `this` is the complex task, `ped` the ped (unused). When the current
/// subtask's type (virtual slot 3, callee 1) is one of the finished
/// go-to ids (0x119, 0x384, 0x3ae), a worker is fetched (callee 2) and
/// initialised (callee 3, flag 1) as the arrived marker (vtable pair and
/// a zeroed slot) and returned; a missing worker yields null. Any other
/// type keeps the current subtask (null).
///
/// Original: 0x00CB3BE0 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB3BE0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 8;
        const GET_TYPE_SLOT: u32 = 0x0c;
        const GET_WORKER: u32 = 2;
        const INIT_MARKER: u32 = 3;
        const ALLOCATOR_GLOBAL: u32 = 0x167e2a0;
        const ARRIVED_A: u32 = 0x119;
        const ARRIVED_B: u32 = 0x384;
        const ARRIVED_C: u32 = 0x3ae;
        const MARKER_VT0: u32 = 0xe98794;
        const MARKER_VT1: u32 = 0xe987e8;
        // File VAs: the worker relocates the image, so derive the address.
        let vt0 = lf_checker_rt::relocated(MARKER_VT0);
        let vt1 = lf_checker_rt::relocated(MARKER_VT1);
        let _ = ped;
        let sub = (this.wrapping_add(SUBTASK) as *const u32).read_unaligned();
        let vtable = (sub as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(GET_TYPE_SLOT) as *const u32).read_unaligned();
        let get_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        match get_type(sub) {
            ARRIVED_A | ARRIVED_B | ARRIVED_C => {
                let alloc =
                    (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
                let marker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
                if marker == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(INIT_MARKER, u32, marker, 1);
                (marker as *mut u32).write_unaligned(vt0);
                (marker.wrapping_add(0x14) as *mut u32).write_unaligned(vt1);
                (marker.wrapping_add(0x20) as *mut u32).write_unaligned(0);
                marker
            }
            _ => 0,
        }
    }
});
