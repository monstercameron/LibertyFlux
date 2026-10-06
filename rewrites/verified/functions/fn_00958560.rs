// original: 0x00958560 task_list_consume (proposed)

/// Consume typed records from a task stream: validate, look up, classify and emit.
///
/// `limit` bounds the record lengths this call accepts: a record whose length
/// word exceeds it ends the call (compared unsigned). A null limit selects a
/// default from the initial callee instead.
///
/// The stream cursor is two globals: `STREAM_BASE` (the buffer) and `STREAM_OFF`
/// (the byte offset into it). Each iteration validates the record at the cursor
/// through the validator callee, dispatches on its opcode byte, and advances the
/// offset by a per-opcode constant. Consecutive records with the same opcode are
/// handled inline without redispatching. Opcodes are: 0 (end-of-stream logic),
/// 41 to 102 (typed records, one handler each), 155 and 157 (bare records), and
/// anything else (skipped by a measured advance from the sizing callee).
///
/// A typed record holds an opcode byte at `+0`, a done flag at `+1`, an unsigned
/// length word at `+4`, two argument words at `+8`/`+0x0c`, a row-kind byte at
/// `+0x14`, and feature-test bytes higher up. A record is acted on only when its
/// field word is not `FIELD_NONE` (-1, signedness is unobservable: equality only)
/// and its flag is clear; acting on it resolves the field through the lookup
/// callee, optionally filters through a virtual slot call, invokes the opcode's
/// own callee with the stream pointer, and sets the flag. Records with a tail
/// then classify the stream twice and, unless the row is absent (`ROW_NONE`,
/// compared for exact equality), emit a six-word event. Opcode 83 additionally
/// resolves an extra argument through its own callee; the limit is preserved
/// across that call.
///
/// Opcode 0 either ends the call (when the stream state byte is 2) or advances
/// to the next stream in the table, dividing the index by the table stride with
/// a signed divide (the divisor is a byte, so never negative; the signedness is
/// unobservable). The call also ends when the validator rejects a record or a
/// record overruns the limit, and returns immediately with the guard word when
/// the two guard globals disagree.
///
/// On entry with the world pointer set, two floats are pulled through the first
/// callee and five floats are copied to their globals; all are bit copies, no
/// arithmetic. The return value is the last value in the accumulator: the guard
/// word on the early path, the validator answer on a validation exit, the stream
/// index on an end-of-stream exit with an empty record, else whatever the last
/// handler or callee left.
///
/// Original: 0x00958560 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00958560(limit: u32) -> u32 {
    unsafe {
        const GUARD_A: u32 = 0x012088B4;
        const GUARD_B: u32 = 0x00F1C040;
        const WORLD_PTR: u32 = 0x0118D800;
        const STREAM_OFF: u32 = 0x01037810;
        const STREAM_BASE: u32 = 0x01037814;
        const STREAM_IDX: u32 = 0x01037818;
        const STREAM_TAB: u32 = 0x011F7000;
        const STREAM_STRIDE: u32 = 0x011F6FFD;
        const STREAM_STATE: u32 = 0x011F7018;
        const FLOAT_ARG: u32 = 0x011F7024;
        const PULLED_LO: u32 = 0x0139C234;
        const PULLED_HI: u32 = 0x0139C238;
        const POSE_0: u32 = 0x0169E790;
        const POSE_1: u32 = 0x0169E794;
        const POSE_2: u32 = 0x0169E798;
        const POSE_3: u32 = 0x0169E79C;
        const HEADING: u32 = 0x01048178;
        const REC_FLAG: u32 = 0x01;
        const REC_LEN: u32 = 0x04;
        const REC_ARG0: u32 = 0x08;
        const REC_FIELD: u32 = 0x0C;
        const ROW_KIND_OFF: u32 = 0x14;
        const VIRT_SLOT: u32 = 0xD0;
        const FIELD_NONE: u32 = 0xFFFF_FFFF;
        const FIELD_ABSENT: u32 = 0xFFFF_FFFE;
        const ROW_NONE: u32 = 0x00FF_FFFF;
        const STATE_END: u8 = 2;
        const WORLD_ARG_OFF: u32 = 0x10;
        const PULLED_OFF: u32 = 0x14;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 { unsafe { (a as *const u8).read() } }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)]
        unsafe fn virt_slot_d0(obj: u32) -> u32 {
            unsafe {
                let vtbl = rd32(obj);
                let slot = rd32(vtbl.wrapping_add(VIRT_SLOT));
                let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
                f(obj)
            }
        }


        let mut eax = unsafe { (lf_checker_rt::global::<u32>(GUARD_A) as *const u32).read() };
        if eax != unsafe { (lf_checker_rt::global::<u32>(GUARD_B) as *const u32).read() } {
            return eax;
        }
        eax = lf_checker_rt::callee_cdecl!(1, u32,);
        let edi_limit = if limit != 0 { limit } else { eax };
        let world = unsafe { (lf_checker_rt::global::<u32>(WORLD_PTR) as *const u32).read() };
        if world != 0 {
            let p1 = lf_checker_rt::callee_thiscall!(3, u32, world.wrapping_add(WORLD_ARG_OFF));
            let pulled_lo = unsafe { (p1 as *const u32).read_unaligned() };
            let p2 = lf_checker_rt::callee_thiscall!(3, u32, world.wrapping_add(WORLD_ARG_OFF));
            let pulled_hi = unsafe { (p2.wrapping_add(PULLED_OFF) as *const u32).read_unaligned() };
            unsafe {
                (lf_checker_rt::global::<u32>(PULLED_LO) as *mut u32).write_unaligned(pulled_lo);
                (lf_checker_rt::global::<u32>(PULLED_HI) as *mut u32).write_unaligned(pulled_hi);
                (lf_checker_rt::global::<u32>(POSE_0) as *mut u32)
                    .write_unaligned((world.wrapping_add(0x80) as *const u32).read_unaligned());
                (lf_checker_rt::global::<u32>(POSE_1) as *mut u32)
                    .write_unaligned((world.wrapping_add(0x84) as *const u32).read_unaligned());
                (lf_checker_rt::global::<u32>(POSE_2) as *mut u32)
                    .write_unaligned((world.wrapping_add(0x88) as *const u32).read_unaligned());
                (lf_checker_rt::global::<u32>(POSE_3) as *mut u32)
                    .write_unaligned((world.wrapping_add(0x8C) as *const u32).read_unaligned());
                (lf_checker_rt::global::<u32>(HEADING) as *mut u32)
                    .write_unaligned((world.wrapping_add(0x2C8) as *const u32).read_unaligned());
            }
        }
        let mut ebp = unsafe { (lf_checker_rt::global::<u32>(STREAM_OFF) as *const u32).read() };
        let mut ebx: u32;
        let mut esi: u32;
        let mut ecx_v: u32 = 0;
        let g32 = |va: u32| unsafe {
            (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned()
        };
        let g32w = |va: u32, v: u32| unsafe {
            (lf_checker_rt::global::<u32>(va) as *mut u32).write_unaligned(v)
        };
        let mut bl: u8 = 0;
        let gbase = || lf_checker_rt::relocated(STREAM_BASE);
        let goff = || lf_checker_rt::relocated(STREAM_OFF);
        let ret: u32 = 'outer: loop {
            esi = unsafe { (gbase() as *const u32).read_unaligned() }.wrapping_add(ebp);
            if unsafe { (esi as *const u8).read() } != 0 {
                eax = lf_checker_rt::callee_thiscall!(2, u32, esi);
                if eax & 0xFF == 0 { break 'outer eax; }
                if unsafe { (esi.wrapping_add(REC_LEN) as *const u32).read_unaligned() } > edi_limit {
                    break 'outer eax;
                }
                ebp = unsafe { (goff() as *const u32).read_unaligned() };
            }
            if bl != 0 { break 'outer eax; }
            ebx = unsafe { (gbase() as *const u32).read_unaligned() }.wrapping_add(ebp);
            let op = unsafe { (ebx as *const u8).read() };
            match op {
                0 => {
                    eax = unsafe { (lf_checker_rt::global::<u8>(STREAM_IDX) as *const u8).read() } as u32;
                    let state = unsafe {
                        (lf_checker_rt::relocated(STREAM_STATE).wrapping_add(eax) as *const u8).read()
                    };
                    if state == STATE_END {
                        bl = 1;
                    } else {
                        let stride = unsafe {
                            (lf_checker_rt::global::<u8>(STREAM_STRIDE) as *const u8).read()
                        } as u32;
                        eax = eax.wrapping_add(1);
                        let q = (eax as i32).wrapping_div(stride as i32);
                        let r = (eax as i32).wrapping_rem(stride as i32);
                        eax = q as u32;
                        ebp = 0;
                        unsafe {
                            (lf_checker_rt::global::<u32>(STREAM_OFF) as *mut u32).write_unaligned(0);
                            (lf_checker_rt::global::<u8>(STREAM_IDX) as *mut u8)
                                .write((r & 0xFF) as u8);
                            eax = (lf_checker_rt::relocated(STREAM_TAB)
                                .wrapping_add(((r & 0xFF) as u32).wrapping_mul(4)) as *const u32)
                                .read_unaligned();
                            (lf_checker_rt::global::<u32>(STREAM_BASE) as *mut u32).write_unaligned(eax);
                        }
                    }
                }
            41 => {
                // opcode 41 (jump-table slot 1, record 28 bytes)
                if rd8(esi) != 41 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax != 0 {
                            eax = lf_checker_rt::callee_thiscall!(10, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(28);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 41 { break 'chain; }
                }
            }
            42 => {
                // opcode 42 (jump-table slot 2, record 16 bytes)
                if rd8(esi) != 42 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax != 0 {
                            eax = lf_checker_rt::callee_thiscall!(11, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(16);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 42 { break 'chain; }
                }
            }
            43 => {
                // opcode 43 (jump-table slot 3, record 24 bytes)
                if rd8(esi) != 43 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax != 0 {
                            eax = lf_checker_rt::callee_thiscall!(12, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(24);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 43 { break 'chain; }
                }
            }
            44 => {
                // opcode 44 (jump-table slot 4, record 40 bytes)
                if rd8(esi) != 44 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(13, u32, ebx, esi, eax);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(40);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 44 { break 'chain; }
                }
            }
            45 => {
                // opcode 45 (jump-table slot 5, record 28 bytes)
                if rd8(esi) != 45 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(14, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(28);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 45 { break 'chain; }
                }
            }
            46 => {
                // opcode 46 (jump-table slot 6, record 44 bytes)
                if rd8(esi) != 46 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(15, u32, ebx, esi, eax);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(44);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 46 { break 'chain; }
                }
            }
            47 => {
                // opcode 47 (jump-table slot 7, record 64 bytes)
                if rd8(esi) != 47 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(16, u32, ebx, esi, eax);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(64);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 47 { break 'chain; }
                }
            }
            48 => {
                // opcode 48 (jump-table slot 8, record 44 bytes)
                if rd8(esi) != 48 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(17, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                    }
                }
                eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                if eax != 0 {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != ROW_NONE {
                        let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                        ebp = g32(STREAM_OFF);
                        eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                            rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                            1u32, row_kind, 0u32);
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                } else {
                    ebp = g32(STREAM_OFF);
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(44);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 48 { break 'chain; }
                }
            }
            49 => {
                // opcode 49 (jump-table slot 9, record 36 bytes)
                if rd8(esi) != 49 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(18, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 49 { break 'chain; }
                }
            }
            50 => {
                // opcode 50 (jump-table slot 10, record 36 bytes)
                if rd8(esi) != 50 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(19, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 50 { break 'chain; }
                }
            }
            51 => {
                // opcode 51 (jump-table slot 11, record 36 bytes)
                if rd8(esi) != 51 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(20, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 51 { break 'chain; }
                }
            }
            52 => {
                // opcode 52 (jump-table slot 12, record 44 bytes)
                if rd8(esi) != 52 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(21, u32, ebx, esi, eax);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                    }
                }
                eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                if eax != 0 {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != ROW_NONE {
                        let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                        ebp = g32(STREAM_OFF);
                        eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                            rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                            1u32, row_kind, 0u32);
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                } else {
                    ebp = g32(STREAM_OFF);
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(44);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 52 { break 'chain; }
                }
            }
            53 => {
                // opcode 53 (jump-table slot 13, record 36 bytes)
                if rd8(esi) != 53 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(22, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 53 { break 'chain; }
                }
            }
            54 => {
                // opcode 54 (jump-table slot 14, record 32 bytes)
                if rd8(esi) != 54 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(23, u32, ebx, esi, eax);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(32);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 54 { break 'chain; }
                }
            }
            55 => {
                // opcode 55 (jump-table slot 15, record 52 bytes)
                if rd8(esi) != 55 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(24, u32, ebx, esi, eax);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(52);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 55 { break 'chain; }
                }
            }
            56 => {
                // opcode 56 (jump-table slot 16, record 16 bytes)
                if rd8(esi) != 56 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(25, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(16);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 56 { break 'chain; }
                }
            }
            57 => {
                // opcode 57 (jump-table slot 17, record 32 bytes)
                if rd8(esi) != 57 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(26, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(32);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 57 { break 'chain; }
                }
            }
            58 => {
                // opcode 58 (jump-table slot 18, record 48 bytes)
                if rd8(esi) != 58 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    if rd8(ebx.wrapping_add(44)) & 2 != 0 {
                        eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    }
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(27, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(48);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 58 { break 'chain; }
                }
            }
            59 => {
                // opcode 59 (jump-table slot 19, record 36 bytes)
                if rd8(esi) != 59 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax != 0 {
                            eax = lf_checker_rt::callee_thiscall!(28, u32, ebx, esi, 0u32);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 59 { break 'chain; }
                }
            }
            60 => {
                // opcode 60 (jump-table slot 20, record 40 bytes)
                if rd8(esi) != 60 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open && field != 0 {
                    eax = lf_checker_rt::callee_thiscall!(29, u32, ebx, eax, 0u32);
                    wr8(ebx.wrapping_add(REC_FLAG), 1);
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(40);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 60 { break 'chain; }
                }
            }
            61 => {
                // opcode 61 (jump-table slot 21, record 16 bytes)
                if rd8(esi) != 61 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax != 0 {
                            eax = lf_checker_rt::callee_thiscall!(30, u32, ebx, esi);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(16);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 61 { break 'chain; }
                }
            }
            62 => {
                // opcode 62 (jump-table slot 22, record 28 bytes)
                if rd8(esi) != 62 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    esi = eax;
                    if esi != 0 {
                        eax = virt_slot_d0(esi);
                        if eax == 0 {
                            eax = lf_checker_rt::callee_thiscall!(31, u32, ebx, esi, eax);
                            wr8(ebx.wrapping_add(REC_FLAG), 1);
                        }
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(28);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 62 { break 'chain; }
                }
            }
            63 => {
                // opcode 63 (jump-table slot 23, record 24 bytes)
                if rd8(esi) != 63 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(32, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(24);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 63 { break 'chain; }
                }
            }
            64 => {
                // opcode 64 (jump-table slot 24, record 36 bytes)
                if rd8(esi) != 64 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(33, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 64 { break 'chain; }
                }
            }
            65 => {
                // opcode 65 (jump-table slot 25, record 40 bytes)
                if rd8(esi) != 65 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    if rd8(ebx.wrapping_add(36)) & 4 != 0 {
                        eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    }
                    eax = lf_checker_rt::callee_thiscall!(34, u32, ebx, eax);
                    wr8(ebx.wrapping_add(REC_FLAG), 1);
                    ebp = g32(STREAM_OFF);
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(40);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 65 { break 'chain; }
                }
            }
            66 => {
                // opcode 66 (jump-table slot 26, record 32 bytes)
                if rd8(esi) != 66 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(35, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(32);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 66 { break 'chain; }
                }
            }
            67 => {
                // opcode 67 (jump-table slot 27, record 48 bytes)
                if rd8(esi) != 67 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(36, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(48);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 67 { break 'chain; }
                }
            }
            68 => {
                // opcode 68 (jump-table slot 28, record 36 bytes)
                if rd8(esi) != 68 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(37, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 68 { break 'chain; }
                }
            }
            69 => {
                // opcode 69 (jump-table slot 29, record 44 bytes)
                if rd8(esi) != 69 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                ecx_v = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = 0;
                    if rd8(ebx.wrapping_add(40)) & 2 != 0 {
                        eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, ecx_v);
                    }
                    eax = lf_checker_rt::callee_thiscall!(38, u32, ebx, eax);
                    wr8(ebx.wrapping_add(REC_FLAG), 1);
                    ebp = g32(STREAM_OFF);
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(44);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 69 { break 'chain; }
                }
            }
            70 => {
                // opcode 70 (jump-table slot 30, record 32 bytes)
                if rd8(esi) != 70 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(39, u32, ebx);
                    wr8(ebx.wrapping_add(REC_FLAG), 1);
                    ebp = g32(STREAM_OFF);
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(32);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 70 { break 'chain; }
                }
            }
            71 => {
                // opcode 71 (jump-table slot 31, record 40 bytes)
                if rd8(esi) != 71 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(40, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(40);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 71 { break 'chain; }
                }
            }
            72 => {
                // opcode 72 (jump-table slot 32, record 16 bytes)
                if rd8(esi) != 72 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(41, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(16);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 72 { break 'chain; }
                }
            }
            73 => {
                // opcode 73 (jump-table slot 33, record 16 bytes)
                if rd8(esi) != 73 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(42, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(16);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 73 { break 'chain; }
                }
            }
            74 => {
                // opcode 74 (jump-table slot 34, record 28 bytes)
                if rd8(esi) != 74 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(43, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(28);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 74 { break 'chain; }
                }
            }
            75 => {
                // opcode 75 (jump-table slot 35, record 16 bytes)
                if rd8(esi) != 75 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(44, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(16);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 75 { break 'chain; }
                }
            }
            76 => {
                // opcode 76 (jump-table slot 36, record 44 bytes)
                if rd8(esi) != 76 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(45, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(44);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 76 { break 'chain; }
                }
            }
            77 => {
                // opcode 77 (jump-table slot 37, record 68 bytes)
                if rd8(esi) != 77 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(46, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(68);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 77 { break 'chain; }
                }
            }
            78 => {
                // opcode 78 (jump-table slot 38, record 44 bytes)
                if rd8(esi) != 78 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(47, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(44);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 78 { break 'chain; }
                }
            }
            79 => {
                // opcode 79 (jump-table slot 39, record 60 bytes)
                if rd8(esi) != 79 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    ecx_v = 0xFFFFFFFFu32;
                    let mut do_main = field == FIELD_ABSENT;
                    if field != FIELD_ABSENT {
                        eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                        ecx_v = eax;
                        if ecx_v != 0 { do_main = true; }
                    }
                    if do_main {
                        eax = lf_checker_rt::callee_thiscall!(48, u32, ebx, ecx_v);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(60);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 79 { break 'chain; }
                }
            }
            80 => {
                // opcode 80 (jump-table slot 40, record 76 bytes)
                if rd8(esi) != 80 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    if rd8(ebx.wrapping_add(51)) & 1 != 0 {
                        eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    }
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(49, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(76);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 80 { break 'chain; }
                }
            }
            81 => {
                // opcode 81 (jump-table slot 41, record 60 bytes)
                if rd8(esi) != 81 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(50, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(60);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 81 { break 'chain; }
                }
            }
            82 => {
                // opcode 82 (jump-table slot 42, record 48 bytes)
                if rd8(esi) != 82 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    ecx_v = 0u32;
                    if field != FIELD_ABSENT {
                        eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                        ecx_v = eax;
                    }
                    eax = lf_checker_rt::callee_thiscall!(51, u32, ebx, ecx_v);
                    wr8(ebx.wrapping_add(REC_FLAG), 1);
                    ebp = g32(STREAM_OFF);
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(48);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 82 { break 'chain; }
                }
            }
            83 => {
                // opcode 83 (jump-table slot 43, record 52 bytes)
                if rd8(esi) != 83 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    if rd8(ebx.wrapping_add(48)) & 1 != 0 {
                        eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    }
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(52, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let emit_a = rd32(ebx.wrapping_add(REC_ARG0));
                            let emit_b = rd32(ebx.wrapping_add(REC_FIELD));
                            eax = lf_checker_rt::callee_cdecl!(9, u32, 0u32);
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, emit_a, emit_b,
                                g32(STREAM_BASE).wrapping_add(ebp), 1u32, eax, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(52);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 83 { break 'chain; }
                }
            }
            84 => {
                // opcode 84 (jump-table slot 44, record 56 bytes)
                if rd8(esi) != 84 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open && field != FIELD_ABSENT {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(53, u32, ebx, eax, g32(FLOAT_ARG));
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(56);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 84 { break 'chain; }
                }
            }
            85 => {
                // opcode 85 (jump-table slot 45, record 44 bytes)
                if rd8(esi) != 85 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    if rd8(ebx.wrapping_add(40)) & 1 != 0 {
                        eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    }
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(54, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(44);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 85 { break 'chain; }
                }
            }
            86 => {
                // opcode 86 (jump-table slot 46, record 28 bytes)
                if rd8(esi) != 86 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(55, u32, ebx);
                    wr8(ebx.wrapping_add(REC_FLAG), 1);
                    ebp = g32(STREAM_OFF);
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(28);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 86 { break 'chain; }
                }
            }
            87 => {
                // opcode 87 (jump-table slot 47, record 36 bytes)
                if rd8(esi) != 87 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(56, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 87 { break 'chain; }
                }
            }
            88 => {
                // opcode 88 (jump-table slot 48, record 28 bytes)
                if rd8(esi) != 88 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(57, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(28);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 88 { break 'chain; }
                }
            }
            89 => {
                // opcode 89 (jump-table slot 49, record 28 bytes)
                if rd8(esi) != 89 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(58, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(28);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 89 { break 'chain; }
                }
            }
            90 => {
                // opcode 90 (jump-table slot 50, record 40 bytes)
                if rd8(esi) != 90 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(59, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(40);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 90 { break 'chain; }
                }
            }
            91 => {
                // opcode 91 (jump-table slot 51, record 32 bytes)
                if rd8(esi) != 91 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(60, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(32);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 91 { break 'chain; }
                }
            }
            92 => {
                // opcode 92 (jump-table slot 52, record 32 bytes)
                if rd8(esi) != 92 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(61, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(32);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 92 { break 'chain; }
                }
            }
            93 => {
                // opcode 93 (jump-table slot 53, record 40 bytes)
                if rd8(esi) != 93 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(62, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(40);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 93 { break 'chain; }
                }
            }
            94 => {
                // opcode 94 (jump-table slot 54, record 20 bytes)
                if rd8(esi) != 94 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(63, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(20);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 94 { break 'chain; }
                }
            }
            95 => {
                // opcode 95 (jump-table slot 55, record 24 bytes)
                if rd8(esi) != 95 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(64, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(24);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 95 { break 'chain; }
                }
            }
            96 => {
                // opcode 96 (jump-table slot 56, record 24 bytes)
                if rd8(esi) != 96 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(65, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(24);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 96 { break 'chain; }
                }
            }
            97 => {
                // opcode 97 (jump-table slot 57, record 44 bytes)
                if rd8(esi) != 97 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(66, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(44);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 97 { break 'chain; }
                }
            }
            98 => {
                // opcode 98 (jump-table slot 58, record 32 bytes)
                if rd8(esi) != 98 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(67, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(32);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 98 { break 'chain; }
                }
            }
            99 => {
                // opcode 99 (jump-table slot 59, record 56 bytes)
                if rd8(esi) != 99 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(68, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(56);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 99 { break 'chain; }
                }
            }
            100 => {
                // opcode 100 (jump-table slot 60, record 40 bytes)
                if rd8(esi) != 100 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(69, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(40);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 100 { break 'chain; }
                }
            }
            101 => {
                // opcode 101 (jump-table slot 61, record 52 bytes)
                if rd8(esi) != 101 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(70, u32, ebx, eax, 0u32);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                    }
                }
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(6, u32, ebx);
                        if eax != ROW_NONE {
                            let row_kind = rd8(ebx.wrapping_add(ROW_KIND_OFF)) as u32;
                            ebp = g32(STREAM_OFF);
                            eax = lf_checker_rt::callee_cdecl!(7, u32, rd32(ebx.wrapping_add(REC_ARG0)),
                                rd32(ebx.wrapping_add(REC_FIELD)), g32(STREAM_BASE).wrapping_add(ebp),
                                1u32, row_kind, 0u32);
                        } else {
                            ebp = g32(STREAM_OFF);
                        }
                    } else {
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(52);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 101 { break 'chain; }
                }
            }
            102 => {
                // opcode 102 (jump-table slot 62, record 36 bytes)
                if rd8(esi) != 102 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                eax = field;
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_cdecl!(4, u32, 1u32, field);
                    if eax != 0 {
                        eax = lf_checker_rt::callee_thiscall!(71, u32, ebx, eax);
                        wr8(ebx.wrapping_add(REC_FLAG), 1);
                        ebp = g32(STREAM_OFF);
                    }
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 102 { break 'chain; }
                }
            }
            154 => {
                // opcode 154 (jump-table slot 63, record 36 bytes)
                if rd8(esi) != 154 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(72, u32, ebx);
                    wr8(ebx.wrapping_add(REC_FLAG), 1);
                    ebp = g32(STREAM_OFF);
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(36);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 154 { break 'chain; }
                }
            }
            157 => {
                // opcode 157 (jump-table slot 64, record 32 bytes)
                if rd8(esi) != 157 { continue 'outer; }
                'chain: loop {
                if rd32(esi.wrapping_add(REC_LEN)) > edi_limit { continue 'outer; }
                let field = rd32(ebx.wrapping_add(REC_FIELD));
                let gate_open = field != FIELD_NONE && rd8(ebx.wrapping_add(REC_FLAG)) == 0;
                if gate_open {
                    eax = lf_checker_rt::callee_thiscall!(73, u32, ebx);
                    wr8(ebx.wrapping_add(REC_FLAG), 1);
                    ebp = g32(STREAM_OFF);
                }
                ebx = g32(STREAM_BASE);
                ebp = ebp.wrapping_add(32);
                ebx = ebx.wrapping_add(ebp);
                esi = ebx;
                g32w(STREAM_OFF, ebp);
                if rd8(esi) != 157 { break 'chain; }
                }
            }
                _ => {
                    eax = unsafe { (esi as *const u8).read() } as u32;
                    eax = lf_checker_rt::callee_cdecl!(8, u32, eax);
                    ebp = ebp.wrapping_add(eax);
                    unsafe {
                        (lf_checker_rt::global::<u32>(STREAM_OFF) as *mut u32).write_unaligned(ebp);
                    }
                }
            }
        };
        ret
    }
});
