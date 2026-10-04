// original: 0x00cf1b50 stat_value_dispatch
use lf_checker_rt::{callee_cdecl, export, global};

// Stats mode global: the dispatcher and the formatter both read it.
const MODE_GLOBAL: u32 = 0x1160CC8;

// Mode-dispatched scaled stat lookup.
//
// Reads the stats mode global and tail-calls one of three scaling getters
// with the caller's stat id. Each getter answers a double on the x87 stack.
export!(cdecl, rw_cf1b50(arg0: u32) -> f64 {
    let mode: u32 = unsafe { global::<u32>(MODE_GLOBAL).read() };
    match mode.wrapping_sub(1) {
        // Modes 1..=4: plain scaled getter.
        0 | 1 | 2 | 3 => callee_cdecl!(1, f64, arg0),
        // Mode 6: alternate scaled getter.
        5 => callee_cdecl!(2, f64, arg0),
        // Mode 5 and everything else: default scaled getter.
        _ => callee_cdecl!(3, f64, arg0),
    }
});
