// original: 0x00d532f0 CCamAnimated::pose_body (proposed)
/// Camera-body update for a blend step: resolves the animation mode, poses
/// the orientation basis from the eased angle, and refreshes the dependent
/// limits and offsets.
///
/// Original: thiscall/1 (blend factor on the stack), returns the latched
/// mode flag.
export!(thiscall, rw_00d532f0(this: u32, arg: u32) -> u32 {
    unsafe {
        // The mode resolver takes the blend by pointer and may update it;
        // every use below sees the updated value.
        let mut blend_bits = arg;
        let target: u32 = callee_thiscall!(1, u32, this);
        let mode: u32 =
            callee_thiscall!(2, u32, this, &mut blend_bits as *mut u32 as u32);
        let blend = f32::from_bits(blend_bits);
        *((this + 0x144) as *mut f32) = blend;
        // Latch the per-frame flags: an exact 1.0 blend keeps the first
        // flag clear in mode 0, and mode 3 (or a zeroed global timer while
        // the generation matches) sets the second.
        let flag_a: u32 = if mode == 0 {
            (blend == *global::<f32>(0xFE88E8)) as u32
        } else {
            1
        };
        let mut flag_b: u32 = (mode == 3) as u32;
        if *global::<u32>(0x1294730) == *global::<u32>(0x1295770)
            && *global::<f32>(0x1295740) == 0.0
        {
            flag_b = 1;
        }
        // Scratch block the solvers fill in (frame-relative on the original).
        let mut frame = [0u32; 4];
        let frame_ptr = frame.as_mut_ptr() as u32;
        // Pose solver: adds the solved offset onto the pivot, takes the
        // solved height from the scratch block.
        let solved: u32 = callee_thiscall!(3, u32, this, frame_ptr, blend_bits, flag_a);
        let px = *((solved + 0) as *const f32);
        let py = *((solved + 4) as *const f32);
        let pz = *((solved + 8) as *const f32);
        let vx = *((this + 0x160) as *const f32) + px;
        let vy = *((this + 0x164) as *const f32) + py;
        let vz = *((this + 0x168) as *const f32) + pz;
        *((this + 0x48) as *mut f32) = vz;
        *((this + 0x4c) as *mut f32) = f32::from_bits(frame[3]);
        *((this + 0x40) as *mut f32) = vx;
        *((this + 0x44) as *mut f32) = vy;
        let aux: u32 = callee_thiscall!(4, u32, this, frame_ptr, blend_bits, flag_a);
        let _: u32 = callee_thiscall!(5, u32, this + 0x10, aux);
        // Orientation easing while the first driver is present.
        if callee_thiscall!(6, u32, target, 0x8a, 0) != 0 {
            let t: f32 = callee_thiscall!(7, f32, this, blend_bits, flag_a);
            // Sine/cosine pair, taking the angle in XMM0 on the original;
            // the rewrite passes the bits on the stack and the stub
            // transports them (checker XMM0-arg convention).
            let c = f32::from_bits(callee_cdecl!(8, u32, t.to_bits()));
            let s = f32::from_bits(callee_cdecl!(9, u32, t.to_bits()));
            let r10 = *((this + 0x10) as *const f32);
            let r14 = *((this + 0x14) as *const f32);
            let r18 = *((this + 0x18) as *const f32);
            let r30 = *((this + 0x30) as *const f32);
            let r34 = *((this + 0x34) as *const f32);
            let r38 = *((this + 0x38) as *const f32);
            // New basis rows, in the original's exact multiply order.
            let n10 = r10 * c - r30 * s;
            let n14 = r14 * c - r34 * s;
            let n18 = r18 * c - s * r38;
            let w30 = r30 * c;
            let w34 = r34 * c;
            let w38 = c * r38;
            *((this + 0x30) as *mut f32) = w30;
            *((this + 0x34) as *mut f32) = w34;
            *((this + 0x38) as *mut f32) = w38;
            *((this + 0x30) as *mut f32) = r10 * s + w30;
            *((this + 0x34) as *mut f32) = r14 * s + w34;
            *((this + 0x38) as *mut f32) = r18 * s + w38;
            *((this + 0x10) as *mut f32) = n10;
            *((this + 0x14) as *mut f32) = n14;
            *((this + 0x18) as *mut f32) = n18;
        }
        // Refresh the dependent limits unless the frame is settled.
        let settled = *global::<i32>(0x11D6FD4) <= 0;
        if flag_a == 0 || mode == 3 || (!settled && mode == 2) {
            if callee_thiscall!(10, u32, target, 0x89, 0) != 0 {
                let _: u32 = callee_thiscall!(11, u32, this, frame_ptr, blend_bits, 1);
                let f0 = f32::from_bits(frame[0]);
                let f1 = f32::from_bits(frame[1]);
                *((this + 0x6c) as *mut f32) = f0;
                if f1 > 0.0 || *global::<f32>(0xFE8E04) > f1 {
                    *((this + 0x70) as *mut f32) = f1;
                }
            }
            if callee_thiscall!(12, u32, target, 0x88, 0) != 0 {
                let t2: f32 = callee_thiscall!(13, f32, this, blend_bits, 1);
                if t2 > 0.0 {
                    *((this + 0x60) as *mut f32) = t2;
                }
            }
        }
        flag_b
    }
});
