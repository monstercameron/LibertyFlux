// original: 0x00c6e970 nm_message_registry_add
use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

/// Registry table pointer: .data word holding the entry-pointer array base.
const REG_TABLE: u32 = 0x16dd668;
/// Registry entry count: .data word just past the table pointer.
const REG_COUNT: u32 = 0x16dd66c;
/// Registry capacity: .data word past the count.
const REG_CAP: u32 = 0x16dd66e;
/// Capacity growth step, in entries, when the table is full.
const GROW_STEP: u16 = 0x10;
/// Parameter ids registered for every new entry, in call order.
const PARAM_IDS: [u32; 14] = [
    0x4b5, 0x1a1, 0x4b2, 0x4b3, 0x36a0, 0x36a1, 0x4c2, 0x4c9, 0x4c3, 0x4d0, 0x1a3, 0x1a8, 0x1a4,
    0x1a9,
];
/// Default parameter weight passed to the setter (1.0f as bits).
const DEFAULT_WEIGHT_BITS: u32 = 0x3f800000;

// Add the input's message key to the behaviour registry, or find it.
//
// Searches the global entry table for a record whose two key words match
// the input's key words (at +0x16/+0x18) and returns the first key word on
// a hit. On a miss, allocates a record plus its parameter object, registers
// the fourteen parameters, appends the record (growing the table by
// GROW_STEP entries when full) and returns the previous count.
export!(cdecl, rw_c6e970(input: u32) -> u32 {
    unsafe {
        let key_a = (input as *const u16).byte_add(0x16).read_unaligned() as u32;
        let key_b = (input as *const u16).byte_add(0x18).read_unaligned() as u32;
        let count = *global::<u16>(REG_COUNT);
        if (count as i16) > 0 {
            let table = *global::<u32>(REG_TABLE);
            let mut i: u32 = 0;
            while i < count as u32 {
                let e = ((table.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
                if e != 0
                    && (e as *const u32).read_unaligned() == key_b
                    && ((e.wrapping_add(4)) as *const u32).read_unaligned() == key_a
                {
                    return key_a;
                }
                i = i.wrapping_add(1);
            }
        }
        let mut rec: u32 = 0;
        let fresh = callee_cdecl!(1, u32, 12u32);
        if fresh != 0 {
            (fresh as *mut u32).write_unaligned(key_b);
            (fresh.wrapping_add(4) as *mut u32).write_unaligned(key_a);
            let obj = callee_cdecl!(2, u32, 0x18u32);
            let init = if obj == 0 {
                0
            } else {
                callee_thiscall!(4, u32, obj, input)
            };
            (fresh.wrapping_add(8) as *mut u32).write_unaligned(init);
            for id in PARAM_IDS {
                let v = callee_cdecl!(5, u32, input, id);
                callee_thiscall!(6, u32, init, v, DEFAULT_WEIGHT_BITS, 0u32);
            }
            rec = fresh;
        }
        let n = *global::<u16>(REG_COUNT);
        let cap = *global::<u16>(REG_CAP);
        if n == cap {
            let ncap = cap.wrapping_add(GROW_STEP);
            *global::<u16>(REG_CAP) = ncap;
            let new_table = callee_cdecl!(3, u32, (ncap as u32).wrapping_mul(4));
            let old_table = *global::<u32>(REG_TABLE);
            let mut j: u32 = 0;
            while j < n as u32 {
                let w =
                    ((old_table.wrapping_add(j.wrapping_mul(4))) as *const u32).read_unaligned();
                ((new_table.wrapping_add(j.wrapping_mul(4))) as *mut u32).write_unaligned(w);
                j = j.wrapping_add(1);
            }
            callee_cdecl!(7, u32, *global::<u32>(REG_TABLE));
            *global::<u32>(REG_TABLE) = new_table;
            *global::<u16>(REG_COUNT) = n.wrapping_add(1);
            ((new_table.wrapping_add((n as u32).wrapping_mul(4))) as *mut u32)
                .write_unaligned(rec);
        } else {
            let table = *global::<u32>(REG_TABLE);
            *global::<u16>(REG_COUNT) = n.wrapping_add(1);
            ((table.wrapping_add((n as u32).wrapping_mul(4))) as *mut u32).write_unaligned(rec);
        }
        n as u32
    }
});
