// original: 0x00c6e420 nm_message_build
// Behaviour-message builder: resolves an object through a two-stage virtual
// dispatch, prepares a record slot, runs a count-driven float loop, and fills
// the record (cdecl/4 -> u32).
//
// Behaviour. Takes a target pointer, a blend amount, and two key words.
// It fetches an object with a one-argument call, probes it through virtual
// slot 0xA0, and either falls back to a word at object+0x100 (null probe)
// or chains a second 0xA0 call with a 0xE0 call on its answer; the word
// past the resulting pointer is kept as a handle. Three frame-object calls
// follow (0, 5 and 2 stack arguments), then a 0x14-byte allocation whose
// construction answer replaces the handle (or zero when it fails), a slot
// lookup on the object array global that stores the handle, and a 0x50-byte
// allocation whose construction answer is kept as a token (or zero).
//
// While the global loop count is positive, each pass blends the amount with
// the pass index scaled by the global step, hands that plus the frame object
// to a four-argument call, runs two gated float-triple calls (the second
// pair summed word-wise into a third call when the first gate answers
// nonzero), copies a 16-byte constant block for a gated call, and when that
// gate answers nonzero copies the block again and evaluates a fixed
// single-precision expression over the two copies into a result block for a
// final call. Every pass ends with a three-argument call carrying the frame
// object, the handle and a global word, with the token as its object.
//
// The tail writes the token, the amount bits, a two-argument hash over the
// word at target+0x1C, and the two key words into the record selected by
// the global index word, then runs a closing frame-object call and returns
// its answer. All floating-point steps run in original order, each pinned,
// so results match bit for bit.
use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Object array global: pointer to the record-pointer table.
const OBJ_ARRAY: u32 = 0x16dd654;
/// Object index global: one-based word selecting the record (low word used).
const OBJ_INDEX: u32 = 0x16dd658;
/// Gate word global: carried as a value into the per-pass closing call.
const GATE_WORD: u32 = 0x16dd634;
/// Loop count global: passes run while this stays positive (signed).
const LOOP_COUNT: u32 = 0x1049734;
/// Loop step global: scale applied to the pass index each pass.
const LOOP_STEP: u32 = 0x1049730;
/// Float triple constants handed to the gated triple calls.
const TRIPLE: u32 = 0x1b4b2a0;
/// 16-byte constant block copied for the gated block calls.
const BLK_CONST: u32 = 0x11100d0;
/// First virtual slot probed on the fetched object (twice).
const VT_PROBE: u32 = 0xA0;
/// Second virtual slot called on the probe chain answer.
const VT_CHAIN: u32 = 0xE0;
/// Fallback offset read on the object when the probe answers null.
const FALLBACK_OFF: u32 = 0x100;
/// First allocation size in bytes.
const ALLOC_SMALL: u32 = 0x14;
/// Second allocation size in bytes.
const ALLOC_BIG: u32 = 0x50;
/// Slot-lookup argument carried with the object array address.
const SLOT_ARG: u32 = 0x10;
/// Hash input offset in the target object.
const HASH_OFF: u32 = 0x1C;

