// original: 0x00650200 rage::ptxSprite::vf14
/// Particle-sprite render pass (vtable slot 14): selects a sprite table entry
/// for this object, validates the caller's sprite descriptor, runs an effect
/// probe on it, and dispatches to one of three worker routines Picked by flag
/// bits before releasing the pass.
///
/// Behaviour: publish the table index from `this+0x17c` and the selected
/// table word to their globals, then bail out returning the descriptor index
/// when the descriptor slot is empty. Otherwise notify the subsystem
/// (callees 1-3); when the context object at `a1+0x268` exists, hash its key
/// (callee 4, Joaat) and run the context's probe slot (callee 5) over two
/// stacks of copied descriptor floats, bailing out with the probe result when
/// it reports false. Then run the geometry fixup (callee 6), the conditional
/// pre-pass (callee 7, flag bit 9), mirror flag bits into the state object at
/// `this+0x684`, run the two state gates (callees 8-9), and, unless gate 9
/// vetoes, dispatch on flag bits 11/31 to one of the three workers
/// (callees 10-12), run the conditional post hook (callee 13), clear three
/// status globals and run the teardown gate (callee 14). Finally release the
/// context through its slot (callee 15), write back the saved status global,
/// re-arm the subsystem (callee 16) and notify again (callee 17), returning
/// the last notify result.
///
/// Two deliberate mirrors of original quirks: the early-out returns the raw
/// descriptor index byte (whatever `eax` still holds), and gate 9's veto and
/// the probe-false path share the same epilogue with their own `eax`.
/// Callee 6 also takes its two register inputs from the stack arguments, but
/// registers cannot be forwarded through a caller-cleanup call from Rust, so
/// only its stack arguments are compared (both derive from `a2`, whose value
/// is pinned by callees 7 and 10-12 anyway).
lf_checker_rt::export!(thiscall, rb112_fn1(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    const CAL_SETUP: u32 = 1; // subsystem setup (thiscall/0, result ignored)
    const CAL_ARM: u32 = 2; // arm helper at this+0x100 (thiscall/0)
    const CAL_NOTIFY: u32 = 3; // subsystem notify (thiscall/1)
    const CAL_HASH: u32 = 4; // key hash, Joaat (cdecl/2)
    const CAL_PROBE: u32 = 5; // context probe slot at vtable+0xc (thiscall/3)
    const CAL_FIXUP: u32 = 6; // geometry fixup (cdecl/2, regs not compared)
    const CAL_PRE: u32 = 7; // conditional pre-pass (thiscall/1)
    const CAL_GATE_A: u32 = 8; // state gate A (thiscall/0, result ignored)
    const CAL_GATE_B: u32 = 9; // state gate B (thiscall/0, zero vetoes)
    const CAL_WORKER_0: u32 = 10; // worker for selector 0 (thiscall/3)
    const CAL_WORKER_2: u32 = 11; // worker for selector 2 (thiscall/3)
    const CAL_WORKER_1: u32 = 12; // worker for selector 1 (thiscall/3)
    const CAL_POST: u32 = 13; // conditional post hook (cdecl/0)
    const CAL_TEARDOWN: u32 = 14; // teardown gate (thiscall/0)
    const CAL_RELEASE: u32 = 15; // context release slot at vtable+0x10
    const CAL_REARM: u32 = 16; // re-arm helper (thiscall/0)
    const CAL_NOTIFY2: u32 = 17; // second notify, result is returned

    const G_TABLE_WORD: u32 = 0x01107878;
    const G_TABLE_BASE: u32 = 0x01107890;
    const G_TABLE_SEL: u32 = 0x011078B4;
    const G_STATUS: u32 = 0x0106B308;
    const G_REARM_PTR: u32 = 0x0110EE10;
    const G_NOTIFY_ARG: u32 = 0x01110090;
    const G_CTX: u32 = 0x017F583C;
    const G_FLAG_BYTE: u32 = 0x017ED94B;
    const G_CLEAR_A: u32 = 0x017F58E4;
    const G_CLEAR_B: u32 = 0x017F58E0;
    const G_CLEAR_C: u32 = 0x017F59D4;
    const G_DESC_PTR: u32 = 0x01BB6678;

    unsafe {
        let sel = (this.wrapping_add(0x17C) as *const u32).read();
        (lf_checker_rt::global::<u32>(G_TABLE_SEL) as *mut u32).write(sel);
        let word = (lf_checker_rt::relocated(G_TABLE_BASE).wrapping_add(sel.wrapping_mul(4))
            as *const u32)
            .read();
        (lf_checker_rt::global::<u32>(G_TABLE_WORD) as *mut u32).write(word);

        let desc_table = (lf_checker_rt::global::<u32>(G_DESC_PTR) as *const u32).read();
        let idx = ((desc_table.wrapping_add(2)) as *const u8).read() as u32;
        let slot = (a2.wrapping_add(idx.wrapping_mul(4)).wrapping_add(0x14) as *const u32).read();
        if slot == 0 {
            return idx;
        }

        lf_checker_rt::callee_thiscall!(CAL_SETUP, u32, this);
        lf_checker_rt::callee_thiscall!(CAL_ARM, u32, this.wrapping_add(0x100));
        let ctx = (lf_checker_rt::global::<u32>(G_CTX) as *const u32).read();
        lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, ctx, lf_checker_rt::relocated(G_NOTIFY_ARG));

        let obj = ((a1.wrapping_add(0x268)) as *const u32).read();
        if obj != 0 {
            let f60 = ((a2.wrapping_add(0x60)) as *const u32).read();
            let f64 = ((a2.wrapping_add(0x64)) as *const u32).read();
            let f68 = ((a2.wrapping_add(0x68)) as *const u32).read();
            let f70 = ((a2.wrapping_add(0x70)) as *const u32).read();
            let f74 = ((a2.wrapping_add(0x74)) as *const u32).read();
            let f78 = ((a2.wrapping_add(0x78)) as *const u32).read();
            let key = ((this.wrapping_add(0x67C)) as *const u32).read();
            let h = lf_checker_rt::callee_cdecl!(CAL_HASH, u32, key, 0);
            let vt = (obj as *const u32).read();
            let probe_addr = ((vt.wrapping_add(0xC)) as *const u32).read();
            let probe: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(probe_addr as usize);
            let mut first = [f60, f64, f68];
            let mut second = [f70, f74, f78];
            let r = probe(obj, first.as_mut_ptr() as u32, second.as_mut_ptr() as u32, h);
            if r & 0xFF == 0 {
                return r;
            }
        }

        lf_checker_rt::callee_cdecl!(
            CAL_FIXUP,
            u32,
            a2.wrapping_add(0x60),
            a2.wrapping_add(0x70)
        );
        let saved = (lf_checker_rt::global::<u32>(G_STATUS) as *const u32).read();
        let flags = ((this.wrapping_add(0x11C)) as *const u32).read();
        if (flags >> 9) & 1 != 0 {
            lf_checker_rt::callee_thiscall!(CAL_PRE, u32, this, a2);
        }

        let state = ((this.wrapping_add(0x684)) as *const u32).read();
        if state != 0 {
            let v = ((!((flags >> 11) & 1)) & 1) as u8;
            ((state.wrapping_add(0xA5)) as *mut u8).write(v);
        }
        let state2 = ((this.wrapping_add(0x684)) as *const u32).read();
        if state2 != 0 {
            let is_neg = ((this.wrapping_add(0x104)) as *const u32).read() == 0xFFFF_FFFF;
            ((state2.wrapping_add(0xA4)) as *mut u8).write(is_neg as u8);
        }

        lf_checker_rt::callee_thiscall!(CAL_GATE_A, u32, this.wrapping_add(0x678));
        let gate = lf_checker_rt::callee_thiscall!(CAL_GATE_B, u32, this.wrapping_add(0x678));
        if gate != 0 {
            let sel2 = if (flags >> 31) & 1 != 0 {
                2
            } else if (flags >> 11) & 1 != 0 {
                1
            } else {
                0
            };
            if sel2 == 0 {
                lf_checker_rt::callee_thiscall!(CAL_WORKER_0, u32, this, a1, a2, a3);
            } else if sel2 == 2 {
                lf_checker_rt::callee_thiscall!(CAL_WORKER_2, u32, this, a1, a2, a3);
            } else {
                lf_checker_rt::callee_thiscall!(CAL_WORKER_1, u32, this, a1, a2, a3);
            }
            let hook = (lf_checker_rt::global::<u8>(G_FLAG_BYTE) as *const u8).read();
            if hook != 0 {
                lf_checker_rt::callee_cdecl!(CAL_POST, u32,);
            }
            (lf_checker_rt::global::<u32>(G_CLEAR_A) as *mut u32).write(0);
            let st3 = ((this.wrapping_add(0x684)) as *const u32).read();
            (lf_checker_rt::global::<u32>(G_CLEAR_B) as *mut u32).write(0);
            let down = ((st3.wrapping_add(4)) as *const u32).read().wrapping_add(0x14);
            lf_checker_rt::callee_thiscall!(CAL_TEARDOWN, u32, down);
            (lf_checker_rt::global::<u32>(G_CLEAR_C) as *mut u32).write(0);
        }

        let obj2 = ((a1.wrapping_add(0x268)) as *const u32).read();
        if obj2 != 0 {
            let vt2 = (obj2 as *const u32).read();
            let rel_addr = ((vt2.wrapping_add(0x10)) as *const u32).read();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rel_addr as usize);
            release(obj2);
        }
        (lf_checker_rt::global::<u32>(G_STATUS) as *mut u32).write(saved);
        lf_checker_rt::callee_thiscall!(CAL_REARM, u32, lf_checker_rt::relocated(G_REARM_PTR));
        let ctx2 = (lf_checker_rt::global::<u32>(G_CTX) as *const u32).read();
        lf_checker_rt::callee_thiscall!(
            CAL_NOTIFY2,
            u32,
            ctx2,
            lf_checker_rt::relocated(G_NOTIFY_ARG)
        )
    }
});
