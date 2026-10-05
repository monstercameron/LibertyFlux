// original: 0x00B40730 ped_task_register_slot (proposed)

/// Register an object into a fixed slot table and initialise the slot.
///
/// `obj` points at the object (a vtable pointer, a position triple, a
/// secondary triple, several flag words). `aux` points at a record whose
/// word at +0x40 contributes one flag bit.
///
/// Behaviour: three floats are range-checked against [-4000, 4000] (taken
/// from the target record when one is attached, else from the object
/// itself; an unordered comparison passes). A gate callee, a table-count
/// global (bailing when it already holds 512) and the range flag decide
/// between an early path (stamp 0xFFFE into a kind-dependent word, maybe
/// run the unlock callee, return 0) and the main body. The main body marks
/// a global, scans the table (stride 0x280) for the first empty slot and,
/// if full, stamps and returns 0 like the early path. Otherwise it attaches
/// a target through two callees when none is attached, initialises a
/// scratch area through a setup callee, copies eight target words and the
/// scratch block into the slot, runs an attach callee, and fills the slot
/// header (link to the object, flag bits from `aux`, the object kind and
/// byte fields, the bump pointer, table count incremented). For kind 4 it
/// stores the slot index, queries a float through a vtable slot callee and
/// sets flag bits from it (500.0 above-test), then resolves another vtable
/// slot and, unless null, measures through one more callee and sets bit 13
/// from a positive measurement with a clear object bit; kind 2 is similar
/// with different bits plus an equality bit from another object word; other
/// kinds skip the query. Always runs the unlock callee and returns 1.
///
/// Float order is trivial (comparisons only); all range and threshold tests
/// use ordered comparisons so NaN behaves as the original's comiss+ja/jbe.
///
/// Original: 0x00B40730 (cdecl, two stack words, returns 0/1 in al).
lf_checker_rt::export!(cdecl, rw_00B40730(obj: u32, aux: u32) -> u32 {
    unsafe {
        const GATE_CALLEE: u32 = 1;
        const UNLOCK_CALLEE: u32 = 2;
        const CTOR_CALLEE: u32 = 3;
        const INIT_CALLEE: u32 = 4;
        const SETUP_CALLEE: u32 = 5;
        const ATTACH_CALLEE: u32 = 6;
        const QUERY_CALLEE: u32 = 7;
        const VT_SLOT_CALLEE: u32 = 8;
        const VT_A0_CALLEE: u32 = 9;
        const MEASURE_CALLEE: u32 = 10;
        const G_MUTEX: u32 = 0x16C6710;
        const G_COUNT: u32 = 0x16B6AF8;
        const G_ACTIVE: u32 = 0x16B6B24;
        const G_TABLE: u32 = 0x16C7484;
        const G_BUMP: u32 = 0x16C7480;
        const SLOT_STRIDE: u32 = 0x280;
        const TABLE_CAP: u32 = 0x200;
        const LO: f32 = -4000.0;
        const HI: f32 = 4000.0;
        const QUERY_REF: f32 = 500.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        let kind_of = |o: u32| unsafe { (rd32(o.wrapping_add(0x28)) >> 6) & 0xF };
        let stamp = |o: u32| unsafe {
            let k = kind_of(o);
            if k == 4 {
                ((o.wrapping_add(0x220)) as *mut u16).write(0xFFFE);
            } else if k == 2 {
                ((o.wrapping_add(0x1074)) as *mut u16).write(0xFFFE);
            }
        };

        let attached = rd32(obj.wrapping_add(0x20));
        let src = if attached != 0 { attached.wrapping_add(0x30) } else { obj.wrapping_add(0x10) };
        let mut ok = true;
        let mut k = 0u32;
        while k < 3 {
            let f = rdf(src.wrapping_add(k.wrapping_mul(4)));
            if LO > f || f > HI {
                ok = false;
                break;
            }
            k += 1;
        }
        let gate = lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, lf_checker_rt::relocated(G_MUTEX));
        let denied = (gate & 0xFF) == 0;
        if lf_checker_rt::global::<u32>(G_COUNT).read() == TABLE_CAP || !ok || denied {
            stamp(obj);
            if !denied {
                lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, lf_checker_rt::relocated(G_MUTEX));
            }
            return 0;
        }

        (lf_checker_rt::global::<u8>(G_ACTIVE)).write(1);
        let table = lf_checker_rt::global::<u32>(G_TABLE).read();
        let mut ebp = 0u32;
        loop {
            if rd32(table.wrapping_add(ebp.wrapping_mul(SLOT_STRIDE))) == 0 {
                break;
            }
            ebp += 1;
            if ebp == TABLE_CAP {
                break;
            }
        }
        if ebp == TABLE_CAP {
            stamp(obj);
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, lf_checker_rt::relocated(G_MUTEX));
            return 0;
        }

        if rd32(obj.wrapping_add(0x20)) == 0 {
            lf_checker_rt::callee_thiscall!(CTOR_CALLEE, u32, obj);
            let p = rd32(obj.wrapping_add(0x20));
            lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, obj.wrapping_add(0x10), p);
        }
        let off = ebp.wrapping_mul(SLOT_STRIDE);
        let slot = table.wrapping_add(off);
        let scratch = table.wrapping_add(0x130).wrapping_add(off);
        lf_checker_rt::callee_thiscall!(SETUP_CALLEE, u32, scratch, obj);
        let target = rd32(obj.wrapping_add(0x20));
        wr32(slot.wrapping_add(0x160), rd32(target));
        wr32(slot.wrapping_add(0x164), rd32(target.wrapping_add(4)));
        wr32(slot.wrapping_add(0x168), rd32(target.wrapping_add(8)));
        wr32(slot.wrapping_add(0x16C), rd32(target.wrapping_add(0xC)));
        wr32(slot.wrapping_add(0x150), rd32(target.wrapping_add(0x20)));
        wr32(slot.wrapping_add(0x154), rd32(target.wrapping_add(0x24)));
        wr32(slot.wrapping_add(0x158), rd32(target.wrapping_add(0x28)));
        wr32(slot.wrapping_add(0x15C), rd32(target.wrapping_add(0x2C)));
        let mut i = 0u32;
        while i < 0x48 {
            wr32(slot.wrapping_add(0x10).wrapping_add(i.wrapping_mul(4)),
                rd32(scratch.wrapping_add(i.wrapping_mul(4))));
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(ATTACH_CALLEE, u32, slot);

        wr32(slot.wrapping_add(0x26C), rd32(slot.wrapping_add(0x26C)) & 0xFFFF_FFFE);
        let mut b2 = 0u32;
        if rd32(aux.wrapping_add(0x40)) & 0x40000 != 0 {
            if (rd32(obj.wrapping_add(0x24)) >> 2) & 1 == 0 {
                if rd32(obj.wrapping_add(0x28)) & 0x3C0 == 0x100 {
                    if ((obj.wrapping_add(0x22B)) as *const u8).read() != 3 {
                        b2 = 1;
                    }
                }
            }
        }
        let mut v = (b2 & 1) << 2;
        v |= rd32(slot.wrapping_add(0x26C)) & 0xFFFF_FC2B;
        v |= 0x22;
        wr32(slot.wrapping_add(0x26C), v);
        ((slot.wrapping_add(0x270)) as *mut u8).write(0);
        wr32(slot, obj);
        wr32(slot.wrapping_add(0x25C), 0);
        wr32(slot.wrapping_add(0x254), 0);
        wr32(slot.wrapping_add(0x258), 0);
        wr32(slot.wrapping_add(0x250), lf_checker_rt::global::<u32>(G_BUMP).read());
        let cnt = lf_checker_rt::global::<u32>(G_COUNT).read();
        lf_checker_rt::global::<u32>(G_COUNT).write(cnt.wrapping_add(1));
        lf_checker_rt::global::<u32>(G_BUMP)
            .write(lf_checker_rt::global::<u32>(G_TABLE).read().wrapping_add(off));

        let query_float = |o: u32| unsafe {
            let qo = lf_checker_rt::callee_thiscall!(QUERY_CALLEE, u32, o);
            let vt = rd32(qo);
            let fp = rd32(vt.wrapping_add(0x24));
            let f: extern "thiscall" fn(u32) -> f32 =
                unsafe { core::mem::transmute(fp as usize) };
            f(qo)
        };
        let kind = kind_of(obj);
        if kind == 4 {
            ((obj.wrapping_add(0x220)) as *mut u16).write(ebp as u16);
            let f = query_float(obj);
            let above = QUERY_REF > f;
            let mut g = rd32(slot.wrapping_add(0x26C)) & 0xFFFF_F3FF;
            g |= (if above { 1u32 } else { 0 }) << 10;
            wr32(slot.wrapping_add(0x26C), g);
            let vt2 = rd32(obj);
            let fp2 = rd32(vt2.wrapping_add(0xA0));
            let r: u32 = {
                let ff: extern "thiscall" fn(u32) -> u32 =
                    unsafe { core::mem::transmute(fp2 as usize) };
                ff(obj)
            };
            let mut b13 = 0u32;
            if r != 0 {
                let q = lf_checker_rt::callee_thiscall!(MEASURE_CALLEE, u32, r);
                let fq = rdf(q.wrapping_add(0x200));
                if fq > 0.0 {
                    if ((obj.wrapping_add(0x210)) as *const u8).read() & 0x40 == 0 {
                        b13 = 1;
                    }
                }
            }
            let mut t = b13 << 13;
            t ^= rd32(slot.wrapping_add(0x26C));
            t &= 0x2000;
            wr32(slot.wrapping_add(0x26C), rd32(slot.wrapping_add(0x26C)) ^ t);
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, lf_checker_rt::relocated(G_MUTEX));
            1
        } else if kind == 2 {
            ((obj.wrapping_add(0x1074)) as *mut u16).write(ebp as u16);
            let f = query_float(obj);
            let above = QUERY_REF > f;
            let mut g = rd32(slot.wrapping_add(0x26C)) & 0xFFFF_FBFF;
            g |= (((if above { 1u32 } else { 0 }) | 2) << 10);
            wr32(slot.wrapping_add(0x26C), g);
            let eq1 = if rd32(obj.wrapping_add(0x1304)) == 1 { 1u32 } else { 0 };
            let h = rd32(slot.wrapping_add(0x26C)) & 0xFFFF_CFFF | (eq1 << 12);
            wr32(slot.wrapping_add(0x26C), h);
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, lf_checker_rt::relocated(G_MUTEX));
            1
        } else {
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, lf_checker_rt::relocated(G_MUTEX));
            1
        }
    }
});
