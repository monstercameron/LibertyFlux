// original: 0x00CA7CA0 CEventHandler::vf32

/// Build this handler's reaction for an event kind: a dispatch over the
/// event's kind word with one maker leg per known kind and a forward to the
/// base handler for the rest.
///
/// `this` is the handler (its ped at `+0x04`, the built reaction stored at
/// `+0x0c`). `a1` is the event: the kind word at `+0x10` selects the leg and
/// the subject at `+0x18` must be non-null (a null subject returns whatever
/// the caller left behind, so the contract always provides one). `a2`/`a3`
/// are only forwarded, untouched, on the default leg.
///
/// Each known-kind leg fetches the factory object and, unless it is missing
/// (which stores 0, except on three legs where the missing factory faults on
/// a null write first), builds through its own maker with constant arguments
/// and stores the result. The legs: `0xC8` stores 0; `0x19C` and `0x19D`
/// build from nothing and from the subject; `0x19F` first probes the handler
/// ped (a nonzero low byte aborts, returning that answer); `0x1AB` builds
/// from (-1, count, 0); `0x2E2` needs the ped link at `+0xB30` and ready bit
/// 2 at `+0x26C`, then builds from the link; `0x38D` splits on the
/// subject-descriptor flags at `+0x28` masked with `0x3C0` equalling `0xC0`
/// (maker, or-bit 3 into the answer at `+0x60`, then a two-word call whose
/// answer is returned unstored) or not (a seven-word maker, flag byte at
/// `+0x39`); `0x39F`, `0x3A5`, `0x3AC` and `0x3FC` build through float-arg
/// makers (the `0x3AC` leg also stamps its answer); `0x3FE` runs a long gate
/// chain (ready bit, link, identity mismatch at link `+0xF50`, a keyed call,
/// an indexed-table call whose answer word at `+8` must be 1, a probe with a
/// zero low byte) then takes a float from a second table call, adds 5.0 to a
/// double-returning call's result, keeps the larger of the two as the
/// radius, and builds from (subject, radius bits) when the radius exceeds
/// the subject-to-ped distance, storing 0 otherwise; `0x4BC` builds only
/// when the squared distance is below 7.5 squared and a game counter is
/// below its clamp. Any other kind forwards (kind, a1, a2, a3) to slot
/// `0x138` of the handler's own table and returns its answer unstored.
///
/// Original: 0x00CA7CA0 (thiscall, ecx = handler, three stack words; signed
/// kind dispatch. The `0x3FE` leg overwrites its own incoming first and
/// third argument slots with scratch, so the stack check is off.)
lf_checker_rt::export!(thiscall, rw_00ca7ca0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const A1_KIND: u32 = 0x10;
        const A1_SUBJ: u32 = 0x18;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        // The subject is read first so a null event faults here, as the
        // original does. (A null subject with a live event would return the
        // caller's leftover register: unreplicable, never fed.)
        let edi = rd32(a1.wrapping_add(A1_SUBJ));
        let kind = rd32(a1.wrapping_add(A1_KIND)) as i32;
        if kind == 0x38d {
            return ca7ca0_leg38d(this, edi);
        }
        if kind > 0x38d {
            return ca7ca0_high(this, a1, a2, a3, edi, kind);
        }
        if kind == 0x19f {
            return ca7ca0_leg19f(this, edi);
        }
        if kind > 0x19f {
            return ca7ca0_mid(this, a1, a2, a3, edi, kind);
        }
        if kind == 0x0c8 {
            wr32(this.wrapping_add(0x0c), 0);
            return 0;
        }
        if kind == 0x19c {
            return ca7ca0_leg19c(this);
        }
        if kind == 0x19d {
            return ca7ca0_leg19d(this, edi);
        }
        ca7ca0_default(this, a1, a2, a3, edi, kind as u32)
    }
});

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
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

/// Factory fetch of rw_00ca7ca0.
unsafe fn ca7ca0_factory() -> u32 {
    unsafe {
        const G_FACTORY: u32 = 0x0167_E2A0;
        lf_checker_rt::callee_thiscall!(
            1, u32, rd32(lf_checker_rt::relocated(G_FACTORY))
        )
    }
}

/// Store-and-return of rw_00ca7ca0.
unsafe fn ca7ca0_store(this: u32, v: u32) -> u32 {
    unsafe {
        wr32(this.wrapping_add(0x0c), v);
        v
    }
}

/// Missing-factory exit of rw_00ca7ca0.
unsafe fn ca7ca0_nofac(this: u32) -> u32 {
    unsafe { ca7ca0_store(this, 0) }
}

