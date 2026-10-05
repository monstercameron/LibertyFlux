// original: 0x00979F60 COLLISIONS (symbols)
//
// Resolve a collision event's voice and submit it. After the audio guards and
// an event-kind gate, looks up the collision record, builds two parameter
// blocks, resolves the sounding object through an index table, scans the
// object's hash list for the cached event hash, and submits the found voice
// with its level, class and mode words, storing the submission handles back
// into the table record.
//
// Descriptor words the original reads from uninitialised frame (the table
// pointer and flag bytes) are planted by the contract's callee stub, since
// the checker's fill cannot reproduce the game's stack history; the scan
// bound is an UNSIGNED byte comparison. The x87 spill (a callee ST0 float
// stored to the frame) is carried as bits. Original: 0x00979F60 (thiscall,
// four stack args, void).

use lf_checker_rt::{callee_cdecl, callee_thiscall, relocated};

const MASTER_FLAG: u32 = 0x11F7060;
const GEN_A: u32 = 0x12088B4;
const GEN_B: u32 = 0x0F1C040;
const MODE_WORD: u32 = 0x1037720;
const MODE_SKIP: u32 = 0x12;

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
unsafe fn g32(va: u32) -> u32 {
    unsafe { (relocated(va) as *const u32).read_unaligned() }
}

const COLL_TABLE: u32 = 0x115DC18;
const HASH_NAME_1: u32 = 0xE8BC6C;
const HASH_NAME_2: u32 = 0xE8BC78;
const HASH_CACHE_FLAG: u32 = 0x12312A8;
const HASH_CACHE_VAL: u32 = 0x12312A4;
const TUNE_WORD: u32 = 0x12202B4;
const KIND_MASK: u8 = 0x0C;
const KIND_WANT: u8 = 4;
const REC_TAG_1: u8 = 0x0E;
const REC_TAG_2: u8 = 0x0D;
const MODE_CONST: u32 = 0x1E;

const C_LOOKUP: u32 = 1;
const C_BLK_INIT_A: u32 = 2;
const C_BLK_INIT_B: u32 = 3;
const C_BLK_APPLY_A: u32 = 4;
const C_DESC_INIT: u32 = 5;
const C_INDEX: u32 = 6;
const C_LOOKUP2: u32 = 7;
const C_BLK_APPLY_B: u32 = 8;
const C_HASH1: u32 = 9;
const C_SUB_INIT: u32 = 10;
const C_CLASS_A: u32 = 11;
const C_CLASS_B: u32 = 12;
const C_LEVEL: u32 = 13;
const C_SUBMIT_A: u32 = 14;
const C_HASH2: u32 = 15;
const C_SUBMIT_B: u32 = 16;
const C_HANDLE: u32 = 17;
const C_COMMIT: u32 = 18;

#[inline(always)]
unsafe fn wglobal(va: u32, v: u32) {
    unsafe { (relocated(va) as *mut u32).write_unaligned(v) }
}

