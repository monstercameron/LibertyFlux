// original: 0x00E4EB50 UIGalleryFileViewer::vf123
//
// Specification (what the rewrite does, in the terms of the checker contract):
// The method takes its object (this) and a resource-set index range from a global
// object, then runs three loops over the loaded entries and builds the child object
// when it is missing:
//   1. Loop 1 walks the signed index range from the range value below 3. Each
//      index asks an availability query; when it answers nonzero, a resource is
//      opened, its records are copied into a buffer through the reserve struct, and
//      the resource is closed. An open that reports no record ends the method early.
//   2. Loop 2 walks the loaded entries. Each entry is matched by a format query; a
//      match runs the entry's name resolution (block A when the flag at +0x378 is set,
//      otherwise the deep name path) and appends the entry's record to the append struct.
//   3. The record call, the child construction (when the child slot is empty), and the
//      row loop: one row per appended record. The row index is taken modulo the divisor
//      word at +0x30c; a zero remainder builds object B and object E from the child's
//      name, a nonzero remainder builds object E from the carried object B.
//   Then the status query on the child; a nonzero status runs the six tail calls.
//   The flag at +0x218 of the child is set, the buffers are freed as the append and
//   reserve structs say, and the answer is the last free's answer, or the child pointer
//   when no free runs.
// Narrowings (see results.json). Every branch the original completes on is written and
// proven. The original FAULTS on these, so the rewrite keeps a deliberate fault there and
// the inputs keep them out: divisor zero (integer division by zero); a remainder-nonzero
// row with no carried object; a null name buffer in the deep path (the terminator store
// writes through it); a null child (its first table call reads through it); a null object
// B or row record E (read or written through); a name at or above the length limit (the
// buffer-overrun fast-fail). The range is steered to -2..4 and 2147483647 (0x80000000
// would make the loop run about 2^31 times); the row count to 0..3; the divisor to
// 1, -1, 3 and -2. Stack-cookie helper runs natively on the original side only.
// Pointer arguments to frame addresses are not compared (call_skip).
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Global object pointer the opening call is made on (file VA).
const GLOBAL_OBJ: u32 = 0x01BB5624;
/// Constant pushed as first argument to the record call (file VA).
const RECORD_FMT: u32 = 0x00E4E6E0;
/// Size argument of the record call.
const RECORD_SIZE: u32 = 0x218;
/// Offset of the child (scrolling-menu) pointer in `this`.
const CHILD_PTR: u32 = 0x32c;
/// Table slot of the tail status call in the child object.
const STATUS_SLOT: u32 = 0x1d4;
/// Offset of the flag byte set in the child object on the status-0 path.
const FLAG_OFF: u32 = 0x218;
/// Opening-call answers below this (signed) enter loop 1.
const EPISODE_LIMIT: i32 = 3;
/// Child-table slots used on the nonzero-status tail path.
const TAIL_A_SLOT: u32 = 0x120;
const TAIL_B_SLOT: u32 = 0x1e0;
const TAIL_C_SLOT: u32 = 0x1ac;
/// Grandchild-table slots used on the nonzero-status tail path.
const GRAND_A_SLOT: u32 = 0x1ac;
const GRAND_B_SLOT: u32 = 0x18;
/// Loop-1 string constants (file VAs).
const RES_NAME: u32 = 0x00F19AD0;
const RES_FMT: u32 = 0x01168DD8;
/// Loop-2 entry-match format (file VA) and entry layout.
const MATCH_FMT: u32 = 0x00F19AE0;
const ENTRY_OFF: u32 = 0x14;
const ENTRY_STRIDE: u32 = 0x218;
/// Loop-2 block-A constants.
const WMV_EXT: u32 = 0x00F19AE8;
const MALLOC_A: u32 = 0xE0;
const APPEND_ARG: u32 = 0x10;
const FMT_SIZE: u32 = 0x200;
/// Default name string (file VA; relocated like any address operand).
const FMT_DEFAULT: u32 = 0xFC9C85;
const NAME_LIMIT: u32 = 0x26;
const GLOBAL_LIMIT: u32 = 0x0106C2C0;
/// Offsets in `this` and the block-A object.
const FLAG378: u32 = 0x378;
const OBJ_A0: u32 = 0xA0;
const OBJ_A8: u32 = 0xA8;
/// Global holding the float-compare limit as a signed int (file VA).
const GLOBAL_FLOATLIM: u32 = 0x0106C284;
/// Loop-2 deep-name constants.
const NAME_BUF: u32 = 0x26;
const THIS_300: u32 = 0x300;
const THIS_35C: u32 = 0x35C;
const THIS_370: u32 = 0x370;
const THIS_374: u32 = 0x374;
const NAME_DEFAULT: u32 = 0xFC9C85;
const CMP_STR: u32 = 0x00F194BE;
/// Section-5 (child construction) constants.
const CHILD_SIZE: u32 = 0x224;
const CHILD_FMT: u32 = 0x00F19AF0;
const CHILD_1FC: u32 = 0x1FC;
const CHILD_1EC: u32 = 0x1EC;
const SUB_1F4: u32 = 0x1F4;
const CHILD_SCALE: u32 = 0x21C;
const SCALE_BITS: u32 = 0x41700000; // 15.0f
const THIS_324: u32 = 0x324;
const THIS_354: u32 = 0x354;
const THIS_VT48: u32 = 0x48;
const THIS_VT4C: u32 = 0x4C;
const CHILD_VT114: u32 = 0x114;
const CHILD_VT7C: u32 = 0x7C;
const SUB_VT120: u32 = 0x120;
/// Per-item constants of the four child-item initialisations.
const ITEM_ARGS: [u32; 4] = [4, 0x10, 8, 2];
/// Loop-3 constants: divisor word, object words read by the row, row formats and sizes.
const THIS_30C: u32 = 0x30C;
const THIS_31C: u32 = 0x31C;
const THIS_320: u32 = 0x320;
const THIS_350: u32 = 0x350;
const THIS_379: u32 = 0x379;
const ROW_B_SIZE: u32 = 0x1F8;
const ROW_E_SIZE: u32 = 0x328;
const ROW_E_324: u32 = 0x324;
const ROW_E_1E8: u32 = 0x1E8;
const ROW_FMT_B1: u32 = 0x00F19B08;
const ROW_FMT_B2: u32 = 0x00F19B20;
const ROW_FMT_B3: u32 = 0x00F19B30;
const ROW_FMT_B4: u32 = 0x00F19B34;
const ROW_FMT_B5: u32 = 0x00F19B44;
/// Loop-1 record call format (file VA).
const LOOP1_FMT: u32 = 0x00E4E710;
/// Record size in dwords; the copy loop moves exactly this many.
const RECORD_WORDS: usize = 0x86;
/// Word offset within the record of the four dwords the copy loop lifts.
const LIFT_OFF: usize = 0x80;
/// Byte offset within the assembled output of the copied string.
const STR_OFF: usize = 0x14;
/// Slots of the resource object used in loop 1.
const RES_OPEN_SLOT: u32 = 0x58;
const RES_NEXT_SLOT: u32 = 0x5c;
const RES_CLOSE_SLOT: u32 = 0x60;
/// Handle value meaning "no record" (takes the early-exit path).
const NO_RECORD: u32 = 0xFFFF_FFFF;