/// Default leg: forward to slot 0x138 of the handler's own table.
unsafe fn ca7ca0_default(this: u32, a1: u32, a2: u32, a3: u32, _edi: u32, kind: u32) -> u32 {
    unsafe {
        const BASE_SLOT: u32 = 0x138;
        let vt = rd32(this);
        let base: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(BASE_SLOT)) as usize);
        base(this, kind, a1, a2, a3)
    }
}

unsafe fn ca7ca0_leg19c(this: u32) -> u32 {
    unsafe {
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(3, u32, fac);
        ca7ca0_store(this, made)
    }
}

unsafe fn ca7ca0_leg19d(this: u32, edi: u32) -> u32 {
    unsafe {
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(2, u32, fac, edi);
        ca7ca0_store(this, made)
    }
}

unsafe fn ca7ca0_leg19f(this: u32, edi: u32) -> u32 {
    unsafe {
        const H_PED: u32 = 0x04;
        const COUNT: u32 = 0x1388;
        let hped = rd32(this.wrapping_add(H_PED));
        let probe: u32 = lf_checker_rt::callee_thiscall!(4, u32, hped);
        if probe & 0xFF != 0 {
            return probe;
        }
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let made: u32 =
            lf_checker_rt::callee_thiscall!(5, u32, fac, 0, COUNT, 0xFFFF_FFFF);
        let _ = edi;
        ca7ca0_store(this, made)
    }
}

unsafe fn ca7ca0_mid(this: u32, a1: u32, a2: u32, a3: u32, edi: u32, kind: i32) -> u32 {
    unsafe {
        if kind == 0x1ab {
            return ca7ca0_leg1ab(this);
        }
        if kind != 0x2e2 {
            return ca7ca0_default(this, a1, a2, a3, edi, kind as u32);
        }
        ca7ca0_leg2e2(this)
    }
}

unsafe fn ca7ca0_leg1ab(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x0098_967F;
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let made: u32 =
            lf_checker_rt::callee_thiscall!(5, u32, fac, 0, COUNT, 0xFFFF_FFFF);
        ca7ca0_store(this, made)
    }
}

unsafe fn ca7ca0_leg2e2(this: u32) -> u32 {
    unsafe {
        const H_PED: u32 = 0x04;
        const PED_LINK: u32 = 0xB30;
        const PED_READY: u32 = 0x26C;
        const READY_BIT: u8 = 0x04;
        let hped = rd32(this.wrapping_add(H_PED));
        let link = rd32(hped.wrapping_add(PED_LINK));
        if link == 0 {
            return 0;
        }
        if (hped.wrapping_add(PED_READY) as *const u8).read() & READY_BIT == 0 {
            return hped;
        }
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(6, u32, fac, link, 0, 0, 0);
        ca7ca0_store(this, made)
    }
}

unsafe fn ca7ca0_leg38d(this: u32, edi: u32) -> u32 {
    unsafe {
        const DESC_FLAGS: u32 = 0x28;
        const FLAGS_MASK: u32 = 0x3C0;
        const FLAGS_WANT: u32 = 0xC0;
        const H_PED: u32 = 0x04;
        const H_REACTION: u32 = 0x0c;
        const PED_LINK: u32 = 0x224;
        const GATE_OFF: u32 = 0x44;
        const F40: u32 = 0x00EE_F940;
        const F44: u32 = 0x00EE_F944;
        const F48: u32 = 0x00EE_F948;
        const F4C: u32 = 0x00EE_F94C;
        if rd32(edi.wrapping_add(DESC_FLAGS)) & FLAGS_MASK == FLAGS_WANT {
            let fac = ca7ca0_factory();
            let made = if fac == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(7, u32, fac, edi, 0)
            };
            wr32(this.wrapping_add(H_REACTION), made);
            // Faults when the factory was missing, like the original.
            wr32(made.wrapping_add(0x60), rd32(made.wrapping_add(0x60)) | 8);
            let hped = rd32(this.wrapping_add(H_PED));
            let mid = rd32(hped.wrapping_add(PED_LINK));
            return lf_checker_rt::callee_thiscall!(
                8, u32, mid.wrapping_add(GATE_OFF), edi, 1
            );
        }
        let fac = ca7ca0_factory();
        if fac == 0 {
            wr32(this.wrapping_add(H_REACTION), 0);
            // Faults, like the original.
            wr8(0x39, 1);
            return 0;
        }
        let f40 = rd32(lf_checker_rt::relocated(F40));
        let f44 = rd32(lf_checker_rt::relocated(F44));
        let f48 = rd32(lf_checker_rt::relocated(F48));
        let f4c = rd32(lf_checker_rt::relocated(F4C));
        let made: u32 =
            lf_checker_rt::callee_thiscall!(9, u32, fac, edi, 1, f40, f44, f48, f4c, 0);
        wr32(this.wrapping_add(H_REACTION), made);
        wr8(made.wrapping_add(0x39), 1);
        made
    }
}

