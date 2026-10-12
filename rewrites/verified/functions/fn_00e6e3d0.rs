// original: 0x00E6E3D0 mainloop_timing_tls_cleanup_group_b

/// Returns the neighboring TLS cleanup method's value when its gate and
/// argument pointer are nonzero; otherwise returns zero.
lf_checker_rt::export!(cdecl, rw_00e6e3d0() -> u32 {
    let gate = unsafe { lf_checker_rt::global::<u16>(CLEANUP_B_FLAG_VA).read() };
    if gate == 0 {
        return 0;
    }
    let argument = unsafe { lf_checker_rt::global::<u32>(CLEANUP_B_ARGUMENT_VA).read() };
    if argument == 0 {
        return 0;
    }
    call_tls_cleanup(argument)
});

const CLEANUP_B_ARGUMENT_VA: u32 = 0x018d_de08;

const CLEANUP_B_FLAG_VA: u32 = 0x018d_de0e;

#[inline(always)]
fn call_tls_cleanup(argument: u32) -> u32 {
    let tls_root = lf_checker_rt::tls_slot(TLS_ROOT_SLOT) as *const u32;
    let cleanup_object = unsafe { tls_root.add(TLS_OBJECT_FIELD_WORD).read_unaligned() };
    let vtable = unsafe { (cleanup_object as *const u32).read_unaligned() };
    let method = unsafe { ((vtable as *const u32).add(VTABLE_METHOD_WORD)).read_unaligned() };
    let call: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
        core::mem::transmute(method as usize)
    };
    call(cleanup_object, argument)
}

const TLS_OBJECT_FIELD_WORD: usize = 2;

const TLS_ROOT_SLOT: usize = 0;

const VTABLE_METHOD_WORD: usize = 3;
