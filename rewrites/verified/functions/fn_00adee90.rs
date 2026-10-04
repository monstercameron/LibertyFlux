// original: 0x00ADEE90 input_commit
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Master enable flag for the input subsystem.
const ENABLED: u32 = 0x1593311;
/// Selects the pre-pass over the pending list before the commit.
const PREPASS: u32 = 0x103F67D;
/// Index of the currently active pending-list set.
const ACTIVE_SET: u32 = 0x1174794;
/// Table of class descriptors indexed by event class id.
const CLASS_TABLE: u32 = 0x1295CD8;
/// Range ordering predicate passed to the range fixer.
const COMPARE_FN: u32 = 0xAE0CF0;

const SLOTS: u32 = 24;
const SLOT_STRIDE: u32 = 0x48;

///
/// Clears the edge latches of every slot, drains the active set's pending
/// event list through the dispatcher (callee 2, one call per event plus one
/// per set flag word), then re-sorts every slot's ranges (callee 3).
/// Runs the pre-pass (callee 1) first when its flag is set, and returns
/// immediately when the subsystem is disabled.
///
/// The original's exit EAX is its last callee answer on the main path and
/// the unobservable entry EAX on the early-out path, so the contract does
/// not compare the return value; this rewrite returns the last answer, or
/// zero on the early-out path.
// original: 0x00ADEE90 input_commit
export!(thiscall, rw_b198_f1(this: u32, mask_a: u32, mask_b: u32, mask_c: u32, mask_d: u32) -> u32 {
    unsafe {
        if global::<u8>(ENABLED).read() == 0 {
            return 0;
        }
        if global::<u8>(PREPASS).read() != 0 {
            callee_thiscall!(1, u32, this);
        }
        // Pass 1: clear the edge latches of the 24 slots.
        let mut p = this.wrapping_add(0x146);
        for _ in 0..SLOTS {
            let mut k = 0u32;
            while k <= 0x40 {
                let latch = (p.wrapping_add(k)) as *mut u16;
                if latch.read() == 0 {
                    latch.write(0);
                    (p.wrapping_add(k).wrapping_sub(6) as *mut u32).write(0);
                }
                (p.wrapping_add(k).wrapping_sub(2) as *mut u16).write(0);
                k += 8;
            }
            p = p.wrapping_add(SLOT_STRIDE);
        }
        // Drain the active set's pending list; the list head is consumed.
        let set = global::<u32>(ACTIVE_SET).read();
        let base = this.wrapping_add(set.wrapping_mul(4));
        let end_slot = (base.wrapping_add(0x14)) as *mut u32;
        let cur_slot = (base.wrapping_add(0x1c)) as *mut u32;
        let end = end_slot.read();
        let mut cur = cur_slot.read();
        cur_slot.write(0);
        end_slot.write(0);
        while cur < end {
            let flags_hi = ((cur.wrapping_add(0x2a)) as *const u8).read();
            let mut code = ((cur.wrapping_add(0x20)) as *const u8).read() as u32;
            if flags_hi & 0x10 != 0 {
                code = 0xFF;
            } else if ((cur.wrapping_add(0x29)) as *const u8).read() & 2 != 0 {
                code = ((cur.wrapping_add(0x21)) as *const u8).read() as u32;
            }
            if code != 0 {
                let mut value = ((cur.wrapping_add(8)) as *const u32).read();
                if ((cur.wrapping_add(0x29)) as *const u8).read() & 1 != 0 {
                    value &= !mask_d;
                }
                if value != 0 {
                    let class = ((cur.wrapping_add(0x1a)) as *const u16).read() as u32;
                    let desc = ((relocated(CLASS_TABLE).wrapping_add(class.wrapping_mul(4)))
                        as *const u32)
                        .read();
                    if ((desc.wrapping_add(0x40)) as *const u32).read() & 0x100 != 0 {
                        callee_thiscall!(2, u32, this, cur, value, 7);
                    } else if code < 0xFF {
                        callee_thiscall!(2, u32, this, cur, value & mask_a, 4);
                    } else {
                        let flags = ((cur.wrapping_add(0x28)) as *const u8).read();
                        if flags & 0x40 != 0 {
                            callee_thiscall!(2, u32, this, cur, value & mask_a, 6);
                        }
                        if flags & 0x10 != 0 {
                            callee_thiscall!(2, u32, this, cur, value & mask_c, 3);
                        }
                        if flags & 8 != 0 {
                            callee_thiscall!(2, u32, this, cur, value & mask_b, 2);
                        }
                        if flags & 0x20 != 0 {
                            callee_thiscall!(2, u32, this, cur, value & mask_a, 5);
                        }
                        let tag =
                            if ((cur.wrapping_add(0x29)) as *const u8).read() & 0x0C != 0 {
                                1
                            } else {
                                0
                            };
                        callee_thiscall!(2, u32, this, cur, value, tag);
                    }
                }
            }
            cur = cur.wrapping_add(((cur.wrapping_add(0x18)) as *const u16).read() as u32);
        }
        // Pass 2: re-sort every slot's four ranges.
        let cmp = relocated(COMPARE_FN);
        let mut q = this.wrapping_add(0x164);
        let mut answer = 0u32;
        for _ in 0..SLOTS {
            let b0 = ((q.wrapping_add(0x0c)) as *const u32).read();
            let n0 = ((q.wrapping_add(0x10)) as *const u16).read() as u32;
            answer = callee_cdecl!(3, u32, b0, b0.wrapping_add(n0.wrapping_mul(4)), cmp);
            let b1 = ((q.wrapping_sub(4)) as *const u32).read();
            let n1 = (q as *const u16).read() as u32;
            answer = callee_cdecl!(3, u32, b1, b1.wrapping_add(n1.wrapping_mul(4)), cmp);
            let b2 = ((q.wrapping_add(4)) as *const u32).read();
            let n2 = ((q.wrapping_add(8)) as *const u16).read() as u32;
            answer = callee_cdecl!(3, u32, b2, b2.wrapping_add(n2.wrapping_mul(4)), cmp);
            let b3 = ((q.wrapping_add(0x14)) as *const u32).read();
            let n3 = ((q.wrapping_add(0x18)) as *const u16).read() as u32;
            answer = callee_cdecl!(3, u32, b3, b3.wrapping_add(n3.wrapping_mul(4)), cmp);
            q = q.wrapping_add(SLOT_STRIDE);
        }
        answer
    }
});