unsafe fn ca7ca0_high(this: u32, a1: u32, a2: u32, a3: u32, edi: u32, kind: i32) -> u32 {
    unsafe {
        if kind == 0x3fc {
            return ca7ca0_leg3fc(this, edi);
        }
        if kind > 0x3fc {
            if kind == 0x3fe {
                return ca7ca0_leg3fe(this, edi);
            }
            if kind == 0x4bc {
                return ca7ca0_leg4bc(this, edi);
            }
            return ca7ca0_default(this, a1, a2, a3, edi, kind as u32);
        }
        if kind == 0x39f {
            return ca7ca0_leg39f(this, edi);
        }
        if kind == 0x3a5 {
            return ca7ca0_leg3a5(this, edi);
        }
        if kind == 0x3ac {
            return ca7ca0_leg3ac(this, edi);
        }
        ca7ca0_default(this, a1, a2, a3, edi, kind as u32)
    }
}

unsafe fn ca7ca0_leg39f(this: u32, edi: u32) -> u32 {
    unsafe {
        const F40: u32 = 0x00EE_F940;
        const F44: u32 = 0x00EE_F944;
        const F48: u32 = 0x00EE_F948;
        const F4C: u32 = 0x00EE_F94C;
        const FBC: u32 = 0x00EE_F9BC;
        const FC0: u32 = 0x00EE_F9C0;
        const FC4: u32 = 0x00EE_F9C4;
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let r = lf_checker_rt::relocated;
        let made: u32 = lf_checker_rt::callee_thiscall!(
            11, u32, fac, edi, 1,
            rd32(r(F40)), rd32(r(F44)), rd32(r(FBC)),
            rd32(r(FC0)), rd32(r(FC4)), rd32(r(F48)), rd32(r(F4C))
        );
        ca7ca0_store(this, made)
    }
}

unsafe fn ca7ca0_leg3a5(this: u32, edi: u32) -> u32 {
    unsafe {
        const TEN: u32 = 0x4120_0000;
        const FIVE: u32 = 0x40A0_0000;
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(10, u32, fac, edi, TEN, FIVE);
        ca7ca0_store(this, made)
    }
}

unsafe fn ca7ca0_leg3ac(this: u32, edi: u32) -> u32 {
    unsafe {
        const H_REACTION: u32 = 0x0c;
        const F40: u32 = 0x00EE_F940;
        const F44: u32 = 0x00EE_F944;
        const F48: u32 = 0x00EE_F948;
        const F4C: u32 = 0x00EE_F94C;
        let fac = ca7ca0_factory();
        if fac == 0 {
            wr32(this.wrapping_add(H_REACTION), 0);
            // Faults, like the original.
            wr32(0x44, 2);
            return 0;
        }
        let r = lf_checker_rt::relocated;
        let made: u32 = lf_checker_rt::callee_thiscall!(
            9, u32, fac, edi, 0,
            rd32(r(F40)), rd32(r(F44)), rd32(r(F48)), rd32(r(F4C)), 0
        );
        wr32(this.wrapping_add(H_REACTION), made);
        let back = rd32(this.wrapping_add(H_REACTION));
        wr32(back.wrapping_add(0x44), 2);
        wr8(back.wrapping_add(0x39), 1);
        back
    }
}

unsafe fn ca7ca0_leg3fc(this: u32, edi: u32) -> u32 {
    unsafe {
        const ONE: u32 = 0x3F80_0000;
        const TEN: u32 = 0x4120_0000;
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(
            12, u32, fac, 2, edi, 0, TEN, 0, 1, 1, ONE
        );
        ca7ca0_store(this, made)
    }
}

