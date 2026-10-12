

/// Deliberately stores the neighboring relocated pointer before the same tail call.
#[unsafe(no_mangle)]
pub extern "cdecl" fn rw_00e6e5a0() -> u32 {
    unsafe {
        lf_checker_rt::global::<u32>(TAIL_INITIALIZER_GLOBAL_VA)
            .write(lf_checker_rt::relocated(TAIL_INITIALIZER_VALUE_VA).wrapping_add(4));
    }
    let initialize: extern "thiscall" fn(u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
    };
    initialize(lf_checker_rt::relocated(TAIL_INITIALIZER_GLOBAL_VA))
}

/// Stores the relocated initializer pointer, then tail-calls the object's
/// thiscall initializer with the global's address in ECX.
lf_checker_rt::export!(cdecl, lane_base_00e6e5a0() -> u32 {
    unsafe {
        lf_checker_rt::global::<u32>(TAIL_INITIALIZER_GLOBAL_VA)
            .write(lf_checker_rt::relocated(TAIL_INITIALIZER_VALUE_VA));
    }
    let initialize: extern "thiscall" fn(u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
    };
    initialize(lf_checker_rt::relocated(TAIL_INITIALIZER_GLOBAL_VA))
});

const TAIL_INITIALIZER_GLOBAL_VA: u32 = 0x0198_1fa8;

const TAIL_INITIALIZER_VALUE_VA: u32 = 0x00fe_4024;
