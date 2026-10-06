// original: 0x008D3580 aabb_gate_or_reseed
/// Gates a reseed/submit sequence on two bounding-box tests over freshly
/// filled sample points, then drives vtable calls and a length exit.
///
/// `this` is the owning object; word 0x598 links the controlled object,
/// whose word 0x20 links a position record holding three floats at 0x30.
/// A gate callee answers first: a zero low byte skips everything and runs
/// the exit block. Otherwise a fill callee writes two three-float sample
/// blocks, and two radii are selected from globals: defaults, or alternates
/// when a flag bit is set on the controlled object and its linked record
/// exists (a second alternate pair when that record's kind word is not 4).
/// The first box test (radius one) compares each sample coordinate widened
/// by the radius against the position; any strictly-above result continues
/// to the flag block, while all-below-or-equal clears a flag byte and runs
/// the exit block. The flag block reseeds once (open plus a 17-argument
/// submit carrying the open answer and a global word) unless the flag byte
/// is already set. The second box test (radius two) uses above-or-equal on
/// the lower sides: any outside result dispatches through vtables (a
/// five-argument call on the linked record's table, or a one-argument call
/// on the controlled object's table), while a fully-inside result runs the
/// exit block directly. The exit block compares the position's squared
/// length against a global limit; above the limit, a float-returning table
/// call minus a subtrahend becomes the argument of a second table call
/// whose answer is returned, otherwise the position link is returned.
///
/// The subtrahend is a stack slot, not the global: the float call pops its
/// argument, so its frame sits one word higher than the slot the global
/// was stored to, and the subtraction reads the neighbouring slot instead.
/// That slot holds the first sample word when the fill ran, or
/// uninitialized scratch (defined as 0 by the contract) when the gate
/// failed; after the one-argument table path, whose extra push shifts the
/// frame by a word, it holds the stored global after all.
///
/// Every ordered comparison mirrors one hardware float-compare-plus-branch
/// exactly, using only greater-than shapes for above-exits so unordered
/// (NaN) inputs take the same fall-through side the hardware takes.
lf_checker_rt::export!(thiscall, aq55_fn1(this: u32) -> u32 {
    unsafe { run_8d3580(this, false) }
});
lf_checker_rt::export!(thiscall, aq55_fn1m(this: u32) -> u32 {
    unsafe { run_8d3580(this, true) }
});

