// original: 0x00E6E3D0 mainloop_timing_tls_cleanup_group_b

/// Returns zero when the neighboring 16-bit gate or cleanup pointer is zero.
/// Otherwise, calls the third method on the current thread's cleanup object and
/// returns the method's 32-bit result.
pub extern "cdecl" fn rw_00e6e3d0() -> u32 {
    const CLEANUP_GATE: u32 = 0x018d_de0e;
    const CLEANUP_ARGUMENT: u32 = 0x018d_de08;

    if unsafe { lf_checker_rt::global::<u16>(CLEANUP_GATE).read() } == 0 {
        return 0;
    }
    let argument = unsafe { lf_checker_rt::global::<u32>(CLEANUP_ARGUMENT).read() };
    if argument == 0 {
        return 0;
    }

    let tls_root = lf_checker_rt::tls_slot(0) as *const u32;
    let cleanup_object = unsafe { tls_root.add(2).read_unaligned() };
    let vtable = unsafe { (cleanup_object as *const u32).read_unaligned() };
    let method = unsafe { ((vtable as *const u32).add(3)).read_unaligned() };
    let call: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
        core::mem::transmute(method as usize)
    };
    call(cleanup_object, argument)
}
