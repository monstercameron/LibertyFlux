// original: 0x00dd46c0 UIMontageContainer::vf114

/// State dispatcher for a montage UI container (thiscall, no stack arguments).
///
/// Reads a nonzero flag from a word behind a global state pointer (`+0x200`);
/// when the flag is zero it clears the cursor (`+0x214`) and pending (`+0x210`)
/// slots and returns 0. Otherwise it opens a session through a helper, then
/// walks three identity gates: each gate calls slot 0 of the session object
/// and a shared lookup helper, and compares the two answers. The first gate
/// taken decides the region:
///
/// * gate 1 equal: mark the container active and iterate the current list
///   (`+0x1e0`), notifying two helpers per element with the element's words at
///   `+0x310`/`+0x314`; returns the final count.
/// * gate 1 differs, gate 2 equal: run a threshold check on a freshness value
///   (100 or more tail-jumps to a shared teardown routine after a state call);
///   otherwise refresh the container, toggle one of two sub-paths on the mode
///   byte (`+0x218`), run a fixed refresh sequence (session re-open, two
///   keyed lookups, property propagation, one float blend value), and return
///   the final helper's answer.
/// * both differ, gate 3 equal: set the phase byte (`+0x21c`) and reconcile
///   the pending index against the list count: either drop the pending entry
///   (copying it to `+0x20c`) or re-arm the container through its own vtable
///   slot `+0x28`; returns the last answer.
/// * all differ: fall through and return the last lookup answer.
///
/// The session-open call takes the leftover zero word pushed for the lookup
/// above it as its third argument. One float travels from a callee's x87
/// return slot into a stack argument of the next call. The tail jump runs with
/// the entry stack restored, so it carries no hidden arguments.
///
/// Original: 0x00dd46c0 (thiscall, `this` in ECX, result in EAX).

use lf_checker_rt::{callee_cdecl, callee_thiscall, relocated};

const STATE_GLOBAL: u32 = 0x018B6C8C;
const STATE_FLAG: u32 = 0x200;
const HELPER_CTX: u32 = 0x01981A4C;
const LOOKUP_CTX: u32 = 0x01176888;
const TAG_A: u32 = 0x00EFAF04;
const TAG_B: u32 = 0x00EFAF14;
const TAG_C: u32 = 0x00EFAF24;
const TAG_D: u32 = 0x00EFAF44;
const TAG_E: u32 = 0x00EFAF50;
const TAG_F: u32 = 0x00EFAF60;
const TAG_G: u32 = 0x00EFAF80;
const TAG_H: u32 = 0x00EFAF90;
const TAG_I: u32 = 0x00EFAF9C;
const TAG_J: u32 = 0x00EFAFAC;
const FRESHNESS_LIMIT: u32 = 0x64;

const O_LIST: u32 = 0x1E0;
const O_AUX: u32 = 0x1E8;
const O_CFG: u32 = 0x1EC;
const O_OUT: u32 = 0x1F0;
const O_VIEW: u32 = 0x1F4;
const O_SRC: u32 = 0x1F8;
const O_WANT: u32 = 0x204;
const O_PENDING: u32 = 0x210;
const O_DROPPED: u32 = 0x20C;
const O_CURSOR: u32 = 0x214;
const O_MODE: u32 = 0x218;
const O_PHASE: u32 = 0x21C;
const O_SLOT_A: u32 = 0x310;
const O_SLOT_B: u32 = 0x314;

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
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

/// Call `slot` of the object at `obj` with no stack arguments (thiscall).
#[inline(always)]
unsafe fn vcall0(obj: u32, slot: u32, this: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(this)
    }
}

/// Call `slot` of the object at `obj` with one stack argument (thiscall).
#[inline(always)]
unsafe fn vcall1(obj: u32, slot: u32, this: u32, a0: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(this, a0)
    }
}

