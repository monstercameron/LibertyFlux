// original: 0x00946030 stream_slot_processor (proposed)

/// Process one stream slot of a two-slot object, by several paths.
///
/// `idx` is `(this[SLOT_SEL] + 1) % 2` (a SIGNED remainder, always 0 or 1
/// here) selecting sub-object `ebp = this + idx * SLOT_STRIDE`. When the
/// sub-object's flag byte at `+SUB_FLAG` is set, path P runs: with
/// `this[MODE_A]` clear and `+SUB_KIND` not 2, callee 0 runs and 0 is
/// returned; otherwise callee 1 runs on `(this[MODE_C], this[MODE_B],
/// arg1, this[ROW_TABLE + idx])` and its answer is returned. When the
/// pool gate `POOL_GATE` is set and the mode helpers answer
/// `(A != 0 || B != 0) && C == 0`, path P2 runs over pool row
/// `[ROW_CURSOR]`: an equal limit or a `0xFF` tag returns 0, otherwise
/// callees 5, 6 and 1 run, the masked row word lands in `+SUB_ACC` unless
/// `MODE_C` is set, the row is tagged `0xFF`, the cursor advances modulo
/// 4, and callee 1's answer is returned. Otherwise the main body runs: an
/// optional divide step stores a remainder into `DIV_CURSOR`; callee 7
/// seeds a mode byte; callee 8 (twice at most: direct, retry, or return
/// 2) fills two scratch words; callee 11 consumes them; a set-up flag and
/// mode 2 select an extended chain (clamped pool pick, a polled loop,
/// callees 15/16, an ST0 float through 17 into 18, callee 19); callee 20
/// may publish one table word; callee 1 runs again; and a second mode
/// gate either returns that answer or continues into a tail block (callee
/// 21 over one scratch byte, a 24-bit xor mix, callee 22) returning
/// callee 1's answer. The `js`/`jl` row gate and the `cmovg` clamp use SIGNED
/// comparisons; the `cmovb` clamp and the divide are unsigned.
///
/// Original: 0x00946030 (thiscall, `this` + two stack words).
lf_checker_rt::export!(thiscall, rw_00946030(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const SLOT_SEL: u32 = 0x1917;
        const SLOT_STRIDE: u32 = 0xbd0;
        const SUB_FLAG: u32 = 0xbca;
        const SUB_KIND: u32 = 0xbc0;
        const SUB_TUNE: u32 = 0xbc4;
        const SUB_ACC: u32 = 0x990;
        const MODE_A: u32 = 0x191f;
        const MODE_B: u32 = 0x191a;
        const MODE_C: u32 = 0x191b;
        const MODE_D: u32 = 0x1918;
        const MODE_E: u32 = 0x1920;
        const SETUP_FLAG: u32 = 0x1930;
        const ROW_BASE: u32 = 0x1910;
        const ROW_TABLE: u32 = 0x17a0;
        const ROW_WORD: u32 = 0x17a8;
        const ROW_TAG: u32 = 0x17ac;
        const ROW_SUB: u32 = 0x17ad;
        const ROW_STRIDE: u32 = 8;
        const ROW_CURSOR: u32 = 0x17c8;
        const ROW_LIMIT: u32 = 0x17cc;
        const ROW_MASK: u32 = 0xffffff;
        const POOL_GATE: u32 = 0x0128465c;
        const DIV_GATE: u32 = 0x011d7629;
        const DIV_CURSOR: u32 = 0x011d762c;
        const DIV_TABLE: u32 = 0x011d764c;
        const DIV_MOD: u32 = 0x39;
        const POOL_COUNT: u32 = 0x011d7680;
        const POOL_THIS: u32 = 0x011d7678;
        const TAB_BASE: u32 = 0x011d7504;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let sel = rd8(this.wrapping_add(SLOT_SEL)) as u32;
        let idx = sel.wrapping_add(1) % 2;
        let ebp = this.wrapping_add(idx.wrapping_mul(SLOT_STRIDE));
        if rd8(ebp.wrapping_add(SUB_FLAG)) != 0 {
            if rd8(this.wrapping_add(MODE_A)) == 0
                && rd8(ebp.wrapping_add(SUB_KIND)) != 2
            {
                let _: u32 = lf_checker_rt::callee_thiscall!(0, u32, ebp,);
                return 0;
            }
            let a = rd8(this.wrapping_add(MODE_C)) as u32;
            let b = rd8(this.wrapping_add(MODE_B)) as u32;
            let row = rd32(this.wrapping_add(ROW_TABLE).wrapping_add(idx.wrapping_mul(4)));
            return lf_checker_rt::callee_thiscall!(1, u32, ebp, row, arg1, b, a);
        }
        let mut arg_slot: u32 = 0;
        if (lf_checker_rt::global::<u8>(POOL_GATE)).read() != 0 {
            let a: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
            let mut enter = (a as u8) != 0;
            if !enter {
                let b: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
                enter = (b as u8) != 0;
            }
            if enter {
                let cc: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
                if (cc as u8) == 0 {
                    let cursor = rd32(this.wrapping_add(ROW_CURSOR));
                    if rd32(this.wrapping_add(ROW_LIMIT)) == cursor {
                        return 0;
                    }
                    let rowp = this.wrapping_add(cursor.wrapping_mul(ROW_STRIDE));
                    let tag = rd8(rowp.wrapping_add(ROW_TAG));
                    if tag == 0xff {
                        return 0;
                    }
                    let sub = rd8(rowp.wrapping_add(ROW_SUB));
                    let mixed = (cursor & 0xffffff00) | (sub as u32);
                    // id5 arg0 is the pushed ECX: only its low byte (the
                    // tag) is meaningful, so the contract masks it to 8 bits.
                    let r5: u32 = lf_checker_rt::callee_thiscall!(
                        5, u32, this, tag as u32, 0, sub as u32, 0
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, ebp, arg1, mixed, r5);
                    let wa = rd8(this.wrapping_add(MODE_C)) as u32;
                    let wb = rd8(this.wrapping_add(MODE_B)) as u32;
                    let word = rd32(rowp.wrapping_add(ROW_WORD)) & ROW_MASK;
                    let tab = rd32(
                        this.wrapping_add(ROW_TABLE).wrapping_add(idx.wrapping_mul(4)),
                    );
                    arg_slot = lf_checker_rt::callee_thiscall!(1, u32, ebp, tab, word, wb, wa);
                    if rd8(this.wrapping_add(MODE_C)) == 0 {
                        let word2 =
                            rd32(rowp.wrapping_add(ROW_WORD)) & ROW_MASK;
                        ((ebp.wrapping_add(SUB_ACC)) as *mut u32).write_unaligned(word2);
                    }
                    ((rowp.wrapping_add(ROW_TAG)) as *mut u8).write(0xff);
                    ((this.wrapping_add(ROW_CURSOR)) as *mut u32)
                        .write_unaligned(cursor.wrapping_add(1) & 3);
                    return arg_slot;
                }
            }
        }
        let mut bl_w: u32 = 2;
        let mut w14: u32 = 0;
        let mut e18: u32 = 0;
        // A divide run loads esi from the fresh cursor slot and skips
        // callees 7, 8 and 10 straight to the retry test.
        let mut esi: u32;
        if (lf_checker_rt::global::<u8>(DIV_GATE)).read() != 0
            && rd8(this.wrapping_add(MODE_D)) == 0
        {
            let cur = (lf_checker_rt::global::<u32>(DIV_CURSOR)).read();
            let tab = (lf_checker_rt::global::<u32>(DIV_TABLE)).read();
            let div = rd16(tab.wrapping_add(DIV_MOD)) as u32;
            e18 = tab.wrapping_add(cur.wrapping_add(8).wrapping_mul(8));
            let rem = tab.wrapping_rem(div);
            let _quot = tab.wrapping_div(div);
            let _ = _quot;
            (lf_checker_rt::global::<u32>(DIV_CURSOR)).write(rem);
            esi = e18;
        } else {
            let r7: u32 = lf_checker_rt::callee_thiscall!(7, u32, this, arg0);
            bl_w = (bl_w & 0xffffff00) | ((r7 as u8) as u32);
            esi = lf_checker_rt::callee_thiscall!(
                8, u32, this, &mut bl_w as *mut u32 as u32, &mut w14 as *mut u32 as u32
            );
            if rd8(this.wrapping_add(SETUP_FLAG)) != 0 && (bl_w as u8) == 2 {
                let k: u32 = lf_checker_rt::callee_cdecl!(10, u32,);
                if (k as u8) == 0 {
                    bl_w &= 0xffffff00;
                }
            }
        }
        if esi == 0 {
            bl_w = (bl_w & 0xffffff00) | 2;
            esi = lf_checker_rt::callee_thiscall!(
                9, u32, this, &mut bl_w as *mut u32 as u32, &mut w14 as *mut u32 as u32
            );
            if esi == 0 {
                return 2;
            }
        }
        let bl = bl_w as u8;
        let ea = rd8(this.wrapping_add(MODE_E)) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, ebp, bl_w, w14, esi, ea);
        ((this.wrapping_add(MODE_E)) as *mut u8).write(0);
        if rd8(this.wrapping_add(SETUP_FLAG)) != 0 && bl == 2 {
            let limit = (lf_checker_rt::global::<u32>(POOL_COUNT)).read();
            let t = limit.wrapping_sub(2);
            let clamped = if (t as i32) > 0 {
                if t < 10 { t } else { 10 }
            } else {
                0
            };
            e18 = clamped;
            let pool = lf_checker_rt::relocated(POOL_THIS);
            let q0: u32 = lf_checker_rt::callee_thiscall!(12, u32, pool,);
            if (q0 as u8) != 0 {
                let q1: u32 = lf_checker_rt::callee_thiscall!(12, u32, pool,);
                if q1 != 2 {
                    let full = rd8(this.wrapping_add(SLOT_SEL)) as u32;
                    let rowp = this.wrapping_add(full.wrapping_mul(SLOT_STRIDE));
                    if rd8(rowp.wrapping_add(SUB_FLAG)) != 0 {
                        let q2: u32 = lf_checker_rt::callee_thiscall!(12, u32, pool,);
                        if q2 == 1 {
                            let tune = rd32(rowp.wrapping_add(SUB_TUNE));
                            let n = rd32(this.wrapping_add(ROW_BASE))
                                .wrapping_add(1)
                                .wrapping_add(tune);
                            if (n as i32) < (limit as i32) {
                                esi = n;
                            } else {
                                esi = 0;
                            }
                        } else {
                            esi = lf_checker_rt::callee_thiscall!(13, u32, pool,);
                        }
                    } else {
                        esi = lf_checker_rt::callee_thiscall!(13, u32, pool,);
                    }
                } else {
                    loop {
                        let e: u32 = e18;
                        esi = lf_checker_rt::callee_thiscall!(13, u32, pool,);
                        let g: u32 =
                            lf_checker_rt::callee_thiscall!(14, u32, this, esi, e);
                        if (g as u8) == 0 {
                            break;
                        }
                    }
                }
            } else {
                loop {
                    let e: u32 = e18;
                    esi = lf_checker_rt::callee_thiscall!(13, u32, pool,);
                    let g: u32 = lf_checker_rt::callee_thiscall!(14, u32, this, esi, e);
                    if (g as u8) == 0 {
                        break;
                    }
                }
            }
            let s15: u32 = lf_checker_rt::callee_thiscall!(15, u32, pool, esi);
            if s15 == 0 {
                return 2;
            }
            let s16: u32 = lf_checker_rt::callee_thiscall!(16, u32, pool, esi);
            let f: f32 = lf_checker_rt::callee_thiscall!(17, f32, pool, esi, s16);
            let s18: u32 =
                lf_checker_rt::callee_thiscall!(18, u32, pool, esi, f.to_bits());
            let scratch0: u32 = 0;
            let _: u32 = lf_checker_rt::callee_thiscall!(19, u32, ebp, scratch0, esi, s18);
        }
        let r20: u32 = lf_checker_rt::callee_thiscall!(20, u32, this, bl_w);
        if r20 != 0 {
            let p = (lf_checker_rt::global::<u32>(TAB_BASE)).read();
            let v = rd32(p.wrapping_add((bl as u32).wrapping_mul(5)).wrapping_add(0xc));
            ((r20.wrapping_add(0xb)) as *mut u32).write_unaligned(v.wrapping_add(arg0));
        }
        let wa = rd8(this.wrapping_add(MODE_C)) as u32;
        let wb = rd8(this.wrapping_add(MODE_B)) as u32;
        let tab = rd32(this.wrapping_add(ROW_TABLE).wrapping_add(idx.wrapping_mul(4)));
        arg_slot = lf_checker_rt::callee_thiscall!(1, u32, ebp, tab, arg1, wb, wa);
        let d: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        let mut cont = (d as u8) != 0;
        if !cont {
            let e: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
            if (e as u8) == 0 {
                return arg_slot;
            }
            cont = true;
        }
        if cont {
            let ff: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
            if (ff as u8) == 0 {
                return arg_slot;
            }
        }
        let mut e4arr = [rd8(ebp.wrapping_add(SUB_KIND)), 0, 0, 0];
        let tune_b = rd8(ebp.wrapping_add(SUB_TUNE)) as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            21, u32, (&mut e4arr[1] as *mut u8) as u32, tune_b
        );
        e4arr[2] = rd8(this.wrapping_add(MODE_D));
        let full = rd8(this.wrapping_add(SLOT_SEL)) as u32;
        let acc = rd32(
            this.wrapping_add(full.wrapping_mul(SLOT_STRIDE)).wrapping_add(SUB_ACC),
        );
        let mut e8slot: u32 = idx;
        e8slot ^= acc;
        e8slot &= ROW_MASK;
        e8slot ^= idx;
        e8slot = (idx ^ acc) & ROW_MASK ^ idx;
        let _: u32 = lf_checker_rt::callee_cdecl!(22, u32, &mut e8slot as *mut u32 as u32);
        let _ = e18;
        arg_slot
    }
});
