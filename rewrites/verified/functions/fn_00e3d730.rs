// original: 0x00E3D730 stats_probe_update
// ---------------------------------------------------------------------------
// 0x00E3D730: probe a service with a fixed code; on success mark the object
// live, forward a looked-up parameter block to two sinks, and fold a float
// stat into an integer slot; then run a two-code state machine over the
// object's status word. Returns nonzero (in AL) when the third probe code
// succeeded.
// ---------------------------------------------------------------------------

/// Seven-argument probe call; only the low byte of the answer matters.
#[inline(always)]
fn probe730(site_id: u32, code: u32) -> bool {
    unsafe { (callee_cdecl!(site_id, u32, code, 0, 0, 0, 0, 0, 0) & 0xFF) != 0 }
}

/// Shared success path: look up the parameter block for key 0x3b, forward
/// its first word to sink 7, then notify sink 8.
#[inline(always)]
fn forward_params730() {
    unsafe {
        let mut slot: u32 = 0;
        let block = callee_cdecl!(6u32, u32, &mut slot as *mut u32 as u32, 0x3b);
        let first = (block as *const u32).read();
        let _ = callee_cdecl!(7u32, u32, first);
        let _ = callee_cdecl!(8u32, u32, 0, 1);
    }
}

/// Fold the float stat selected by key 0x33 into the integer slot.
#[inline(always)]
fn fold_float_stat730() {
    unsafe {
        let mut slot: u32 = 0;
        let found = callee_cdecl!(10u32, u32, &mut slot as *mut u32 as u32, 0x33);
        let value = cvttss2si((found as *const f32).read());
        let _ = callee_thiscall!(11u32, u32, relocated(0x01161578), value as u32, 0, 0xff);
    }
}

/// Rewrite of the stats probe/update routine.
///
/// The object layout used here is byte 0 = live flag (set to 1 on any
/// successful probe path) and the halfword at +0x14 = status word. Probe
/// answers are tested in AL only.
///
/// Callee ids in the contract: 1-5 = the 7-argument probe (one id per call
/// site so every probe outcome varies independently), 6 = parameter lookup,
/// 7/8 = sinks, 9 = backend-state query, 10 = float-stat lookup, 11 = integer
/// slot store (thiscall/3), 12/13/14 = sibling object updates (thiscall/0).
export!(thiscall, rw_00e3d730(obj: u32) -> u32 {
    unsafe {
        let mut matched_third = false;
        if probe730(1, 0) {
            (obj as *mut u8).write(1);
            forward_params730();
            let backend = callee_cdecl!(9u32, u32, (global::<u32>(0x01160C0C) as *const u32).read());
            if backend == 0 || backend == 8 {
                fold_float_stat730();
            }
        } else if probe730(2, 1) {
            (obj as *mut u8).write(1);
            forward_params730();
            let backend = callee_cdecl!(9u32, u32, (global::<u32>(0x01160C0C) as *const u32).read());
            if backend == 7 || backend == 8 {
                fold_float_stat730();
            }
        } else if probe730(3, 0x0b) {
            (obj as *mut u8).write(1);
            forward_params730();
            matched_third = true;
        }
        ((obj + 0x14) as *mut u8).write(0);
        if probe730(4, 0x0d) {
            ((obj + 0x14) as *mut u16).write(1);
            let _ = callee_thiscall!(12u32, u32, obj);
        } else if probe730(5, 0x0e) {
            ((obj + 0x14) as *mut u16).write(0x0101);
            let _ = callee_thiscall!(12u32, u32, obj);
        } else if (callee_thiscall!(13u32, u32, obj) & 0xFF) != 0 {
            ((obj + 0x14) as *mut u8).write(1);
        }
        if (obj as *const u8).read() != 0 {
            let _ = callee_thiscall!(14u32, u32, obj);
        }
        u32::from(matched_third)
    }
});
