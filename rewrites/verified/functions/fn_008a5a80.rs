// original: 0x008A5A80 aud_blend_voice_update (proposed)
/// Update one blended audio voice: pick its two blend operands, then either
/// accumulate into the running mix (kind 2) or resolve and commit a new voice.
///
/// `this` points to the voice object, `arg0` is forwarded to the resolver
/// callees. Byte `+0x40` is the table row, byte `+0x48` the voice selector
/// (`0xFF` absent). The selector resolves through the audio table
/// (`table + row*0x6F40 + 0x6F10` plus `stride * selector`); a zero
/// resolution returns 0. Only AL is meaningful on return.
///
/// The two operands come from pointer-or-immediate pairs: `+0xD8`/`+0xDC`
/// and `+0xD4`/`+0xE4` (each a pointer, or the immediate float when null).
/// The resolved voice's word at `+6` selects the path.
///
/// Kind 2 accumulates: the voice is resolved through callee 1 and polled
/// through callee 2 (low byte tested; nonzero returns 1), then `*p +=
/// operand2` where `p` is the mix pointer at `+0xCC`. A limit check follows:
/// when operand2 is below the low constant the voice passes if `*p` is above
/// `operand1 - high`, otherwise if `operand1 + high` is above `*p` (strictly
/// above in both cases; unordered counts as below for the first test and as
/// failure for the second). Then bit 3 of `+0x39` must be clear; the slot is
/// poked through callee 3 and staged through callee 4 with `+0xC8`, 0 and
/// `this+0xB0`, returning 1.
///
/// Any other kind resolves a chain: callee 1 is called up to three times and
/// the chain continues when the second answer's word at `+6` is 0 or the
/// third's is 1 (a null first answer skips to the third). Then bit 5 of
/// `+0x39` is patched into the low byte of operand2, the sign-extended word
/// at `+0x3C` is keyed through callee 5, and callee 6 commits with the key,
/// the patched operand2 and 0: answer 1 continues through callee 1 and
/// callee 7 with `arg0` and returns 1, any other answer returns whether it
/// is 0.
///
/// Float operations run in the original's operand order through pinned
/// helpers; all integer comparisons are unsigned or equality.
///
/// Original: 0x008A5A80 (thiscall, one stack word).
export!(thiscall, rw_008a5a80(this: u32, arg0: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f10;
        const ABSENT: u8 = 0xFF;

        let t = this as *const u8;
        let rd8 = |off: u32| t.add(off as usize).read();
        let rd32 = |off: u32| (t.add(off as usize) as *const u32).read_unaligned();
        let rdf = |off: u32| f32::from_bits(rd32(off));

        let sel = rd8(0x48);
        if sel == ABSENT {
            return 0;
        }
        let row = rd8(0x40);
        let table = global::<u32>(0x115d988).read();
        let stride = global::<u32>(0x115d964).read();
        let cell = ((table
            .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_BIAS)) as *const u32)
            .read_unaligned();
        let obj = stride.wrapping_mul(sel as u32).wrapping_add(cell);
        if obj == 0 {
            return 0;
        }

        let p1 = rd32(0xD8);
        let op1 = if p1 == 0 {
            rdf(0xDC)
        } else {
            f32::from_bits((p1 as *const u32).read_unaligned())
        };
        let p2 = rd32(0xD4);
        let op2 = if p2 == 0 {
            rdf(0xE4)
        } else {
            f32::from_bits((p2 as *const u32).read_unaligned())
        };

        let kind = ((obj.wrapping_add(6)) as *const u16).read_unaligned();
        if kind == 2 {
            let f: u32 = callee_thiscall!(1, u32, this, 0);
            let poll: u32 = callee_thiscall!(2, u32, f, arg0);
            if (poll & 0xFF) != 0 {
                return 1;
            }
            let low = f32::from_bits(global::<u32>(0xfe8628).read());
            let high = f32::from_bits(global::<u32>(0xfe868c).read());
            let p = rd32(0xCC);
            let acc = f32::from_bits((p as *const u32).read_unaligned());
            let acc2 = fadd(acc, op2);
            (p as *mut u32).write_unaligned(acc2.to_bits());
            // jb after comiss(op2, low): taken when op2 < low or unordered.
            let below = !(op2 >= low);
            let pass = if !below {
                fadd(op1, high) > acc2
            } else {
                acc2 > fsub(op1, high)
            };
            if !pass {
                return 0;
            }
            if (rd8(0x39) & 8) != 0 {
                return 0;
            }
            let _: u32 = callee_thiscall!(3, u32, this, 0);
            let _: u32 =
                callee_thiscall!(4, u32, this, rd32(0xC8), 0, this.wrapping_add(0xB0));
            return 1;
        }

        let commit = |op2v: f32| {
            let flag = ((rd8(0x39) >> 5) & 1) as u32;
            let patched = (op2v.to_bits() & 0xFFFF_FF00) | flag;
            let sext = (t.add(0x3C) as *const i16).read_unaligned() as i32 as u32;
            let key: u32 = callee_cdecl!(5, u32, sext);
            let f4: u32 = callee_thiscall!(1, u32, this, 0);
            let ans: u32 = callee_thiscall!(6, u32, f4, key, patched, 0);
            if ans != 1 {
                return (ans == 0) as u32;
            }
            let f5: u32 = callee_thiscall!(1, u32, this, 0);
            let _: u32 = callee_thiscall!(7, u32, f5, arg0);
            1
        };

        let f1: u32 = callee_thiscall!(1, u32, this, 0);
        if f1 != 0 {
            let f2: u32 = callee_thiscall!(1, u32, this, 0);
            let w = ((f2.wrapping_add(6)) as *const u16).read_unaligned();
            if w == 0 {
                return commit(op2);
            }
        }
        let f3: u32 = callee_thiscall!(1, u32, this, 0);
        let w3 = ((f3.wrapping_add(6)) as *const u16).read_unaligned();
        if w3 != 1 {
            return 0;
        }
        commit(op2)
    }
});
