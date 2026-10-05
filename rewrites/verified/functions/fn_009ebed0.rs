// original: 0x009EBED0 CPlayerPed::vf33

/// Player-ped state refresh gate.
///
/// Decides whether the ped may refresh this tick, runs the refresh chain
/// through the ped's own virtual table and a few helpers, then re-arms a
/// status word and returns the readiness probe result.
///
/// `thiscall`: `this` in ECX, no stack arguments. Returns a u32 in EAX:
/// 0 when any gate refuses, otherwise the readiness probe's value.
///
/// Behaviour in order:
/// 1. Gate A: call the data-table hook (slot in global data) with a global
///    dword argument. A nonzero answer passes; a zero answer falls back to
///    two global flag bytes (fail only when the first is clear, or the
///    first is set while the second is clear).
/// 2. Gate B: the Gate-A bit ORed with two more global bytes must be zero,
///    or a guard byte must be nonzero; otherwise return 0.
/// 3. Refresh: virtual slot `VT_REFRESH` on the ped's own table (argument:
///    the ped), then a direct helper on the returned pointer.
/// 4. Optional attachment step: a signed 16-bit index at `O_INDEX` selects
///    a global table entry; when the entry, its flag byte at `O_ENTRY_FLAG`
///    and the ped byte at `O_ATT_FLAG` are all nonzero, call virtual slot
///    `VT_ATTACH` on the object found via `O_OUTER` -> `O_INNER` (argument:
///    the ped).
/// 5. Gate C: three global dwords must read (not 1, equal pair, not
///    `MODE_REFUSE`) or return 0.
/// 6. Two direct probes on the word at `O_PROBE_OBJ` each clear bit
///    `BIT_BUSY` of the dword at `O_BITS` when they answer nonzero. When
///    the second answers zero and bit 1 is clear and byte `O_ATT_FLAG` is
///    clear and the float at `O_FVAL` differs from its constant, a further
///    probe runs; when that answers zero and bit `BIT_LOCK` of `O_STATUS28`
///    is clear, a direct helper runs on the sub-object at `O_SUB`.
/// 7. When bit 1 of byte `O_FLAG_F4` is set, call virtual slot `VT_SUB` on
///    the sub-object at `O_SUB`. Set bit `BIT_ARMED` of `O_STATUS29C`.
/// 8. Return the direct readiness probe on the ped; when it answers 0, call
///    virtual slot `VT_TAIL` on the ped first (result still 0).
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    unsafe { (a as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}

const G_HOOK_ARG: u32 = 0x017ACCD8;
const G_HOOK_SLOT: u32 = 0x00E733DC;
const G_FLAG_A: u32 = 0x0105B48F;
const G_FLAG_B: u32 = 0x017ED8D1;
const G_OR0: u32 = 0x01173590;
const G_OR1: u32 = 0x01173591;
const G_GUARD: u32 = 0x01160C39;
const G_STATE: u32 = 0x011F7060;
const G_PAIR_A: u32 = 0x012088B4;
const G_PAIR_B: u32 = 0x00F1C040;
const G_MODE: u32 = 0x01037720;
const G_TABLE: u32 = 0x01295CD8;
const G_FCONST: u32 = 0x00FE8628;

const VT_REFRESH: u32 = 0xD0;
const VT_TAIL: u32 = 0xCC;
const VT_ATTACH: u32 = 0x0C;
const VT_SUB: u32 = 0x04;

const O_INDEX: u32 = 0x2E;
const O_STATUS28: u32 = 0x28;
const O_FLAG_F4: u32 = 0xF4;
const O_ATT_FLAG: u32 = 0x219;
const O_PROBE_OBJ: u32 = 0x224;
const O_STATUS29C: u32 = 0x29C;
const O_SUB: u32 = 0xBB0;
const O_BITS: u32 = 0xBE0;
const O_FVAL: u32 = 0xBF8;
const O_OUTER: u32 = 0xE98;
const O_INNER: u32 = 0x58;
const O_ENTRY_FLAG: u32 = 0x8C;

const STATE_REFUSE: u32 = 1;
const MODE_REFUSE: u32 = 0x12;
const BIT_BUSY: u32 = 0x2;
const BIT_ARMED: u32 = 0x0100_0000;
const BIT_LOCK: u32 = 0x0080_0000;

unsafe fn gate_a() -> u8 {
    unsafe {
        let arg = rd32(lf_checker_rt::relocated(G_HOOK_ARG));
        let slot = rd32(lf_checker_rt::relocated(G_HOOK_SLOT));
        let hook: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if hook(arg) != 0 {
            return 1;
        }
        if rd8(lf_checker_rt::relocated(G_FLAG_A)) == 0 {
            return 0;
        }
        if rd8(lf_checker_rt::relocated(G_FLAG_B)) != 0 {
            return 1;
        }
        0
    }
}

unsafe fn refresh_chain(this: u32) {
    unsafe {
        let vt = rd32(this);
        let refresh: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_REFRESH)) as usize);
        let got = refresh(this, this);
        lf_checker_rt::callee_thiscall!(3, u32, got);
    }
}

