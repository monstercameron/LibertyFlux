// original: 0x00aec9c0 timing_main_update (proposed, STAGE A1 — see below)

/// Main timing update: build a worker object and run the update chain.
///
/// Full rewrite: short path, middle chain, row loop and table section.
/// `this`+0x14 is a byte gate for the table section; the index global
/// selects one of the table rows through a bit-count loop, the float query
/// (virtual slot +0x58) picks between two row addresses, and the close
/// sequence links the worker through the row callee's answer.
///
/// Thiscall with one stack word (`arg`); `this` is a controller. Returns the
/// entry accumulator (contract-fixed to 0) when `arg` is null or when the
/// header word at `arg+0xc` is set without the ready bit at `arg+0xa`.
/// Otherwise it ticks the controller, opens a frame through the frame
/// callee, clears the header, resolves the probe (null on this stage),
/// builds the worker object, links the mode table entry, runs the two
/// virtual slots and the bind call, stamps the worker flags, records the
/// null probe result and runs the close callee, returning its answer.
///
/// The scratch slot the original reserves is only written on the middle
/// path; on this stage it keeps the defined stack fill (0), which the close
/// sequence stores. The return is the full accumulator.
lf_checker_rt::export!(thiscall, rw_00aec9c0(this: u32, arg: u32) -> u32 {
    unsafe {
        const HDR_A: u32 = 0x0a;
        const HDR_C: u32 = 0x0c;
        const TICK: u32 = 0x10;
        const GATEW: u32 = 0x14;
        const ARG4: u32 = 0x04;
        const ARGBASE: u32 = 0x10;
        const WOBJ_ARG: u32 = 0x104;
        const WOBJ_FLAGS: u32 = 0x24;
        const WOBJ_LINK: u32 = 0x224;
        const WOBJ_IDX: u32 = 0x2e;
        const WOBJ_TAG: u32 = 0x41;
        const WOBJ_STATE: u32 = 0x63;
        const ROW_TABLE: u32 = 0x01295cd8;
        const G_LINK: u32 = 0x011735b4;
        const VT_RUN: u32 = 0x30;
        const VT_SYNC: u32 = 0x40;
        const VT_BIND: u32 = 0x04;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if arg == 0 {
            return 0;
        }
        let hdr_c = rd32(arg + HDR_C);
        if hdr_c != 0 && ((arg + HDR_A) as *const u8).read() & 0x80 == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(1, u32, this);
        let tickp = (this + TICK) as *mut u32;
        tickp.write_unaligned(tickp.read_unaligned().wrapping_add(1));
        let frame: u32 = lf_checker_rt::callee_thiscall!(2, u32, arg);
        ((arg + HDR_C) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(3, u32, arg, 0x80u32, 0u32);
        let ebp: u32 = if hdr_c != 0 {
            lf_checker_rt::callee_cdecl!(4, u32, hdr_c)
        } else {
            0
        };
        let worker: u32 = lf_checker_rt::callee_cdecl!(
            5,
            u32,
            rd32(frame.wrapping_add(WOBJ_ARG)),
            4u32,
            0u32,
            0u32,
            1u32,
            0xffff_ffffu32,
        );
        ((worker + WOBJ_FLAGS) as *mut u32)
            .write_unaligned(rd32(worker + WOBJ_FLAGS) | 0x0400_0000);
        ((worker + WOBJ_LINK) as *mut u32)
            .write_unaligned((lf_checker_rt::global::<u32>(G_LINK)).read_unaligned());
        lf_checker_rt::callee_thiscall!(6, u32, worker, arg, 0u32);
        lf_checker_rt::callee_thiscall!(7, u32, worker);
        let idx = ((worker + WOBJ_IDX) as *const i16).read_unaligned();
        let row = rd32(
            lf_checker_rt::relocated(ROW_TABLE).wrapping_add((idx as i32 as u32).wrapping_mul(4)),
        );
        lf_checker_rt::callee_thiscall!(8, u32, row);
        let vt = rd32(worker);
        let run: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_RUN) as usize);
        run(worker, 1);
        let sync: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_SYNC) as usize);
        sync(worker);
        ((worker + WOBJ_FLAGS) as *mut u32)
            .write_unaligned(rd32(worker + WOBJ_FLAGS) | 0x0008_0000);
        let a4 = rd32(arg + ARG4);
        ((a4 + 0x10) as *mut u32).write_unaligned(0x80);
        ((a4 + 0x14) as *mut u32).write_unaligned(0x7bee);
        let bind: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_BIND) as usize);
        bind(worker, arg.wrapping_add(ARGBASE), 1, 0);
        ((worker + WOBJ_FLAGS) as *mut u32)
            .write_unaligned(rd32(worker + WOBJ_FLAGS) | 0x0000_0100);
        // Scratch slot: the row loop's table object, else the fill (0).
        let mut slot12 = 0u32;
        if ebp == 0 {
            ((worker + WOBJ_TAG) as *mut u8).write(2);
            lf_checker_rt::callee_cdecl!(27, u32, worker, 0u32);
            ((worker + WOBJ_STATE) as *mut u8).write(0xff);
        } else {
            let esi = if rd32(ebp + 0x28) & 0x3c0 == 0x100 && rd32(ebp + 0x280) != 0 {
                rd32(ebp + 0x280)
            } else {
                ebp
            };
            lf_checker_rt::callee_thiscall!(12, u32, worker, esi);
            ((worker + WOBJ_STATE) as *mut u8)
                .write(((ebp + 0x63) as *const u8).read());
            if rd32(esi + 0x28) & 0x3c0 != 0x80 {
                ((esi + 0x24) as *mut u32)
                    .write_unaligned(rd32(esi + 0x24) | 0x0000_0100);
            }
            lf_checker_rt::callee_thiscall!(
                13,
                u32,
                lf_checker_rt::relocated(0x013b_aba0),
                ebp,
                worker,
            );
            lf_checker_rt::callee_thiscall!(
                14,
                u32,
                lf_checker_rt::relocated(0x0139_4d60),
                ebp,
                worker,
            );
            lf_checker_rt::callee_cdecl!(15, u32, ebp, worker);
            if rd32(ebp + 0x28) & 0x3c0 == 0x100 {
                let r16: u32 = lf_checker_rt::callee_thiscall!(16, u32, ebp);
                if r16 & 0xff != 0 {
                    ((ebp + 0x210) as *mut u32)
                        .write_unaligned(rd32(ebp + 0x210) | 0x0080_0000);
                }
            }
            if ((ebp + 0x148) as *const u8).read() != 0 {
                lf_checker_rt::callee_thiscall!(17, u32, ebp.wrapping_add(0x124));
            }
            if rd32(esi + 0x28) & 0x3c0 == 0x100
                && ((esi + 0x22a) as *const u8).read() == 2
            {
                ((worker + WOBJ_LINK) as *mut u32).write_unaligned(
                    rd32(worker + WOBJ_LINK).wrapping_add(0x0001_86a0),
                );
            }
            let w = (rd32(ebp + 0x28) ^ rd32(worker + 0x28)) & 0x0010_0000;
            ((worker + 0x28) as *mut u32)
                .write_unaligned(rd32(worker + 0x28) ^ w);
            if rd32(ebp + 0x28) & 0x3c0 != 0xc0 {
                let idx = ((worker + WOBJ_IDX) as *const i16).read_unaligned();
                let row = rd32(
                    lf_checker_rt::relocated(ROW_TABLE)
                        .wrapping_add((idx as i32 as u32).wrapping_mul(4)),
                );
                let q: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    rd32(rd32(row) + 0x0c) as usize,
                );
                if q(row) & 0xff == 6 {
                    ((worker + WOBJ_FLAGS) as *mut u32)
                        .write_unaligned(rd32(worker + WOBJ_FLAGS) & 0xffff_ffdf);
                }
            } else {
                ((worker + WOBJ_FLAGS) as *mut u32)
                    .write_unaligned(rd32(worker + WOBJ_FLAGS) & 0xffff_ffdf);
            }
            // Row loop: six entries, skipping non-positive ones, breaking
            // when the row callee answers zero.
            if rd32(ebp + 0x28) & 0x3c0 == 0x80 {
                let tidx = ((ebp + 0x2e) as *const i16).read_unaligned();
                let tobj = rd32(
                    lf_checker_rt::relocated(ROW_TABLE)
                        .wrapping_add((tidx as i32 as u32).wrapping_mul(4)),
                );
                slot12 = tobj;
                let mut k = 0u32;
                let mut broke = 0u32;
                let mut did_break = false;
                while k < 0x30 {
                    let e = rd32(
                        rd32(tobj + 0xcc).wrapping_add(k).wrapping_add(0x18c),
                    );
                    if (e as i32) > -1 {
                        let r: u32 =
                            lf_checker_rt::callee_thiscall!(19, u32, arg, e);
                        if r & 0xff == 0 {
                            broke = e;
                            did_break = true;
                            break;
                        }
                    }
                    k = k.wrapping_add(8);
                }
                if did_break {
                    let e = broke;
                    ((worker + 0x214) as *mut u32)
                        .write_unaligned(rd32(worker + 0x214) | 8);
                    let frame2: u32 = lf_checker_rt::callee_thiscall!(2, u32, arg);
                    let yobj = rd32(rd32(arg + 0x64) + 0xe4);
                    let p = rd32(rd32(frame2 + 0xec) + 0x84)
                        .wrapping_add(e.wrapping_shl(6));
                    lf_checker_rt::callee_thiscall!(20, u32, yobj, e, p);
                    let frame3: u32 = lf_checker_rt::callee_thiscall!(2, u32, arg);
                    let q = rd32(rd32(frame3 + 0xec) + 0x84)
                        .wrapping_add(e.wrapping_shl(6));
                    lf_checker_rt::callee_stdcall!(21, u32, e, q);
                    lf_checker_rt::callee_thiscall!(22, u32, yobj, 0u32);
                    lf_checker_rt::callee_thiscall!(23, u32, yobj, 0u32);
                    let frame4: u32 = lf_checker_rt::callee_thiscall!(2, u32, arg);
                    let r = rd32(rd32(frame4 + 0xd4) + e.wrapping_mul(4));
                    let di = ((r + 0x0e) as *const i16).read_unaligned() as i32;
                    let va = rd32(arg);
                    let f24: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(va + 0xe0) as usize);
                    let s2 = (di.wrapping_mul(5).wrapping_mul(16) as u32)
                        .wrapping_add(rd32(f24(arg) + 0x0c));
                    lf_checker_rt::callee_thiscall!(25, u32, s2);
                    let f26: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(rd32(arg) + 0xe0) as usize);
                    let di2 = di.wrapping_mul(0xe0) as u32;
                    let u = rd32(rd32(f26(arg) + 4));
                    ((s2 + 0x30) as *mut u32)
                        .write_unaligned(rd32(u + di2 + 0x20));
                    ((s2 + 0x34) as *mut u32)
                        .write_unaligned(rd32(u + di2 + 0x24));
                    ((s2 + 0x38) as *mut u32)
                        .write_unaligned(rd32(u + di2 + 0x28));
                    ((s2 + 0x3c) as *mut u32)
                        .write_unaligned(rd32(u + di2 + 0x2c));
                }
            }
            let tail_a = ((ebp + 0x44) as *const i16).read_unaligned();
            if tail_a != -1 && ((ebp + 0x40) as *const u8).read() == 0x3f {
                ((worker + WOBJ_TAG) as *mut u8).write(2);
                lf_checker_rt::callee_cdecl!(27, u32, worker, 0u32);
            } else {
                lf_checker_rt::callee_thiscall!(28, u32, worker, ebp, 0u32);
                lf_checker_rt::callee_cdecl!(27, u32, worker, 0u32);
            }
        }
        lf_checker_rt::callee_thiscall!(29, u32, worker);
        if ((this + GATEW) as *const u8).read() != 0 {
            let mut eax =
                (lf_checker_rt::global::<u32>(0x0159af24)).read_unaligned();
            let mut ecx = 0u32;
            if (eax as i32) >= 0 {
                loop {
                    eax = eax.wrapping_mul(2);
                    ecx = ecx.wrapping_add(1);
                    eax |= 1;
                    if (eax as i32) < 0 {
                        break;
                    }
                }
            }
            let ptr = lf_checker_rt::relocated(0x016156bc)
                .wrapping_sub(ecx.wrapping_mul(0x54));
            let x = f32::from_bits(rd32(ptr + 0x50));
            let qc = if x >= 0.0 {
                let q: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(rd32(rd32(worker) + 0x58) as usize);
                let x0 = core::hint::black_box(q(worker))
                    * core::hint::black_box(
                        (lf_checker_rt::global::<f32>(0x00fe8a24)).read_unaligned(),
                    );
                if x0 > x {
                    ptr.wrapping_add(0x30)
                } else {
                    ptr.wrapping_add(0x10)
                }
            } else {
                ptr.wrapping_add(0x10)
            };
            let a31: u32 = lf_checker_rt::callee_thiscall!(31, u32, qc, 0x10u32);
            (a31 as *mut u32).write_unaligned(worker);
            (a31.wrapping_add(4) as *mut u32).write_unaligned(slot12);
        }
        lf_checker_rt::callee_thiscall!(32, u32, slot12)
    }
});