unsafe fn ca7ca0_leg4bc(this: u32, edi: u32) -> u32 {
    unsafe {
        const H_PED: u32 = 0x04;
        const POS: u32 = 0x20;
        const LIM: u32 = 0x0105_7474;
        const CTR: u32 = 0x017A_6538;
        const CLAMP: u32 = 0x0105_7488;
        let hped = rd32(this.wrapping_add(H_PED));
        let ep = rd32(edi.wrapping_add(POS));
        let hp = rd32(hped.wrapping_add(POS));
        let dx = sub(rdf(hp.wrapping_add(0x30)), rdf(ep.wrapping_add(0x30)));
        let dy = sub(rdf(hp.wrapping_add(0x34)), rdf(ep.wrapping_add(0x34)));
        let dz = sub(rdf(hp.wrapping_add(0x38)), rdf(ep.wrapping_add(0x38)));
        let sq = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let lim = rdf(lf_checker_rt::relocated(LIM));
        let lim2 = mul(lim, lim);
        if !(lim2 > sq) {
            return hp;
        }
        let ctr = rd32(lf_checker_rt::relocated(CTR)) as i32;
        let clamp = rd32(lf_checker_rt::relocated(CLAMP)) as i32;
        if ctr >= clamp {
            return ctr as u32;
        }
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(13, u32, fac, edi);
        ca7ca0_store(this, made)
    }
}

unsafe fn ca7ca0_leg3fe(this: u32, edi: u32) -> u32 {
    unsafe {
        const H_PED: u32 = 0x04;
        const PED_READY: u32 = 0x26C;
        const READY_BIT: u8 = 0x04;
        const PED_LINK: u32 = 0xB30;
        const LINK_BACK: u32 = 0xF50;
        const PED_MID: u32 = 0x224;
        const GATE_OFF: u32 = 0x44;
        const KEY: u32 = 0x2C5;
        const TAB: u32 = 0x2B0;
        const FIVE: u32 = 0x00FE_8AD8;
        const POS: u32 = 0x20;
        const MOOD: u32 = 0x370;
        let hped = rd32(this.wrapping_add(H_PED));
        if (hped.wrapping_add(PED_READY) as *const u8).read() & READY_BIT == 0 {
            // eax still holds the dispatch residue kind-0x39F-6-7 = 0x52.
            return 0x52;
        }
        let link = rd32(hped.wrapping_add(PED_LINK));
        if link == 0 {
            return 0;
        }
        if hped == rd32(link.wrapping_add(LINK_BACK)) {
            return link;
        }
        let mid = rd32(hped.wrapping_add(PED_MID));
        let keyed: u32 = lf_checker_rt::callee_thiscall!(15, u32, mid.wrapping_add(GATE_OFF), KEY);
        if keyed == 0 {
            return 0;
        }
        let idx = rd32(hped.wrapping_add(TAB));
        let slot = rd32(
            hped
                .wrapping_add(idx.wrapping_add(3).wrapping_mul(3).wrapping_mul(4))
                .wrapping_add(TAB),
        );
        let first: u32 = lf_checker_rt::callee_cdecl!(16, u32, slot);
        if rd32(first.wrapping_add(8)) != 1 {
            return first;
        }
        let probe: u32 = lf_checker_rt::callee_thiscall!(14, u32, mid, edi);
        if probe & 0xFF != 0 {
            return probe;
        }
        let idx2 = rd32(hped.wrapping_add(TAB));
        let slot2 = rd32(
            hped
                .wrapping_add(idx2.wrapping_add(3).wrapping_mul(3).wrapping_mul(4))
                .wrapping_add(TAB),
        );
        let second: u32 = lf_checker_rt::callee_cdecl!(16, u32, slot2);
        let saved = rdf(second.wrapping_add(0x14));
        let d: f64 = lf_checker_rt::callee_thiscall!(17, f64, mid);
        let fresh = add(d as f32, rdf(lf_checker_rt::relocated(FIVE)));
        let radius = if saved > fresh { saved } else { fresh };
        let ep = rd32(edi.wrapping_add(POS));
        let hp = rd32(hped.wrapping_add(POS));
        let dx = sub(rdf(ep.wrapping_add(0x30)), rdf(hp.wrapping_add(0x30)));
        let dy = sub(rdf(ep.wrapping_add(0x34)), rdf(hp.wrapping_add(0x34)));
        let dz = sub(rdf(ep.wrapping_add(0x38)), rdf(hp.wrapping_add(0x38)));
        let dist = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
        if !(radius > dist) {
            wr32(this.wrapping_add(0x0c), 0);
            // eax still holds the double call's low word, as the stub left it.
            return d.to_bits() as u32;
        }
        let mood = (hped.wrapping_add(MOOD) as *const u8).read() as u32;
        let fac = ca7ca0_factory();
        if fac == 0 {
            return ca7ca0_nofac(this);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(
            18, u32, fac, edi, 0, radius.to_bits(), mood, 8, 0
        );
        ca7ca0_store(this, made)
    }
}