/// One iteration of loop 1 with the resource available: open the indexed
/// resource, copy one assembled record (four lifted dwords, the index, and
/// a NUL-terminated string) to a fresh buffer, and close it. Returns
/// `Some(ret)` when the open call reports no record: the function then
/// frees the entry base and returns straight from the tail.
unsafe fn loop1_body(idx: u32, reserve_mirror: &mut [u32; 12]) -> Option<u32> {
    unsafe {
        let _: u32 = callee_cdecl!(9, u32, idx);
        let _: u32 = callee_cdecl!(10, u32, relocated(RES_NAME), 0u32);
        let res: u32 = callee_cdecl!(11, u32, relocated(RES_FMT), 1u32);
        // The original assembles the record in a frame buffer the open call
        // fills; the stub fills nothing, so both sides read zeros (the
        // contract sets stack_fill 0; this buffer is zero-initialised).
        let mut rec = [0u32; RECORD_WORDS];
        let rtable = (res as *const u32).read_unaligned();
        let open: extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
            ((rtable + RES_OPEN_SLOT) as *const u32).read_unaligned() as usize,
        );
        let handle: u32 = open(res, relocated(RES_FMT), rec.as_mut_ptr() as u32);
        reserve_mirror[3] = handle;
        if handle == NO_RECORD {
            // Early exit: jumps to the second tail free's branch, whose
            // flags come from the count high word of the reserve struct, not the other word.
            // With a nonzero high word the base is freed and the free's
            // answer is returned; otherwise the handle itself returns.
            if (reserve_mirror[2] >> 16) != 0 {
                let r: u32 = callee_cdecl!(16, u32, reserve_mirror[1]);
                return Some(r);
            }
            return Some(handle);
        }
        let next: extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
            ((rtable + RES_NEXT_SLOT) as *const u32).read_unaligned() as usize,
        );
        // Record loop: assemble, reserve, copy, ask for more; repeat while the answer's low
        // byte is nonzero. The stub fills nothing, so every lap reads the same bytes.
        loop {
            let mut out = [0u32; RECORD_WORDS];
            out[0] = rec[LIFT_OFF + 2];
            out[1] = rec[LIFT_OFF + 3];
            out[2] = rec[LIFT_OFF];
            out[3] = rec[LIFT_OFF + 1];
            out[4] = idx;
            let recb = rec.as_ptr() as *const u8;
            let outb = out.as_mut_ptr() as *mut u8;
            let mut i = 0usize;
            loop {
                let b = recb.add(i).read();
                outb.add(STR_OFF + i).write(b);
                if b == 0 {
                    break;
                }
                i += 1;
            }
            // The reserve call takes as its receiver the reserve struct (base, count, ...)
            // laid out in the frame: [this, base, count,
            // handle, 0, ...].
            let dest: u32 = callee_thiscall!(13, u32, reserve_mirror.as_mut_ptr().add(1) as u32, 0x10u32);
            (dest as *mut u32).copy_from_nonoverlapping(out.as_ptr(), RECORD_WORDS);
            let more: u32 = next(res, handle, rec.as_mut_ptr() as u32);
            if more as u8 == 0 {
                break;
            }
        }
        let close: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((rtable + RES_CLOSE_SLOT) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = close(res, handle);
        // Record call takes (base, count-low): zeros before stage-7
        // writes, stub-filled after.
        let _: u32 = callee_cdecl!(
            1,
            u32,
            reserve_mirror[1],
            reserve_mirror[2] & 0xFFFF,
            RECORD_SIZE,
            relocated(LOOP1_FMT)
        );
        None
    }
}

