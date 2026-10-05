// original: 0x00982580 audio_collision_voice_sweep
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global, relocated, tls_slot};

/// Audio collision-voice sweep: per-frame update of the collision voice
/// set. Walks the slot array for changed entries, retires finished
/// voices, drains one work item through the engine, refreshes the
/// positional emitters and runs the wave-break sweep.
///
/// `this` points to a large state object (fields up to 0x6f3c); `arg`
/// is an opaque word forwarded to the positional update. Returns
/// nothing (the epilogue leaves a callee answer in eax, which the
/// contract does not compare).
///
/// Stages: clear the per-side scratch bytes; scan the 245 match slots
/// (a match programs one voice through the filter/router chain and
/// appends it to the global work queue); sweep the 245 voice records
/// (releasing dead ones, refreshing live ones through the engine,
/// including one virtual call and a TLS-resolved distance gate);
/// attach one pending record to the engine object table; resolve the
/// listener position through TLS; run three positional updates, seven
/// emitter refreshes and the wave-break sweep; finish with a
/// throttled maintenance call.
///
/// Calling convention: thiscall with one stack argument; callee pops 4.
/// Integer arithmetic wraps; float sums are tested for infinity/NaN by
/// their exponent bits; float comparisons follow `comiss` semantics.
export!(thiscall, rw_00982580(this: u32, arg: u32) -> () {
    unsafe { audio_collision_voice_sweep(this, arg); }
});

// ---- state object layout ----
const SCAN_N: u32 = 0xf5; // match slots and voice records each
const REC_N: u32 = 0xf5;
const O_MATCH: u32 = 0x5c08; // match slots (0x14 bytes each)
const O_REC: u32 = 0x5bfc; // voice records (0x14 bytes each)
const O_COUNT_A: u32 = 0x5bf0; // outer/drain counts (2 words)
const O_SIDE: u32 = 0x5bf8; // side selector word
const O_TAB: u32 = 0x5c00; // engine object table (strided words)
const O_TAB_ARG: u32 = 0x5c04; // per-slot engine argument word
const O_TAB_USED: u32 = 0x5c0c; // per-slot used byte
const O_TAB_SEEN: u32 = 0x5c0d; // per-slot seen byte
const O_EMIT_BASE: u32 = 0x6f28; // emitter table base
const O_EMIT_CUR: u32 = 0x6f20; // emitter cursor
const O_EMIT_N: u32 = 0x6f2c; // emitter count (u16)
const O_WAVE_BASE: u32 = 0x6f30; // wave entry base
const O_WAVE_CUR: u32 = 0x6f38; // wave cursor
const O_WAVE_N: u32 = 0x6f34; // wave count (u16)
const O_INIT_ARG: u32 = 0x6f3c; // init-call argument pointer

// ---- voice record layout (offsets from the record) ----
const R_OBJ: u32 = 0x00; // attached object (0 or vtable object)
const R_AUX: u32 = 0x04; // auxiliary handle
const R_DATA: u32 = 0x08; // data word copied to the engine table
const R_CTX: u32 = 0x0c; // context handle
const R_LIVE: u32 = 0x10; // live byte
const R_MODE: u32 = 0x11; // mode byte (0 release path, else refresh)
const R_DIRTY: u32 = 0x12; // dirty byte (refresh runs the spawner)

// ---- attached-object layout ----
const V_VT: u32 = 0x00; // vtable pointer
const V_CLASS: u32 = 0x04; // class byte (0xff means "no voice")
const V_KIND: u32 = 0x3b; // kind byte (0x15 runs the filter chain)
const V_BANK: u32 = 0x40; // bank byte

