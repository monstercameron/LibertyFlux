#![allow(non_snake_case, unused, non_upper_case_globals)]


#[allow(dead_code)]
mod k_0056e520 {
// original: 0x0056e520 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_118, player_schema::LeaderboardInfo, 10>::vf8
/// Width class of `keys[index]`: 4, 8, or NONE (0).
///
/// Fetches the tables for LEADERBOARD_ID and returns NONE when the fetch
/// fails. Otherwise classifies `keys[index]` (keys table at frame word 5)
/// through the shared classify step, which takes the key in ECX. A class of
/// -1, class 0, or anything past 5 means NONE; classes 1..5 map through
/// [4, 8, 8, 0, 4] (the original's five-entry jump table, re-expressed).
///
/// stdcall with one stack argument; no `this`, no globals, no caller-visible
/// writes.
lf_checker_rt::export!(stdcall, rw_0056e520(index: u32) -> u32 {
    unsafe {
        const NONE: u32 = 0;
        const BAD_CLASS: u32 = 0xFFFF_FFFF;
        const KEYS_WORD: usize = 5;
        const CLASSIFY_CALLEE: u32 = 2;
        const LEADERBOARD_ID: u32 = 321;
        const FETCH_CALLEE: u32 = 1;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32, LEADERBOARD_ID,
            frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NONE;
        }
        let keys = frame[KEYS_WORD];
        let key = (keys.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        let class = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, key);
        if class == BAD_CLASS {
            return NONE;
        }
        let slot = class.wrapping_sub(1);
        if slot > 4 {
            return NONE;
        }
        const MAP: [u32; 5] = [4u32, 8, 8, 0, 4];
        MAP[slot as usize]
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0066f8c0 {
// original: 0x0066F8C0 tok_skip_to_eol (proposed)

/// Consume input up to and including the next line feed.
///
/// `this` points to the tokenizer. Characters are consumed with the same
/// reader as the next-character routine (pushback count `+0x18` and buffer
/// `+0x1C`, read sign-extended, else the stream at `+0x0C`, else the read
/// callee): a line feed increments the line counter at `+0x08` and ends the
/// scan, -1 (a 0xFF pushback byte) ends it too, and a read failure returns
/// the callee's answer straight away. The returned value is the terminating
/// character (normally a line feed).
///
/// Original: 0x0066F8C0 (thiscall, no stack arguments, 1 call).
lf_checker_rt::export!(thiscall, rw_0066f8c0(this: u32) -> u32 {
    unsafe {
        const LINE: u32 = 0x08;
        const STREAM: u32 = 0x0c;
        const COUNT: u32 = 0x18;
        const PBUF: u32 = 0x1c;
        const S_BUF: u32 = 0x08;
        const S_POS: u32 = 0x10;
        const S_END: u32 = 0x14;
        const READ_CALLEE: u32 = 1;
        const LF: u32 = 0x0a;
        const EOF: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn slot_addr(s: &mut u32) -> u32 {
            unsafe { (s as *mut u32) as u32 }
        }

        loop {
            let c: u32;
            let count = rd32(this + COUNT);
            if count != 0 {
                let top = count.wrapping_sub(1);
                wr32(this + COUNT, top);
                c = ((this + PBUF + top) as *const i8).read_unaligned() as i32 as u32;
            } else {
                let stream = rd32(this + STREAM);
                let pos = rd32(stream + S_POS);
                if (pos as i32) < (rd32(stream + S_END) as i32) {
                    let buf = rd32(stream + S_BUF);
                    c = rd8(buf + pos);
                    wr32(stream + S_POS, pos.wrapping_add(1));
                } else {
                    let mut slot: u32 = 0;
                    let byte_ptr = (slot_addr(&mut slot)).wrapping_add(3);
                    let n = lf_checker_rt::callee_thiscall!(READ_CALLEE, u32, stream, byte_ptr, 1);
                    if n != 1 {
                        return n;
                    }
                    c = slot >> 24;
                }
            }
            if c == LF {
                wr32(this + LINE, rd32(this + LINE).wrapping_add(1));
                return c;
            }
            if c == EOF {
                return c;
            }
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00598630 {
// original: 0x00598630 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_2, player_schema::LeaderboardInfo, 10>::vf6

/// Find a column id in this leaderboard's column table (index lookup).
///
/// `this` (ECX, thiscall) is the leaderboard-info object; it is never read:
/// the board is identified solely by `BOARD_ID`. `want` is the column id to
/// find. The schema callee (fastcall: ECX = board id, EDX = out-struct) is
/// asked for the table; it reports success in AL and fills `COUNT_OFF`
/// (element count, signed) and `ARRAY_OFF` (pointer to the id array).
///
/// On success the array is scanned linearly and the first index whose element
/// equals `want` is returned. `NOT_FOUND` (-1) is returned when the callee
/// reports failure, when the count is not positive, or when no element
/// matches. A negative count takes the not-found path without reading the
/// array.
///
/// Original: 0x00598630 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00598630(_this: u32, want: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x155;
        const CALLEE_SCHEMA: u32 = 1;
        const COUNT_OFF: usize = 0x0c;
        const ARRAY_OFF: usize = 0x10;
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut schema = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_SCHEMA,
            u32,
            BOARD_ID,
            schema.as_mut_ptr() as u32
        );
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let count = schema[COUNT_OFF / 4] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let items = schema[ARRAY_OFF / 4] as *const u32;
        let mut i = 0i32;
        while i < count {
            if *items.offset(i as isize) == want {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00cadf00 {
// original: 0x00cadf00 motion_output_update (proposed)
/// Fill a motion output record from a queried pose, gated by speed and tag.
///
/// `a` points at the source (handle at `+0xD68`, velocity at `+0xD70`/`+0xD74`/
/// `+0xD78`); `out` takes 16 bytes, `out2` a type word. Queries a four-dword
/// pose for the handle into scratch (intercepted callee 1, thiscall/1 with the
/// scratch address as its stack argument; its contents are scripted and
/// compared through the copy). When `flag` is zero returns 0 at once.
/// Otherwise stores 2 into `out2`, then compares the squared speed
/// (`x*x + y*y + z*z` in that association, pinned) against the limit `0.01`
/// from `0x00FE8710`: below the limit the low byte of `byteflag` decides
/// (zero returns 0, nonzero continues); at or above, or NaN, continues.
/// Then reads the tag at the handle's `+0x00` masked with 7: 2 or 3 copies
/// the queried pose into `out`, anything else zeroes `out[0..8]`. Returns 1.
/// Original is cdecl(`a`, `out`, `flag`, `out2`, `byteflag`).
lf_checker_rt::export!(cdecl, rw_00cadf00(a: u32, out: u32, flag: u32, out2: u32, byteflag: u32) -> u32 {
    unsafe {
        const QUERY: u32 = 1;
        const HANDLE: u32 = 0xD68;
        const VEL: u32 = 0xD70;
        const LIMIT: u32 = 0x00FE_8710;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn dw(base: u32, off: u32) -> u32 {
            unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
        }
        let mut buf = [0u32; 4];
        let _: u32 = lf_checker_rt::callee_thiscall!(QUERY, u32,
            dw(a, HANDLE), buf.as_mut_ptr() as u32);
        if flag == 0 {
            return 0;
        }
        (out2 as *mut u32).write_unaligned(2);
        let x = f32::from_bits(dw(a, VEL));
        let y = f32::from_bits(dw(a, VEL.wrapping_add(4)));
        let z = f32::from_bits(dw(a, VEL.wrapping_add(8)));
        let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
        let limit = f32::from_bits(
            (lf_checker_rt::global::<u32>(LIMIT) as *const u32).read());
        if len2 < limit {
            if byteflag & 0xFF == 0 {
                return 0;
            }
        }
        let tag = dw(dw(a, HANDLE), 0) & 7;
        if tag == 2 || tag == 3 {
            for k in 0..4u32 {
                ((out.wrapping_add(k * 4)) as *mut u32).write_unaligned(buf[k as usize]);
            }
        } else {
            for k in 0..3u32 {
                ((out.wrapping_add(k * 4)) as *mut u32).write_unaligned(0);
            }
        }
        1
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00542e10 {
// original: 0x00542E10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_4, player_schema::LeaderboardInfo, 10>::vf8
/// Byte size of one leaderboard column value by row index.
///
/// Calls the leaderboard-info callee (fastcall slot 0) with this board's
/// numeric id in ECX and a scratch info block in EDX. When the callee
/// reports failure the result is 0. Otherwise the info block's
/// value-array pointer (at +0x14) is read, the row at `index` is passed
/// in ECX to the kind callee (thiscall slot 1), and the returned kind is
/// mapped through a size table: kind 1 takes 4 bytes, kinds 2 and 3 take
/// 8, kind 5 takes 4, and anything else (including kind 4 and -1) takes
/// 0. The object pointer in ECX is unused. Original is thiscall with one
/// stack word; the table is a jump table in the original.
lf_checker_rt::export!(thiscall, rw_00542e10(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADER_ID: u32 = 0xA4;
        const INFO_VALUES: u32 = 0x14;
        const INFO_CALLEE: u32 = 0;
        const KIND_CALLEE: u32 = 1;
        const SIZE_OF_KIND: [u32; 5] = [4, 8, 8, 0, 4];
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADER_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let values = info[(INFO_VALUES / 4) as usize];
        let v = (values as *const u32).add(index as usize).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, v);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        let t = kind.wrapping_sub(1);
        if t > 4 {
            return 0;
        }
        SIZE_OF_KIND[t as usize]
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0058e950 {
// original: 0x0058E950 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_236, player_schema::LeaderboardInfo, 10>::vf8
/// Leaderboard element width: classify the key at position `index`.
/// Calls the leaderboard helper (id 0x1c4) for the key table (record
/// word 5, `+0x14`), reads the key at `index`, and passes it to the
/// classifier helper. The classifier's answer minus one selects 4, 8, 8, 0
/// or 4; any other answer (including failure of either helper) yields 0.
/// Original: 0x0058E950 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0058e950(index: u32) -> u32 {
    unsafe {
        const RACE_ID: u32 = 0x1c4;
        const KEYS_WORD: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 6];
        let ok = lf_checker_rt::callee_fastcall!(1, u32, RACE_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let elem = rd32(info[KEYS_WORD].wrapping_add(index.wrapping_mul(4)));
        let v = lf_checker_rt::callee_thiscall!(2, u32, elem);
        if v == 0xFFFF_FFFF {
            return 0;
        }
        match v.wrapping_sub(1) {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0058e4b0 {
// original: 0x0058E4B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_235, player_schema::LeaderboardInfo, 10>::vf7
/// Leaderboard column read: element INDEX of this board's value array, or -1.
///
/// Arguments: INDEX is the single stack word. ECX is ignored (overwritten
/// with the board id). Calls the shared lookup helper (fastcall: ECX = board
/// id 0x1C3, EDX = frame buffer) and tests only AL. On success the array
/// pointer is at buffer+0x10 and the result is array[INDEX] with no bounds
/// check, so a wild index faults exactly like the original. AL == 0 returns
/// 0xFFFFFFFF without reading memory.
/// Original: 0x0058E4B0 (stdcall, one stack word; ignores ECX).

lf_checker_rt::export!(stdcall, rw_0058E4B0(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x1C3;
        const ARRAY_OFF: u32 = 0x10;
        let mut out = [0u32; 8];
        let r: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if (r & 0xFF) == 0 {
            return 0xFFFFFFFF;
        }
        let arr = out[(ARRAY_OFF / 4) as usize];
        ((arr.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned()
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0097fcf0 {
// original: 0x0097FCF0 MOLOTOV_FUSE_FIRE_LOOP_IN_HAND (symbols, low confidence)

/// Refresh one ped's task/effect state and (re)create its missing pieces.
///
/// `this` points to the ped-task object. The object holds four nullable
/// phase slots (`+0x14/+0x18/+0x20/+0x198`, each a small record with live
/// fields at `+0xa4/+0xa8/+0xac`), a pointer to a larger state object at
/// `+0x120`, several flags and timers, and one float at `+0x40` that is
/// rescaled on every call. The function walks the slots in a fixed order:
/// for each slot it either runs the slot's effect chain directly (slot
/// non-null) or runs a creation sequence (a family of `0xE3...` calls) that
/// fills the slot and then runs the chain. Effect chains mix scripted float
/// stages (values arrive through the x87 return channel of small helpers
/// and are combined with `comiss`-gated min/max blends) with record
/// updates. Two globals cache created descriptors (`0x1231738/0x1231740`,
/// guarded by flag bits in `0x123173c`). Three early gates at the top (a
/// mode global, a version pair, a state id) skip everything and run a short
/// teardown path instead.
///
/// Calling convention: thiscall, no stack arguments, returns in `eax` (the
/// last helper's value on the teardown paths, zero on most others). The
/// original keeps a 0xD8-byte aligned frame; the slots below mirror it:
/// temps at `0x08..0x14`, a three-word descriptor triple at `0x18`, a small
/// scratch struct at `0x24`, a second scratch struct at `0x3c`, and the
/// main record at `0x84` (12 words, the bytes at `+0x2c..0x2e` are flag
/// bits set along the way). Callee ids are the checker's, not the game's.
lf_checker_rt::export!(thiscall, rw_0097FCF0(this: u32) -> u32 {
    unsafe {
        // ---- globals (file VAs) ----
        const G_COOKIE: u32 = 0x01057FB4;
        const G_MODE: u32 = 0x011F7060;
        const G_VERA: u32 = 0x012088B4;
        const G_VERB: u32 = 0x00F1C040;
        const G_STATE: u32 = 0x01037720;
        const G_TIMER: u32 = 0x011735B4;
        const G_TIMERF: u32 = 0x011735BC;
        const G_MUL: u32 = 0x0115D968;
        const G_TBL: u32 = 0x0115D988;
        const G_ARR: u32 = 0x01231360;
        const G_D20: u32 = 0x01231738;
        const G_FLAGS: u32 = 0x0123173C;
        const G_D198: u32 = 0x01231740;
        const F_8670: u32 = 0x00FE8670;
        const F_88E8: u32 = 0x00FE88E8;
        const F_8BB0: u32 = 0x00FE8BB0;
        const F_8B38: u32 = 0x00FE8B38;
        const F_8AB8: u32 = 0x00FE8AB8;
        const F_8C10: u32 = 0x00FE8C10;
        const F_8B00: u32 = 0x00FE8B00;
        const F_88B0: u32 = 0x00FE88B0;
        const F_E8D714: u32 = 0x00E8D714;
        // ---- this + field offsets ----
        const T_PTR8: u32 = 0x08;
        const T_PH14: u32 = 0x14;
        const T_PH18: u32 = 0x18;
        const T_PH20: u32 = 0x20;
        const T_F40: u32 = 0x40;
        const T_IDX: u32 = 0x7C;
        const T_OBJ: u32 = 0x120;
        const T_F129: u32 = 0x129;
        const T_P130: u32 = 0x130;
        const T_T138: u32 = 0x138;
        const T_W144: u32 = 0x144;
        const T_T170: u32 = 0x170;
        const T_B174: u32 = 0x174;
        const T_PH198: u32 = 0x198;
        // ---- obj+0x120 field offsets ----
        const O_VT: u32 = 0x00;
        const O_B218: u32 = 0x218;
        const O_B219: u32 = 0x219;
        const O_W224: u32 = 0x224;
        const O_B26C: u32 = 0x26C;
        const O_SUB: u32 = 0x2B0;
        const O_W2C4: u32 = 0x2C4;
        const O_S780: u32 = 0x780;
        const O_PB30: u32 = 0xB30;
        // ---- frame slots (mirror of the original's aligned frame) ----
        const F_TA: usize = 0x08;
        const F_TB: usize = 0x0C;
        const F_TC: usize = 0x10;
        const F_TD: usize = 0x14;
        const F_TR0: usize = 0x18;
        const F_TR1: usize = 0x1C;
        const F_TR2: usize = 0x20;
        const F_IS: usize = 0x24;
        const F_S48: usize = 0x3C;
        const F_5C: usize = 0x5C;
        const F_M: usize = 0x84;
        const F_M90: usize = 0x90;
        const F_M94: usize = 0x94;
        const F_M98: usize = 0x98;
        const F_M9C: usize = 0x9C;
        const F_MA0: usize = 0xA0;
        const F_MA4: usize = 0xA4;
        const F_MA8: usize = 0xA8;
        const F_MB0: usize = 0xB0;
        const F_MB1: usize = 0xB1;
        const F_MB2: usize = 0xB2;
        const F_COOKIE: usize = 0xD0;
        // ---- callee ids (checker contract) ----
        const C_OK: u32 = 1;
        const C_CLR: u32 = 2;
        const C_NEW: u32 = 3;
        const C_NEW2: u32 = 43;
        const C_FIN: u32 = 4;
        const C_MK1: u32 = 5;
        const C_FMT: u32 = 6;
        const C_RUN: u32 = 7;
        const C_ALT: u32 = 8;
        const C_BYE: u32 = 9;
        const C_RST: u32 = 10;
        const C_F1A: u32 = 11;
        const C_F1B: u32 = 40;
        const C_F1C: u32 = 41;
        const C_F2A: u32 = 12;
        const C_F2B: u32 = 44;
        const C_F2C: u32 = 45;
        const C_MK18: u32 = 13;
        const C_MK20: u32 = 36;
        const C_ATT: u32 = 14;
        const C_F3A: u32 = 15;
        const C_F3B: u32 = 46;
        const C_F3C: u32 = 47;
        const C_USE: u32 = 16;
        const C_SEQ: u32 = 17;
        const C_CFG: u32 = 18;
        const C_EMIT: u32 = 19;
        const C_ONE: u32 = 20;
        const C_ZERO: u32 = 21;
        const C_SUB: u32 = 22;
        const C_MK14: u32 = 23;
        const C_PREP: u32 = 24;
        const C_SET: u32 = 25;
        const C_K3: u32 = 26;
        const C_K1: u32 = 27;
        const C_LIM: u32 = 28;
        const C_SEL: u32 = 29;
        const C_F4A: u32 = 30;
        const C_F4B: u32 = 37;
        const C_F4C: u32 = 38;
        const C_IDX: u32 = 31;
        const C_K7: u32 = 32;
        const C_MK198: u32 = 33;
        const C_CKY: u32 = 34;
        const C_VT: u32 = 35;
        const C_MID: u32 = 39;
        const VT_SLOT: u32 = 0xEC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 { unsafe { (a as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 { unsafe { f32::from_bits(rd32(a)) } }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) { unsafe { wr32(a, v.to_bits()) } }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gset(va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 { unsafe { f32::from_bits(g32(va)) } }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Exact `cvttss2si` (truncate; NaN, infinities and out-of-range
        /// magnitudes give 0x80000000, unlike Rust's saturating `as`).
        #[inline(always)]
        fn cvt(x: f32) -> u32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000
            } else {
                x as i32 as u32
            }
        }
        /// Manager object behind a possibly-null slot: 0xff tag means absent.
        #[inline(always)]
        unsafe fn resolve(tagged: u32) -> u32 {
            unsafe {
                let tag = rd8(tagged.wrapping_add(4)) as u32;
                if tag == 0xff {
                    return 0;
                }
                let row = rd8(tagged.wrapping_add(0x40)) as u32;
                let col = tag.wrapping_mul(g32(G_MUL));
                let tbl = g32(G_TBL);
                col.wrapping_add(rd32(
                    tbl.wrapping_add(row.wrapping_mul(0x6f40)).wrapping_add(0x6f14),
                ))
            }
        }
        let mut f = [0u8; 0xD8];
        let fbase = unsafe { f.as_mut_ptr() as u32 };
        let fp = |off: usize| fbase.wrapping_add(off as u32);
        let mut eax: u32 = 0;
        wr32(fp(F_COOKIE), g32(G_COOKIE));

        // Entry gates: any one skips the refresh and runs teardown.
        if g32(G_MODE) == 1 || g32(G_VERA) != g32(G_VERB) || g32(G_STATE) == 0x12 {
            if rd8(this.wrapping_add(T_F129)) != 0 {
                eax = lf_checker_rt::callee_thiscall!(C_BYE, u32, this);
                wr8(this.wrapping_add(T_F129), 0);
            }
            eax = lf_checker_rt::callee_thiscall!(C_RST, u32, this);
            let p8 = rd32(this.wrapping_add(T_PTR8));
            if p8 != 0 {
                let obj = rd32(this.wrapping_add(T_OBJ));
                let lo: u32 =
                    if rd8(obj.wrapping_add(O_B219)) != 0 { 0xc8 } else { 0x3e8 };
                eax = lf_checker_rt::callee_thiscall!(C_LIM, u32, p8, lo, 0xfa0, 0x3f000000);
            }
            let p8b = rd32(this.wrapping_add(T_PTR8));
            if p8b != 0 {
                let obj = rd32(this.wrapping_add(T_OBJ));
                let pushed = if obj != 0
                    && rd8(obj.wrapping_add(O_B26C)) & 4 != 0
                    && rd32(obj.wrapping_add(O_PB30)) != 0
                {
                    // The re-test below repeats the flag test above, so the
                    // null arm is dead; it is kept for fidelity.
                    let b = if rd8(obj.wrapping_add(O_B26C)) & 4 != 0 {
                        rd32(obj.wrapping_add(O_PB30))
                    } else {
                        0
                    };
                    rd32(b.wrapping_add(0xc30))
                } else {
                    0
                };
                eax = lf_checker_rt::callee_thiscall!(C_SEL, u32, p8b, pushed);
            }
            eax = rd32(this.wrapping_add(T_PTR8));
            if eax == 0 {
                lf_checker_rt::callee_cdecl!(C_CKY, u32,);
                return 0;
            }
            let obj = rd32(this.wrapping_add(T_OBJ));
            if rd8(obj.wrapping_add(O_B218)) == 0 && rd8(obj.wrapping_add(O_B219)) != 0 {
                wr8(eax.wrapping_add(0xaa), 1);
                lf_checker_rt::callee_cdecl!(C_CKY, u32,);
                return eax;
            }
            wr8(eax.wrapping_add(0xaa), 0);
            lf_checker_rt::callee_cdecl!(C_CKY, u32,);
            return eax;
        }
        // Main path.
        let obj0 = rd32(this.wrapping_add(T_OBJ));
        let timer = g32(G_TIMER);
        let ok: u8 =
            lf_checker_rt::callee_thiscall!(C_OK, u8, rd32(obj0.wrapping_add(O_W224)));
        wr32(fp(F_TD), timer);
        if ok == 0
            && rd8(this.wrapping_add(T_B174)) != ok
            && rd32(this.wrapping_add(T_T170)).wrapping_add(0x3e8) < timer
        {
            lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_M));
            wr32(fp(F_MA4), rd32(this.wrapping_add(T_PTR8)));
            wr32(fp(F_M90), obj0.wrapping_add(O_S780));
            let nid: u32 = lf_checker_rt::callee_thiscall!(C_NEW, u32, fp(F_M));
            let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, nid);
            let made: u8 = lf_checker_rt::callee_thiscall!(
                C_MK1, u8, this, lf_checker_rt::relocated(0xe8cc24), fp(F_M), nid, fin, 0u32);
            if made != 0 {
                wr32(fp(F_TR0), 0);
                wr32(fp(F_TR1), 0xffffffff);
                wr32(fp(F_TR2), 0x37);
                let h: u32 = lf_checker_rt::callee_cdecl!(C_FMT, u32, lf_checker_rt::relocated(0xe8cc34), 0u32);
                eax = lf_checker_rt::callee_cdecl!(
                    C_RUN, u32, h, 0u32, 0u32, 1u32, fp(F_M), fp(F_TR0),
                    rd32(this.wrapping_add(T_OBJ)), nid);
            } else {
                eax = lf_checker_rt::callee_cdecl!(C_ALT, u32, nid);
            }
            wr32(this.wrapping_add(T_T170), rd32(fp(F_TD)));
        }

        let obj1 = rd32(this.wrapping_add(T_OBJ));
        let ok2: u8 =
            lf_checker_rt::callee_thiscall!(C_OK, u8, rd32(obj1.wrapping_add(O_W224)));
        wr8(this.wrapping_add(T_B174), ok2);
        if rd8(this.wrapping_add(T_F129)) != 0 {
            eax = lf_checker_rt::callee_thiscall!(C_BYE, u32, this);
            wr8(this.wrapping_add(T_F129), 0);
        }
        eax = lf_checker_rt::callee_thiscall!(C_RST, u32, this);

        // Gate slot: indexed descriptor or skip to the sub-object check.
        let arr_base = lf_checker_rt::relocated(G_ARR);
        let elem = rd32(
            arr_base.wrapping_add(rd32(this.wrapping_add(T_IDX)).wrapping_mul(4)),
        );
        wr32(fp(F_TB), elem);
        if elem != 0 {
            let a: f32 = lf_checker_rt::callee_thiscall!(C_F1A, f32, this, 1u32);
            wrf(fp(F_TC), a);
            let b: f32 = lf_checker_rt::callee_thiscall!(C_F1B, f32, this, 0u32);
            wrf(fp(F_TA), b);
            // max, NaN-tolerant the way `comiss`+`ja` is (NaN takes b).
            let mut m = rdf(fp(F_TC));
            if !(m > rdf(fp(F_TA))) {
                m = rdf(fp(F_TA));
            }
            wrf(fp(F_TC), m);
            let c: f32 =
                lf_checker_rt::callee_thiscall!(C_F2A, f32, lf_checker_rt::relocated(0x1231478), m.to_bits());
            wrf(fp(F_TA), c);
            // min against the ceiling, NaN keeps the value (`jbe`).
            let mut d = rdf(fp(F_TA));
            if d > gf(F_88E8) {
                d = gf(F_88E8);
            }
            wrf(fp(F_TA), d);
            let e: f32 = lf_checker_rt::callee_thiscall!(
                C_F2B, f32, lf_checker_rt::relocated(0x12315ec), rdf(fp(F_TC)).to_bits());
            wrf(fp(F_TC), e);
            let scaled = mul(rdf(fp(F_TC)), gf(F_8BB0));
            wr32(fp(F_TC), cvt(scaled));
            if rdf(fp(F_TA)) > gf(F_8670) {
                let slot18 = this.wrapping_add(T_PH18);
                if rd32(this.wrapping_add(T_PH18)) == 0 {
                    lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_M));
                    wr32(fp(F_MA4), rd32(this.wrapping_add(T_PTR8)));
                    wr32(fp(F_M90), rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_S780));
                    let e12 = rd32(rd32(fp(F_TB)).wrapping_add(0x12));
                    lf_checker_rt::callee_thiscall!(
                        C_MK18, u32, this, e12, slot18, fp(F_M),
                        0xffffffffu32, 0u32, 0u32);
                    if rd32(slot18) != 0 {
                        wr32(fp(F_TR0), 0);
                        wr32(fp(F_TR1), 0xffffffff);
                        wr32(fp(F_TR2), 0x58);
                        let e12b = rd32(rd32(fp(F_TB)).wrapping_add(0x12));
                        let id: u32 = lf_checker_rt::callee_cdecl!(
                            C_RUN, u32, e12b, 0u32, 1u32, 1u32, fp(F_M),
                            fp(F_TR0), rd32(this.wrapping_add(T_OBJ)),
                            0xffffffffu32);
                        eax = id;
                        let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, id);
                        eax = fin;
                        let ph = rd32(this.wrapping_add(T_PH18));
                        wr32(ph.wrapping_add(0xa4), id);
                        wr32(ph.wrapping_add(0xa8), fin);
                        wr32(ph.wrapping_add(0xac), 0);
                        lf_checker_rt::callee_thiscall!(
                            C_ATT, u32, rd32(slot18), 0u32, 0u32, 0u32);
                    }
                }
                if rd32(slot18) != 0 {
                    let g: f32 = lf_checker_rt::callee_cdecl!(
                        C_F3A, f32, rdf(fp(F_TA)).to_bits());
                    wrf(fp(F_TB), g);
                    let ph = rd32(slot18);
                    lf_checker_rt::callee_thiscall!(
                        C_USE, u32, ph, rdf(fp(F_TB)).to_bits());
                    lf_checker_rt::callee_thiscall!(C_SEQ, u32, ph, rd32(fp(F_TC)));
                    lf_checker_rt::callee_thiscall!(C_CFG, u32, fp(F_M), 0x58u32);
                    let ph2 = rd32(slot18);
                    let x0 = rdf(fp(F_TB));
                    let ec = rd32(ph2.wrapping_add(0xa4));
                    let ed = rd32(fp(F_TC));
                    wr8(fp(F_MB1), rd8(fp(F_MB1)) | 2 | 8);
                    wrf(fp(F_M90), x0);
                    wr32(fp(F_MA0), ed);
                    eax = lf_checker_rt::callee_cdecl!(C_EMIT, u32, ec, fp(F_M));
                }
            } else {
                let ph = rd32(this.wrapping_add(T_PH18));
                if ph != 0 {
                    eax = lf_checker_rt::callee_cdecl!(
                        C_ONE, u32, rd32(ph.wrapping_add(0xa4)));
                    eax = lf_checker_rt::callee_thiscall!(
                        C_ZERO, u32, rd32(this.wrapping_add(T_PH18)), 0u32);
                }
            }
        }
        // Sub-object must be live and in state 5 for the +0x14 chain.
        let ob2 = rd32(this.wrapping_add(T_OBJ));
        let mut ph14_ok = false;
        if rd32(ob2.wrapping_add(O_W2C4)) != 0 {
            let s1: u32 = lf_checker_rt::callee_thiscall!(
                C_SUB, u32, rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_SUB));
            eax = s1;
            if s1 != 0 {
                let s2: u32 = lf_checker_rt::callee_thiscall!(
                    C_SUB, u32, rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_SUB));
                eax = s2;
                if rd32(s2.wrapping_add(0x18)) == 5 {
                    ph14_ok = true;
                }
            }
        }
        if ph14_ok {
            let slot14 = this.wrapping_add(T_PH14);
            // The s3 chain runs only right after creation, never on the
            // direct-run path (which jumps straight to the hook below).
            let was_null = rd32(this.wrapping_add(T_PH14)) == 0;
            if was_null {
                lf_checker_rt::callee_thiscall!(
                    C_MK14, u32, this, lf_checker_rt::relocated(0xe8cc44), slot14, 1u32, 0u32,
                    rd32(this.wrapping_add(T_PTR8)), 0xffffffffu32, 0u32, 0u32);
            }
            if was_null && rd32(slot14) != 0 {
                lf_checker_rt::callee_thiscall!(C_PREP, u32, this, 3u32, fp(F_IS));
                lf_checker_rt::callee_thiscall!(C_SET, u32, rd32(slot14), fp(F_IS));
                lf_checker_rt::callee_thiscall!(C_K3, u32, rd32(slot14), 3u32);
                lf_checker_rt::callee_thiscall!(C_K1, u32, rd32(slot14), 1u32);
                lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_S48));
                wr32(fp(F_5C), rd32(this.wrapping_add(T_PTR8)));
                wr32(fp(F_TR0), 0);
                wr32(fp(F_TR1), 0xffffffff);
                wr32(fp(F_TR2), 0x4f);
                lf_checker_rt::callee_thiscall!(C_CFG, u32, fp(F_M), 0x4fu32);
                wr8(fp(F_MB0), rd8(fp(F_MB0)) | 4);
                wr8(fp(F_MB1), rd8(fp(F_MB1)) | 4 | 0x20);
                wrf(fp(F_M94), rdf(fp(F_IS)));
                wrf(fp(F_M98), rdf(fp(F_IS).wrapping_add(4)));
                wrf(fp(F_M9C), rdf(fp(F_IS).wrapping_add(8)));
                wr32(fp(F_MA8), 3);
                wr8(fp(F_MB2), 1);
                let h: u32 = lf_checker_rt::callee_cdecl!(C_FMT, u32, lf_checker_rt::relocated(0xe8cc64), 0u32);
                let id0: u32 = lf_checker_rt::callee_cdecl!(
                    C_RUN, u32, h, 0u32, 1u32, 0u32, fp(F_S48), fp(F_TR0),
                    rd32(this.wrapping_add(T_OBJ)), 0xffffffffu32);
                eax = id0;
                let id: u32 = lf_checker_rt::callee_cdecl!(
                    C_MID, u32, id0, fp(F_M), rd32(this.wrapping_add(T_OBJ)));
                eax = id;
                let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, id);
                eax = fin;
                let ph = rd32(this.wrapping_add(T_PH14));
                wr32(ph.wrapping_add(0xa4), id);
                wr32(ph.wrapping_add(0xa8), fin);
                wr32(ph.wrapping_add(0xac), 0);
                lf_checker_rt::callee_thiscall!(
                    C_ATT, u32, rd32(slot14), 0u32, 0u32, 0u32);
            }
            if rd32(slot14) != 0 {
                let ob3 = rd32(this.wrapping_add(T_OBJ));
                let timer2 = g32(G_TIMER);
                let hook: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(
                        rd32(ob3.wrapping_add(O_VT)).wrapping_add(VT_SLOT),
                    ) as usize);
                let got = hook(ob3, fp(F_IS));
                let y = rdf(got.wrapping_add(4));
                let mut x = rdf(got);
                let mut z = rdf(got.wrapping_add(8));
                x = mul(x, x);
                let yy = mul(y, y);
                z = mul(z, z);
                x = add(x, yy);
                let cap = gf(F_88E8);
                x = add(x, z);
                let mut n = x.sqrt();
                n = mul(n, gf(G_TIMERF));
                n = mul(n, gf(F_8B38));
                wrf(fp(F_TB), n);
                if !(cap > n) {
                    wrf(fp(F_TB), cap);
                }
                let q: f32 = lf_checker_rt::callee_thiscall!(C_F1C, f32, this, 2u32);
                wrf(fp(F_TC), q);
                let bl = mul(rdf(fp(F_TC)), rdf(fp(F_TB)));
                let r: f32 = lf_checker_rt::callee_thiscall!(
                    C_F4A, f32, this.wrapping_add(0x90), bl.to_bits(), timer2);
                wrf(fp(F_TC), r);
                let s: f32 =
                    lf_checker_rt::callee_thiscall!(C_K7, f32, lf_checker_rt::relocated(0x1289230), 7u32);
                wrf(fp(F_TB), s);
                let mut x2 = rdf(fp(F_TB));
                let mut x1 = rdf(fp(F_TC));
                let mut x3 = x2;
                x3 = mul(x3, gf(F_8AB8));
                x2 = mul(x2, gf(F_8C10));
                let mut x0 = x1;
                x0 = mul(x0, gf(F_8B00));
                x1 = mul(x1, gf(F_E8D714));
                x3 = add(x3, x0);
                let ph = rd32(this.wrapping_add(T_PH14));
                x2 = add(x2, x1);
                wrf(fp(F_TB), x3);
                let cnt = cvt(x2);
                lf_checker_rt::callee_thiscall!(C_USE, u32, ph, x3.to_bits());
                lf_checker_rt::callee_thiscall!(
                    C_SEQ, u32, rd32(this.wrapping_add(T_PH14)), cnt);
                lf_checker_rt::callee_thiscall!(C_CFG, u32, fp(F_M), 0x4fu32);
                let ph2 = rd32(this.wrapping_add(T_PH14));
                let y0 = rdf(fp(F_TB));
                let ec = rd32(ph2.wrapping_add(0xa4));
                wr8(fp(F_MB1), rd8(fp(F_MB1)) | 2 | 8);
                wrf(fp(F_M90), y0);
                wr32(fp(F_MA0), cnt);
                eax = lf_checker_rt::callee_cdecl!(C_EMIT, u32, ec, fp(F_M));
            }
        } else {
            let ph = rd32(this.wrapping_add(T_PH14));
            if ph != 0 {
                eax = lf_checker_rt::callee_cdecl!(
                    C_ONE, u32, rd32(ph.wrapping_add(0xa4)));
                eax = lf_checker_rt::callee_thiscall!(
                    C_ZERO, u32, rd32(this.wrapping_add(T_PH14)), 0u32);
            }
        }

        // Optional slot at +0x130, live only before its deadline.
        let p130 = rd32(this.wrapping_add(T_P130));
        if p130 != 0 {
            let dl = rd32(this.wrapping_add(T_T138));
            if dl != 0 && dl < g32(G_TIMER) {
                eax = lf_checker_rt::callee_cdecl!(
                    C_ONE, u32, rd32(p130.wrapping_add(0xa4)));
                eax = lf_checker_rt::callee_thiscall!(
                    C_ZERO, u32, rd32(this.wrapping_add(T_P130)), 0u32);
            }
        }

        // Limit + selector on the +0x8 object.
        let p8 = rd32(this.wrapping_add(T_PTR8));
        if p8 != 0 {
            let obj = rd32(this.wrapping_add(T_OBJ));
            let lo: u32 = if rd8(obj.wrapping_add(O_B219)) != 0 { 0xc8 } else { 0x3e8 };
            eax = lf_checker_rt::callee_thiscall!(C_LIM, u32, p8, lo, 0xfa0, 0x3f000000);
        }
        let p8b = rd32(this.wrapping_add(T_PTR8));
        if p8b != 0 {
            let obj = rd32(this.wrapping_add(T_OBJ));
            let pushed = if obj != 0
                && rd8(obj.wrapping_add(O_B26C)) & 4 != 0
                && rd32(obj.wrapping_add(O_PB30)) != 0
            {
                let b = if rd8(obj.wrapping_add(O_B26C)) & 4 != 0 {
                    rd32(obj.wrapping_add(O_PB30))
                } else {
                    0
                };
                rd32(b.wrapping_add(0xc30))
            } else {
                0
            };
            eax = lf_checker_rt::callee_thiscall!(C_SEL, u32, p8b, pushed);
        }
        let p8c = rd32(this.wrapping_add(T_PTR8));
        if p8c != 0 {
            let base = rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_B218);
            let v: u8 =
                if rd8(base) == 0 && rd8(base.wrapping_add(1)) != 0 { 1 } else { 0 };
            wr8(p8c.wrapping_add(0xaa), v);
        }
        // Blend the +0x40 float, then the +0x20 phase.
        let td = rd32(fp(F_TD));
        let f40 = rdf(this.wrapping_add(T_F40));
        let h2: f32 = lf_checker_rt::callee_thiscall!(
            C_F4B, f32, this.wrapping_add(0x24), f40.to_bits(), td);
        wrf(fp(F_TA), h2);
        let p20 = rd32(this.wrapping_add(T_PH20));
        let f40b = rdf(this.wrapping_add(T_F40));
        wrf(this.wrapping_add(T_F40), mul(f40b, gf(F_88B0)));
        let h2b = rdf(fp(F_TA));
        let slot20 = this.wrapping_add(T_PH20);
        if p20 == 0 && h2b > gf(F_8670) {
            let mut fl = g32(G_FLAGS);
            if fl & 1 == 0 {
                fl |= 1;
                gset(G_FLAGS, fl);
                let d: u32 = lf_checker_rt::callee_cdecl!(C_FMT, u32, lf_checker_rt::relocated(0xe8cc94), 0u32);
                eax = d;
                gset(G_D20, d);
            }
        }
        if p20 == 0 && h2b > gf(F_8670) {
            let dcur = g32(G_D20);
            lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_M));
            wr32(fp(F_MA4), rd32(this.wrapping_add(T_PTR8)));
            wr32(fp(F_M90), rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_S780));
            lf_checker_rt::callee_thiscall!(
                C_MK20, u32, this, dcur, slot20, fp(F_M),
                0xffffffffu32, 0u32, 0u32);
            if rd32(this.wrapping_add(T_PH20)) != 0 {
                wr32(fp(F_TR0), 0);
                wr32(fp(F_TR1), 0xffffffff);
                wr32(fp(F_TR2), 0x5f);
                let id: u32 = lf_checker_rt::callee_cdecl!(
                    C_RUN, u32, g32(G_D20), 0u32, 1u32, 1u32, fp(F_M),
                    fp(F_TR0), rd32(this.wrapping_add(T_OBJ)), 0xffffffffu32);
                eax = id;
                let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, id);
                eax = fin;
                let ph = rd32(this.wrapping_add(T_PH20));
                wr32(ph.wrapping_add(0xa4), id);
                wr32(ph.wrapping_add(0xa8), fin);
                wr32(ph.wrapping_add(0xac), 0);
                lf_checker_rt::callee_thiscall!(
                    C_ATT, u32, rd32(slot20), 0u32, 0u32, 0u32);
            }
        }
        // Run the +0x20 slot, then the tail chain.
        let xh = h2b;
        eax = rd32(slot20);
        if eax != 0 {
            let g: f32 = lf_checker_rt::callee_cdecl!(C_F3B, f32, xh.to_bits());
            wrf(fp(F_TB), g);
            eax = rd32(slot20);
            let mgr = resolve(eax);
            eax = lf_checker_rt::callee_thiscall!(
                C_IDX, u32, mgr, rdf(fp(F_TB)).to_bits());
            if gf(F_8670) >= rdf(fp(F_TA)) {
                eax = rd32(slot20);
                eax = lf_checker_rt::callee_cdecl!(
                    C_ONE, u32, rd32(eax.wrapping_add(0xa4)));
                eax = lf_checker_rt::callee_thiscall!(
                    C_ZERO, u32, rd32(slot20), 0u32);
            }
        }
        let ob4 = rd32(this.wrapping_add(T_OBJ));
        if rd8(ob4.wrapping_add(O_B26C)) & 4 != 0 && rd32(ob4.wrapping_add(O_PB30)) != 0 {
            wr32(this.wrapping_add(T_W144), 0);
        }
        if rd8(ob4.wrapping_add(O_B218)) != 0 || rd8(ob4.wrapping_add(O_B219)) == 0 {
            lf_checker_rt::callee_cdecl!(C_CKY, u32,);
            return eax;
        }
        // Second hook site: through the sub-object when flagged, else
        // direct; a mismatched sub-state skips to the +0x198 single-shot.
        let mut skip_hook = false;
        let mut got2 = 0u32;
        let mut hconst = 0u32;
        if rd8(ob4.wrapping_add(O_B26C)) & 4 != 0 && rd32(ob4.wrapping_add(O_PB30)) != 0 {
            let b = if rd8(ob4.wrapping_add(O_B26C)) & 4 != 0 {
                rd32(ob4.wrapping_add(O_PB30))
            } else {
                0
            };
            if rd32(b.wrapping_add(0x1304)) != 1 {
                skip_hook = true;
            } else {
                let ob = if rd8(ob4.wrapping_add(O_B26C)) & 4 != 0 {
                    rd32(ob4.wrapping_add(O_PB30))
                } else {
                    0
                };
                let hook: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(
                        rd32(ob.wrapping_add(O_VT)).wrapping_add(VT_SLOT),
                    ) as usize);
                got2 = hook(ob, fp(F_IS));
                hconst = lf_checker_rt::relocated(0x12314cc);
            }
        } else {
            let hook: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(
                    rd32(ob4.wrapping_add(O_VT)).wrapping_add(VT_SLOT),
                ) as usize);
            got2 = hook(ob4, fp(F_IS));
            hconst = lf_checker_rt::relocated(0x1231548);
        }
        if !skip_hook {
            let y = rdf(got2.wrapping_add(4));
            let mut x = rdf(got2);
            let mut z = rdf(got2.wrapping_add(8));
            x = mul(x, x);
            let yy = mul(y, y);
            z = mul(z, z);
            x = add(x, yy);
            x = add(x, z);
            let n = x.sqrt();
            let r: f32 =
                lf_checker_rt::callee_thiscall!(C_F2C, f32, hconst, n.to_bits());
            let tdv = rd32(fp(F_TD));
            let r2: f32 = lf_checker_rt::callee_thiscall!(
                C_F4C, f32, this.wrapping_add(0x17c), r.to_bits(), tdv);
            wrf(fp(F_TD), r2);
            if rdf(fp(F_TD)) > gf(F_8670) {
                let slot198 = this.wrapping_add(T_PH198);
                if rd32(this.wrapping_add(T_PH198)) == 0 {
                    let mut fl = g32(G_FLAGS);
                    if fl & 2 == 0 {
                        fl |= 2;
                        gset(G_FLAGS, fl);
                        let d: u32 =
                            lf_checker_rt::callee_cdecl!(C_FMT, u32, lf_checker_rt::relocated(0xe8ccc0), 0u32);
                        eax = d;
                        gset(G_D198, d);
                    }
                    lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_M));
                    wr32(fp(F_M90), rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_S780));
                    wr32(fp(F_MA4), rd32(this.wrapping_add(T_PTR8)));
                    let nid: u32 = lf_checker_rt::callee_thiscall!(C_NEW2, u32, fp(F_M));
                    wr32(fp(F_TA), nid);
                    let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, nid);
                    let made: u8 = lf_checker_rt::callee_thiscall!(
                        C_MK198, u8, this, g32(G_D198), slot198, fp(F_M),
                        rd32(fp(F_TA)), fin, 0u32);
                    if made != 0 {
                        wr32(fp(F_TR0), 0);
                        wr32(fp(F_TR1), 0xffffffff);
                        wr32(fp(F_TR2), 0x38);
                        eax = lf_checker_rt::callee_cdecl!(
                            C_RUN, u32, g32(G_D198), 0u32, 1u32, 1u32, fp(F_M),
                            fp(F_TR0), rd32(this.wrapping_add(T_OBJ)),
                            rd32(fp(F_TA)));
                    } else {
                        eax = lf_checker_rt::callee_cdecl!(C_ALT, u32, rd32(fp(F_TA)));
                    }
                }
                let x0 = rdf(fp(F_TD));
                let edi2 = rd32(slot198);
                if edi2 == 0 {
                    lf_checker_rt::callee_cdecl!(C_CKY, u32,);
                    return eax;
                }
                let g: f32 = lf_checker_rt::callee_cdecl!(C_F3C, f32, x0.to_bits());
                wrf(fp(F_TB), g);
                let x0b = rdf(fp(F_TB));
                lf_checker_rt::callee_thiscall!(C_USE, u32, edi2, x0b.to_bits());
                lf_checker_rt::callee_thiscall!(C_CFG, u32, fp(F_M), 0x38u32);
                let ph = rd32(slot198);
                let y0 = rdf(fp(F_TB));
                let ec = rd32(ph.wrapping_add(0xa4));
                wr8(fp(F_MB1), rd8(fp(F_MB1)) | 2);
                wrf(fp(F_M90), y0);
                eax = lf_checker_rt::callee_cdecl!(C_EMIT, u32, ec, fp(F_M));
                lf_checker_rt::callee_cdecl!(C_CKY, u32,);
                return eax;
            }
        }
        // Single-shot fallback for the +0x198 slot.
        let eph = rd32(this.wrapping_add(T_PH198));
        if eph == 0 {
            lf_checker_rt::callee_cdecl!(C_CKY, u32,);
            return eph;
        }
        eax = lf_checker_rt::callee_cdecl!(C_ONE, u32, rd32(eph.wrapping_add(0xa4)));
        eax = lf_checker_rt::callee_thiscall!(
            C_ZERO, u32, rd32(this.wrapping_add(T_PH198)), 0u32);
        lf_checker_rt::callee_cdecl!(C_CKY, u32,);
        eax
    }
});
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00a8a420 {
// original: 0x00A8A420 pool_guarded_lookup (proposed)

/// Look up the entry for the argument unless its kind selects the null one.
///
/// The session helper first runs on a frame slot with the global scope;
/// the kind bits (`+0x28` of `*arg`, shifted down 6, low four) 2 or 3
/// select the null entry: tear down and return 0. Otherwise the fetch
/// helper runs on the sub-object at `this+8` with the argument slot; a
/// non-zero answer tears down and is returned. On zero the list rooted at
/// `this+0x18` (ending at `this+8`) is walked for the first object whose
/// `+0x24` word has none of bits `0xA00` set, which is poked through its
/// function table slot `+0x44`; the fetch helper runs once more and its
/// answer (after tear-down) is returned.
///
/// Original: thiscall, one stack word, returns u32 in EAX. Four callee
/// shapes: session (thiscall, one stack word, frame object), fetch
/// (thiscall, one stack word), tear-down (thiscall, no stack words,
/// frame object), poke (thiscall through the object, no stack words),
/// the last intercepted by a planted stub address.
lf_checker_rt::export!(thiscall, rw_00A8A420(this: u32, arg: u32) -> u32 {
    unsafe {
        const SCOPE_FILE_VA: u32 = 0x12fb1dc;
        const SUB_OFF: u32 = 8;
        const HEAD_OFF: u32 = 0x18;
        const KIND_OFF: u32 = 0x28;
        const FLAG_OFF: u32 = 0x24;
        const FLAG_MASK: u32 = 0xa00;
        const POKE_SLOT: u32 = 0x44;
        const SESSION: u32 = 1;
        const FETCH: u32 = 2;
        const TEARDOWN: u32 = 3;
        let scope = lf_checker_rt::relocated(SCOPE_FILE_VA);
        let mut session_slot: [u32; 2] = [0, 0];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SESSION,
            u32,
            &mut session_slot as *mut u32 as u32,
            scope
        );
        let kind = ((((arg + KIND_OFF) as *const u32).read_unaligned() >> 6) & 0xf) as u8;
        // The tear-down takes the session slot; its contents are the
        // session helper's and unobserved here.
        let teardown = |slot: &mut [u32; 2]| {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                TEARDOWN,
                u32,
                slot as *mut [u32; 2] as u32
            );
        };
        if kind == 2 || kind == 3 {
            teardown(&mut session_slot);
            return 0;
        }
        let sub = this.wrapping_add(SUB_OFF);
        // The fetch helper takes the address of our argument slot, like
        // the original takes its incoming stack slot; both are S+4.
        let mut arg_copy = arg;
        let r: u32 = lf_checker_rt::callee_thiscall!(
            FETCH,
            u32,
            sub,
            &mut arg_copy as *mut u32 as u32
        );
        if r != 0 {
            teardown(&mut session_slot);
            return r;
        }
        let end = sub;
        let mut link = ((this + HEAD_OFF) as *const u32).read_unaligned();
        if link != end {
            loop {
                let obj = (link as *const u32).read_unaligned();
                if ((obj + FLAG_OFF) as *const u32).read_unaligned() & FLAG_MASK == 0 {
                    let slot = ((((obj as *const u32).read_unaligned()) + POKE_SLOT)
                        as *const u32)
                        .read_unaligned();
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    let _ = f(obj);
                    break;
                }
                link = ((link + 4) as *const u32).read_unaligned();
                if link == end {
                    break;
                }
            }
        }
        let r2: u32 = lf_checker_rt::callee_thiscall!(
            FETCH,
            u32,
            sub,
            &mut arg_copy as *mut u32 as u32
        );
        teardown(&mut session_slot);
        r2
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00ace110 {
// original: 0x00ace110 audio_mix_slices
/// Mix pass: resolve the source offsets, run the per-slice loop that emits
/// one triple per slice and re-emits slices whose id scan hits, then publish
/// the scaled direction, the optional output vector and the transformed
/// tail triple through the voice's hooks.
///
/// `a` is the voice, `b` the source, `c` the slice array of `count` entries,
/// `f1`/`f2` mix scalars, `d` an opaque per-slice argument, `out` an optional
/// output vector. Returns nothing; effects are the outgoing calls and the
/// output-vector writes.
///
/// Build note: verified bit-exact in the optimizing release build. The
/// optimizer otherwise reorders some SSE operands, which changes NaN payload
/// propagation where two different-payload NaNs meet (random-bit harness
/// fills only); opaque `black_box` barriers on the cascade sub-terms and the
/// reciprocal uses below pin the original's operand order (value-preserving:
/// association unchanged, all finite results bit-identical).
export!(cdecl, rw_ace110(
    a: u32,
    b: u32,
    c: u32,
    count: i32,
    f1_bits: u32,
    d: u32,
    f2_bits: u32,
    out: u32,
) -> u32 {
    /// File VA of the id table consulted by the per-slice scan.
    const ID_TABLE: u32 = 0x103f348;
    /// File VA of the shared vector pushed to the first virtual hook.
    const HOOK_VEC: u32 = 0x1b4b320;
    /// Stride of the slice array walked by the outer loop and the inner scan.
    const SLICE_STRIDE: u32 = 0x170;
    /// File VA of the output adjustment vector.
    const OUT_VEC: u32 = 0x103f390;
    unsafe {
        let f2 = f32::from_bits(f2_bits);
        let af = a as *const f32;
        let bf = b as *const f32;
        let mut ob = [0.0f32; 16];
        callee_cdecl!(
            1,
            u32,
            ob.as_mut_ptr() as u32,
            a.wrapping_add(0xd0),
            b.wrapping_add(0x40),
            b.wrapping_add(0x50),
            f2_bits
        );
        let mut t3 = [
            *bf.add(0x30 / 4) - ob[12],
            *bf.add(0x34 / 4) - ob[13],
            *bf.add(0x38 / 4) - ob[14],
        ];
        let vt = *(a as *const u32);
        let slot70 = *((vt + 0x70) as *const u32);
        let hook3: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot70 as usize);
        hook3(a, t3.as_mut_ptr() as u32, relocated(HOOK_VEC), 0);
        let mut e8 = [0.0f32; 4];
        callee_thiscall!(
            6,
            u32,
            e8.as_mut_ptr() as u32,
            ob.as_mut_ptr() as u32,
            b
        );
        let mut id6buf = [0.0f32; 2];
        callee_thiscall!(
            7,
            u32,
            e8.as_mut_ptr() as u32,
            id6buf.as_mut_ptr() as u32
        );
        let mut w3 = [0.0f32; 3];
        let mut scale = 0.0f32;
        callee_thiscall!(
            8,
            u32,
            id6buf.as_mut_ptr() as u32,
            w3.as_mut_ptr() as u32,
            core::ptr::addr_of_mut!(scale) as u32
        );
        w3[0] = w3[0] * scale;
        w3[1] = w3[1] * scale;
        w3[2] = w3[2] * scale;
        let slot74 = *((vt + 0x74) as *const u32);
        let hook1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot74 as usize);
        hook1(a, w3.as_mut_ptr() as u32);
        let d0 = *bf.add(0x40 / 4) - *af.add(0x110 / 4);
        let d1 = *bf.add(0x44 / 4) - *af.add(0x114 / 4);
        let d2 = *bf.add(0x48 / 4) - *af.add(0x118 / 4);
        let mut e3 = [
            *bf.add(0x50 / 4) - *af.add(0x120 / 4),
            *bf.add(0x54 / 4) - *af.add(0x124 / 4),
            *bf.add(0x58 / 4) - *af.add(0x128 / 4),
        ];
        if count > 0 {
            let mut c_cur = c;
            let mut e_cur = b.wrapping_add(0x60);
            let mut x_cur = b.wrapping_add(0x80);
            for _ in 0..count {
                callee_thiscall!(
                    9,
                    u32,
                    c_cur,
                    a,
                    b,
                    b.wrapping_add(0x40),
                    b.wrapping_add(0x50),
                    *(e_cur as *const u32),
                    x_cur,
                    f1_bits,
                    d,
                    f2_bits,
                    w3.as_mut_ptr() as u32
                );
                // The original skips the scan only when all three emitted
                // words are exactly zero (each lahf test jumps on zero).
                if w3[0] != 0.0 || w3[1] != 0.0 || w3[2] != 0.0 {
                    let entry0 = *(c_cur as *const u32);
                    let table = relocated(ID_TABLE);
                    let want = *((table.wrapping_add(entry0.wrapping_mul(4)))
                        as *const u32);
                    let mut j: i32 = 0;
                    while j < count {
                        let at = c.wrapping_add((j as u32).wrapping_mul(SLICE_STRIDE));
                        if *(at as *const u32) == want {
                            break;
                        }
                        j += 1;
                    }
                    if j < count {
                        let lane =
                            c.wrapping_add((j as u32).wrapping_mul(SLICE_STRIDE));
                        if lane != 0 {
                            callee_thiscall!(
                                10, u32, lane, a, w3.as_mut_ptr() as u32
                            );
                        }
                    }
                }
                c_cur = c_cur.wrapping_add(SLICE_STRIDE);
                e_cur = e_cur.wrapping_add(4);
                x_cur = x_cur.wrapping_add(0x10);
            }
        }
        let c0 = *af.add(0xc0 / 4);
        let r = 1.0f32 / f2;
        t3[0] = (d0 * c0) * r;
        t3[1] = (d1 * c0) * r;
        t3[2] = (d2 * c0) * r;
        let slot84 = *((vt + 0x84) as *const u32);
        let hook84: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot84 as usize);
        hook84(a, t3.as_mut_ptr() as u32);
        if out != 0 {
            let g = global::<[f32; 3]>(OUT_VEC);
            // The original stores the intermediate products first and then
            // the adjusted values; only the final values are observable.
            *((out) as *mut f32) = ((d0 * c0) * r) - (c0 * (*g)[0]);
            *((out + 4) as *mut f32) = ((d1 * c0) * r) - (c0 * (*g)[1]);
            *((out + 8) as *mut f32) = ((d2 * c0) * r) - (c0 * (*g)[2]);
        }
        let e = (a.wrapping_add(0xd0)) as *const f32;
        let e0 = *e.add(0);
        let e1 = *e.add(1);
        let e2 = *e.add(2);
        let e4 = *e.add(4);
        let e5 = *e.add(5);
        let e6 = *e.add(6);
        let e8v = *e.add(8);
        let e9 = *e.add(9);
        let e10 = *e.add(10);
        let v0 = e3[0];
        let v1 = e3[1];
        let v2 = e3[2];
        // Opaque barriers pin the original's SSE operand order (see the
        // build note above); association is unchanged, so every finite and
        // single-NaN value is bit-identical with or without them.
        let r1 = core::hint::black_box(
            core::hint::black_box(v0 * e0) + core::hint::black_box(v1 * e1),
        ) + core::hint::black_box(v2 * e2);
        let r4 = core::hint::black_box(
            core::hint::black_box(e5 * v1) + core::hint::black_box(e4 * v0),
        ) + core::hint::black_box(e6 * v2);
        let r2 = core::hint::black_box(
            core::hint::black_box(e9 * v1) + core::hint::black_box(v0 * e8v),
        ) + core::hint::black_box(e10 * v2);
        let s5 = core::hint::black_box(r1) * *af.add(0xa0 / 4);
        let s2 = *af.add(0xa4 / 4) * core::hint::black_box(r4);
        let s3 = *af.add(0xa8 / 4) * core::hint::black_box(r2);
        let f78 = core::hint::black_box(
            core::hint::black_box(core::hint::black_box(s5) * e0)
                + core::hint::black_box(e4 * core::hint::black_box(s2)),
        ) + core::hint::black_box(core::hint::black_box(s3) * e8v);
        let f7c = core::hint::black_box(
            core::hint::black_box(e5 * core::hint::black_box(s2))
                + core::hint::black_box(core::hint::black_box(s5) * e1),
        ) + core::hint::black_box(e9 * core::hint::black_box(s3));
        let f80 = core::hint::black_box(
            core::hint::black_box(e6 * core::hint::black_box(s2))
                + core::hint::black_box(core::hint::black_box(s5) * e2),
        ) + core::hint::black_box(e10 * core::hint::black_box(s3));
        e3[0] = f78;
        e3[1] = f7c;
        e3[2] = f80;
        // The original also spills one word from below the resolve block
        // here; that slot is never written, so the harness defines it as
        // zero, and the stored word is never read again either.
        t3[0] = f78 * core::hint::black_box(r);
        t3[1] = f7c * core::hint::black_box(r);
        t3[2] = f80 * core::hint::black_box(r);
        let slot88 = *((vt + 0x88) as *const u32);
        let hook88: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot88 as usize);
        hook88(a, t3.as_mut_ptr() as u32);
        0
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_008e63f0 {
// original: 0x008e63f0 gated_dual_channel_update
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};
#[inline]
unsafe fn rd8(base: u32, off: u32) -> u8 {
    unsafe { ((base.wrapping_add(off)) as *const u8).read() }
}

#[inline]
unsafe fn rd32(base: u32, off: u32) -> u32 {
    unsafe { ((base.wrapping_add(off)) as *const u32).read() }
}

#[inline]
unsafe fn wr8(base: u32, off: u32, v: u8) {
    unsafe { ((base.wrapping_add(off)) as *mut u8).write(v) }
}

#[inline]
unsafe fn wr16(base: u32, off: u32, v: u16) {
    unsafe { ((base.wrapping_add(off)) as *mut u16).write(v) }
}

#[inline]
unsafe fn wr32(base: u32, off: u32, v: u32) {
    unsafe { ((base.wrapping_add(off)) as *mut u32).write(v) }
}

/// Pinned entry-EAX value, returned on paths where no call ran.
const ENTRY_EAX: u32 = 0x1234_5678;
/// Gated dual-channel level update.
///
/// Returns the caller's entry EAX unchanged when the enable byte at +0x43E
/// is clear. Otherwise it asks the engine audio object for two input levels
/// through out-params, transforms each through the channel object at +0x54
/// (float in on the stack, float out on the x87 stack), and compares both
/// against the -100.0 silence limit: an above-limit channel with no handle
/// yet gets one created through the 17-argument creator and then has its
/// level set, while an at-or-below-limit channel with a live handle is shut
/// down through the stop helper. The returned EAX is the last callee answer
/// on the path taken (a float answer leaves its bits in EAX), or the entry
/// EAX when no call ran.

/// Pinned entry-EAX value, returned on the early path. Must equal regs[0].
/* k-skip: duplicate const ENTRY_EAX dropped */
/// Engine audio object the level query runs against (constant address).
const ENGINE_OBJ: u32 = 0x0116_5880;
/// Silence-limit float (-100.0), read from the image like the original.
const LIMIT_F: u32 = 0x00FE_8DF8;
/// Per-channel creator parameter globals.
const CH0_PARAM: u32 = 0x0117_6880;
const CH1_PARAM: u32 = 0x0117_6884;

export!(thiscall, rw_008E63F0(this: u32) -> u32 {
    let mut eax = ENTRY_EAX;
    if unsafe { rd8(this, 0x43E) } == 0 {
        return eax;
    }
    let mut level_b: u32 = 0;
    let mut level_a: u32 = 0;
    // (A's EAX answer is overwritten by B2 before anything reads it.)
    let _a = callee_thiscall!(
        1, u32, relocated(ENGINE_OBJ),
        core::ptr::addr_of_mut!(level_b) as u32,
        core::ptr::addr_of_mut!(level_a) as u32
    );
    let f1 = callee_thiscall!(2, f32, this.wrapping_add(0x54), level_b);
    // (B1's EAX residue is overwritten by B2 before anything reads it.)
    let f2 = callee_thiscall!(3, f32, this.wrapping_add(0x54), level_a);
    eax = f2.to_bits();
    let limit = f32::from_bits(unsafe { global::<u32>(LIMIT_F).read() });
    // Channel 0, handle slot at +0x48.
    if f1 > limit {
        if unsafe { rd32(this, 0x48) } == 0 {
            let p = unsafe { global::<u32>(CH0_PARAM).read() };
            eax = callee_thiscall!(
                4, u32, this, p, this.wrapping_add(0x48),
                0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0xFFFF_FFFF, 0, 0
            );
        }
        let h = unsafe { rd32(this, 0x48) };
        if h != 0 {
            eax = callee_thiscall!(5, u32, h, f1.to_bits());
        }
    } else {
        let h = unsafe { rd32(this, 0x48) };
        if h != 0 {
            eax = callee_thiscall!(6, u32, h, 0);
        }
    }
    // Channel 1, handle slot at +0x4C.
    if f2 > limit {
        if unsafe { rd32(this, 0x4C) } == 0 {
            let p = unsafe { global::<u32>(CH1_PARAM).read() };
            eax = callee_thiscall!(
                7, u32, this, p, this.wrapping_add(0x4C),
                0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0xFFFF_FFFF, 0, 0
            );
        }
        let h = unsafe { rd32(this, 0x4C) };
        if h != 0 {
            eax = callee_thiscall!(5, u32, h, f2.to_bits());
        }
    } else {
        let h = unsafe { rd32(this, 0x4C) };
        if h != 0 {
            eax = callee_thiscall!(6, u32, h, 0);
        }
    }
    eax
});

use lf_checker_rt::{callee_stdcall, callee_fastcall, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00bf9500 {
// original: 0x00bf9500 task_handle_cascade (proposed)

/// Ensure the task handle, run one of two setup cascades, then dispatch on
/// two flag bits and stamp the handle with the current tick.
///
/// `this` carries a dword at `+0x08`, a dword at `+0x20`, a dword at `+0x24`
/// and a flag byte at `+0x28`; `a` selects the long cascade (with the flag)
/// and is passed on to two of its calls.
///
/// Behaviour: the manager object is asked for the handle for (`this.+0x08`,
/// 0, 0) while `this` is saved in a frame slot; a null handle returns 0. When
/// bit 1 of the flag byte is set and `a` is non-null, the long cascade runs:
/// a fetch call, a query call whose answer becomes a scratch value, a vector
/// call and a register call on the handle, a copy call, and two transform
/// calls sharing one scratch object; then the saved `this` is restored. The
/// short cascade instead runs two gather calls, a matrix call filling a
/// fifteen-word block, and an apply call on that block. The tail copies
/// sixteen bytes from one frame slot to another (matrix words on the short
/// path, never-stored zeros on the long path), selects one of two global
/// floats by bit 0 of the flag byte, and dispatches on bits 2-3: values 1
/// and 2 each run a clear call plus a five-argument dispatch (handle, copied
/// block pointer, selected float, 3.0, -1.0) to different callees, value 3
/// runs a sixteen-byte fill whose first eight bytes go to handle `+0x00`
/// while the next four go to handle `+0x08` with a trailing constant word,
/// and value 0 runs nothing. Then `this.+0x24` is stored at handle `+0x1ec`,
/// a post call runs, and the stamp slot (`+0x1d4`) gets the global tick,
/// bumped by one when the tick-check call's answer equals the global compare
/// word. The stamp is also the return value.
///
/// Original: 0x00bf9500 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bf9500(this: u32, a: u32) -> u32 {
    unsafe {
        const TASK_MGR: u32 = 0x01394d60;
        const G_SEL0: u32 = 0x00fe8a94;
        const G_SEL1: u32 = 0x00fe8b20;
        const TICK_CMP: u32 = 0x011f702c;
        const TICK_SRC: u32 = 0x011f70c4;
        const TYPE_ID: u32 = 0x08;
        const THIS_W20: u32 = 0x20;
        const THIS_W24: u32 = 0x24;
        const THIS_FLAG: u32 = 0x28;
        const HANDLE_V3HI: u32 = 0x1ec;
        const HANDLE_STAMP: u32 = 0x1d4;
        const THREE_BITS: u32 = 0x40400000;
        const NEGONE_BITS: u32 = 0xbf800000;
        const MAKE_CONST: u32 = 0x00c19080;
        const TRAIL_CONST: u32 = 0x0062e7a0;
        const C_ACQUIRE: u32 = 1;
        const C_FETCH: u32 = 2;
        const C_QUERY: u32 = 3;
        const C_VEC: u32 = 4;
        const C_REG: u32 = 5;
        const C_COPY: u32 = 6;
        const C_X1: u32 = 7;
        const C_X2: u32 = 8;
        const C_G1: u32 = 9;
        const C_G2: u32 = 10;
        const C_MAT: u32 = 11;
        const C_APPLY: u32 = 12;
        const C_CLEAR: u32 = 13;
        const C_D1: u32 = 14;
        const C_D2: u32 = 15;
        const C_MAKE: u32 = 16;
        const C_POST: u32 = 17;
        const C_TICK2: u32 = 18;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let type_id = rd32(this.wrapping_add(TYPE_ID));
        let handle = lf_checker_rt::callee_thiscall!(
            C_ACQUIRE,
            u32,
            lf_checker_rt::relocated(TASK_MGR),
            type_id,
            0,
            0
        );
        if handle == 0 {
            return 0;
        }
        let flag = (rd32(this.wrapping_add(THIS_FLAG)) & 0xff) as u8;
        // Sixteen-byte slot shared by the tail copy: matrix words on the
        // short path, never-stored zeros on the long path.
        let mut slot60 = [0u32; 4];
        if flag & 2 != 0 && a != 0 {
            let mut cell80 = [0u32; 1];
            lf_checker_rt::callee_thiscall!(C_FETCH, u32, this, cell80.as_mut_ptr() as u32);
            let mut cell_b: u32 = 1;
            let q = lf_checker_rt::callee_cdecl!(
                C_QUERY,
                u32,
                a,
                rd32(this.wrapping_add(THIS_W20)),
                1,
                (&mut cell_b as *mut u32) as u32
            );
            lf_checker_rt::callee_thiscall!(C_VEC, u32, handle, q);
            lf_checker_rt::callee_thiscall!(
                C_REG,
                u32,
                lf_checker_rt::relocated(TASK_MGR),
                handle,
                a,
                0
            );
            lf_checker_rt::callee_thiscall!(C_COPY, u32, handle, cell80.as_mut_ptr() as u32);
            let mut obj30 = [0u32; 1];
            lf_checker_rt::callee_thiscall!(
                C_X1,
                u32,
                obj30.as_mut_ptr() as u32,
                cell80.as_mut_ptr() as u32
            );
            lf_checker_rt::callee_thiscall!(C_X2, u32, obj30.as_mut_ptr() as u32, q);
        } else {
            let mut cell20 = [0u32; 3];
            lf_checker_rt::callee_thiscall!(C_G1, u32, this, cell20.as_mut_ptr() as u32);
            let mut cell70 = [0u32; 3];
            lf_checker_rt::callee_thiscall!(C_G2, u32, this, cell70.as_mut_ptr() as u32);
            let mut cell30 = [0u32; 15];
            lf_checker_rt::callee_cdecl!(
                C_MAT,
                u32,
                cell30.as_mut_ptr() as u32,
                cell70.as_mut_ptr() as u32,
                cell20.as_mut_ptr() as u32,
                0
            );
            lf_checker_rt::callee_thiscall!(C_APPLY, u32, handle, cell30.as_mut_ptr() as u32);
            slot60[0] = cell30[12];
            slot60[1] = cell30[13];
            slot60[2] = cell30[14];
            // slot60[3] is never stored (contract stack_fill 0).
        }
        let mut cell10 = slot60;
        let sel = if flag & 1 != 0 {
            rd32(lf_checker_rt::relocated(G_SEL1))
        } else {
            rd32(lf_checker_rt::relocated(G_SEL0))
        };
        match (flag.wrapping_shr(2)) & 3 {
            1 => {
                lf_checker_rt::callee_thiscall!(C_CLEAR, u32, handle);
                lf_checker_rt::callee_cdecl!(
                    C_D1,
                    u32,
                    handle,
                    cell10.as_mut_ptr() as u32,
                    sel,
                    THREE_BITS,
                    NEGONE_BITS
                );
            }
            2 => {
                lf_checker_rt::callee_thiscall!(C_CLEAR, u32, handle);
                lf_checker_rt::callee_cdecl!(
                    C_D2,
                    u32,
                    handle,
                    cell10.as_mut_ptr() as u32,
                    sel,
                    THREE_BITS,
                    NEGONE_BITS
                );
            }
            3 => {
                let mut cell20c = [0u32; 4];
                lf_checker_rt::callee_thiscall!(
                    C_MAKE,
                    u32,
                    cell20c.as_mut_ptr() as u32,
                    0,
                    lf_checker_rt::relocated(MAKE_CONST),
                    0,
                    0
                );
                wr32(handle, cell20c[0]);
                wr32(handle.wrapping_add(4), cell20c[1]);
                wr32(handle.wrapping_add(8), cell20c[2]);
                wr32(handle.wrapping_add(12), lf_checker_rt::relocated(TRAIL_CONST));
            }
            _ => {}
        }
        wr32(
            handle.wrapping_add(HANDLE_V3HI),
            rd32(this.wrapping_add(THIS_W24)),
        );
        lf_checker_rt::callee_thiscall!(C_POST, u32, handle);
        let tick_answer = lf_checker_rt::callee_cdecl!(C_TICK2, u32,);
        let cmp = rd32(lf_checker_rt::relocated(TICK_CMP));
        let tick = rd32(lf_checker_rt::relocated(TICK_SRC));
        let stamped = if cmp != tick_answer {
            tick
        } else {
            tick.wrapping_add(1)
        };
        wr32(handle.wrapping_add(HANDLE_STAMP), stamped);
        stamped
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_009a8810 {
// original: 0x009a8810 script_event_forward
/// Forward the script event for slot `idx` to its owner's handler.
///
/// A negative index returns at once. Otherwise the resolver (stubbed,
/// cdecl/0) supplies a table whose word at `+4` selects the slot's
/// owner, and the opener (stubbed, thiscall/1) is tried: a
/// non-negative answer forwards the event (below) and returns. A
/// negative answer runs the resetter (stubbed, thiscall/1 with 0)
/// and retries the opener; a non-negative retry forwards the same
/// way. When both attempts fail, the event source (stubbed, cdecl/0)
/// is polled: a null event returns, otherwise the event is formatted
/// into a stack buffer (stubbed, cdecl/3) and logged (stubbed,
/// cdecl/5). Forwarding reads the slot pair at `this + idx*16 +
/// 0x33d4` (object, generation): a null object or a generation
/// mismatching the opener's answer returns, otherwise the object's
/// handler at vtable slot 3 is invoked with the object in ECX and
/// (`arg1`, `arg2`) on the stack (the vtable is fabricated heap
/// holding the stub address). The CRT security cookie is checked on
/// exit (stubbed, preserving all registers). Thiscall, three stack
/// words, no compared result (EAX is cookie-derived garbage).
export!(thiscall, rw_009A8810(this: u32, idx: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x33d4;
        const STRIDE: u32 = 16;
        const LOG_TAG: u32 = 0xe91918;
        if (idx as i32) < 0 {
            let _: u32 = callee_cdecl!(9, u32,);
            return 0;
        }
        let p: u32 = callee_cdecl!(1, u32,);
        let owner = ((p + 4) as *const u32).read_unaligned();
        let r1: u32 = callee_thiscall!(2, u32, this, owner);
        if (r1 as i32) >= 0 {
            event_forward(this, idx, arg1, arg2, r1);
            let _: u32 = callee_cdecl!(9, u32,);
            return 0;
        }
        let _: u32 = callee_thiscall!(3, u32, this, 0);
        let r3: u32 = callee_thiscall!(4, u32, this, owner);
        if (r3 as i32) >= 0 {
            event_forward(this, idx, arg1, arg2, r3);
            let _: u32 = callee_cdecl!(9, u32,);
            return 0;
        }
        let ev: u32 = callee_cdecl!(5, u32,);
        let mut buf = [0u32; 32];
        let bufp = (&mut buf as *mut u32) as u32;
        let _: u32 = callee_cdecl!(6, u32, bufp, 0, 0x78);
        if ev != 0 {
            let _: u32 = callee_cdecl!(7, u32, bufp, 0x80, relocated(LOG_TAG), ev, r3);
        }
        let _: u32 = callee_cdecl!(9, u32,);
        0
    }
});

/// Forward one event through the slot object's vtable handler.
unsafe fn event_forward(this: u32, idx: u32, arg1: u32, arg2: u32, gen: u32) {
    unsafe {
        let e = this + idx * 16 + 0x33d4;
        let obj = (e as *const u32).read_unaligned();
        if obj == 0 {
            return;
        }
        if ((e.wrapping_add(4)) as *const u32).read_unaligned() != gen {
            return;
        }
        let vt = (obj as *const u32).read_unaligned();
        let _tgt = ((vt + 0x0c) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(8, u32, obj, arg1, arg2);
    }
}

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00888d60 {
// original: 0x00888D60 stream_bank_attach (proposed)

/// Attach a stream bank the first time it is seen, under a guard.
///
/// Opens the guard (callee 1) on a stack scratch word. When the attached
/// flag global is already set, or the bank index `arg1` is 8 or more,
/// closes the guard (callee 3) and returns the guard answer with its low
/// byte cleared. Otherwise records the index, copies `arg1 * 8` bytes from
/// `arg0` to the bank name slot (callee 2), stamps the attach time
/// (callee 4), sets the flag and returns the guard answer with its low
/// byte set to 1.
///
/// The two scratch words are only ever passed to the guard entries and
/// never read back, so the contract does not compare them.
///
/// Original: 0x00888D60 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00888D60(arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const GUARD_ARG_FILE_VA: u32 = 0x0115_a4f4;
        const NAME_SLOT_FILE_VA: u32 = 0x0115_a488;
        const INDEX_GLOBAL: u32 = 0x0115_a478;
        const STAMP_GLOBAL: u32 = 0x0115_a474;
        const AUX_GLOBAL: u32 = 0x0115_a47c;
        const FLAG_GLOBAL: u32 = 0x0115_a480;
        const FLAG2_GLOBAL: u32 = 0x0115_a481;
        const MAX_BANK: u32 = 8;
        const OPEN: u32 = 1;
        const COPY: u32 = 2;
        const CLOSE: u32 = 3;
        const STAMP: u32 = 4;
        let mut scratch = [0u32; 2];
        let sp = &mut scratch as *mut u32 as u32;
        let ga = lf_checker_rt::relocated(GUARD_ARG_FILE_VA);
        lf_checker_rt::callee_thiscall!(OPEN, u32, sp, ga);
        let flag = (lf_checker_rt::relocated(FLAG_GLOBAL) as *const u8).read();
        let done = if flag != 0 || arg1 >= MAX_BANK {
            0u32
        } else {
            (lf_checker_rt::relocated(INDEX_GLOBAL) as *mut u32)
                .write_unaligned(arg1);
            let dst = lf_checker_rt::relocated(NAME_SLOT_FILE_VA);
            lf_checker_rt::callee_cdecl!(COPY, u32, dst, arg0, arg1.wrapping_mul(8));
            let t = lf_checker_rt::callee_cdecl!(STAMP, u32,);
            (lf_checker_rt::relocated(STAMP_GLOBAL) as *mut u32).write_unaligned(t);
            (lf_checker_rt::relocated(AUX_GLOBAL) as *mut u32).write_unaligned(0);
            (lf_checker_rt::relocated(FLAG_GLOBAL) as *mut u8).write(1);
            (lf_checker_rt::relocated(FLAG2_GLOBAL) as *mut u8).write(0);
            1u32
        };
        let g = lf_checker_rt::callee_thiscall!(CLOSE, u32, sp.wrapping_add(4));
        (g & 0xffff_ff00) | done
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00d447f0 {
// original: 0x00d447f0 position_query_b (proposed)

/// Query three float slots from the position helper; return the second.
///
/// Reserves three scratch slots and calls the helper with the incoming `arg`
/// plus one pointer per slot (pushed in an order that makes the helper's
/// parameter order (arg, third, second, first), where "first" is the slot
/// pushed first). Returns the second-pushed slot's float, bit for bit,
/// through the floating-point result channel. The helper is cdecl/4 and is
/// intercepted; its out-words are scripted by the checker. Differs from its
/// twin only in which slot is returned.
///
/// Original: cdecl, one stack word, caller cleans up.
lf_checker_rt::export!(cdecl, rw_00d447f0(arg: u32) -> f32 {
    unsafe {
        const QUERY: u32 = 1;
        let mut first = 0u32;
        let mut second = 0u32;
        let mut third = 0u32;
        let p_first = core::ptr::addr_of_mut!(first) as u32;
        let p_second = core::ptr::addr_of_mut!(second) as u32;
        let p_third = core::ptr::addr_of_mut!(third) as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(QUERY, u32, arg, p_third, p_second, p_first);
        f32::from_bits(second)
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00a934d0 {
// original: 0x00a934d0 stream_scan_rows_dispatch (proposed)

/// Scan the streaming row set, dispatching each live row by a per-row flag.
///
/// Callee 1 fills a 4-byte scratch buffer first (its contents are never
/// read). Then rows 1 through 0x1f3 are visited: callee 2 writes one flag
/// byte; rows are skipped while the global kill flag at `FLAG` is set, while
/// the row's select byte (`selset[i]`, select base at `set+0x04`) has bit 7
/// set, or while the row address (`set+0x00 + stride*i`, stride at
/// `set+0x0c`) is null. A nonzero flag byte dispatches the index to callee 3;
/// a zero flag byte dispatches to callee 4 only when the row's head word is
/// nonzero and its state byte at `+0x54` is nonzero. The flag is a full word:
/// the checker's scripted callee write is word-sized.
///
/// Returns 1 in al (upper bytes are call leftovers). Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_00a934d0() -> u32 {
    unsafe {
        const SET_GLOBAL: u32 = 0x012fb258;
        const FLAG_WORD: u32 = 0x0116d27c;
        const FIRST: u32 = 1;
        const LIMIT: u32 = 0x1f4;
        const BASE_OFF: u32 = 0x00;
        const SELECT_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const STATE_OFF: u32 = 0x54;
        const SKIP_BIT: u8 = 0x80;
        let mut scratch = [0u8; 4];
        lf_checker_rt::callee_cdecl!(1, u32, scratch.as_mut_ptr() as u32, 4);
        let mut i = FIRST;
        while i < LIMIT {
            let mut flag = 0u32;
            lf_checker_rt::callee_cdecl!(2, u32, &mut flag as *mut u32 as u32, 1);
            let killed =
                (lf_checker_rt::global::<u32>(FLAG_WORD).read_unaligned() >> 8) & 0xff;
            if killed == 0 {
                let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
                let selbase = ((set + SELECT_OFF) as *const u32).read_unaligned();
                if (((selbase + i) as *const u8).read() & SKIP_BIT) == 0 {
                    let stride = ((set + STRIDE_OFF) as *const u32).read_unaligned();
                    let base = ((set + BASE_OFF) as *const u32).read_unaligned();
                    let row = base.wrapping_add(stride.wrapping_mul(i));
                    if row != 0 {
                        if flag != 0 {
                            lf_checker_rt::callee_cdecl!(3, u32, i);
                        } else {
                            let head = (row as *const u32).read_unaligned();
                            if head != 0
                                && ((row + STATE_OFF) as *const u8).read() != 0
                            {
                                lf_checker_rt::callee_cdecl!(4, u32, i);
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        1
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00526b90 {
// original: 0x00526b90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race59NoHolds, player_schema::LeaderboardInfo, 10>::vf6

/// Finds a column value in this leaderboard's column list.
///
/// Asks the leaderboard registry (callee) for id LEADERBOARD_ID's column
/// list into a frame buffer (count at +12, array pointer at +16), then
/// returns the index of `value` in it, or -1 when the lookup fails,
/// the list is empty, or the value is absent. stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_00526b90(value: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x4a;
        const LOOKUP: u32 = 1;
        const COUNT: usize = 3;
        const ARRAY: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let count = buf[COUNT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let arr = buf[ARRAY] as u32;
        let mut i = 0i32;
        while i < count {
            let v = (arr.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read_unaligned();
            if v == value {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00a88ee0 {
// original: 0x00a88ee0 range_gate_update
/// Gate a range state on the length of a callee-supplied vector.
///
/// Calls vtable slot 0x3B of the argument object (intercepted,
/// thiscall/1) to obtain a 3-vector, measures its length in
/// single-precision SSE, and compares it against the fixed threshold.
/// Above the threshold it sets flag bits 0-1 and stores 1.0; at or below
/// (NaN included) it sets bits 0, 1 and 3 and stores the small constant.
/// Returns the callee answer with its low byte replaced on the long path,
/// exactly as the original leaves EAX.
export!(thiscall, rw_00a88ee0(this_obj: u32, arg0: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_add_ss, _mm_cvtss_f32, _mm_mul_ss, _mm_set_ss,
                              _mm_sqrt_ss};
        *((this_obj.wrapping_add(0x1480)) as *mut u8) &= !2;
        let vtable = *(arg0 as *const u32);
        let target = *((vtable.wrapping_add(0xec)) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let mut buf = [0u32; 3];
        let ans = f(arg0, buf.as_mut_ptr() as u32);
        let x = *((ans) as *const f32);
        let y = *((ans.wrapping_add(4)) as *const f32);
        let z = *((ans.wrapping_add(8)) as *const f32);
        let x2 = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(x), _mm_set_ss(x)));
        let y2 = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(y), _mm_set_ss(y)));
        let z2 = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(z), _mm_set_ss(z)));
        let s01 = _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(x2), _mm_set_ss(y2)));
        let s = _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(s01), _mm_set_ss(z2)));
        let d = _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(s)));
        let t = f32::from_bits(*global::<u32>(0xfe87d0));
        // jbe: at-or-below or NaN takes the short path.
        if d > t {
            let b = *((this_obj.wrapping_add(0x1480)) as *const u8);
            let nb = (b & !8) | 3;
            *((this_obj.wrapping_add(0x145c)) as *mut u32) = 0x3f800000;
            *((this_obj.wrapping_add(0x1480)) as *mut u8) = nb;
            (ans & 0xffffff00) | (nb as u32)
        } else {
            *((this_obj.wrapping_add(0x1480)) as *mut u8) |= 0xb;
            *((this_obj.wrapping_add(0x145c)) as *mut u32) = 0x3c23d70b;
            ans
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0051a790 {
// original: 0x0051A790 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race14NoHolds, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one entry of this leaderboard's value table into a width.
///
/// `index` selects the entry. A scratch row is handed to the schema
/// callee (id `0x6b`) which fills in the value-table pointer (row word
/// at `+0x14`); a zero answer means no table and yields 0. The entry
/// is classified by the class callee; kind `NOT_FOUND`, or a kind
/// whose predecessor falls outside the five-row table, yields 0.
/// Otherwise the row gives the width: row k holds `4, 8, 8, 0, 4`.
///
/// Original: 0x0051A790 (stdcall, one stack word; incoming registers ignored).
lf_checker_rt::export!(stdcall, rw_0051a790(index: u32) -> u32 {
    unsafe {        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        const SCHEMA_ID: u32 = 0x6b;
        const SCHEMA_CALLEE: u32 = 1;
        const CLASS_CALLEE: u32 = 2;
        const NOT_FOUND: u32 = 0xffff_ffff;
        const WIDTHS: [u32; 5] = [4, 8, 8, 0, 4];

        // Scratch row the schema callee fills: value table at +0x14.
        let mut row = [0u32; 8];
        row[5] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(SCHEMA_CALLEE, u32, SCHEMA_ID, row.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0;
        }
        let table = row[5];
        let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(CLASS_CALLEE, u32, entry);
        if kind == NOT_FOUND {
            return 0;
        }
        let arm = kind.wrapping_sub(1);
        if arm > 4 {
            return 0;
        }
        WIDTHS[arm as usize]
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00533480 {
// original: 0x00533480 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race45Standard, player_schema::LeaderboardInfo, 10>::vf6

/// Index of an entry id in this board's id list, or -1.
///
/// `want` is the entry id to find. The list comes from the leaderboard
/// lookup callee (fastcall: board id in ECX, out-struct in EDX, boolean in
/// AL), which writes the entry count at struct offset `+0x0c` and a pointer
/// to the id array at `+0x10`. This instantiation asks for board
/// `LEADERBOARD_ID`.
///
/// When the lookup fails, when the count is not positive (compared signed),
/// or when no element equals `want`, the result is `NOT_FOUND` (-1).
/// Otherwise it is the index of the first equal element, scanning from 0.
///
/// The object pointer (`this`) is unused: the board is selected by the id
/// constant alone. Original: thiscall with one stack word, callee pops 4.

lf_checker_rt::export!(thiscall, rw_00533480(_this: u32, want: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x87;
        const NOT_FOUND: u32 = 0xffff_ffff;

        /// Out-struct filled by the lookup callee: count at `+0x0c`, array at `+0x10`.
        #[repr(C)]
        struct EntryList {
            _reserved: [u32; 3],
            count: u32,
            items: u32,
        }

        #[inline(always)]
        unsafe fn read_word(base: u32, index: u32) -> u32 {
            unsafe { (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() }
        }

        let mut list = EntryList { _reserved: [0; 3], count: 0, items: 0 };
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, &mut list as *mut EntryList as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        if (list.count as i32) <= 0 {
            return NOT_FOUND;
        }
        let mut index = 0u32;
        while index < list.count {
            if read_word(list.items, index) == want {
                return index;
            }
            index += 1;
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0089bda0 {
// original: 0x0089bda0 audio_dual_slot_max (proposed)

/// Runs the slot operation on two slots and returns the SIGNED max of the answers.
///
/// `this` carries a bank byte at `+0x40` and two slot bytes at `+0x48`/`+0x49`.
/// Each slot selects a target (`TABLE[bank*0x6f40+0x6f10]+STRIDE*slot`, or
/// null for a 0xFF slot) on which the operation (callees 1 and 2, thiscall/1)
/// runs with a scratch out-flag byte; the flags start zeroed. When `out` is
/// non-null and either flag came back nonzero, `1` is stored to `out`. The
/// scratch bytes live in a padded C-layout struct so the word-granular
/// scripted writes land where the original's frame puts them. Returns the
/// greater of the two answers as SIGNED integers (the original uses `cmovg`).
///
/// Original: 0x0089bda0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0089bda0(this: u32, out: u32) -> u32 {
    unsafe {
        const BANK_OFF: u32 = 0x40;
        const SLOT0_OFF: u32 = 0x48;
        const SLOT1_OFF: u32 = 0x49;
        const NO_SLOT: u32 = 0xff;
        const TABLE_GLOB: u32 = 0x0115d988;
        const STRIDE_GLOB: u32 = 0x0115d964;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_SLOT: u32 = 0x6f10;
        const OP0: u32 = 1;
        const OP1: u32 = 2;
        #[repr(C)]
        struct Scratch {
            _pad: [u8; 8],
            s0: u8,
            s1: u8,
        }
        let mut scr = Scratch { _pad: [0; 8], s0: 0, s1: 0 };
        #[inline(always)]
        unsafe fn target(this: u32, slot_off: u32, table: u32, stride: u32, bank: u32) -> u32 {
            unsafe {
                let slot = (this as *const u8).byte_add(slot_off as usize).read() as u32;
                if slot == NO_SLOT {
                    return 0;
                }
                let row = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
                    .byte_add(ROW_SLOT as usize).read_unaligned();
                row.wrapping_add(stride.wrapping_mul(slot))
            }
        }
        let bank = (this as *const u8).byte_add(BANK_OFF as usize).read() as u32;
        let table = lf_checker_rt::global::<u32>(TABLE_GLOB).read_unaligned();
        let stride = lf_checker_rt::global::<u32>(STRIDE_GLOB).read_unaligned();
        let r0: u32 = lf_checker_rt::callee_thiscall!(
            OP0, u32, target(this, SLOT0_OFF, table, stride, bank),
            (&mut scr.s0 as *mut u8) as u32);
        let r1: u32 = lf_checker_rt::callee_thiscall!(
            OP1, u32, target(this, SLOT1_OFF, table, stride, bank),
            (&mut scr.s1 as *mut u8) as u32);
        if out != 0 && (scr.s0 != 0 || scr.s1 != 0) {
            (out as *mut u8).write(1);
        }
        // SIGNED max (original `cmovg`).
        if (r0 as i32) > (r1 as i32) { r0 } else { r1 }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00537ce0 {
// original: 0x00537ce0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompCarSteal_BG, player_schema::LeaderboardInfo, 10>::vf13

/// Value lookup by id for one ranked leaderboard (vf13).
///
/// Fetch the leaderboard tables through the fetch callee (fastcall: ECX =
/// leaderboard id, EDX = out-frame), then scan the id array. The callee answers
/// in AL (nonzero = ok) and fills count and table pointers into the frame.
/// `id` is scanned for linearly over `count` entries (signed bound: a count
/// at or below zero finds nothing); on a match the parallel value at the
/// same index is returned, otherwise -1. The original re-checks the found
/// index against -1 after the loop, which is unreachable (the index counts
/// up from 0) and is not repeated here. Leaderboard id: 0x20.
/// Original: stdcall of one stack word; incoming ECX is overwritten, not read.
lf_checker_rt::export!(stdcall, rw_00537ce0(id: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x20;
        const FETCH_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut frame = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = frame[3] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let ids = frame[4] as *const u32;
        let values = frame[5];
        let mut i = 0i32;
        loop {
            if i >= count {
                return NOT_FOUND;
            }
            if ids.add(i as usize).read_unaligned() == id {
                break;
            }
            i = i.wrapping_add(1);
        }
        (values.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read_unaligned()
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00a293c0 {
// original: 0x00a293c0 task_aim_assist_init (proposed)

/// Initialise a ped task's aim-assist state from a target, a scale and a
/// parameter block, through one of three assist paths.
///
/// `this` is the task (thiscall, three stack words: `a1` a state block,
/// `a2` a float scale as bits, `a3` a parameter block). The prologue stores
/// `a3+PARAM_OFF` into `+HANDLE_OFF`, fixes several counters and flags (byte
/// 1 at `+READY_OFF`, `0xEA60` at `+TIME_OFF`, zeros, 1.0f at `+RATE_OFF`)
/// and, when bit 1 of the flag byte at `+FLAG_OFF` is set, seeds `+ASSIST_OFF`
/// from `a3+A3_SEED_OFF`. Bit 1 is kept as `cl` below and bit 2 as `flag2`.
///
/// The target path runs when `a1+T Bit4` has bit 2 set and `a1+LINK_OFF` is
/// non-null: with `cl`, `+AUX_OFF` takes `a1+FLOAT_OFF` minus the scale;
/// then the id 1 helper (xmm0 in and out; the rewrite passes the value as a
/// stack word and the stub transports it, comparing only XMM0) runs on the
/// float at `[[LINK]+0x20]+0x18` clamped into `[-1, 1]`, and the answer is
/// added into `+ASSIST_OFF`.
///
/// The search path runs when `a1+ALT_OFF` is null. With `cl`, the id 2
/// search callee runs (thiscall on `this`: two scratch words, `&this+AUX`,
/// `a1`); the scratch addresses are skipped in the comparison (their
/// contents are uninitialised on the original side), while the callee's
/// three one-word
/// out-params (verified from its body) are scripted. A zero answer with both
/// global bytes clear stores `FLOAT+scale` into `+AUX_OFF` when `flag2` is
/// set (otherwise it skips ahead); a set global byte stores the alternate
/// global instead. Clear global bytes then run the dot-product block when
/// `a1+T Bit0` has bit 0: the triple at `a1+DOT_OFF` dotted with
/// `a1[VEC]+0x10` in `(b1+b0)+b2` order, clamped into `[-1, 1]`, answered by
/// id 1 again and subtracted from `+ASSIST_OFF`. Set global bytes run the
/// lookup block instead: the id 3 and id 4 lookup callees (same target, two
/// sites, scripted pointer-or-null each) feed, when non-null and `flag2`,
/// one float each (`+0x304`, `+0x190`) into `+ASSIST_OFF`, re-zeroed when the
/// value is strictly between the reference (exclusive) and zero; two nulls
/// with `flag2` store the id 5 filtered global there.
///
/// The direct path runs otherwise: two triples (from `a1+ALT_OFF`, either
/// `+0x10` or through `+0x20` plus `0x30`, and from `a1+VEC_OFF+0x30`) are
/// differenced, and unless the sum of squares is ordered-equal to zero the
/// differences are normalised by its square root; the id 6 combine callee
/// (cdecl, two scratch words and `&this+AUX`, one scripted out-param word
/// verified from its body) then runs, and the three scaled differences plus
/// the out-param word are stored as four words to `VEC_GLOBAL` (the fourth
/// word re-reads the caller's own scratch, which the contract defines to
/// zero, as does the rewrite).
///
/// The tail always runs: the id 7 rating callee (thiscall, `a1` and 1; its
/// float answer, also written to `a1+0x1f4` by the real callee, is scripted
/// once for both) into `+SCORE_OFF`, then the id 8 pose callee (thiscall on
/// `this+0x10` with the assist and aux words; its eleven output words at
/// `+0x10` verified from its nested callee's body are scripted), and bit 2
/// of the flag byte is set. Ordered compares repeat the original exactly.
/// There is no designed return value (EAX ends as callee residue), so the
/// contract compares everything except it.
///
/// Original: 0x00a293c0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00a293c0(this: u32, a1: u32, a2bits: u32, a3: u32) -> u32 {
    unsafe {
        const PARAM_OFF: u32 = 0x0018;
        const HANDLE_OFF: u32 = 0x01f0;
        const READY_OFF: u32 = 0x023c;
        const TIME_OFF: u32 = 0x0238;
        const TIME_VAL: u32 = 0xea60;
        const ZERO_A: u32 = 0x0240;
        const ZERO_B: u32 = 0x0220;
        const ZERO_C: u32 = 0x0224;
        const RATE_OFF: u32 = 0x0228;
        const ONE_BITS: u32 = 0x3f80_0000;
        const ZERO_D: u32 = 0x0178;
        const ZERO_E: u32 = 0x0174;
        const ZERO_F: u32 = 0x0170;
        const FLAG_OFF: u32 = 0x0216;
        const ASSIST_OFF: u32 = 0x0218;
        const AUX_OFF: u32 = 0x021c;
        const A3_SEED_OFF: u32 = 0x000c;
        const TSTATE_OFF: u32 = 0x026c;
        const LINK_OFF: u32 = 0x0b30;
        const FLOAT_OFF: u32 = 0x0aa0;
        const ALT_OFF: u32 = 0x0398;
        const VEC_OFF: u32 = 0x0020;
        const DOT_OFF: u32 = 0x0b00;
        const SCORE_OFF: u32 = 0x01f4;
        const GLOB_A: u32 = 0x0103_ce46;
        const GLOB_B: u32 = 0x0103_ce47;
        const ALT_GLOBAL: u32 = 0x0128_e3a0;
        const FILT_GLOBAL: u32 = 0x0128_e328;
        const VEC_GLOBAL: u32 = 0x012d_d600;
        const REF_C: u32 = 0x00fe_8d68;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        /// Clamp into [-1, 1] with the original's ordered-compare shape
        /// (both clamp sites map to this; NaN passes through).
        #[inline(always)]
        fn clamp11(v: f32) -> f32 {
            if v > 1.0 {
                1.0
            } else if -1.0 > v {
                -1.0
            } else {
                v
            }
        }

        wr32(this.wrapping_add(HANDLE_OFF), rd32(a3.wrapping_add(PARAM_OFF)));
        wr8(this.wrapping_add(READY_OFF), 1);
        wr32(this.wrapping_add(TIME_OFF), TIME_VAL);
        wr32(this.wrapping_add(ZERO_A), 0);
        wr32(this.wrapping_add(ZERO_B), 0);
        wr32(this.wrapping_add(ZERO_C), 0);
        wr32(this.wrapping_add(RATE_OFF), ONE_BITS);
        wr32(this.wrapping_add(ZERO_D), 0);
        wr32(this.wrapping_add(ZERO_E), 0);
        wr32(this.wrapping_add(ZERO_F), 0);
        let flags = rd8(this.wrapping_add(FLAG_OFF));
        let cl = (flags >> 1) & 1 != 0;
        let flag2 = flags & 2 != 0;
        if cl {
            wr32(this.wrapping_add(ASSIST_OFF), rd32(a3.wrapping_add(A3_SEED_OFF)));
        }
        let a2 = f32::from_bits(a2bits);
        let ga = rd8(lf_checker_rt::relocated(GLOB_A));
        let gb = rd8(lf_checker_rt::relocated(GLOB_B));
        if rd8(a1.wrapping_add(TSTATE_OFF)) & 4 != 0 && rd32(a1.wrapping_add(LINK_OFF)) != 0 {
            if cl {
                let f = rdf(a1.wrapping_add(FLOAT_OFF));
                wrf(this.wrapping_add(AUX_OFF), f - a2);
            }
            let l0 = rd32(a1.wrapping_add(LINK_OFF));
            let l1 = rd32(l0.wrapping_add(0x20));
            let v = clamp11(rdf(l1.wrapping_add(0x18)));
            let ans: u32 = lf_checker_rt::callee_cdecl!(1, u32, v.to_bits());
            let old = rdf(this.wrapping_add(ASSIST_OFF));
            wrf(this.wrapping_add(ASSIST_OFF), f32::from_bits(ans) + old);
        } else if rd32(a1.wrapping_add(ALT_OFF)) == 0 {
            if cl {
                let mut s0 = [0u32; 1];
                let mut s1 = [0u32; 1];
                // Argument order is the reverse of the original's pushes:
                // the last push (a1) is argument 0.
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    2,
                    u32,
                    this,
                    a1,
                    this.wrapping_add(AUX_OFF),
                    s1.as_mut_ptr() as u32,
                    s0.as_mut_ptr() as u32
                );
                if (r as u8) == 0 {
                    if ga == 0 && gb == 0 {
                        if flag2 {
                            let f = rdf(a1.wrapping_add(FLOAT_OFF));
                            wrf(this.wrapping_add(AUX_OFF), f + a2);
                        }
                    } else if flag2 {
                        wrf(this.wrapping_add(AUX_OFF), rdf(lf_checker_rt::relocated(ALT_GLOBAL)));
                    }
                }
            }
            if ga != 0 || gb != 0 {
                let p1: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, 1, 0, 0);
                if p1 != 0 {
                    if flag2 {
                        let x = rdf(p1.wrapping_add(0x304));
                        wrf(this.wrapping_add(ASSIST_OFF), x);
                        let c = rdf(lf_checker_rt::relocated(REF_C));
                        if 0.0 > x && x > c {
                            wr32(this.wrapping_add(ASSIST_OFF), 0);
                        }
                    }
                } else {
                    let p2: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, 2, 0, 0);
                    if p2 != 0 {
                        if flag2 {
                            let x = rdf(p2.wrapping_add(0x190));
                            wrf(this.wrapping_add(ASSIST_OFF), x);
                            let c = rdf(lf_checker_rt::relocated(REF_C));
                            if 0.0 > x && x > c {
                                wr32(this.wrapping_add(ASSIST_OFF), 0);
                            }
                        }
                    } else if flag2 {
                        let d: f32 = lf_checker_rt::callee_cdecl!(
                            5,
                            f32,
                            rd32(lf_checker_rt::relocated(FILT_GLOBAL))
                        );
                        wrf(this.wrapping_add(ASSIST_OFF), d);
                    }
                }
            } else {
                // Both global bytes clear (the no-store sub-path joins here
                // directly, with the same bytes).
                if rd8(a1.wrapping_add(TSTATE_OFF)) & 1 != 0 {
                    let v = rd32(a1.wrapping_add(VEC_OFF));
                    let d0 = rdf(a1.wrapping_add(DOT_OFF)) * rdf(v.wrapping_add(0x10));
                    let d1 = rdf(a1.wrapping_add(DOT_OFF + 4)) * rdf(v.wrapping_add(0x14));
                    let mut dot = d1 + d0;
                    let d2 = rdf(a1.wrapping_add(DOT_OFF + 8)) * rdf(v.wrapping_add(0x18));
                    dot = dot + d2;
                    let ans: u32 =
                        lf_checker_rt::callee_cdecl!(1, u32, clamp11(dot).to_bits());
                    let old = rdf(this.wrapping_add(ASSIST_OFF));
                    wrf(this.wrapping_add(ASSIST_OFF), old - f32::from_bits(ans));
                }
            }
        } else {
            let alt = rd32(a1.wrapping_add(ALT_OFF));
            let link = rd32(alt.wrapping_add(0x20));
            let base = if link != 0 { link.wrapping_add(0x30) } else { alt.wrapping_add(0x10) };
            let v = rd32(a1.wrapping_add(VEC_OFF));
            let x5 = rdf(base.wrapping_add(4)) - rdf(v.wrapping_add(0x34));
            let x4 = rdf(base) - rdf(v.wrapping_add(0x30));
            let x6 = rdf(base.wrapping_add(8)) - rdf(v.wrapping_add(0x38));
            let mut n = x4 * x4;
            n = n + x5 * x5;
            n = n + x6 * x6;
            // The original skips unless the sum is ordered-unequal to zero
            // (ucomiss/lahf/test/jnp); `!=` matches, NaN included.
            let mut inv = 0.0f32;
            if n != 0.0 {
                inv = 1.0 / n.sqrt();
            }
            let m4 = x4 * inv;
            let m0 = inv * x5;
            let m2 = inv * x6;
            // The original passes the m4 slot to id 6, whose stub overwrites
            // it, then stores four words starting there: out-param, m0, m2,
            // and the next scratch word, which nothing ever wrote (zero under
            // the contract's stack fill on both sides).
            let mut slot = m4;
            let mut dummy = [0u32; 1];
            let _: u32 = lf_checker_rt::callee_cdecl!(
                6,
                u32,
                (&mut slot as *mut f32) as u32,
                dummy.as_mut_ptr() as u32,
                this.wrapping_add(AUX_OFF)
            );
            let g = lf_checker_rt::relocated(VEC_GLOBAL);
            wrf(g, slot);
            wrf(g.wrapping_add(4), m0);
            wrf(g.wrapping_add(8), m2);
            wr32(g.wrapping_add(12), 0);
        }
        let f: f32 = lf_checker_rt::callee_thiscall!(7, f32, this, a1, 1);
        wrf(this.wrapping_add(SCORE_OFF), f);
        let w0 = rd32(this.wrapping_add(ASSIST_OFF));
        let w1 = rd32(this.wrapping_add(AUX_OFF));
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, this.wrapping_add(0x10), w0, w1);
        wr8(this.wrapping_add(FLAG_OFF), rd8(this.wrapping_add(FLAG_OFF)) | 2);
        0
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0055fb70 {
// original: 0x0055FB70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_65, player_schema::LeaderboardInfo, 10>::vf14
//
// Poll one ranked leaderboard's rows and reduce them into three outputs.
//
// `this` is the leaderboard-info object (its virtual table supplies the
// handle slot at `+0x2c` and the row-index slot at `+0x30`). The six stack
// arguments are: `base` (the running cursor's start), `payload_out` (8
// bytes receiving one row's payload), `bits_out` (8 bytes receiving a
// one-hot row mask), `flag_out` (one byte receiving the last row-test
// outcome), `ctx` (an opaque context handed to the row callees) and
// `span` (the cursor may advance at most this far past `base`).
//
// Algorithm: clear `bits_out` and `flag_out`, fetch the query handle,
// open query `LEADERBOARD_ID` (which yields the row-handle table), then
// run 19 iterations. Iteration `i` maps to a row index through the index
// slot; a skip test against `ctx` may pass it over. Otherwise the row's
// kind decides the
// cursor step (8 for kinds 1, 2, 3 and 5, else 0). When the handle equals
// the iteration number the row object is fetched and, if its signed
// size is at most 8 (a negative size copies), its payload copied to
// `payload_out` and `flag_out` set.
// Otherwise the cursor advances by the step: it must stay within
// `base + span`, and a place call must accept the row, whose bit is then
// written into `bits_out` (overwriting the previous iteration's bit).
// The loop stops early when an iteration reports failure. Returns the
// last outcome byte in `al` (upper bytes are the last callee's leftover).
//
// Edge cases: a rejected query returns 0 with the outputs cleared; a null
// row or an oversize one clears the flag; a wrapped `base + span` fails
// the bound check. The kind switch reads a table of code addresses; the
// five entries collapse to the two step values above.
//
// Original: 0x0055FB70 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_0055FB70(this: u32, base: u32, payload_out: u32, bits_out: u32, flag_out: u32, ctx_arg: u32, span: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x10C;
        const ITERATIONS: u32 = 19;
        const STEP_WIDE: u32 = 8;
        const VT_HANDLE: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const Q_ROWS_WORD: usize = 2;
        const ROW_PAYLOAD_OFF: u32 = 4;
        const PAYLOAD_MAX: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }

        wr32(bits_out, 0);
        wr32(bits_out.wrapping_add(4), 0);
        (flag_out as *mut u8).write(0);
        let limit = span.wrapping_add(base);
        let vtable = rd32(this);
        let handle_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_HANDLE)) as usize);
        let handle = handle_of(this);
        // Query block shared with the open callee: it stores the row table
        // at word 2, the only word either side reads afterwards.
        let mut query = [0u32, 0u32, 0u32];
        let opened: u32 = lf_checker_rt::callee_fastcall!(
            3, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if (opened & 0xff) == 0 {
            return 0;
        }
        let rows = query[Q_ROWS_WORD];
        let index_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_INDEX)) as usize);
        let mut ok: u8 = 1;
        let mut cursor = base;
        let mut i = 0u32;
        while i < ITERATIONS {
            if ok == 0 {
                break;
            }
            let idx = index_of(this, i);
            let skipped: u32 =
                lf_checker_rt::callee_thiscall!(4, u32, ctx_arg, idx);
            if (skipped & 0xff) == 0 {
                ok = 0;
                let row_handle = rd32(rows.wrapping_add(idx.wrapping_mul(4)));
                let kind: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, row_handle);
                let step = if matches!(kind, 1 | 2 | 3 | 5) { STEP_WIDE } else { 0 };
                if handle == i {
                    let row: u32 =
                        lf_checker_rt::callee_thiscall!(6, u32, ctx_arg, idx);
                    ok = 0;
                    if row != 0 {
                        let size: u32 =
                            lf_checker_rt::callee_thiscall!(7, u32, row);
                        // Signed compare (jg): a negative size copies.
                        if (size as i32) <= PAYLOAD_MAX as i32 {
                            wr64(payload_out, rd64(row.wrapping_add(ROW_PAYLOAD_OFF)));
                            ok = 1;
                        }
                    }
                    (flag_out as *mut u8).write(ok);
                } else {
                    // The place call sees the cursor from before this
                    // iteration's step (the original passes its spill slot,
                    // which the loop end refreshes only afterwards).
                    let prev = cursor;
                    cursor = cursor.wrapping_add(step);
                    if cursor > limit {
                        ok = 0;
                    } else {
                        let placed: u32 = lf_checker_rt::callee_thiscall!(
                            8, u32, ctx_arg, idx, prev, step);
                        if (placed & 0xff) == 0 {
                            ok = 0;
                        } else {
                            ok = 1;
                            let bit = 1u32 << (i & 31);
                            let (lo, hi) = if i < 0x20 {
                                (bit, 0)
                            } else if i < 0x40 {
                                (0, bit)
                            } else {
                                (0, 0)
                            };
                            wr32(bits_out, lo);
                            wr32(bits_out.wrapping_add(4), hi);
                        }
                    }
                }
            }
            i += 1;
        }
        ok as u32
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00ae1a90 {
// original: 0x00ae1a90 slot_range_resolve (proposed)

/// Resolve the active index range of the current slot and store it.
///
/// Takes no arguments (cdecl, nothing read from the incoming stack). The
/// slot is looked up through the registry global: a null registry, a
/// negative selector, or a selector at or above the table's 16-bit count
/// all mean "no slot" and the call ends with no writes. Otherwise the slot
/// is the selector-th slot pointer of the table.
///
/// With a slot, the helper object (thiscall on a fixed object, no
/// arguments) and the live-bits global decide the guard: kind is the slot's
/// first byte minus one. Kind 8 needs a non-negative word at `+0x10` and
/// kind 9 needs nothing further, but either returns early -- copying the
/// base index at `+0x14` to both outputs at `+0x18`/`+0x1c` -- when the
/// live bits or the helper answer is null. Any other kind stores 0 and
/// `0xffffffff` to the outputs and returns.
///
/// Kind 8 scans forward then backward. Each scan starts from the resolver
/// callee (base global added to the slot's `+0x14`) refined through the
/// locator callee, then walks one step at a time while the probe callee
/// (slot index, two scratch buffers, live word) and the test callee
/// (thiscall on the helper answer, same two buffers) both answer true;
/// the forward walk stops at the limit global, the backward walk at zero,
/// each with a one-step overshoot correction. The locator's `+0xc` word
/// minus the base global lands in `+0x1c` (forward) and `+0x18`
/// (backward). A final measure call (whose third argument reuses the
/// locator's still-pushed argument) widens `+0x18` up and the limiter
/// call narrows `+0x1c` down, both comparisons unsigned.
///
/// Kind 9 has the same skeleton with the probe called as
/// (index, live word, buffer, -1, 0) and the second test callee taking
/// (slot, buffer): the forward walk passes the helper answer, while the
/// backward walk reloads the helper answer from its spill slot (the load
/// reads the same saved word the forward walk's setup uses) and passes it
/// along; it reloads its candidate from the current index each round and
/// stops at zero.
///
/// Original: 0x00ae1a90 (cdecl, no stack arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ae1a90() -> u32 {
    unsafe {
        const SLOT_KIND: u32 = 0x00;
        const SLOT_INDEX: u32 = 0x10;
        const SLOT_BASE: u32 = 0x14;
        const SLOT_LO: u32 = 0x18;
        const SLOT_HI: u32 = 0x1c;
        const REG_SELECT: u32 = 0xf8;
        const REG_TABLE: u32 = 0x9c;
        const TAB_COUNT: u32 = 0x04;
        const LIVE_WORD: u32 = 0x64;
        const G_REG: u32 = 0x01593b6c;
        const G_LIVE: u32 = 0x011f70fc;
        const G_BASE: u32 = 0x011f7028;
        const G_LIMIT: u32 = 0x011f707c;
        const HELPER_OBJ: u32 = 0x0103e498;
        const CALLEE_HELPER: u32 = 1;
        const CALLEE_RESOLVE: u32 = 2;
        const CALLEE_LOCATE: u32 = 3;
        const CALLEE_PROBE: u32 = 4;
        const CALLEE_PROBE9: u32 = 9;
        const CALLEE_TEST: u32 = 5;
        const CALLEE_MEASURE: u32 = 6;
        const CALLEE_LIMIT: u32 = 7;
        const CALLEE_TEST2: u32 = 8;

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

        let reg = rd32(lf_checker_rt::relocated(G_REG));
        let mut slot: u32 = 0;
        if reg != 0 {
            let sel = rd32(reg.wrapping_add(REG_SELECT));
            if (sel as i32) >= 0 {
                let tab = rd32(reg.wrapping_add(REG_TABLE));
                if sel < rd16(tab.wrapping_add(TAB_COUNT)) as u32 {
                    slot = rd32(rd32(tab).wrapping_add(sel.wrapping_mul(4)));
                }
            }
        }
        if slot == 0 {
            return 0;
        }
        let live0 = rd32(lf_checker_rt::relocated(G_LIVE));
        let helper: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_HELPER,
            u32,
            lf_checker_rt::relocated(HELPER_OBJ),
        );
        let kind = rd8(slot.wrapping_add(SLOT_KIND)).wrapping_sub(1);
        let base = rd32(lf_checker_rt::relocated(G_BASE));
        let limit = rd32(lf_checker_rt::relocated(G_LIMIT));
        if kind == 8 {
            if rd32(slot.wrapping_add(SLOT_INDEX)) == 0xffffffff {
                wr32(slot.wrapping_add(SLOT_LO), 0);
                wr32(slot.wrapping_add(SLOT_HI), 0xffffffff);
                return 0;
            }
            if live0 == 0 || helper == 0 {
                let b = rd32(slot.wrapping_add(SLOT_BASE));
                wr32(slot.wrapping_add(SLOT_LO), b);
                wr32(slot.wrapping_add(SLOT_HI), b);
                return 0;
            }
            let live_word = rd32(live0.wrapping_add(LIVE_WORD));
            let mut buf_a: u32 = 0;
            let mut buf_b: u32 = 0;
            let pa = core::ptr::addr_of_mut!(buf_a) as u32;
            let pb = core::ptr::addr_of_mut!(buf_b) as u32;
            // Forward scan.
            let r = lf_checker_rt::callee_cdecl!(
                CALLEE_RESOLVE,
                u32,
                rd32(slot.wrapping_add(SLOT_BASE)).wrapping_add(base)
            );
            let l = lf_checker_rt::callee_cdecl!(
                CALLEE_LOCATE,
                u32,
                rd32(r.wrapping_add(0x1c)).wrapping_add(1)
            );
            let mut idx = rd32(l.wrapping_add(0x1c));
            if idx < limit {
                loop {
                    let ok: u32 = lf_checker_rt::callee_cdecl!(
                        CALLEE_PROBE,
                        u32,
                        idx,
                        live_word,
                        pb,
                        rd32(slot.wrapping_add(SLOT_INDEX)),
                        pa
                    );
                    if (ok & 0xff) == 0 {
                        break;
                    }
                    let ok2: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_TEST, u32, helper, pb, pa);
                    if (ok2 & 0xff) == 0 {
                        idx = idx.wrapping_sub(1);
                        break;
                    }
                    idx = idx.wrapping_add(1);
                    if idx >= limit {
                        break;
                    }
                }
            }
            if idx == limit {
                idx = idx.wrapping_sub(1);
            }
            let l2 = lf_checker_rt::callee_cdecl!(CALLEE_LOCATE, u32, idx);
            wr32(
                slot.wrapping_add(SLOT_HI),
                rd32(l2.wrapping_add(0x0c)).wrapping_sub(base),
            );
            // Backward scan.
            let r = lf_checker_rt::callee_cdecl!(
                CALLEE_RESOLVE,
                u32,
                rd32(slot.wrapping_add(SLOT_BASE)).wrapping_add(base)
            );
            let mut idx = rd32(r.wrapping_add(0x1c));
            if (idx as i32) >= 0 {
                loop {
                    let ok: u32 = lf_checker_rt::callee_cdecl!(
                        CALLEE_PROBE,
                        u32,
                        idx,
                        live_word,
                        pb,
                        rd32(slot.wrapping_add(SLOT_INDEX)),
                        pa
                    );
                    if (ok & 0xff) == 0 {
                        break;
                    }
                    let ok2: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_TEST, u32, helper, pb, pa);
                    if (ok2 & 0xff) == 0 {
                        idx = idx.wrapping_add(1);
                        break;
                    }
                    idx = idx.wrapping_sub(1);
                    if (idx as i32) < 0 {
                        break;
                    }
                }
                if (idx as i32) < 0 {
                    idx = 0;
                }
            } else {
                idx = 0;
            }
            let l3 = lf_checker_rt::callee_cdecl!(CALLEE_LOCATE, u32, idx);
            wr32(
                slot.wrapping_add(SLOT_LO),
                rd32(l3.wrapping_add(0x0c)).wrapping_sub(base),
            );
            let m = lf_checker_rt::callee_cdecl!(
                CALLEE_MEASURE,
                u32,
                1u32,
                rd32(slot.wrapping_add(SLOT_INDEX)),
                idx
            );
            let mut floor: u32 = 0;
            if m != 0 {
                floor = rd32(m.wrapping_add(0x68));
            }
            let cap: u32 = lf_checker_rt::callee_cdecl!(
                CALLEE_LIMIT,
                u32,
                rd32(slot.wrapping_add(SLOT_INDEX))
            );
            if rd32(slot.wrapping_add(SLOT_LO)) < floor {
                wr32(slot.wrapping_add(SLOT_LO), floor);
            }
            if rd32(slot.wrapping_add(SLOT_HI)) > cap {
                wr32(slot.wrapping_add(SLOT_HI), cap);
            }
            return 0;
        }
        if kind == 9 {
            if live0 == 0 || helper == 0 {
                let b = rd32(slot.wrapping_add(SLOT_BASE));
                wr32(slot.wrapping_add(SLOT_LO), b);
                wr32(slot.wrapping_add(SLOT_HI), b);
                return 0;
            }
            let live_word = rd32(live0.wrapping_add(LIVE_WORD));
            let mut buf_b: u32 = 0;
            let pb = core::ptr::addr_of_mut!(buf_b) as u32;
            // Forward scan.
            let r = lf_checker_rt::callee_cdecl!(
                CALLEE_RESOLVE,
                u32,
                rd32(slot.wrapping_add(SLOT_BASE)).wrapping_add(base)
            );
            let mut idx = rd32(r.wrapping_add(0x1c));
            if idx < limit {
                loop {
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        CALLEE_PROBE9,
                        u32,
                        idx,
                        live_word,
                        pb,
                        0xffffffffu32,
                        0u32
                    );
                    let ok: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_TEST2, u32, helper, slot, pb);
                    if (ok & 0xff) == 0 {
                        if idx != 0 {
                            idx = idx.wrapping_sub(1);
                        }
                        break;
                    }
                    idx = idx.wrapping_add(1);
                    if idx >= limit {
                        break;
                    }
                }
            }
            if idx == limit {
                idx = idx.wrapping_sub(1);
            }
            let l2 = lf_checker_rt::callee_cdecl!(CALLEE_LOCATE, u32, idx);
            wr32(
                slot.wrapping_add(SLOT_HI),
                rd32(l2.wrapping_add(0x0c)).wrapping_sub(base),
            );
            // Backward scan.
            let r = lf_checker_rt::callee_cdecl!(
                CALLEE_RESOLVE,
                u32,
                rd32(slot.wrapping_add(SLOT_BASE)).wrapping_add(base)
            );
            let mut idx = rd32(r.wrapping_add(0x1c));
            if idx != 0 {
                loop {
                    let cand = idx.wrapping_sub(1);
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        CALLEE_PROBE9,
                        u32,
                        cand,
                        live_word,
                        pb,
                        0xffffffffu32,
                        0u32
                    );
                    // The original reloads the spilled helper answer here.
                    let ok: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_TEST2, u32, helper, slot, pb);
                    if (ok & 0xff) == 0 {
                        idx = idx.wrapping_add(1);
                        break;
                    }
                    idx = cand;
                    if idx == 0 {
                        break;
                    }
                }
            }
            let l3 = lf_checker_rt::callee_cdecl!(CALLEE_LOCATE, u32, idx);
            wr32(
                slot.wrapping_add(SLOT_LO),
                rd32(l3.wrapping_add(0x0c)).wrapping_sub(base),
            );
            return 0;
        }
        wr32(slot.wrapping_add(SLOT_LO), 0);
        wr32(slot.wrapping_add(SLOT_HI), 0xffffffff);
        0
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_005769c0 {
// original: 0x005769c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_149, player_schema::LeaderboardInfo, 10>::vf12

/// Map a leaderboard value slot back to its key index, or -1 when absent.
///
/// `this` (ECX) is ignored; `index` addresses the values table with no
/// bounds check. Fetches the leaderboard tables for id 0x16d
/// through the fetch callee into a six-word scratch structure: words 1..3
/// are the first (count, keys) pair, word 5 is the values pointer. When
/// the fetch fails (low byte of the answer is zero) returns -1. Otherwise
/// reads `want = values[index]`; when that is -1 returns -1 without
/// searching. When the count is zero returns -1; otherwise linearly scans
/// `keys[0..count]` for `want` with an unsigned bound and returns the
/// first matching index, or -1 when no element matches.
///
/// Original: 0x005769c0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_005769c0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x16d;
        const FETCH: u32 = 0;
        const MISSING: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 6];
        out[1] = 0;
        out[2] = 0;
        out[5] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return MISSING;
        }
        let vals = out[5];
        let want = rd(vals.wrapping_add(index.wrapping_mul(4)));
        if want == MISSING {
            return MISSING;
        }
        let count = out[1];
        if count == 0 {
            return MISSING;
        }
        let keys = out[2];
        let mut i = 0u32;
        while i < count {
            if rd(keys.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i = i.wrapping_add(1);
        }
        MISSING
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00dc87b0 {
// original: 0x00DC87B0 blend_task_vector (proposed)

use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

/// Blend a task's target vector toward a probe result, or copy the source
/// vector through when either probe refuses.
///
/// `this` holds a source vector at `+0x20` (four words) and a target vector
/// at `+0x30` (four words). `p` points at an object whose word at `+0x20`
/// points at a float block. A classifier object is prepared from the word at
/// `this+0x40`, the seed float at block `+0x38` and the constant 0.25 (nine
/// stack words, as in the neighbouring row picker), then asked about the
/// source vector. A zero low byte of its answer copies the source vector
/// over the target and returns the copy's last source word.
///
/// Otherwise the offset from the block's point (`+0x30..+0x38`) to the
/// target point is normalised (reciprocal length, zero when the squared
/// length compares equal to zero, which also covers NaN by taking the
/// divide path) and a second probe runs against the classifier with three
/// scratch outputs and the target vector. Its zero answer copies the source
/// vector over the target; a nonzero answer blends: each of three scratch
/// words scaled by 0.25 is added to a base word, the three sums land at
/// target `+0x00/+0x08/+0x04` (note the order) and a seventh scratch word
/// lands at target `+0x0c`. All float operation orders are the original's,
/// including the deliberately mixed orders in the normalisation
/// (`x*inv`, `inv*y`, `inv*z`). The blend path returns the probe's full
/// answer; both copy paths return the last source word, which the copy
/// leaves in the return register.
///
/// Original: 0x00DC87B0 (thiscall, one stack word).
export!(thiscall, rw_dc87b0(this: u32, p: u32) -> u32 {
    const SRC_VEC: u32 = 0x20;
    const DST_VEC: u32 = 0x30;
    const KEY_OFF: u32 = 0x40;
    const BLK_OFF: u32 = 0x20;
    const SEED_OFF: u32 = 0x38;
    const QUARTER: f32 = 0.25;
    const ONE: f32 = 1.0;
    const CAL_PREP: u32 = 1;
    const CAL_ASK: u32 = 2;
    const CAL_PROBE: u32 = 3;
    const CAL_COOKIE: u32 = 4;

    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    #[inline(always)]
    fn sub(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) - core::hint::black_box(b)
    }
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn wrf(a: u32, v: f32) {
        unsafe { wr32(a, v.to_bits()) }
    }
    unsafe fn copy4(dst: u32, src: u32) {
        unsafe {
            wr32(dst, rd32(src));
            wrf(dst.wrapping_add(4), rdf(src.wrapping_add(4)));
            wrf(dst.wrapping_add(8), rdf(src.wrapping_add(8)));
            wr32(dst.wrapping_add(12), rd32(src.wrapping_add(12)));
        }
    }

    unsafe {
        let dst = this.wrapping_add(DST_VEC);
        let srcv = this.wrapping_add(SRC_VEC);
        let block = rd32(p.wrapping_add(BLK_OFF));
        let seed = rdf(block.wrapping_add(SEED_OFF));
        let obj = [0u32; 1];
        let base = obj.as_ptr() as u32;
        callee_thiscall!(CAL_PREP, u32, base, rd32(this.wrapping_add(KEY_OFF)), seed.to_bits(), QUARTER.to_bits(), 0, 0, 0, 0, 0, 0);
        let ask: u32 = callee_thiscall!(CAL_ASK, u32, base, srcv);
        if (ask & 0xff) == 0 {
            copy4(dst, srcv);
            callee_cdecl!(CAL_COOKIE, u32, );
            // The copy leaves the last source word in the return register.
            return rd32(srcv.wrapping_add(12));
        }
        // Offset from the block's point to the target point, normalised.
        let blk_pt = block.wrapping_add(DST_VEC);
        let x = sub(rdf(dst), rdf(blk_pt));
        let y = sub(rdf(dst.wrapping_add(4)), rdf(blk_pt.wrapping_add(4)));
        let z = sub(rdf(dst.wrapping_add(8)), rdf(blk_pt.wrapping_add(8)));
        let xx = mul(x, x);
        let yy = mul(y, y);
        let len2 = add(add(xx, yy), mul(z, z));
        // The original's ucomiss/lahf/test/jnp skips the divide exactly
        // when the squared length compares equal to +0.0 (NaN takes the
        // divide path, as here, since NaN != 0.0).
        let inv = if len2 == 0.0 {
            0.0
        } else {
            core::hint::black_box(ONE) / core::hint::black_box(len2.sqrt())
        };
        let _nx = mul(x, inv);
        let _ny = mul(inv, y);
        let _nz = mul(inv, z);
        // The normalised offset is consumed only by the probe's scratch
        // protocol on the original side; the stub answers without reading
        // it, so only the calls and their order are reproduced here.
        let mut w_p1 = [0u32; 3];
        let mut w_p2 = [0u32; 4];
        let mut w_p3 = [0u32; 3];
        // Argument order is last-pushed-first: the target vector (pushed
        // last) is arg0, then the three scratch pointers in reverse.
        let probe: u32 = callee_thiscall!(CAL_PROBE, u32, base, dst, w_p3.as_mut_ptr() as u32, w_p2.as_mut_ptr() as u32, w_p1.as_mut_ptr() as u32);
        if (probe & 0xff) == 0 {
            copy4(dst, srcv);
            callee_cdecl!(CAL_COOKIE, u32, );
            // As above: the copy's last source word is the return value.
            return rd32(srcv.wrapping_add(12));
        }
        let b0 = f32::from_bits(w_p1[0]);
        let b1 = f32::from_bits(w_p1[1]);
        let b2 = f32::from_bits(w_p1[2]);
        let s0 = mul(f32::from_bits(w_p3[0]), QUARTER);
        let s1 = mul(f32::from_bits(w_p3[1]), QUARTER);
        let s2 = mul(f32::from_bits(w_p3[2]), QUARTER);
        wrf(dst, add(b0, s0));
        wrf(dst.wrapping_add(8), add(b2, s2));
        wrf(dst.wrapping_add(4), add(b1, s1));
        wr32(dst.wrapping_add(12), w_p2[3]);
        callee_cdecl!(CAL_COOKIE, u32, );
        probe
    }
});

use lf_checker_rt::{callee_stdcall, callee_fastcall, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0058b710 {
// original: 0x0058B710 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_225, player_schema::LeaderboardInfo, 10>::vf14
///
/// Poll one ranked leaderboard's rows and reduce them into three outputs.
///
/// `this` is the leaderboard-info object (its virtual table supplies the
/// handle slot at `+0x2c` and the row-index slot at `+0x30`). The six stack
/// arguments are: `base` (the running cursor's start), `payload_out` (8
/// bytes receiving one row's payload), `bits_out` (8 bytes receiving a
/// one-hot row mask), `flag_out` (one byte receiving the last row-test
/// outcome), `ctx` (an opaque context handed to the row callees) and
/// `span` (the cursor may advance at most this far past `base`).
///
/// Algorithm: clear `bits_out` and `flag_out`, fetch the query handle,
/// open query `LEADERBOARD_ID` (which yields the row-handle table), then
/// run 19 iterations. Iteration `i` maps to a row index through the index
/// slot; a skip test against `ctx` may pass it over. Otherwise the row's
/// kind decides the
/// cursor step (8 for kinds 1, 2, 3 and 5, else 0). When the handle equals
/// the iteration number the row object is fetched and, if its signed
/// size is at most 8 (a negative size copies), its payload copied to
/// `payload_out` and `flag_out` set.
/// Otherwise the cursor advances by the step: it must stay within
/// `base + span`, and a place call must accept the row, whose bit is then
/// written into `bits_out` (overwriting the previous iteration's bit).
/// The loop stops early when an iteration reports failure. Returns the
/// last outcome byte in `al` (upper bytes are the last callee's leftover).
///
/// Edge cases: a rejected query returns 0 with the outputs cleared; a null
/// row or an oversize one clears the flag; a wrapped `base + span` fails
/// the bound check. The kind switch reads a table of code addresses; the
/// five entries collapse to the two step values above.
///
/// Original: 0x0058B710 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_0058B710(this: u32, base: u32, payload_out: u32, bits_out: u32, flag_out: u32, ctx_arg: u32, span: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1B9;
        const ITERATIONS: u32 = 19;
        const STEP_WIDE: u32 = 8;
        const VT_HANDLE: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const Q_ROWS_WORD: usize = 2;
        const ROW_PAYLOAD_OFF: u32 = 4;
        const PAYLOAD_MAX: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }

        wr32(bits_out, 0);
        wr32(bits_out.wrapping_add(4), 0);
        (flag_out as *mut u8).write(0);
        let limit = span.wrapping_add(base);
        let vtable = rd32(this);
        let handle_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_HANDLE)) as usize);
        let handle = handle_of(this);
        // Query block shared with the open callee: it stores the row table
        // at word 2, the only word either side reads afterwards.
        let mut query = [0u32, 0u32, 0u32];
        let opened: u32 = lf_checker_rt::callee_fastcall!(
            3, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if (opened & 0xff) == 0 {
            return 0;
        }
        let rows = query[Q_ROWS_WORD];
        let index_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_INDEX)) as usize);
        let mut ok: u8 = 1;
        let mut cursor = base;
        let mut i = 0u32;
        while i < ITERATIONS {
            if ok == 0 {
                break;
            }
            let idx = index_of(this, i);
            let skipped: u32 =
                lf_checker_rt::callee_thiscall!(4, u32, ctx_arg, idx);
            if (skipped & 0xff) == 0 {
                ok = 0;
                let row_handle = rd32(rows.wrapping_add(idx.wrapping_mul(4)));
                let kind: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, row_handle);
                let step = if matches!(kind, 1 | 2 | 3 | 5) { STEP_WIDE } else { 0 };
                if handle == i {
                    let row: u32 =
                        lf_checker_rt::callee_thiscall!(6, u32, ctx_arg, idx);
                    ok = 0;
                    if row != 0 {
                        let size: u32 =
                            lf_checker_rt::callee_thiscall!(7, u32, row);
                        // Signed compare (jg): a negative size copies.
                        if (size as i32) <= PAYLOAD_MAX as i32 {
                            wr64(payload_out, rd64(row.wrapping_add(ROW_PAYLOAD_OFF)));
                            ok = 1;
                        }
                    }
                    (flag_out as *mut u8).write(ok);
                } else {
                    // The place call sees the cursor from before this
                    // iteration's step (the original passes its spill slot,
                    // which the loop end refreshes only afterwards).
                    let prev = cursor;
                    cursor = cursor.wrapping_add(step);
                    if cursor > limit {
                        ok = 0;
                    } else {
                        let placed: u32 = lf_checker_rt::callee_thiscall!(
                            8, u32, ctx_arg, idx, prev, step);
                        if (placed & 0xff) == 0 {
                            ok = 0;
                        } else {
                            ok = 1;
                            let bit = 1u32 << (i & 31);
                            let (lo, hi) = if i < 0x20 {
                                (bit, 0)
                            } else if i < 0x40 {
                                (0, bit)
                            } else {
                                (0, 0)
                            };
                            wr32(bits_out, lo);
                            wr32(bits_out.wrapping_add(4), hi);
                        }
                    }
                }
            }
            i += 1;
        }
        ok as u32
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0053ff50 {
// original: 0x0053ff50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_CoopSwatAssault_BG_TIME, player_schema::LeaderboardInfo, 10>::vf12
///
/// leaderboard key search by row index: find the position of one row's key inside the key table.
///
/// Asks the helper (callee 1, id 0x4e) for the board's key list (count at
/// out offset 4, key-table pointer at offset 8) and the row table (pointer at
/// offset 20). Looks up the key of row `index` in the row table; a missing
/// row (key NOT_FOUND) or a helper failure yields NOT_FOUND. Then scans the
/// key table linearly (unsigned bound) for that key and returns its position,
/// or NOT_FOUND when the table is empty or holds no such key.
///
/// Original: thiscall/1, the callee pops 4 bytes.
#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

lf_checker_rt::export!(thiscall, rw_0053ff50(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x4e;
        const COUNT_SLOT: usize = 1;
        const KEYS_SLOT: usize = 2;
        const ROWS_SLOT: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let wanted = rd32(out[ROWS_SLOT].wrapping_add(index.wrapping_mul(4)));
        if wanted == NOT_FOUND {
            return NOT_FOUND;
        }
        let (count, keys) = (out[COUNT_SLOT], out[KEYS_SLOT]);
        if count == 0 {
            return NOT_FOUND;
        }
        for i in 0..count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == wanted {
                return i;
            }
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_005888d0 {
// original: 0x005888D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_214, player_schema::LeaderboardInfo, 10>::vf7

/// Board query vf7 of one ranked-race leaderboard instantiation.
///
/// Returns the table entry at `index`, or all-ones when the helper
/// fails. There is no bounds check: the caller guarantees the index.
///
/// TABLE_SLOT (word 4): value table for the index argument.
/// Board id 0x1ae, passed to the helper in ecx.
///
/// Original: stdcall, one stack word, no register inputs (ecx is set to the
/// board id before the helper call), return value in eax, no heap or global
/// writes.
lf_checker_rt::export!(stdcall, rw_005888D0(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema helper in ecx.
        const BOARD_ID: u32 = 0x1ae;
        /// Helper callee id in the proof contract.
        const HELPER: u32 = 1;
        /// Helper out-slot holding the value table.
        const TABLE_SLOT: usize = 4;
        /// Failure marker, also the table-miss value.
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut out = [0u32; 5];
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, BOARD_ID, out.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let table = out[TABLE_SLOT];
        let entry = ((table.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        entry
    }
});


use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00560f10 {
// original: 0x00560f10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_69, player_schema::LeaderboardInfo, 10>::vf8

/// Classifies one entry of this board's value table into a width.
/// 
/// The loader callee (id `LEADERBOARD_ID`) fills one out word through a
/// frame pointer: the value array base (at `+0x14`). When the loader
/// reports failure the result is 0. Otherwise `base[index]` is passed (in
/// ECX) to the rank callee; a rank of -1 yields 0, and the rank minus one
/// selects the width from [4, 8, 8, 0, 4], with any rank outside 1..=5
/// yielding 0.
/// 
/// Original: 0x00560f10 (stdcall, one stack word; the rank switch is a
/// five-entry jump table in the original). Episodic race board 69; only
/// the loader id differs between instantiations.
lf_checker_rt::export!(stdcall, rw_00560f10(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x110;
        const OUT_VALUES: usize = 5;
        const WIDTHS: [u32; 5] = [4, 8, 8, 0, 4];
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let values = out[OUT_VALUES] as *const u32;
        let entry = values.add(index as usize).read_unaligned();
        let rank: u32 = lf_checker_rt::callee_thiscall!(2, u32, entry);
        if rank == 0xffff_ffff {
            return 0;
        }
        let sel = rank.wrapping_sub(1);
        if sel > 4 {
            return 0;
        }
        WIDTHS[sel as usize]
    }

});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0058f170 {
// original: 0x0058F170 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_238, player_schema::LeaderboardInfo, 10>::vf6
/// Leaderboard key lookup: find `wanted` in the race's key table.
/// Calls the leaderboard helper (id 0x1c6) with a five-word scratch
/// record; on success the helper leaves the entry count at record word 3
/// (`+0x0c`) and the key-table pointer at word 4 (`+0x10`). Returns the
/// zero-based index of the first matching key, or 0xFFFFFFFF when the
/// helper fails, the count is not positive, or no key matches. The count
/// is treated as signed: a negative count misses without reading the table.
/// Original: 0x0058F170 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0058f170(wanted: u32) -> u32 {
    unsafe {
        const RACE_ID: u32 = 0x1c6;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
        const MISS: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 5];
        let ok = lf_checker_rt::callee_fastcall!(1, u32, RACE_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISS;
        }
        let count = info[COUNT_WORD] as i32;
        if count <= 0 {
            return MISS;
        }
        let keys = info[KEYS_WORD];
        let mut i = 0i32;
        while i < count {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == wanted {
                return i as u32;
            }
            i += 1;
        }
        MISS
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00b0d190 {
// original: 0x00b0d190 net_position_query_hook (proposed)

/// Build a spatial query around an object's position and fire its hook.
///
/// `obj` points to an object with a vtable pointer at `+0`, a position
/// triple at `+0x10`, and an optional sub-object link at `+0x20`. The query
/// centre comes from the sub-object's triple at `+0x30` when the link is
/// non-null, otherwise from the object's own triple. The function fills a
/// stack descriptor with that centre (once plain, once with the height
/// lowered by `DROP` (0.3) and once raised by `LIFT` (1.5)), three copies of
/// a static triple, and flag words, then asks callee 1 (seven stack
/// arguments: three frame pointers, the object, and the constants 6, 1, 4),
/// which answers 0 or non-zero and delivers a result triple through the
/// frame. On a non-zero answer the result's height is raised back by `DROP`
/// and the object's hook (vtable slot `+8`, thiscall with the object in
/// `ecx` and stack arguments (a pointer to the result triple, 0, 0)) runs.
/// Returns nothing observable.
///
/// Only the position block of the descriptor is observed (eight snapshotted
/// words); the static-triple copies and flag words past it are overwritten
/// by the callee's scripted answer or never read back, so no comparison can
/// see them.
///
/// Original: 0x00b0d190 (cdecl, one stack word, no return value).
lf_checker_rt::export!(cdecl, rw_00b0d190(obj: u32) -> u32 {
    unsafe {
        const POS_LINK: u32 = 0x20;
        const POS_FALLBACK: u32 = 0x10;
        const SUB_POS: u32 = 0x30;
        const VTABLE_HOOK: u32 = 0x08;
        const QUERY_CALLEE: u32 = 1;
        const DROP: f32 = f32::from_bits(0x3E99_999A);
        const LIFT: f32 = f32::from_bits(0x3FC0_0000);
        const STATIC_X: u32 = 0x01B4_B320;
        const STATIC_Y: u32 = 0x01B4_B324;
        const STATIC_Z: u32 = 0x01B4_B328;
        // Descriptor words, relative to its base (callee arg 1).
        const W_PX: usize = 0;
        const W_PY: usize = 1;
        const W_PZ_LO: usize = 2;
        const W_RX: usize = 4;
        const W_RY: usize = 5;
        const W_RZ: usize = 6;
        const W_ZERO: usize = 8;
        const W_ANS: usize = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let link = rd32(obj + POS_LINK);
        let base = if link != 0 {
            link.wrapping_add(SUB_POS)
        } else {
            obj.wrapping_add(POS_FALLBACK)
        };
        let px = rdf(base);
        let py = rdf(base + 4);
        let pz = rdf(base + 8);
        let sx = rdf(lf_checker_rt::relocated(STATIC_X));
        let sy = rdf(lf_checker_rt::relocated(STATIC_Y));
        let sz = rdf(lf_checker_rt::relocated(STATIC_Z));
        let pz_lo = sub(pz, DROP);
        let pz_hi = add(pz, LIFT);
        // Zeroed first: the holes between the set words read as zero.
        let mut st = [0u32; 32];
        st[W_PX] = px.to_bits();
        st[W_PY] = py.to_bits();
        st[W_PZ_LO] = pz_lo.to_bits();
        st[W_RX] = px.to_bits();
        st[W_RY] = py.to_bits();
        st[W_RZ] = pz_hi.to_bits();
        st[W_ZERO] = 0;
        st[W_ANS] = sx.to_bits();
        st[W_ANS + 1] = sy.to_bits();
        st[W_ANS + 2] = sz.to_bits();
        st[W_ANS + 4] = sx.to_bits();
        st[W_ANS + 5] = sy.to_bits();
        st[W_ANS + 6] = sz.to_bits();
        st[W_ANS + 8] = sx.to_bits();
        st[W_ANS + 9] = sy.to_bits();
        st[W_ANS + 10] = sz.to_bits();
        st[24] = 0;
        st[25] = 0;
        st[26] = 0;
        st[27] = 0xFFFF;
        // st[28] holds the trailing zero byte and zero word.
        let p2 = st.as_mut_ptr();
        let p3 = p2.add(W_RX) as u32;
        let p1 = p2.add(W_ZERO) as u32;
        let p2 = p2 as u32;
        let r = lf_checker_rt::callee_cdecl!(QUERY_CALLEE, u32, p3, p2, obj, p1, 6, 1, 4);
        if r == 0 {
            return 0;
        }
        let rx = f32::from_bits(st[W_ANS]);
        let ry = f32::from_bits(st[W_ANS + 1]);
        let rz = add(f32::from_bits(st[W_ANS + 2]), DROP);
        st[W_RX] = rx.to_bits();
        st[W_RY] = ry.to_bits();
        st[W_RZ] = rz.to_bits();
        let p4 = st.as_mut_ptr().add(W_RX) as u32;
        let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(rd32(obj) + VTABLE_HOOK) as usize) };
        hook(obj, p4, 0, 0);
        0
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_005267d0 {
// original: 0x005267d0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race58NoHolds, player_schema::LeaderboardInfo, 10>::vf8

/// Reports the storage size class of one leaderboard column.
///
/// Asks the registry for id LEADERBOARD_ID's column list (array
/// at +20), classifies entry `index` through the kind callee and
/// maps kind-1 to a size class (4 or 8 bytes, else 0), returning
/// 0 when the lookup fails, the entry has no kind, or the kind is
/// out of range. stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_005267d0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x47;
        const LOOKUP: u32 = 1;
        const KIND_OF: u32 = 2;
        const ARRAY: usize = 5;
        const NO_KIND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return 0;
        }
        let arr = buf[ARRAY] as u32;
        let elem = (arr.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_OF, u32, elem);
        if kind == NO_KIND {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 0x4,
            1 => 0x8,
            2 => 0x8,
            3 => 0x0,
            4 => 0x4,
            _ => 0,
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0054c960 {
// original: 0x0054C960 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_22, player_schema::LeaderboardInfo, 10>::vf14
///
/// Poll one ranked leaderboard's rows and reduce them into three outputs.
///
/// `this` is the leaderboard-info object (its virtual table supplies the
/// handle slot at `+0x2c` and the row-index slot at `+0x30`). The six stack
/// arguments are: `base` (the running cursor's start), `payload_out` (8
/// bytes receiving one row's payload), `bits_out` (8 bytes receiving a
/// one-hot row mask), `flag_out` (one byte receiving the last row-test
/// outcome), `ctx` (an opaque context handed to the row callees) and
/// `span` (the cursor may advance at most this far past `base`).
///
/// Algorithm: clear `bits_out` and `flag_out`, fetch the query handle,
/// open query `LEADERBOARD_ID` (which yields the row-handle table), then
/// run 24 iterations. Iteration `i` maps to a row index through the index
/// slot; a skip test against `ctx` may pass it over. Otherwise the row's
/// kind decides the
/// cursor step (8 for kinds 1, 2, 3 and 5, else 0). When the handle equals
/// the iteration number the row object is fetched and, if its signed
/// size is at most 8 (a negative size copies), its payload copied to
/// `payload_out` and `flag_out` set.
/// Otherwise the cursor advances by the step: it must stay within
/// `base + span`, and a place call must accept the row, whose bit is then
/// written into `bits_out` (overwriting the previous iteration's bit).
/// The loop stops early when an iteration reports failure. Returns the
/// last outcome byte in `al` (upper bytes are the last callee's leftover).
///
/// Edge cases: a rejected query returns 0 with the outputs cleared; a null
/// row or an oversize one clears the flag; a wrapped `base + span` fails
/// the bound check. The kind switch reads a table of code addresses; the
/// five entries collapse to the two step values above.
///
/// Original: 0x0054C960 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_0054C960(this: u32, base: u32, payload_out: u32, bits_out: u32, flag_out: u32, ctx_arg: u32, span: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x0CD;
        const ITERATIONS: u32 = 24;
        const STEP_WIDE: u32 = 8;
        const VT_HANDLE: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const Q_ROWS_WORD: usize = 2;
        const ROW_PAYLOAD_OFF: u32 = 4;
        const PAYLOAD_MAX: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }

        wr32(bits_out, 0);
        wr32(bits_out.wrapping_add(4), 0);
        (flag_out as *mut u8).write(0);
        let limit = span.wrapping_add(base);
        let vtable = rd32(this);
        let handle_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_HANDLE)) as usize);
        let handle = handle_of(this);
        // Query block shared with the open callee: it stores the row table
        // at word 2, the only word either side reads afterwards.
        let mut query = [0u32, 0u32, 0u32];
        let opened: u32 = lf_checker_rt::callee_fastcall!(
            3, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if (opened & 0xff) == 0 {
            return 0;
        }
        let rows = query[Q_ROWS_WORD];
        let index_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_INDEX)) as usize);
        let mut ok: u8 = 1;
        let mut cursor = base;
        let mut i = 0u32;
        while i < ITERATIONS {
            if ok == 0 {
                break;
            }
            let idx = index_of(this, i);
            let skipped: u32 =
                lf_checker_rt::callee_thiscall!(4, u32, ctx_arg, idx);
            if (skipped & 0xff) == 0 {
                ok = 0;
                let row_handle = rd32(rows.wrapping_add(idx.wrapping_mul(4)));
                let kind: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, row_handle);
                let step = if matches!(kind, 1 | 2 | 3 | 5) { STEP_WIDE } else { 0 };
                if handle == i {
                    let row: u32 =
                        lf_checker_rt::callee_thiscall!(6, u32, ctx_arg, idx);
                    ok = 0;
                    if row != 0 {
                        let size: u32 =
                            lf_checker_rt::callee_thiscall!(7, u32, row);
                        // Signed compare (jg): a negative size copies.
                        if (size as i32) <= PAYLOAD_MAX as i32 {
                            wr64(payload_out, rd64(row.wrapping_add(ROW_PAYLOAD_OFF)));
                            ok = 1;
                        }
                    }
                    (flag_out as *mut u8).write(ok);
                } else {
                    // The place call sees the cursor from before this
                    // iteration's step (the original passes its spill slot,
                    // which the loop end refreshes only afterwards).
                    let prev = cursor;
                    cursor = cursor.wrapping_add(step);
                    if cursor > limit {
                        ok = 0;
                    } else {
                        let placed: u32 = lf_checker_rt::callee_thiscall!(
                            8, u32, ctx_arg, idx, prev, step);
                        if (placed & 0xff) == 0 {
                            ok = 0;
                        } else {
                            ok = 1;
                            let bit = 1u32 << (i & 31);
                            let (lo, hi) = if i < 0x20 {
                                (bit, 0)
                            } else if i < 0x40 {
                                (0, bit)
                            } else {
                                (0, 0)
                            };
                            wr32(bits_out, lo);
                            wr32(bits_out.wrapping_add(4), hi);
                        }
                    }
                }
            }
            i += 1;
        }
        ok as u32
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_005909c0 {
// original: 0x005909C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_244, player_schema::LeaderboardInfo, 10>::vf13

/// Look up one leaderboard key in the fetched table and return its value.
///
/// Asks the row-table helper (fastcall callee 1: ECX = `LEADERBOARD_ID`,
/// EDX = out-pointer) for this leaderboard's table. The helper answers
/// nonzero on success and fills three words at the out-pointer: the row
/// `count` at `+0x0c`, the `keys` array pointer at `+0x10` and the parallel
/// `vals` array pointer at `+0x14`. The count is compared SIGNED: zero or
/// negative means not found without touching either array.
///
/// On success the keys are scanned in order for `key`; the first match
/// returns the value at the same index, and no match returns -1, as does
/// a failed fetch. `NOT_FOUND` is -1 as u32.
///
/// Original: stdcall, one stack word, callee pops 4. Incoming ECX is dead
/// (overwritten with the id before the call). The helper lives in the
/// encrypted first megabyte and never executes under the checker; its
/// call site is patched and its answers are scripted.
lf_checker_rt::export!(stdcall, rw_005909C0(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1cc;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const OUT_COUNT: usize = 3;
        const OUT_KEYS: usize = 4;
        const OUT_VALS: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut query = [0u32; 8];
        query[OUT_COUNT] = 0;
        query[OUT_KEYS] = 0;
        query[OUT_VALS] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = query[OUT_COUNT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = query[OUT_KEYS];
        let vals = query[OUT_VALS];
        let mut i = 0i32;
        while i < count {
            let at = (i as u32).wrapping_mul(4);
            if rd32(keys.wrapping_add(at)) == key {
                return rd32(vals.wrapping_add(at));
            }
            i += 1;
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00542910 {
// original: 0x00542910 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_3, player_schema::LeaderboardInfo, 10>::vf6
/// Position of a key inside the board's key column.
///
/// Calls the leaderboard-info callee (fastcall slot 0) with this board's
/// numeric id in ECX and a scratch info block in EDX. When the callee
/// reports failure, or the signed `count` (at +0x0c) is not positive, the
/// result is NOT_FOUND. Otherwise the key array (at +0x10) is scanned
/// linearly for `key` and the first matching position returned, or
/// NOT_FOUND when absent. The object pointer in ECX is unused. Original
/// is thiscall with one stack word.
lf_checker_rt::export!(thiscall, rw_00542910(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADER_ID: u32 = 0xA3;
        const INFO_COUNT: u32 = 0x0C;
        const INFO_KEYS: u32 = 0x10;
        const INFO_CALLEE: u32 = 0;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADER_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = info[(INFO_COUNT / 4) as usize] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = info[(INFO_KEYS / 4) as usize] as *const u32;
        let mut i = 0i32;
        loop {
            if keys.add(i as usize).read_unaligned() == key {
                return i as u32;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00bf4540 {
// original: 0x00bf4540 tune_entity_response (proposed name)
/// Tune an entity session's response curves from live measurements.
///
/// `this_` carries configuration (key at +8, floats at +0x28/+0x2c/+0x30),
/// `arg1` the entity and `arg2` a blend weight. A null entity or a failed
/// session lookup returns zero. A zero weight takes the direct path
/// (prepare, push, two weighted calls, row copy); a nonzero weight may
/// run a gated probe-and-forward block. Both join at a virtual fetch of
/// a 3-float measurement whose length, scaled and clamped to one, drives
/// two more weighted calls. A flag byte then decides between returning
/// the last answer and flushing the session. Returns the last answer, or
/// zero on the early exits.
export!(thiscall, rw_00bf4540(this_: *mut u8, arg1: *mut u8, arg2: f32) -> u32 {
    unsafe {
        if arg1.is_null() {
            return 0;
        }
        let a8 = *(this_.add(8) as *const u32);
        let mut flag = [0u32; 2];
        let a1p10 = (arg1 as u32).wrapping_add(10);
        let esi = callee_thiscall!(1, u32, relocated(0x01394D60), a1p10, a8,
            flag.as_mut_ptr() as u32, 0, 0);
        if esi == 0 {
            return 0;
        }
        if arg2 == 0.0 {
            let mut s20 = [0u32; 4];
            callee_thiscall!(2, u32, this_ as u32, s20.as_mut_ptr() as u32);
            callee_thiscall!(3, u32, esi, s20.as_mut_ptr() as u32);
            let f28 = *(this_.add(0x28) as *const f32);
            callee_thiscall!(4, u32, esi, relocated(0x00EBBE14), f28.to_bits());
            let f2c = *(this_.add(0x2c) as *const f32);
            callee_thiscall!(4, u32, esi, relocated(0x00EBBE20), f2c.to_bits());
            *((esi + 0x1a0) as *mut u32) = *(this_.add(0x30) as *const u32);
        } else if (flag[0] as u8) == 0 {
            let t = callee_thiscall!(5, u32, this_ as u32);
            if (t as u8) != 0 {
                let mut se = [0u32; 4];
                callee_stdcall!(6, u32, se.as_mut_ptr() as u32);
                // The original reads its middle argument from a scratch
                // word no call writes (uninitialized stack, i.e. the
                // checker's stack fill); it also clobbers its own saved
                // register slot with the weight, which no one re-reads.
                callee_thiscall!(7, u32, this_ as u32, esi, 0, arg2.to_bits());
            }
        }
        let vt = *(arg1 as *const u32);
        let slot_v = ((vt + 0xec) as *const u32).read();
        let vcall: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot_v as usize);
        let mut sj = [0u32; 4];
        let ans = vcall(arg1 as u32, sj.as_mut_ptr() as u32);
        let x = *(ans as *const f32);
        let y = *((ans + 4) as *const f32);
        let z = *((ans + 8) as *const f32);
        let mut d = x * x;
        d += y * y;
        d += z * z;
        let r0 = d.sqrt() * f32::from_bits(0x3D4CCCCD);
        let r = if 1.0f32 > r0 { r0 } else { 1.0f32 };
        callee_thiscall!(4, u32, esi, relocated(0x00EBBE28), r.to_bits());
        let m0 = *(arg1.add(0x1ed4) as *const f32);
        let m = if 1.0f32 > m0 { m0 } else { 1.0f32 };
        let ans4 = callee_thiscall!(4, u32, esi, relocated(0x00EBBE30), m.to_bits());
        if (flag[0] as u8) == 0 {
            return ans4;
        }
        callee_thiscall!(8, u32, esi)
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00d3c120 {
// original: 0x00d3c120 dummytask_helper_a (proposed)

/// Wander-task movement update: steers the task at `task` toward its target
/// using the mover state in `arg_a` and the target descriptor in `arg_b`.
/// The third stack argument is not read.
///
/// Layout: `task+0x10` is an optional position source (null selects the
/// task's own stored vector at `+0x20`), `task+0x38` a slot handle or zero.
/// `arg_a+0` is a small element count, `arg_a+0xc` a mode flag, `arg_a+0x18`
/// a timer, `arg_a+0x28` an array of 16-byte elements. `arg_b+0x20` points at
/// the target position vector (`+0x30`/`+0x34`/`+0x38`).
///
/// Behaviour: after notifying the base object, the function resolves or
/// releases the slot handle, asks the mover for its current vector, then
/// either ticks the timer down (mode set) or measures the planar or spatial
/// distance to the target (chosen by comparing a scaled counter against a
/// threshold) and reports it. A trailing loop adds one to the first lane of
/// each array element, two state words are reset, and a final mover call
/// runs whose result is returned. Two global words are read (a float seed
/// and a state word) and several read-only float constants.
///
/// Float order throughout is the original's; the two float comparisons that
/// treat unordered as taken/not-taken are written to match exactly.
///
/// Original: 0x00d3c120 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d3c120(task: u32, arg_a: u32, arg_b: u32, _arg_c: u32) -> u32 {
    unsafe {
        const T_SRC: u32 = 0x10;
        const T_VEC: u32 = 0x20;
        const T_HANDLE: u32 = 0x38;
        const A_COUNT: u32 = 0x00;
        const A_STATE: u32 = 0x04;
        const A_WORD: u32 = 0x08;
        const A_MODE: u32 = 0x0c;
        const A_VEC: u32 = 0x10;
        const A_TIMER: u32 = 0x18;
        const A_ARR: u32 = 0x28;
        const B_OBJ: u32 = 0x20;
        const G_SEED: u32 = 0x01054a30;
        const G_WORD: u32 = 0x011735b4;
        const K_TICK: u32 = 0x00fe878c;
        const K_SCALE: u32 = 0x00fe8684;
        const K_THRESH: u32 = 0x00fe879c;
        const K_ONE: u32 = 0x00fe88e8;
        const K_ALT: u32 = 0x00fe8b80;
        const FLAT_ARG: u32 = 0x42700000;

        const C_BASE: u32 = 1;
        const C_PROBE: u32 = 2;
        const C_ACQUIRE: u32 = 3;
        const C_ALIVE: u32 = 4;
        const C_RELEASE: u32 = 5;
        const C_MOVER: u32 = 6;
        const C_TICK: u32 = 7;
        const C_COUNTER: u32 = 8;
        const C_REPORT2: u32 = 9;
        const C_REPORT3: u32 = 10;
        const C_FINISH: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn rdc(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Resolve the current position vector: the source object's vector
        /// (through its indirect pointer when present, else its inline one),
        /// or the task's stored vector when there is no source. The vector is
        /// also stored back to the task. Pure 32-bit copies.
        #[inline(always)]
        unsafe fn load_vec(task: u32) -> [u32; 4] {
            unsafe {
                let src = rd32(task + T_SRC);
                if src == 0 {
                    [rd32(task + T_VEC), rd32(task + T_VEC + 4),
                     rd32(task + T_VEC + 8), rd32(task + T_VEC + 12)]
                } else {
                    let ind = rd32(src + 0x20);
                    let base = if ind == 0 { src + 0x10 } else { ind + 0x30 };
                    let v = [rd32(base), rd32(base + 4), rd32(base + 8), rd32(base + 12)];
                    wr32(task + T_VEC, v[0]);
                    wr32(task + T_VEC + 4, v[1]);
                    wr32(task + T_VEC + 8, v[2]);
                    wr32(task + T_VEC + 12, v[3]);
                    v
                }
            }
        }

        let obj = rd32(arg_b + B_OBJ);
        let seed = rd32(lf_checker_rt::global::<u32>(G_SEED) as u32);
        lf_checker_rt::callee_thiscall!(C_BASE, u32, task);
        let mover = arg_a;
        // Scratch word reused by the final call: the seed, still intact
        // (the measure block's writes land above it).
        if rd32(mover + A_MODE) == 0 {
            if rd32(task + T_HANDLE) == 0 {
                let r: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, arg_b);
                if (r as u8) != 0 {
                    let mut vec = load_vec(task);
                    let h: u32 = lf_checker_rt::callee_cdecl!(
                        C_ACQUIRE,
                        u32,
                        arg_b,
                        vec.as_mut_ptr() as u32
                    );
                    wr32(task + T_HANDLE, h);
                }
            } else {
                let h = rd32(task + T_HANDLE);
                if h != 0 {
                    let r: u32 = lf_checker_rt::callee_cdecl!(C_ALIVE, u32, h);
                    if (r as u8) != 0 {
                        lf_checker_rt::callee_cdecl!(C_RELEASE, u32, h, arg_b);
                        wr32(task + T_HANDLE, 0);
                    }
                }
            }
        }
        let mut slot = 0u32;
        let r: u32 = lf_checker_rt::callee_thiscall!(
            C_MOVER,
            u32,
            mover,
            &mut slot as *mut u32 as u32,
            seed
        );
        if (r as u8) != 0 {
            if rd32(mover + A_MODE) != 0 {
                lf_checker_rt::callee_thiscall!(C_TICK, u32, mover);
                wrf(mover + A_TIMER, fsub(rdf(mover + A_TIMER), rdc(K_TICK)));
            } else {
                let tx = rdf(obj + 0x30);
                let ty = rdf(obj + 0x34);
                let tz = rdf(obj + 0x38);
                let vec = load_vec(task);
                let n: u32 = lf_checker_rt::callee_cdecl!(C_COUNTER, u32,);
                let prod = fmul((n as i32) as f32, rdc(K_SCALE));
                if !(rdc(K_THRESH) > prod) {
                    let dx = fsub(tx, f32::from_bits(vec[0]));
                    let dy = fsub(ty, f32::from_bits(vec[1]));
                    let dz = fsub(tz, f32::from_bits(vec[2]));
                    let len2 = fadd(
                        fadd(fmul(dx, dx), fmul(dy, dy)),
                        fmul(dz, dz),
                    );
                    let v = fadd(len2.sqrt(), rdc(K_ALT));
                    let (mut sa, mut sb) = (0u32, 0u32);
                    lf_checker_rt::callee_cdecl!(
                        C_REPORT3,
                        u32,
                        &mut sb as *mut u32 as u32,
                        &mut sa as *mut u32 as u32,
                        v.to_bits(),
                        0x10,
                        mover,
                        mover + A_VEC
                    );
                } else {
                    let dx = fsub(tx, f32::from_bits(vec[0]));
                    let dy = fsub(ty, f32::from_bits(vec[1]));
                    let len2 = fadd(fmul(dx, dx), fmul(dy, dy));
                    // The original tests the squared length against zero
                    // through an unordered-aware compare: only an exact zero
                    // takes the flat path, NaN goes through the root.
                    let s = if len2 == 0.0 {
                        0.0f32
                    } else {
                        fdiv(rdc(K_ONE), len2.sqrt())
                    };
                    let _dirx = fmul(dx, s);
                    let _diry = fmul(s, dy);
                    let _dirz = fmul(s, 0.0);
                    let (mut sa, mut sb) = (0u32, 0u32);
                    lf_checker_rt::callee_cdecl!(
                        C_REPORT2,
                        u32,
                        &mut sb as *mut u32 as u32,
                        &mut sa as *mut u32 as u32,
                        FLAT_ARG,
                        0x10,
                        mover,
                        mover + A_VEC
                    );
                }
                let count = rd32(mover + A_COUNT);
                if count > 1 {
                    let base = mover + A_ARR;
                    let one = rdc(K_ONE);
                    let mut i = 1u32;
                    while i < count {
                        let p = base.wrapping_add((i - 1).wrapping_mul(0x10));
                        wrf(p, fadd(rdf(p), one));
                        i = i.wrapping_add(1);
                    }
                }
            }
            wr32(mover + A_STATE, 0);
            wr32(mover + A_WORD, rd32(lf_checker_rt::global::<u32>(G_WORD) as u32));
        }
        let mut slot2 = 0u32;
        lf_checker_rt::callee_thiscall!(
            C_FINISH,
            u32,
            mover,
            &mut slot2 as *mut u32 as u32,
            seed
        )
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0056a2f0 {
// original: 0x56a2f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_103, player_schema::LeaderboardInfo, 10>::vf6
/// Index lookup by id for one ranked leaderboard. (`vf6 Race103`).
///
/// Fetches the tables and scans the id array for `id`, returning the match
/// position. Returns -1 when the fetch fails, the count is not positive
/// (signed), or the id is absent.
/// Leaderboard id: 0x132.
export!(stdcall, rw_0056a2f0(id: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x132;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) as u8 == 0 {
            return 0xFFFF_FFFF;
        }
        let count = frame[3] as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let ids = frame[4] as *const u32;
        let mut i = 0i32;
        while i < count {
            if ids.add(i as usize).read() == id {
                return i as u32;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00d94a80 {
// original: 0x00d94a80 traverse_cells (proposed)

/// Look up the table for a query point, probe it, and resolve the hit cell.
///
/// `p0` points to the query point (four floats, copied to `*p3` when `p3`
/// is non-null); `p1`/`p2` are out-slots for the table and the hit entry;
/// `p4` is a float threshold forwarded to the probe; the low byte of `p5`
/// selects the cached-table fast path. Returns negative sentinels on the
/// way out: -4 when the offset lookup misses, -5 on a null table, -7 when
/// the probe reports no-hit, 0 with the hit entry stored on success.
///
/// Behaviour: with the flag set and a cached offset that is not 0xfff the
/// table lookup uses it directly, otherwise the offset callee (id 0) is
/// asked and 0xfff means -4. The table callee (id 1) resolves the offset;
/// null means -5. The probe (id 2) is asked with the table and the
/// threshold; 0xffff means no-hit: the advance callee (id 4, non-null `p3`)
/// or the simple callee (id 5) runs and -7 is returned. Otherwise the hit
/// entry (table base + answer*40) is stored to `*p2`, the use callee
/// (id 6) runs, and a clear marker bit means success (0). A set bit enters
/// the loop head: the fallback table is planted to `*p1` and the probe is
/// asked again (id 3): 0xffff repeats the advance/simple pair and returns
/// -7, any other answer re-resolves through the tail and a clear bit
/// returns 0. (A set tail bit with an unchanged answer would revisit the
/// head forever; the contract pins it clear.)
///
/// The probe/advance/simple frames carry the masked query words; the
/// checker stubs model answer-only callees, so frame addresses are skipped
/// and their contents unobserved (see `narrowed`). The stale frame word
/// copied to `p3+8` reads the defined stack fill (0 in the contract).
///
/// Original: 0x00d94a80 (thiscall, six stack words, full-eax signed result).
lf_checker_rt::export!(thiscall, rw_00d94a80(this: u32, p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> u32 {
    unsafe {
        const ID_OFS: u32 = 0;
        const ID_TAB: u32 = 1;
        const ID_RAY: u32 = 2;
        const ID_RAY2: u32 = 3;
        const ID_ADV: u32 = 4;
        const ID_SIMPLE: u32 = 5;
        const ID_USE: u32 = 6;
        const FALLBACK: u32 = 0x16b8f8c;
        const NOHIT: u32 = 0xffff;
        const TEN: u32 = 0x41200000;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if p3 != 0 {
            wr32(p3, rd32(p0));
            wr32(p3 + 4, rd32(p0 + 4));
            wr32(p3 + 8, rd32(p0 + 8));
            wr32(p3 + 12, rd32(p0 + 12));
        }
        let eax: u32;
        if (p5 & 0xff) != 0 {
            let cached = rd32(rd32(p1) + 0x80);
            if cached != 0xfff {
                eax = cached;
            } else {
                let a0: u32 = lf_checker_rt::callee_cdecl!(ID_OFS, u32, p0);
                if a0 == 0xfff {
                    return 0xfffffffc;
                }
                eax = a0;
            }
        } else {
            let a0: u32 = lf_checker_rt::callee_cdecl!(ID_OFS, u32, p0);
            if a0 == 0xfff {
                return 0xfffffffc;
            }
            eax = a0;
        }
        let table: u32 = lf_checker_rt::callee_cdecl!(ID_TAB, u32, eax);
        wr32(p1, table);
        if table == 0 {
            return 0xfffffffb;
        }
        // Masked query words (low 12 bits of w10 forced, feeds the -7 checks).
        let w10 = rd32(p0) | 0xffff0fff;
        let _w14 = (rd32(p0 + 4) | 0x0fffffff) & 0xefffffff;
        let dummy = [0u32; 8];
        let ans1: u32 = lf_checker_rt::callee_thiscall!(ID_RAY, u32, table, dummy.as_ptr() as u32, dummy.as_ptr() as u32, p4);
        if ans1 == NOHIT {
            let pick = if ((rd32(table + 0x50) >> 2) & 1) != 0 {
                rd32(table + 0x80)
            } else {
                0xfff
            };
            if p3 != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(ID_ADV, u32, this, dummy.as_ptr() as u32, TEN, dummy.as_ptr() as u32, 0, p3, pick);
            } else {
                let _: u32 = lf_checker_rt::callee_thiscall!(ID_SIMPLE, u32, this, dummy.as_ptr() as u32, TEN, dummy.as_ptr() as u32, 0, pick);
            }
            if (w10 & 0xfff) == 0xfff {
                return 0xfffffff9;
            }
            // Unreachable in practice (low 12 bits forced above); kept so a
            // wrong mask analysis fails loudly instead of silently.
            if ((w10 >> 16) & 0xffff) == 0xffff {
                return 0xfffffff9;
            }
            return 0xfffffff9;
        }
        if p3 != 0 {
            wr32(p3 + 8, 0);
        }
        let out9 = rd32(table + 0x6c).wrapping_add(ans1.wrapping_mul(40));
        wr32(p2, out9);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_USE, u32, this, out9);
        if ((rd32(out9) >> 0x12) & 1) == 0 {
            return 0;
        }
        loop {
            let planted = rd32(lf_checker_rt::relocated(FALLBACK));
            wr32(p1, planted);
            let ans2: u32 = lf_checker_rt::callee_thiscall!(ID_RAY2, u32, planted, dummy.as_ptr() as u32, dummy.as_ptr() as u32, p4);
            if ans2 != NOHIT {
                if p3 != 0 {
                    wr32(p3 + 8, 0);
                }
                let outt = rd32(planted + 0x6c).wrapping_add(ans2.wrapping_mul(40));
                wr32(p2, outt);
                if ((rd32(outt) >> 0x12) & 1) == 0 {
                    return 0;
                }
            } else {
                let pick = if ((rd32(planted + 0x50) >> 2) & 1) != 0 {
                    rd32(planted + 0x80)
                } else {
                    0xfff
                };
                if p3 != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(ID_ADV, u32, this, dummy.as_ptr() as u32, TEN, dummy.as_ptr() as u32, 1, p3, pick);
                } else {
                    let _: u32 = lf_checker_rt::callee_thiscall!(ID_SIMPLE, u32, this, dummy.as_ptr() as u32, TEN, dummy.as_ptr() as u32, 1, pick);
                }
                let w10b = w10 | 0xffff0fff;
                if (w10b & 0xfff) == 0xfff {
                    return 0xfffffff9;
                }
                if ((w10b >> 16) & 0xffff) == 0xffff {
                    return 0xfffffff9;
                }
                return 0xfffffff9;
            }
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00546640 {
// original: 0x00546640 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_17, player_schema::LeaderboardInfo, 10>::vf6

/// Index of a row id in this leaderboard's row array, or -1 when absent.
///
/// Fetches the board's row list through the info callee (id `0xd4`, out-words
/// at the frame pointer: count, array), then scans the array with a signed
/// comparison for `wanted` and returns the first matching index. Returns -1
/// when the callee reports failure (low byte of its answer is zero) or the
/// count is not positive. stdcall, one stack argument; entry ECX ignored.
lf_checker_rt::export!(stdcall, rw_00546640(wanted: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xd4;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 7];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32,
            LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        let count = info[3];
        let items = info[4];
        if (count as i32) <= 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let v = ((items.wrapping_add(i.wrapping_mul(4))) as *const u32)
                .read_unaligned();
            if v == wanted {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0058b4d0 {
// original: 0x0058b4d0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_224, player_schema::LeaderboardInfo, 10>::vf8

/// Column-width class for one ranked episodic-race leaderboard.
///
/// Asks the leaderboard helper (callee 1) for the column list of leaderboard
/// id 0x1b8, classifies element `index` of the returned array through the
/// column classifier (callee 2, argument in ECX), and maps the class to a
/// width: class 1 -> 4, classes 2-3 -> 8, class 5 -> 4, anything else
/// (including classifier failure, reported as -1) -> 0. Helper failure also
/// yields 0. The index is not bounds-checked.
///
/// The helper takes the leaderboard id in ECX and an out-buffer in EDX and
/// answers in AL; on success it fills the array pointer at buffer byte 20.
/// This method ignores its `this` pointer and takes one stack argument
/// (`index`). Original is stdcall (the callee pops 4 bytes); the class map is the original's
/// five-entry jump table, whose entries pair up as (4, 8, 8, 0, 4).

lf_checker_rt::export!(stdcall, rw_0058b4d0(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const LEADERBOARD_ID: u32 = 0x1b8;
        const HELPER: u32 = 1;
        const CLASSIFY: u32 = 2;
        const ARRAY_WORD: usize = 5;
        const CLASS_FAILED: u32 = 0xffff_ffff;

        let mut info = [0u32; 6];
        info[ARRAY_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0;
        }
        let cell = info[ARRAY_WORD].wrapping_add(index.wrapping_mul(4));
        let class: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, rd32(cell));
        if class == CLASS_FAILED {
            return 0;
        }
        match class.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_005b5510 {
// original: 0x005B5510 menu_present_apply (proposed)

/// Drive one front-end present/apply pass from two mode flags and scripted getters.
///
/// Reads two mode bytes (`MODE_FLAG_A`, `MODE_FLAG_B`) and selects a mode word:
/// 2 when the first flag equals `MODE_VALUE_DIRECT`, else 7 when the second flag
/// is clear, else 2. It then runs a fixed getter sequence: a one-word probe
/// (callee 0), two indexed reads (callee 1, indexes 8 and 11) of which the
/// second returns a pointer the function dereferences for one float word, a
/// four-word setup call (callee 2) taking (0, mode, two frame pointers), and
/// three colour reads (callee 3, indexes 0x42, 0x3e, 0x3b), each returning a
/// pointer the function dereferences for one word.
///
/// The gathered words feed a seven-argument apply call (callee 4) together with
/// two globals (a dword and a float, passed by value) and the address of a
/// third global. After a two-word ping (callee 5) a status word is fetched
/// (callee 6) and compared, SIGNED, against -0x5c for equality and then with a
/// signed greater-or-equal against a 16-bit bound loaded from `BOUND_TABLE +
/// index * 24` (the index is a global; the bound word is zero-extended, so it
/// is always non-negative as a signed value). Either match selects the fallback
/// selector 0, otherwise the status word itself is the selector. The tail is
/// identical on both paths: a two-word select (callee 7) of (0, selector), a
/// two-word commit (callee 9) of its answer, a three-word select (callee 8) of
/// (0, selector, 1), and a two-word final commit (callee 10) of its answer.
///
/// The float word from the second indexed read is stored to a frame slot that
/// ends up as the fifth stack word of the setup call; the contract declares the
/// setup callee with five arguments so that word is compared (the real callee
/// takes four; the fifth word is a spill the original happens to place there).
/// All frame-pointer arguments are skipped in the comparison and their contents
/// are unobserved: the stubs write nothing through them and the caller never
/// reads them back. Takes no arguments, returns nothing (cdecl, 0 args).
lf_checker_rt::export!(cdecl, rw_005B5510() -> u32 {
    unsafe {
        const MODE_FLAG_A: u32 = 0x116C250;
        const MODE_FLAG_B: u32 = 0x116C253;
        const MODE_VALUE_DIRECT: u8 = 0x6A;
        const MODE_DIRECT: u32 = 2;
        const MODE_ALT: u32 = 7;
        const G_FLOAT: u32 = 0x1161864;
        const G_DWORD_A: u32 = 0x1161850;
        const G_ADDR_IMM: u32 = 0x116185C;
        const G_INDEX: u32 = 0x1160C0C;
        const BOUND_TABLE: u32 = 0x19D33A4;
        const BOUND_STRIDE: u32 = 24;
        const STATUS_FALLBACK: u32 = 0xFFFF_FFA4; // -0x5c, compared SIGNED
        const C_PROBE: u32 = 0;
        const C_INDEXED: u32 = 1;
        const C_SETUP: u32 = 2;
        const C_COLOUR: u32 = 3;
        const C_APPLY: u32 = 4;
        const C_PING: u32 = 5;
        const C_STATUS: u32 = 6;
        const C_SELECT2: u32 = 7;
        const C_SELECT3: u32 = 8;
        const C_COMMIT: u32 = 9;
        const C_FINAL: u32 = 10;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }

        let flag_a = rd8(lf_checker_rt::relocated(MODE_FLAG_A));
        let flag_b = rd8(lf_checker_rt::relocated(MODE_FLAG_B));
        let mode = if flag_a == MODE_VALUE_DIRECT {
            MODE_DIRECT
        } else if flag_b == 0 {
            MODE_ALT
        } else {
            MODE_DIRECT
        };

        // Frame buffers. Their addresses are skipped in the comparison; the
        // stubs write nothing through them and nothing reads them back.
        let mut buf_probe = [0u32; 2];
        let mut buf_idx = [0u32; 2];
        let mut buf_setup = [0u32; 2];
        let mut buf_colour = [0u32; 2];
        let p_probe = (&mut buf_probe as *mut u32) as u32;
        let p_idx = (&mut buf_idx as *mut u32) as u32;
        let p_setup = (&mut buf_setup as *mut u32) as u32;
        let p_colour = (&mut buf_colour as *mut u32) as u32;

        lf_checker_rt::callee_cdecl!(C_PROBE, u32, p_probe);
        lf_checker_rt::callee_cdecl!(C_INDEXED, u32, p_idx, 8);
        let float_ptr = lf_checker_rt::callee_cdecl!(C_INDEXED, u32, p_idx, 0x0B);
        let float_bits = rd32(float_ptr);
        lf_checker_rt::callee_cdecl!(C_SETUP, u32, 0, mode, p_setup, p_idx, float_bits);

        let c0 = lf_checker_rt::callee_cdecl!(C_COLOUR, u32, p_idx, 0x42);
        let v0 = rd32(c0);
        let c1 = lf_checker_rt::callee_cdecl!(C_COLOUR, u32, p_setup, 0x3E);
        let v1 = rd32(c1);
        let c2 = lf_checker_rt::callee_cdecl!(C_COLOUR, u32, p_colour, 0x3B);
        let v2 = rd32(c2);

        let g_float = rd32(lf_checker_rt::relocated(G_FLOAT));
        let g_dword = rd32(lf_checker_rt::relocated(G_DWORD_A));
        let index = rd32(lf_checker_rt::relocated(G_INDEX));
        lf_checker_rt::callee_cdecl!(
            C_APPLY, u32, index, g_dword,
            lf_checker_rt::relocated(G_ADDR_IMM),
            g_float, v2, v1, v0
        );

        lf_checker_rt::callee_cdecl!(C_PING, u32, 0, 1);
        let status = lf_checker_rt::callee_cdecl!(C_STATUS, u32, index);
        let bound = rd16(
            lf_checker_rt::relocated(BOUND_TABLE).wrapping_add(index.wrapping_mul(BOUND_STRIDE)),
        );
        // Both comparisons are SIGNED (je after cmp against -0x5c; jge after
        // cmp against the zero-extended bound).
        let fallback =
            status == STATUS_FALLBACK || (status as i32) >= (bound as i32);
        let sel = if fallback { 0 } else { status };

        let t = lf_checker_rt::callee_cdecl!(C_SELECT2, u32, 0, sel);
        lf_checker_rt::callee_cdecl!(C_COMMIT, u32, 0, t);
        let u = lf_checker_rt::callee_cdecl!(C_SELECT3, u32, 0, sel, 1);
        lf_checker_rt::callee_cdecl!(C_FINAL, u32, 0, u);
    }
    0
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_005480e0 {
// original: 0x005480E0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_5, player_schema::LeaderboardInfo, 10>::vf7
/// Row lookup for one ranked leaderboard board (virtual slot 7).
///
/// Fetches the board tables for board id 0xB7, then returns the row word at
/// `index`: `rows[index]`, or -1 when the fetch step reports failure (low
/// byte of its answer is zero).
///
/// Frame layout (offsets from the out-struct pointer in EDX): the rows-table
/// pointer lands at `+0x10`.
/// The shared fetch step fills a six-word frame passed by pointer in EDX
/// with the board id in ECX; only the low byte of its answer is tested.
///
/// Edge cases: a failing fetch returns -1 without reading any table; the
/// index is scaled by 4 with wraparound and read with one 32-bit load, so a
/// wild index faults exactly like the original's load.
/// Calling convention: stdcall with one stack argument; the incoming ECX
/// (`this`) is ignored by the original (its first use is the board-id store)
/// and is likewise ignored here.
lf_checker_rt::export!(stdcall, rw_005480e0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xB7;
        const FETCH: u32 = 1;
        const ROWS_SLOT: usize = 4; // frame offset 0x10
        let mut frame = [0u32; 6];
        let answer =
            lf_checker_rt::callee_fastcall!(FETCH, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = frame[ROWS_SLOT];
        (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read()
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00beec60 {
// original: 0x00beec60 ped_mover_blend_b (proposed)

/// Blend a ped mover's base offset toward a source offset by factor `t`,
/// after aligning and mixing its orientation through three helpers.
///
/// `this` is the mover: word `+0x14` seeds the orientation callee, which
/// fills a four-float quaternion buffer. That quaternion is dotted with the
/// four direction floats at `dir` (accumulated as
/// `(dir1*q1+dir0*q0)+dir2*q2+dir3*q3`); a strictly negative dot flips the
/// sign of all four words so the quaternion faces the direction. The mixer
/// callee then combines the seed, the aligned quaternion and the direction
/// into a four-float accumulator, which the consumer callee reads. `out`
/// receives the base words from `this+0x18/+0x1c/+0x20` at `+0x30/+0x34/+0x38`,
/// each blended with the matching source float as `base*(1-t)+src*t` (first
/// and third terms keep the original's `(1-t)*base` operand order), and the
/// accumulator's last word at `+0x3c`.
///
/// A NaN dot product is unordered and skips the sign flip, matching the
/// original's `comiss`+`jbe`.
///
/// Original: thiscall, four stack words (`out`, `dir`, `src`, `t`), no
/// meaningful return value. Twin of 0x00beeac0 with every `this` offset four
/// higher.
lf_checker_rt::export!(thiscall, rw_00beec60(this: u32, out: u32, dir: u32, src: u32, t: u32) -> u32 {
    unsafe {
        const QUAT_SEED: u32 = 0x14;
        const BASE_X: u32 = 0x18;
        const BASE_Y: u32 = 0x1c;
        const BASE_Z: u32 = 0x20;
        const OUT_X: u32 = 0x30;
        const OUT_Y: u32 = 0x34;
        const OUT_Z: u32 = 0x38;
        const OUT_W: u32 = 0x3c;
        const SIGN_BIT: u32 = 0x8000_0000;
        const ONE: f32 = 1.0;
        const CALLEE_QUAT: u32 = 1;
        const CALLEE_MIX: u32 = 2;
        const CALLEE_USE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let mut q = [0u32; 4];
        lf_checker_rt::callee_cdecl!(CALLEE_QUAT, u32, q.as_mut_ptr() as u32, rd32(this + QUAT_SEED));
        let tt = f32::from_bits(t);
        let qf = [
            f32::from_bits(q[0]),
            f32::from_bits(q[1]),
            f32::from_bits(q[2]),
            f32::from_bits(q[3]),
        ];
        let mut dot = add(mul(rdf(dir + 4), qf[1]), mul(rdf(dir), qf[0]));
        dot = add(dot, mul(rdf(dir + 8), qf[2]));
        dot = add(dot, mul(rdf(dir + 12), qf[3]));
        if 0.0 > dot {
            for w in q.iter_mut() {
                *w ^= SIGN_BIT;
            }
        }
        let mut acc = [0u32; 4];
        lf_checker_rt::callee_thiscall!(CALLEE_MIX, u32, acc.as_mut_ptr() as u32, t, q.as_mut_ptr() as u32, dir);
        lf_checker_rt::callee_thiscall!(CALLEE_USE, u32, out, acc.as_mut_ptr() as u32);

        wr32(out + OUT_X, rd32(this + BASE_X));
        wr32(out + OUT_Y, rd32(this + BASE_Y));
        wr32(out + OUT_Z, rd32(this + BASE_Z));
        let k = sub(ONE, tt);
        let ax = mul(rdf(src), tt);
        let az = mul(rdf(src + 8), tt);
        let ay = mul(rdf(src + 4), tt);
        let bx = mul(rdf(out + OUT_X), k);
        let by = mul(k, rdf(out + OUT_Y));
        let bz = mul(k, rdf(out + OUT_Z));
        wrf(out + OUT_X, add(bx, ax));
        wrf(out + OUT_Y, add(by, ay));
        wrf(out + OUT_Z, add(bz, az));
        wr32(out + OUT_W, acc[3]);
        0
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0051a6f0 {
// original: 0x0051a6f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race14NoHolds, player_schema::LeaderboardInfo, 10>::vf6
/// Index lookup by id for one ranked leaderboard (`vf6 Race14`).
///
/// Fetches the tables and scans the id array for `id`, returning the match
/// position. Returns -1 when the fetch fails, when the count is not positive
/// (signed), or when the id is absent.
/// Leaderboard id: 0x6B.
export!(stdcall, rw_0051a6f0(id: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x6B;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if (fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let count = frame.as_ptr().add(3).read() as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let ids = frame.as_ptr().add(4).read() as *const u32;
        let mut i = 0i32;
        while i < count {
            if ids.add(i as usize).read() == id {
                return i as u32;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00a7f270 {
// original: 0x00a7f270 CSimpleSidewaysDiveTaskInfo::vf6
/// Serialize the sideways-dive nonzero test; out-param write untestable (byte flag in scratch).
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f270(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut flag: u32 = a1 & 0xFF;
        let fp = &mut flag as *mut u32 as u32;
        let d = *(this_.wrapping_add(0x18) as *const u32);
        let a2: u32 = callee_thiscall!(2, u32, buf, (d != 0) as u32, fp);
        (a2 & 0xFFFFFF00) | (flag & 0xFF)
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00c08150 {
// original: 0x00c08150 stream_header_check (proposed)

/// Open the streaming header and check its two magic words.
///
/// The opener (callee 1) always runs first with the image pointer `M1` and
/// zero; the finder (callee 2) receives the caller's `key` and the image
/// pointer `M2`. A null find skips the reads. Otherwise the reader (callees 3
/// and 4, one id per site so each answer can be scripted on its own) fills
/// two scratch words through out-parameters, and the found flag is set when
/// they equal `MAGIC1` and `MAGIC2`; the releaser (callee 5) then runs with
/// the find. The closer (callee 6) always runs last with the image pointer
/// `M3`. Returns the closer's answer with its low byte replaced by the flag.
///
/// Original: 0x00c08150 (cdecl, key first, one ignored word; all callees cdecl).
lf_checker_rt::export!(cdecl, rw_00c08150(key: u32, _ignored: u32) -> u32 {
    unsafe {
        const M1: u32 = 0xebde8c;
        const M2: u32 = 0xebde94;
        const M3: u32 = 0xebdc8f;
        const MAGIC1: u32 = 0x10291205;
        const MAGIC2: u32 = 0x52334850;
        const OPEN: u32 = 1;
        const FIND: u32 = 2;
        const READ0: u32 = 3;
        const READ1: u32 = 4;
        const RELEASE: u32 = 5;
        const CLOSE: u32 = 6;
        let _: u32 = lf_checker_rt::callee_cdecl!(OPEN, u32, lf_checker_rt::relocated(M1), 0);
        let h: u32 = lf_checker_rt::callee_cdecl!(FIND, u32, key, lf_checker_rt::relocated(M2));
        let mut flag = 0u32;
        if h != 0 {
            let mut w0 = 0u32;
            let mut w1 = 0u32;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                READ0, u32, h, (&mut w0 as *mut u32) as u32, 4);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                READ1, u32, h, (&mut w1 as *mut u32) as u32, 4);
            if w0 == MAGIC1 && w1 == MAGIC2 {
                flag = 1;
            }
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, h);
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(CLOSE, u32, lf_checker_rt::relocated(M3));
        (r & 0xffff_ff00) | flag
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0054e780 {
// original: 0x0054e780 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_2, player_schema::LeaderboardInfo, 10>::vf13
// Rank value of a leaderboard entry by key.
//
// Fetches the key table and a parallel value table, finds `target` in the
// keys, and returns the value at the same position, or -1 when the table is
// unavailable, empty, or holds no such key.
//
export!(stdcall, rs252_0054e780(target: u32) -> i32 {
    let mut out = [0u32; 8];
    let ok: u8 = callee_fastcall!(1, u8, 0xb1, out.as_mut_ptr() as u32);
    if ok == 0 {
        return -1;
    }
    let count = out[3] as i32;
    if count <= 0 {
        return -1;
    }
    let keys = out[4];
    let mut i = 0u32;
    loop {
        let v = unsafe { (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read() };
        if v == target {
            break;
        }
        i += 1;
        if (i as i32) >= count {
            return -1;
        }
    }
    // The original re-checks the found index against -1 here; that branch
    // is dead (the index is always >= 0), so only the live path remains.
    let values = out[5];
    unsafe { (values.wrapping_add(i.wrapping_mul(4)) as *const i32).read() }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00574100 {
// original: 0x00574100 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_139, player_schema::LeaderboardInfo, 10>::vf8

/// Ranked-race leaderboard row size class: fetch this board's row
/// table, classify the row at `index` through the shared classifier, and
/// map its key to a size (1 to 4, 2 and 3 to 8, 5 to 4, anything else
/// to 0, via the original's jump table).
///
/// `LEADERBOARD_ID` selects the board; `this` is ignored. Returns 0 when
/// the fetch fails or the classifier reports -1.
///
/// Original: 0x00574100 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00574100(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x163;
        const DESC_TABLE: usize = 5;
        const SIZE_CLASS: [u32; 5] = [4, 8, 8, 0, 4];
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut desc = [0u32; 6];
        desc[DESC_TABLE] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let table = desc[DESC_TABLE];
        let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let key: u32 = lf_checker_rt::callee_thiscall!(2, u32, entry);
        if key == 0xffff_ffff {
            return 0;
        }
        let slot = key.wrapping_sub(1);
        if slot > 4 {
            return 0;
        }
        SIZE_CLASS[slot as usize]
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00d874f0 {
// original: 0x00d874f0 ui_tracking_update (proposed)

/// Refresh a tracked UI element from its anchor, then either reset its
/// tracking state or hand it to the range updater.
///
/// `obj` is the tracker: `+0x20` points at the anchor holding the reference
/// point (`+0x30`/`+0x34`) and the heading source (`+0x10`/`+0x14`);
/// `+0xE4C`/`+0xE50` is the tracker's own point. `elem` is the element with
/// a heading at `+0x18`, a state word pair at `+0x20`/`+0x22` and flags at
/// `+0x2B`. The distance cutoff (30.0), the measure cutoff (10.0) and the
/// length cutoff (1.5) are read from the image's read-only constants.
///
/// Algorithm: dist = length(own - anchor). If dist < 30 and the heading is
/// negative, replace it with the heading callee's answer for the anchor
/// source. If dist < 10, call the measure hook (virtual slot `+0xEC`) and,
/// when its returned vector is shorter than 1.5, clear the range index
/// (`+0xE68`) and the element state. If both state words are zero and the
/// readiness callee answers positive, clear four words at `+0x1EB4..+0x1EC0`,
/// call the reset callee and set flag bit 2; otherwise tail-call the range
/// updater with (obj, elem, 1). Comparisons use the original's unordered
/// (NaN skips the block) semantics. No value is returned.
///
/// Original: 0x00D874F0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00d874f0(obj: u32, elem: u32) -> u32 {
    unsafe {
        const OBJ_VTABLE: u32 = 0x00;
        const OBJ_ANCHOR: u32 = 0x20;
        const OBJ_POS_X: u32 = 0x0e4c;
        const OBJ_POS_Y: u32 = 0x0e50;
        const OBJ_RANGE: u32 = 0x0e68;
        const OBJ_SLOT0: u32 = 0x1eb4;
        const OBJ_SLOT1: u32 = 0x1eb8;
        const OBJ_SLOT2: u32 = 0x1ebc;
        const OBJ_SLOT3: u32 = 0x1ec0;
        const ANCH_SRC_X: u32 = 0x10;
        const ANCH_SRC_Y: u32 = 0x14;
        const ANCH_REF_X: u32 = 0x30;
        const ANCH_REF_Y: u32 = 0x34;
        const ELEM_HEADING: u32 = 0x18;
        const ELEM_STATE: u32 = 0x20;
        const ELEM_STATE_HI: u32 = 0x22;
        const ELEM_FLAGS: u32 = 0x2b;
        const VT_MEASURE: u32 = 0xec;
        const FLAG_DONE: u8 = 0x04;
        const C_DIST_FAR: u32 = 0x00fe8b48;
        const C_DIST_NEAR: u32 = 0x00fe8b08;
        const C_LEN: u32 = 0x00fe8960;
        const CAL_HEADING: u32 = 1;
        const CAL_MEASURE: u32 = 2;
        const CAL_READY: u32 = 3;
        const CAL_RESET: u32 = 4;
        const CAL_RANGE_UPD: u32 = 5;

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
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn bb(x: f32) -> f32 {
            core::hint::black_box(x)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            bb(a) - bb(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            bb(a) * bb(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            bb(a) + bb(b)
        }
        #[inline(always)]
        fn vlen(x: f32, y: f32) -> f32 {
            bb(add(mul(x, x), mul(y, y))).sqrt()
        }

        let tail = |obj: u32, elem: u32| -> u32 {
            lf_checker_rt::callee_cdecl!(CAL_RANGE_UPD, u32, obj, elem, 1u32);
            0
        };

        let anchor = rd32(obj + OBJ_ANCHOR);
        let dx = sub(rdf(obj + OBJ_POS_X), rdf(anchor + ANCH_REF_X));
        let dy = sub(rdf(obj + OBJ_POS_Y), rdf(anchor + ANCH_REF_Y));
        let dist = vlen(dx, dy);
        if bb(rdf(lf_checker_rt::relocated(C_DIST_FAR))) > bb(dist) {
            let cur = rdf(elem + ELEM_HEADING);
            if bb(0.0) > bb(cur) {
                let h: f32 = lf_checker_rt::callee_cdecl!(
                    CAL_HEADING,
                    f32,
                    rd32(anchor + ANCH_SRC_X),
                    rd32(anchor + ANCH_SRC_Y)
                );
                wrf(elem + ELEM_HEADING, h);
            }
        }
        if bb(rdf(lf_checker_rt::relocated(C_DIST_NEAR))) > bb(dist) {
            let slot = rd32(rd32(obj + OBJ_VTABLE) + VT_MEASURE);
            let hook: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let mut out_slot = 0u32;
            let got = hook(obj, &mut out_slot as *mut u32 as u32);
            let len = vlen(rdf(got), rdf(got + 4));
            if bb(rdf(lf_checker_rt::relocated(C_LEN))) > bb(len) {
                wr32(obj + OBJ_RANGE, 0);
                wr32(elem + ELEM_STATE, 0);
            }
        }
        if rd16(elem + ELEM_STATE) != 0 {
            return tail(obj, elem);
        }
        if rd16(elem + ELEM_STATE_HI) != 0 {
            return tail(obj, elem);
        }
        let ready: u32 = lf_checker_rt::callee_thiscall!(CAL_READY, u32, obj);
        if (ready as i32) <= 0 {
            return tail(obj, elem);
        }
        wr32(obj + OBJ_SLOT3, 0);
        wr32(obj + OBJ_SLOT0, 0);
        wr32(obj + OBJ_SLOT1, 0);
        wr32(obj + OBJ_SLOT2, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, obj);
        ((elem + ELEM_FLAGS) as *mut u8).write(rd8(elem + ELEM_FLAGS) | FLAG_DONE);
        0
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0056e8e0 {
// original: 0x0056e8e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_119, player_schema::LeaderboardInfo, 10>::vf6

/// Find the position of a key in a leaderboard column.
///
/// `key` is the value sought. Calls the leaderboard fetch helper
/// (fastcall: ECX = leaderboard id 0x142, EDX = scratch record) and,
/// when it reports success, scans the column array the helper wrote at
/// record offset 0x10 in order and returns the first position
/// holding `key`. A non-positive row count (record offset 0xc)
/// yields 0xffffffff without reading the array; no match yields 0xffffffff.
///
/// Original: 0x0056e8e0 (rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_119, player_schema::LeaderboardInfo, 10>::vf6; stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0056e8e0(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x142;
        const COUNT_WORD: usize = 3;
        const COLS_WORD: usize = 4;
        const NOT_FOUND: u32 = 0xffffffff;
        let mut record = [0u32; 8];
        record[COUNT_WORD] = 0;
        record[COLS_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(
            1, u8, LEADERBOARD_ID, record.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = record[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let cols = record[COLS_WORD];
        let mut i: u32 = 0;
        while (i as i32) < count {
            let v = (cols.wrapping_add(i.wrapping_mul(4)) as *const u32).read();
            if v == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00597330 {
// original: 0x00597330 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_268, player_schema::LeaderboardInfo, 10>::vf14
//
// Poll one ranked leaderboard's rows and reduce them into three outputs.
//
// `this` is the leaderboard-info object (its virtual table supplies the
// handle slot at `+0x2c` and the row-index slot at `+0x30`). The six stack
// arguments are: `base` (the running cursor's start), `payload_out` (8
// bytes receiving one row's payload), `bits_out` (8 bytes receiving a
// one-hot row mask), `flag_out` (one byte receiving the last row-test
// outcome), `ctx` (an opaque context handed to the row callees) and
// `span` (the cursor may advance at most this far past `base`).
//
// Algorithm: clear `bits_out` and `flag_out`, fetch the query handle,
// open query `LEADERBOARD_ID` (which yields the row-handle table), then
// run 19 iterations. Iteration `i` maps to a row index through the index
// slot; a skip test against `ctx` may pass it over. Otherwise the row's
// kind decides the
// cursor step (8 for kinds 1, 2, 3 and 5, else 0). When the handle equals
// the iteration number the row object is fetched and, if its signed
// size is at most 8 (a negative size copies), its payload copied to
// `payload_out` and `flag_out` set.
// Otherwise the cursor advances by the step: it must stay within
// `base + span`, and a place call must accept the row, whose bit is then
// written into `bits_out` (overwriting the previous iteration's bit).
// The loop stops early when an iteration reports failure. Returns the
// last outcome byte in `al` (upper bytes are the last callee's leftover).
//
// Edge cases: a rejected query returns 0 with the outputs cleared; a null
// row or an oversize one clears the flag; a wrapped `base + span` fails
// the bound check. The kind switch reads a table of code addresses; the
// five entries collapse to the two step values above.
//
// Original: 0x00597330 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00597330(this: u32, base: u32, payload_out: u32, bits_out: u32, flag_out: u32, ctx_arg: u32, span: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1E0;
        const ITERATIONS: u32 = 19;
        const STEP_WIDE: u32 = 8;
        const VT_HANDLE: u32 = 0x2c;
        const VT_INDEX: u32 = 0x30;
        const Q_ROWS_WORD: usize = 2;
        const ROW_PAYLOAD_OFF: u32 = 4;
        const PAYLOAD_MAX: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }

        wr32(bits_out, 0);
        wr32(bits_out.wrapping_add(4), 0);
        (flag_out as *mut u8).write(0);
        let limit = span.wrapping_add(base);
        let vtable = rd32(this);
        let handle_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_HANDLE)) as usize);
        let handle = handle_of(this);
        // Query block shared with the open callee: it stores the row table
        // at word 2, the only word either side reads afterwards.
        let mut query = [0u32, 0u32, 0u32];
        let opened: u32 = lf_checker_rt::callee_fastcall!(
            3, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if (opened & 0xff) == 0 {
            return 0;
        }
        let rows = query[Q_ROWS_WORD];
        let index_of: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_INDEX)) as usize);
        let mut ok: u8 = 1;
        let mut cursor = base;
        let mut i = 0u32;
        while i < ITERATIONS {
            if ok == 0 {
                break;
            }
            let idx = index_of(this, i);
            let skipped: u32 =
                lf_checker_rt::callee_thiscall!(4, u32, ctx_arg, idx);
            if (skipped & 0xff) == 0 {
                ok = 0;
                let row_handle = rd32(rows.wrapping_add(idx.wrapping_mul(4)));
                let kind: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, row_handle);
                let step = if matches!(kind, 1 | 2 | 3 | 5) { STEP_WIDE } else { 0 };
                if handle == i {
                    let row: u32 =
                        lf_checker_rt::callee_thiscall!(6, u32, ctx_arg, idx);
                    ok = 0;
                    if row != 0 {
                        let size: u32 =
                            lf_checker_rt::callee_thiscall!(7, u32, row);
                        // Signed compare (jg): a negative size copies.
                        if (size as i32) <= PAYLOAD_MAX as i32 {
                            wr64(payload_out, rd64(row.wrapping_add(ROW_PAYLOAD_OFF)));
                            ok = 1;
                        }
                    }
                    (flag_out as *mut u8).write(ok);
                } else {
                    // The place call sees the cursor from before this
                    // iteration's step (the original passes its spill slot,
                    // which the loop end refreshes only afterwards).
                    let prev = cursor;
                    cursor = cursor.wrapping_add(step);
                    if cursor > limit {
                        ok = 0;
                    } else {
                        let placed: u32 = lf_checker_rt::callee_thiscall!(
                            8, u32, ctx_arg, idx, prev, step);
                        if (placed & 0xff) == 0 {
                            ok = 0;
                        } else {
                            ok = 1;
                            let bit = 1u32 << (i & 31);
                            let (lo, hi) = if i < 0x20 {
                                (bit, 0)
                            } else if i < 0x40 {
                                (0, bit)
                            } else {
                                (0, 0)
                            };
                            wr32(bits_out, lo);
                            wr32(bits_out.wrapping_add(4), hi);
                        }
                    }
                }
            }
            i += 1;
        }
        ok as u32
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0054d9f0 {
// original: 0x0054d9f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_26, player_schema::LeaderboardInfo, 10>::vf12

/// Find the rank id of leaderboard slot `index` in the rank table.
///
/// Calls the info-fetch callee (fastcall: ECX = schema id 0xb6, EDX = info
/// buffer) which reports success in AL and fills the buffer: rank count at
/// `+0x04`, rank-table pointer at `+0x08`, slot-table pointer at `+0x14`.
/// On success reads the rank id `slot[index]`; if it is -1 the answer is
/// `NOT_FOUND`. Otherwise linearly scans the rank table (up to `count`
/// entries, compared unsigned) for that id and returns the first matching
/// position, or `NOT_FOUND` (0xffffffff) when the count is zero or no
/// entry matches. The index is used as-is with no bounds check.
///
/// Original: stdcall, one stack word; incoming ECX is unused (overwritten
/// with the schema id before the call).
lf_checker_rt::export!(stdcall, rw_0054d9f0(index: u32) -> u32 {
    unsafe {
        const SCHEMA_ID: u32 = 0xb6;
        const COUNT_OFF: u32 = 0x04;
        const RANKS_OFF: u32 = 0x08;
        const SLOTS_OFF: u32 = 0x14;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(0, u32, SCHEMA_ID, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let slots = info[(SLOTS_OFF / 4) as usize];
        let needle = rd32(slots.wrapping_add(index.wrapping_mul(4)));
        if needle == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = info[(COUNT_OFF / 4) as usize];
        if count == 0 {
            return NOT_FOUND;
        }
        let ranks = info[(RANKS_OFF / 4) as usize];
        let mut i: u32 = 0;
        while i < count {
            if rd32(ranks.wrapping_add(i.wrapping_mul(4))) == needle {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0051f5e0 {
// original: 0x0051f5e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race32NoHolds, player_schema::LeaderboardInfo, 10>::vf7

/// Key fetch: fetch this leaderboard's key table and return the key at
/// `index`.
///
/// Calls the table-fetch callee with the leaderboard index
/// (`LEADERBOARD_INDEX`) in ECX and a scratch frame in EDX; the callee
/// fills the key-table pointer at frame `+16`. Returns `MISSING` (-1)
/// when the fetch reports failure, otherwise the table word at `index`
/// (no bounds check: a wild index faults, like the original).
///
/// Original: 0x0051f5e0 (thiscall, one stack word; `this` is unread).
lf_checker_rt::export!(thiscall, rw_0051f5e0(this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_INDEX: u32 = 0x75;
        const FETCH_CALLEE: u32 = 1;
        const MISSING: u32 = 0xffff_ffff;
        let _ = this;
        #[repr(C)]
        struct FetchOut {
            _pad0: u32,
            _pad1: u32,
            _pad2: u32,
            _pad3: u32,
            keys: u32,
        }
        let mut out: FetchOut = core::mem::zeroed();
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE,
            u32,
            LEADERBOARD_INDEX,
            (&mut out as *mut FetchOut) as u32
        );
        if (ok & 0xff) == 0 {
            return MISSING;
        }
        ((out.keys.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned()
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_0057a310 {
// original: 0x0057a310 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_162, player_schema::LeaderboardInfo, 10>::vf13
/// Value lookup: mapped value of one key in this race's tables.
///
/// Asks the leaderboard-data callee (contract callee 1) for this race's
/// tables, passing LEADERBOARD_ID and a scratch record the callee fills with
/// the key count (+12), the key array (+16) and the value array (+20). Only
/// the low byte of the callee's answer is significant: zero means no data.
/// `wanted` is then searched for in KEYS (at most `count` entries, the bound
/// compared signed) and the value at the same position in VALUES is returned,
/// or -1 when absent, when the count is not positive, or when there is no
/// data. A post-search check for position -1 in the original is dead (a found
/// position is never negative) and is not reproduced.
///
/// Original: thiscall, object in ECX (ignored), one stack word.
lf_checker_rt::export!(thiscall, rw_0057a310(_this: u32, wanted: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x17a;
        const CALLEE_DATA: u32 = 1;
        const NONE: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }

        /// Scratch record the data callee fills: key count, key array and
        /// value array at the offsets the original's frame used.
        #[repr(C)]
        struct Tables {
            _pad: [u32; 3],
            count: u32,
            keys: u32,
            values: u32,
        }

        let mut tables = Tables { _pad: [0; 3], count: 0, keys: 0, values: 0 };
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_DATA, u32, LEADERBOARD_ID, &mut tables as *mut Tables as u32);
        // Only the low byte is the callee's boolean; the upper bytes are
        // whatever the caller left in the register on each side.
        if (answer & 0xFF) == 0 {
            return NONE;
        }
        let count = tables.count as i32;
        if count <= 0 {
            return NONE;
        }
        let mut at = 0i32;
        loop {
            if rd32(tables.keys.wrapping_add((at as u32).wrapping_mul(4))) == wanted {
                return rd32(tables.values.wrapping_add((at as u32).wrapping_mul(4)));
            }
            at = at.wrapping_add(1);
            if at >= count {
                return NONE;
            }
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00596530 {
// original: 0x00596530 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_265, player_schema::LeaderboardInfo, 10>::vf12
/// Look up one leaderboard value by row, then find that value's position in
/// the key column. Returns the column index, or -1 when the fetch fails, the
/// row holds -1, the table is empty, or the value is absent.
export!(stdcall, rw_00596530(arg: u32) -> i32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1dd;
        let mut buf = [0u32; 6];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return -1;
        }
        let count = buf[1];
        let keys = buf[2] as *const u32;
        let values = buf[5] as *const u32;
        let elem = *values.add(arg as usize);
        if elem == 0xFFFF_FFFF {
            return -1;
        }
        if count == 0 {
            return -1;
        }
        let mut i: u32 = 0;
        loop {
            if *keys.add(i as usize) == elem {
                return i as i32;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return -1;
            }
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00b2acb0 {
// original: 0x00B2ACB0 anchored_vector_blend
//! Anchored vector blend: two-object distance gates plus a vector-blend tail
//! (cdecl/3 -> u32).
//!
//! Behaviour. The function takes three object pointers. It resolves the
//! first one through two helper calls into a working object and tests a
//! masked type word: one value selects an early distance check, any other
//! value selects the main path.
//!
//! The early path loads a secondary object from the second argument, runs
//! the same resolver over the first argument, and measures the distance
//! between a three-float point chosen by a null test (either an offset into
//! the resolver answer or into the object it points at) and an anchor triple
//! in the secondary object. When a global limit does not exceed that
//! distance it stores one state byte and returns the chosen point; otherwise
//! it stores another state byte, clamps a second state byte down to thirteen,
//! and returns the clamped byte.
//!
//! The main path fetches a vector through the working object's getter slot,
//! measures its length, and runs three sampler calls that each fill two
//! frame words (the calls take a frame pointer as object and a heap or frame
//! pointer plus zero on the stack). From the six sampled words it derives a
//! scale factor (a global divided by a root, or zero when the root input is
//! an ordered zero), two differences, and two scaled products, then runs two
//! mixer calls that each fill three frame words. Two dot products of mixer
//! outputs, a ratio of one dot over the earlier root, and a chain of three
//! global comparisons with a state-byte check select between two intermediate
//! exits (which store a state byte and return the second mixer's answer) and
//! the tail.
//!
//! The tail re-fetches through the same getter slot, dots the answer against
//! a triple from the working object's child, clamps that dot through a
//! state-byte-selected low/high/none scheme against three more globals,
//! blends the child triple scaled by the clamped dot over a base triple,
//! publishes the blend into the first object and through two finish calls,
//! and finally gates on two more distance-to-global comparisons: either can
//! exit early (returning a heap pointer or the last getter answer), otherwise
//! a byte and a relocated global word are stored into the third object and
//! the word is returned.
//!
//! Proof notes. The getter slot calls are intercepted by planting stub
//! addresses in fabricated heap tables; the first two share one slot and
//! are told apart by a per-call answer sequence, while the third runs
//! against a second object's table. Sampler and mixer out-words are declared
//! callee writes with edge-valued rows, so both sides observe identical
//! out-words; the rewrite passes its own frame slots. Frame-pointer call
//! arguments are skipped with call-time snapshots of the pointed-to words,
//! except the last getter call's argument, whose words were already
//! snapshotted at the previous getter call with no writer between. The
//! child-null case faults identically on both sides after the getter call
//! (fault parity, kept at a low rate). Every comparison follows the original
//! exactly: below-or-equal branches use the negated ordered-above test so
//! unordered counts as taken, above branches use the ordered test, and the
//! zero-gated scale uses an ordered-zero test. All float steps are pinned
//! with an opaque compiler barrier.

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// File VAs (image base 0x400000) of the globals this function reads.
const G_EARLY_LIM: u32 = 0x00FE_8AE0;
const G_SCALE_K: u32 = 0x00FE_88E8;
const G_SQRT_LIM: u32 = 0x00FE_8A24;
const G_ROOT_LIM: u32 = 0x00FE_8B38;
const G_RATIO_LIM: u32 = 0x00E7_7F98;
const G_DIV_LIM: u32 = 0x00FE_8878;
const G_CLAMP_C0: u32 = 0x00FE_8DC0;
const G_CLAMP_LO: u32 = 0x00FE_8AD8;
const G_CLAMP_HI: u32 = 0x00FE_8DCC;
const G_TAIL_LIM: u32 = 0x00FE_8AF0;
const G_VEC_LIM: u32 = 0x00FE_8B1C;
const G_BASE_WORD: u32 = 0x0117_35B4;

/// Getter slot all three virtual calls go through (offset into the table).
const VT_GET: usize = 0xEC / 4;
/// Masked type value selecting the early path.
const TYPE_EARLY: u32 = 0xC0;
const TYPE_MASK: u32 = 0x3C0;

/// Single-precision steps pinned against operand reorder and contraction.
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    black_box(black_box(a) + black_box(b))
}
#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    black_box(black_box(a) - black_box(b))
}
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    black_box(black_box(a) * black_box(b))
}
#[inline(always)]
fn fdiv(a: f32, b: f32) -> f32 {
    black_box(black_box(a) / black_box(b))
}
#[inline(always)]
fn fsqrt(a: f32) -> f32 {
    black_box(black_box(a).sqrt())
}

/// The object's getter call: load the table, load the slot, call through it
/// exactly like the original, so both sides land on the planted stub.
#[inline(always)]
fn vcall_get(obj: u32, arg0: u32) -> u32 {
    let table = unsafe { (obj as *const u32).read() } as *const u32;
    let target = unsafe { table.add(VT_GET).read() } as usize;
    let f: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(target) };
    f(obj, arg0)
}

#[inline(always)]
fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read() }
}
#[inline(always)]
fn rdf(addr: u32) -> f32 {
    f32::from_bits(rd32(addr))
}

fn body<const MUT: bool>(o0: u32, o1: u32, o2: u32) -> u32 {
    let a1 = callee_thiscall!(1, u32, o0);
    let esi_a = callee_cdecl!(2, u32, a1);
    let type_word = rd32(esi_a.wrapping_add(0x28));

    if (type_word & TYPE_MASK) == TYPE_EARLY {
        let e1 = rd32(o1.wrapping_add(0x20));
        let p = callee_thiscall!(3, u32, o0);
        let q = rd32(p.wrapping_add(0x20));
        let leaf = if q == 0 {
            p.wrapping_add(0x10)
        } else {
            q.wrapping_add(0x30)
        };
        let f0 = rdf(leaf);
        let f1 = rdf(leaf.wrapping_add(4));
        let f2 = rdf(leaf.wrapping_add(8));
        let d1 = fsub(f1, rdf(e1.wrapping_add(0x34)));
        let d0 = fsub(f0, rdf(e1.wrapping_add(0x30)));
        let d2 = fsub(f2, rdf(e1.wrapping_add(0x38)));
        let dist2 = fadd(fadd(fmul(d1, d1), fmul(d0, d0)), fmul(d2, d2));
        let lim = f32::from_bits(unsafe { global::<u32>(G_EARLY_LIM).read() });
        let dist = fsqrt(dist2);
        if !(lim > dist) {
            let b = unsafe { ((o0 + 0x27) as *const u8).read() };
            let mark: u8 = if MUT { 0x1b } else { 0x1a };
            unsafe { ((o0 + 0x26) as *mut u8).write(mark) };
            let capped = if b < 0x0d { b as u32 } else { 0x0d };
            unsafe { ((o0 + 0x27) as *mut u8).write(capped as u8) };
            return capped;
        }
        unsafe { ((o0 + 0x26) as *mut u8).write(5) };
        return leaf;
    }

    let zeros = [0u32; 3];
    let v1 = vcall_get(esi_a, zeros.as_ptr() as u32);
    let v10 = rdf(v1);
    let v11 = rdf(v1.wrapping_add(4));
    let v12 = rdf(v1.wrapping_add(8));
    let dv1 = fadd(
        fadd(fmul(v10, v10), fmul(v11, v11)),
        fmul(v12, v12),
    );
    let sqrt_v1 = fsqrt(dv1);

    let a1x = rd32(o1.wrapping_add(0x20));
    let mut s1slot = [0u32; 2];
    let _ = callee_thiscall!(
        5, u32,
        s1slot.as_mut_ptr() as u32,
        a1x.wrapping_add(0x30),
        0
    );
    let e20 = rd32(esi_a.wrapping_add(0x20));
    let s2arg = if e20 == 0 {
        esi_a.wrapping_add(0x10)
    } else {
        e20.wrapping_add(0x30)
    };
    let mut s2slot = [0u32; 2];
    let _ = callee_thiscall!(6, u32, s2slot.as_mut_ptr() as u32, s2arg, 0);
    let e20b = rd32(esi_a.wrapping_add(0x20));
    let mut s3slot = [0u32; 2];
    let _ = callee_thiscall!(
        7, u32,
        s3slot.as_mut_ptr() as u32,
        e20b.wrapping_add(0x10),
        0
    );
    let s1 = [f32::from_bits(s1slot[0]), f32::from_bits(s1slot[1])];
    let s2 = [f32::from_bits(s2slot[0]), f32::from_bits(s2slot[1])];
    let s3 = [f32::from_bits(s3slot[0]), f32::from_bits(s3slot[1])];

    let ss = fadd(fmul(s3[0], s3[0]), fmul(s3[1], s3[1]));
    let kk = f32::from_bits(unsafe { global::<u32>(G_SCALE_K).read() });
    let x2 = if ss != 0.0 { fdiv(kk, fsqrt(ss)) } else { 0.0 };
    let d_a = fsub(s1[1], s2[1]);
    let d_b = fsub(s1[0], s2[0]);
    let p3 = fmul(s3[0], x2);
    let q_a = fmul(d_a, d_a);
    let q_b = fmul(d_b, d_b);
    let p4 = fmul(s3[1], x2);
    let dd = fadd(q_a, q_b);
    let st_rq = fsqrt(dd);

    let mut t1slot = [0u32; 3];
    let pair1 = [p3.to_bits(), p4.to_bits()];
    let _ = callee_thiscall!(
        8, u32,
        t1slot.as_mut_ptr() as u32,
        pair1.as_ptr() as u32,
        1
    );
    let mut t2slot = [0u32; 3];
    let pair2 = [d_b.to_bits(), d_a.to_bits()];
    let a9 = callee_thiscall!(
        9, u32,
        t2slot.as_mut_ptr() as u32,
        pair2.as_ptr() as u32,
        1
    );
    let t1 = [
        f32::from_bits(t1slot[0]),
        f32::from_bits(t1slot[1]),
        f32::from_bits(t1slot[2]),
    ];
    let t2 = [
        f32::from_bits(t2slot[0]),
        f32::from_bits(t2slot[1]),
        f32::from_bits(t2slot[2]),
    ];
    let m0 = fmul(t2[1], t1[1]);
    let m1 = fmul(t2[0], t1[0]);
    let s_a = fadd(m1, m0);
    let m2 = fmul(t2[2], t1[2]);
    let res1 = fadd(s_a, m2);

    let g_a24 = f32::from_bits(unsafe { global::<u32>(G_SQRT_LIM).read() });
    let div = fdiv(res1, st_rq);
    if !(g_a24 > sqrt_v1) {
        return tail::<MUT>(o0, o1, o2, esi_a, a9, &t1slot);
    }
    let g_b38 = f32::from_bits(unsafe { global::<u32>(G_ROOT_LIM).read() });
    if !(g_b38 > st_rq) {
        return tail::<MUT>(o0, o1, o2, esi_a, a9, &t1slot);
    }
    let g_7798 = f32::from_bits(unsafe { global::<u32>(G_RATIO_LIM).read() });
    if !(g_7798 > div) {
        return zone03::<MUT>(o0, o1, o2, esi_a, a9, div, &t1slot);
    }
    let b26 = unsafe { ((o0 + 0x26) as *const u8).read() };
    if b26 != 0x22 {
        return zone03::<MUT>(o0, o1, o2, esi_a, a9, div, &t1slot);
    }
    unsafe { ((o0 + 0x26) as *mut u8).write(0x1c) };
    a9
}

fn zone03<const MUT: bool>(
    o0: u32, o1: u32, o2: u32, esi_a: u32, a9: u32, div: f32, t1slot: &[u32; 3],
) -> u32 {
    let g_8878 = f32::from_bits(unsafe { global::<u32>(G_DIV_LIM).read() });
    if !(div > g_8878) {
        return tail::<MUT>(o0, o1, o2, esi_a, a9, t1slot);
    }
    let b26 = unsafe { ((o0 + 0x26) as *const u8).read() };
    if b26 != 0x22 {
        unsafe { ((o0 + 0x26) as *mut u8).write(0x1c) };
        return a9;
    }
    tail::<MUT>(o0, o1, o2, esi_a, a9, t1slot)
}

fn tail<const MUT: bool>(
    o0: u32, o1: u32, o2: u32, esi_a: u32, _a9: u32, t1slot: &[u32; 3],
) -> u32 {
    let _ = MUT;
    let esi_b = rd32(esi_a.wrapping_add(0x20));
    let v2 = vcall_get(esi_a, t1slot.as_ptr() as u32);
    let w1 = rdf(v2.wrapping_add(4));
    let u0 = rdf(esi_b.wrapping_add(0x10));
    let w0 = rdf(v2);
    let u1 = rdf(esi_b.wrapping_add(0x14));
    let w2 = rdf(v2.wrapping_add(8));
    let u2 = rdf(esi_b.wrapping_add(0x18));
    let dot = fadd(fadd(fmul(w1, u1), fmul(u0, w0)), fmul(w2, u2));

    let b = unsafe { ((o0 + 0x26) as *const u8).read() };
    let cc = f32::from_bits(unsafe { global::<u32>(G_CLAMP_HI).read() });
    let d8 = f32::from_bits(unsafe { global::<u32>(G_CLAMP_LO).read() });
    let clamped = if b == 0x22 {
        if dot > cc { cc } else { dot }
    } else if b == 0x21 {
        if dot > d8 { dot } else { d8 }
    } else {
        let c0 = f32::from_bits(unsafe { global::<u32>(G_CLAMP_C0).read() });
        if c0 > dot {
            if dot > cc { cc } else { dot }
        } else if dot > d8 {
            dot
        } else {
            d8
        }
    };

    let m2 = fmul(u0, clamped);
    let m3 = fmul(u1, clamped);
    let m4 = fmul(u2, clamped);
    let base = esi_b.wrapping_add(0x30);
    let r_a = fadd(rdf(base), m2);
    let r_b = fadd(rdf(base.wrapping_add(4)), m3);
    let r_c = fadd(rdf(base.wrapping_add(8)), m4);
    unsafe { ((o0 + 0x26) as *mut u8).write(0x1a) };
    let _ = callee_thiscall!(10, u32, o0, 0);
    unsafe { ((o0 + 4) as *mut u32).write(r_a.to_bits()) };
    unsafe { ((o0 + 8) as *mut u32).write(r_b.to_bits()) };
    unsafe { ((o0 + 0x0c) as *mut u32).write(r_c.to_bits()) };
    // Pushed (o0, o1) so the top-first argument order is (o1, o0).
    let _ = callee_cdecl!(11, u32, o1, o0);

    let a1e = rd32(o1.wrapping_add(0x20));
    let h0 = rdf(a1e.wrapping_add(0x30));
    let h1 = rdf(a1e.wrapping_add(0x34));
    let h2 = rdf(a1e.wrapping_add(0x38));
    let e1 = fsub(r_b, h1);
    let e0 = fsub(r_a, h0);
    let e2 = fsub(r_c, h2);
    let dd2 = fadd(fadd(fmul(e1, e1), fmul(e0, e0)), fmul(e2, e2));
    let lim2 = f32::from_bits(unsafe { global::<u32>(G_TAIL_LIM).read() });
    let sq2 = fsqrt(dd2);
    if !(lim2 > sq2) {
        return a1e;
    }

    let v3 = vcall_get(o1, t1slot.as_ptr() as u32);
    let z0 = rdf(v3);
    let z1 = rdf(v3.wrapping_add(4));
    let z2 = rdf(v3.wrapping_add(8));
    let qq = fadd(fadd(fmul(z0, z0), fmul(z1, z1)), fmul(z2, z2));
    let sq3 = fsqrt(qq);
    let lim3 = f32::from_bits(unsafe { global::<u32>(G_VEC_LIM).read() });
    if !(sq3 > lim3) {
        return v3;
    }
    let g = unsafe { global::<u32>(G_BASE_WORD).read() }.wrapping_add(0x320);
    unsafe { ((o2 + 0x2a) as *mut u8).write(6) };
    unsafe { ((o2 + 0x10) as *mut u32).write(g) };
    g
}

export!(cdecl, rw_00b2acb0(o0: u32, o1: u32, o2: u32) -> u32 {
    body::<false>(o0, o1, o2)
});

export!(cdecl, mut_00b2acb0(o0: u32, o1: u32, o2: u32) -> u32 {
    body::<true>(o0, o1, o2)
});

use lf_checker_rt::{callee_stdcall, callee_fastcall, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}


#[allow(dead_code)]
mod k_00cf8b00 {
// original: 0x00cf8b00 ladder_task_dispatch (proposed)

/// Dispatch a climb-ladder task event by id (thiscall: task, ped).
///
/// `this` is the climb-ladder task, `task` the event id, `ped` the ped.
/// Each arm ends by returning its last callee's answer (or 0, or by
/// faulting on a null handle where the original does):
///
/// - 0x120: look up the ped's 0x84 entry (thiscall pair); run the align
///   callee (cdecl: frame slot, state word, pitch float, `this+0x20`,
///   `this+0x30`); fetch the shared handle (null returns 0); run the
///   start callee (thiscall with the handle: subtask word, `this+0x20`,
///   `this+0x30`, frame slot, pitch float, flag byte twice, 6 when the
///   state word is 4 else 4).
/// - 0xcb: fetch the shared handle (null returns 0); run the speed callee
///   (thiscall: 0x3e8 when `this+8` is set else 1, 0, 0, 8.0).
/// - 0x191: return the ladder anim-request callee over (state, flag byte).
/// - 0x386: smooth the pitch (raw `[this+0x70]`, or the x87 filter callee
///   over pitch + pi when the state is not 1 and the flag byte is 0);
///   fetch two handles (first null returns 0); without the second, run
///   the finish callee with a 0 result, else run the blend callee
///   (thiscall: smoothed pitch, 2.0, 0.02) and finish with its answer.
/// - default (anything else, e.g. 0x516): run the fallback callee with
///   (`this`, `ped`), return 0.
/// - 0x387/0x3ae: score the approach (see below), setting the flag byte;
///   0x387 then fetches two handles and finishes like 0x386 but through
///   the wide-blend callee (thiscall: 2.0, `this+0x40`, 0.2, 2.0, 0, 0);
///   0x3ae fetches one handle (null faults on `[0+0xdc]` like the
///   original), runs the full-blend callee (thiscall: 2.0, frame slot,
///   0.2, 3.0, -1, 1, 0, 0, 0, 1), sets bit 0x40 at result+0xdc, fetches
///   again (null returns 0) and finishes with the blend result.
/// - 0x3a6: smooth the pitch as in 0x386; when the state is 4, blend the
///   0x50-row by 0.1 into the 0x40-row (frame slots and `this+0x60..68`)
///   and store the unread frame slot (0 with the contract's zero stack
///   fill) at `this+0x6c`; fetch the handle (null faults on `[0+0xdc]`
///   like the original), run the commit callee (thiscall: frame slot,
///   smoothed pitch, 1000.0), stamp 2pi/0.1/1000 at result+0xdc/+0xe4/
///   +0xe8 and return the result.
///
/// Approach score: dx/dy from `this+0x20/0x24` minus target+0x30/0x34;
/// q = dy*dy + dx*dx; s = 1/sqrt(q) when q is positive or NaN (the
/// original's lahf/jp test), else 0; x1 = t14*(dy*s) + (dx*s)*t10 +
/// t18*(s*0), all in the original's operand order; the flag byte is 1
/// when -0.4 exceeds x1 (ordered) and the state is 4, else 0. The set
/// path also notifies (thiscall with (`[ped+0xa80]`, 1)) and retunes the
/// pitch (thiscall with (`ped`, pitch + pi)).
lf_checker_rt::export!(thiscall, rw_00cf8b00(this: u32, task: u32, ped: u32) -> u32 {
    unsafe {
        const ANIM_SET: u32 = 0x167e2a0;
        const SUBTASK: u32 = 8;
        const TASK_STATE: u32 = 0x14;
        const PITCH: u32 = 0x70;
        const FLAG75: u32 = 0x75;
        const FLAG74: u32 = 0x74;
        const STATE_WORD: u32 = 0x90;
        const PED_ANIM: u32 = 0xa80;
        const PED_SLOT: u32 = 0x78;
        const PED_TARGET: u32 = 0x20;
        const PI_BITS: u32 = 0x4049_0fdb;
        const DOT_LIMIT_BITS: u32 = 0xbecc_cccd;
        const BLEND_K_BITS: u32 = 0x3ca3_d70a;
        const WIDE_K_BITS: u32 = 0x3e4c_cccd;
        const FULL_K_BITS: u32 = 0x4040_0000;
        const STEP_K_BITS: u32 = 0x3dcc_cccd;
        const TWO: f32 = 2.0;
        const ONE: f32 = 1.0;
        const LOOKUP: u32 = 1;
        const ACQUIRE: u32 = 2;
        const ALIGN: u32 = 3;
        const FETCH: u32 = 4;
        const START: u32 = 5;
        const ANIMREQ: u32 = 6;
        const FILTER: u32 = 7;
        const BLEND: u32 = 8;
        const FINISH: u32 = 9;
        const FALLBACK: u32 = 10;
        const NOTIFY: u32 = 11;
        const RETUNE: u32 = 12;
        const WBLEND: u32 = 13;
        const FBLEND: u32 = 14;
        const COMMIT: u32 = 15;
        const SPEED: u32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn fetch_shared() -> u32 {
            unsafe {
                let set = (lf_checker_rt::relocated(ANIM_SET) as *const u32).read_unaligned();
                lf_checker_rt::callee_thiscall!(FETCH, u32, set)
            }
        }
        #[inline(always)]
        unsafe fn smooth_pitch(this: u32) -> f32 {
            unsafe {
                let state = rd32(this + TASK_STATE);
                let flag = ((this + FLAG75) as *const u8).read();
                if state == 1 || flag != 0 {
                    rdf(this + PITCH)
                } else {
                    let arg = add(rdf(this + PITCH), f32::from_bits(PI_BITS));
                    lf_checker_rt::callee_cdecl!(FILTER, f32, arg.to_bits())
                }
            }
        }

        match task {
            0x120 => {
                let slot = rd32(ped + PED_SLOT);
                let entry = lf_checker_rt::callee_thiscall!(LOOKUP, u32, slot, 0x84);
                if entry != 0 {
                    lf_checker_rt::callee_thiscall!(ACQUIRE, u32, slot, entry);
                }
                let mut f1 = [0u32; 1];
                lf_checker_rt::callee_cdecl!(
                    ALIGN, u32, f1.as_mut_ptr() as u32, rd32(this + TASK_STATE),
                    rd32(this + PITCH), this + 0x20, this + 0x30
                );
                let h = fetch_shared();
                if h == 0 {
                    return 0;
                }
                let k = if rd32(this + TASK_STATE) == 4 { 6 } else { 4 };
                let flag = ((this + FLAG74) as *const u8).read() as u32;
                let mut f2 = [0u32; 1];
                lf_checker_rt::callee_thiscall!(
                    START, u32, h, rd32(this + STATE_WORD), this + 0x20,
                    this + 0x30, f2.as_mut_ptr() as u32, rd32(this + PITCH), flag, k
                )
            }
            0xcb => {
                let h = fetch_shared();
                if h == 0 {
                    return 0;
                }
                let speed = if rd32(this + SUBTASK) == 0 { 1 } else { 0x3e8 };
                lf_checker_rt::callee_thiscall!(SPEED, u32, h, speed, 0, 0, 0x4100_0000)
            }
            0x191 => {
                let flag = ((this + FLAG75) as *const u8).read() as u32;
                lf_checker_rt::callee_cdecl!(ANIMREQ, u32, rd32(this + TASK_STATE), flag)
            }
            0x386 => {
                let x = smooth_pitch(this);
                let h1 = fetch_shared();
                if h1 == 0 {
                    return 0;
                }
                let h2 = fetch_shared();
                let r = if h2 == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        BLEND, u32, h2, x.to_bits(), TWO.to_bits(), BLEND_K_BITS
                    )
                };
                lf_checker_rt::callee_thiscall!(FINISH, u32, h1, r, 0, 0, 0)
            }
            0x387 | 0x3ae => {
                let tgt = rd32(ped + PED_TARGET);
                let dx = sub(rdf(this + 0x20), rdf(tgt + 0x30));
                let dy = sub(rdf(this + 0x24), rdf(tgt + 0x34));
                let q = add(mul(dy, dy), mul(dx, dx));
                let s = if q > 0.0 || q.is_nan() {
                    div(ONE, q.sqrt())
                } else {
                    0.0
                };
                let dy_s = mul(dy, s);
                let dx_s = mul(dx, s);
                let t3 = mul(dx_s, rdf(tgt + 0x10));
                let t1 = mul(rdf(tgt + 0x14), dy_s);
                let z = mul(s, 0.0);
                let t0 = mul(rdf(tgt + 0x18), z);
                let x1 = add(add(t1, t3), t0);
                let state = rd32(this + TASK_STATE);
                if f32::from_bits(DOT_LIMIT_BITS) > x1 && state == 4 {
                    ((this + FLAG75) as *mut u8).write(1);
                    let anim = rd32(ped + PED_ANIM);
                    lf_checker_rt::callee_thiscall!(NOTIFY, u32, anim, 1);
                    let tuned = add(rdf(this + PITCH), f32::from_bits(PI_BITS));
                    lf_checker_rt::callee_thiscall!(RETUNE, u32, ped, tuned.to_bits());
                } else {
                    ((this + FLAG75) as *mut u8).write(0);
                }
                if task == 0x387 {
                    let h1 = fetch_shared();
                    if h1 == 0 {
                        return 0;
                    }
                    let h2 = fetch_shared();
                    let r = if h2 == 0 {
                        0
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            WBLEND, u32, h2, TWO.to_bits(), this + 0x40,
                            WIDE_K_BITS, TWO.to_bits(), 0, 0
                        )
                    };
                    lf_checker_rt::callee_thiscall!(FINISH, u32, h1, r, 0, 0, 0)
                } else {
                    let h = fetch_shared();
                    let mut fs = [0u32; 1];
                    let r = if h == 0 {
                        0
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            FBLEND, u32, h, TWO.to_bits(), fs.as_mut_ptr() as u32,
                            WIDE_K_BITS, FULL_K_BITS, 0xffff_ffff, 1, 0, 0, 0, 1
                        )
                    };
                    let at = ((r + 0xdc) as *mut u32).read();
                    ((r + 0xdc) as *mut u32).write(at | 0x40);
                    let h2 = fetch_shared();
                    if h2 == 0 {
                        return 0;
                    }
                    lf_checker_rt::callee_thiscall!(FINISH, u32, h2, r, 0, 0, 0)
                }
            }
            0x3a6 => {
                let x = smooth_pitch(this);
                let v4 = rdf(this + 0x40);
                let v5 = rdf(this + 0x44);
                let v6 = rdf(this + 0x48);
                if rd32(this + TASK_STATE) == 4 {
                    let k = f32::from_bits(STEP_K_BITS);
                    let w3 = add(mul(rdf(this + 0x50), k), v4);
                    let w2 = add(mul(rdf(this + 0x54), k), v5);
                    let w1 = add(mul(rdf(this + 0x58), k), v6);
                    ((this + 0x6c) as *mut u32).write_unaligned(0);
                    ((this + 0x60) as *mut u32).write_unaligned(w3.to_bits());
                    ((this + 0x64) as *mut u32).write_unaligned(w2.to_bits());
                    ((this + 0x68) as *mut u32).write_unaligned(w1.to_bits());
                }
                let h = fetch_shared();
                let mut fs = [0u32; 1];
                let r = if h == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        COMMIT, u32, h, fs.as_mut_ptr() as u32, x.to_bits(), 0x447a_0000
                    )
                };
                ((r + 0xdc) as *mut u32).write(0x40c9_0fdb);
                ((r + 0xe4) as *mut u32).write(0x3dcc_cccd);
                ((r + 0xe8) as *mut u32).write(0x3e8);
                r
            }
            _ => {
                lf_checker_rt::callee_thiscall!(FALLBACK, u32, this, ped);
                0
            }
        }
    }
});

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, callee_addr, tls_slot, xmm_word, x87_raw, x87_f64, x87_f32};

}
