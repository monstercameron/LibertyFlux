// original: 0x00BF46D0 vehfx_heli_rotor_emitter_setup
/// Selects a vehicle-effect emitter path by hashed-name gates, then builds
/// and configures the emitter through a chain of engine calls.
///
/// Takes the owner in `this` and a vehicle record in `edi`; returns nothing
/// and exits at once for a null record. Three hashed-name probes against
/// the owner's key word choose the path: a match on either of the first two
/// takes path B, a match only on the third (against the same key) takes
/// path A, and no match returns. Path A resolves an emitter id through a
/// vehicle table indexed by a signed word from the record (a negative id
/// returns), opens the emitter, fetches a float triple from a parameter
/// block scaled by the id, submits it twice, colorizes the emitter from a
/// color table entry picked by a record byte (each channel widened to float
/// and scaled by a reciprocal global), then runs create, attach and finish
/// calls. Path B resolves the same table, probes a fourth hash to pick one
/// of two id pairs (each with a negative-fallback word), opens another
/// emitter, runs three matrix fills with a twelve-word interleave copy
/// between the first two, then repeats the submit/colorless tail with a
/// wild scaled table read and a final guarded finish call.
///
/// The matrix words written by the fill stub are read back volatile; every
/// buffer handed to a stub is stored volatile. Wild scaled reads fault or
/// return whatever the mapping holds on both sides identically.
lf_checker_rt::export!(thiscall, aq55_fn2(this: u32, edi: u32) -> () {
    unsafe { run_bf46d0(this, edi, false) }
});
lf_checker_rt::export!(thiscall, aq55_fn2m(this: u32, edi: u32) -> () {
    unsafe { run_bf46d0(this, edi, true) }
});