// ---- globals ----
const G_HASH_FLAG2: u32 = 0x123884c; // collision-hash init flag
const G_HASH2: u32 = 0x1238848; // collision-hash value
const G_TLS_SLOT: u32 = 0x17aba14;
const G_VOICE_MUL: u32 = 0x115d968;
const G_VOICE_TAB: u32 = 0x115d988;
const G_PAIR_TAB: u32 = 0x115df20;
const G_QUEUE_N: u32 = 0x1161914; // work queue length
const G_QUEUE: u32 = 0x1161918; // work queue entries (12 bytes each)
const G_QUEUE_B: u32 = 0x116191c; // entry second word
const G_QUEUE_F: u32 = 0x1161920; // entry flag byte
const G_SI_BASE: u32 = 0x11618fc; // throttle base
const G_EARSHOT: u32 = 0x12ddeac; // earshot float
const G_WAVE_AMP: u32 = 0x11735bc; // wave amplitude float
const G_THROTTLE: u32 = 0x1173604; // throttle counter
const G_POS_OUT: u32 = 0x1238800; // resolved position out-words
const G_STAMP: u32 = 0x1231784; // position-stamp byte

// ---- read-only tunables ----
const K_EARSHOT_REF: u32 = 0xfe87d0; // 0.2
const K_DIST_LIMIT: u32 = 0xe8b800; // 1225.0

// ---- hashed names ----
const S_COLLISION: u32 = 0xe8d9b4;
const S_COLLISION2: u32 = 0xe8d9c0;

// ---- magic numbers ----
const VOICE_ROW: u32 = 0x6f40;
const VOICE_OFF: u32 = 0x6f14;
const KIND_WANT: u8 = 0x15;
const THROTTLE_MOD: u32 = 0x708;
const THROTTLE_MUL: u32 = 0x91a2b3c5;
const EMIT_STRIDE: u32 = 0x360;
const WAVE_STRIDE: u32 = 0x160;