unsafe fn maybe_attach(this: u32) {
    unsafe {
        let idx = (rd16(this.wrapping_add(O_INDEX)) as i16) as i32;
        let entry = rd32(
            lf_checker_rt::relocated(G_TABLE)
                .wrapping_add((idx as u32).wrapping_mul(4)),
        );
        if entry == 0 {
            return;
        }
        if rd8(entry.wrapping_add(O_ENTRY_FLAG)) == 0 {
            return;
        }
        if rd8(this.wrapping_add(O_ATT_FLAG)) == 0 {
            return;
        }
        let outer = rd32(this.wrapping_add(O_OUTER));
        let inner = rd32(outer.wrapping_add(O_INNER));
        let attach: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(inner).wrapping_add(VT_ATTACH)) as usize);
        attach(inner, this);
    }
}

unsafe fn probe_and_clear(this: u32, callee: u32) -> u8 {
    unsafe {
        let obj = rd32(this.wrapping_add(O_PROBE_OBJ));
        let ans: u32 = if callee == 5 {
            lf_checker_rt::callee_thiscall!(5, u32, obj)
        } else {
            lf_checker_rt::callee_thiscall!(6, u32, obj)
        };
        if (ans & 0xFF) != 0 {
            let bits = rd32(this.wrapping_add(O_BITS));
            wr32(this.wrapping_add(O_BITS), bits & !BIT_BUSY);
        }
        (ans & 0xFF) as u8
    }
}

unsafe fn maybe_deep_refresh(this: u32) {
    unsafe {
        let ans: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
        if (ans & 0xFF) != 0 {
            return;
        }
        if rd32(this.wrapping_add(O_STATUS28)) & BIT_LOCK != 0 {
            return;
        }
        lf_checker_rt::callee_thiscall!(8, u32, this.wrapping_add(O_SUB));
    }
}

lf_checker_rt::export!(thiscall, rw_009EBED0(this: u32) -> u32 {
    unsafe {
        let mut acc = gate_a();
        acc |= rd8(lf_checker_rt::relocated(G_OR0));
        acc |= rd8(lf_checker_rt::relocated(G_OR1));
        if acc != 0 && rd8(lf_checker_rt::relocated(G_GUARD)) == 0 {
            return 0;
        }
        refresh_chain(this);
        maybe_attach(this);
        if rd32(lf_checker_rt::relocated(G_STATE)) == STATE_REFUSE {
            return 0;
        }
        if rd32(lf_checker_rt::relocated(G_PAIR_A))
            != rd32(lf_checker_rt::relocated(G_PAIR_B))
        {
            return 0;
        }
        if rd32(lf_checker_rt::relocated(G_MODE)) == MODE_REFUSE {
            return 0;
        }
        probe_and_clear(this, 5);
        if probe_and_clear(this, 6) == 0 {
            let bit_set = (rd32(this.wrapping_add(O_BITS)) >> 1) & 1 != 0;
            if bit_set {
                maybe_deep_refresh(this);
            } else if rd8(this.wrapping_add(O_ATT_FLAG)) == 0 {
                let got = f32::from_bits(rd32(this.wrapping_add(O_FVAL)));
                let want = f32::from_bits(rd32(lf_checker_rt::relocated(G_FCONST)));
                if got != want {
                    maybe_deep_refresh(this);
                }
            }
        }
        if rd8(this.wrapping_add(O_FLAG_F4)) & 2 != 0 {
            let sub = this.wrapping_add(O_SUB);
            let vt = rd32(sub);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_SUB)) as usize);
            f(sub);
        }
        let s = rd32(this.wrapping_add(O_STATUS29C));
        wr32(this.wrapping_add(O_STATUS29C), s | BIT_ARMED);
        let ready: u32 = lf_checker_rt::callee_thiscall!(10, u32, this);
        if ready == 0 {
            let vt = rd32(this);
            let tail: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_TAIL)) as usize);
            tail(this);
        }
        ready
    }
});
