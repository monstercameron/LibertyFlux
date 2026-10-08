//! Proof scope: complete target body within a declared non-null 16-byte heap fixture.
//! Allocation and both method calls use recorder stubs; native helper effects are untested.
//! The method recorder returns 12. Negative correction arms and null/fault paths are untested.
//! All nine declared comparisons passed in 1000 completed original trials.
//! The same-contract field mutant failed in three completed originals at the stock failure cap.
//! Acceptance binds the preserved tested source and immutable DLL generation.
//! This unbuilt projection removes only the mutant export and explanatory comments;
//! the tested helper, dormant mutation branch, and positive export remain unchanged.
//! Earlier unrepaired passes and resource-blocked zero-trial attempts are excluded.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

unsafe fn body(input: u32, mutant: bool) -> u32 {
    let obj = callee_cdecl!(1, u32, 12u32, 0u32) as *mut u32;
    let counter = global::<u32>(0x0103_27a0);
    let old = obj.add(1).read();
    let count = counter.read();
    obj.write(relocated(0x00e7_e048));
    let delta = (old ^ count) & 0x3fff;
    obj.add(1).write(old ^ delta);
    counter.write(count.wrapping_add(1));
    obj.write(relocated(0x00ea_bc50));
    obj.add(2).write(if mutant { input ^ 1 } else { input });

    let first = callee_thiscall!(2, u32, obj as u32);
    let mut eax = first & 0x8000_000f;
    if (eax as i32) < 0 {
        eax = eax.wrapping_sub(1);
        eax |= 0xffff_fff0;
        eax = eax.wrapping_add(1);
    }
    let mut esi = 0x10u32.wrapping_sub(eax);
    if (esi as i32) < 0 {
        esi = esi.wrapping_sub(1);
        esi |= 0xffff_fff0;
        esi = esi.wrapping_add(1);
    }
    let second = callee_thiscall!(2, u32, obj as u32);
    eax = second.wrapping_add(esi);
    let edx = if (eax as i32) < 0 { u32::MAX } else { 0 };
    let edx = edx & 0x0f;
    eax = eax.wrapping_add(edx);
    eax = ((eax as i32) >> 4) as u32;
    eax = eax.wrapping_shl(14);
    eax ^= obj.add(1).read();
    eax &= 0x01ff_c000;
    obj.add(1).write(obj.add(1).read() ^ eax);
    eax
}

export!(cdecl, rw_q236(input: u32) -> u32 { unsafe { body(input, false) } });
