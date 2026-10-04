// original: 0x00c17760 cam_replay_update (STAGE 1 SUBSET)

/// Advance the camera-replay blender one step.
///
/// STAGE-1 SUBSET: covers the entry pair-fetch, the type-9 blend path (path A)
/// and the fail paths. The record-kind paths (path B, callee ids 6-11) are
/// constrained away by stage-1 pins (every entry is pinned to kind 9 / mode 3,
/// so a non-null pair always takes path A); the checker guards the premise
/// with its undeclared-callee check. Stage 2 adds path B.
export!(thiscall, rw_00c17760(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        // Replay-table head; a null head selects the fail path.
        let head = global::<u32>(0x0159_3b6c).read();
        let mut cur = 0u32;
        let mut nxt = 0u32;
        if head != 0 {
            let idx = ((head + 0xf8) as *const u32).read();
            if (idx as i32) >= 0 {
                let tab = ((head + 0x9c) as *const u32).read();
                let count = ((tab + 4) as *const u16).read() as u32;
                if idx < count {
                    let arr = (tab as *const u32).read();
                    cur = ((arr + idx.wrapping_mul(4)) as *const u32).read();
                }
            }
            let idx1 = idx.wrapping_add(1);
            if (idx1 as i32) >= 0 {
                let tab = ((head + 0x9c) as *const u32).read();
                let count = ((tab + 4) as *const u16).read() as u32;
                if idx1 < count {
                    let arr = (tab as *const u32).read();
                    nxt = ((arr + idx.wrapping_mul(4) + 4) as *const u32).read();
                }
            }
        }
        // Path-A gate: both entries present, current is kind 9 / mode 3, next is kind 9.
        let on_path_a = cur != 0
            && (cur as *const u8).read() == 9
            && ((cur + 5) as *const u8).read() == 3
            && nxt != 0
            && (nxt as *const u8).read() == 9;
        if !on_path_a {
            // STAGE 1: off path A only the fail path is reachable (path-B kinds
            // are pinned away); stage 2 implements the record paths.
            callee_thiscall!(5, u32, this);
            return 0;
        }
        // Blend position of the current stamp between the two entries.
        let e0 = ((cur + 0x14) as *const u32).read();
        let e1 = ((nxt + 0x14) as *const u32).read();
        let span = e1.wrapping_sub(e0) as f32;
        let now = callee_cdecl!(1, u32,);
        let pos = now.wrapping_sub(e0) as f32;
        let clamp = global::<f32>(0x00fe_88e8).read();
        let mut ratio = pos / span;
        if ratio < 0.0 {
            ratio = 0.0;
        } else if ratio > clamp {
            ratio = clamp;
        }
        let t = callee_thiscall!(2, f32, this, cur, nxt, ratio.to_bits());
        // Lerp the position pair.
        let ax1 = ((nxt + 0x30) as *const f32).read();
        let ax0 = ((cur + 0x30) as *const f32).read();
        let dx = ax1 - ax0;
        let px = dx * t;
        ((this + 0x2c8) as *mut f32).write(px + ax0);
        let ay1 = ((nxt + 0x28) as *const f32).read();
        let ay0 = ((cur + 0x28) as *const f32).read();
        let dy = ay1 - ay0;
        let py = dy * t;
        ((this + 0x2c0) as *mut f32).write(py + ay0);
        // Lerp the level byte as float, then store its truncated byte value back as float.
        let b1 = ((nxt + 2) as *const u8).read() as f32;
        let b0 = ((cur + 2) as *const u8).read() as f32;
        let db = b1 - b0;
        let pb = db * t;
        let lb = pb + b0;
        ((this + 0x2b0) as *mut f32).write(lb);
        let trunc =
            core::arch::x86::_mm_cvtt_ss2si(core::arch::x86::_mm_set_ss(lb)) as u8;
        ((this + 0x2bc) as *mut f32).write(trunc as f32);
        // Filtered scalar through the helper.
        let s0 = ((cur + 0x2c) as *const u32).read();
        let s1 = ((nxt + 0x2c) as *const u32).read();
        let f = callee_cdecl!(3, f32, s0, s1, t.to_bits());
        ((this + 0x2c4) as *mut f32).write(f);
        // Fan the frame out to the dependents.
        callee_thiscall!(4, u32, this, a0, a1, a2, a3, a4, t.to_bits(), nxt);
        // Lerp the last scalar and flag the frame blended.
        let z1 = ((nxt + 0x48) as *const f32).read();
        let z0 = ((cur + 0x48) as *const f32).read();
        let dz = z1 - z0;
        let pz = dz * t;
        ((this + 0x2b4) as *mut f32).write(pz + z0);
        let fb = ((this + 0x39b) as *const u8).read();
        ((this + 0x39b) as *mut u8).write(fb | 0x10);
        callee_thiscall!(5, u32, this);
    }
    1
});
