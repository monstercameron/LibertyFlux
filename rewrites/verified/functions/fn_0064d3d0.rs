// original: 0x0064D3D0 rage::ptxSprite::vf12

/// Reproduces the pre-loop lookup on the empty-list path. The contract pins
/// the list head to null and checks the lookup call and stack cleanup. Per-node
/// interpolation, particle updates, callbacks, and the residual EAX value are
/// outside this proof.
use lf_checker_rt::{callee_cdecl, relocated};

const LOOKUP_CALLEE: u32 = 1;
const LOOKUP_SYMBOL_A: u32 = 0x0115_0B94;
const LOOKUP_SYMBOL_B: u32 = 0x0115_0B24;

unsafe fn call_empty_list_lookup(parameters: *mut u8, symbol_a: u32) {
    let lookup_context = unsafe { parameters.add(0xE0).cast::<u32>().read_unaligned() };
    let symbol_b = relocated(LOOKUP_SYMBOL_B);
    let _lookup_result = callee_cdecl!(
        LOOKUP_CALLEE,
        u32,
        lookup_context,
        0,
        symbol_a,
        symbol_b,
        0,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "thiscall" fn rw_b156(
    _sprite: *mut u8,
    _update_parameters: *mut u8,
    parameters: *mut u8,
    _mode: u32,
    _delta_time: f32,
) {
    unsafe { call_empty_list_lookup(parameters, relocated(LOOKUP_SYMBOL_A)) };
}