/// Gate-1-equal region: iterate the list, notifying two helpers per element.
unsafe fn region_list(this: u32) -> u32 {
    unsafe {
        let active: u32 = callee_thiscall!(3, u32, relocated(HELPER_CTX), relocated(TAG_B));
        wr8(active + 0x20B, 1);
        let _: u32 = callee_thiscall!(5, u32, relocated(LOOKUP_CTX), relocated(TAG_C));
        let list = rd32(this + O_LIST);
        let mut index = 0u32;
        let mut count: u32 = vcall0(list, 0x1D4, list);
        if count != 0 {
            loop {
                let elem: u32 = vcall1(list, 0x1E0, list, index);
                let src = rd32(this + O_SRC);
                let _: u32 = callee_thiscall!(6, u32, src, index, rd32(elem + O_SLOT_A));
                let _: u32 = callee_thiscall!(7, u32, src, index, rd32(elem + O_SLOT_B));
                index += 1;
                count = vcall0(list, 0x1D4, list);
                if index >= count {
                    break;
                }
            }
        }
        count
    }
}

/// Gate-2-equal region: threshold check, then a fixed refresh sequence.
unsafe fn region_refresh(this: u32, session_outer: u32) -> u32 {
    unsafe {
        let src = rd32(this + O_SRC);
        let freshness: u32 = callee_thiscall!(8, u32, src);
        // Signed: the original's conditional jump after this comparison
        // is a signed one, so values with the high bit set take the
        // else path. (a-S07 fix: the verified text compared unsigned.)
        if (freshness as i32) >= FRESHNESS_LIMIT as i32 {
            let _: u32 = callee_cdecl!(9, u32, 0x2A);
            let state = rd32(lf_checker_rt::global::<u32>(STATE_GLOBAL) as u32);
            return callee_thiscall!(11, u32, state);
        }
        let active: u32 = callee_thiscall!(3, u32, relocated(HELPER_CTX), relocated(TAG_E));
        wr8(active + 0x20B, 1);
        let state = rd32(lf_checker_rt::global::<u32>(STATE_GLOBAL) as u32);
        let _: u32 = callee_thiscall!(10, u32, state);
        if rd8(this + O_MODE) != 0 {
            let aux = rd32(this + O_AUX);
            wr8(this + O_MODE, 0);
            let _: u32 = vcall1(aux, 0x120, aux, 0);
            let list = rd32(this + O_LIST);
            let sub = rd32(list + 0x21C);
            let _: u32 = vcall1(sub, 0x120, sub, 1);
            let cfg = rd32(this + O_CFG);
            wr8(cfg + 0x1F4, 0);
            let out = rd32(this + O_OUT);
            let _: u32 = callee_thiscall!(12, u32, out, 0);
        } else {
            let list = rd32(this + O_LIST);
            let sub = rd32(list + O_LIST);
            let probe: u32 = vcall0(sub, 0x220, sub);
            let _: u32 = vcall0(probe, 0x1B0, probe);
            let list2 = rd32(this + O_LIST);
            let sub2 = rd32(list2 + O_LIST);
            let probe2: u32 = vcall0(sub2, 0x220, sub2);
            let _: u32 = vcall1(probe2, 0x18, probe2, 0);
        }
        let _: u32 = callee_thiscall!(5, u32, relocated(LOOKUP_CTX), relocated(TAG_F));
        let _: u32 = callee_cdecl!(13, u32, relocated(TAG_G), 0);
        let handle: u32 = callee_cdecl!(2, u32, relocated(TAG_H));
        // The session-open call takes the leftover zero word pushed for the
        // lookup above it as its third argument.
        let session: u32 = callee_thiscall!(14, u32, this, session_outer, handle, 0);
        let list = rd32(this + O_LIST);
        let first: u32 = vcall0(session, 0x4C, session);
        let key: u32 = vcall1(list, 0x1E4, list, first);
        let _: u32 = callee_thiscall!(15, u32, this, key);
        let _: u32 = callee_thiscall!(16, u32, this, session);
        let first2: u32 = vcall0(session, 0x4C, session);
        let list3 = rd32(this + O_LIST);
        let key2: u32 = vcall1(list3, 0x1E4, list3, first2);
        let list4 = rd32(this + O_LIST);
        let _: u32 = callee_thiscall!(17, u32, list4, key2, 1);
        let _: u32 = vcall1(session, 0x18, session, 1);
        let list5 = rd32(this + O_LIST);
        let _: u32 = vcall1(list5, 0x18, list5, 1);
        let _: u32 = callee_thiscall!(18, u32, this);
        let sub_a = rd32(rd32(this + O_LIST) + O_LIST);
        let prop: u32 = vcall0(sub_a, 0x21C, sub_a);
        let cfg = rd32(this + O_CFG);
        let _: u32 = callee_thiscall!(19, u32, cfg, prop);
        let sub_b = rd32(rd32(this + O_LIST) + O_LIST);
        let prop2: u32 = vcall0(sub_b, 0x21C, sub_b);
        let src2 = rd32(this + O_SRC);
        let blend: f32 = callee_thiscall!(20, f32, src2, prop2);
        let target: u32 = callee_thiscall!(4, u32, relocated(HELPER_CTX), relocated(TAG_I), blend.to_bits());
        callee_thiscall!(21, u32, target)
    }
}

