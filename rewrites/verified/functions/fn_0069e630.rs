// original: 0x0069E630 record_refresh
//! Record refresh for one files-memory table entry.
//!
//! Rebuilds the entry object at `this` for table row `index`: a mapping of
//! the four live rows is built on the stack and handed to the row loader
//! through its data slot; a failed load clears the entry instead. On
//! success the loaded words are decoded into the entry's color bytes and
//! flag bits, with an optional second load and a trailing notify call.

use lf_checker_rt::{export, global, relocated};

const F3_ENABLE: u32 = 0x018B8030;
const F3_FLAG0: u32 = 0x018B7E48;
const F3_FLAG1: u32 = 0x018B7EE0;
const F3_FLAG2: u32 = 0x018B7F78;
const F3_FLAG3: u32 = 0x018B8010;
const F3_TABLE: u32 = 0x018B7DC8;
const F3_SLOT1: u32 = 0x018B7DC0;
const F3_SLOT2: u32 = 0x018B8028;
const F3_SLOT3: u32 = 0x018B802C;

/// Record refresh for one files-memory table entry (see module docs).
export!(thiscall, rw_69e630(this: u32, index: u32, flag: u32) -> u32 {
    unsafe {
        let esi = this as *mut u8;
        *(esi.add(8) as *mut u32) = *(esi.add(4) as *mut u32);
        *(esi.add(4) as *mut u32) = 0;
        *(esi.add(0x0C) as *mut u32) = 0x80808080;
        *(esi.add(0x10) as *mut u64) = 0;
        *(esi.add(0x18) as *mut u32) = 0;
        *(esi.add(0x1C) as *mut u32) = 0x02000200;
        *(esi.add(0x20) as *mut u32) = 0x02000200;
        if *global::<u32>(F3_ENABLE) == 0 {
            return 0x02000000;
        }
        if (flag & 0xFF) != 0 {
            return 0x02000000;
        }
        // Mapping of live rows; padded with zeros exactly like the
        // original's zero-filled stack scratch past the four slots.
        let mut map = [0u32; 8];
        let mut edi = 0u32;
        map[0] = 0xFFFFFFFF;
        if *global::<u8>(F3_FLAG0) != 0 {
            map[0] = edi;
            edi = 1;
        }
        map[1] = 0xFFFFFFFF;
        if *global::<u8>(F3_FLAG1) != 0 {
            map[1] = edi;
            edi += 1;
        }
        map[2] = 0xFFFFFFFF;
        if *global::<u8>(F3_FLAG2) != 0 {
            map[2] = edi;
            edi += 1;
        }
        map[3] = 0xFFFFFFFF;
        if *global::<u8>(F3_FLAG3) != 0 {
            map[3] = edi;
            edi += 1;
        }
        let mval = *((map.as_ptr() as *const u8)
            .wrapping_byte_add((index as usize).wrapping_mul(4))
            as *const i32);
        let rec = index
            .wrapping_mul(0x98)
            .wrapping_add(relocated(F3_TABLE));
        if mval >= 0 {
            edi = mval as u32;
        }
        type Load = extern "stdcall" fn(u32, u32) -> u32;
        let load: Load = core::mem::transmute(
            (*global::<u32>(F3_SLOT1)) as usize,
        );
        let r1 = load(edi, map.as_mut_ptr() as u32);
        if r1 != 0 {
            *esi.add(0x24) = 0;
            *((rec.wrapping_add(0x81)) as *mut u8) = 0;
            return r1;
        }
        if *esi.add(0x24) == 0
            && *((rec.wrapping_add(0x81)) as *const u8) == 0
        {
            let mut buf2 = [0u32; 1];
            type Load2 = extern "stdcall" fn(u32, u32, u32) -> u32;
            let load2: Load2 = core::mem::transmute(
                (*global::<u32>(F3_SLOT2)) as usize,
            );
            let r2 = load2(edi, 1, buf2.as_mut_ptr() as u32);
            if r2 == 0 && *((buf2.as_ptr() as *const u8).add(1)) == 2 {
                *((rec.wrapping_add(0x81)) as *mut u8) = 1;
            }
        }
        // Decode the loaded words (the stub refilled `map`).
        let rb = map.as_ptr() as *const u8;
        let rd = |off: usize| -> u32 {
            u32::from_le_bytes([
                *rb.add(off),
                *rb.add(off + 1),
                *rb.add(off + 2),
                *rb.add(off + 3),
            ])
        };
        *esi.add(0x24) = 1;
        *esi.add(0x0C) = ((rd(8) >> 8) as u8).wrapping_sub(0x80);
        *esi.add(0x0D) = !((rd(10) >> 8) as u8).wrapping_sub(0x80);
        *esi.add(0x0E) = ((rd(12) >> 8) as u8).wrapping_sub(0x80);
        *esi.add(0x0F) = !((rd(14) >> 8) as u8).wrapping_sub(0x80);
        let w = rd(4);
        let mut f4 = 0u32;
        if w & 0x1000 != 0 {
            f4 |= 0x40;
        }
        if w & 0x2000 != 0 {
            f4 |= 0x20;
        }
        if w & 0x4000 != 0 {
            f4 |= 0x80;
        }
        if w & 0x8000 != 0 {
            f4 |= 0x10;
        }
        if w & 0x100 != 0 {
            f4 |= 4;
        }
        if w & 0x200 != 0 {
            f4 |= 8;
        }
        let cl = *rb.add(6);
        if cl >= 0x40 {
            f4 |= 1;
        }
        let dl = *rb.add(7);
        if dl >= 0x40 {
            f4 |= 2;
        }
        let al = w as u8;
        if al & 0x10 != 0 {
            f4 |= 0x800;
        }
        if al & 0x20 != 0 {
            f4 |= 0x100;
        }
        if al & 0x40 != 0 {
            f4 |= 0x200;
        }
        if al & 0x80 != 0 {
            f4 |= 0x400;
        }
        if al & 1 != 0 {
            f4 |= 0x1000;
        }
        if al & 2 != 0 {
            f4 |= 0x4000;
        }
        if al & 4 != 0 {
            f4 |= 0x8000;
        }
        if al & 8 != 0 {
            f4 |= 0x2000;
        }
        *(esi.add(4) as *mut u32) = f4;
        *esi.add(0x1A) = cl;
        *esi.add(0x1B) = dl;
        if *esi.add(0x26) == 0 {
            return w;
        }
        let c25 = *esi.add(0x25);
        let ax = if c25 != 0 {
            *(esi.add(0x32) as *const u16)
        } else {
            0
        };
        let ax2 = if c25 != 0 {
            *(esi.add(0x34) as *const u16)
        } else {
            0
        };
        // The original parks these words in its incoming arg1 slot (both
        // words packed into the one slot, the next word left at the zero
        // fill); the values are verified through the call snapshot.
        let mut buf3 = [0u32; 2];
        buf3[0] = ((ax2 as u32) << 16) | (ax as u32);
        buf3[1] = 0;
        type Notify = extern "stdcall" fn(u32, u32) -> u32;
        let notify: Notify = core::mem::transmute(
            (*global::<u32>(F3_SLOT3)) as usize,
        );
        let r3 = notify(edi, buf3.as_mut_ptr() as u32);
        if r3 == 0 {
            *esi.add(0x26) = 0;
        }
        r3
    }
});
