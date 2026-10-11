// original: 0x00aff120 pool_master_pass
/// Run the pool maintenance pass after its global and service gates allow it.
/// The pass asks the manager to fill a local context, selects the fallback
/// scale when the table index is negative, runs the update and pool checks,
/// and clears the status word on the selected completion path.
use lf_checker_rt::{callee_cdecl, export, global};

const GATE_STATE: u32 = 0x011F_7060;
const SERVICE_STATE: u32 = 0x0120_88B4;
const REQUIRED_SERVICE: u32 = 0x00F1_C040;
const PASS_MODE: u32 = 0x0103_7720;
const TABLE_INDEX: u32 = 0x0160_031C;
const CONTEXT_SCALE: u32 = 0x0103_FF50;
const MODE_ONE_SCALE: u32 = 0x0103_FF54;
const MODE_TWO_SCALE: u32 = 0x0103_FF58;
const DEFAULT_SCALE: u32 = 0x00FE_88E8;
const FLAG_BYTES: u32 = 0x0103_FF76;
const STATUS_WORD: u32 = 0x012E_21F0;

#[inline(always)]
unsafe fn load_u32(address: u32) -> u32 {
    (address as *const u32).read_unaligned()
}

export!(cdecl, rw_afff120() -> u32 {
    unsafe { body_afff120(false) }
});

// Mutant: flip the final service answer after preserving the status update.
export!(cdecl, mut_afff120() -> u32 {
    unsafe { body_afff120(true) }
});

// Mutant for the initial gate contract: clear the status word on an early
// return where the original leaves it untouched.
export!(cdecl, mut_afff120_gate() -> u32 {
    unsafe {
        if global::<u32>(GATE_STATE).read() == 1 {
            global::<u32>(STATUS_WORD).write(0);
            return 0;
        }
        body_afff120(false)
    }
});

unsafe fn body_afff120(mutant: bool) -> u32 {
    if global::<u32>(GATE_STATE).read() == 1 {
        return 0;
    }
    if global::<u32>(SERVICE_STATE).read() != global::<u32>(REQUIRED_SERVICE).read() {
        return 0;
    }
    if global::<u32>(PASS_MODE).read() == 0x12 {
        return 0;
    }

    if callee_cdecl!(1, u32,) as u8 != 0 {
        return 0;
    }
    if callee_cdecl!(2, u32,) as u8 != 0 {
        return 0;
    }
    let _ = callee_cdecl!(3, u32,);
    let _ = callee_cdecl!(4, u32,);

    // These offsets mirror the local values handed to the manager. The
    // contract snapshots the five word outputs; the single-byte marker is
    // read here on the exercised zero-marker path.
    let mut context = [0u8; 0x40];
    let context_base = context.as_mut_ptr();
    let out_marker = context_base.add(3) as u32;
    let out_upper = context_base.add(0x30) as u32;
    let out_mode = context_base.add(0x20) as u32;
    let out_value = context_base.add(0x08) as u32;
    let out_low = context_base.add(0x0C) as u32;
    let out_radius = context_base.add(0x10) as u32;
    let _ = callee_cdecl!(5, u32, out_radius, out_low, out_value, out_mode, out_upper, out_marker);

    if global::<i32>(TABLE_INDEX).read() >= 0 {
        return 0;
    }

    let mode = load_u32(context_base.add(0x08) as u32);
    let default = global::<f32>(DEFAULT_SCALE).read();
    let mode_scale = match mode {
        1 => global::<f32>(MODE_ONE_SCALE).read(),
        2 => global::<f32>(MODE_TWO_SCALE).read(),
        _ => default,
    };
    let context_scale = global::<f32>(CONTEXT_SCALE).read();
    let scale = core::hint::black_box(context_scale) * core::hint::black_box(mode_scale);

    if context[3] != 0 {
        return 0;
    }
    let context_out = context_base.add(0x10) as u32;
    let _ = callee_cdecl!(6, u32, context_out, scale.to_bits(), 0u32);
    let _ = callee_cdecl!(7, u32,);
    let _ = callee_cdecl!(8, u32,);
    let _ = callee_cdecl!(9, u32,);

    let first_optional_flag = global::<u8>(FLAG_BYTES).read();
    let second_optional_flag = global::<u8>(FLAG_BYTES + 1).read();
    if first_optional_flag != 0 || second_optional_flag != 0 {
        return 0;
    }

    let final_gate = callee_cdecl!(1, u32,);
    if final_gate as u8 != 0 {
        global::<u32>(STATUS_WORD).write(0);
        return if mutant { final_gate ^ 1 } else { final_gate };
    }

    0
}