/// Gate-3-equal region: reconcile the pending index against the list count.
unsafe fn region_reconcile(this: u32) -> u32 {
    unsafe {
        wr8(this + O_PHASE, 3);
        let list = rd32(this + O_LIST);
        let count_a: u32 = vcall0(list, 0x1D4, list);
        let want = rd32(this + O_WANT);
        let cursor_seen = rd32(this + O_CURSOR) == count_a.wrapping_sub(1);
        let list2 = rd32(this + O_LIST);
        let count_b: u32 = vcall0(list2, 0x1D4, list2);
        let cursor = rd32(this + O_CURSOR);
        let want_seen = want == count_b;
        let adjacent = cursor == want || cursor.wrapping_add(1) == want;
        let pending = rd32(this + O_PENDING);
        if pending != 0 && (!cursor_seen || !want_seen) && !adjacent {
            let _: u32 = callee_thiscall!(22, u32, this, pending);
            let dropped = rd32(this + O_PENDING);
            wr32(this + O_DROPPED, dropped);
            wr32(this + O_CURSOR, 0);
            wr32(this + O_PENDING, 0);
            return dropped;
        }
        let list3 = rd32(this + O_LIST);
        let _: u32 = callee_thiscall!(17, u32, list3, cursor, 1);
        let view = rd32(this + O_VIEW);
        let _: u32 = vcall1(view, 0x120, view, 0);
        let sub = rd32(rd32(this + O_CFG) + 0x1E4);
        let _: u32 = vcall1(sub, 0x120, sub, 0);
        let answer: u32 = vcall1(this, 0x28, this, 1);
        wr8(this + O_PHASE, 0);
        wr32(this + O_CURSOR, 0);
        wr32(this + O_PENDING, 0);
        answer
    }
}

lf_checker_rt::export!(thiscall, rw_00dd46c0(this: u32) -> u32 {
    unsafe {
        let state = rd32(lf_checker_rt::global::<u32>(STATE_GLOBAL) as u32);
        let flag = rd32(state + STATE_FLAG);
        if flag == 0 {
            wr32(this + O_CURSOR, 0);
            wr32(this + O_PENDING, 0);
            return 0;
        }
        let session: u32 = callee_thiscall!(1, u32, relocated(HELPER_CTX), flag);
        let _opened: u32 = vcall0(this, 0x1C0, this);
        let gate1: u32 = vcall0(session, 0, session);
        let want1: u32 = callee_cdecl!(2, u32, relocated(TAG_A));
        if gate1 == want1 {
            return region_list(this);
        }
        let gate2: u32 = vcall0(session, 0, session);
        let want2: u32 = callee_cdecl!(2, u32, relocated(TAG_D));
        if gate2 == want2 {
            return region_refresh(this, session);
        }
        let gate3: u32 = vcall0(session, 0, session);
        let want3: u32 = callee_cdecl!(2, u32, relocated(TAG_J));
        if gate3 != want3 {
            return want3;
        }
        region_reconcile(this)
    }
});
