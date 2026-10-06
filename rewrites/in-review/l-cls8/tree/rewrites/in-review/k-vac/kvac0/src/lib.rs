#![allow(non_snake_case, unused, non_upper_case_globals)]


#[allow(dead_code)]
mod k_0051a030 {

// original: 0x0051a030 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race13NoHolds, player_schema::LeaderboardInfo, 10>::vf12
/// Reverse row search for one ranked leaderboard (`vf12 Race13`).
///
/// Fetches the tables, loads the row id at `index`, and scans the id array
/// for it, returning the position. Returns -1 when the fetch fails, when the
/// loaded id is -1, when the array is empty, or when the id is absent.
/// Leaderboard id: 0x6A.
export!(stdcall, rw_0051a030(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x6A;
        let mut frame = [0u32; 6];
        let fetch: extern "fastcall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if (fetch(LEADERBOARD_ID, frame.as_mut_ptr() as u32) & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = frame.as_ptr().add(5).read();
        let want = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        if want == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = frame.as_ptr().add(1).read();
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let ids = frame.as_ptr().add(2).read() as *const u32;
        let mut i = 0u32;
        while i < count {
            if ids.add(i as usize).read() == want {
                return i;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_005e5f80 {

// original: 0x005e5f80 hash_table_find_string
// rw_005e5f80: look up a string key in a chained hash table.
//
// Hashes the key, picks the bucket by remainder, and walks the chain
// comparing full strings; an empty tag selects the shared empty string on
// either side. Returns the value pointer of the first match, or null.
export!(thiscall, rw_005e5f80(this: u32, key: u32) -> u32 {
    unsafe {
        const EMPTY: u32 = 0xFC9C85;
        let obj = this as *const u8;
        let count = *((obj.add(4)) as *const u16);
        if count == 0 {
            return 0;
        }
        let key_tag = *((key as *const u8).add(4) as *const u16);
        let key_text = if key_tag == 0 {
            relocated(EMPTY)
        } else {
            *(key as *const u32)
        };
        let hash: u32 = callee_cdecl!(1, u32, key_text);
        let table = *(this as *const u32) as *const u32;
        let mut node = *table.add((hash % count as u32) as usize);
        while node != 0 {
            let tag = *((node as *const u8).add(4) as *const u16);
            let text = if tag == 0 {
                relocated(EMPTY)
            } else {
                *(node as *const u32)
            };
            let canon: u32 = callee_thiscall!(2, u32, key);
            let mut i = 0usize;
            let same = loop {
                let x = *((canon as *const u8).add(i));
                let y = *((text as *const u8).add(i));
                if x != y {
                    break false;
                }
                if x == 0 {
                    break true;
                }
                i += 1;
            };
            if same {
                return node.wrapping_add(8);
            }
            node = *((node as *const u8).add(0xC) as *const u32);
        }
        0
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00dfe14a {

// original: 0x00dfe14a wcs_copy_checked
// rs03f13: bounded wide-character copy with error reporting (cdecl/3).
//
// Copies up to `n` wide characters from `src` to `dst`, stopping after an
// embedded NUL. A null destination or a zero count reports 0x16 without
// touching memory; a null source clears the destination word first and then
// reports 0x16. A copy that exhausts the count without finding a NUL clears
// the destination and reports 0x22. Success returns 0. Reports go through
// the errno slot provider (cdecl/0 callee 1) and the handler (cdecl/0
// callee 2).
export!(cdecl, rw_rs03f13(dst: u32, n: u32, src: u32) -> u32 {
    unsafe {
        if dst == 0 || n == 0 {
            *(callee_cdecl!(1, u32,) as *mut u32) = 0x16;
            callee_cdecl!(2, u32,);
            return 0x16;
        }
        if src == 0 {
            *(dst as *mut u16) = 0;
            *(callee_cdecl!(1, u32,) as *mut u32) = 0x16;
            callee_cdecl!(2, u32,);
            return 0x16;
        }
        let mut d = dst as *mut u16;
        let mut s = src as *const u16;
        let mut left = n;
        loop {
            let w = *s;
            *d = w;
            s = s.add(1);
            d = d.add(1);
            if w == 0 {
                return 0;
            }
            left -= 1;
            if left == 0 {
                break;
            }
        }
        *(dst as *mut u16) = 0;
        *(callee_cdecl!(1, u32,) as *mut u32) = 0x22;
        callee_cdecl!(2, u32,);
        0x22
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_008d8380 {

// original: 0x008d8380 slot_field_byte
/// Read an 8-bit field from a slot record: `base[(entry + a1) * 3][4]`.
export!(cdecl, rw_008d8380(a1: u32, a2: u32) -> u32 {
    unsafe {
        let row = (*global::<u32>(0x013053A8)
            .byte_add(a2.wrapping_mul(100) as usize)).wrapping_add(a1).wrapping_mul(3);
        let base = *global::<u32>(0x0103E8D0);
        let addr = base.wrapping_add(row.wrapping_mul(8)).wrapping_add(0x04);
        (addr as *const u8).read() as u32
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e636f0 {

// original: 0x00e636f0 f32_ratio_store_3
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at a third address triple.
export!(cdecl, rw_00e636f0() -> () {
    unsafe {
        let a = *global::<f32>(0x1036AD4);
        let b = *global::<f32>(0x1036AD8);
        *global::<f32>(0x11A1BE4) = a / b;
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00bc77c0 {

// original: 0x00bc77c0 SET_CAR_RANDOM_ROUTE_SEED
/// Script native `SET_CAR_RANDOM_ROUTE_SEED` (hash 0x19D302AE).
///
/// Forwards two script arguments (a vehicle handle and a seed) to the engine. No return slot is written.
export!(cdecl, rw_00bc77c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00648da0 {

// original: 0x00648da0 rage::rmPtfxShaderVar_Float3::vf8
/// Store three float arguments into the value slots, copying the bits as-is.
export!(thiscall, rw_00648da0(this: *mut u8, x: u32, y: u32, z: u32) -> () {
    unsafe {
        *(this.add(0x20) as *mut u32) = x;
        *(this.add(0x24) as *mut u32) = y;
        *(this.add(0x28) as *mut u32) = z;
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00bc6d00 {

// original: 0x00bc6d00 IS_PLAYBACK_GOING_ON_FOR_CAR
/// Script native `IS_PLAYBACK_GOING_ON_FOR_CAR` (hash 0x375F145D).
///
/// Forwards one script argument (a vehicle handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6d00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00bb99a0 {

// original: 0x00bb99a0 TASK_GET_OFF_BOAT
/// Script native `TASK_GET_OFF_BOAT` (hash 0x6C63251D).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bb99a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00a00780 {

// original: 0x00a00780 CREATE_PICKUP
/// Script native `CREATE_PICKUP` (hash 0x7E2868D4).
///
/// Forwards seven script arguments to the engine: two integers, three float
/// bit-patterns (spawn coordinates), one integer, and a boolean flag. The
/// flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip). No return slot is written.
export!(cdecl, rw_00a00780(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(6) != 0);
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            flag,
        )
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00a011c0 {

// original: 0x00a011c0 HAS_OBJECT_BEEN_PHOTOGRAPHED
/// Script native `HAS_OBJECT_BEEN_PHOTOGRAPHED` (hash 0x57895F38).
///
/// Forwards one script argument (an object handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00a011c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e3f4c0 {

// original: 0x00e3f4c0 SlotRun_LeadCount
// 0x00E3F4C0: count the leading run of free (-1) slots in a row table.
// (stdcall/2)
export!(stdcall, rw_00e3f4c0(base: u32, count: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        let n = count as i32;
        if n <= 0 {
            return 0;
        }
        let mut run = 0i32;
        let mut i = 0i32;
        while i < n {
            let tag = *((base
                .wrapping_add(0x14)
                .wrapping_add((i as u32).wrapping_mul(STRIDE)))
                as *const i32);
            if tag != -1 {
                break;
            }
            run += 1;
            i += 1;
        }
        if run > 0 {
            run as u32
        } else {
            0
        }
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_006952e0 {

// original: 0x006952e0 anim_register_statics
/// Register the thirteen static animation objects in order.
export!(cdecl, rs80_6952e0() -> u32 {
    const OBJECTS: [u32; 13] = [
        0x01110298, 0x01110250, 0x01110274, 0x01110280, 0x011102BC, 0x01110244, 0x0111025C,
        0x011102A4, 0x011102B0, 0x01110238, 0x01110268, 0x011102C8, 0x0111028C,
    ];
    let mut r: u32 = 0;
    for o in OBJECTS {
        r = callee_thiscall!(1, u32, relocated(o));
    }
    r
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00bd9710 {

// original: 0x00bd9710 STOP_KILL_TRACKING
/// Script native `STOP_KILL_TRACKING` (hash 0x28CA0AFE).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd9710(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00ba1c70 {

// original: 0x00ba1c70 SET_CHAR_WILL_FLY_THROUGH_WINDSCREEN
/// Script native `SET_CHAR_WILL_FLY_THROUGH_WINDSCREEN` (hash 0x6FC75ABD).
///
/// Forwards a character handle and a boolean flag coerced with
/// `arg != 0` to the engine.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00ba1c70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00b945b0 {

// original: 0x00b945b0 GET_IS_DISPLAYINGSAVEMESSAGE
/// Reports whether the save message is showing (0 or 1 in return slot).
///
/// Calls the engine query with no arguments, zero-extends its low byte
/// and stores that in the return slot. Returns the return-slot pointer
/// (the original reloads it into `eax` for the store).
export!(cdecl, rw_00B945B0(ctx: *const u8) -> u32 {
    unsafe {
        let ans: u32 = callee_cdecl!(1, u32,);
        let ret = *(ctx as *const *mut u32);
        *ret = ans & 0xFF;
        ret as u32
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_008a0990 {

// original: 0x008a0990 forward_cdecl2_to_thiscall1_8a01e0
/// Forwards two cdecl arguments to a one-argument `thiscall` callee.
///
/// The first argument becomes the `this` pointer; the second is passed on the
/// stack. Returns the callee's answer unchanged.
export!(cdecl, rw_008a0990(a0: u32, a1: u32) -> u32 {
    unsafe { callee_thiscall!(1, u32, a0, a1) }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_008a2280 {

// original: 0x008a2280 rage::audCollapsingStereoSound::vf7
//! Collapse a stereo sound's four channels onto solved voices.
//!
//! After the gate accepts the request, each of the four channels is solved
//! in turn: the mix cursor is preserved across the solver call, and the
//! returned voice address is converted back to a slot value by subtracting
//! the bank base and dividing by the voice stride (an unassigned channel
//! keeps the empty marker). If any channel ends up without a voice the
//! collapse is rejected. Otherwise each parameter set is routed through
//! the melt hook, the first pair is retuned (from live parameters when
//! both are present, else with fixed defaults), the block's levels are
//! latched, and the collapse is accepted. Returns 1 on accept, 0 on reject.
//!
//! Callee ids (see contract): 1 = collapse gate (thiscall/3, low byte
//! tested), 2 = field solver (thiscall/4 on the global field object, answer
//! converted back to a slot), 3 = melt hook (thiscall/1 through the
//! object's function table slot 4), 4 = retune (thiscall/1).
//!
//! Note: the voice-table constants, `voice_for_slot` and `cvt_f32_i32` below
//! are shared verbatim with this lane's other rewrite files; keep one copy
//! when merging.

/// Row stride of the voice bank table: one bank holds voices for 256 slots.
const VOICE_BANK_STRIDE: u32 = 0x6f40;
/// Bias from a bank row start to its voice-array pointer.
const VOICE_TABLE_BIAS: u32 = 0x6f10;
/// Slot value meaning "no voice assigned".
const NO_VOICE: u8 = 0xFF;
/// Global: byte stride between adjacent voices in a voice array.
const G_VOICE_STRIDE: u32 = 0x115d964;
/// Global: pointer to the voice bank table.
const G_VOICE_TABLE: u32 = 0x115d988;

/// Resolve the voice object for bank `bank` and slot value `idx`.
///
/// A slot holding 0xFF means no voice; otherwise the bank table gives the
/// voice-array base and the slot value indexes into it with the global stride.
fn voice_for_slot(bank: u8, idx: u8) -> u32 {
    if idx == NO_VOICE {
        return 0;
    }
    unsafe {
        let stride = *(global::<u32>(G_VOICE_STRIDE) as *const u32);
        let table = *(global::<u32>(G_VOICE_TABLE) as *const u32);
        let row = (bank as u32).wrapping_mul(VOICE_BANK_STRIDE);
        let base = *((table.wrapping_add(row).wrapping_add(VOICE_TABLE_BIAS)) as *const u32);
        base.wrapping_add(stride.wrapping_mul(idx as u32))
    }
}

/// Truncate a float toward zero with x86 `cvttss2si` semantics: NaN and
/// out-of-range inputs yield 0x80000000 instead of saturating the way a
/// plain Rust float-to-int cast would.
fn cvt_f32_i32(f: f32) -> i32 {
    if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
        0x80000000u32 as i32
    } else {
        f as i32
    }
}

export!(thiscall, rw_008a2280(this: *mut u8, arg0: u32, arg1: u32, cursor: *mut u8) -> u32 {
    unsafe {
        let gate: u32 = callee_thiscall!(1, u32, this as u32, arg0, arg1, cursor as u32);
        if gate & 0xFF == 0 {
            return 0;
        }
        let bank = *(this.add(0x40) as *const u8);
        let blk = *(this.add(0x94) as *const u32);
        let stride = *(global::<u32>(G_VOICE_STRIDE) as *const u32);
        let table = *(global::<u32>(G_VOICE_TABLE) as *const u32);
        let row = (bank as u32).wrapping_mul(VOICE_BANK_STRIDE);
        let tab = *((table.wrapping_add(row).wrapping_add(VOICE_TABLE_BIAS)) as *const u32);
        let field = relocated(0x115dc18);
        for pass in 0..4u32 {
            let mut saved = [0u32; 6];
            for i in 0..6 {
                saved[i] = *((cursor as *const u32).add(i));
            }
            let pick = *((blk.wrapping_add(if pass % 2 == 0 { 0 } else { 4 })) as *const u32);
            let ans: u32 = callee_thiscall!(2, u32, field, pick, this as u32, arg1, cursor as u32);
            for i in 0..6 {
                *((cursor as *mut u32).add(i)) = saved[i];
            }
            // A null answer leaves the channel empty; otherwise the slot is
            // the voice index the solver address encodes. The stride is a
            // fixed table layout constant and is never zero in practice.
            let slot = if ans == 0 {
                NO_VOICE
            } else if stride == 0 {
                NO_VOICE
            } else {
                (ans.wrapping_sub(tab) / stride) as u8
            };
            *(this.add((0x48 + pass) as usize) as *mut u8) = slot;
        }
        for slot in 0..4usize {
            let v = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
            if v == 0 {
                return 0;
            }
        }
        let vtable = *(this as *const u32);
        let melt: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtable.wrapping_add(0x10)) as *const u32) as usize);
        for i in 0..5u32 {
            let p = *((blk.wrapping_add(0x10).wrapping_add(i.wrapping_mul(4))) as *const u32);
            if p != 0 {
                let ans = melt(this as u32, p);
                *((this.add(0xb8).wrapping_add((i.wrapping_mul(4)) as usize)) as *mut u32) = ans;
            }
        }
        let tune0 = *(this.add(0xc4) as *const u32);
        let tune1 = *(this.add(0xc8) as *const u32);
        if tune1 != 0 && tune0 != 0 {
            let a = cvt_f32_i32(f32::from_bits(*(tune0 as *const u32)));
            let v0 = voice_for_slot(bank, *(this.add(0x48) as *const u8));
            callee_thiscall!(4, u32, v0, a as u32);
            let b = cvt_f32_i32(f32::from_bits(*(tune1 as *const u32)));
            let v1 = voice_for_slot(bank, *(this.add(0x49) as *const u8));
            callee_thiscall!(4, u32, v1, b as u32);
        } else {
            let v0 = voice_for_slot(bank, *(this.add(0x48) as *const u8));
            callee_thiscall!(4, u32, v0, 0x10e);
            let v1 = voice_for_slot(bank, *(this.add(0x49) as *const u8));
            callee_thiscall!(4, u32, v1, 0x5a);
        }
        *(this.add(0xb0) as *mut u32) = *((blk.wrapping_add(8)) as *const u32);
        *(this.add(0xb4) as *mut u32) = *((blk.wrapping_add(0xc)) as *const u32);
        *(this.add(0xcd) as *mut u8) = *((blk.wrapping_add(0x24)) as *const u8);
        1
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00ae12c0 {

// original: 0x00ae12c0 ui_edge1_or_state_bit0
/// Input predicate: rising edge on line 1 bit 0, or the two-byte state gate.
///
/// Returns 1 on a 0-to-1 edge of bit 0 between the previous and current
/// shared words; otherwise, when the device-enable byte is nonzero, when
/// the first checksum is above 0x7f while the second is not. The upper 24
/// bits of the result repeat the worker answer.
export!(cdecl, rw_00ae12c0() -> u32 {
    unsafe {
        const PREV: u32 = 0x018B7A84;
        const CUR: u32 = 0x018B7A88;
        let dev = callee_cdecl!(1, u32, 0);
        let cur = *global::<u32>(CUR);
        let rise = (cur ^ *global::<u32>(PREV)) & cur;
        let out = if rise & 1 != 0 {
            1
        } else if *((dev + 0x328c) as *const u8) == 0 {
            0
        } else {
            let refb = *((dev + 0x2bcc) as *const u8);
            let d1 = *((dev + 0x2bce) as *const u8) ^ refb;
            if d1 <= 0x7f {
                0
            } else {
                let d2 = *((dev + 0x2bcf) as *const u8) ^ refb;
                (d2 <= 0x7f) as u32
            }
        };
        (dev & 0xFFFFFF00) | out
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_008abef0 {

// original: 0x008ABEF0 rage::audBiquadFilterEffect::vf0
/// Deleting destructor: release the extra buffer, restore the class
/// vtable, run the base destructor, and free the object when deleting.
export!(thiscall, rw_008ABEF0(obj: *mut u8, deleting: u32) -> u32 {
    unsafe {
        let extra = *(obj.add(0x74) as *const u32);
        *(obj as *mut u32) = relocated(0x00E7C548);
        callee_cdecl!(1, u32, extra);
        callee_thiscall!(2, u32, obj as u32);
        if deleting & 1 != 0 {
            callee_cdecl!(1, u32, obj as u32);
        }
        obj as u32
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00ba0130 {

// original: 0x00ba0130 IS_PED_IN_COMBAT
/// Script native `IS_PED_IN_COMBAT` (hash 0x020106D6).
///
/// Forwards one script argument to the engine and stores the low byte of
/// its answer (zero-extended) into the return slot.
export!(cdecl, rw_00ba0130(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00b952a0 {

// original: 0x00b952a0 TERMINATE_ALL_SCRIPTS_WITH_THIS_NAME
/// Script native `TERMINATE_ALL_SCRIPTS_WITH_THIS_NAME`.
///
/// Forwards one script argument (a script-name hash) to the engine. No
/// return slot is written. (The original cleans its one pushed argument
/// with `(an instruction of the original)`; the effect on the stack pointer is identical to the
/// plain cdecl return here.)
export!(cdecl, rw_00b952a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_0069a4b0 {

// original: 0x0069a4b0 install_vtable_fe3c94
/// Install the channel vtable at `obj`, doing nothing for a null pointer.
/// Returns the pointer it was given.
export!(cdecl, rw_0069a4b0(obj: u32) -> u32 {
    unsafe {
        if obj != 0 {
            *(obj as *mut u32) = relocated(0xFE3C94);
        }
        obj
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e63800 {

// original: 0x00e63800 f32_ratio_store_7
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// Same shape as [`rw_00e634c0`] at a seventh address triple.
export!(cdecl, rw_00e63800() -> () {
    unsafe {
        let a = *global::<f32>(0x1036EA8);
        let b = *global::<f32>(0x1036EAC);
        *global::<f32>(0x11A2EB4) = a / b;
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00c69250 {

// original: 0x00c69250 gated_count_check
// Return 1 when the id's table object carries 1 at +0x70 and the embedded
// slot array holds at most 2 leading entries, else 0.
export!(thiscall, rw_00c69250(obj: u32, id: u32) -> u32 {
    unsafe {
        let ent = *(relocated(0x1295cd8).wrapping_add(id.wrapping_mul(4)) as *const u32);
        if *((ent as *const u8).add(0x70) as *const u32) != 1 {
            return 0;
        }
        let n: u32 = callee_thiscall!(1, u32, obj);
        if (n as i32) > 2 {
            0
        } else {
            1
        }
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_006259b0 {

// original: 0x006259b0 dllist_insert_64
// Insert a node into a doubly-linked list with head, tail and count.
//
// With a null position appends at the tail (initialising an empty list);
// before the head prepends; otherwise links the node in front of the
// position node. Increments the count and publishes the (node, list) pair
// through the out pointer. Returns the out pointer.
export!(thiscall, rw_006259b0(list: u32, out: u32, pos: u32, node: u32) -> u32 {
    unsafe {
        const NEXT_OFF: u32 = 0x64;
        const PREV_OFF: u32 = 0x68;
        const TAIL_OFF: u32 = 4;
        const COUNT_OFF: u32 = 8;
        if pos == 0 {
            let tail: u32 = *((list.wrapping_add(TAIL_OFF)) as *const u32);
            if tail == 0 {
                let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
                *slot = (*slot).wrapping_add(1);
                *((list.wrapping_add(TAIL_OFF)) as *mut u32) = node;
                *(list as *mut u32) = node;
            } else {
                *((tail.wrapping_add(NEXT_OFF)) as *mut u32) = node;
                *((node.wrapping_add(PREV_OFF)) as *mut u32) = tail;
                *((list.wrapping_add(TAIL_OFF)) as *mut u32) = node;
                if *(list as *const u32) == 0 {
                    *(list as *mut u32) = node;
                }
                let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
                *slot = (*slot).wrapping_add(1);
            }
        } else if pos == *(list as *const u32) {
            *((node.wrapping_add(NEXT_OFF)) as *mut u32) = pos;
            *((pos.wrapping_add(PREV_OFF)) as *mut u32) = node;
            *(list as *mut u32) = node;
            let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
            *slot = (*slot).wrapping_add(1);
        } else {
            *((node.wrapping_add(NEXT_OFF)) as *mut u32) = pos;
            let prev: u32 = *((pos.wrapping_add(PREV_OFF)) as *const u32);
            *((node.wrapping_add(PREV_OFF)) as *mut u32) = prev;
            *((prev.wrapping_add(NEXT_OFF)) as *mut u32) = node;
            *((pos.wrapping_add(PREV_OFF)) as *mut u32) = node;
            let slot = (list.wrapping_add(COUNT_OFF)) as *mut u32;
            *slot = (*slot).wrapping_add(1);
        }
        *(out as *mut u32) = node;
        *((out.wrapping_add(4)) as *mut u32) = list;
        out
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00b45bf0 {

// original: 0x00b45bf0 CEventCarUndriveable::~CEventCarUndriveable
/// Destructor for CEventCarUndriveable::~CEventCarUndriveable: stamp the vtable, release 1 guarded member, tail into the base destructor.
///
/// Stamps the vtable pointer, then for each guarded member slot (0xc) calls the member cleanup with the slot address when the guard word is non-zero. Forwards to the base destructor (modelled as a tail call).
export!(thiscall, rw_00b45bf0(obj: *mut u8) -> u32 {
    unsafe {
        *(obj as *mut u32) = relocated(0xeae334);
        let member = *(obj.add(0xc) as *const u32);
        if member != 0 {
            callee_thiscall!(1, u32, member, (obj as u32).wrapping_add(0xc));
        }
        callee_thiscall!(9, u32, obj as u32)
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e5cef0 {

// original: 0x00e5cef0 timer_state_reset
/// Reset the mainloop timer state block, then report success.
///
/// Calls the timer helper (`thiscall/0`, stubbed) with ECX pointing at the
/// state block, then writes the power-on pattern over `0x01908DA8..0x01908E00`:
/// zeroed 64-bit slots and zeroed words mixed with `0xFFFFFFFF` marker dwords.
/// Three 2-byte gaps (`0xDAA`, `0xDE2`, `0xDEA`) are left untouched, and two
/// 64-bit slots are written twice, exactly like the original. Returns 0.
export!(cdecl, rw_00e5cef0() -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x1908DC0));
        *global::<u64>(0x1908DC0) = 0;
        *global::<u64>(0x1908DC8) = 0;
        *global::<u32>(0x1908DA8) = 0xFFFFFFFF;
        *global::<u64>(0x1908DB0) = 0;
        *global::<u64>(0x1908DB8) = 0;
        *global::<u32>(0x1908DD0) = 0;
        *global::<u64>(0x1908DC0) = 0;
        *global::<u64>(0x1908DC8) = 0;
        *global::<u32>(0x1908DD4) = 0xFFFFFFFF;
        *global::<u16>(0x1908DD8) = 0;
        *global::<u32>(0x1908DDC) = 0xFFFFFFFF;
        *global::<u16>(0x1908DE0) = 0;
        *global::<u32>(0x1908DE4) = 0xFFFFFFFF;
        *global::<u16>(0x1908DE8) = 0;
        *global::<u32>(0x1908DEC) = 0;
        *global::<u64>(0x1908DF0) = 0;
        *global::<u32>(0x1908DF8) = 0xFFFFFFFF;
        *global::<u32>(0x1908DFC) = 0xFFFFFFFF;
        0
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_006264d0 {

// original: 0x006264d0 vtable_release_publish_pair
// Release one object through its table, then publish it on another.
//
// Does nothing for a null first object (that path returns entry residue and
// is not exercised). Otherwise calls slot 0 of the first object's table,
// then slot 3 of the second object's table with the first object as the
// argument. Returns the second call's answer.
export!(fastcall, rw_006264d0(first: u32, second: u32) -> u32 {
    unsafe {
        if first == 0 {
            return 0;
        }
        type SlotFn = extern "thiscall" fn(u32, u32) -> u32;
        let table_a: u32 = *(first as *const u32);
        let release: SlotFn =
            core::mem::transmute((*(table_a as *const u32)) as usize);
        let _released: u32 = release(first, 0);
        let table_b: u32 = *(second as *const u32);
        let publish: SlotFn =
            core::mem::transmute((*((table_b.wrapping_add(0x0c)) as *const u32)) as usize);
        publish(second, first)
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00952c20 {

// original: 0x00952c20 record_seed_from_clock
/// Seed a 12-byte record from the shared clock words, then fold in a tick.
///
/// Copies the two clock words and the generation byte-word into `dst`. When
/// the source flags carry bit 1, the tick replaces the low word, the counter
/// byte advances, and the high word is reloaded from the table slot the new
/// counter selects; otherwise the tick is added to the low word.
export!(cdecl, rw_00952c20(dst: u32, src: u32) -> u32 {
    unsafe {
        core::ptr::copy_nonoverlapping(
            global::<u8>(0x120CA50),
            dst as *mut u8,
            12,
        );
        let flags = *((src.wrapping_add(4)) as *const u8);
        if flags & 2 != 0 {
            let tick: u32 = callee_cdecl!(0, u32,);
            let counter = (dst.wrapping_add(8)) as *mut u8;
            *counter = (*counter).wrapping_add(1);
            *(dst as *mut u32) = tick;
            let slot = *counter as usize;
            let entry = *(relocated(0x11F6F7C).wrapping_add(slot as u32 * 4) as *const u32);
            *((dst.wrapping_add(4)) as *mut u32) = entry;
            entry
        } else {
            let tick: u32 = callee_cdecl!(0, u32,);
            let low = dst as *mut u32;
            *low = (*low).wrapping_add(tick);
            tick
        }
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e4e740 {

// original: 0x00e4e740 delete_clip_and_reconcile
/// Delete a gallery clip file and its tag sidecar, then reconcile the
/// viewer's item list (original 0x00E4E740).
///
/// Builds a path from a global directory prefix plus the per-item suffix at
/// `this+0x350+0x1F8`, deletes that file, swaps the last three characters
/// for the `tag` extension and deletes the sidecar too. Then it retires the
/// `+0x350` object through a helper-created replacement, and either takes a
/// short release path (replacement quiet but secondary check exactly 1) or
/// sweeps the counted item table under `+0x32C`, refreshing each entry whose
/// status query is below the `+0x30C` threshold, accumulating a second status
/// sweep into a sum that gates one final notification when it does not exceed
/// `+0x354`, and finally marks the `+0x32C` object dirty (`+0x218 = 1`) and
/// notifies it twice. Does nothing when `+0x350` is null. Returns nothing.
///
/// The overlong-path branch (>= 0x203 chars) is kept exact but unreachable in
/// trials: any such path already overwrote the original's security cookie, so
/// its behaviour is frame-dependent.
export!(thiscall, rw_00e4e740(this: u32) -> () {
    const SUFFIX_OFF: u32 = 0x1F8;
    const PREFIX: u32 = 0x01168DD8;
    const TAG_WORD: u32 = 0x00F19B64;
    const CLIPS_DIR: u32 = 0x00F19C68;
    const FACTORY: u32 = 0x01981A4C;
    const PATH_MAX: usize = 0x200;
    unsafe {
        let words = this as *const u32;
        let obj = words.add(0x350 / 4).read();
        if obj == 0 {
            return;
        }
        callee_cdecl!(1, u32, relocated(CLIPS_DIR), 0);
        // path = prefix + suffix; both short NUL-terminated strings.
        let mut path = [0u8; PATH_MAX];
        let prefix = relocated(PREFIX) as *const u8;
        let mut pre_len = 0usize;
        loop {
            let b = prefix.add(pre_len).read();
            path[pre_len] = b;
            pre_len += 1;
            if b == 0 {
                break;
            }
        }
        let pre_len = pre_len - 1;
        let suffix = (obj.wrapping_add(SUFFIX_OFF)) as *const u8;
        let mut suf_len = 0usize;
        while suffix.add(suf_len).read() != 0 {
            suf_len += 1;
        }
        let mut i = 0usize;
        while i <= suf_len {
            path[pre_len + i] = suffix.add(i).read();
            i += 1;
        }
        delete_file(path.as_ptr() as u32);
        let full_len = (pre_len + suf_len) as u32;
        let stem_len = full_len.wrapping_sub(3);
        if stem_len >= 0x200 {
            // Original jumps to a fatal-error stub here; unreachable without
            // smashing its stack cookie (see doc comment).
            panic!("gallery clip path too long");
        }
        path[stem_len as usize] = 0;
        let tag = (relocated(TAG_WORD) as *const u32).read();
        ((path.as_mut_ptr() as u32).wrapping_add(stem_len) as *mut u32).write(tag);
        delete_file(path.as_ptr() as u32);
        // Retire the +0x350 object through its replacement.
        let secondary = words.add(0x32C / 4).read();
        let token = vcall0(obj, 0x54);
        let fresh = callee_thiscall!(4, u32, relocated(FACTORY), token);
        if words.add(0x350 / 4).read() != 0 {
            vcall1(obj, 0x08, 1);
        }
        (this as *mut u32).add(0x350 / 4).write(0);
        vcall0(fresh, 0x238);
        vcall1(fresh, 0x218, 0);
        if vcall0(fresh, 0x1D4) != 0 {
            sweep_items(this, secondary, fresh);
            return;
        }
        if vcall0(secondary, 0x1D4) != 1 {
            sweep_items(this, secondary, fresh);
            return;
        }
        vcall1(fresh, 0x08, 1);
        callee_thiscall!(11, u32, secondary);
    }
});

/// Main item-table sweep plus the final notification sequence.
fn sweep_items(this: u32, secondary: u32, fresh: u32) {
    unsafe {
        let list = ((secondary.wrapping_add(0x1E0)) as *const u32).read();
        let first = vcall0(list, 0x1D0);
        let again = vcall0(list, 0x1D0);
        let first_base = (first as *const u32).read();
        if first_base == list_end(again) {
            finish_sweep(this, secondary);
            return;
        }
        let mut cursor = first_base.wrapping_add(4);
        loop {
            let prev = (cursor.wrapping_sub(4) as *const u32).read();
            if vcall0(prev, 0x1D4) == 0 {
                if vcall0(secondary, 0x1D4) > 1 {
                    callee_thiscall!(24, u32, secondary);
                }
                vcall1(prev, 0x08, 1);
                break;
            }
            let threshold = ((this.wrapping_add(0x30C)) as *const u32).read();
            if vcall0(prev, 0x1D4) >= threshold {
                cursor = cursor.wrapping_add(4);
                if cursor.wrapping_sub(4) == list_end(vcall0(list, 0x1D0)) {
                    break;
                }
                continue;
            }
            let current = (cursor as *const u32).read();
            if cursor != list_end(vcall0(list, 0x1D0)) {
                if vcall0(current, 0x1D4) != 0 {
                    let session = vcall1(current, 0x1E0, 0);
                    let cookie = vcall0(session, 0x4C);
                    vcall1(prev, 0x1D8, cookie);
                    let stamp = vcall0(prev, 0x4C);
                    vcall1(session, 0x58, stamp);
                    if current == fresh {
                        vcall2(current, 0x22C, 0, 1);
                    }
                    let ticket = vcall1(current, 0x1D0, 0);
                    callee_thiscall!(23, u32, ticket);
                    vcall0(current, 0x238);
                    vcall0(prev, 0x238);
                }
            }
            if cursor != list_end(vcall0(list, 0x1D0)) {
                if vcall0(current, 0x1D4) == 0 {
                    vcall1(current, 0x08, 1);
                }
            }
            cursor = cursor.wrapping_add(4);
            if cursor.wrapping_sub(4) != list_end(vcall0(list, 0x1D0)) {
                continue;
            }
            break;
        }
        finish_sweep(this, secondary);
    }
}

/// Second status sweep with the accumulating sum, the gated notification and
/// the final dirty-mark plus double notification of the secondary object.
fn finish_sweep(this: u32, secondary: u32) {
    unsafe {
        let list = ((secondary.wrapping_add(0x1E0)) as *const u32).read();
        let head = vcall0(list, 0x1D0);
        let head_base = (head as *const u32).read();
        let mut sum: u32 = 0;
        let mut cursor = head_base;
        if head_base != list_end(vcall0(list, 0x1D0)) {
            loop {
                let item = (cursor as *const u32).read();
                sum = sum.wrapping_add(vcall0(item, 0x1D4));
                cursor = cursor.wrapping_add(4);
                if cursor == list_end(vcall0(list, 0x1D0)) {
                    break;
                }
            }
        }
        let budget = ((this.wrapping_add(0x354)) as *const u32).read();
        if sum <= budget {
            let secondary2 = ((this.wrapping_add(0x32C)) as *const u32).read();
            let target = ((secondary2.wrapping_add(0x1EC)) as *const u32).read();
            vcall1(target, 0x120, 0);
        }
        callee_thiscall!(11, u32, secondary);
        (secondary.wrapping_add(0x218) as *mut u8).write(1);
        vcall1(secondary, 0x18, 1);
        callee_thiscall!(27, u32, secondary, 0, 1);
    }
}

/// End pointer of a counted table struct: base word plus u16 count words.
unsafe fn list_end(table: u32) -> u32 {
    let base = (table as *const u32).read();
    let count = ((table.wrapping_add(4)) as *const u16).read() as u32;
    base.wrapping_add(count.wrapping_mul(4))
}

/// File deletion through the import slot, exactly like the original's
/// `(an instruction of the original)`: both sides land on the checker's recorder stub.
unsafe fn delete_file(path: u32) -> u32 {
    const DELETE_FILE_SLOT: u32 = 0x00E73268;
    let addr = global::<u32>(DELETE_FILE_SLOT).read();
    let f: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(addr as usize);
    f(path)
}

/// Virtual call with no stack arguments: object in ECX, callee cleans up.
unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
    let vtable = (obj as *const u32).read();
    let addr = ((vtable.wrapping_add(slot)) as *const u32).read();
    let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(addr as usize);
    f(obj)
}

/// Virtual call with one stack argument.
unsafe fn vcall1(obj: u32, slot: u32, a0: u32) -> u32 {
    let vtable = (obj as *const u32).read();
    let addr = ((vtable.wrapping_add(slot)) as *const u32).read();
    let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(addr as usize);
    f(obj, a0)
}

/// Virtual call with two stack arguments.
unsafe fn vcall2(obj: u32, slot: u32, a0: u32, a1: u32) -> u32 {
    let vtable = (obj as *const u32).read();
    let addr = ((vtable.wrapping_add(slot)) as *const u32).read();
    let f: extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(addr as usize);
    f(obj, a0, a1)
}

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e5f500 {

// original: 0x00E5F500 prepend_static_node_10ef48
// Prepend the static node at 0x0110EF48 to the global list at 0x017ACD24.
//
// Reads the list head, stores it as the node's successor word
// (node + 4) and makes the node the new head. Returns the previous
// head, which is the value the original leaves in EAX.
lf_checker_rt::export!(cdecl, rw_00e5f500() -> u32 {
    unsafe {
        let head = lf_checker_rt::global::<u32>(0x017ACD24);
        let prev = *head;
        *lf_checker_rt::global::<u32>(0x0110EF4C) = prev;
        *head = lf_checker_rt::relocated(0x0110EF48);
        prev
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e61350 {

// original: 0x00e61350 init_then_submit_04
/// init_then_submit_04: run one initializer, then submit one stub.
///
/// Runs this pair's initializer, then hands this pair's code stub to the
/// shared submit routine. Returns the submit routine's answer; the
/// initializer's answer is discarded.
lf_checker_rt::export!(cdecl, rw_00e61350() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32,);
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E70040))
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00875ac0 {

// original: 0x00875ac0 crmt_filter_request_init_guarded
/// Initialize a filter-request object unless the pointer is null.
///
/// cdecl/1. Null-guarded form of [`rw_008759a0`]; a null pointer is a no-op.
export!(cdecl, rw_00875ac0(obj: *mut u8) -> () {
    unsafe {
        if obj.is_null() {
            return;
        }
        const TAG: u32 = 0x00130000;
        *(obj.add(0x04) as *mut u32) = TAG;
        *(obj.add(0x08) as *mut u32) = 0;
        *(obj.add(0x0c) as *mut u32) = 0;
        *(obj.add(0x10) as *mut u32) = 0;
        *(obj.add(0x14) as *mut u32) = 0;
        *(obj.add(0x18) as *mut u32) = 0;
        *(obj.add(0x1c) as *mut u32) = 0;
        *(obj as *mut u32) = relocated(0x00fe816c);
        *(obj.add(0x20) as *mut u32) = 0;
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e61430 {

// original: 0x00e61430 init_then_submit_11
/// init_then_submit_11: run one initializer, then submit one stub.
///
/// Runs this pair's initializer, then hands this pair's code stub to the
/// shared submit routine. Returns the submit routine's answer; the
/// initializer's answer is discarded.
lf_checker_rt::export!(cdecl, rw_00e61430() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32,);
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E700B0))
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e65bc0 {

// original: 0x00e65bc0 NO_VOICE
/// Hash the "NO_VOICE" voice name and stash the handle.
///
/// Calls the shared name-hash helper on this slot's static name string and
/// stores the resulting handle in this slot's static cell, returning it.
lf_checker_rt::export!(cdecl, rw_00e65bc0() -> u32 {
    let handle: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00E90F78), 0);
    unsafe { *lf_checker_rt::global::<u32>(0x012844B4) = handle; }
    handle
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00875090 {

// original: 0x00875090 source202_ctor
/// Construct a motion-source object.
///
/// Initialises the embedded member at offset 4 (vtable plus three zero
/// words, then its constructor through the shared data slot), installs
/// the source vtable, zeroes offsets `0x14..=0x90` and returns the object.
export!(thiscall, rw_00875090(this: u32) -> u32 {
    unsafe {
        const MEMBER_VT: usize = 1;
        const FIRST_WORD: usize = 0x14 / 4;
        const COUNT: usize = 32;
        let base = this as *mut u32;
        base.write(relocated(0xFE7FC8));
        base.add(2).write(0);
        base.add(MEMBER_VT).write(relocated(0xFE7FB4));
        base.add(3).write(0);
        base.add(4).write(0);
        let slot = global::<u32>(0xFE7FB8);
        let member_ctor: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute::<u32, extern "thiscall" fn(u32) -> u32>(*slot);
        let _: u32 = member_ctor(this.wrapping_add(4));
        base.write(relocated(0xFE811C));
        let mut i: usize = 0;
        while i < COUNT {
            base.add(FIRST_WORD + i).write(0);
            i += 1;
        }
        this
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00945dd0 {

// original: 0x00945dd0 radio_rebuild_station_tables
/// Rebuild the radio station tables for one station group (original 0x00945DD0).
///
/// Walks the group's entries: each entry pointer is refreshed through the
/// station manager, its item table is relocated and each item pointer is
/// refreshed (copying the weight byte into the item's weight word when the
/// item is active). The entry name is copied to a frame buffer and resolved
/// to a station object; when resolution succeeds every active item is linked
/// into the station (merging weights, appending to the item chain, or queuing
/// the entry on the station's pending list). When resolution fails a fresh
/// station object is allocated and published into the global station table.
/// Returns the cookie-check answer, like the original (callers ignore it).
export!(cdecl, rw_00945dd0(group: u32) -> u32 {
    const MANAGER: u32 = 0x0115D9A0;
    const COUNT_OFF: u32 = 0x0A;
    const SLOTS_OFF: u32 = 0x0B;
    unsafe {
        let manager = relocated(MANAGER);
        let mut outer: u8 = 0;
        while outer < ((group + COUNT_OFF) as *const u8).read() {
            let slot = group.wrapping_add(outer as u32 * 4);
            let entry = (slot + SLOTS_OFF) as *mut u32;
            let raw = entry.read();
            if raw != 0 {
                let station = callee_thiscall!(1, u32, manager, raw);
                entry.write(station);
                if station != 0 {
                    rebuild_entry(station, manager);
                }
            }
            outer = outer.wrapping_add(1);
        }
        // Epilogue cookie check; its answer is the (ignored) return value.
        callee_cdecl!(7, u32,)
    }
});

/// Refresh one entry's item table and link it into its station object.
#[inline(never)]
unsafe fn rebuild_entry(station: u32, manager: u32) {
    const ALLOC_SIZE: u32 = 0x1934;
    const NAME_LIMIT: u32 = 0xFF;
    const NAME_LEN_OFF: u32 = 0x1D;
    const NAME_OFF: u32 = 0x1E;
    const ITEM_COUNT_OFF: u32 = 0x13;
    const ITEMS_OFF: u32 = 0x14;
    const ITEM_BASE_OFF: u32 = 0x11D;
    const ITEMS_BASE_OFF: u32 = 0x11E;
    const ACTIVE_OFF: u32 = 0x0A;
    const WEIGHT_WORD_OFF: u32 = 0x39;
    const WEIGHT_BYTE_OFF: u32 = 0x3F;
    const CHAIN_OFF: u32 = 0x3B;
    const PENDING_HEAD_OFF: u32 = 0x17D0;
    const PENDING_LINK_OFF: u32 = 0x18;
    const STATION_COUNT: u32 = 0x011D74F1;
    const STATION_TABLE: u32 = 0x011D76AC;
    unsafe {
        let name_len = ((station + NAME_LEN_OFF) as *const u8).read() as u32;
        let base = station.wrapping_sub(0xFFu32.wrapping_sub(name_len));
        let item_count = ((base + ITEM_BASE_OFF) as *const u8).read();
        let items = base.wrapping_add(ITEMS_BASE_OFF);
        ((station + ITEM_COUNT_OFF) as *mut u8).write(item_count);
        ((station + ITEMS_OFF) as *mut u32).write(items);
        // Refresh every item pointer through the manager.
        let mut inner: u8 = 0;
        while inner < ((station + ITEM_COUNT_OFF) as *const u8).read() {
            let slot = (items + inner as u32 * 4) as *mut u32;
            let raw = slot.read();
            if raw != 0 {
                let item = callee_thiscall!(1, u32, manager, raw);
                slot.write(item);
                if ((item + ACTIVE_OFF) as *const u8).read() != 0 {
                    let weight = ((item + WEIGHT_BYTE_OFF) as *const u8).read();
                    ((item + WEIGHT_WORD_OFF) as *mut u16).write(weight as u16);
                }
            }
            inner = inner.wrapping_add(1);
        }
        // Resolve the entry name to a station object.
        let mut name = [0u8; 256];
        callee_cdecl!(
            2,
            u32,
            name.as_mut_ptr() as u32,
            station.wrapping_add(NAME_OFF),
            name_len
        );
        let name_len = ((station + NAME_LEN_OFF) as *const u8).read() as usize;
        if name_len >= NAME_LIMIT as usize {
            // Unreachable in the contract (names are short); the original
            // calls its fatal-error routine here.
            core::hint::unreachable_unchecked()
        }
        name[name_len] = 0;
        let resolved = callee_cdecl!(3, u32, name.as_mut_ptr() as u32);
        if resolved == 0 {
            // Resolution failed: allocate and publish a fresh object.
            let fresh = callee_cdecl!(5, u32, ALLOC_SIZE);
            let published = if fresh == 0 {
                0
            } else {
                callee_thiscall!(6, u32, fresh, station)
            };
            let count = global::<u8>(STATION_COUNT);
            let table = global::<u32>(STATION_TABLE).read() as *mut u32;
            table.add(count.read() as usize).write(published);
            count.write(count.read().wrapping_add(1));
            return;
        }
        // Link every active item into the resolved station.
        let mut link: u8 = 0;
        while link < ((station + ITEM_COUNT_OFF) as *const u8).read() {
            let item = ((items + link as u32 * 4) as *const u32).read();
            let active = ((item + ACTIVE_OFF) as *const u8).read();
            if active != 0 {
                let target = callee_thiscall!(4, u32, resolved, active as u32);
                if target == 0 {
                    // Queue the entry on the station's pending list.
                    let mut node = (resolved + PENDING_HEAD_OFF) as *const u32;
                    node = node.read() as *const u32;
                    while ((node as u32 + PENDING_LINK_OFF) as *const u32).read() != 0 {
                        node = ((node as u32 + PENDING_LINK_OFF) as *const u32).read()
                            as *const u32;
                    }
                    if node as u32 != station {
                        ((node as u32 + PENDING_LINK_OFF) as *mut u32).write(station);
                    }
                } else if target != item {
                    let extra =
                        ((item + WEIGHT_BYTE_OFF) as *const u8).read() as u16;
                    let slot = (target + WEIGHT_WORD_OFF) as *mut u16;
                    slot.write(slot.read().wrapping_add(extra));
                    // Append the item at the tail of the target's chain.
                    let mut node = target;
                    while ((node + CHAIN_OFF) as *const u32).read() != 0 {
                        node = ((node + CHAIN_OFF) as *const u32).read();
                    }
                    ((node + CHAIN_OFF) as *mut u32).write(item);
                }
            }
            link = link.wrapping_add(1);
        }
    }
}

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00a7e7c0 {

// original: 0x00a7e7c0 CAnimTaskInfo::vf0
/// Scalar deleting destructor for `CAnimTaskInfo` (vtable slot 0).
///
/// Runs the class destructor, then frees the object through the game's
/// allocator when the low flag bit is set. Returns `this`.
lf_checker_rt::export!(thiscall, rw_00a7e7c0(this: u32, flags: u32) -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        let allocator = unsafe { lf_checker_rt::global::<u32>(0x12fb1ac).read() };
        lf_checker_rt::callee_thiscall!(2, u32, allocator, this);
    }
    this
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00dfc669 {

// original: 0x00dfc669 vswprintf_s_l
/// Locale-aware bounded wide-character formatted print.
///
/// Validates the buffer, size and format arguments (reporting `EINVAL` and
/// returning -1 when any is missing), then forwards everything to the
/// formatting helper. A negative result NUL-terminates the buffer; the
/// helper's truncation signal is translated to `ERANGE` with a -1 return.
export!(cdecl, rw_00dfc669(buf: u32, count: u32, fmt: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        if fmt == 0 || buf == 0 || count == 0 {
            let slot = callee_cdecl!(1, u32,);
            (slot as *mut u32).write(0x16);
            callee_cdecl!(2, u32,);
            return 0xFFFF_FFFF;
        }
        let out = lf_checker_rt::relocated(0x00e082a2);
        let written = callee_cdecl!(3, u32, out, buf, count, fmt, a3, a4);
        if (written as i32) < 0 {
            (buf as *mut u16).write(0);
        }
        if written == 0xFFFF_FFFE {
            let slot = callee_cdecl!(1, u32,);
            (slot as *mut u32).write(0x22);
            callee_cdecl!(2, u32,);
            return 0xFFFF_FFFF;
        }
        written
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00ca2400 {

// original: 0x00ca2400 swap_ref_148

/// Swap the link at `+0x148`, adjusting refcounts through the vtable.
///
/// The old link, when non-null, goes through vtable slot 0x54 (callee 1)
/// and the byte at `+0x15` of the answer is decremented; the new value is
/// stored, and when non-null goes through the same slot with the answer
/// byte incremented. No value is returned. Both indirect calls use the
/// fabricated objects exactly like the original.
///
/// Original: 0x00ca2400 (thiscall, one stack word = new link).
lf_checker_rt::export!(thiscall, rw_00ca2400(this: u32, new: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const LINK: u32 = 0x148;
        const SLOT: u32 = 0x54;
        const REFCOUNT: u32 = 0x15;
        let old = rd32(this + LINK);
        if old != 0 {
            let slot = rd32(rd32(old) + SLOT);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            let back = f(old);
            wr8(back + REFCOUNT, rd8(back + REFCOUNT).wrapping_sub(1));
        }
        wr32(this + LINK, new);
        if new != 0 {
            let slot = rd32(rd32(new) + SLOT);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            let back = f(new);
            wr8(back + REFCOUNT, rd8(back + REFCOUNT).wrapping_add(1));
        }
        0
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00cdcd50 {

// original: 0x00CDCD50 CTaskSimpleNMBalance::vf27 (merged symbol, vtable slot 27)

/// Balance-task update: forward the ped's position to the NaturalMotion
/// engine and keep the task's balance/brace messages alive.
///
/// `this` is the task, `ped` the ped. The work runs in four stages, each
/// guarded by the task and ped state:
///
/// 1. Retrigger stamp: when the ped's sub-object at `+0x16c` has mode bits
///    `0xc0` (bits 6-9 of its word at `+0x28`) and its flag at `+0x219` is
///    set, and the ped's timer at `+0x170` is above zero, the stamp global
///    is copied to the task word at `+0x24`.
/// 2. Position message: when the task object at `+0x28` exists, its anchor
///    point (the matrix at `+0x20` plus `0x30`, or the object itself plus
///    `0x10`) is read as three floats. A mode-`0xc0` anchor is first passed
///    to the anchor callee with code `0x4b5`; then a message carrying the
///    point as a vec3 slot is built on the stack and sent.
/// 3. Balance message: when the timer at `+0x50` is above zero a message is
///    built. Flag bits 9-10 of the word at `+0xc0` select a short form (one
///    cleared boolean slot, timer reset to zero). Otherwise the second
///    timer at `+0xe4` ticks down by the frame step and expiry also selects
///    the short form. The full form blends the lean vector at `+0x30`/`+0x34`
///    (scaled by its own length through the square-root callee when the
///    squared length exceeds 0.01, else the ped matrix row at `+0x10`) with
///    two boolean slots, one vec3 slot and the `+0x50` timer as a float slot.
/// 4. Shared update plus brace message: the shared timer routine (the
///    function at 0x00CDCBF0) runs on the sub-object at `+0x60`. When the
///    flag at `+0x54` is set and the `+0xc0` hold bits are clear, a set flag
///    at `+0x56` selects a tiny message (one cleared boolean slot, flag
///    cleared); set hold bits select the brace message instead: the reach
///    callee fills a point, its length scaled by the per-profile factor
///    table feeds a clamped float slot, and profile booleans/floats from the
///    same table fill the remaining slots.
///
/// Returns the last callee's answer. Original: 0x00CDCD50 (thiscall, one
/// stack word). Float operation order is the original's.
lf_checker_rt::export!(thiscall, rw_00cdcd50(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_STAMP: u32 = 0x24;
        const TASK_ANCHOR: u32 = 0x28;
        const TASK_LEAN_X: u32 = 0x30;
        const TASK_LEAN_Y: u32 = 0x34;
        const TASK_TIMER: u32 = 0x50;
        const TASK_FLAG54: u32 = 0x54;
        const TASK_FLAG56: u32 = 0x56;
        const TASK_SUB: u32 = 0x60;
        const TASK_HOLD: u32 = 0xc0;
        const TASK_PROFILE: u32 = 0xe0;
        const TASK_TIMER2: u32 = 0xe4;
        const PED_MATRIX: u32 = 0x20;
        const PED_SUB: u32 = 0x16c;
        const PED_TIMER: u32 = 0x170;
        const PED_NMCTX: u32 = 0x7b4;
        const ANCHOR_MATRIX: u32 = 0x20;
        const ANCHOR_MODE: u32 = 0x28;
        const ANCHOR_FLAG: u32 = 0x219;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_LIVE: u32 = 0xc0;
        const ANCHOR_CODE: u32 = 0x4b5;
        const LEN_LIMIT_BITS: u32 = 0x3c23d70a; // 0.01
        const DT_STEP: u32 = 0x11735bc;
        const STAMP_SRC: u32 = 0x11735b4;
        const ZERO_F: u32 = 0xfe8628;
        const NM_CTOR: u32 = 2;
        const NM_SET_BOOL: u32 = 3;
        const NM_SEND: u32 = 4;
        const NM_DTOR: u32 = 5;
        const NM_SET_VEC3: u32 = 7;
        const NM_SET_FLOAT: u32 = 8;
        const ANCHOR_CALLEE: u32 = 9;
        const SQRT_CALLEE: u32 = 10;
        const REACH_CALLEE: u32 = 11;
        const SHARED_FN: u32 = 12;
        const COOKIE: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn anchor_point(obj: u32) -> u32 {
            unsafe {
                let m = rd32(obj + ANCHOR_MATRIX);
                if m != 0 { m.wrapping_add(0x30) } else { obj.wrapping_add(0x10) }
            }
        }

        let zero = f32::from_bits(g32(ZERO_F));
        let mut eax = 0u32;

        // Stage 1: retrigger stamp.
        let psub = rd32(ped + PED_SUB);
        if psub != 0
            && (rd32(psub + ANCHOR_MODE) & MODE_MASK) == MODE_LIVE
            && rd8(psub + ANCHOR_FLAG) != 0
            && rdf(ped + PED_TIMER) > zero
        {
            wr32(this + TASK_STAMP, g32(STAMP_SRC));
        }

        // Stage 2: position message.
        let anchor = rd32(this + TASK_ANCHOR);
        if anchor != 0 {
            let p = anchor_point(anchor);
            let px = rdf(p);
            let py = rdf(p.wrapping_add(4));
            let pz = rdf(p.wrapping_add(8));
            if (rd32(anchor + ANCHOR_MODE) & MODE_MASK) == MODE_LIVE {
                let mut probe = [0u32; 3];
                probe[0] = px.to_bits();
                probe[1] = py.to_bits();
                probe[2] = pz.to_bits();
                eax = lf_checker_rt::callee_thiscall!(
                    ANCHOR_CALLEE, u32, anchor,
                    probe.as_mut_ptr() as u32, ANCHOR_CODE);
            }
            let mut msg = [0u32; 16];
            let buf = msg.as_mut_ptr() as u32;
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SET_BOOL, u32, buf, g32(0x1051e18), 1);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SET_VEC3, u32, buf, g32(0x1051e1c), px.to_bits(), py.to_bits(), pz.to_bits());
            eax = lf_checker_rt::callee_thiscall!(
                NM_SEND, u32, rd32(ped + PED_NMCTX), g32(0x1051dfc), buf);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
        }

        // Stage 3: balance message.
        if rdf(this + TASK_TIMER) > zero {
            let mut msg = [0u32; 16];
            let buf = msg.as_mut_ptr() as u32;
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
            let hold = rd32(this + TASK_HOLD);
            let short = ((hold >> 9) & 1) != 0 || ((hold >> 10) & 1) != 0;
            let mut short2 = short;
            if !short2 {
                let t = rdf(this + TASK_TIMER2);
                if t > 0.0 {
                    let rest = sub(t, f32::from_bits(g32(DT_STEP)));
                    wrf(this + TASK_TIMER2, rest);
                    if 0.0 >= rest {
                        short2 = true;
                    }
                }
            }
            if short2 {
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051cc8), 0);
                wr32(this + TASK_TIMER, 0);
            } else {
                let lx = rdf(this + TASK_LEAN_X);
                let ly = rdf(this + TASK_LEAN_Y);
                let x2 = mul(lx, lx);
                let y2 = mul(ly, ly);
                let len2 = add(x2, y2);
                let (vx, vy, vz);
                if len2 > f32::from_bits(LEN_LIMIT_BITS) {
                    let root: f32 = lf_checker_rt::callee_cdecl!(
                        SQRT_CALLEE, f32, len2.to_bits());
                    vx = mul(root, lx);
                    vy = mul(root, ly);
                    vz = mul(root, zero);
                    eax = root.to_bits();
                } else {
                    let m = rd32(ped + PED_MATRIX);
                    vx = rdf(m.wrapping_add(0x10));
                    vy = rdf(m.wrapping_add(0x14));
                    vz = rdf(m.wrapping_add(0x18));
                }
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051cc8), 1);
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051e98), 1);
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_VEC3, u32, buf, g32(0x1051e90), vx.to_bits(), vy.to_bits(), vz.to_bits());
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_FLOAT, u32, buf, g32(0x1051e94), rd32(this + TASK_TIMER));
            }
            eax = lf_checker_rt::callee_thiscall!(
                NM_SEND, u32, rd32(ped + PED_NMCTX), g32(0x1051e88), buf);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
        }

        // Stage 4: shared update, then the brace message.
        eax = lf_checker_rt::callee_thiscall!(SHARED_FN, u32, this.wrapping_add(TASK_SUB), ped);
        if rd8(this + TASK_FLAG54) != 0 {
            let hold = rd32(this + TASK_HOLD);
            if ((hold >> 9) & 1) == 0 && ((hold >> 10) & 1) == 0 {
                if rd8(this + TASK_FLAG56) != 0 {
                    let mut msg = [0u32; 16];
                    let buf = msg.as_mut_ptr() as u32;
                    eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_BOOL, u32, buf, g32(0x1051cc8), 0);
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SEND, u32, rd32(ped + PED_NMCTX), g32(0x1051dcc), buf);
                    wr8(this + TASK_FLAG56, 0);
                    eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
                }
            } else {
                let mut pin = [0u32; 3];
                let mut pout = [0u32; 3];
                eax = lf_checker_rt::callee_thiscall!(
                    REACH_CALLEE, u32, ped, pout.as_mut_ptr() as u32,
                    pin.as_mut_ptr() as u32, 0, 0);
                let px = f32::from_bits(pout[0]);
                let py = f32::from_bits(pout[1]);
                let pz = f32::from_bits(pout[2]);
                let row = rd32(this + TASK_PROFILE).wrapping_mul(0xb0);
                let prof = |va: u32| unsafe { rd32(lf_checker_rt::relocated(va).wrapping_add(row)) };
                let proff = |va: u32| f32::from_bits(prof(va));
                let d2 = add(add(mul(px, px), mul(py, py)), mul(pz, pz));
                // Original order: packed square root of the sum, times the factor.
                let root = core::hint::black_box(d2).sqrt();
                let dist = mul(root, proff(0x171cb38));
                let mut msg = [0u32; 16];
                let buf = msg.as_mut_ptr() as u32;
                eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
                if rd8(this + TASK_FLAG56) == 0 {
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_BOOL, u32, buf, g32(0x1051cc8), 1);
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_FLOAT, u32, buf, g32(0x1051dd8), prof(0x171cb24));
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_FLOAT, u32, buf, g32(0x1051de4), prof(0x171cb28));
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_FLOAT, u32, buf, g32(0x1051dec), prof(0x171cb2c));
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_FLOAT, u32, buf, g32(0x1051df0), prof(0x171cb30));
                }
                let top = proff(0x171cb34);
                let clamped = if prof(0x171cbb8) & 0xff == 0 {
                    top
                } else if 0.0 > dist {
                    0.0
                } else if dist > top {
                    top
                } else {
                    dist
                };
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_FLOAT, u32, buf, g32(0x1051de8), clamped.to_bits());
                let w = proff(0x171cb94);
                let ge = if w >= zero { 1u32 } else { 0u32 };
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051df4), ge);
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_FLOAT, u32, buf, g32(0x1051df8), w.to_bits());
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051dd4), 0);
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SEND, u32, rd32(ped + PED_NMCTX), g32(0x1051dcc), buf);
                wr8(this + TASK_FLAG56, 1);
                eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
            }
        }

        lf_checker_rt::callee_stdcall!(COOKIE, u32,);
        eax
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00511970 {

// original: 0x00511970 ctor_Ranked_Episodic_Race_123
/// Constructor for the ranked episodic race 123 leaderboard info object.
///
/// Runs the shared base constructor on `this`, then installs this class's
/// two vtables, sets the initialised flag bit, stores the empty-slot marker
/// (-1) and clears the five trailing state words. Returns `this`.
export!(thiscall, rw_00511970(this_ptr: u32) -> u32 {
    unsafe {
        // Base-class constructor (thiscall/0), intercepted by the checker.
        callee_thiscall!(1, u32, this_ptr);
        const VTABLE: u32 = 0x00fdb0e8;
        const VTABLE_INNER: u32 = 0x00fcf2cc;
        const EMPTY_SLOT: u32 = 0xffff_ffff;
        (this_ptr as *mut u32).write(relocated(VTABLE));
        (this_ptr.wrapping_add(0x4a0) as *mut u32).write(relocated(VTABLE_INNER));
        let flags = this_ptr.wrapping_add(0x5a4) as *mut u8;
        flags.write(flags.read() | 1);
        (this_ptr.wrapping_add(0x4a4) as *mut u32).write(EMPTY_SLOT);
        (this_ptr.wrapping_add(0x4a8) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4ac) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4b0) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4b4) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4b8) as *mut u32).write(0);
        this_ptr
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00be2c10 {

// original: 0x00be2c10 CTaskComplexMoveSequence::vf19 (symbols)

/// Forward to the shared sequence-step helper with this task's cursors.
///
/// Passes the incoming argument through unchanged, together with pointers to
/// the two cursor words stored inline in this task (`this + CURSOR_A`, pushed
/// first, and `this + CURSOR_B`). The callee reads the second cursor as an
/// index into the task's child table and dispatches through it; this wrapper
/// contributes no logic of its own. Returns the callee's answer.
///
/// The callee keeps the caller's stack frame layout: it pops all three words
/// itself (thiscall, three stack arguments, object in ECX).
///
/// Original: 0x00be2c10 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be2c10(this: u32, arg: u32) -> u32 {
    unsafe {
        const CURSOR_A: u32 = 0x6c;
        const CURSOR_B: u32 = 0x64;
        const STEP: u32 = 1;
        lf_checker_rt::callee_thiscall!(
            STEP,
            u32,
            this,
            arg,
            this.wrapping_add(CURSOR_B),
            this.wrapping_add(CURSOR_A)
        )
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_0068ef60 {

// original: 0x0068EF60 anim_fetch_tracks (proposed)

/// Copy stored per-bone records into an animation's tracks.
///
/// `this` holds two pointers: the track set (`+0x00`) and the record holder
/// (`+0x04`, whose record array pointer is at `+0x00`, 0xe0 bytes per record).
/// A track set has a lookup key (`+0x04`), an enable word (`+0x08`), the track
/// pointer array (`+0x0c`) and a 16-bit track count (`+0x10`). A track has a
/// flags byte (`+0x04`), a kind byte (`+0x05`: 0 = translation, 1 = rotation),
/// an id word (`+0x06`) and four value words at `+0x10..+0x1c`. `owner` carries
/// the record table (`+0x00`) and a ready word (`+0x2c`).
///
/// When the set is enabled, has a key, `owner` is ready, and the shared lookup
/// answers with an entry whose block (`+0x0c`) is present, the block lists
/// packed (track index << 16 | record index) words and each named record's
/// words are copied to its track (kind 0: `+0x20..+0x2c`; kind 1:
/// `+0x40..+0x4f`). Otherwise every track is visited and its record is found
/// through the owner's record-id lookup (callee), accepted only if the record
/// carries the kind's mask bits (0x380 for translation, 0x0e for rotation)
/// before the same copy runs. After either copy the track's flag bit 0x10 is
/// cleared. When the lookup supplied an entry, its reference count is dropped
/// under the owner's mutex at the end.
///
/// The record-id lookup's out word aliases the caller's incoming argument slot
/// (rotation arm) or a frame slot (translation arm) in the original; here both
/// are locals, so the stack comparison is off for this function.
///
/// Original: 0x0068EF60 (thiscall, three stack words; only the first, `owner`,
/// is read; returns nothing).
lf_checker_rt::export!(thiscall, rw_0068ef60(this: u32, owner: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const SET_KEY: u32 = 0x04;
        const SET_ENABLED: u32 = 0x08;
        const SET_TRACKS: u32 = 0x0c;
        const SET_COUNT: u32 = 0x10;
        const HOLDER_RECORDS: u32 = 0x00;
        const RECORD_STRIDE: u32 = 0xe0;
        const OWNER_READY: u32 = 0x2c;
        const TRACK_FLAGS: u32 = 0x04;
        const TRACK_KIND: u32 = 0x05;
        const TRACK_ID: u32 = 0x06;
        const TRACK_VALUE: u32 = 0x10;
        const SKIP_BIT: u8 = 0x10;
        const KIND_TRANSLATION: u8 = 0;
        const KIND_ROTATION: u8 = 1;
        const MASK_TRANSLATION: u32 = 0x380;
        const MASK_ROTATION: u8 = 0x0e;
        const TRANS_SRC: u32 = 0x20;
        const ROT_SRC: u32 = 0x40;
        const ENTRY_REFS: u32 = 0x08;
        const ENTRY_BLOCK: u32 = 0x0c;
        const OWNER_MUTEX: u32 = 0x10;
        const INFINITE: u32 = 0xffff_ffff;
        const IAT_WAIT: u32 = 0x00e7_3188;
        const IAT_RELEASE: u32 = 0x00e7_31b0;

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

        /// Copy the four translation words of a record to a track.
        unsafe fn copy_translation(track: u32, src: u32) {
            unsafe {
                wr32(track + TRACK_VALUE, rd32(src + TRANS_SRC));
                wr32(track + TRACK_VALUE + 4, rd32(src + TRANS_SRC + 4));
                wr32(track + TRACK_VALUE + 8, rd32(src + TRANS_SRC + 8));
                wr32(track + TRACK_VALUE + 12, rd32(src + TRANS_SRC + 12));
            }
        }

        /// Copy the four rotation words of a record to a track.
        unsafe fn copy_rotation(track: u32, src: u32) {
            unsafe {
                wr32(track + TRACK_VALUE, rd32(src + ROT_SRC));
                wr32(track + TRACK_VALUE + 4, rd32(src + ROT_SRC + 4));
                wr32(track + TRACK_VALUE + 8, rd32(src + ROT_SRC + 8));
                wr32(track + TRACK_VALUE + 12, rd32(src + ROT_SRC + 12));
            }
        }

        #[inline(always)]
        unsafe fn clear_skip(track: u32) {
            unsafe { wr8(track + TRACK_FLAGS, rd8(track + TRACK_FLAGS) & !SKIP_BIT) }
        }

        let set = rd32(this);
        // The shared lookup fills these two words: the owning object (holds the
        // mutex handle) and the entry (holds the reference count and block).
        let mut lookup_out = [0u32; 2];
        if rd32(set + SET_ENABLED) != 0 && rd32(owner + OWNER_READY) != 0 {
            let key = rd32(set + SET_KEY);
            if key != 0 {
                lf_checker_rt::callee_stdcall!(
                    1,
                    u32,
                    core::ptr::addr_of_mut!(lookup_out) as u32,
                    set,
                    owner,
                    key
                );
            }
        }
        let lock_owner = lookup_out[0];
        let entry = lookup_out[1];
        let block = if entry != 0 { rd32(entry + ENTRY_BLOCK) } else { 0 };

        if block != 0 {
            let count = rd32(block) as i32;
            if count > 0 {
                let tracks = rd32(set + SET_TRACKS);
                let mut item_addr = block + 4;
                for _ in 0..count {
                    let item = rd32(item_addr);
                    item_addr += 4;
                    let track = rd32(tracks + (item >> 16) * 4);
                    let src = rd32(rd32(this + 4) + HOLDER_RECORDS) + (item & 0xffff) * RECORD_STRIDE;
                    match rd8(track + TRACK_KIND) {
                        KIND_TRANSLATION => copy_translation(track, src),
                        KIND_ROTATION => copy_rotation(track, src),
                        _ => continue,
                    }
                    clear_skip(track);
                }
            }
        } else {
            let n = rd16(set + SET_COUNT);
            if n > 0 {
                let tracks = rd32(set + SET_TRACKS);
                for i in 0..n {
                    let track = rd32(tracks + i * 4);
                    let kind = rd8(track + TRACK_KIND);
                    if kind != KIND_TRANSLATION && kind != KIND_ROTATION {
                        continue;
                    }
                    // Record-id lookup on the owner: answers a bool and writes
                    // the record index (16 bits used) through its out pointer.
                    let mut slot = 0u32;
                    let found = lf_checker_rt::callee_thiscall!(
                        2,
                        u32,
                        owner,
                        rd16(track + TRACK_ID),
                        core::ptr::addr_of_mut!(slot) as u32
                    ) as u8;
                    if found == 0 {
                        continue;
                    }
                    let idx = slot & 0xffff;
                    let rec = rd32(owner) + idx * RECORD_STRIDE + 4;
                    let accepted = if kind == KIND_TRANSLATION {
                        rd32(rec) & MASK_TRANSLATION != 0
                    } else {
                        rd8(rec) & MASK_ROTATION != 0
                    };
                    if !accepted {
                        continue;
                    }
                    let src = rd32(rd32(this + 4) + HOLDER_RECORDS) + idx * RECORD_STRIDE;
                    if kind == KIND_TRANSLATION {
                        copy_translation(track, src);
                    } else {
                        copy_rotation(track, src);
                    }
                    clear_skip(track);
                }
            }
        }

        // Drop the entry's reference count under the owner's mutex.
        if entry != 0 {
            let mutex = rd32(lock_owner + OWNER_MUTEX);
            if mutex != 0 {
                let wait: extern "stdcall" fn(u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(IAT_WAIT).read() as usize);
                wait(mutex, INFINITE);
            }
            wr32(entry + ENTRY_REFS, rd32(entry + ENTRY_REFS).wrapping_sub(1));
            let mutex = rd32(lock_owner + OWNER_MUTEX);
            if mutex != 0 {
                let release: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(IAT_RELEASE).read() as usize);
                release(mutex);
            }
        }
        0
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00d08cc0 {

// original: 0x00D08CC0 nearest_ped_scan_16way (proposed)
//
// Scan a fixed 16-slot ped table for entries near a center point, counting
// the matches and tracking the smallest squared distance found.
//
// Arguments (cdecl, seven stack words): `list_head` points at a holder whose
// dword at +0x224 points at the slot array, whose 16 entries start at +0x168;
// `center` points at three floats (x, y, z); `radius_bits` is the search
// radius as float bits; `filter` selects the plain path when -1, else the
// callee-filtered path; `want_word` (unless -1) must equal the sign-extended
// word at ped +0x2e; `excluded` is one ped pointer to skip; `best_out` (may
// be null) receives the running best squared distance, seeded with
// radius*radius on entry.
// Each slot is skipped when null, flagged at +0x24 bit 0x400, dead (byte at
// +0x211), of the wrong kind (dword at [+0x21c]+0x12c != 2), or at squared
// distance (point-minus-center, summed (dy^2+dx^2)+dz^2) not below
// radius*radius. The filtered path resolves each candidate through three
// intercepted callees (probe/next/test); the plain path folds it straight
// into the best. The fold compares ((dy')^2+(dx')^2)+(dz')^2 with
// center-minus-point deltas against the stored best and keeps the smaller.
// Returns the match count. Float operation order is the original's.
lf_checker_rt::export!(cdecl, rw_00D08CC0(
    list_head: u32, center: u32, radius_bits: u32, filter: u32,
    want_word: u32, excluded: u32, best_out: u32,
) -> u32 {
    unsafe {
        const SLOTS_AT: u32 = 0x224;
        const SLOTS_ENTRIES: u32 = 0x168;
        const SLOT_COUNT: u32 = 16;
        const PED_FLAGS: u32 = 0x24;
        const PED_DEAD: u32 = 0x211;
        const PED_KIND_OBJ: u32 = 0x21c;
        const PED_MTX: u32 = 0x20;
        const PED_TAG: u32 = 0x2e;
        const PED_INFO: u32 = 0x224;
        const KIND_OFF: u32 = 0x12c;
        const KIND_WANTED: u32 = 2;
        const FLAG_SKIP: u32 = 0x400;
        const MTX_X: u32 = 0x30;
        const MTX_Y: u32 = 0x34;
        const MTX_Z: u32 = 0x38;
        const CALLEE_PROBE: u32 = 1;
        const CALLEE_NEXT: u32 = 2;
        const CALLEE_TEST: u32 = 3;
        const TEST_PROBE_ARG: u32 = 0x39b;
        const INFO_TEST_OFF: u32 = 0x2e0;
        const NONE: u32 = 0xFFFF_FFFF;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        /// Keep the smaller of `*best_out` and the candidate's squared
        /// distance ((dy')^2 + (dx')^2) + (dz')^2, center-minus-point.
        unsafe fn fold_best(best_out: u32, center: u32, mtx: u32) {
            unsafe {
                let best = rdf(best_out);
                let dx = sub(rdf(center), rdf(mtx.wrapping_add(MTX_X)));
                let dy = sub(rdf(center.wrapping_add(4)), rdf(mtx.wrapping_add(MTX_Y)));
                let dz = sub(rdf(center.wrapping_add(8)), rdf(mtx.wrapping_add(MTX_Z)));
                let probe = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                if probe > best {
                    return;
                }
                wrf(best_out, probe);
            }
        }

        let radius = f32::from_bits(radius_bits);
        if best_out != 0 {
            wrf(best_out, mul(radius, radius));
        }
        let table = rd32(list_head.wrapping_add(SLOTS_AT)).wrapping_add(SLOTS_ENTRIES);
        let mut count = 0u32;
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let mut ped = rd32(table.wrapping_add(i.wrapping_mul(4)));
            let mut consider = ped != 0;
            if consider && rd32(ped.wrapping_add(PED_FLAGS)) & FLAG_SKIP != 0 {
                consider = false;
            }
            if consider && rd8(ped.wrapping_add(PED_DEAD)) != 0 {
                consider = false;
            }
            if consider {
                let kind_obj = rd32(ped.wrapping_add(PED_KIND_OBJ));
                if rd32(kind_obj.wrapping_add(KIND_OFF)) != KIND_WANTED {
                    consider = false;
                }
            }
            let mut gate = false;
            let mut mtx = 0u32;
            if consider {
                mtx = rd32(ped.wrapping_add(PED_MTX));
                let dx = sub(rdf(mtx.wrapping_add(MTX_X)), rdf(center));
                let dy = sub(rdf(mtx.wrapping_add(MTX_Y)), rdf(center.wrapping_add(4)));
                let dz = sub(rdf(mtx.wrapping_add(MTX_Z)), rdf(center.wrapping_add(8)));
                let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                gate = mul(radius, radius) > dist2;
            }
            if consider && gate {
                if ped == excluded {
                    consider = false;
                } else if want_word != NONE
                    && rd16(ped.wrapping_add(PED_TAG)) as i16 as i32 as u32 != want_word
                {
                    consider = false;
                }
            } else if consider {
                consider = false;
            }
            if !consider {
                i += 1;
                continue;
            }
            if filter == NONE {
                if best_out != 0 {
                    fold_best(best_out, center, mtx);
                }
                count = count.wrapping_add(1);
                i += 1;
                continue;
            }
            let probe: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE, u32, ped);
            if probe != 0 {
                let cursor = probe.wrapping_add(8);
                let n1: u32 = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, cursor);
                if n1 != 0 {
                    let n2: u32 = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, cursor);
                    if n2 != ped {
                        let info = rd32(ped.wrapping_add(PED_INFO)).wrapping_add(INFO_TEST_OFF);
                        let t: u32 = lf_checker_rt::callee_thiscall!(
                            CALLEE_TEST, u32, info, TEST_PROBE_ARG, 0
                        );
                        if (t as u8) != 0 {
                            ped = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, cursor);
                        }
                    }
                }
            }
            let info = rd32(ped.wrapping_add(PED_INFO)).wrapping_add(INFO_TEST_OFF);
            let t: u32 =
                lf_checker_rt::callee_thiscall!(CALLEE_TEST, u32, info, filter, 0);
            if (t as u8) != 0 {
                if best_out != 0 {
                    fold_best(best_out, center, rd32(ped.wrapping_add(PED_MTX)));
                }
                count = count.wrapping_add(1);
            }
            i += 1;
        }
        count
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00a04360 {

// original: 0x00A04360 frag_traced_dispatch (proposed)

/// Build a trace record from four floats and dispatch on the trace test.
///
/// Forms `t = a3 * a3 * k` with the global factor (original operand order),
/// packs `t` and the inputs into two frame blocks, and runs the trace
/// tester callee with (`block_b`, const, `block_a`, 0x10, 0xd), where
/// `block_b` is [`a0`, `a1`, `a2`] and `block_a` starts [`t`, out]. The
/// tester reports through `block_a[1]` (an out-slot the original reads
/// back, ignoring the register answer), so the stub writes it. Later words
/// of both blocks are uninitialised holes, so only the first three of
/// `block_b` and the first two of `block_a` are compared. When the slot
/// reads zero, the out-pointer `a4` receives zero and `a4` is returned;
/// otherwise the resolver callee runs on a global object with the slot
/// value, its answer is stored through `a4` and returned.
///
/// Original: 0x00A04360 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00A04360(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const FACTOR: u32 = 0x00FE8AB8;
        const TARGET: u32 = 0x0094B7D0;
        const RESOLVER_OBJ: u32 = 0x01632C60;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let k = (lf_checker_rt::relocated(FACTOR) as *const f32).read_unaligned();
        let f3 = f32::from_bits(a3);
        let t = mul(mul(f3, f3), k);
        let block_b = [a0, a1, a2, 0u32];
        let mut block_a = [t.to_bits(), 0u32, 0, 0];
        lf_checker_rt::callee_cdecl!(1, u32, block_b.as_ptr() as u32,
            lf_checker_rt::relocated(TARGET), block_a.as_ptr() as u32, 0x10u32, 0x0Du32);
        // Volatile: the stub wrote the out-slot through the passed pointer.
        let r = core::ptr::read_volatile(&block_a[1]);
        if r == 0 {
            (a4 as *mut u32).write_unaligned(0);
            return a4;
        }
        let obj = (lf_checker_rt::relocated(RESOLVER_OBJ) as *const u32).read_unaligned();
        let ans = lf_checker_rt::callee_thiscall!(2, u32, obj, r);
        (a4 as *mut u32).write_unaligned(ans);
        ans
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00584b30 {

// original: 0x00584b30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_200, player_schema::LeaderboardInfo, 10>::vf6

/// Look up one leaderboard row id in the fetched key column.
///
/// `needle` is the row id to find. The fetch callee (id 1) is called with
/// this instantiation's leaderboard id and a pointer to an eight-word stack
/// buffer; on success it leaves a signed row count at `+0x0c` and a pointer
/// to the key column (one `u32` per row) at `+0x10`. Only the low byte of the
/// fetch result is significant: zero means failure and yields -1.
///
/// The keys are then scanned in order and the index of the first row equal
/// to `needle` is returned, or -1 when the count is not positive or no row
/// matches. The comparison is a signed `i32` loop bound.
///
/// Original: 0x00584b30 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00584b30(needle: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1a0;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn read_u32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return NOT_FOUND;
        }
        let count = out[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = out[KEYS_WORD];
        let mut i = 0i32;
        while i < count {
            if read_u32(keys.wrapping_add((i as u32).wrapping_mul(4))) == needle {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00d29090 {

// original: 0x00d29090 CPedTargetting::vf10 (symbols)

/// Test whether a ped may target another, tail-calling the decider.
///
/// Bails out with 0 unless the candidate record `arg` has exactly the wanted
/// flag bits (`[arg + 0x28] & 0x3c0 == 0xc0`), differs from the current target
/// at `this + 0x24c`, either has its byte at `+0x210` clear or a state word
/// at `+0xa74` outside {1, 2}, and holds a nonzero link at `+0x224`.
/// Otherwise tail-calls the target decider (intercepted) with the current
/// target's link block and `arg`. Only the low byte of the early-out result
/// is defined.
///
/// Original: 0x00D29090 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d29090(this: u32, arg: u32) -> u32 {
    unsafe {
        const FLAGS_OFF: u32 = 0x28;
        const FLAGS_MASK: u32 = 0x3c0;
        const FLAGS_WANT: u32 = 0xc0;
        const CURRENT_OFF: u32 = 0x24c;
        const FLAG_OFF: u32 = 0x210;
        const STATE_OFF: u32 = 0xa74;
        const LINK_OFF: u32 = 0x224;
        if unsafe { ((arg + FLAGS_OFF) as *const u32).read_unaligned() } & FLAGS_MASK != FLAGS_WANT {
            return 0;
        }
        let current = unsafe { ((this + CURRENT_OFF) as *const u32).read_unaligned() };
        if arg == current {
            return 0;
        }
        if unsafe { ((arg + FLAG_OFF) as *const u8).read() } != 0 {
            let state = unsafe { ((arg + STATE_OFF) as *const u32).read_unaligned() };
            if state == 1 || state == 2 {
                return 0;
            }
        }
        if unsafe { ((arg + LINK_OFF) as *const u32).read_unaligned() } == 0 {
            return 0;
        }
        let block = unsafe { ((current + LINK_OFF) as *const u32).read_unaligned() };
        lf_checker_rt::callee_thiscall!(1, u32, block, arg)
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00add1c0 {

// original: 0x00add1c0 CRenderPhaseScript2d::vf0

/// Scalar deleting destructor of the script-2d render phase.
///
/// `this` is the object; `flags` is the standard destructor flag word.
/// Runs the (intercepted) destructor, then frees the object with the
/// scalar `operator delete` callee when bit 0 of `flags` is set.
/// Returns `this` unchanged in all cases.
///
/// Edge cases: any flag value with bit 0 clear skips the delete call;
/// only bit 0 is tested, all other bits are ignored.
///
/// Original: thiscall, one stack word, callee id 1 is the destructor
/// (thiscall, no stack arguments), callee id 2 is `operator delete`
/// (cdecl, one argument).
lf_checker_rt::export!(thiscall, rw_00add1c0(this: u32, flags: u32) -> u32 {
    const DELETE_FLAG: u32 = 1;
    lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & DELETE_FLAG != 0 {
        lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00e67b20 {

// original: 0x00e67b20 asset_slot_supergt

/// Register the per-asset slot for "supergt" with the asset registry.
///
/// Calls the shared slot-registration routine (thiscall: the slot
/// pointer in ECX, the name pointer pushed) with this wrapper's two
/// constants: `SLOT_PTR`, the slot record in the data section, and
/// `NAME_PTR`, the asset name string in the read-only section. The
/// routine stores the name into the slot, links the slot into the
/// registry list, and returns the slot pointer, which this wrapper
/// returns unchanged.
///
/// Takes no arguments and reads no caller state (the incoming ECX
/// is overwritten); stack effect is zero (plain `ret`: the callee
/// pops its one word). Original: cdecl/0, one outgoing call.
lf_checker_rt::export!(cdecl, rw_00e67b20() -> u32 {
    const NAME_PTR: u32 = 0x00E9E1C4;
    const SLOT_PTR: u32 = 0x012F9FC8;
    const SLOT_REGISTER: u32 = 1;
    lf_checker_rt::callee_thiscall!(
        SLOT_REGISTER,
        u32,
        lf_checker_rt::relocated(SLOT_PTR),
        lf_checker_rt::relocated(NAME_PTR)
    )
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_005493d0 {

// original: 0x005493D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_10, player_schema::LeaderboardInfo, 10>::vf2

/// Check this board's leaderboard id against an expected value and, on a match, publish the
/// board's info pointer.
///
/// Calls the second virtual slot of the object in `this` (a vtable call taking no arguments, `this`
/// still in ecx) to get the board's leaderboard id. If it differs from `expected`, or `out` is null,
/// returns 0. Otherwise writes the board's info-table address (`INFO_PTR`, a file VA in .rdata,
/// relocated at load) to `*out` and returns `out`.
///
/// Edge cases: id mismatch and null `out` both yield 0; the stored value is an image address, not a
/// small integer.
///
/// Original: 0x005493D0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_005493d0(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4;
        const INFO_PTR: u32 = 0x00FD9244;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let vtable = rd32(this);
        let target = rd32(vtable.wrapping_add(VTABLE_SLOT));
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if get_id(this) != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(INFO_PTR));
        out
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00598690 {

// original: 0x00598690 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_2, player_schema::LeaderboardInfo, 10>::vf7

/// Read one entry of this leaderboard's id table by index.
///
/// `this` (ECX, thiscall) is the leaderboard-info object; it is never read:
/// the board is identified solely by `BOARD_ID`. `index` counts in 4-byte
/// elements. The schema callee (fastcall: ECX = board id, EDX = out-struct)
/// is asked for the table; it reports success in AL and fills `ARRAY_OFF`
/// (pointer to the id array).
///
/// On success the element at `index` is returned. `NOT_FOUND` (-1) is
/// returned only when the callee reports failure; the index itself is not
/// bounds-checked, matching the original.
///
/// Original: 0x00598690 (thiscall, one stack word; ECX ignored).
lf_checker_rt::export!(thiscall, rw_00598690(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x155;
        const CALLEE_SCHEMA: u32 = 1;
        const ARRAY_OFF: usize = 0x10;
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut schema = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_SCHEMA,
            u32,
            BOARD_ID,
            schema.as_mut_ptr() as u32
        );
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let items = schema[ARRAY_OFF / 4] as *const u32;
        *items.add(index as usize)
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00560320 {

// original: 0x00560320 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_67, player_schema::LeaderboardInfo, 10>::vf2

/// Verifies a leaderboard key through the secondary info object, then
/// publishes this board's vtable pointer to the caller's out slot.
/// 
/// `this` points to the board info object whose first word is a pointer to
/// its secondary vtable; slot 1 (at `+0x04`) is called with `this` and
/// returns the live key. When that key equals `key` and `out` is non-null,
/// the board vtable address (`VTABLE`, relocated) is stored to `*out`
/// and `out` is returned; otherwise the result is null (a null `out` or
/// a key mismatch both yield 0, and a mismatch stores nothing).
/// 
/// Original: 0x00560320 (thiscall: this in ECX, two stack words, callee
/// pops 8). Episodic race board 67 of the template family; only the
/// vtable address differs between instantiations.
lf_checker_rt::export!(thiscall, rw_00560320(this: u32, out: u32, key: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 0x04;
        const VTABLE: u32 = 0x00fdb554;
        let vtable = (this as *const u32).read_unaligned();
        let live_key: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vtable + VTABLE_SLOT) as *const u32).read_unaligned() as usize);
        if live_key(this) != key {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        out
    }

});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_005232c0 {

// original: 0x005232c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race46NoHolds, player_schema::LeaderboardInfo, 10>::vf6

/// Find a leaderboard key in the key list.
///
/// `want` is the key to find. The lookup callee (id 1, leaderboard id
/// 0x1f) fills a five-word query block: key count at word 3, key-list
/// pointer at word 4; only its low result byte matters. Returns the key's
/// index, or all-bits-set when the lookup fails, the count is zero or
/// negative, or the key is absent.
/// Edge cases: the count is signed.
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_005232c0(want: u32) -> u32 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 1;
        const LEADERBOARD_ID: u32 = 0x1f;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;

        let mut block = [0u32; 5];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE,
            u32,
            LEADERBOARD_ID,
            block.as_mut_ptr() as u32
        );
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let n = block[3] as i32;
        if n <= 0 {
            return NOT_FOUND;
        }
        let keys = block[4];
        let mut i = 0i32;
        while i < n {
            let at = (i as u32).wrapping_mul(4);
            let here = (keys.wrapping_add(at) as *const u32).read_unaligned();
            if here == want {
                return i as u32;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_005809f0 {

// original: 0x005809f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_185, player_schema::LeaderboardInfo, 10>::vf7

/// Leaderboard column lookup for one episodic-race leaderboard (id 0x191).
///
/// Calls the leaderboard query with this leaderboard's id and a six-word
/// scratch descriptor; the query fills words 1..=5 with `countA`, `arrA`,
/// `countB`, `arrB`, `arrC` (byte offsets +4..+20) and answers nonzero in
/// its low byte on success. Only the low byte of the answer is significant.
/// Returns `arrB[index]` with 32-bit wraparound on the scaled index.
/// Original: stdcall, one stack word, returns -1 when the query fails.
lf_checker_rt::export!(stdcall, rw_005809f0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x191;
        const DESC_ARR_B: usize = 4;
        const CALLEE_QUERY: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut desc = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(CALLEE_QUERY, u32, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let arr = desc[DESC_ARR_B];
        rd32(arr.wrapping_add(index.wrapping_mul(4)))
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_0050f0f0 {

// original: 0x0050f0f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_15,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the ranked episodic race 15 leaderboard update object.
///
/// Runs the shared base-class constructor, then installs this template
/// instantiation's vtable pair and initializes the embedded leaderboard-info
/// sub-object: an invalid-slot marker, zeroed counters and a set flag byte.
/// Returns the object pointer.
lf_checker_rt::export!(thiscall, rw_0050f0f0(this: u32) -> u32 {
    const UPDATE_VTABLE: u32 = 0x00FDA150;
    const INFO_VTABLE: u32 = 0x00FD8C5C;
    const INFO_OFF: usize = 0x4A0;
    const SLOT_OFF: usize = 0x4A4;
    const FLAG_OFF: usize = 0x5A4;
    const ZERO_WORDS: [usize; 5] = [0x4A8, 0x4AC, 0x4B0, 0x4B4, 0x4B8];
    lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe {
        let base = this as *mut u8;
        (base as *mut u32).write(lf_checker_rt::relocated(UPDATE_VTABLE));
        (base.add(INFO_OFF) as *mut u32).write(lf_checker_rt::relocated(INFO_VTABLE));
        *base.add(FLAG_OFF) |= 1;
        (base.add(SLOT_OFF) as *mut u32).write(u32::MAX);
        for off in ZERO_WORDS {
            (base.add(off) as *mut u32).write(0);
        }
    }
    this
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_009e2c80 {

// original: 0x009e2c80 audio_slot_add_and_dispatch
/// Add the argument into the big tracker slot, then tail-dispatch
/// to the slot-store routine. (thiscall/1, tail jump)
export!(thiscall, rw_009e2c80(this: *mut u8, a: u32) -> u32 {
    unsafe {
        let slot = *((this.add(0xB88)) as *const u32);
        callee_thiscall!(1, u32, this as u32, a.wrapping_add(slot))
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_0057c110 {

// original: 0x0057C110 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_169, player_schema::LeaderboardInfo, 10>::vf2
/// Publish this leaderboard's column id when the caller expects it.
///
/// Reads the object's virtual table pointer from `this` and calls the slot at
/// `+4` (`VTABLE_SLOT`, the id query) with `this`. When the answer equals
/// `want` and `out` is non-null, writes the column descriptor address (file
/// VA `0xfdedbc`, relocated at load) to `*out` and returns `out`; otherwise
/// returns 0 (a null `out` or a mismatch both give 0 without writing).
///
/// The original embeds the address as an immediate that the loader relocates
/// (proven: the original writes exactly file-value plus the load delta), so
/// the rewrite derives it with `relocated`, never hard-coded.
///
/// Original: 0x0057C110 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0057C110(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4;
        const COLUMN_FILE_VA: u32 = 0xfdedbc;
        let vtab = (this as *const u32).read_unaligned();
        let slot = (vtab.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        if query(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(COLUMN_FILE_VA));
        out
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}


#[allow(dead_code)]
mod k_00d4dd90 {

// original: 0x00D4DD90 CTaskSimpleDuck::vf0

/// Scalar deleting destructor of CTaskSimpleDuck.
///
/// Runs the class destructor on `this` (intercepted callee 1, thiscall with
/// no stack arguments), then, when the low bit of `flags` is set, frees the
/// object through the game allocator whose pointer lives in a global slot
/// (intercepted callee 2, thiscall taking the object pointer). Returns `this`
/// in both cases. The destructor's own return value is discarded, and the
/// allocator pointer is read only on the freeing path.
///
/// Original: 0x00D4DD90 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d4dd90(this: u32, flags: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167E2A0;
        const DTOR: u32 = 1;
        const DELETE: u32 = 2;
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flags & 1 != 0 {
            let alloc = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
            lf_checker_rt::callee_thiscall!(DELETE, u32, alloc, this);
        }
        this
    }
});

use lf_checker_rt::{callee_addr, callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated, tls_slot, xbase, xmm_word};
}