/// The method body. The specification at the top of the file says what it does; the
/// shipped export calls this once, with the object pointer only.
unsafe fn stage_body(this: u32) -> u32 {
    unsafe {
        let host = global::<u32>(GLOBAL_OBJ).read();
        let range: u32 = callee_thiscall!(0, u32, host);
        // Reserve-struct mirror, persistent across loop-1 iterations like
        // the frame: [this, base, count, handle, 0, ...]. The id13 stub
        // fills base/count through it from stage 7 on.
        let mut reserve_mirror = [0u32; 12];
        reserve_mirror[0] = this;
        // Append struct (base, count | capacity << 16) written by callee 39 through its receiver.
        let mut append_struct = [0u32; 2];
        // Loop-2 frame word (the original's local at offset 0x1c): zero at entry, written by the deep path
        // (the name length) and by block A (the object's float), read by the block-A float tail.
        let mut frame_word: u32 = 0;
        // Loop 1 (the range is a SIGNED count; the loop runs while it is below 3).
        // Stage 4 steers -2..=4 (0-5 iterations).
        let mut idx = range as i32;
        // loop2_bound after loop 1 = count low word of the last entered iteration
        // (0 when loop 1 never entered): the loop-2 bound.
        let mut loop2_bound = 0u32;
        while idx < EPISODE_LIMIT {
            let avail: u32 = callee_thiscall!(8, u32, host, idx as u32);
            if avail as u8 != 0 {
                // Only the low byte of the answer is tested.
                if let Some(ret) = loop1_body(idx as u32, &mut reserve_mirror) {
                    return ret;
                }
                loop2_bound = reserve_mirror[2] & 0xFFFF;
            }
            idx += 1;
        }
        // Loop 2 over the loaded entries (signed comparison on the low word of the count).
        if (loop2_bound as i32) > 0 {
            let base = reserve_mirror[1];
            let mut k = 0u32;
            while (k as i32) < loop2_bound as i32 {
                let entry = base.wrapping_add(ENTRY_OFF).wrapping_add(k.wrapping_mul(ENTRY_STRIDE));
                let m: u32 = callee_cdecl!(17, u32, entry, relocated(MATCH_FMT));
                if m != 0 {
                    loop2_body(this, entry, &mut append_struct, &mut frame_word);
                }
                k += 1;
            }
        }
        // Record call always runs.
        // Push order is (fmt, size, 0, 0), so the callee sees (0, 0, size, fmt).
        let _: u32 = callee_cdecl!(1, u32, append_struct[0], append_struct[1] & 0xFFFF, RECORD_SIZE, relocated(RECORD_FMT));
        let child_slot = (this + CHILD_PTR) as *const u32;
        let mut child = child_slot.read_unaligned();
        if child == 0 {
            child = construct_child(this, append_struct[1] & 0xFFFF);
            ((this + CHILD_PTR) as *mut u32).write_unaligned(child);
        }
        // Loop 3: one row per appended record (count = low word of the append struct).
        let row_count = (append_struct[1] & 0xFFFF) as i32;
        let mut k: i32 = 0;
        let mut ob_carry: u32 = 0;
        while k < row_count {
            // Remainder of the row index by the divisor word (idiv). A zero divisor makes the
            // original fault: it is kept out of the inputs and listed as narrowed.
            let divisor = ((this + THIS_30C) as *const i32).read_unaligned();
            let rem = k.wrapping_rem(divisor);
            let row = append_struct[0].wrapping_add(ENTRY_OFF).wrapping_add((k as u32).wrapping_mul(ENTRY_STRIDE));
            if rem == 0 {
                ob_carry = row_rem_zero(this, row, k as u32);
            } else {
                if ob_carry == 0 {
                    panic!("original faults here: remainder-nonzero row with no carried object (narrowed)");
                }
                row_rem_nonzero(this, row, k as u32, ob_carry);
            }
            k += 1;
        }
        let table = (child as *const u32).read_unaligned();
        let status: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((table + STATUS_SLOT) as *const u32).read_unaligned() as usize);
        let st = status(child);
        if st != 0 {
            // Nonzero-status tail: six further indirect calls.
            let table = (child as *const u32).read_unaligned();
            let tail_a: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                ((table + TAIL_A_SLOT) as *const u32).read_unaligned() as usize,
            );
            let _: u32 = tail_a(child, 1);
            let tail_b: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                ((table + TAIL_B_SLOT) as *const u32).read_unaligned() as usize,
            );
            let g1: u32 = tail_b(child, 0);
            let g1t = (g1 as *const u32).read_unaligned();
            let grand_a: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((g1t + GRAND_A_SLOT) as *const u32).read_unaligned() as usize,
            );
            let _: u32 = grand_a(g1);
            let g2: u32 = tail_b(child, 0);
            let g2t = (g2 as *const u32).read_unaligned();
            let grand_b: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                ((g2t + GRAND_B_SLOT) as *const u32).read_unaligned() as usize,
            );
            let _: u32 = grand_b(g2, 1);
            let tail_c: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((table + TAIL_C_SLOT) as *const u32).read_unaligned() as usize,
            );
            let _: u32 = tail_c(child);
        }
        // Unconditional: both tail paths store the flag (the status branch
        // only skips the six calls above).
        ((child + FLAG_OFF) as *mut u8).write(1u8);
        // Tail. The original's answer register holds the child pointer (set by the flag store)
        // unless a free runs: the first free (append buffer, when its capacity word is nonzero)
        // and then the second free (reserve buffer, when its count high word is nonzero) leave
        // their own answers in it, so the last free that runs decides the return value.
        let mut answer = child;
        if (append_struct[1] >> 16) != 0 {
            answer = callee_cdecl!(16, u32, append_struct[0]);
        }
        if (reserve_mirror[2] >> 16) != 0 {
            answer = callee_cdecl!(16, u32, reserve_mirror[1]);
        }
        answer
    }
}

