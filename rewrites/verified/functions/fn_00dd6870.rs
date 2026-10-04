// original: 0x00dd6870 ui_element_set_init
//! Batch initializer for a set of UI display elements.
//!
//! Guards on a virtual readiness flag, stores two float parameters and
//! zero-initialized fields into the owner object, then builds four
//! named display elements in order (background texture, texture, font
//! string, clip-length string): each element allocates and initializes
//! a record object, resolves a color entry, runs a short sequence of
//! parameterized record calls, and finalizes the record. Finishes by
//! clearing a scratch range and invoking the owner completion hook.
//! The `flag == 2` first-section variant is dead (the flag is always 0
//! here) and is omitted; every outgoing call goes to a checker stub.
lf_checker_rt::export!(thiscall, fn_00dd6870(
    this_ptr: *mut u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3_bits: u32,
    a4_bits: u32
) -> u32 {
    unsafe {
        let this = this_ptr as u32;
        let vt = *((this) as *const u32);
        // gate: vcall this+0x140; nonzero low byte returns straight out.
        let gate_target = *(((vt).wrapping_add(0x140)) as *const u32);
        let gate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(gate_target as usize);
        let g = gate(this);
        if (g & 0xFF) != 0 {
            return g;
        }
        *((this as *mut u8).add(0x2F8)) = (g & 0xFF) as u8;
        *((this as *mut u32).add(0x1EC / 4)) = 0;
        *((this as *mut u32).add(0xA4 / 4)) = a3_bits;
        *((this as *mut u32).add(0xA8 / 4)) = 0;
        *((this as *mut u32).add(0xAC / 4)) = 0;
        *((this as *mut u32).add(0xB0 / 4)) = a4_bits;
        *((this as *mut u32).add(0xB4 / 4)) = 0;
        *((this as *mut u32).add(0xB8 / 4)) = 0;
        // The flag just stored is 0, so the `flag == 2` arm (variant A) is
        // dead; execution always continues with variant B below.
        let f48_target = *(((vt).wrapping_add(0x48)) as *const u32);
        let f48: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(f48_target as usize);
        // ---- section B (UITextureBG), field 0x1E4 ----
        let esi_b: u32 = lf_checker_rt::callee_cdecl!(3, u32, 0x25Cu32);
        let p_b: u32;
        if esi_b != 0 {
            let v0 = f48(this);
            let vt2 = *((this) as *const u32);
            let f48b: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(((vt2).wrapping_add(0x48)) as *const u32) as usize,
            );
            let v1 = f48b(this);
            let name: u32 = lf_checker_rt::callee_cdecl!(
                7,
                u32,
                lf_checker_rt::relocated(0xEFB900),
                v1
            );
            p_b = lf_checker_rt::callee_thiscall!(8, u32, esi_b, name, v0);
        } else {
            p_b = 0;
        }
        *((this as *mut u32).add(0x1E4 / 4)) = p_b;
        let v_b: u32 = lf_checker_rt::callee_cdecl!(
            10,
            u32,
            (&a3_bits as *const u32) as u32,
            0x3Eu32
        );
        lf_checker_rt::callee_thiscall!(15, u32, p_b, 0u32, 0u32, v_b, 0xFFFFFFFFu32);
        let p_b2 = *((this as *const u32).add(0x1E4 / 4));
        let q_b: u32 = core::ptr::read_volatile(p_b2 as *const u32);
        let f114_b: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*(((q_b).wrapping_add(0x114)) as *const u32) as usize);
        let mut tmp = [0u32; 6];
        // cycle 1 (mutant varies this tag)
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0u32,
            0u32
        );
        f114_b(
            p_b2, 4u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        // cycle 2 (tag 8)
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0u32,
            0u32
        );
        f114_b(
            p_b2, 8u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        // cycle 3 (tag 0x10)
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0u32,
            0u32
        );
        f114_b(
            p_b2, 0x10u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        // cycle 4 (tag 2)
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0u32,
            0u32
        );
        f114_b(
            p_b2, 2u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        *((p_b2 as *mut u32).add(0x1D8 / 4)) = 2;
        let f120_b: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_b).wrapping_add(0x120)) as *const u32) as usize);
        f120_b(p_b2, 0u32);
        // ---- section 3 (UITexture), field 0x1E0 ----
        let esi_3: u32 = lf_checker_rt::callee_cdecl!(4, u32, 0x25Cu32);
        let p_3: u32;
        if esi_3 != 0 {
            let vt3 = *((this) as *const u32);
            let f48c: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(((vt3).wrapping_add(0x48)) as *const u32) as usize,
            );
            let v0 = f48c(this);
            let vt4 = *((this) as *const u32);
            let f48d: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(((vt4).wrapping_add(0x48)) as *const u32) as usize,
            );
            let v1 = f48d(this);
            let name: u32 = lf_checker_rt::callee_cdecl!(
                7,
                u32,
                lf_checker_rt::relocated(0xEFB910),
                v1
            );
            p_3 = lf_checker_rt::callee_thiscall!(8, u32, esi_3, name, v0);
        } else {
            p_3 = 0;
        }
        *((this as *mut u32).add(0x1E0 / 4)) = p_3;
        lf_checker_rt::callee_thiscall!(15, u32, p_3, a0, a1, a2, 0xFFFFFFFFu32);
        let p_32 = *((this as *const u32).add(0x1E0 / 4));
        let q_3: u32 = core::ptr::read_volatile(p_32 as *const u32);
        let f114_3: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*(((q_3).wrapping_add(0x114)) as *const u32) as usize);
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0x40000000u32,
            0x40000000u32
        );
        f114_3(
            p_32, 6u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0xC0000000u32,
            0xC0000000u32
        );
        f114_3(
            p_32, 0x18u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        let fa0: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_3).wrapping_add(0xA0)) as *const u32) as usize);
        fa0(p_32, a4_bits);
        *((p_32 as *mut u32).add(0x1D8 / 4)) = 2;
        // ---- section 4 (UIFontString), field 0x1E8 ----
        let esi_4: u32 = lf_checker_rt::callee_cdecl!(5, u32, 0x610u32);
        let p_4: u32;
        if esi_4 != 0 {
            let vt5 = *((this) as *const u32);
            let f48e: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(((vt5).wrapping_add(0x48)) as *const u32) as usize,
            );
            let v0 = f48e(this);
            let vt6 = *((this) as *const u32);
            let f48f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(((vt6).wrapping_add(0x48)) as *const u32) as usize,
            );
            let v1 = f48f(this);
            let name: u32 = lf_checker_rt::callee_cdecl!(
                7,
                u32,
                lf_checker_rt::relocated(0xEFB920),
                v1
            );
            p_4 = lf_checker_rt::callee_thiscall!(9, u32, esi_4, name, v0);
        } else {
            p_4 = 0;
        }
        *((this as *mut u32).add(0x1E8 / 4)) = p_4;
        // NOTE: on the NULL path the faulting load below runs BEFORE the
        // color call, unlike sections B/3.
        let q_4: u32 = core::ptr::read_volatile(p_4 as *const u32);
        let v_4: u32 = lf_checker_rt::callee_cdecl!(
            11,
            u32,
            (&a4_bits as *const u32) as u32,
            7u32
        );
        let f1cc_4: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*(((q_4).wrapping_add(0x1CC)) as *const u32) as usize);
        f1cc_4(p_4, 0x41900000u32, v_4, 0u32, 2u32);
        let p_3b = *((this as *const u32).add(0x1E0 / 4));
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0u32,
            0x40400000u32
        );
        let f4c: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*(((q_3).wrapping_add(0x4C)) as *const u32) as usize);
        let r4: u32 = f4c(
            p_3b, 0x12u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        let p_4b = *((this as *const u32).add(0x1E8 / 4));
        let f104_4: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(*(((q_4).wrapping_add(0x104)) as *const u32) as usize);
        f104_4(p_4b, 6u32, r4);
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0u32,
            0x3F800000u32
        );
        let f114_4: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*(((q_4).wrapping_add(0x114)) as *const u32) as usize);
        f114_4(
            p_4b, 0x18u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        let v_4b: u32 = lf_checker_rt::callee_cdecl!(
            12,
            u32,
            (&a4_bits as *const u32) as u32,
            0x3Bu32
        );
        let p_4c = *((this as *const u32).add(0x1E8 / 4));
        let q_4c: u32 = core::ptr::read_volatile(p_4c as *const u32);
        let f208_4: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            *(((q_4c).wrapping_add(0x208)) as *const u32) as usize,
        );
        f208_4(p_4c, v_4b);
        let f1fc_4: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_4c).wrapping_add(0x1FC)) as *const u32) as usize);
        f1fc_4(p_4c, 1u32);
        let f118_4: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_4c).wrapping_add(0x118)) as *const u32) as usize);
        f118_4(p_4c, 1u32);
        let f120_4: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_4c).wrapping_add(0x120)) as *const u32) as usize);
        f120_4(p_4c, 0u32);
        let f1d4_4: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_4c).wrapping_add(0x1D4)) as *const u32) as usize);
        f1d4_4(p_4c, 0x12u32);
        // ---- section 5 (UIClipLengthString), field 0x1EC ----
        let esi_5: u32 = lf_checker_rt::callee_cdecl!(6, u32, 0x610u32);
        let p_5: u32;
        if esi_5 != 0 {
            let vt7 = *((this) as *const u32);
            let f48g: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(((vt7).wrapping_add(0x48)) as *const u32) as usize,
            );
            let v0 = f48g(this);
            let vt8 = *((this) as *const u32);
            let f48h: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(((vt8).wrapping_add(0x48)) as *const u32) as usize,
            );
            let v1 = f48h(this);
            let name: u32 = lf_checker_rt::callee_cdecl!(
                7,
                u32,
                lf_checker_rt::relocated(0xEFB930),
                v1
            );
            p_5 = lf_checker_rt::callee_thiscall!(9, u32, esi_5, name, v0);
        } else {
            p_5 = 0;
        }
        *((this as *mut u32).add(0x1EC / 4)) = p_5;
        let q_5: u32 = core::ptr::read_volatile(p_5 as *const u32);
        let v_5: u32 = lf_checker_rt::callee_cdecl!(
            11,
            u32,
            (&a4_bits as *const u32) as u32,
            7u32
        );
        let f1cc_5: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*(((q_5).wrapping_add(0x1CC)) as *const u32) as usize);
        f1cc_5(p_5, 0x41900000u32, v_5, 0u32, 2u32);
        let p_3c = *((this as *const u32).add(0x1E0 / 4));
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0u32,
            0xC1500000u32
        );
        let f4c_5: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*(((q_3).wrapping_add(0x4C)) as *const u32) as usize);
        let r5: u32 = f4c_5(
            p_3c, 6u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        let p_5b = *((this as *const u32).add(0x1EC / 4));
        let f104_5: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(*(((q_5).wrapping_add(0x104)) as *const u32) as usize);
        f104_5(p_5b, 6u32, r5);
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        let h: u32 = lf_checker_rt::callee_thiscall!(
            16,
            u32,
            tmp.as_mut_ptr() as u32,
            0u32,
            0xBF800000u32
        );
        let r5b: u32 = f4c_5(
            p_3c, 6u32, *((h) as *const u32), *((h.wrapping_add(4)) as *const u32),
            *((h.wrapping_add(8)) as *const u32), *((h.wrapping_add(12)) as *const u32),
            *((h.wrapping_add(16)) as *const u32), *((h.wrapping_add(20)) as *const u32),
        );
        f104_5(p_5b, 0x18u32, r5b);
        lf_checker_rt::callee_thiscall!(17, u32, tmp.as_mut_ptr() as u32);
        let v_5b: u32 = lf_checker_rt::callee_cdecl!(
            14,
            u32,
            (&a4_bits as *const u32) as u32,
            2u32
        );
        let p_5c = *((this as *const u32).add(0x1EC / 4));
        let q_5c: u32 = core::ptr::read_volatile(p_5c as *const u32);
        let f208_5: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            *(((q_5c).wrapping_add(0x208)) as *const u32) as usize,
        );
        f208_5(p_5c, v_5b);
        let f1fc_5: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_5c).wrapping_add(0x1FC)) as *const u32) as usize);
        f1fc_5(p_5c, 1u32);
        let f118_5: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_5c).wrapping_add(0x118)) as *const u32) as usize);
        f118_5(p_5c, 1u32);
        let f120_5: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_5c).wrapping_add(0x120)) as *const u32) as usize);
        f120_5(p_5c, 0u32);
        let f1d4_5: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_5c).wrapping_add(0x1D4)) as *const u32) as usize);
        f1d4_5(p_5c, 0x12u32);
        let f1f8_5: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(((q_5c).wrapping_add(0x1F8)) as *const u32) as usize);
        f1f8_5(p_5c, 0u32);
        // ---- epilogue ----
        lf_checker_rt::callee_cdecl!(30, u32, this.wrapping_add(0x1F8), 0u32, 0x100u32);
        let vt9 = *((this) as *const u32);
        let f13c: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            *(((vt9).wrapping_add(0x13C)) as *const u32) as usize,
        );
        return f13c(this, 1u32);
    }};
}

lf_checker_rt::export!(thiscall, rw_dd6870(
    this_ptr: *mut u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3_bits: u32,
    a4_bits: u32
) -> u32 {
    unsafe { body!(4u32, this_ptr, a0, a1, a2, a3_bits, a4_bits) }
    }
});
