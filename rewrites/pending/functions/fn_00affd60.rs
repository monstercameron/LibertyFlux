// original: 0x00AFFD60 model_gate_spawn (proposed)
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Gate a spawn pass on the model's membership, then run it.
///
/// Returns 0 when the gate callee rejects the tag. Otherwise checks whether
/// the object's model id appears in the global id table (or equals the
/// special id), derives a clamped attempt budget from the level byte and the
/// two integer arguments, grows the low attempt bound by random draws, caps
/// the budget at one for flagged models, and runs one spawn round per budget
/// slot. The low byte of the return is always 1 on this path; the upper
/// bytes are the last callee answer (or flag word) left in EAX.
export!(cdecl, rw_affd60(obj: u32, lo: i32, hi: i32, tag: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x0169E380;
        const ID_TABLE: u32 = 0x0169DBF8;
        const SPECIAL_ID: u32 = 0x012F9FE4;
        const MODEL_TABLE: u32 = 0x01295CD8;
        const THRESH_GLOB: u32 = 0x0103FFB8;
        const SCALE_GLOB: u32 = 0x00FE8684;
        const FIXED_OBJ: u32 = 0x01908EF0;

        let gate: u32 = callee_thiscall!(1, u32, obj, tag);
        if gate == 0 {
            return 0;
        }
        let model: i32 = ((obj + 0x2E) as *const u16).read_unaligned() as i16 as i32;
        let n: i32 = global::<i32>(COUNT).read();
        let mut member = false;
        if n > 0 {
            let mut i: i32 = 0;
            while i < n {
                let w = (global::<u8>(ID_TABLE).add((i as u32 * 2) as usize) as *const u16)
                    .read_unaligned() as i32;
                if w == model {
                    member = true;
                    break;
                }
                i += 1;
            }
        }
        if !member {
            let special: i32 = global::<i32>(SPECIAL_ID).read();
            if model == special {
                member = true;
            }
        }
        let level: u32 = ((obj + 0x1070) as *const u8).read() as u32;
        let (bound, mut low): (i32, i32) = if member {
            if ((obj + 0x1474) as *const u8).read() & 1 == 0 {
                (level.wrapping_sub(1) as i32, 1)
            } else {
                (0, lo)
            }
        } else {
            (level as i32, lo)
        };
        let mut budget: i32 = hi;
        if bound < budget {
            budget = bound;
        }
        if low < budget {
            let mut left = (budget.wrapping_sub(low)) as u32;
            let thresh: f32 = global::<f32>(THRESH_GLOB).read();
            let scale: f32 = global::<f32>(SCALE_GLOB).read();
            loop {
                let r: u32 = callee_cdecl!(2, u32,);
                let probe = (r as i32) as f32 * scale;
                if thresh > probe {
                    low = low.wrapping_add(1);
                }
                left = left.wrapping_sub(1);
                if left == 0 {
                    break;
                }
            }
        }
        if low < budget {
            budget = low;
        }
        let tbl = global::<u8>(MODEL_TABLE) as u32;
        let ent_addr = tbl.wrapping_add((model as u32).wrapping_mul(4));
        let entry: u32 = (ent_addr as *const u32).read_unaligned();
        let flags = (entry.wrapping_add(0x94) as *const u32).read_unaligned();
        let mut residue: u32 = flags >> 5;
        if (residue as u8) & 1 == 1 {
            let mut cap: i32 = 1;
            if budget < cap {
                cap = budget;
            }
            budget = cap;
            residue = cap as u32;
        }
        if budget > 0 {
            let mut s: i32 = 0;
            while s < budget {
                let slot: u32 = if member { (s + 1) as u32 } else { s as u32 };
                let seen: u32 = callee_cdecl!(3, u32,);
                if seen != 0 {
                    let ready: u32 = callee_thiscall!(4, u32, relocated(FIXED_OBJ), 1, 0);
                    residue = ready;
                    if ready == 0 {
                        s += 1;
                        continue;
                    }
                }
                let node: u32 = callee_thiscall!(5, u32, obj, slot);
                residue = node;
                if node != 0 {
                    let done: u32 = callee_thiscall!(6, u32, node);
                    residue = done;
                }
                s += 1;
            }
        }
        (residue & 0xFFFF_FF00) | 1
    }
});