/// Byte length of a NUL-terminated string, as the original's strlen loops
/// compute it (counting up from the pointer to the terminator, exclusive).
unsafe fn strlen(p: u32) -> u32 {
    unsafe {
        let mut q = p;
        loop {
            if (q as *const u8).read() == 0 {
                break;
            }
            q = q.wrapping_add(1);
        }
        q.wrapping_sub(p)
    }
}

/// Byte-wise strcmp returning (equal, less): 0 when equal, otherwise
/// +1/-1 by the first differing byte, as the original's unrolled loop
/// computes it (the sign of the difference, 0 when equal).
unsafe fn strcmp(a: u32, b: u32) -> i32 {
    unsafe {
        let mut x = a;
        let mut y = b;
        loop {
            let d = (x as *const u8).read();
            let e = (y as *const u8).read();
            if d != e {
                return if d < e { -1 } else { 1 };
            }
            if d == 0 {
                return 0;
            }
            let d1 = ((x + 1) as *const u8).read();
            let e1 = ((y + 1) as *const u8).read();
            if d1 != e1 {
                return if d1 < e1 { -1 } else { 1 };
            }
            if d1 == 0 {
                return 0;
            }
            x += 2;
            y += 2;
        }
    }
}

/// One loop-2 iteration whose entry matched: with the +0x378 flag set,
/// build the entry's name through block A and leave via the unsigned
/// limit check or the float check; otherwise, or when the float check
/// falls through, resolve the entry's short name (format it, optionally
/// attach it, append the base name, and either release it when it names
/// the default or copy the entry's record through the reserve call).
unsafe fn loop2_body(this: u32, entry: u32, append_struct: &mut [u32; 2], frame_word: &mut u32) {
    unsafe {
        if ((this + FLAG378) as *const u8).read() == 0 {
            deep_entry(this, entry, append_struct, frame_word);
            return;
        }
        let a: u32 = callee_cdecl!(18, u32, MALLOC_A);
        if a == 0 {
            block_a_float_tail(this, entry, append_struct, frame_word);
            return;
        }
        let b: u32 = callee_thiscall!(19, u32, a);
        if b == 0 {
            block_a_float_tail(this, entry, append_struct, frame_word);
            return;
        }
        let ebx = b;
        // Post-construction init (three zero args); return ignored.
        let _: u32 = callee_thiscall!(26, u32, ebx, 0u32, 0u32, 0u32);
        // Name assembly struct [ptr, len], filled by the append stub.
        let mut st = [0u32; 2];
        let stp = st.as_mut_ptr() as u32;
        let l1 = strlen(relocated(RES_FMT));
        let _: u32 = callee_thiscall!(20, u32, stp, relocated(RES_FMT), l1);
        let l2 = strlen(entry);
        let _: u32 = callee_thiscall!(20, u32, stp, entry, l2);
        // Word-sized `len -= 5`; a zero result skips the terminator store.
        let nl = (st[1] as u16).wrapping_sub(5);
        st[1] = (st[1] & 0xFFFF_0000) | nl as u32;
        if nl != 0 {
            ((st[0].wrapping_add(nl as u32)) as *mut u8).write(0);
        }
        let _: u32 = callee_thiscall!(20, u32, stp, relocated(WMV_EXT), 4u32);
        // Format call: when the length word is nonzero the string pointer
        // goes first (otherwise the default name string), i.e. the
        // struct base), otherwise the default name string.
        let a21 = if (st[1] as u16) != 0 {
            st[0]
        } else {
            relocated(FMT_DEFAULT)
        };
        let mut b21 = [0u32; 2];
        let _: u32 = callee_cdecl!(21, u32, a21, b21.as_mut_ptr() as u32, FMT_SIZE);
        let mut b22 = [0u32; 2];
        let _: u32 = callee_thiscall!(22, u32, ebx, b22.as_mut_ptr() as u32);
        let _: u32 = callee_thiscall!(23, u32, ebx);
        *frame_word = ((ebx + OBJ_A0) as *const u32).read_unaligned();
        // The +0xa0 float lands in a dead frame slot; only +0xa8 matters.
        let e = ((ebx + OBJ_A8) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(24, u32, ebx);
        let _: u32 = callee_thiscall!(25, u32, ebx);
        let _: u32 = callee_cdecl!(16, u32, ebx);
        let _: u32 = callee_cdecl!(16, u32, st[0]);
        // Unsigned limit check: above it, next entry; otherwise the
        // float compare decides (ordered greater-than exits).
        let g = global::<u32>(GLOBAL_LIMIT).read();
        if e <= g {
            let f = f32::from_bits(*frame_word);
            let lim = global::<u32>(GLOBAL_FLOATLIM).read() as i32 as f32;
            if !(f > lim) {
                deep_entry(this, entry, append_struct, frame_word);
                return;
            }
        }
    }
}

/// Entry into the deep name path (the original's flag-zero branch, also reached from the
/// float tail): the name length minus one is stored in the frame word, then checked.
unsafe fn deep_entry(this: u32, entry: u32, append_struct: &mut [u32; 2], frame_word: &mut u32) {
    unsafe {
        let v = strlen(entry).wrapping_sub(1);
        *frame_word = v;
        if v >= NAME_LIMIT {
            return;
        }
        deep_name(this, entry, v, append_struct);
    }
}

/// Block A could not be built (its allocation or constructor returned nothing). The original
/// does not fault: it compares the frame word with the global float limit, continues with the
/// next entry when the word is greater, and otherwise takes the deep path.
unsafe fn block_a_float_tail(this: u32, entry: u32, append_struct: &mut [u32; 2], frame_word: &mut u32) {
    unsafe {
        let f = f32::from_bits(*frame_word);
        let lim = global::<u32>(GLOBAL_FLOATLIM).read() as i32 as f32;
        if !(f > lim) {
            deep_entry(this, entry, append_struct, frame_word);
        }
    }
}

/// Resolve a short entry name: format it into a fresh buffer, attach it
/// through the name hook when enabled, append the base name, and either
/// release the buffer when the result names the default or copy the
/// entry's record into the reserve buffer.
unsafe fn deep_name(this: u32, entry: u32, v: u32, append_struct: &mut [u32; 2]) {
    unsafe {
        let buf: u32 = callee_cdecl!(41, u32, NAME_BUF);
        if buf == 0 {
            panic!("original faults here: the name buffer is null and the terminator store writes through it (narrowed)");
        }
        let _: u32 = callee_cdecl!(38, u32, buf, entry, v.wrapping_sub(4));
        // Terminator at buf+v-4 (below the buffer for very short names;
        // the contract keeps a declared segment under it).
        ((buf.wrapping_add(v).wrapping_sub(4)) as *mut u8).write(0);
        if ((this + THIS_35C) as *const u8).read() != 0 {
            let mut st = [0u32; 2];
            let stp = st.as_mut_ptr() as u32;
            let lb = strlen(buf);
            let _: u32 = callee_thiscall!(20, u32, stp, buf, lb);
            let w374 = ((this + THIS_374) as *const u16).read_unaligned();
            let arg = if w374 != 0 {
                ((this + THIS_370) as *const u32).read_unaligned()
            } else {
                relocated(NAME_DEFAULT)
            };
            let r: u32 = callee_thiscall!(40, u32, stp, arg);
            if r as u8 == 0 {
                let _: u32 = callee_cdecl!(16, u32, buf);
                let _: u32 = callee_cdecl!(16, u32, st[0]);
                return;
            }
            let _: u32 = callee_cdecl!(16, u32, st[0]);
        }
        // Append the base name (empty on the proven paths).
        let src0 = this + THIS_300;
        let len1 = strlen(src0);
        let mut de = buf;
        while (de as *const u8).read() != 0 {
            de = de.wrapping_add(1);
        }
        let mut i = 0u32;
        while i < len1 {
            ((de + i) as *mut u8).write(((src0 + i) as *const u8).read());
            i += 1;
        }
        if strcmp(buf, relocated(CMP_STR)) == 0 {
            let _: u32 = callee_cdecl!(16, u32, buf);
            return;
        }
        // Copy the formatted name back over the entry, then the entry's
        // record through the reserve call.
        let mut j = 0u32;
        loop {
            let b = ((buf + j) as *const u8).read();
            ((entry + j) as *mut u8).write(b);
            if b == 0 {
                break;
            }
            j += 1;
        }
        let recbase = entry.wrapping_sub(ENTRY_OFF);
        let mut out = [0u32; RECORD_WORDS];
        let src = recbase as *const u32;
        for k in 0..RECORD_WORDS {
            out[k] = src.add(k).read_unaligned();
        }
        let dest: u32 = callee_thiscall!(39, u32, append_struct.as_mut_ptr() as u32, APPEND_ARG);
        (dest as *mut u32).copy_from_nonoverlapping(out.as_ptr(), RECORD_WORDS);
    }
}

/// Build the child object when the slot is empty: allocate it, derive
/// its name through two virtual calls and a format call, construct and
/// initialise it, set two mode words, initialise four items (each: fetch
/// a 24-byte record, run the item hook, release the record), run two
/// more hooks, set the scale, and optionally enable the first item.
unsafe fn construct_child(this: u32, count_lo: u32) -> u32 {
    unsafe {
        let a: u32 = callee_cdecl!(37, u32, CHILD_SIZE);
        if a == 0 {
            // Malloc failure: the slot is cleared, init runs on null,
            // and the mode-word store faults (mirrored for parity).
            ((this + CHILD_PTR) as *mut u32).write_unaligned(0);
            let f = f32::from_bits(((this + THIS_324) as *const u32).read_unaligned());
            let _: u32 = callee_thiscall!(30, u32, 0u32, 0u32, f.to_bits(), 0x3F800000u32);
            ((CHILD_1FC) as *mut u32).write_unaligned(2);
            panic!("original faults here: the null child is used for a table call (narrowed)");
        }
        let this_table = (this as *const u32).read_unaligned();
        // The slot pops nothing: the first call passes nothing and the
        // second call's pushed word is removed later by the shared
        // stack cleanup, so the stub takes no arguments and the pushed
        // answer is dead (unlogged). Anything else drifts the stack and the
        // epilogue cookie check faults.
        let vt48: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((this_table + THIS_VT48) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = vt48(this);
        let _: u32 = vt48(this);
        // Pops one word (the format); the other pushed word belongs to
        // the shared cleanup above. The receiver is `this` (left over).
        let f: u32 = callee_thiscall!(28, u32, this, relocated(CHILD_FMT));
        let c: u32 = callee_thiscall!(29, u32, a, f);
        // Argument order: the pushed zero first, then the float and 1.0.
        let f324 = ((this + THIS_324) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(30, u32, c, 0u32, f324, 0x3F800000u32);
        ((c + CHILD_1FC) as *mut u32).write_unaligned(2);
        let sub = ((c + CHILD_1EC) as *const u32).read_unaligned();
        ((sub + SUB_1F4) as *mut u32).write_unaligned(2);
        let child_table = (c as *const u32).read_unaligned();
        // Seven stack words: the item constant plus the six words of the
        // fetched record. The callee pops all of them (like the fetch
        // call pops its two zero words): anything less leaves the stack
        // pointer off and the epilogue's cookie check faults.
        let hook114: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(
                ((child_table + CHILD_VT114) as *const u32).read_unaligned() as usize,
            );
        let mut m31 = [0u32; 2];
        for k in 0..4usize {
            let rec: u32 = callee_thiscall!(31, u32, m31.as_mut_ptr() as u32, 0u32, 0u32);
            let src = rec as *const u32;
            let w = [
                src.read_unaligned(),
                src.add(1).read_unaligned(),
                src.add(2).read_unaligned(),
                src.add(3).read_unaligned(),
                src.add(4).read_unaligned(),
                src.add(5).read_unaligned(),
            ];
            let _: u32 = hook114(c, ITEM_ARGS[k], w[0], w[1], w[2], w[3], w[4], w[5]);
            let mut m33 = [0u32; 1];
            let _: u32 = callee_thiscall!(33, u32, m33.as_mut_ptr() as u32);
        }
        let vt4c: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((this_table + THIS_VT4C) as *const u32).read_unaligned() as usize,
        );
        let r34: u32 = vt4c(this);
        let hook7c: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(
            ((child_table + CHILD_VT7C) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = hook7c(c, 0x3F800000u32, 0x3F266666u32, r34);
        ((c + CHILD_SCALE) as *mut u32).write_unaligned(SCALE_BITS);
        // The count (the row count) is the one read after the record call (loop 2 counts
        // or is skipped with a zero bound), so this fires iff the slot
        // holds a negative value (signed comparison).
        let lim354 = ((this + THIS_354) as *const u32).read_unaligned();
        if (count_lo as i32) > lim354 as i32 {
            let sub_table = (sub as *const u32).read_unaligned();
            let sub120: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                ((sub_table + SUB_VT120) as *const u32).read_unaligned() as usize,
            );
            let _: u32 = sub120(sub, 1);
        }
        c
    }
}


/// Loop-3 row on the remainder-zero branch (the row index is a multiple of the divisor).
/// Object B (0x1F8 bytes) is built from the child's name and a format; row object E (0x328
/// bytes) is built from B's answers. Returns B (the original keeps it in a register,
/// which the remainder-nonzero branch reads on the next row).
unsafe fn row_rem_zero(this: u32, p: u32, k: u32) -> u32 {
    unsafe {
        let ob: u32 = callee_cdecl!(42, u32, ROW_B_SIZE);
        if ob == 0 {
            panic!("original faults here: object B is null and is read through (narrowed)");
        }
        // Child name: slot 0x48 of the child's table, no arguments.
        let child = ((this + CHILD_PTR) as *const u32).read_unaligned();
        let child_tab = (child as *const u32).read_unaligned();
        let name: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((child_tab + 0x48) as *const u32).read_unaligned() as usize,
        );
        let sr = name(child);
        let r: u32 = callee_cdecl!(44, u32, relocated(ROW_FMT_B1), k);
        let ob = callee_thiscall!(45, u32, ob, r, sr);
        // Object B's table: slot 0x1fc (args 2, float, 0), slot 0xa0 (float).
        let btab = (ob as *const u32).read_unaligned();
        let f1fc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(
            ((btab + 0x1FC) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = f1fc(ob, 2, ((this + THIS_320) as *const u32).read_unaligned(), 0);
        let fa0: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((btab + 0xA0) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = fa0(ob, ((this + THIS_31C) as *const u32).read_unaligned());
        // Row object E.
        let e: u32 = callee_cdecl!(48, u32, ROW_E_SIZE);
        if e == 0 {
            panic!("original faults here: row record E is null and is written through (narrowed)");
        }
        // Slot 0x48 of B, twice (both calls take no arguments; the format below takes the second answer).
        let f48: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((btab + 0x48) as *const u32).read_unaligned() as usize,
        );
        let a1 = f48(ob);
        let a2 = f48(ob);
        let r2: u32 = callee_cdecl!(51, u32, relocated(ROW_FMT_B2), a2, k);
        let e = callee_thiscall!(52, u32, e, r2, a1);
        row_common(this, p, e, ROW_FMT_B3);
        // Remainder-zero rows also store the first row object into this+0x350 when that is empty.
        if ((this + THIS_350) as *const u32).read_unaligned() == 0 {
            ((this + THIS_350) as *mut u32).write_unaligned(e);
        }
        ob
    }
}

/// Loop-3 row on the remainder-nonzero branch: row object E' (0x328 bytes) is built from the
/// previous row's object B (carried across rows as the original's register), then the same common block.
unsafe fn row_rem_nonzero(this: u32, p: u32, k: u32, ob: u32) {
    unsafe {
        let btab = (ob as *const u32).read_unaligned();
        let e0: u32 = callee_cdecl!(48, u32, ROW_E_SIZE);
        if e0 == 0 {
            panic!("original faults here: row record E is null and is written through (narrowed)");
        }
        let f48: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((btab + 0x48) as *const u32).read_unaligned() as usize,
        );
        let a1 = f48(ob);
        let a2 = f48(ob);
        let r2: u32 = callee_cdecl!(51, u32, relocated(ROW_FMT_B4), a2, k);
        let e = callee_thiscall!(52, u32, e0, r2, a1);
        row_common(this, p, e, ROW_FMT_B5);
    }
}

/// The block shared by both row branches: the name field store, record 53 with the given format,
/// the name string at P (len - 4 must be below 0x26), name assembly 54, record 55, then the
/// sub-object and table calls on E.
unsafe fn row_common(this: u32, p: u32, e: u32, fmt53: u32) {
    unsafe {
        ((e + ROW_E_324) as *mut u32).write_unaligned(((p - 4) as *const u32).read_unaligned());
        let flag = ((this + THIS_379) as *const u8).read() as u32;
        let f318 = ((this + 0x318) as *const u32).read_unaligned();
        let f31c = ((this + THIS_31C) as *const u32).read_unaligned();
        let mut frame_buf = [0u32; 40];
        let buf = frame_buf.as_mut_ptr() as u32;
        let _: u32 = callee_thiscall!(53, u32, e, p, relocated(fmt53), buf, f318, f31c, flag, 4);
        let mut len = 0u32;
        while ((p + len) as *const u8).read() != 0 {
            len += 1;
        }
        let len4 = len.wrapping_sub(4);
        let _: u32 = callee_cdecl!(54, u32, buf, p, len4);
        if len4 >= 0x26 {
            panic!("original fast-fails here: name length at or above the limit (buffer-overrun handler; narrowed)");
        }
        // The original terminates the name at its measured length before the record call reads it.
        ((buf + len4) as *mut u8).write(0);
        let _: u32 = callee_thiscall!(55, u32, e, buf, 0);
        // Sub-object of E: slot 0x1d4 with argument 0x12.
        let sub = ((e + ROW_E_1E8) as *const u32).read_unaligned();
        let sub_tab = (sub as *const u32).read_unaligned();
        let f1d4: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((sub_tab + 0x1D4) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = f1d4(sub, 0x12);
        // This table's slot 0x4c before each of E's table calls 0x170, 0x17c, 0x188.
        let this_tab = (this as *const u32).read_unaligned();
        let f4c: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((this_tab + 0x4C) as *const u32).read_unaligned() as usize,
        );
        let e_tab = (e as *const u32).read_unaligned();
        let r34 = f4c(this);
        let f170: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((e_tab + 0x170) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = f170(e, r34);
        let r34b = f4c(this);
        let f17c: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((e_tab + 0x17C) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = f17c(e, r34b);
        let r34c = f4c(this);
        let f188: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((e_tab + 0x188) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = f188(e, r34c);
        let _: u32 = callee_thiscall!(63, u32, e, 1);
        // Sub-object: slot 0x1f8 with argument 0; E's table: slot 0x120 (0), slot 0x28 (1).
        let sub2 = ((e + ROW_E_1E8) as *const u32).read_unaligned();
        let sub_tab2 = (sub2 as *const u32).read_unaligned();
        let f1f8: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((sub_tab2 + 0x1F8) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = f1f8(sub2, 0);
        let f120: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((e_tab + 0x120) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = f120(e, 0);
        let f28: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((e_tab + 0x28) as *const u32).read_unaligned() as usize,
        );
        let _: u32 = f28(e, 1);
    }
}

export!(thiscall, rw_00e4eb50(this: u32) -> u32 {
    unsafe { stage_body(this) }
});
