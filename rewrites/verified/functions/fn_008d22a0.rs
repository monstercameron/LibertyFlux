// original: 0x008D22A0 SetPlayerControlState
// Callee ids for fn_008D22A0 (see the contract): each id is one intercepted
// original callee; the worker answers with the contract's scripted values.
const C_STATE_INNER: u32 = 1; // inner state update, thiscall/1
const C_RESET_ALL: u32 = 2; // full reset, cdecl/1
const C_NOTIFY_OFF: u32 = 3; // restrict notify, cdecl/0
const C_NOTIFY_ON: u32 = 4; // restrict notify, cdecl/0
const C_APPLY_A: u32 = 5; // apply, thiscall/0
const C_GATE: u32 = 6; // gated action, thiscall/2
const C_REGISTER: u32 = 7; // register object, thiscall/1
const C_READ_POS: u32 = 8; // read position triple, thiscall/1 (out-pointer)
const C_STORE_POS: u32 = 9; // store position, thiscall/2
const C_APPLY_B: u32 = 10; // apply, cdecl/2
const C_COMMIT: u32 = 11; // commit, cdecl/0
const C_FINISH: u32 = 12; // finish, cdecl/1
const C_APPLY_C: u32 = 13; // apply, thiscall/0
const C_SYNC: u32 = 14; // sync, cdecl/2


// Player control-state setter (original `SetPlayerControlState`).
//
// `this` points at the player info object. Updates the control-bits word
// from the two flag bytes, then runs one of two paths: the fully-enabled
// path clears state bits and notifies, while the restricted path sets them,
// clamps a per-player timer at zero, and fans out to the position and sync
// helpers. Both paths share the tail, which reloads the gate pointer and
// either syncs (returning the sync answer) or returns the gate pointer.
export!(thiscall, rw_008d22a0(this: u32, a: u32, b: u32, f: u32, c: u32, d: u32) -> u32 {
    unsafe {
        const CTL: u32 = 0x4C8;
        const LINK: u32 = 0x598;
        // First flag byte selects which control bit the second flag sets.
        let ctl = (this + CTL) as *mut u32;
        if (a & 0xFF) != 0 {
            if (b & 0xFF) != 0 {
                *ctl |= 0x800;
            } else {
                *ctl &= !0x800u32;
            }
        } else if (b & 0xFF) != 0 {
            *ctl |= 0x20;
        } else {
            *ctl &= !0x20u32;
        }
        let obj = *((this + LINK) as *const u32);
        if (*ctl & 0x820) == 0 {
            // Fully enabled: clear the restriction bits and notify, then
            // fall through to the shared tail like the original's jump.
            *((this + 0xCA) as *mut u8) &= !4u8;
            *((obj + 0x118) as *mut u32) &= !0x400u32;
            callee_thiscall!(C_STATE_INNER, u32, obj, 1u32);
            if (d & 0xFF) != 0 {
                callee_cdecl!(C_RESET_ALL, u32, 0u32);
            }
        } else {
            // Restricted: set the bits, clear the pedals bit, apply.
            *((this + 0xCA) as *mut u8) |= 4u8;
            callee_cdecl!(C_NOTIFY_OFF, u32,);
            callee_cdecl!(C_NOTIFY_ON, u32,);
            *((obj + 0x118) as *mut u32) |= 0x400u32;
            let inner = *((obj + 0x228) as *const u32);
            // A null link faults at a low address on the read below, as the
            // original's read-modify-write does; mapped trials never take it.
            let base = if inner == 0 { 0 } else { inner.wrapping_add(0x70) };
            let slot = base.wrapping_add(0x3D0) as *mut u32;
            *slot &= !0x10u32;
            callee_thiscall!(C_APPLY_A, u32, obj);
            // Clamp a negative timer to zero. NaN compares false, so it is
            // left alone, matching the original's comiss/jbe pair.
            let timer = (this + 0x424) as *mut f32;
            if *timer < 0.0 {
                *timer = 0.0;
            }
            let gate = *((obj + 0x6C) as *const u32);
            if (gate == 0 || *((gate + 0xE) as *const u8) == 0) && (c & 0xFF) != 0 {
                let target = *((obj + 0x224) as *const u32);
                callee_thiscall!(C_GATE, u32, target, 1u32, 0u32);
            }
            let registry: u32 = relocated(0x012E2420);
            callee_thiscall!(C_REGISTER, u32, registry, obj);
            if (d & 0xFF) != 0 {
                // Each position read fills a scratch triple; the helper
                // answers are scripted, so only the call shapes are compared.
                let mut scratch = [0u32; 3];
                let pos = scratch.as_mut_ptr() as u32;
                let r1 = callee_thiscall!(C_READ_POS, u32, this, pos);
                callee_thiscall!(C_STORE_POS, u32, registry, r1, f);
                let r2 = callee_thiscall!(C_READ_POS, u32, this, pos);
                callee_cdecl!(C_APPLY_B, u32, 0xFFFFFFFFu32, r2);
                callee_cdecl!(C_COMMIT, u32,);
                let r3 = callee_thiscall!(C_READ_POS, u32, this, pos);
                callee_cdecl!(C_FINISH, u32, r3);
            }
            callee_thiscall!(C_APPLY_C, u32, obj);
        }
        // Shared tail: the original reloads the gate into eax here, so a
        // skipped sync returns the gate pointer rather than the last answer.
        let gate2 = *((obj + 0x6C) as *const u32);
        if gate2 == 0 || *((gate2 + 0xE) as *const u8) == 0 {
            callee_cdecl!(C_SYNC, u32, 4u32, b)
        } else {
            gate2
        }
    }
});
// Player control-state setter (original `SetPlayerControlState`).
//
// `this` points at the player info object. Updates the control-bits word
// from the two flag bytes, then runs one of two paths: the fully-enabled
// path clears state bits and notifies, while the restricted path sets them,
// clamps a per-player timer at zero, and fans out to the position and sync
// helpers. Both paths share the tail, which reloads the gate pointer and
// either syncs (returning the sync answer) or returns the gate pointer.
export!(thiscall,
