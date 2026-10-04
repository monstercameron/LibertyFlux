// original: 0x00bf4f70 vehfx_glass_emitter_setup
/// Look up the effect record for a vehicle, create its glass emitter and
/// configure it from the record's frame.
///
/// `this` is the vehicle FX object, `arg0` the vehicle record. A mode
/// byte selects one of three glass effect names (anything out of range
/// ends the call). Callee 2 creates the emitter; callees 3 and 4 fetch
/// two input buffers, callee 5 mixes them; after attach, aim-buffer
/// setup and registration, the first buffer is transformed by the
/// record's 3x3 frame plus offset (exact single-precision order below)
/// and handed with two constants to callee 9. The record's virtual aim
/// hook (slot 0xec) supplies a triple stored to the emitter, then
/// callee 11 finishes.
///
/// Returns 0; the original returns void (checker `ret: none`).
/// Callee ids: 1 hash-name, 2 create-emitter, 3 fetch-in1, 4 fetch-in2,
/// 5 mix, 6 aim-buffer, 7 attach, 8 register, 9 transform-sink,
/// 10 aim-hook (planted virtual), 11 finish, 12 pre-sink.
export!(thiscall, rw_00bf4f70(this: u32, arg0: u32) -> u32 {
    unsafe {
        if arg0 == 0 {
            return 0;
        }
        let fx = relocated(0x1394d60);
        let t = (*(this.wrapping_add(0x1c) as *const u8) as u32)
            .wrapping_add(0xffffffce);
        if t > 0x0c {
            return 0;
        }
        let name = match t {
            0 | 1 | 11 | 12 => 0xebbc0c_u32,
            2 | 3 | 8 | 9 | 10 => 0xebbc20,
            _ => 0xebbc30,
        };
        let h: u32 = callee_cdecl!(1, u32, relocated(name), 0);
        let emitter: u32 = callee_thiscall!(2, u32, fx, h);
        if emitter == 0 {
            return 0;
        }
        // Callees 3/4 observe ambient register state as their `this`
        // (the original loads it from below its own saved slot); the
        // contract compares no registers for them, so any value passes.
        let mut b1: [u32; 3] = [0, 0, 0];
        callee_thiscall!(3, u32, this, b1.as_mut_ptr() as u32);
        let mut b2: [u32; 4] = [0, 0, 0, 0];
        callee_thiscall!(4, u32, this, b2.as_mut_ptr() as u32);
        let mut bx: [u32; 4] = [0, 0, 0, 0];
        callee_cdecl!(
            5, u32,
            bx.as_mut_ptr() as u32,
            b1.as_mut_ptr() as u32,
            b2.as_mut_ptr() as u32,
            0
        );
        callee_thiscall!(
            7, u32, emitter,
            *(arg0.wrapping_add(0x20) as *const u32)
        );
        let mut bg: [u32; 5] = [0, 0, 0, 0, 0];
        callee_thiscall!(6, u32, emitter, bg.as_mut_ptr() as u32);
        callee_thiscall!(8, u32, fx, emitter, arg0, 0);
        // Transform: result[i] = dot(column i, in1) + offset[i], with
        // the original's per-term order. Intrinsics pin the instruction
        // selection; inputs stay clear of NaN/Inf per the contract, so
        // the compiler's operand choice cannot change any bit.
        let base = *(arg0.wrapping_add(0x20) as *const u32);
        let at = |i: u32| *((base.wrapping_add(i * 4)) as *const f32);
        let s = |x: f32| core::arch::x86::_mm_set_ss(x);
        let v0 = s(f32::from_bits(b1[0]));
        let v1 = s(f32::from_bits(b1[1]));
        let v2 = s(f32::from_bits(b1[2]));
        let mul = |a: f32, b: core::arch::x86::__m128| {
            core::arch::x86::_mm_mul_ss(s(a), b)
        };
        let add = |a: core::arch::x86::__m128, b: core::arch::x86::__m128| {
            core::arch::x86::_mm_add_ss(a, b)
        };
        let t = mul(at(0), v0);
        let r0 = add(add(mul(at(4), v1), t), mul(at(8), v2));
        let r0 = add(r0, s(at(12)));
        let t = mul(at(1), v0);
        let r1 = add(add(mul(at(5), v1), t), mul(at(9), v2));
        let r1 = add(r1, s(at(13)));
        let t = mul(at(2), v0);
        let r2 = add(add(mul(at(6), v1), t), mul(at(10), v2));
        let r2 = add(r2, s(at(14)));
        let mut r: [u32; 4] = [
            core::arch::x86::_mm_cvtss_f32(r0).to_bits(),
            core::arch::x86::_mm_cvtss_f32(r1).to_bits(),
            core::arch::x86::_mm_cvtss_f32(r2).to_bits(),
            b2[3],
        ];
        callee_thiscall!(12, u32, emitter);
        callee_cdecl!(
            9, u32, emitter,
            r.as_mut_ptr() as u32,
            0x40200000,
            0xbf800000
        );
        let vtable = *(arg0 as *const u32);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtable.wrapping_add(0xec)) as *const u32));
        let mut scratch: [u32; 4] = [0, 0, 0, 0];
        let ans = hook(arg0, scratch.as_mut_ptr() as u32);
        *(emitter.wrapping_add(0x190) as *mut u32) = *(ans as *const u32);
        *(emitter.wrapping_add(0x194) as *mut u32) =
            *((ans.wrapping_add(4)) as *const u32);
        *(emitter.wrapping_add(0x198) as *mut u32) =
            *((ans.wrapping_add(8)) as *const u32);
        callee_thiscall!(11, u32, emitter);
        0
    }
});
