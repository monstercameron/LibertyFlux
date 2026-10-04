// original: 0x00bf4cf0 vehfx_fueltank_emitter_setup
/// Look up the effect record for a vehicle, create its fuel-tank emitter,
/// aim it from the vehicle's virtual aim point and configure it.
///
/// `this` is the vehicle FX object, `arg0` the vehicle record. Callee 1
/// tests the record; when it answers nonzero the truck effect name is
/// used, otherwise the record's kind field selects car, bike, boat or
/// heli (anything else ends the call). Callee 3 creates the emitter.
/// The record's virtual aim hook (slot 0xec) is called twice: the first
/// answer's three words become the emitter's aim triple, the second
/// answer is a vector whose scaled length, clamped to 1.0, becomes the
/// speed parameter. Callee 7 registers the emitter; when the creation
/// flag is set, callee 8 fetches a point that is re-based into the
/// record's frame (exact single-precision order below), optionally
/// mirrored, and handed to callee 9, before callee 10 finishes.
///
/// Returns 0; the original returns void (checker `ret: none`).
/// Callee ids: 1 record-test, 2 hash-name, 3 create-emitter, 4 aim-hook
/// (planted virtual, sequenced answers), 5 set-param, 6 attach,
/// 7 register, 8 fetch-point, 9 aim-emitter, 10 finish.
export!(thiscall, rw_00bf4cf0(this: u32, arg0: u32) -> u32 {
    unsafe {
        if arg0 == 0 {
            return 0;
        }
        let fx = relocated(0x1394d60);
        let bit = *(this.wrapping_add(0x24) as *const u8) & 1;
        let probe: u32 = callee_thiscall!(1, u32, relocated(0x16d9f58), arg0);
        let name = if probe & 0xFF != 0 {
            0xebbb9c_u32
        } else {
            match *(arg0.wrapping_add(0x1304) as *const u32) {
                0 => 0xebbbb4,
                1 => 0xebbbc8,
                2 => 0xebbbdc,
                4 => 0xebbbf0,
                _ => return 0,
            }
        };
        let h: u32 = callee_cdecl!(2, u32, relocated(name), 0);
        let mut flag: u32 = 0;
        let emitter: u32 = callee_thiscall!(
            3, u32, fx,
            arg0.wrapping_add(6).wrapping_add(bit as u32),
            h,
            (&mut flag as *mut u32) as u32,
            0,
            0
        );
        if emitter == 0 {
            return 0;
        }
        // The record's aim hook, called through its vtable exactly like
        // the original; both calls land on the same planted stub.
        let vtable = *(arg0 as *const u32);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtable.wrapping_add(0xec)) as *const u32));
        let mut scratch: [u32; 4] = [0, 0, 0, 0];
        let aim1 = hook(arg0, scratch.as_mut_ptr() as u32);
        *(emitter.wrapping_add(0x190) as *mut u32) = *(aim1 as *const u32);
        *(emitter.wrapping_add(0x194) as *mut u32) =
            *((aim1.wrapping_add(4)) as *const u32);
        *(emitter.wrapping_add(0x198) as *mut u32) =
            *((aim1.wrapping_add(8)) as *const u32);
        let aim2 = hook(arg0, scratch.as_mut_ptr() as u32);
        let vx = *(aim2 as *const f32);
        let vy = *((aim2.wrapping_add(4)) as *const f32);
        let vz = *((aim2.wrapping_add(8)) as *const f32);
        let x2 = vx * vx;
        let y2 = vy * vy;
        let z2 = vz * vz;
        let mut len = x2 + y2;
        len += z2;
        len = len.sqrt();
        len *= *global::<f32>(0xfe879c);
        let speed = if len < 1.0 { len } else { 1.0 };
        callee_thiscall!(5, u32, emitter, relocated(0xebbc04), speed.to_bits());
        callee_thiscall!(6, u32, emitter, *(arg0.wrapping_add(0x20) as *const u32));
        callee_thiscall!(7, u32, fx, emitter, arg0, 0);
        if flag & 0xFF == 0 {
            return 0;
        }
        let mut pt: [u32; 3] = [0, 0, 0];
        callee_thiscall!(8, u32, this, pt.as_mut_ptr() as u32);
        // Re-base the fetched point into the record's frame. The
        // intrinsics pin the instruction selection, but rustc still
        // chooses the operand order of the commutative adds, which can
        // change NaN payloads; the contract therefore keeps NaN/Inf out
        // of the matrix, where every input combination here is bit-exact.
        let base = *(arg0.wrapping_add(0x20) as *const u32);
        let at = |i: u32| *((base.wrapping_add(i * 4)) as *const f32);
        let s = |x: f32| core::arch::x86::_mm_set_ss(x);
        let d0 = core::arch::x86::_mm_sub_ss(s(f32::from_bits(pt[0])), s(at(12)));
        let d1 = core::arch::x86::_mm_sub_ss(s(f32::from_bits(pt[1])), s(at(13)));
        let d2 = core::arch::x86::_mm_sub_ss(s(f32::from_bits(pt[2])), s(at(14)));
        let mul = |a: f32, b: core::arch::x86::__m128| {
            core::arch::x86::_mm_mul_ss(s(a), b)
        };
        let add = |a: core::arch::x86::__m128, b: core::arch::x86::__m128| {
            core::arch::x86::_mm_add_ss(a, b)
        };
        let t = mul(at(0), d0);
        let r0 = add(add(mul(at(1), d1), t), mul(at(2), d2));
        let t = mul(at(4), d0);
        let r1 = add(add(mul(at(5), d1), t), mul(at(6), d2));
        let t = mul(at(8), d0);
        let r2 = add(add(mul(at(9), d1), t), mul(at(10), d2));
        let mut r0 = core::arch::x86::_mm_cvtss_f32(r0);
        let r1 = core::arch::x86::_mm_cvtss_f32(r1);
        let r2 = core::arch::x86::_mm_cvtss_f32(r2);
        if *(this.wrapping_add(0x24) as *const u8) & 1 != 0 {
            r0 = -r0;
            *(emitter.wrapping_add(0x1e2) as *mut u8) = 1;
        }
        pt[0] = r0.to_bits();
        pt[1] = r1.to_bits();
        pt[2] = r2.to_bits();
        callee_thiscall!(9, u32, emitter, pt.as_mut_ptr() as u32);
        callee_thiscall!(10, u32, emitter);
        0
    }
});