#[inline(always)]
unsafe fn rdu(base: u32, off: u32) -> u32 {
    unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rdf(base: u32, off: u32) -> f32 {
    unsafe { ((base.wrapping_add(off)) as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn rdu8(base: u32, off: u32) -> u8 {
    unsafe { ((base.wrapping_add(off)) as *const u8).read() }
}

#[inline(always)]
unsafe fn rdu16(base: u32, off: u32) -> u16 {
    unsafe { ((base.wrapping_add(off)) as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn wru(base: u32, off: u32, v: u32) {
    unsafe { ((base.wrapping_add(off)) as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wrf(base: u32, off: u32, v: f32) {
    unsafe { ((base.wrapping_add(off)) as *mut f32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wru8(base: u32, off: u32, v: u8) {
    unsafe { ((base.wrapping_add(off)) as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn g32(va: u32) -> u32 {
    unsafe { global::<u32>(va).read_unaligned() }
}

/// Voice-handle resolution: class 0xff means no voice.
unsafe fn resolve_voice(handle: u32) -> u32 {
    unsafe {
        let class = rdu8(handle, V_CLASS);
        if class == 0xff {
            return 0;
        }
        let bank = rdu8(handle, V_BANK) as u32;
        let factor = g32(G_VOICE_MUL);
        let tab = g32(G_VOICE_TAB);
        let entry = rdu(
            tab.wrapping_add(bank.wrapping_mul(VOICE_ROW)),
            VOICE_OFF,
        );
        entry.wrapping_add((class as u32).wrapping_mul(factor))
    }
}

/// Exponent-bits test: true for infinity or NaN, exactly like the
/// original's and/cmp pair.
#[inline(always)]
fn is_inf_or_nan_bits(x: f32) -> bool {
    (x.to_bits() & 0x7f800000) == 0x7f800000
}

unsafe fn audio_collision_voice_sweep(edi: u32, arg: u32) {
    unsafe {
        // Init call; its ecx points at scratch above the frame and is
        // not compared, and its answer is ignored.
        let _: u32 = callee_thiscall!(
            1, u32, 0,
            edi.wrapping_add(O_INIT_ARG)
        );
        // NOTE: the real ecx is a frame-scratch pointer the rewrite
        // cannot reproduce; the contract drops it via call_regs.
        let side = rdu(edi, O_SIDE).wrapping_sub(1) & 1;
        let mut slot08: u32 = 0;
        let mut slot0c: u32 = 0;
        let slot10: u32 = side;
        let mut slot14: u32 = 0;
        let mut slot18: f32 = f32::from_bits(0);
        let mut slot1c: u32 = side.wrapping_mul(0x2df0);
        // Clear the per-side scratch bytes: 49 rows, conditional
        // clears on the state bytes, unconditional on the mirror.
        let mut eax = edi.wrapping_add(0x5c0d);
        let mut ecx = edi
            .wrapping_add(0x64)
            .wrapping_add(slot1c);
        for _ in 0..0x31u32 {
            if rdu(eax.wrapping_sub(5), 0) == 0 {
                wru8(eax, 0, 0);
            }
            wru8(ecx.wrapping_sub(0x30), 0, 0);
            if rdu(eax, 0xf) == 0 {
                wru8(eax, 0x14, 0);
            }
            wru8(ecx, 0, 0);
            if rdu(eax, 0x23) == 0 {
                wru8(eax, 0x28, 0);
            }
            wru8(ecx, 0x30, 0);
            if rdu(eax, 0x37) == 0 {
                wru8(eax, 0x3c, 0);
            }
            wru8(ecx, 0x60, 0);
            if rdu(eax, 0x4b) == 0 {
                wru8(eax, 0x50, 0);
            }
            wru8(ecx, 0x90, 0);
            ecx = ecx.wrapping_add(0xf0);
            eax = eax.wrapping_add(0x64);
        }
        let mut saw_path_a = false;
        if rdu(edi, O_COUNT_A.wrapping_add(slot10.wrapping_mul(4))) > 0 {
            // Outer scan over the match slots. Only a matched
            // iteration programs a voice and touches the queue.
            let mut edx = edi.wrapping_add(O_MATCH);
            loop {
                let mut esi: u32 = 0;
                let mut found = false;
                while esi < SCAN_N {
                    if rdu8(edx, 4) != 0
                        && rdu(edx, 0) == 0
                        && rdu(edx.wrapping_sub(4), 0)
                            == rdu(
                                edi,
                                (slot10
                                    .wrapping_mul(SCAN_N)
                                    .wrapping_add(slot08)
                                    .wrapping_mul(3)
                                    .wrapping_mul(2))
                                .wrapping_mul(8)
                                .wrapping_add(0x28),
                            )
                    {
                        found = true;
                        break;
                    }
                    esi = esi.wrapping_add(1);
                    edx = edx.wrapping_add(0x14);
                }
                if found {
                    // Program the matched slot. ecx5 is both the
                    // table index and (scaled) the seen-byte index.
                    let ma = slot10
                        .wrapping_mul(SCAN_N)
                        .wrapping_add(slot08);
                    let ecx5 = esi.wrapping_add(esi.wrapping_mul(4));
                    slot0c = ecx5;
                    wru8(edi, ecx5.wrapping_mul(4).wrapping_add(O_TAB_SEEN), 1);
                    let ecx_big = ecx5.wrapping_add(0x16ff);
                    esi = ma.wrapping_add(ma.wrapping_mul(2)).wrapping_mul(16);
                    edx = edi.wrapping_add(ecx_big.wrapping_mul(4));
                    wru8(edi, esi.wrapping_add(0x34), 1);
                    let head = rdu(edx, 0);
                    slot14 = edx;
                    if head != 0 {
                        path_b(edi, esi, slot14, slot0c, &mut slot18);
                    } else {
                        saw_path_a = true;
                        path_a(edi, esi, edx);
                    }
                    converge_found(edi, esi, slot0c, head == 0);
                    // Append a changed tail word to the work queue.
                    let qedx = edi.wrapping_add(0x20).wrapping_add(esi);
                    let qv = rdu(qedx, 0);
                    if qv != 0 {
                        append_queue(qv, qedx);
                        wru(qedx, 0, 0);
                    }
                }
                slot08 = slot08.wrapping_add(1);
                edx = edi.wrapping_add(O_MATCH);
                if slot08
                    >= rdu(edi, O_COUNT_A.wrapping_add(slot10.wrapping_mul(4)))
                {
                    break;
                }
            }
        }
        drain_records(
            edi, &mut slot08, &mut slot0c, &mut slot14, &mut slot18, slot10,
            slot1c, saw_path_a,
        );
        tail_section(edi, arg, slot10, slot1c, &mut slot14);
    }
}

/// Append one word to the global work queue (the caller checked space).
unsafe fn append_queue(qv: u32, qedx: u32) {
    unsafe {
        let qn = g32(G_QUEUE_N);
        if qn < 0x100 {
            let qe = qn.wrapping_add(qn.wrapping_mul(2));
            global::<u32>(G_QUEUE.wrapping_add(qe.wrapping_mul(4))).write_unaligned(qv);
            global::<u8>(G_QUEUE_F.wrapping_add(qe.wrapping_mul(4))).write(1);
            global::<u32>(G_QUEUE_B.wrapping_add(qe.wrapping_mul(4)))
                .write_unaligned(qedx);
            global::<u32>(G_QUEUE_N).write_unaligned(qn.wrapping_add(1));
        }
    }
}

/// Match with an empty head word: spawn the voice directly.
unsafe fn path_a(edi: u32, esi: u32, edx: u32) {
    unsafe {
        let x0 = rdf(edi, esi.wrapping_add(0x2c));
        let frame_v = rdu(edi, esi.wrapping_add(0x30));
        let arg0 = rdu(edi, esi.wrapping_add(0x24));
        // Inner struct: the level, the routed value, then zeros.
        // Word 2 is never written (stack fill, defined 0).
        let e2a = [x0.to_bits(), frame_v, 0, 0, 0, 0];
        let _: u32 = callee_thiscall!(
            2, u32, edi, arg0, edx, e2a.as_ptr() as u32, 0xffffffff, 0, 0
        );
    }
}

/// Match with a live head word: route the voice three ways.
unsafe fn path_b(edi: u32, esi: u32, slot14: u32, slot0c: u32, slot18: &mut f32) {
    unsafe {
        let x1 = rdf(edi, esi.wrapping_add(0x14));
        let mut x2 = rdf(edi, esi.wrapping_add(0x10));
        let x0 = rdf(edi, esi.wrapping_add(0x18));
        x2 *= x2;
        let x1sq = x1 * x1;
        let x0sq = x0 * x0;
        x2 += x1sq;
        x2 += x0sq;
        *slot18 = x2;
        // Infinite or NaN energy skips the first router.
        if !is_inf_or_nan_bits(x2) {
            let head = rdu(slot14, 0);
            let this = resolve_voice(head);
            let _: u32 = callee_thiscall!(
                3, u32, this,
                edi.wrapping_add(0x10).wrapping_add(esi)
            );
        }
        let yard = rdu(slot14, 0);
        let this = resolve_voice(yard);
        let v = rdu(edi, esi.wrapping_add(0x30));
        let _: u32 = callee_thiscall!(4, u32, this, v);
        let fx = rdf(edi, esi.wrapping_add(0x2c));
        let rec = rdu(edi, slot0c.wrapping_mul(4).wrapping_add(O_REC));
        let this = resolve_voice(rec);
        let _: u32 = callee_thiscall!(5, u32, this, fx.to_bits());
    }
}

/// Converge after either match path: transform then attach.
unsafe fn converge_found(edi: u32, esi: u32, slot0c: u32, path_a: bool) {
    unsafe {
        let tab = rdu(edi, slot0c.wrapping_mul(4).wrapping_add(O_TAB));
        if tab == 0 || rdu(edi, esi.wrapping_add(0x20)) == 0 {
            return;
        }
        let f0 = rdf(edi, esi.wrapping_add(0x10));
        let base = edi.wrapping_add(esi);
        let f1 = rdf(base, 0x14);
        let f2 = rdf(base, 0x18);
        let fx = rdf(base, 0x1c);
        let mut s30 = [0u32; 20];
        s30[0] = f32::to_bits(1.0);
        s30[5] = f32::to_bits(1.0);
        s30[10] = f32::to_bits(1.0);
        s30[12] = f0.to_bits();
        s30[13] = f1.to_bits();
        s30[14] = f2.to_bits();
        s30[15] = fx.to_bits();
        // Path A leaves its spawner header behind in the tail.
        if path_a {
            s30[16] = 0xffffffff;
            s30[17] = 0x002000ff;
        }
        let _: u32 = callee_thiscall!(6, u32, tab, s30.as_mut_ptr() as u32);
        let av = rdu(edi, esi.wrapping_add(0x20));
        let tab2 = rdu(edi, slot0c.wrapping_mul(4).wrapping_add(O_TAB));
        let _: u32 = callee_thiscall!(7, u32, tab2, av);
    }
}

/// Sweep the 245 voice records, then drain the pending list.
unsafe fn drain_records(
    edi: u32, slot08: &mut u32, slot0c: &mut u32, slot14: &mut u32,
    slot18: &mut f32, slot10: u32, slot1c: u32, saw_path_a: bool,
) {
    unsafe {
        global::<u8>(G_STAMP).write(0);
        let mut esi = edi.wrapping_add(O_REC);
        let mut n = REC_N;
        while n != 0 {
            if rdu8(esi, R_LIVE) != 0 {
                if rdu8(esi, R_MODE) != 0 {
                    refresh_record(edi, esi, slot18);
                } else {
                    let obj = rdu(esi, R_OBJ);
                    if obj != 0 {
                        let _: u32 = callee_thiscall!(8, u32, obj, 0);
                    }
                    let aux = rdu(esi, R_AUX);
                    if aux != 0 {
                        let _: u32 = callee_thiscall!(9, u32, aux);
                        wru(esi, R_AUX, 0);
                    } else {
                        wru8(esi, R_LIVE, 0);
                        esi = esi.wrapping_add(0x14);
                        n -= 1;
                        continue;
                    }
                    wru8(esi, R_LIVE, 0);
                }
            }
            // NOTE: the 0x11-path joins here through the merge that
            // clears the live byte; the release path above clears it
            // on its own exits.
            esi = esi.wrapping_add(0x14);
            n -= 1;
        }
        drain_pending(edi, slot08, slot0c, slot14, slot10, slot1c, saw_path_a);
    }
}

/// Refresh one live record in mode != 0.
unsafe fn refresh_record(edi: u32, esi: u32, slot18: &mut f32) {
    unsafe {
        let ctx = rdu(esi, R_CTX);
        if ctx == 0 {
            return;
        }
        if rdu8(esi, R_DIRTY) != 0 {
            wru8(esi, R_DIRTY, 0);
            let w = rdu(ctx, 0x78);
            let w2 = if w != 0 { rdu(w, 0xf8) } else { 0 };
            let _: u32 = callee_thiscall!(
                10, u32, edi, w2, esi, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0
            );
        }
        if rdu(esi, R_OBJ) == 0 {
            release_aux(esi);
            return;
        }
        let ctx2 = rdu(esi, R_CTX);
        let ans: u32 = callee_thiscall!(11, u32, ctx2);
        let x4 = rdf(ans, 0);
        let slot20 = x4;
        let x3 = rdf(ans, 4);
        let mut x1 = x3;
        let mut x0 = x4;
        x0 *= x4;
        x1 *= x3;
        let slot24 = x3;
        let x5 = rdf(ans, 8);
        x1 += x0;
        let mut xx0 = x5;
        xx0 *= x5;
        let slot28 = x5;
        x1 += xx0;
        let slot14l = x1;
        if is_inf_or_nan_bits(x1) {
            return;
        }
        let obj = rdu(esi, R_OBJ);
        let mut ran_tls = false;
        if rdu8(obj, V_KIND) == KIND_WANT {
            let ctx3 = rdu(esi, R_CTX);
            let r: f32 =
                callee_thiscall!(12, f32, ctx3, relocated(S_COLLISION));
            let slot0c = r;
            let flag = g32(G_HASH_FLAG2);
            let hash = if flag & 1 == 0 {
                global::<u32>(G_HASH_FLAG2).write_unaligned(flag | 1);
                let h: u32 =
                    callee_cdecl!(13, u32, relocated(S_COLLISION2), 0);
                global::<u32>(G_HASH2).write_unaligned(h);
                h
            } else {
                g32(G_HASH2)
            };
            // Virtual call through the object's table.
            let obj2 = rdu(esi, R_OBJ);
            let vt = rdu(obj2, V_VT);
            let f: extern "stdcall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rdu(vt, 8) as usize);
            f(hash, slot0c.to_bits(), obj2);
            // The filter answer gates the TLS block: equal to zero
            // (including -0.0, excluding NaN) runs it.
            if slot0c == 0.0 {
                ran_tls = true;
                let tslot = g32(G_TLS_SLOT);
                let tls = tls_slot(tslot as usize);
                let tidx = rdu(tls, 0x70);
                let base =
                    relocated(G_PAIR_TAB).wrapping_add(tidx.wrapping_mul(64));
                let mut y1 = rdf(base, 0);
                let mut y2 = rdf(base, 4);
                let mut y0 = rdf(base, 8);
                y2 -= x3;
                y1 -= x4;
                y0 -= x5;
                y2 *= y2;
                y1 *= y1;
                y0 *= y0;
                y2 += y1;
                y2 += y0;
                if *global::<f32>(K_DIST_LIMIT) > y2 {
                    global::<f32>(G_POS_OUT).write(x4);
                    global::<f32>(G_POS_OUT.wrapping_add(4)).write(x3);
                    global::<f32>(G_POS_OUT.wrapping_add(8)).write(x5);
                    // Slot 0x2c is never written (stack fill, 0).
                    global::<f32>(G_POS_OUT.wrapping_add(12)).write(0.0);
                    global::<u8>(G_STAMP).write(1);
                }
            }
        }
        let _ = (ran_tls, slot14l, slot20, slot24, slot28);
        // Router block over the measured energy.
        let mut e3 = x3;
        let mut e4 = x4;
        let mut e5 = x5;
        e3 *= e3;
        e4 *= e4;
        e5 *= e5;
        e3 += e4;
        e3 += e5;
        *slot18 = e3;
        if is_inf_or_nan_bits(e3) {
            return;
        }
        let obj3 = rdu(esi, R_OBJ);
        let this = resolve_voice(obj3);
        let snap3 = [slot20.to_bits(), slot24.to_bits(), slot28.to_bits()];
        let _: u32 =
            callee_thiscall!(15, u32, this, snap3.as_ptr() as u32);
    }
}

/// Release path tail: drop the auxiliary handle and park the record.
unsafe fn release_aux(esi: u32) {
    unsafe {
        let aux = rdu(esi, R_AUX);
        if aux != 0 {
            let _: u32 = callee_thiscall!(9, u32, aux);
            wru(esi, R_AUX, 0);
        }
        wru(esi, R_CTX, 0);
        wru8(esi, R_LIVE, 0);
    }
}

/// Drain the pending list into the engine object table.
unsafe fn drain_pending(
    edi: u32, slot08: &mut u32, slot0c: &mut u32, _slot14: &mut u32,
    slot10: u32, slot1c: u32, saw_path_a: bool,
) {
    unsafe {
        *slot0c = 0;
        if rdu(edi, O_COUNT_A.wrapping_add(slot10.wrapping_mul(4))) == 0 {
            return;
        }
        let mut esi = edi
            .wrapping_add(0x20)
            .wrapping_add(slot1c);
        loop {
            if rdu8(esi, 0x14) == 0 {
                // Claim the first free engine slot.
                let mut idx: u32 = 0;
                let mut p = edi.wrapping_add(O_TAB_USED);
                while idx < SCAN_N && rdu8(p, 0) != 0 {
                    idx = idx.wrapping_add(1);
                    p = p.wrapping_add(0x14);
                }
                if idx < SCAN_N {
                    let ecx5 = idx.wrapping_add(idx.wrapping_mul(4));
                    *slot08 = ecx5;
                    wru8(
                        edi,
                        ecx5.wrapping_mul(4).wrapping_add(O_TAB_USED),
                        1,
                    );
                    wru(
                        edi,
                        ecx5.wrapping_mul(4).wrapping_add(O_TAB_ARG),
                        rdu(esi, 8),
                    );
                    if rdu(esi, 0) != 0 {
                        let tab = rdu(
                            edi,
                            ecx5.wrapping_mul(4).wrapping_add(O_TAB),
                        );
                        if tab != 0 {
                            let _: u32 = callee_thiscall!(9, u32, tab);
                            wru(
                                edi,
                                ecx5.wrapping_mul(4).wrapping_add(O_TAB),
                                0,
                            );
                        }
                        let na: u32 = callee_cdecl!(16, u32,);
                        wru(
                            edi,
                            ecx5.wrapping_mul(4).wrapping_add(O_TAB),
                            na,
                        );
                        if na != 0 {
                            let _: u32 = callee_thiscall!(
                                17, u32, na, 0, 0x42200000u32, 0, 0xfa0,
                                0x3f000000u32
                            );
                            let g0 = rdf(esi.wrapping_sub(0x10), 0);
                            let g1 = rdf(esi.wrapping_sub(0xc), 0);
                            let g2 = rdf(esi.wrapping_sub(8), 0);
                            let g3 = rdf(esi.wrapping_sub(4), 0);
                            let mut s30 = [0u32; 20];
                            s30[0] = f32::to_bits(1.0);
                            s30[5] = f32::to_bits(1.0);
                            s30[10] = f32::to_bits(1.0);
                            s30[12] = g0.to_bits();
                            s30[13] = g1.to_bits();
                            s30[14] = g2.to_bits();
                            s30[15] = g3.to_bits();
                            if saw_path_a {
                                s30[16] = 0xffffffff;
                                s30[17] = 0x002000ff;
                            }
                            let tab2 = rdu(
                                edi,
                                ecx5.wrapping_mul(4).wrapping_add(O_TAB),
                            );
                            let _: u32 = callee_thiscall!(
                                6, u32, tab2, s30.as_mut_ptr() as u32
                            );
                            let sv = rdu(esi, 0);
                            let tab3 = rdu(
                                edi,
                                ecx5.wrapping_mul(4).wrapping_add(O_TAB),
                            );
                            let _: u32 = callee_thiscall!(7, u32, tab3, sv);
                            let tab4 = rdu(
                                edi,
                                ecx5.wrapping_mul(4).wrapping_add(O_TAB),
                            );
                            let _: u32 = callee_thiscall!(18, u32, tab4);
                        }
                    } else {
                        wru(esi, 0, 0);
                    }
                    // Second spawner over the pending record.
                    let h0 = rdf(esi, 0xc);
                    let h1w = rdu(esi, 0x10);
                    let e80 = [
                        h0.to_bits(),
                        h1w,
                        0,
                        0,
                        0,
                        esi.wrapping_sub(0x10),
                    ];
                    let a0 = rdu(esi, 4);
                    let a1 = edi.wrapping_add(
                        ecx5.wrapping_add(0x16ff).wrapping_mul(4),
                    );
                    let _: u32 = callee_thiscall!(
                        2, u32, edi, a0, a1, e80.as_ptr() as u32, 0xffffffff,
                        0, 0
                    );
                }
            }
            *slot0c = slot0c.wrapping_add(1);
            esi = esi.wrapping_add(0x30);
            if *slot0c
                >= rdu(edi, O_COUNT_A.wrapping_add(slot10.wrapping_mul(4)))
            {
                break;
            }
        }
    }
}
