// original: 0x00a123d0 follow_cam_init_tail (proposed)
/// Initialise a follow-camera object, then tail-jump to the base routine.
///
/// Installs the follow-camera virtual table, runs the first initialiser,
/// sets two globals to 1.0 and clears a flag byte, then transfers control
/// to the second initialiser with the same object (a tail jump, expressed
/// here as a returning call with the same observable calls and result).
/// Thiscall, no stack arguments.
export!(thiscall, rw_00a123d0(this: u32) -> u32 {
    unsafe {
        const FIRST_INIT: u32 = 1;
        const TAIL_INIT: u32 = 2;
        const VTABLE: u32 = 0x00e9af3c;
        const ONE_BITS: u32 = 0x3f800000;
        callee_thiscall!(FIRST_INIT, u32, this);
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        *global::<u32>(0x01032350) = ONE_BITS;
        *global::<u32>(0x0103234c) = ONE_BITS;
        *global::<u8>(0x012bd191) = 0;
        callee_thiscall!(TAIL_INIT, u32, this)
    }
});