/// Shared body; `mutant` inverts the kind-word select (see below).
unsafe fn run_8d3580(this: u32, mutant: bool) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_GATE: u32 = 1; // gate (thiscall/0, low byte decides)
    const CAL_FILL: u32 = 2; // sample fill (cdecl/2: two out-blocks)
    const CAL_OPEN: u32 = 3; // reseed open (thiscall/1: key)
    const CAL_SUBMIT: u32 = 4; // reseed submit (cdecl/17)

    // Object layout.
    const OFF_CTL: u32 = 0x598; // controlled-object link
    const OFF_POS: u32 = 0x20; // position-record link
    const OFF_FLAGB: u32 = 0x26C; // flag byte tested for bit 2
    const FLAG_BIT: u32 = 4;
    const OFF_LINK: u32 = 0xB30; // linked-record link
    const OFF_KIND: u32 = 0x1304; // linked-record kind word
    const KIND_WANT: u32 = 4;
    const OFF_SUB: u32 = 0x6C; // sub-record link
    const OFF_SUBB: u32 = 0x0E; // sub-record flag byte
    const SLOT_CTL_A: u32 = 0xF4; // controlled table: single-arg slot
    const SLOT_CTL_F: u32 = 0xFC; // controlled table: float-result slot
    const SLOT_LINK: u32 = 0x180; // linked table: five-arg slot

    // Globals (file VAs; resolved through the worker's image base).
    const G_R1: u32 = 0x00FE8BB0; // radius one default
    const G_R2: u32 = 0x00FE8B38; // radius two default
    const G_R1EQ: u32 = 0x00FE8C2C; // radius one when kind matches
    const G_R1NE: u32 = 0x00FE8BF4; // radius one when kind differs
    const G_R2NE: u32 = 0x00FE8B68; // radius two when kind differs
    const G_SUB: u32 = 0x011735BC; // exit-block subtrahend
    const G_LIM: u32 = 0x00E80DC0; // exit-block squared-length limit
    const G_FLAG: u32 = 0x011736C0; // reseed flag byte
    const G_OBJ: u32 = 0x0116BFF0; // reseed-open target object
    const G_KEY: u32 = 0x00E80D60; // reseed-open key
    const G_WORD: u32 = 0x0116C24C; // submit argument word

    #[inline(always)]
    unsafe fn load(base: u32, off: u32) -> u32 {
        *((base.wrapping_add(off)) as *const u32)
    }
    #[inline(always)]
    unsafe fn fload(base: u32, off: u32) -> f32 {
        *((base.wrapping_add(off)) as *const f32)
    }
    /// Call a planted table slot exactly like the original: load the slot
    /// and call through it. Both sides land on the same stub.
    #[inline(always)]
    unsafe fn vcall5(object: u32, slot: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
        let table = *(object as *const u32);
        let target = *((table.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object, a0, a1, a2, a3, a4)
    }
    #[inline(always)]
    unsafe fn vcall1(object: u32, slot: u32, a0: u32) -> u32 {
        let table = *(object as *const u32);
        let target = *((table.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(target as usize);
        f(object, a0)
    }
    #[inline(always)]
    unsafe fn vcall1f(object: u32, slot: u32, a0: u32) -> f32 {
        let table = *(object as *const u32);
        let target = *((table.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> f32 = core::mem::transmute(target as usize);
        f(object, a0)
    }

    unsafe {
        let gate = lf_checker_rt::callee_thiscall!(CAL_GATE, u32, this);
        if (gate & 0xFF) == 0 {
            *(lf_checker_rt::global::<u8>(G_FLAG)) = 0;
            // No fill ran: the subtrahend slot holds scratch, defined 0.
            return exit_block(this, 0.0);
        }
        let mut sa = [0u32; 3];
        let mut sb = [0u32; 3];
        lf_checker_rt::callee_cdecl!(
            CAL_FILL, u32,
            sa.as_ptr() as u32,
            sb.as_ptr() as u32
        );
        // Volatile: the words were written by the stub through the raw
        // pointers above, and plain reads can be folded to the
        // initializers (observed: a probe returned 0 until the read was
        // made volatile, then the scripted word).
        let ax = f32::from_bits(core::ptr::read_volatile(sa.as_ptr()));
        let ay = f32::from_bits(core::ptr::read_volatile(sa.as_ptr().add(1)));
        let az = f32::from_bits(core::ptr::read_volatile(sa.as_ptr().add(2)));
        let bx = f32::from_bits(core::ptr::read_volatile(sb.as_ptr()));
        let by = f32::from_bits(core::ptr::read_volatile(sb.as_ptr().add(1)));
        let bz = f32::from_bits(core::ptr::read_volatile(sb.as_ptr().add(2)));
        let ctl = load(this, OFF_CTL);
        let r1d = *(lf_checker_rt::global::<f32>(G_R1));
        let r2d = *(lf_checker_rt::global::<f32>(G_R2));
        let mut r1 = r1d;
        let mut r2 = r2d;
        if (load(ctl, OFF_FLAGB) & FLAG_BIT) != 0 {
            let link = load(ctl, OFF_LINK);
            if link != 0 {
                // The mutant inverts this select: kind-equal trials take
                // the kind-differ radii and vice versa.
                let is_want = load(link, OFF_KIND) == KIND_WANT;
                if is_want != mutant {
                    r1 = *(lf_checker_rt::global::<f32>(G_R1EQ));
                    r2 = r1d;
                } else {
                    r1 = *(lf_checker_rt::global::<f32>(G_R1NE));
                    r2 = *(lf_checker_rt::global::<f32>(G_R2NE));
                }
            }
        }
        let pos = load(ctl, OFF_POS);
        let px = fload(pos, 0x30);
        let py = fload(pos, 0x34);
        let pz = fload(pos, 0x38);
        // First box test: any strictly-above result continues; only an
        // all-below-or-equal outcome (unordered counts as below) clears
        // the flag and exits.
        let mut inside = true;
        if (ax + r1) > px {
            inside = false;
        } else if px > (bx - r1) {
            inside = false;
        } else if (ay + r1) > py {
            inside = false;
        } else if py > (by - r1) {
            inside = false;
        } else if (az + r1) > pz {
            inside = false;
        } else if !(pz > (bz - r1)) {
            // Final side uses the below-or-equal exit: unordered takes it.
            *(lf_checker_rt::global::<u8>(G_FLAG)) = 0;
            // Fill ran: the subtrahend slot holds the first sample word.
            return exit_block(this, ax);
        } else {
            inside = false;
        }
        let _ = inside;
        // Flag block: reseed unless already done.
        if *(lf_checker_rt::global::<u8>(G_FLAG)) == 0 {
            let opened = lf_checker_rt::callee_thiscall!(
                CAL_OPEN, u32,
                lf_checker_rt::relocated(G_OBJ),
                lf_checker_rt::relocated(G_KEY)
            );
            let word = *(lf_checker_rt::global::<u32>(G_WORD));
            lf_checker_rt::callee_cdecl!(
                CAL_SUBMIT, u32,
                opened, word, 0, 0xFFFF_FFFF, 0xFA0, 0x2710, 0, 0,
                0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF,
                0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0
            );
            *(lf_checker_rt::global::<u8>(G_FLAG)) = 1;
        }
        // Second box test: above on the high sides, above-or-equal on the
        // low sides dispatches; a fully-inside result (the final side uses
        // the strict-below exit, which unordered takes) runs the exit block.
        let mut dispatch = false;
        if (ax + r2) > px {
            dispatch = true;
        } else if px >= (bx - r2) {
            dispatch = true;
        } else if (ay + r2) > py {
            dispatch = true;
        } else if py >= (by - r2) {
            dispatch = true;
        } else if (az + r2) > pz {
            dispatch = true;
        } else if !(pz >= (bz - r2)) {
            return exit_block(this, ax);
        } else {
            dispatch = true;
        }
        let mut path_b = false;
        if dispatch {
            if (load(ctl, OFF_FLAGB) & FLAG_BIT) == 0 {
                vcall1(ctl, SLOT_CTL_A, 0);
                path_b = true;
            } else {
                let d = load(ctl, OFF_LINK);
                let e = load(d, OFF_SUB);
                if e != 0 && *((e.wrapping_add(OFF_SUBB)) as *const u8) != 0 {
                    vcall1(ctl, SLOT_CTL_A, 0);
                    path_b = true;
                } else {
                    vcall5(d, SLOT_LINK, 0, 0, 1, 0, 0x33);
                }
            }
        }
        // The one-argument path pushes one word more than the callee pops,
        // shifting the exit frame so the subtrahend slot is the stored
        // global; otherwise it is still the first sample word.
        let sub = if path_b {
            *(lf_checker_rt::global::<f32>(G_SUB))
        } else {
            ax
        };
        exit_block(this, sub)
    }
}

/// Squared-length exit shared by every path: returns the position link
/// when the squared length is at or below the limit (unordered counts as
/// below), otherwise feeds a float table answer minus the caller-provided
/// subtrahend into a second table call and returns its answer.
unsafe fn exit_block(this: u32, sub: f32) -> u32 {
    const OFF_CTL: u32 = 0x598;
    const OFF_POS: u32 = 0x20;
    const SLOT_CTL_A: u32 = 0xF4;
    const SLOT_CTL_F: u32 = 0xFC;
    const G_LIM: u32 = 0x00E80DC0;
    unsafe {
        let ctl = *((this.wrapping_add(OFF_CTL)) as *const u32);
        let pos = *((ctl.wrapping_add(OFF_POS)) as *const u32);
        let px = *((pos.wrapping_add(0x30)) as *const f32);
        let py = *((pos.wrapping_add(0x34)) as *const f32);
        let pz = *((pos.wrapping_add(0x38)) as *const f32);
        // Pinned accumulation order: (px*px + py*py) + pz*pz.
        let px2 = core::hint::black_box(px) * core::hint::black_box(px);
        let py2 = core::hint::black_box(py) * core::hint::black_box(py);
        let pz2 = core::hint::black_box(pz) * core::hint::black_box(pz);
        let s01 = core::hint::black_box(px2) + core::hint::black_box(py2);
        let lensq = core::hint::black_box(s01) + core::hint::black_box(pz2);
        let lim = *(lf_checker_rt::global::<f32>(G_LIM));
        if !(lensq > lim) {
            return pos;
        }
        let table = *(ctl as *const u32);
        let target_f = *((table.wrapping_add(SLOT_CTL_F)) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> f32 =
            core::mem::transmute(target_f as usize);
        let ans = f(ctl, 0);
        let diff = core::hint::black_box(ans) - core::hint::black_box(sub);
        let target_a = *((table.wrapping_add(SLOT_CTL_A)) as *const u32);
        let g: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target_a as usize);
        g(ctl, diff.to_bits())
    }
}
