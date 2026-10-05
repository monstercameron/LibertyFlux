// original: 0x00DC8160 CMovementEventHandler::vf1

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// Handle a movement event: resolve the event's target, then either attach
/// a freshly built follower or refresh the current one.
///
/// `this+0x04` holds a context `b`, `arg1+0x18` the event target `e` (kept
/// non-null by the contract: the null path returns whatever the entry
/// garbage in the return register was, which no rewrite can reproduce).
/// Callee 1 (on `[b+0x224]+0x44`) resolves a handler; a null third argument,
/// a set bit 2 at `b+0x26c`, or a zero low byte from the handler's virtual
/// slot `+0x28` keeps the current follower silently, returning the value
/// the failing step left in the return register (the handler, its answer,
/// the context, or the event kind at `arg1+0x1c`, which must also differ
/// from 1).
///
/// The handler's slot `+0x30` yields a worker whose slot `+0x1c` consumes
/// one scratch pointer. Callee 5 (on the same context member, id `0x3b5`)
/// picks the half. First half: callee 6 (on a global) supplies an object,
/// or null to clear the follower (`[this+8] = 0`, return 0); callee 7
/// (cdecl, three words, float in x87 ST0, popped by nobody: its argument
/// words stay on the stack and become callee 8's) scores the event kind;
/// the score is stored over the index word, and callee 8 builds the
/// follower from the score, the scratch pointer and the target. The
/// follower is stored at `[this+8]` and returned.
/// Second half: callee 9 (id `0x41e`, extra 0) may keep the follower; else
/// callee 6 supplies again and callee 10 builds a candidate from the event
/// kind, a scratch word, the target and zero (null supply yields null).
/// A global counter is incremented (and observed); from 16 on, callee 11
/// (the game's random generator) rolls, and a roll below `0x3fff` sets bits
/// `0x30` at the candidate's `+0x9c` and resets the counter. The candidate
/// lands at `[this+8]`; the return is the counter or the roll. A null
/// candidate on the roll path faults at `[0x9c]` (fault parity).
///
/// Original: 0x00DC8160 (thiscall, three stack words; the middle one unread).
export!(thiscall, rw_dc8160(this: u32, arg1: u32, _arg2: u32, arg3: u32) -> u32 {
    const CTX_OFF: u32 = 0x04;
    const TARGET_OFF: u32 = 0x18;
    const KIND_OFF: u32 = 0x1c;
    const MEMBER_OFF: u32 = 0x224;
    const MEMBER_SUB: u32 = 0x44;
    const MEMBER_SUB2: u32 = 0x2e0;
    const FLAG_OFF: u32 = 0x26c;
    const FOLLOWER_OFF: u32 = 0x08;
    const FIRST_ID: u32 = 0x3b5;
    const SECOND_ID: u32 = 0x41e;
    const CAND_FLAG_OFF: u32 = 0x9c;
    const CAND_FLAG_BITS: u32 = 0x30;
    const COUNT_LIMIT: u32 = 0x10;
    const ROLL_LIMIT: u32 = 0x3fff;
    const TASK_GLOBAL: u32 = 0x0167_e2a0;
    const COUNT_GLOBAL: u32 = 0x017a_b504;

    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }

    unsafe {
        let b = rd32(this.wrapping_add(CTX_OFF));
        let e = rd32(arg1.wrapping_add(TARGET_OFF));
        if e == 0 {
            // Excluded from the proof (see doc comment): the original
            // returns its entry return register here.
            return 0;
        }
        if rd8(b.wrapping_add(FLAG_OFF)) & 4 != 0 {
            return b;
        }
        let member = rd32(b.wrapping_add(MEMBER_OFF));
        let handler: u32 = callee_thiscall!(1, u32, member.wrapping_add(MEMBER_SUB));
        if arg3 == 0 {
            return handler;
        }
        let gate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(handler).wrapping_add(0x28)) as usize);
        let g: u32 = gate(handler);
        if (g & 0xff) == 0 {
            return g;
        }
        let spawn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(handler).wrapping_add(0x30)) as usize);
        let worker = spawn(handler);
        let feed: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(worker).wrapping_add(0x1c)) as usize);
        let scratch = [0u32; 1];
        let _ = feed(worker, scratch.as_ptr() as u32);
        let kind = rd32(arg1.wrapping_add(KIND_OFF));
        if kind == 1 {
            return kind;
        }
        let pick: u32 = callee_thiscall!(5, u32, member.wrapping_add(MEMBER_SUB), FIRST_ID);
        if pick != 0 {
            let supply: u32 = callee_thiscall!(6, u32, global::<u32>(TASK_GLOBAL).read());
            if supply == 0 {
                wr32(this.wrapping_add(FOLLOWER_OFF), 0);
                return 0;
            }
            let scratch2 = [0u32; 1];
            let score: f32 = callee_cdecl!(7, f32, kind, scratch2.as_ptr() as u32, e);
            // Callee 8's stack words are callee 7's leftovers (cdecl pops
            // nothing): the stored score where the index was, the scratch
            // pointer (skipped: own stack), and the target.
            let built: u32 = callee_thiscall!(8, u32, supply, score.to_bits(), scratch2.as_ptr() as u32, e);
            wr32(this.wrapping_add(FOLLOWER_OFF), built);
            return built;
        }
        let keep: u32 = callee_thiscall!(9, u32, member.wrapping_add(MEMBER_SUB2), SECOND_ID, 0);
        if (keep & 0xff) != 0 {
            return keep;
        }
        let supply: u32 = callee_thiscall!(6, u32, global::<u32>(TASK_GLOBAL).read());
        let mut cand = 0u32;
        if supply != 0 {
            let scratch3 = [0u32; 1];
            cand = callee_thiscall!(10, u32, supply, kind, scratch3.as_ptr() as u32, e, 0);
        }
        let count = global::<u32>(COUNT_GLOBAL).read().wrapping_add(1);
        global::<u32>(COUNT_GLOBAL).write(count);
        let mut ret = count;
        if count >= COUNT_LIMIT {
            let roll: u32 = callee_cdecl!(11, u32,);
            ret = roll;
            if roll < ROLL_LIMIT {
                let q = rd32(cand.wrapping_add(CAND_FLAG_OFF)) | CAND_FLAG_BITS;
                wr32(cand.wrapping_add(CAND_FLAG_OFF), q);
                global::<u32>(COUNT_GLOBAL).write(0);
            }
        }
        wr32(this.wrapping_add(FOLLOWER_OFF), cand);
        ret
    }
});
