// original: 0x00DC8D70 CTaskComplexEvasiveStep::vf20

use lf_checker_rt::{callee_stdcall, callee_thiscall, export, global};

/// Decide an evasive step's follow-up: keep the current sub-task, replace
/// it, or steer by an angle.
///
/// `this+0x08` holds the current sub-task object `o`. Its virtual slot at
/// `+0x0c` is asked first; anything but `0x190` keeps `o` (returned). Then
/// `s = [o+0x14]` must be non-null, the flag byte at `this+0x45` must be
/// clear (it also selects the request id `0x89`, else `0x8a`, used below),
/// and the word at `s+0x04` steers: bit `0x1000` clear takes the
/// angle-steering path (see below), set continues when the float at
/// `s+0x4c` is above 0.5.
///
/// The main path asks callee 2 (`this`=o, the stack argument, 1, 0); a zero
/// low byte keeps `o`. Callee 3 resolves the request id and `[s+0x0c]` into
/// a value (written through its third argument). Callee 4 (on a global)
/// supplies an object, or null, and callee 5 builds the replacement from
/// thirteen words (the resolved value, zero, 3, 0xe600, 0x447a0000, -1,
/// seven zeros); the float at `s+0x4c` is stored at the replacement's
/// `+0xb4` and the replacement returned. When callee 4 answers null, or
/// callee 5 does, the store goes through null `+0xb4` and faults: both
/// fault paths are part of the proof (fault parity).
///
/// The angle-steering path (flag bit clear) is NOT covered by this proof:
/// its callee takes two 64-bit floats in the vector registers and the
/// checker's stack transport moves only 4 bytes per register, so the call
/// cannot be reproduced. The contract pins the flag bit set, the callee is
/// left undeclared (reaching the path would fail loudly), and the rewrite
/// carries the path's computation with the call unwired for a later lane.
///
/// Original: 0x00DC8D70 (thiscall, one stack word).
export!(thiscall, rw_dc8d70(this: u32, arg1: u32) -> u32 {
    const OBJ_OFF: u32 = 0x08;
    const FLAG_OFF: u32 = 0x45;
    const SUB_OFF: u32 = 0x14;
    const STEER_BIT: u32 = 0x1000;
    const RATE_OFF: u32 = 0x4c;
    const KEY_OFF: u32 = 0x0c;
    const WANT: u32 = 0x190;
    const REQ_BASE: u32 = 0x89;
    const HALF: f32 = 0.5;
    const STORE_OFF: u32 = 0xb4;
    const TASK_GLOBAL: u32 = 0x0167_e2a0;
    const CAL_ASK: u32 = 2;
    const CAL_RESOLVE: u32 = 3;
    const CAL_SUPPLY: u32 = 4;
    const CAL_BUILD: u32 = 5;

    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }

    unsafe {
        let o = rd32(this.wrapping_add(OBJ_OFF));
        let flag = rd8(this.wrapping_add(FLAG_OFF));
        let req = REQ_BASE.wrapping_add((flag != 0) as u32);
        // Virtual slot +0x0c of o, called with o as `this`, like the original.
        let vtslot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(o).wrapping_add(0x0c)) as usize);
        if vtslot(o) != WANT {
            return o;
        }
        let s = rd32(o.wrapping_add(SUB_OFF));
        if s == 0 {
            return o;
        }
        if flag != 0 {
            return o;
        }
        if rd32(s.wrapping_add(4)) & STEER_BIT == 0 {
            // Angle-steering path: excluded from this proof (see doc
            // comment). The computation mirrors the original; the vector-
            // register call is unwired (callee 6 undeclared: reaching here
            // faults loudly instead of passing quietly).
            let ax = rd32(this.wrapping_add(0x40));
            if ax == 0 {
                return o;
            }
            let _cx = rd32(ax.wrapping_add(0x20));
            let _r6: f64 = callee_stdcall!(6, f64,);
            return o;
        }
        if !(rdf(s.wrapping_add(RATE_OFF)) > HALF) {
            return o;
        }
        let r2: u32 = callee_thiscall!(CAL_ASK, u32, o, arg1, 1, 0);
        if (r2 & 0xff) == 0 {
            return o;
        }
        // The original zeroes its [E-4] scratch and its [E+4] argument slot
        // before the call; the stub writes the resolved value through the
        // first pointer.
        let mut slot_e4 = 0u32;
        let mut slot_arg = 0u32;
        let _: u32 = callee_stdcall!(CAL_RESOLVE, u32, req, rd32(s.wrapping_add(KEY_OFF)), &mut slot_e4 as *mut u32 as u32, &mut slot_arg as *mut u32 as u32);
        let supply: u32 = callee_thiscall!(CAL_SUPPLY, u32, global::<u32>(TASK_GLOBAL).read());
        let rate = rd32(s.wrapping_add(RATE_OFF));
        if supply == 0 {
            // Fault path A: the store goes through null + 0xb4. The address
            // is laundered so the compiler emits a real store (an access
            // violation like the original's) rather than a trap.
            let dst = core::hint::black_box(STORE_OFF);
            unsafe { (dst as *mut u32).write(rate) };
            return 0;
        }
        let built: u32 = callee_thiscall!(CAL_BUILD, u32, supply, slot_e4, slot_arg, 3, 0xe600, 0x447a0000, 0xffff_ffff, 0, 0, 0, 0, 0, 0, 0);
        // Fault path B when the build answers null.
        unsafe { ((built.wrapping_add(STORE_OFF)) as *mut u32).write(rate) };
        built
    }
});
