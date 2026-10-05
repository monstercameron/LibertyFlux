// original: 0x00884ee0 stream_driver_init (proposed)
/// Initialise a streaming driver object through its staged bring-up.
///
/// Stores the device id (`arg1`) at `this+0x10`, publishes `arg2` to the
/// driver slot (global at file address `0x115a3e8`), and installs the three
/// handler addresses (`0x884d80`, `0x4016a0`, `0x884d20`) at `this+0x00`,
/// `+0x04` and `+0x08` with a null word at `+0x0c`. Then runs the bring-up
/// chain, failing with 0 at the first bad step and succeeding with 1 only
/// when every step passes: open the session (intercepted callee 1, thiscall,
/// scratch in `ecx`), configure stream `255` on the sub-object at `this+0x14`
/// (intercepted callee 2, thiscall: scratch in `ecx`, `0`, sub-object,
/// `255` as the stack arguments), verify it (intercepted callee 3, cdecl,
/// sub-object; needs a non-zero low byte), check the registry word at file
/// address `0x115a0c0` (intercepted callee 4, cdecl; needs zero), probe the
/// driver tables (intercepted callee 5, thiscall: object in `ecx`, registry
/// word as the stack argument; needs zero), start the engine (intercepted
/// callee 6, thiscall: object in `ecx`; needs zero) and arm it (intercepted
/// callee 7, cdecl; needs zero). Either way the session is closed
/// (intercepted callee 8, thiscall, scratch in `ecx`); the scratch area is
/// two zero words, matching the checker's zero stack fill.
///
/// Return value: the original ends with `(an instruction of the original)` after the intercepted
/// close call, so the upper bytes are the close stub's scripted answer: the
/// rewrite returns that answer with its low byte replaced by the flag.
///
/// Original: thiscall, two stack arguments, callee cleans 8.
lf_checker_rt::export!(thiscall, rw_00884ee0(this: u32, device: u32, slot: u32) -> u32 {
    unsafe {
        const DRIVER_SLOT: u32 = 0x0115_a3e8;
        const REGISTRY: u32 = 0x0115_a0c0;
        const SUB_OBJECT: u32 = 0x14;
        const STREAM_ID: u32 = 0xff;
        const OPEN_CALLEE: u32 = 1;
        const CONFIG_CALLEE: u32 = 2;
        const VERIFY_CALLEE: u32 = 3;
        const REGCHECK_CALLEE: u32 = 4;
        const PROBE_CALLEE: u32 = 5;
        const START_CALLEE: u32 = 6;
        const ARM_CALLEE: u32 = 7;
        const CLOSE_CALLEE: u32 = 8;
        let mut scratch = [0u32; 2];
        ((this + 0x10) as *mut u32).write_unaligned(device);
        (lf_checker_rt::global::<u32>(DRIVER_SLOT) as *mut u32).write_unaligned(slot);
        ((this + 0x00) as *mut u32).write_unaligned(lf_checker_rt::relocated(0x0088_4d80));
        ((this + 0x04) as *mut u32).write_unaligned(lf_checker_rt::relocated(0x0040_16a0));
        ((this + 0x08) as *mut u32).write_unaligned(lf_checker_rt::relocated(0x0088_4d20));
        ((this + 0x0c) as *mut u32).write_unaligned(0);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(OPEN_CALLEE, u32, scratch.as_mut_ptr() as u32);
        let sub = this.wrapping_add(SUB_OBJECT);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CONFIG_CALLEE,
            u32,
            scratch.as_mut_ptr() as u32,
            0,
            sub,
            STREAM_ID
        );
        let verified: u32 = lf_checker_rt::callee_cdecl!(VERIFY_CALLEE, u32, sub);
        let mut ok = verified as u8 != 0;
        if ok {
            let reg = lf_checker_rt::relocated(REGISTRY);
            let checked: u32 = lf_checker_rt::callee_cdecl!(REGCHECK_CALLEE, u32, reg);
            ok = checked == 0;
        }
        if ok {
            let reg = lf_checker_rt::relocated(REGISTRY);
            let probed: u32 = lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, this, reg);
            ok = probed == 0;
        }
        if ok {
            let started: u32 = lf_checker_rt::callee_thiscall!(START_CALLEE, u32, this);
            ok = started == 0;
        }
        if ok {
            let armed: u32 = lf_checker_rt::callee_cdecl!(ARM_CALLEE, u32,);
            ok = armed == 0;
        }
        let close_answer: u32 =
            lf_checker_rt::callee_thiscall!(CLOSE_CALLEE, u32, scratch.as_mut_ptr() as u32);
        // The original's trailing `(an instruction of the original)` keeps the stub's upper bytes.
        (close_answer & 0xFFFF_FF00) | (ok as u32)
    }
});
