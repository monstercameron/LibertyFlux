// original: 0x00bf3d30 dispatch_and_propagate_rows
/// Fetch a helper record through two dispatch stages, then propagate its
/// rows into a bit array and a per-row level value.
///
/// `arg0` supplies an object whose table slot 0xa0 yields a working record;
/// a null record returns zero. A flag byte on `this` sets or clears one bit
/// in the record header. A second flag byte and the record's link word select
/// one of four continuations: an alternate table slot call, a session call
/// through a global object, or nothing. The record's link word, when nonzero,
/// locates a target block 0x100 bytes past it; a helper call on the record
/// then yields the source record (a null answer returns zero).
///
/// The source record carries a length byte and a table of row pointers. Each
/// row carries a selector byte. For every row below the length, a one-bit
/// mask chosen by the row index is tested against two mask pairs carried on
/// `this`: the first test sets or clears the selector's bit in the
/// target bit array (or skips that when there is no target block), and the
/// second test writes a negative or positive unit level for that selector
/// (into the target block, or into the working record when there is no
/// target). Row indexes past the mask width match nothing. Returns the
/// length byte after the loop, the source record when the length is zero.
export!(thiscall, rw_00bf3d30(this_: *mut u8, arg0: *mut u8) -> u32 {
    const HEADER_FLAG_BIT: u32 = 2;
    const NEG_UNIT_BITS: u32 = 0xBF80_0000;
    const POS_UNIT_BITS: u32 = 0x3F80_0000;
    const SESSION_GLOBAL: u32 = 0x018B_7448;
    unsafe {
        let table = (arg0 as *const u32).read_unaligned();
        let fetch_slot = ((table.wrapping_add(0xA0)) as *const u32).read_unaligned();
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(fetch_slot as usize);
        let record = fetch(arg0 as u32);
        if record == 0 {
            return 0;
        }
        let header = (record.wrapping_add(0x60)) as *mut u32;
        if *this_.add(2) != 0 {
            header.write_unaligned(header.read_unaligned() | HEADER_FLAG_BIT);
        } else {
            header.write_unaligned(header.read_unaligned() & !HEADER_FLAG_BIT);
        }
        let link = ((record.wrapping_add(0x64)) as *const u32).read_unaligned();
        let use_alt = *this_.add(1) != 0;
        if use_alt {
            if link == 0 {
                let record_table = (record as *const u32).read_unaligned();
                let alt = ((record_table.wrapping_add(0xA8)) as *const u32)
                    .read_unaligned();
                let alt_fn: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(alt as usize);
                let _ = alt_fn(record);
            }
        } else if link != 0 {
            let outer =
                lf_checker_rt::global::<u32>(SESSION_GLOBAL).read_unaligned();
            let session = ((outer.wrapping_add(0x2F0)) as *const u32)
                .read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(3, u32, session, link);
        }
        let mut target: u32 = 0;
        let link_now = ((record.wrapping_add(0x64)) as *const u32).read_unaligned();
        if link_now != 0 {
            target = link_now.wrapping_add(0x100);
        }
        let source = lf_checker_rt::callee_thiscall!(4, u32, record);
        if source == 0 {
            return 0;
        }
        let bound = *((source.wrapping_add(0x1F3)) as *const u8);
        if bound == 0 {
            return source;
        }
        let row_table =
            ((source.wrapping_add(0xD4)) as *const u32).read_unaligned();
        let mut index: u32 = 0;
        let mut high: u32 = 0;
        loop {
            let row = (((row_table.wrapping_add(index.wrapping_mul(4)))
                as *const u32)
                .read_unaligned());
            let selector =
                *((row.wrapping_add(0x0C)) as *const u8) as u32;
            let bit = 1u32 << (index & 31);
            let (mask_hi, mask_lo) = if index < 32 {
                (0, bit)
            } else if index < 64 {
                (bit, 0)
            } else {
                (0, 0)
            };
            let first = (((this_.wrapping_add(8)) as *const u32).read_unaligned()
                & mask_lo)
                | (((this_.wrapping_add(0x0C)) as *const u32).read_unaligned()
                    & mask_hi);
            if first != 0 {
                if target != 0 {
                    let holder = ((target.wrapping_add(0x0C)) as *const u32)
                        .read_unaligned();
                    let bits = (holder as *const u32).read_unaligned();
                    let slot = ((bits.wrapping_add(
                        (selector >> 5).wrapping_mul(4),
                    )) as *mut u32);
                    slot.write_unaligned(
                        slot.read_unaligned() | (1u32 << (selector & 31)),
                    );
                }
            } else if target != 0 {
                let holder = ((target.wrapping_add(0x0C)) as *const u32)
                    .read_unaligned();
                let bits = (holder as *const u32).read_unaligned();
                let slot = ((bits.wrapping_add(
                    (selector >> 5).wrapping_mul(4),
                )) as *mut u32);
                slot.write_unaligned(
                    slot.read_unaligned() & !(1u32 << (selector & 31)),
                );
                let second = (((this_.wrapping_add(0x10)) as *const u32)
                    .read_unaligned()
                    & mask_lo)
                    | (((this_.wrapping_add(0x14)) as *const u32)
                        .read_unaligned()
                        & mask_hi);
                let levels = ((target.wrapping_add(4)) as *const u32)
                    .read_unaligned();
                ((levels.wrapping_add(selector.wrapping_mul(4))) as *mut u32)
                    .write_unaligned(if second != 0 {
                        NEG_UNIT_BITS
                    } else {
                        POS_UNIT_BITS
                    });
            } else {
                let second = (((this_.wrapping_add(0x10)) as *const u32)
                    .read_unaligned()
                    & mask_lo)
                    | (((this_.wrapping_add(0x14)) as *const u32)
                        .read_unaligned()
                        & mask_hi);
                ((record.wrapping_add(0x68)) as *mut u32).write_unaligned(
                    if second != 0 { NEG_UNIT_BITS } else { POS_UNIT_BITS },
                );
            }
            index = index.wrapping_add(1);
            if index == 0 {
                high = high.wrapping_add(1);
            }
            if high > 0 {
                break;
            }
            if index >= bound as u32 {
                break;
            }
        }
        bound as u32
    }
});