lf_checker_rt::export!(thiscall, rw_00979F60(obj: u32, ev: u32, level_bits: u32, cls: u32, mode: u32) -> u32 {
    unsafe {
        if g32(MASTER_FLAG) == 1 {
            return 0;
        }
        if g32(GEN_A) != g32(GEN_B) {
            return 0;
        }
        if g32(MODE_WORD) == MODE_SKIP {
            return 0;
        }
        if rd8(ev.wrapping_add(5)) & KIND_MASK != KIND_WANT {
            return 0;
        }
        let rec = callee_thiscall!(C_LOOKUP, u32, relocated(COLL_TABLE), rd32(ev.wrapping_add(0x0A)));
        let mut blk_a = [0u32; 9];
        callee_thiscall!(C_BLK_INIT_A, u32, blk_a.as_mut_ptr() as u32);
        let mut blk_b = [0u32; 9];
        callee_thiscall!(C_BLK_INIT_B, u32, blk_b.as_mut_ptr() as u32);
        if rec == 0 {
            return 0;
        }
        if rd8(rec) != REC_TAG_1 {
            return 0;
        }
        let applied = callee_cdecl!(C_BLK_APPLY_A, u32, rec, blk_a.as_ptr() as u32);
        // Descriptor bytes; the stub plants the table pointer and flag byte.
        let mut desc = [0u8; 12];
        callee_cdecl!(
            C_DESC_INIT, u32, applied,
            desc.as_mut_ptr() as u32,
            desc.as_mut_ptr().add(1) as u32
        );
        let flag = desc[0];
        if flag == 0 {
            return 0;
        }
        let table = rd32(desc.as_ptr().add(1) as u32);
        let index = callee_cdecl!(C_INDEX, u32, 0, (flag as u32).wrapping_sub(1));
        let entry = rd32(table.wrapping_add(index.wrapping_mul(8)));
        let rec2 = callee_thiscall!(C_LOOKUP2, u32, relocated(COLL_TABLE), entry);
        if rec2 == 0 {
            return 0;
        }
        if rd8(rec2) != REC_TAG_2 {
            return 0;
        }
        let list = callee_cdecl!(C_BLK_APPLY_B, u32, rec2, blk_b.as_ptr() as u32);
        let cached = g32(HASH_CACHE_FLAG);
        let hash = if cached & 1 != 0 {
            g32(HASH_CACHE_VAL)
        } else {
            wglobal(HASH_CACHE_FLAG, cached | 1);
            let h = callee_cdecl!(C_HASH1, u32, relocated(HASH_NAME_1), 0);
            wglobal(HASH_CACHE_VAL, h);
            h
        };
        // Hash-list scan; the bound is an UNSIGNED byte comparison.
        let count = rd8(list) as u32;
        if count == 0 {
            return 0;
        }
        let mut slot = 0u32;
        let mut found = false;
        let mut i = 0u32;
        while i < count {
            if rd32(list.wrapping_add(5).wrapping_add(i.wrapping_mul(8))) == hash {
                slot = i;
                found = true;
                break;
            }
            i += 1;
        }
        if !found {
            return 0;
        }
        let voice = rd32(list.wrapping_add(1).wrapping_add(slot.wrapping_mul(8)));
        if voice == 0xFFFF_FFFF {
            return 0;
        }
        let mut sub = [0u32; 13];
        callee_thiscall!(C_SUB_INIT, u32, sub.as_mut_ptr() as u32);
        sub[8] = callee_thiscall!(C_CLASS_A, u32, cls);
        sub[3] = callee_thiscall!(C_CLASS_B, u32, cls);
        sub[7] = g32(TUNE_WORD);
        sub[10] = mode;
        sub[11] = MODE_CONST;
        sub[0] = callee_cdecl!(C_LEVEL, u32, level_bits);
        callee_thiscall!(
            C_SUBMIT_A, u32, obj,
            voice,
            desc.as_ptr().add(1) as u32,
            sub.as_ptr() as u32,
            0xFFFF_FFFFu32, 0, 0
        );
        if table == 0 {
            return 0;
        }
        let mut tail = [0u32; 3];
        tail[1] = 0xFFFF_FFFF;
        tail[2] = 0x0E;
        let g1 = callee_thiscall!(C_CLASS_A, u32, cls);
        let picked = if g1 != 0 {
            cls
        } else if callee_thiscall!(C_CLASS_B, u32, cls) != 0 {
            cls
        } else {
            0
        };
        tail[0] = callee_cdecl!(C_HASH2, u32, relocated(HASH_NAME_2), 0);
        let out1 = callee_cdecl!(
            C_SUBMIT_B, u32, voice, 1, 0, 1,
            sub.as_ptr() as u32,
            tail.as_ptr() as u32,
            picked, 0xFFFF_FFFFu32
        );
        let out2 = callee_cdecl!(C_HANDLE, u32, out1);
        wr32(table.wrapping_add(0xA4), out1);
        wr32(table.wrapping_add(0xA8), out2);
        wr32(table.wrapping_add(0xAC), 0);
        callee_thiscall!(C_COMMIT, u32, table, 0, 0, 0);
        0
    }
});
