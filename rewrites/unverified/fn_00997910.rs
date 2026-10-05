// original: 0x00997910 ROTOR_VOLUME (merged symbol; low-confidence placeholder name)

/// Update the rotor-volume voice (`ROTOR_VOLUME` is the inventory's
/// placeholder name): transform the direction vector, drive seven channel
/// objects, then forward the frame to 0x997370.
///
/// `this` is the voice object, `arg` the frame struct. The unrelocated
/// writable dword at file VA 0x11735B4 is saved for the tail call, and a
/// mode word is chosen by comparing the unrelocated read-only constant
/// `K1` (0xFE88E8) against the unrelocated writable threshold at
/// 0x103234C (strictly greater, ordered, selects the unrelocated writable
/// word at 0x1038C8C, else 0). Seven channel pointers at `this+0xA88` ..
/// `this+0xAA0` must all be non-null or the function returns the mode word.
///
/// Otherwise: an index from `[this+0x820]+0x2E` selects a table entry from
/// the unrelocated writable table at 0x1295CD8; callee 32 fills a 45-word
/// frame vector (callee 31 resolves its seed); a 3x4 matrix at
/// `[[this+0x820]+0x20]` transforms the vector in place (rows at
/// `+0x00/0x04/0x08`, `+0x10/0x14/0x18`, `+0x20/0x24/0x28`, translation at
/// `+0x30/0x34/0x38`; the `w` slot passes through); callee 33 consumes the
/// result. A rotor term starts at `K3` (unrelocated read-only constant at
/// 0xFE8DF8) and is zeroed when the voice is active (callee 34, else callee
/// 35 through `[this+0x820]+0xF50`, plus byte `this+0xB3E`) while the
/// unrelocated writable word at 0x1168A58 equals 2. Callee 36 takes four
/// constants (all 1.0, or 1.0/0.8/0.05/0.0 when bit 3 of
/// `[this+0x820]+0xF17` is clear) and callee 37 answers the blend factor.
/// Two derived cutoffs come from comparing zero against the floats at
/// `[this+0x820]+0x1EF4/0x1EF8/0x1EFC` (each `<= 0.0`, NaN counts as above).
/// Seven `set` calls (callee 38) and three `set-indirect` calls (callee 39)
/// drive the channels; one index is the signed max of `[arg+8]+mode` and
/// the unrelocated writable word at 0x1038CB8. A second callee-34 test plus
/// the unrelocated writable word at 0x11F70CC equalling 4 selects object
/// creation (callees 40/41, then 42) over release (callees 43/44). Two
/// lookups (callee 45, cdecl) each feed a register-indirect call through
/// slot 0 of `[[this+0xA90]]` (callee 46, planted stub); non-null results
/// receive `[arg+0x0C]`/`[arg+0x04]`. Finally the frame is forwarded to
/// 0x997370 (callee 47) with `(saved, arg)`, `[this+0xAD4]` gets
/// `[arg+0x0C]` plus the first cutoff, and the forward result is returned.
///
/// Original: 0x00997910 (thiscall, one stack word). Unverifiable on the
/// stock v5 worker and under read-only shadowing alike: the original reads
/// unrelocated writable data on every path, starting at its first memory
/// access.
lf_checker_rt::export!(thiscall, rw_00997910(this: u32, arg: u32) -> u32 {
    unsafe {
        const SEED_CALLEE: u32 = 31;
        const VEC_CALLEE: u32 = 32;
        const USE_CALLEE: u32 = 33;
        const TEST_CALLEE: u32 = 34;
        const TEST2_CALLEE: u32 = 35;
        const K4_CALLEE: u32 = 36;
        const BLEND_CALLEE: u32 = 37;
        const SET_CALLEE: u32 = 38;
        const SETI_CALLEE: u32 = 39;
        const MK_CALLEE: u32 = 40;
        const MK2_CALLEE: u32 = 41;
        const USEOBJ_CALLEE: u32 = 42;
        const REL1_CALLEE: u32 = 43;
        const REL2_CALLEE: u32 = 44;
        const LOOKUP_CALLEE: u32 = 45;
        const FWD_CALLEE: u32 = 47;
        const SAVE_VA: u32 = 0x0117_35B4;
        const K1_VA: u32 = 0x00FE_88E8;
        const THR_VA: u32 = 0x0103_234C;
        const MODE_VA: u32 = 0x0103_8C8C;
        const VTAB_VA: u32 = 0x0129_5CD8;
        const K3_VA: u32 = 0x00FE_8DF8;
        const ACTIVE_VA: u32 = 0x0116_8A58;
        const G80_VA: u32 = 0x0103_8C80;
        const CLAMP_VA: u32 = 0x0103_8CB8;
        const MODE2_VA: u32 = 0x011F_70CC;
        const MINUS100: u32 = 0xC2C8_0000;
        const NINE: u32 = 0x4110_0000;
        const ONE: u32 = 0x3F80_0000;
        const P08: u32 = 0x3F4C_CCCD;
        const P005: u32 = 0x3E4C_CCCD;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn set(obj: u32, x: f32) {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) =
                    core::mem::transmute(lf_checker_rt::callee_addr(SET_CALLEE) as usize);
                f(obj, x.to_bits())
            }
        }
        #[inline(always)]
        unsafe fn seti(obj: u32, v: u32) {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::callee_addr(SETI_CALLEE) as usize);
                f(obj, v);
            }
        }
        #[inline(always)]
        unsafe fn test0() -> u32 {
            unsafe {
                let f: extern "cdecl" fn() -> u32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(TEST_CALLEE) as usize,
                );
                f()
            }
        }

        unsafe {
            let saved = g32(SAVE_VA);
            let k1 = f32::from_bits(g32(K1_VA));
            let thr = f32::from_bits(g32(THR_VA));
            let mode = if k1 > thr { g32(MODE_VA) } else { 0 };
            if rd32(this + 0xa94) == 0
                || rd32(this + 0xa90) == 0
                || rd32(this + 0xa8c) == 0
                || rd32(this + 0xa88) == 0
                || rd32(this + 0xa98) == 0
                || rd32(this + 0xa9c) == 0
                || rd32(this + 0xaa0) == 0
            {
                return mode;
            }
            let p = rd32(this + 0x820);
            let sw = rd16(p + 0x2e) as i16 as i32;
            let t = rd32(g32(VTAB_VA).wrapping_add((sw as u32).wrapping_mul(4)));
            let a = rd32(t + 0xcc);
            let s = rd32(a + 0x16c);
            let seed: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(SEED_CALLEE) as usize);
            let pres = seed(p);
            let s2 = s.wrapping_shl(6).wrapping_add(rd32(pres + 0x10));
            let mut m = [0u32; 45];
            let b = m.as_mut_ptr() as u32;
            let fill: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(VEC_CALLEE) as usize);
            fill(b, s2);
            let vx = f32::from_bits(m[0x42 / 4]);
            let vy = f32::from_bits(m[0x46 / 4]);
            let vz = f32::from_bits(m[0x4a / 4]);
            let w = m[0xae / 4];
            let mb = rd32(p + 0x20);
            let m0 = rdf(mb);
            let m4 = rdf(mb + 4);
            let m8 = rdf(mb + 8);
            let m10 = rdf(mb + 0x10);
            let m14 = rdf(mb + 0x14);
            let m18 = rdf(mb + 0x18);
            let m20 = rdf(mb + 0x20);
            let m24 = rdf(mb + 0x24);
            let m28 = rdf(mb + 0x28);
            let m30 = rdf(mb + 0x30);
            let m34 = rdf(mb + 0x34);
            let m38 = rdf(mb + 0x38);
            let nx = add(add(add(mul(m10, vy), mul(m0, vx)), mul(m20, vz)), m30);
            let ny = add(add(add(mul(m4, vx), mul(m14, vy)), mul(m24, vz)), m34);
            let nz = add(add(add(mul(m8, vx), mul(m18, vy)), mul(m28, vz)), m38);
            m[0x42 / 4] = nx.to_bits();
            m[0x46 / 4] = ny.to_bits();
            m[0x4a / 4] = nz.to_bits();
            m[0x4e / 4] = w;
            let usev: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(USE_CALLEE) as usize);
            usev(rd32(this + 0xa94), b + 0x42);

            let k3 = f32::from_bits(g32(K3_VA));
            let mut t_rotor = k3;
            let al = (test0() & 0xFF) as u8;
            let mut go = al != 0 && rd8(this + 0xb3e) != 0;
            if al == 0 {
                let qq = rd32(p + 0xf50);
                if qq != 0 {
                    let t2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                        lf_checker_rt::callee_addr(TEST2_CALLEE) as usize,
                    );
                    if (t2(qq) & 0xFF) as u8 != 0 {
                        go = true;
                    }
                }
            }
            if go && g32(ACTIVE_VA) == 2 {
                t_rotor = 0.0;
            }
            let esi1 = this + 0x3d8;
            let k4: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(K4_CALLEE) as usize);
            if rd8(p + 0xf17) & 8 != 0 {
                k4(esi1, ONE, ONE, ONE, ONE);
            } else {
                k4(esi1, 0, P005, P08, ONE);
            }
            let blend: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(
                lf_checker_rt::callee_addr(BLEND_CALLEE) as usize,
            );
            let bf = blend(esi1);
            let g80 = f32::from_bits(g32(G80_VA));
            let d = sub(k1, rdf(arg + 0x30));
            let t164 = mul(g80, bf);
            let g80b = sub(g80, t164);
            let e1 = mul(d, g80b);
            let cut0 = if rdf(p + 0x1ef4) <= 0.0 { k3 } else { 0.0 };
            let t184 = if rdf(p + 0x1ef8) <= 0.0 {
                k3
            } else if rdf(p + 0x1efc) <= 0.0 {
                k3
            } else {
                0.0
            };
            set(rd32(this + 0xa90), cut0);
            let mut n = rd32(arg + 8).wrapping_add(mode);
            let lim = g32(CLAMP_VA);
            if (lim as i32) > (n as i32) {
                n = lim;
            }
            seti(rd32(this + 0xa90), n);
            set(rd32(this + 0xa94), add(rdf(arg + 0x20), t184));
            set(rd32(this + 0xa8c), add(rdf(arg + 0x10), e1));
            seti(rd32(this + 0xa8c), rd32(arg + 0x14));
            set(rd32(this + 0xa88), rdf(arg + 0x18));
            seti(rd32(this + 0xa88), rd32(arg + 0x1c));
            set(rd32(this + 0xa98), add(rdf(arg + 0x34), t164));
            set(rd32(this + 0xa9c), rdf(arg + 0x34));
            set(rd32(this + 0xaa0), t_rotor);
            let al3 = (test0() & 0xFF) as u8;
            if al3 != 0 && g32(MODE2_VA) == 4 {
                wr32(arg + 0x0c, MINUS100);
                wr32(arg + 0x04, MINUS100);
                let slot = this + 0xaa8;
                if rd32(slot) == 0 {
                    let mut f2 = [0u32; 16];
                    let f2p = f2.as_mut_ptr() as u32;
                    let mk: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                        lf_checker_rt::callee_addr(MK_CALLEE) as usize,
                    );
                    mk(f2p);
                    f2[0] = NINE;
                    f2[3] = p.wrapping_add(0xdb8);
                    f2[11] = 0;
                    f2[14] = 4;
                    let mk2: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(
                            lf_checker_rt::callee_addr(MK2_CALLEE) as usize,
                        );
                    mk2(this, 0x00e8_ff1c, slot, f2p, 0xFFFF_FFFF, 0, 0);
                }
                let obj = rd32(slot);
                if obj != 0 {
                    let uo: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        lf_checker_rt::callee_addr(USEOBJ_CALLEE) as usize,
                    );
                    uo(obj, 0);
                }
            } else {
                let obj2 = rd32(this + 0xaa8);
                if obj2 != 0 {
                    let r1: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        lf_checker_rt::callee_addr(REL1_CALLEE) as usize,
                    );
                    r1(obj2, 0x64);
                    let r2: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        lf_checker_rt::callee_addr(REL2_CALLEE) as usize,
                    );
                    r2(rd32(this + 0xaa8), 0);
                }
            }
            let v = rd32(this + 0xa90);
            let look: extern "cdecl" fn(u32, u32) -> u32 = core::mem::transmute(
                lf_checker_rt::callee_addr(LOOKUP_CALLEE) as usize,
            );
            let ra = look(0x00e8_ff34, 0);
            let s_esi = rd32(v);
            let ind: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(s_esi) as usize);
            let rb = ind(v, ra);
            if rb != 0 {
                wr32(rb, rd32(arg + 0x0c));
            }
            let rc = look(0x00e8_ff44, 0);
            let ind2: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(v)) as usize);
            let rd = ind2(v, rc);
            if rd != 0 {
                wr32(rd, rd32(arg + 0x04));
            }
            let fwd: extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
                lf_checker_rt::callee_addr(FWD_CALLEE) as usize,
            );
            let r = fwd(this, saved, arg);
            wr32(this + 0xad4, add(rdf(arg + 0x0c), cut0).to_bits());
            r
        }
    }
});
