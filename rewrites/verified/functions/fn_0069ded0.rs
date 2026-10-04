// original: 0x0069DED0 table_entry_refresh
//! Table-driven refresh of one files-memory entry.
//!
//! Rebuilds the entry object at `this` from table row `[this]` when the row
//! is live: live rows with the flag set are forwarded to the record loader,
//! otherwise the row's fields are decoded into the entry's color bytes and
//! flag bits through the type, range and frequency ladders below. Rows past
//! the live count take the append path instead, and the entry's trailing
//! words select an optional finalizer call.

use lf_checker_rt::{callee_addr, export, global, relocated};

const F2_COUNT: u32 = 0x018B7DC4;
const F2_ENABLE: u32 = 0x018B8030;
const F2_TABLE: u32 = 0x018B7DC8;
const F2_ROWFLAG: u32 = 0x018B7E48;
const F2_SLOT: u32 = 0x018B802C;
const F2_MASKTAB: u32 = 0x01151F60;

/// Table-driven refresh of one files-memory entry (see module docs).
export!(thiscall, rw_69ded0(this: u32, arg0: u32) -> u32 {
    unsafe {
        let esi = this as *mut u8;
        let rd = |off: usize| -> u32 { *(esi.add(off) as *const u32) };
        *(esi.add(8) as *mut u32) = rd(4);
        *(esi.add(4) as *mut u32) = 0;
        *(esi.add(0x0C) as *mut u32) = 0x80808080;
        *(esi.add(0x10) as *mut u64) = 0;
        *(esi.add(0x18) as *mut u32) = 0;
        *(esi.add(0x1C) as *mut u32) = 0x02000200;
        *(esi.add(0x20) as *mut u32) = 0x02000200;
        let count = *global::<i32>(F2_COUNT);
        let index = rd(0) as i32;
        let mut eax = arg0;
        if count <= index {
            // At or past the end: only the exact end appends.
            if count != index {
                return eax;
            }
            if *global::<u32>(F2_ENABLE) == 0 {
                return eax;
            }
            let before = *esi.add(0x24);
            type LoadRec = extern "thiscall" fn(u32, u32, u32) -> u32;
            let load: LoadRec =
                core::mem::transmute(callee_addr(1) as usize);
            let r = load(this, index as u32, arg0);
            eax = (r & 0xFFFFFF00) | (*esi.add(0x24) as u32);
            let after = *esi.add(0x24);
            if after == 0 || after == before {
                return eax;
            }
            let slot = (index as u32).wrapping_mul(0x98);
            *global::<i32>(F2_COUNT) = count.wrapping_add(1);
            *((relocated(F2_ROWFLAG).wrapping_add(slot)) as *mut u8) = 1;
            return slot;
        }
        if (arg0 & 0xFF) != 0 {
            // Flag path: count the live rows, notify, maybe finalize.
            if *global::<u32>(F2_ENABLE) == 0 {
                return eax;
            }
            eax = (index as u32).wrapping_mul(0x98);
            if *((relocated(F2_ROWFLAG).wrapping_add(eax)) as *const u8)
                == 0
            {
                return eax;
            }
            if *esi.add(0x26) != 0 {
                let mut n = 0u32;
                if index >= 0 {
                    let mut cur = relocated(F2_ROWFLAG);
                    for _ in 0..(index as u32).wrapping_add(1) {
                        if *(cur as *const u8) != 0 {
                            n += 1;
                        }
                        cur = cur.wrapping_add(0x98);
                    }
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
                // The original packs these into its incoming arg slot;
                // verified through the call snapshot like fn3's buffer.
                let mut buf = [(((ax2 as u32) << 16) | (ax as u32))];
                type Notify = extern "stdcall" fn(u32, u32) -> u32;
                let notify: Notify = core::mem::transmute(
                    (*global::<u32>(F2_SLOT)) as usize,
                );
                eax = notify(n.wrapping_sub(1), buf.as_mut_ptr() as u32);
                if eax == 0 {
                    *esi.add(0x26) = 0;
                }
            }
            if *(esi.add(0x32) as *const u16) == 0
                && *(esi.add(0x34) as *const u16) == 0
            {
                return eax;
            }
            type Fin = extern "thiscall" fn(u32) -> u32;
            let fin: Fin =
                core::mem::transmute(callee_addr(3) as usize);
            return fin(this);
        }
        // Main decode of the record at row `index`.
        let rec = (index as u32)
            .wrapping_mul(0x98)
            .wrapping_add(relocated(F2_TABLE));
        let rf = |off: u32| -> u32 {
            *(rec.wrapping_add(off) as *const u32)
        };
        if *global::<u32>(F2_ENABLE) != 0
            && *((rec.wrapping_add(0x80)) as *const u8) != 0
        {
            type LoadRec = extern "thiscall" fn(u32, u32, u32) -> u32;
            let load: LoadRec =
                core::mem::transmute(callee_addr(1) as usize);
            return load(this, index as u32, 0);
        }
        *esi.add(0x0C) = ((rf(0x0C) >> 8) as u8) as u8;
        *esi.add(0x0D) = ((rf(0x10) >> 8) as u8) as u8;
        // NOTE: sar (arithmetic) then take al == logical >> 8 then take al.
        let mut f4 = 0u32;
        let ty = rf(0x7C);
        if rf(0) < 4 {
            if rf(8) != 0 {
                let cx = *((rec.wrapping_add(0x74)) as *const u16);
                *(esi.add(0x0E) as *mut u16) = 0x8080;
                if cx == 0 {
                    *esi.add(0x0F) = 0;
                } else if cx == 0x2328 {
                    *esi.add(0x0E) = 0xFF;
                } else if cx == 0x4650 {
                    *esi.add(0x0F) = 0xFF;
                } else {
                    if cx == 0x6978 {
                        *esi.add(0x0E) = 0;
                    }
                }
            }
        } else if ty == 6 {
            *esi.add(0x0E) = ((rf(0x24) >> 8) as u8) as u8;
            *esi.add(0x0F) = ((rf(0x14) >> 8) as u8) as u8;
        } else if ty == 2 || ty == 4 || ty == 5 || ty == 9 || ty == 0x0D
            || ty == 0x12 || ty == 0x0B
        {
            *esi.add(0x0E) = ((rf(0x14) >> 8) as u8) as u8;
            *esi.add(0x0F) = ((rf(0x20) >> 8) as u8) as u8;
        } else if ty == 8 {
            *esi.add(0x0E) = ((rf(0x20) >> 8) as u8) as u8;
            *esi.add(0x0F) = ((rf(0x24) >> 8) as u8) as u8;
        } else if ty == 0x0A || ty == 0x0C || ty == 0x14 {
            *esi.add(0x0E) = ((rf(0x18) >> 8) as u8) as u8;
            *esi.add(0x0F) = ((rf(0x1C) >> 8) as u8) as u8;
        } else if ty == 0x0E || ty == 0x15 || ty == 0x16 {
            *esi.add(0x0D) = 0x7F;
            *esi.add(0x0E) = 0x7F;
            *esi.add(0x0F) = 0x7F;
        } else if ty == 7 || ty == 0x0F || ty == 0x10 || ty == 0x13
            || ty == 0x11 || ty == 0x17
        {
            *esi.add(0x0E) = ((rf(0x20) >> 8) as u8) as u8;
            *esi.add(0x0F) = ((rf(0x14) >> 8) as u8) as u8;
            if ty == 0x0F || ty == 0x10 {
                if (((rf(0x24) >> 8) as u8) as u8) != 0 {
                    *esi.add(0x0F) = 0x7F;
                }
            }
        } else {
            *esi.add(0x0E) = ((rf(0x14) >> 8) as u8) as u8;
            *esi.add(0x0F) = ((rf(0x18) >> 8) as u8) as u8;
        }
        if ty == 0x0C || ty == 0x0E || ty == 0x12 || ty == 0x13
            || ty == 0x14
        {
            // Range ladder on the frequency word.
            let cx = *((rec.wrapping_add(0x74)) as *const u16);
            if (cx as i16) >= 0 {
                if (cx as i16) < 0x2328
                    || (cx as i16) > 0x6978
                {
                    f4 |= 0x1000;
                }
                if (((cx.wrapping_sub(1)) as u16) as u32)
                    <= (0x464E - 0)
                {
                    f4 |= 0x2000;
                }
                if (cx.wrapping_sub(0x2329) as u32) <= 0x464E {
                    f4 |= 0x4000;
                }
                if (cx as i16) > 0x4650 {
                    let se = cx as i16 as i32;
                    if se < 0x8CA0 {
                        f4 |= 0x8000;
                    }
                }
            }
        } else if rf(8) != 0 {
            let c4 = rf(4);
            if c4.wrapping_add(4) < 0x20 {
                let ax = *((rec.wrapping_add(0x74)) as *const u16);
                if ax == 0 {
                    f4 |= 1u32.wrapping_shl(c4);
                } else if ax == 0x1194 {
                    f4 |= 1u32.wrapping_shl(c4);
                    f4 |= 1u32.wrapping_shl(c4.wrapping_add(1));
                } else if ax == 0x2328 {
                    f4 |= 1u32.wrapping_shl(c4.wrapping_add(1));
                } else if ax == 0x34BC {
                    f4 |= 1u32.wrapping_shl(c4.wrapping_add(1));
                    f4 |= 1u32.wrapping_shl(c4.wrapping_add(2));
                } else if ax == 0x4650 {
                    f4 |= 1u32.wrapping_shl(c4.wrapping_add(2));
                } else if ax == 0x57E4 {
                    f4 |= 1u32.wrapping_shl(c4.wrapping_add(2));
                    f4 |= 1u32.wrapping_shl(c4.wrapping_add(3));
                } else if ax == 0x6978 {
                    f4 |= 1u32.wrapping_shl(c4.wrapping_add(3));
                } else if ax == 0x7B0C {
                    f4 |= 1u32.wrapping_shl(c4.wrapping_add(3));
                    f4 |= 1u32.wrapping_shl(c4);
                }
            }
        }
        if ty == 5 || ty == 6 {
            // Frequency ladder A.
            let cx = *((rec.wrapping_add(0x74)) as *const u16);
            if cx == 0xFFFF {
                eax = ty;
            } else {
                let a1 = cx.wrapping_sub(0x2328);
                if (a1 as u32) > 0x4650 {
                    f4 |= 0x1000;
                } else {
                    let a2 = cx.wrapping_sub(0x2329);
                    if (a2 as u32) <= 0x464E {
                        f4 |= 0x4000;
                    }
                }
                let a3 = cx.wrapping_sub(1);
                if (a3 as u32) <= 0x464E {
                    f4 |= 0x2000;
                } else if (cx as i16) > 0x4650 {
                    f4 |= 0x8000;
                }
                eax = (cx as u32).wrapping_sub(1);
            }
        } else if ty == 0x0B {
            if (rf(0x28) as i32) > 0x7FFF {
                f4 |= 1;
            }
            if (rf(0x24) as i32) > 0x7FFF {
                f4 |= 2;
            }
            eax = ty;
        } else if ty == 0x0C {
            if (rf(0x14) as i32) > 0x7FFF {
                f4 |= 1;
            }
            if (rf(0x14) as i32) < 0x7FFF {
                f4 |= 2;
            }
            let d = 0x7FFFu32.wrapping_sub(rf(0x14));
            let c = if (d as i32) > 0 { d } else { 0 };
            *esi.add(0x1B) = ((c >> 7) as u8) as u8;
            let e = (rf(0x14) as i32).wrapping_sub(0x7FFF);
            eax = if e < 0 {
                *esi.add(0x1A) = 0;
                0
            } else {
                let g = if e > 0x7FFF { 0x7FFF } else { e };
                let s = (g as u32) >> 7;
                *esi.add(0x1A) = (s as u8) as u8;
                s
            };
        } else if ty == 0x0E {
            let c = 0xFFFFu32;
            let a1 = c.wrapping_sub(rf(0x10));
            let a1 = if a1 > c { c } else { a1 };
            *esi.add(0x1B) = ((a1 >> 7) as u8) as u8;
            let a2 = c.wrapping_sub(rf(0x20));
            let a2 = if a2 > c { c } else { a2 };
            *esi.add(0x1A) = ((a2 >> 7) as u8) as u8;
            eax = a2 >> 7;
        } else if ty == 0x15 {
            let c = 0xFFFFu32;
            let a1 = c.wrapping_sub(rf(0x24));
            let a1 = if a1 > c { c } else { a1 };
            *esi.add(0x1B) = ((a1 >> 7) as u8) as u8;
            let a2 = c.wrapping_sub(rf(0x10));
            let a2 = if a2 > c { c } else { a2 };
            *esi.add(0x1A) = ((a2 >> 7) as u8) as u8;
            eax = a2 >> 7;
        } else if ty == 0x16 {
            let c = 0xFFFFu32;
            let a1 = c.wrapping_sub(rf(0x10));
            let a1 = if a1 > c { c } else { a1 };
            *esi.add(0x1B) = ((a1 >> 7) as u8) as u8;
            let a2 = c.wrapping_sub(rf(0x14));
            let a2 = if a2 > c { c } else { a2 };
            *esi.add(0x1A) = ((a2 >> 7) as u8) as u8;
            eax = a2 >> 7;
        } else if ty == 0x17 {
            let a = rf(0x18) as i32;
            if a < 0x4E20 {
                f4 |= 0x1000;
            } else {
                let c = rf(0x24) as i32;
                if c > 0x9C40 {
                    f4 |= 0x2000;
                } else if a > 0x9C40 {
                    f4 |= 0x4000;
                } else if c < 0x4E20 {
                    f4 |= 0x8000;
                }
            }
            eax = rf(0x18);
        } else if rf(0) == 6 {
            let a = rf(0x28) as i32;
            if a < 0x4E20 {
                f4 |= 0x8000;
            } else if a > 0x9C40 {
                f4 |= 0x2000;
            }
            let b = rf(0x24) as i32;
            if b < 0x4E20 {
                f4 |= 0x1000;
            } else if b > 0x9C40 {
                f4 |= 0x4000;
            }
            eax = rf(0x24);
        } else {
            eax = ty;
        }
        // Mask-table fold over the row's group bits.
        let groups = rf(0x6C);
        let mut bit = 1u32;
        for slot in [0u32, 4, 8, 12] {
            for k in 0..4u32 {
                if groups & bit != 0 {
                    let idx = ty
                        .wrapping_mul(16)
                        .wrapping_add(slot);
                    let addr = relocated(F2_MASKTAB)
                        .wrapping_add(4u32.wrapping_mul(k))
                        .wrapping_add(idx.wrapping_mul(4));
                    eax = *(addr as *const u32);
                    f4 |= eax;
                }
                bit = bit.rotate_left(1);
            }
        }
        *(esi.add(4) as *mut u32) = f4;
        eax
    }
});
