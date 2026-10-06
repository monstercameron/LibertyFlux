// original: 0x00d6c060 filemem_table_sync (proposed)

/// Walk the file-memory table, syncing each entry with the render state.
///
/// `this` is the file-memory context (sub-object at `+4`, entry table at
/// `+0x9c` with an array pointer at `+0` and a 16-bit count at `+4`). Takes
/// no stack words; returns the selected index. Each entry carries a kind
/// byte at `+0`, a class byte at `+1`, flag bytes at `+8`/`+9`, a flag word
/// at `+0xc`, a key at `+0x14` and three position words at `+0x28`/`+0x2c`/
/// `+0x30`.
///
/// Behaviour: build a scratch control block (callee 1, through a frame
/// pointer; only its bytes at `+8`/`+9` are ever read back, both zero from
/// the real constructor) and clear the selection at `+0xf8` to -1. For each
/// table index in order: ask the locator (callee 2); when it names this
/// index, the entry key matches `+0xcc` and the sub-check (callee 3) reports
/// a zero low byte, the index is adopted (`+0xf8` becomes index - 1 when
/// nothing was selected yet) and the entry is skipped, otherwise the entry
/// is processed. Processing exchanges bytes `+8`/`+9` with the scratch
/// seeds (direction by flag bits 0 and 13 of `+0xc`, loading on the first
/// index), then compares position keys against the running key (callee 4,
/// UNSIGNED): the last entry needs key below-or-at, others need the key
/// below-or-at with the next key strictly above, else a fallback path runs
/// (first entry only, key at-or-below: republish fixed slots and reset the
/// kind to 7). A processed entry publishes kind - 1 (callee 5); a nonzero
/// flag byte `+8` looks a slot up in a data table (callee 6) and records
/// whether it is 0x14 at `+0x10a`, a zero byte runs two resetters (callees
/// 7 and 8). When the mode global is 1, a class dispatch maps class bytes
/// to small codes through a mapper (callee 9, last entry) or an inline
/// table (others) into a selector (callee 10); bit 2 of byte `+3` is
/// published to a flag global and two channels (callees 11 and 12) get the
/// `+0x24`/`+0x20` words. When the processed index is the selected one,
/// kinds 9 and 10 take extra paths: kind 9 with a changed selection stores
/// `+0x10` (callee 5) and pushes the three position words through a
/// transform (callees 5 and 13); a nonzero mode global clears state bits
/// (callee 5), a zero one either sets them for kind 10 (gated on two
/// sub-checks, callees 14 and 3) or clears them; kind 9 then reloads the
/// three position words (callee 5) into the entry. The epilogue republishes
/// slot 0 (callee 7) when the previous (`+0xfc`) and current selections are
/// both non-negative, differ, and disagree on byte `+8`; it stores the
/// current selection as previous and returns it.
///
/// All key comparisons are UNSIGNED (`jb`/`jae`/`ja`); the loop bound, the
/// index sign tests and the epilogue sign tests are SIGNED; the sub-check
/// answers are tested on the LOW byte only.
///
/// Original: 0x00d6c060 (thiscall, ECX = this, no stack words; callees 1, 2,
/// 3 and 5 take no stack words, callees 11 and 12 take one, callees 6, 7, 9
/// and 10 are cdecl of one, callee 13 takes three stack words, callees 4, 8
/// and 14 ignore their registers).
lf_checker_rt::export!(thiscall, rw_00d6c060(this: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x04;
        const TABLE_OFF: u32 = 0x9C;
        const SEL_OFF: u32 = 0xF8;
        const PREV_OFF: u32 = 0xFC;
        const CC_OFF: u32 = 0xCC;
        const FLAG14_OFF: u32 = 0x10A;
        const ARR_OFF: u32 = 0x00;
        const COUNT_OFF: u32 = 0x04;
        const KIND_OFF: u32 = 0x00;
        const CLASS_OFF: u32 = 0x01;
        const FLAG3_OFF: u32 = 0x03;
        const B8_OFF: u32 = 0x08;
        const B9_OFF: u32 = 0x09;
        const FLAGS_OFF: u32 = 0x0C;
        const W10_OFF: u32 = 0x10;
        const KEY_OFF: u32 = 0x14;
        const CH0_OFF: u32 = 0x20;
        const CH1_OFF: u32 = 0x24;
        const P0_OFF: u32 = 0x28;
        const P1_OFF: u32 = 0x2C;
        const P2_OFF: u32 = 0x30;
        const SINGLETON_A: u32 = 0x0103E498;
        const SINGLETON_B: u32 = 0x01176888;
        const SLOT_TABLE: u32 = 0x01056890;
        const GMODE: u32 = 0x01037720;
        const GCLASS: u32 = 0x011F70CC;
        const GFLAG_SRC: u32 = 0x01176D38;
        const GFLAG: u32 = 0x01176D39;
        const CAL_CTOR: u32 = 1;
        const CAL_LOCATE: u32 = 2;
        const CAL_SUB: u32 = 3;
        const CAL_KEY: u32 = 4;
        const CAL_PUB: u32 = 5;
        const CAL_SLOT: u32 = 6;
        const CAL_RESET: u32 = 7;
        const CAL_CLEAR: u32 = 8;
        const CAL_MAP: u32 = 9;
        const CAL_SELECT: u32 = 10;
        const CAL_CH1: u32 = 11;
        const CAL_CH0: u32 = 12;
        const CAL_XFORM: u32 = 13;
        const CAL_GATE: u32 = 14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let sing_a = lf_checker_rt::relocated(SINGLETON_A);
        let sing_b = lf_checker_rt::relocated(SINGLETON_B);
        let gmode = lf_checker_rt::relocated(GMODE);
        let gclass = lf_checker_rt::relocated(GCLASS);
        let gflag = lf_checker_rt::relocated(GFLAG);
        let gflag_src = lf_checker_rt::relocated(GFLAG_SRC);
        let slot_table = lf_checker_rt::relocated(SLOT_TABLE);
        // Scratch control block; the constructor's only bytes this function
        // reads back are at +8/+9 (both zero from the real constructor).
        let mut cframe: [u32; 3] = [0, 0, 0];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_CTOR, u32, &mut cframe as *mut u32 as u32
        );
        let seed8 = (cframe[2] & 0xFF) as u8;
        let seed9 = ((cframe[2] >> 8) & 0xFF) as u8;
        let table = rd32(this.wrapping_add(TABLE_OFF));
        wr32(this.wrapping_add(SEL_OFF), 0xFFFFFFFF);
        let count = rd16(table.wrapping_add(COUNT_OFF));
        let array = rd32(table.wrapping_add(ARR_OFF));
        let last = if count == 0 {
            0
        } else {
            rd32(array.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4))
        };
        // The seed bytes load once; the loop-back jumps past their reload,
        // so the shuffle leftovers carry into the next iteration.
        let mut f8 = seed8;
        let mut f9 = seed9;
        let mut idx: u32 = 0;
        while idx < count {
            let sub = rd32(this.wrapping_add(SUB_OFF));
            // Locator sync: adopt (index - 1) and skip the entry on a full
            // match with nothing selected yet; skip without adopting when
            // something already is; otherwise process the entry.
            let loci: u32 = lf_checker_rt::callee_thiscall!(CAL_LOCATE, u32, this);
            let mut process = true;
            if loci == idx {
                let e = rd32(array.wrapping_add(idx.wrapping_mul(4)));
                if rd32(e.wrapping_add(KEY_OFF)) == rd32(this.wrapping_add(CC_OFF)) {
                    let s: u32 = lf_checker_rt::callee_thiscall!(CAL_SUB, u32, sub);
                    if (s & 0xFF) == 0 {
                        if rd32(this.wrapping_add(SEL_OFF)) == 0xFFFFFFFF {
                            wr32(this.wrapping_add(SEL_OFF), idx.wrapping_sub(1));
                        }
                        process = false;
                    }
                }
            }
            if process {
                let edi = rd32(array.wrapping_add(idx.wrapping_mul(4)));
                if edi != 0 {
                    if idx == 0 || (rd8(edi.wrapping_add(FLAGS_OFF)) & 1) != 0 {
                        f8 = rd8(edi.wrapping_add(B8_OFF));
                    } else {
                        wr8(edi.wrapping_add(B8_OFF), f8);
                    }
                    if idx == 0 || ((rd32(edi.wrapping_add(FLAGS_OFF)) >> 13) & 1) != 0 {
                        f9 = rd8(edi.wrapping_add(B9_OFF));
                    } else {
                        wr8(edi.wrapping_add(B9_OFF), f9);
                    }
                }
                if edi == last {
                    let p: u32 = lf_checker_rt::callee_stdcall!(CAL_KEY, u32,);
                    if p < rd32(edi.wrapping_add(KEY_OFF)) {
                        // Fallback (first entry, key at-or-below).
                        let arr0 = rd32(rd32(this.wrapping_add(TABLE_OFF)));
                        if edi == rd32(arr0) {
                            let p3: u32 = lf_checker_rt::callee_stdcall!(CAL_KEY, u32,);
                            if p3 <= rd32(edi.wrapping_add(KEY_OFF)) {
                                wr8(gflag, rd8(gflag_src));
                                let _: u32 =
                                    lf_checker_rt::callee_thiscall!(CAL_CH1, u32, sing_b, 4);
                                let _: u32 =
                                    lf_checker_rt::callee_thiscall!(CAL_CH0, u32, sing_b, 4);
                                let a: u32 =
                                    lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                                wr32(a.wrapping_add(0x38C), 7);
                                let _: u32 = lf_checker_rt::callee_stdcall!(CAL_CLEAR, u32,);
                                let _: u32 = lf_checker_rt::callee_cdecl!(CAL_SELECT, u32, 0);
                            }
                        }
                    } else {
                        wr32(this.wrapping_add(SEL_OFF), idx);
                        let kind1 = (rd8(edi.wrapping_add(KIND_OFF)) as u32).wrapping_sub(1);
                        let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                        wr32(a.wrapping_add(0x38C), kind1);
                        let e8 = rd8(edi.wrapping_add(B8_OFF));
                        if e8 != 0 {
                            let tv = rd32(slot_table.wrapping_add((e8 as u32).wrapping_mul(4)));
                            let _: u32 = lf_checker_rt::callee_cdecl!(CAL_SLOT, u32, tv);
                            wr8(this.wrapping_add(FLAG14_OFF), if e8 == 0x14 { 1 } else { 0 });
                        } else {
                            let _: u32 = lf_checker_rt::callee_cdecl!(CAL_RESET, u32, 0);
                            let _: u32 = lf_checker_rt::callee_stdcall!(CAL_CLEAR, u32,);
                        }
                        if rd32(gmode) == 1 {
                            let m1: u32 = lf_checker_rt::callee_cdecl!(
                                CAL_MAP, u32, rd8(edi.wrapping_add(CLASS_OFF)) as u32
                            );
                            if m1 != rd32(gclass) {
                                let _: u32 = lf_checker_rt::callee_cdecl!(CAL_SELECT, u32, m1);
                            }
                            wr8(gflag, (rd8(edi.wrapping_add(FLAG3_OFF)) >> 2) & 1);
                            let c1v = rd32(edi.wrapping_add(CH1_OFF));
                            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_CH1, u32, sing_b, c1v);
                            let c0v = rd32(edi.wrapping_add(CH0_OFF));
                            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_CH0, u32, sing_b, c0v);
                        }
                    }
                } else {
                    let next = rd32(array.wrapping_add(idx.wrapping_mul(4)).wrapping_add(4));
                    let p1: u32 = lf_checker_rt::callee_stdcall!(CAL_KEY, u32,);
                    let p1_ok = p1 >= rd32(edi.wrapping_add(KEY_OFF));
                    let mut main = false;
                    if p1_ok {
                        let p2: u32 = lf_checker_rt::callee_stdcall!(CAL_KEY, u32,);
                        if p2 < rd32(next.wrapping_add(KEY_OFF)) {
                            main = true;
                        }
                    }
                    if !main {
                        let arr0 = rd32(rd32(this.wrapping_add(TABLE_OFF)));
                        if edi == rd32(arr0) {
                            let p3: u32 = lf_checker_rt::callee_stdcall!(CAL_KEY, u32,);
                            if p3 <= rd32(edi.wrapping_add(KEY_OFF)) {
                                wr8(gflag, rd8(gflag_src));
                                let _: u32 =
                                    lf_checker_rt::callee_thiscall!(CAL_CH1, u32, sing_b, 4);
                                let _: u32 =
                                    lf_checker_rt::callee_thiscall!(CAL_CH0, u32, sing_b, 4);
                                let a: u32 =
                                    lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                                wr32(a.wrapping_add(0x38C), 7);
                                let _: u32 = lf_checker_rt::callee_stdcall!(CAL_CLEAR, u32,);
                                let _: u32 = lf_checker_rt::callee_cdecl!(CAL_SELECT, u32, 0);
                            }
                        }
                    } else {
                        wr32(this.wrapping_add(SEL_OFF), idx);
                        let kind1 = (rd8(edi.wrapping_add(KIND_OFF)) as u32).wrapping_sub(1);
                        let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                        wr32(a.wrapping_add(0x38C), kind1);
                        let e8 = rd8(edi.wrapping_add(B8_OFF));
                        if e8 != 0 {
                            let tv = rd32(slot_table.wrapping_add((e8 as u32).wrapping_mul(4)));
                            let _: u32 = lf_checker_rt::callee_cdecl!(CAL_SLOT, u32, tv);
                            wr8(this.wrapping_add(FLAG14_OFF), if e8 == 0x14 { 1 } else { 0 });
                        } else {
                            let _: u32 = lf_checker_rt::callee_cdecl!(CAL_RESET, u32, 0);
                            let _: u32 = lf_checker_rt::callee_stdcall!(CAL_CLEAR, u32,);
                        }
                        if rd32(gmode) == 1 {
                            let ka = rd8(edi.wrapping_add(CLASS_OFF));
                            let g = rd32(gclass);
                            let mut v: u32 = 0;
                            let mut hit = false;
                            if ka == 0xE1 {
                                if g != 4 {
                                    v = 4;
                                    hit = true;
                                }
                            } else if ka == 0xC8 {
                                if g != 8 {
                                    v = 8;
                                    hit = true;
                                }
                            } else if ka == 0xAF {
                                if g != 7 {
                                    v = 7;
                                    hit = true;
                                }
                            } else if ka == 0x96 {
                                if g != 6 {
                                    v = 6;
                                    hit = true;
                                }
                            } else if ka == 0x7D {
                                if g != 5 {
                                    v = 5;
                                    hit = true;
                                }
                            }
                            if hit {
                                let _: u32 = lf_checker_rt::callee_cdecl!(CAL_SELECT, u32, v);
                            }
                            let mut v2: u32 = 0;
                            let mut hit2 = false;
                            if ka == 0x64 {
                                if g != 0 {
                                    v2 = 0;
                                    hit2 = true;
                                }
                            } else if ka == 0x4B {
                                if g != 1 {
                                    v2 = 1;
                                    hit2 = true;
                                }
                            } else if ka == 0x32 {
                                if g != 2 {
                                    v2 = 2;
                                    hit2 = true;
                                }
                            } else if ka == 0x19 {
                                if g != 3 {
                                    v2 = 3;
                                    hit2 = true;
                                }
                            }
                            if hit2 {
                                let _: u32 = lf_checker_rt::callee_cdecl!(CAL_SELECT, u32, v2);
                            }
                            wr8(gflag, (rd8(edi.wrapping_add(FLAG3_OFF)) >> 2) & 1);
                            let c1v = rd32(edi.wrapping_add(CH1_OFF));
                            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_CH1, u32, sing_b, c1v);
                            let c0v = rd32(edi.wrapping_add(CH0_OFF));
                            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_CH0, u32, sing_b, c0v);
                        }
                    }
                }
                if idx == rd32(this.wrapping_add(SEL_OFF)) {
                    let k2 = (rd8(edi.wrapping_add(KIND_OFF)) as u32).wrapping_sub(1);
                    let f10 = if k2 == 9 { 1u8 } else { 0 };
                    let f9 = if k2 == 8 { 1u8 } else { 0 };
                    if f9 != 0 && rd32(this.wrapping_add(PREV_OFF)) != rd32(this.wrapping_add(SEL_OFF))
                    {
                        let e10 = rd32(edi.wrapping_add(W10_OFF));
                        let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                        wr32(a.wrapping_add(0x2D4), e10);
                        let f28 = rd32(edi.wrapping_add(P0_OFF));
                        let f2c = rd32(edi.wrapping_add(P1_OFF));
                        let f30 = rd32(edi.wrapping_add(P2_OFF));
                        let b: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_XFORM, u32, b, f28, f2c, f30);
                    }
                    if rd32(gmode) != 0 {
                        let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                        wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) & 0xFD);
                        let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                        wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) & 0xFB);
                        if rd32(this.wrapping_add(SEL_OFF)) != rd32(this.wrapping_add(PREV_OFF)) {
                            let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                            wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) & 0xFE);
                        }
                        // The set-mode path rejoins at the loop end, past the
                        // kind-9 reload below.
                    } else {
                        if f10 != 0 {
                        let z: u32 = lf_checker_rt::callee_stdcall!(CAL_GATE, u32,);
                        if (z & 0xFF) != 0 {
                            let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                            wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) & 0xFD);
                            let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                            wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) & 0xFB);
                        } else {
                            let s: u32 = lf_checker_rt::callee_thiscall!(CAL_SUB, u32, sub);
                            if (s & 0xFF) == 0 {
                                let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                                wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) & 0xFD);
                                let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                                wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) & 0xFB);
                            } else {
                                let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                                wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) | 2);
                                let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                                wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) | 4);
                            }
                        }
                    } else {
                        let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                        wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) & 0xFD);
                        let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                        wr8(a.wrapping_add(0x398), rd8(a.wrapping_add(0x398)) & 0xFB);
                    }
                    if f9 != 0 {
                        let s: u32 = lf_checker_rt::callee_thiscall!(CAL_SUB, u32, sub);
                        if (s & 0xFF) != 0 {
                            let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                            let v0 = rd32(a.wrapping_add(0x2C0));
                            wr32(edi.wrapping_add(P0_OFF), v0);
                            let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                            wr32(edi.wrapping_add(P1_OFF), rd32(a.wrapping_add(0x2C4)));
                            let a: u32 = lf_checker_rt::callee_thiscall!(CAL_PUB, u32, sing_a);
                            wr32(edi.wrapping_add(P2_OFF), rd32(a.wrapping_add(0x2C8)));
                        }
                    }
                    }
                }
            }
            idx = idx.wrapping_add(1);
        }
        let fc = rd32(this.wrapping_add(PREV_OFF));
        let f8 = rd32(this.wrapping_add(SEL_OFF));
        if (fc as i32) >= 0 && (f8 as i32) >= 0 && fc != f8 {
            let arr = rd32(rd32(this.wrapping_add(TABLE_OFF)));
            let e1 = rd32(arr.wrapping_add(fc.wrapping_mul(4)));
            let e2 = rd32(arr.wrapping_add(f8.wrapping_mul(4)));
            if rd8(e1.wrapping_add(B8_OFF)) != rd8(e2.wrapping_add(B8_OFF)) {
                let _: u32 = lf_checker_rt::callee_cdecl!(CAL_RESET, u32, 0);
            }
        }
        let ret = rd32(this.wrapping_add(SEL_OFF));
        wr32(this.wrapping_add(PREV_OFF), ret);
        ret
    }
});
