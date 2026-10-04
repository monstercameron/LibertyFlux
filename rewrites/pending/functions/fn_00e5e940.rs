// original: 0x00e5e940 rage::snEvent::AutoIdDesc__::AutoIdDesc__
/// Initialise one static network-event descriptor and register its callback.
///
/// Calls the object routine (stubbed, callee-cleaned, 3 words of scratch)
/// that initialises the static descriptor, records the constant `0xFE21F0` in the
/// descriptor header at `0x019f0800`, then passes the callback thunk `0x00e6f0e0` to the
/// registrar helper (stubbed, cdecl/1). Takes no arguments; entry registers
/// are ignored. Returns the registrar's answer. The scratch words the original
/// passes uninitialised read as the trial fill on both sides.
export!(cdecl, rw_00e5e940() -> u32 {
    unsafe {
        let _: u32 = callee_stdcall!(1, u32, 0, 0, 0);
        *global::<u32>(0x19F0800) = relocated(0xFE21F0);
        callee_cdecl!(2, u32, relocated(0xE6F0E0))
    }
});