fn body<const MUT: bool>(target: u32, amount: f32, key_a: u32, key_b: u32) -> u32 {
    unsafe {
        let rd32 = |addr: u32| -> u32 { (addr as *const u32).read_unaligned() };
        // Frame model: one address-taken object word plus three kept slots.
        // The object word is never read by either side, only its address is
        // passed; the kept slots mirror the original's frame words.
        let frame_obj: u32 = 0;
        let frame_ptr = core::ptr::addr_of!(frame_obj) as u32;

        // Entry: fetch the object and run the two-stage virtual dispatch.
        let obj = callee_cdecl!(1, u32, 0u32);
        let probe_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj).wrapping_add(VT_PROBE)) as usize);
        let probe = probe_fn(obj);
        let inner: u32;
        if probe == 0 {
            inner = rd32(obj.wrapping_add(FALLBACK_OFF));
        } else {
            let chain_fn: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(obj).wrapping_add(VT_PROBE)) as usize);
            let mid = chain_fn(obj);
            let tail_fn: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(mid).wrapping_add(VT_CHAIN)) as usize);
            inner = tail_fn(mid);
        }
        let mut handle = rd32(inner.wrapping_add(4));
        let slot_handle = handle;

        // Frame-object setup calls.
        callee_thiscall!(4, u32, frame_ptr);
        callee_thiscall!(5, u32, frame_ptr, handle, 1u32, 0u32, 1u32, 0u32);
        callee_thiscall!(6, u32, frame_ptr, handle, 0u32);

        // First allocation: the construction answer replaces the handle.
        let small = callee_cdecl!(7, u32, ALLOC_SMALL);
        if small == 0 {
            handle = 0;
        } else {
            handle = callee_thiscall!(8, u32, small);
        }

        // Slot lookup stores the handle; the indexed table entry is kept.
        let fresh = callee_thiscall!(9, u32, relocated(OBJ_ARRAY), SLOT_ARG);
        (fresh as *mut u32).write_unaligned(handle);
        let idx = global::<u16>(OBJ_INDEX).read_unaligned() as u32;
        let arr = global::<u32>(OBJ_ARRAY).read();
        let slot_entry = rd32(arr.wrapping_add(idx.wrapping_mul(4).wrapping_sub(4)));

        // Second allocation: its construction answer is the pass token.
        let big = callee_cdecl!(10, u32, ALLOC_BIG);
        let slot_token = if big == 0 {
            0
        } else {
            callee_thiscall!(11, u32, big)
        };

        // Count-driven loop.
        let count = global::<i32>(LOOP_COUNT).read();
        if count > 0 {
            let step = global::<f32>(LOOP_STEP).read_unaligned();
            let gate = global::<u32>(GATE_WORD).read();
            let c0 = global::<f32>(TRIPLE).read_unaligned();
            let c1 = global::<f32>(TRIPLE.wrapping_add(4)).read_unaligned();
            let c2 = global::<f32>(TRIPLE.wrapping_add(8)).read_unaligned();
            let blk = global::<[u32; 4]>(BLK_CONST).read_unaligned();
            let mut ctr = count;
            let mut fctr = count.wrapping_sub(1);
            while ctr > 0 {
                let scaled = black_box(black_box(fctr as f32) * black_box(step));
                // MUTANT (MUT=true): adds the scaled index instead of
                // subtracting it; the per-pass call argument must differ.
                let first = if MUT {
                    black_box(amount + black_box(scaled))
                } else {
                    black_box(amount - black_box(scaled))
                };
                callee_thiscall!(
                    12, u32, target, first.to_bits(), frame_ptr, 0u32, 1u32
                );
                let mut t1 = [c0, c1, c2];
                let g1 = callee_thiscall!(
                    13, u32, frame_ptr, 5u32, 0u32, core::ptr::addr_of_mut!(t1) as u32
                );
                if g1 & 0xFF != 0 {
                    let mut t2 = [c0, c1, c2];
                    callee_thiscall!(
                        14, u32, frame_ptr, 0u32, 0u32, core::ptr::addr_of_mut!(t2) as u32
                    );
                    let s0 = black_box(t2[0] + t1[0]);
                    let s1 = black_box(t2[1] + t1[1]);
                    let s2 = black_box(t2[2] + t1[2]);
                    let mut sums = [s0, s1, s2];
                    callee_thiscall!(
                        15, u32, frame_ptr, 0u32, 0u32, core::ptr::addr_of_mut!(sums) as u32
                    );
                }
                let mut blk1 = blk;
                let g2 = callee_thiscall!(
                    16, u32, frame_ptr, 6u32, 0u32, core::ptr::addr_of_mut!(blk1) as u32
                );
                if g2 & 0xFF != 0 {
                    let mut blk2 = blk;
                    callee_thiscall!(
                        17, u32, frame_ptr, 1u32, 0u32, core::ptr::addr_of_mut!(blk2) as u32
                    );
                    // Fixed single-precision expression over the two block
                    // copies. Names track the original's vector registers:
                    // x6/x7 hold first-block word 0 / second-block word 3,
                    // x4/x5 the matching inner words, x0/x2/x3 stream the
                    // remaining words; each arithmetic result is pinned.
                    let x6 = f32::from_bits(blk1[0]);
                    let x7 = f32::from_bits(blk2[3]);
                    let mut x4 = f32::from_bits(blk2[1]);
                    let mut x5 = f32::from_bits(blk1[1]);
                    let mut res = blk;
                    let mut x0 = f32::from_bits(blk2[0]);
                    let mut x2 = f32::from_bits(blk2[2]);
                    let mut x3 = f32::from_bits(blk1[2]);
                    x0 = black_box(x0 * x6);
                    let mut x1 = x7;
                    x1 = black_box(x1 * f32::from_bits(blk1[3]));
                    x1 = black_box(x1 - x0);
                    x0 = x4;
                    x0 = black_box(x0 * x5);
                    x1 = black_box(x1 - x0);
                    x0 = x2;
                    x0 = black_box(x0 * x3);
                    x1 = black_box(x1 - x0);
                    x0 = f32::from_bits(blk2[0]);
                    x0 = black_box(x0 * f32::from_bits(blk1[3]));
                    res[3] = x1.to_bits();
                    x1 = x6;
                    x1 = black_box(x1 * x7);
                    x1 = black_box(x1 + x0);
                    x0 = x2;
                    x0 = black_box(x0 * x5);
                    x1 = black_box(x1 + x0);
                    x0 = x3;
                    x0 = black_box(x0 * x4);
                    x1 = black_box(x1 - x0);
                    x0 = x4;
                    x0 = black_box(x0 * f32::from_bits(blk1[3]));
                    x4 = black_box(x4 * x6);
                    res[0] = x1.to_bits();
                    x1 = x5;
                    x5 = black_box(x5 * f32::from_bits(blk2[0]));
                    x1 = black_box(x1 * x7);
                    x1 = black_box(x1 + x0);
                    x0 = x3;
                    x0 = black_box(x0 * f32::from_bits(blk2[0]));
                    x3 = black_box(x3 * x7);
                    x1 = black_box(x1 + x0);
                    x0 = x2;
                    x2 = black_box(x2 * f32::from_bits(blk1[3]));
                    x0 = black_box(x0 * x6);
                    x3 = black_box(x3 + x2);
                    x1 = black_box(x1 - x0);
                    x3 = black_box(x3 + x4);
                    res[1] = x1.to_bits();
                    x3 = black_box(x3 - x5);
                    res[2] = x3.to_bits();
                    callee_thiscall!(
                        18, u32, frame_ptr, 1u32, 0u32, core::ptr::addr_of_mut!(res) as u32
                    );
                }
                callee_thiscall!(19, u32, slot_token, frame_ptr, slot_handle, gate);
                ctr = ctr.wrapping_sub(1);
                fctr = fctr.wrapping_sub(1);
            }
        }

        // Tail: fill the selected record, run the closer, return its answer.
        let out = slot_entry;
        (out.wrapping_add(0x10) as *mut u32).write_unaligned(slot_token);
        (out.wrapping_add(0x0C) as *mut u32).write_unaligned(amount.to_bits());
        let hv = rd32(target.wrapping_add(HASH_OFF));
        let h = callee_cdecl!(20, u32, hv, 0u32);
        (out.wrapping_add(8) as *mut u32).write_unaligned(h);
        (out as *mut u32).write_unaligned(key_a);
        (out.wrapping_add(4) as *mut u32).write_unaligned(key_b);
        callee_thiscall!(21, u32, frame_ptr)
    }
}

export!(cdecl, rw_c6e420(target: u32, amount: f32, key_a: u32, key_b: u32) -> u32 {
    body::<false>(target, amount, key_a, key_b)
});

export!(cdecl, mut_c6e420(target: u32, amount: f32, key_a: u32, key_b: u32) -> u32 {
    body::<true>(target, amount, key_a, key_b)
});
