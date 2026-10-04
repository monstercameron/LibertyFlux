// original: 0x00e63770 init_obj_4arg
/// Initialise the object at 0x11A2E78 with two data pointers.
///
/// Calls the thiscall/4 init routine (stubbed) with alternating pointer and
/// zero arguments, returning its answer.
export!(cdecl, rw_00e63770() -> u32 {
    unsafe {
        let init: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        init(
            relocated(0x11A2E78),
            0,
            relocated(0xE8772C),
            0,
            relocated(0xE87700),
        )
    }
});
