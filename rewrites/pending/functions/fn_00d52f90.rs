// original: 0x00d52f90 CCamAnimated::vf4
/// Animated camera frame update: drives the blend factor through the
/// animation callback, runs the shared camera-body update, then pushes the
/// new pose to every active scene-graph attachment.
///
/// Original: thiscall/0, returns 1 when the camera posed, 0 when it has no
/// animation target or no animation callback.
export!(thiscall, rw_00d52f90(this: u32) -> u32 {
    unsafe {
        // Resolve the animation target; nothing to pose without one.
        let anim: u32 = callee_thiscall!(1, u32, this);
        if anim == 0 {
            return 0;
        }
        // The animation callback pointer lives on the camera object.
        let callback = *((this + 0x148) as *const u32);
        if callback == 0 {
            return 0;
        }
        let run_callback: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(callback as usize);
        let rate = *global::<f32>(0xFE86B4);
        let one = *global::<f32>(0xFE88E8);
        let zero = *global::<f32>(0xFE8628);
        let period = || *((anim + 0x0c) as *const f32);
        // True when the stored blend factor is exactly zero.
        let stored_is_zero = *((this + 0x140) as *const f32) == zero;

        // The blend factor is the callback's answer scaled by the rate and
        // normalised by the animation period, clamped to [0, 1]. The
        // original re-queries the callback at each clamp stage, so the
        // rewrite walks the same four stages in order.
        let mut stage: u8 = 0;
        let blend = loop {
            let t = run_callback(this);
            let x = t * rate;
            let s = period();
            match stage {
                // Stage A: normalise, or retry when the period is not positive.
                0 => {
                    if !(s > zero) {
                        stage = 1;
                    } else if !(one > x / s) {
                        stage = 2;
                    } else {
                        stage = 1;
                    }
                }
                // Stage B: normalise, clamping negative answers to zero.
                1 => {
                    if !(s > 0.0) {
                        stage = 2;
                    } else {
                        let y = x / s;
                        if 0.0 > y {
                            break 0.0;
                        }
                        stage = 2;
                    }
                }
                // Stage C: normalise, clamping answers above one.
                2 => {
                    if !(s > zero) {
                        stage = 3;
                    } else {
                        let x = x / s;
                        if !(one > x) {
                            break one;
                        }
                        stage = 3;
                    }
                }
                // Stage D: final normalise, zero when the period is not positive.
                _ => {
                    if !(s > 0.0) {
                        break 0.0;
                    }
                    break x / s;
                }
            }
        };
        *((this + 0x140) as *mut f32) = blend;

        // Shared camera-body update, then the attachment refresh.
        let body_ok: u32 = callee_thiscall!(3, u32, this, blend.to_bits());
        let _: u32 = callee_thiscall!(4, u32, this);

        // Walk the attachment table selected by the global slot index.
        let slot = *global::<u32>(0x118D818);
        let table = global::<u32>(0x118D818u32.wrapping_add(slot.wrapping_mul(4)));
        let head = *table;
        if head != 0 {
            let count = *((head + 0x404) as *const u16) as u32;
            let mut i = 0u32;
            while i < count {
                let array = *((head + 0x400) as *const u32);
                let obj = *((array + i * 4) as *const u32);
                let vtable = *(obj as *const u32);
                let is_active: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(*((vtable + 0x34) as *const u32) as usize);
                if is_active(obj) & 0xFF == 0 {
                    i += 1;
                    continue;
                }
                let attach: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(*((vtable + 0x2c) as *const u32) as usize);
                let elem = attach(obj);
                if elem != 0 {
                    // Copy the camera pose into the element twice (current + previous).
                    let w0 = *((this + 0x40) as *const u32);
                    let w1 = *((this + 0x44) as *const u32);
                    let w2 = *((this + 0x48) as *const u32);
                    let w3 = *((this + 0x4c) as *const u32);
                    *((elem + 0x20) as *mut u32) = w0;
                    *((elem + 0x24) as *mut u32) = w1;
                    *((elem + 0x28) as *mut u32) = w2;
                    *((elem + 0x2c) as *mut u32) = w3;
                    *((elem + 0x10) as *mut u32) = w0;
                    *((elem + 0x14) as *mut u32) = w1;
                    *((elem + 0x18) as *mut u32) = w2;
                    *((elem + 0x1c) as *mut u32) = w3;
                    *((elem + 0x3c) as *mut u8) = 0;
                    let evtable = *(elem as *const u32);
                    let notify: extern "thiscall" fn(u32, u32, u32) -> u32 =
                        core::mem::transmute(*((evtable + 8) as *const u32) as usize);
                    let _: u32 = notify(elem, 0, 0xFFFF_FFFE);
                    // Mark the element's data current when the body moved or
                    // the stored blend was already zero.
                    if (body_ok & 0xFF) != 0 || stored_is_zero {
                        let data = *((elem + 0x30) as *const u32);
                        if data != 0 {
                            *((data + 0x78) as *mut u8) = 1;
                            *((data + 0x108) as *mut u8) = 0xFF;
                            *((data + 0x104) as *mut u32) = 0;
                            *((data + 0x72) as *mut u8) = 1;
                        }
                    }
                }
                // Publish the shared frame tag.
                let dst = *global::<u32>(0x11A28F4);
                let tag = *global::<u32>(0x1055380);
                *((dst + 0x260) as *mut u32) = tag;
                i += 1;
            }
        }
        1
    }
});