/// Shared body; `mutant` swaps the red and blue emitter channels.
unsafe fn run_bf46d0(this: u32, edi: u32, mutant: bool) {
    const CAL_HASH3: u32 = 1; // hashed-name probe, three-call sequence
    const CAL_HASH1: u32 = 2; // path-B hashed-name probe
    const CAL_OPEN: u32 = 3; // emitter open (thiscall/3)
    const CAL_TAB: u32 = 4; // parameter block fetch (thiscall/0)
    const CAL_SET: u32 = 5; // parameter submit (thiscall/1)
    const CAL_VEC: u32 = 6; // vector submit (thiscall/1, frame pointer)
    const CAL_REG: u32 = 7; // emitter register (thiscall/3)
    const CAL_CREATE: u32 = 8; // emitter create (thiscall/0)
    const CAL_ATTACH: u32 = 9; // emitter attach (cdecl/5)
    const CAL_FINISH: u32 = 10; // emitter finish (thiscall/0)
    const CAL_OPEN2: u32 = 11; // second emitter open (thiscall/1)
    const CAL_MAT: u32 = 12; // matrix fill (thiscall/1, frame pointer)
    const CAL_MAT2: u32 = 13; // second matrix fill (thiscall/1)
    const CAL_PREP: u32 = 14; // emitter prepare (thiscall/1)
    const CAL_GUARD: u32 = 15; // guarded finish (thiscall/2)

    const STR_A: u32 = 0x00EBBE54;
    const STR_B: u32 = 0x00EBBE6C;
    const STR_C: u32 = 0x00EBBE9C;
    const STR_D: u32 = 0x00EBBE84;
    const G_OBJ: u32 = 0x01394D60; // engine object for open/register
    const G_VTAB: u32 = 0x01295CD8; // vehicle table (indexed by record word)
    const G_COL: u32 = 0x012FAE88; // color table (indexed by record byte)
    const G_RECIP: u32 = 0x00FE86E8; // channel scale reciprocal
    const G_G1: u32 = 0x012FA56C; // guard compare words (pristine)
    const G_G2: u32 = 0x012F9F30;

    #[inline(always)]
    unsafe fn load(base: u32, off: u32) -> u32 {
        *((base.wrapping_add(off)) as *const u32)
    }
    #[inline(always)]
    unsafe fn vstore(slot: *mut u32, v: u32) {
        core::ptr::write_volatile(slot, v);
    }
    #[inline(always)]
    unsafe fn vload(slot: *const u32) -> u32 {
        core::ptr::read_volatile(slot)
    }

    unsafe {
        if edi == 0 {
            return;
        }
        let key = load(this, 8);
        let h1 = lf_checker_rt::callee_cdecl!(CAL_HASH3, u32, lf_checker_rt::relocated(STR_A), 0);
        if key == h1 {
            return path_b(this, edi, mutant);
        }
        let h2 = lf_checker_rt::callee_cdecl!(CAL_HASH3, u32, lf_checker_rt::relocated(STR_B), 0);
        if key == h2 {
            return path_b(this, edi, mutant);
        }
        let h3 = lf_checker_rt::callee_cdecl!(CAL_HASH3, u32, lf_checker_rt::relocated(STR_C), 0);
        if key != h3 {
            return;
        }
        // Path A.
        let idx = *((edi.wrapping_add(0x2E)) as *const i16) as i32;
        let sub = *(lf_checker_rt::global::<u32>(G_VTAB).add(idx as usize));
        let blk = load(sub, 0xCC);
        let id = load(blk, 0xA0) as i32;
        if id < 0 {
            return;
        }
        let emitter = lf_checker_rt::callee_thiscall!(
            CAL_OPEN, u32, lf_checker_rt::relocated(G_OBJ), key, 0, 0
        );
        if emitter == 0 {
            return;
        }
        let tbl = lf_checker_rt::callee_thiscall!(CAL_TAB, u32, edi);
        let base = load(tbl, 0);
        let off = (id as u32).wrapping_mul(0xE0);
        let f0 = *((base.wrapping_add(off).wrapping_add(0x20)) as *const f32);
        let f1 = *((base.wrapping_add(off).wrapping_add(0x24)) as *const f32);
        let f2 = *((base.wrapping_add(off).wrapping_add(0x28)) as *const f32);
        let arg20 = load(edi, 0x20);
        lf_checker_rt::callee_thiscall!(CAL_SET, u32, emitter, arg20);
        let mut vec = [0u32; 3];
        vstore(vec.as_mut_ptr(), f0.to_bits());
        vstore(vec.as_mut_ptr().add(1), f1.to_bits());
        vstore(vec.as_mut_ptr().add(2), f2.to_bits());
        lf_checker_rt::callee_thiscall!(CAL_VEC, u32, emitter, vec.as_ptr() as u32);
        lf_checker_rt::callee_thiscall!(
            CAL_REG, u32, lf_checker_rt::relocated(G_OBJ), emitter, edi, 0
        );
        let ci = *((edi.wrapping_add(0xF94)) as *const u8) as usize;
        let color = *(lf_checker_rt::global::<u32>(G_COL).add(ci));
        let recip = *(lf_checker_rt::global::<f32>(G_RECIP));
        let r = (((color >> 16) & 0xFF) as f32) * recip;
        let g = (((color >> 8) & 0xFF) as f32) * recip;
        let b = ((color & 0xFF) as f32) * recip;
        // The mutant swaps the red and blue channels.
        let (r, b) = if mutant { (b, r) } else { (r, b) };
        *((emitter.wrapping_add(0x180)) as *mut f32) = r;
        *((emitter.wrapping_add(0x184)) as *mut f32) = g;
        *((emitter.wrapping_add(0x188)) as *mut f32) = b;
        lf_checker_rt::callee_thiscall!(CAL_CREATE, u32, emitter);
        lf_checker_rt::callee_cdecl!(
            CAL_ATTACH, u32, emitter, arg20.wrapping_add(0x30),
            0x41200000, 0x40800000, 0xBF800000
        );
        lf_checker_rt::callee_thiscall!(CAL_FINISH, u32, emitter);
    }

    /// Path B: alternate id select, matrix fills with interleave copy,
    /// then the register/create/guard/finish tail.
    unsafe fn path_b(this: u32, edi: u32, _mutant: bool) {
        const CAL_HASH1: u32 = 2;
        const CAL_OPEN: u32 = 3;
        const CAL_SET: u32 = 5;
        const CAL_VEC: u32 = 6;
        const CAL_REG: u32 = 7;
        const CAL_CREATE: u32 = 8;
        const CAL_FINISH: u32 = 10;
        const CAL_OPEN2: u32 = 11;
        const CAL_MAT: u32 = 12;
        const CAL_MAT2: u32 = 13;
        const CAL_PREP: u32 = 14;
        const CAL_GUARD: u32 = 15;
        const STR_D: u32 = 0x00EBBE84;
        const G_OBJ: u32 = 0x01394D60;
        const G_VTAB: u32 = 0x01295CD8;
        const G_G1: u32 = 0x012FA56C;
        const G_G2: u32 = 0x012F9F30;
        unsafe {
            let idx = *((edi.wrapping_add(0x2E)) as *const i16) as i32;
            let tptr = *(lf_checker_rt::global::<u32>(G_VTAB).add(idx as usize));
            let h4 =
                lf_checker_rt::callee_cdecl!(CAL_HASH1, u32, lf_checker_rt::relocated(STR_D), 0);
            let blk = *((tptr.wrapping_add(0xCC)) as *const u32);
            let id: u32;
            if *((this.wrapping_add(8)) as *const u32) == h4 {
                let a = *((blk.wrapping_add(0x170)) as *const u32);
                id = if a != 0xFFFF_FFFF {
                    a
                } else {
                    *((blk.wrapping_add(0x16C)) as *const u32)
                };
            } else {
                let a = *((blk.wrapping_add(0x168)) as *const u32);
                id = if a != 0xFFFF_FFFF {
                    a
                } else {
                    *((blk.wrapping_add(0x164)) as *const u32)
                };
            }
            if (id as i32) < 0 {
                return;
            }
            let em2 = lf_checker_rt::callee_thiscall!(CAL_OPEN2, u32, edi, id);
            let arg20 = *((edi.wrapping_add(0x20)) as *const u32);
            let mut m = [0u32; 15];
            lf_checker_rt::callee_thiscall!(CAL_MAT, u32, m.as_ptr() as u32, arg20);
            // Twelve-word interleave copy: rows of three, every fourth
            // source word skipped.
            let w = |i: usize| core::ptr::read_volatile(m.as_ptr().add(i));
            let mut d = [0u32; 12];
            let src = [0usize, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14];
            let mut k = 0;
            while k < 12 {
                core::ptr::write_volatile(d.as_mut_ptr().add(k), w(src[k]));
                k += 1;
            }
            let d3 = core::ptr::read_volatile(d.as_ptr().add(3));
            lf_checker_rt::callee_thiscall!(CAL_MAT2, u32, d.as_ptr().add(11) as u32, d3);
            let w2 = w(2);
            lf_checker_rt::callee_thiscall!(CAL_MAT, u32, m.as_ptr() as u32, w2);
            lf_checker_rt::callee_thiscall!(CAL_PREP, u32, em2, edi);
            lf_checker_rt::callee_thiscall!(
                CAL_OPEN, u32, lf_checker_rt::relocated(G_OBJ), em2, 0, 0
            );
            // Wild scaled table read: the slot holds the table pointer,
            // scaled by row stride with wraparound.
            let edx = tptr.wrapping_mul(0xE0);
            let base = *((em2.wrapping_add(8)) as *const u32);
            let f0 = core::ptr::read_volatile(
                (base.wrapping_add(edx).wrapping_add(0x20)) as *const f32
            );
            let f1 = core::ptr::read_volatile(
                (base.wrapping_add(edx).wrapping_add(0x24)) as *const f32
            );
            let f2 = core::ptr::read_volatile(
                (base.wrapping_add(edx).wrapping_add(0x28)) as *const f32
            );
            let _ = (f0, f1, f2);
            lf_checker_rt::callee_thiscall!(CAL_SET, u32, em2, arg20);
            let mut sbuf = [0u32; 3];
            core::ptr::write_volatile(sbuf.as_mut_ptr(), tptr);
            core::ptr::write_volatile(sbuf.as_mut_ptr().add(1), 0);
            core::ptr::write_volatile(sbuf.as_mut_ptr().add(2), f0.to_bits());
            lf_checker_rt::callee_thiscall!(CAL_VEC, u32, em2, sbuf.as_ptr() as u32);
            lf_checker_rt::callee_thiscall!(
                CAL_REG, u32, lf_checker_rt::relocated(G_OBJ), em2, edi, 0
            );
            lf_checker_rt::callee_thiscall!(CAL_CREATE, u32, em2);
            let a2c = *((edi.wrapping_add(0x2C)) as *const u32);
            let ans = lf_checker_rt::callee_thiscall!(CAL_GUARD, u32, em2, a2c, arg20);
            if ans == *(lf_checker_rt::global::<u32>(G_G1)) {
                return;
            }
            if ans != *(lf_checker_rt::global::<u32>(G_G2)) {
                return;
            }
            lf_checker_rt::callee_thiscall!(CAL_FINISH, u32, em2);
        }
    }
}
