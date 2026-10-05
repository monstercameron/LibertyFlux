// original: 0x00bf34b0 update_entity_placement (proposed name)
/// Refresh one entity's placement record from its live transform.
///
/// `this_` carries the placement key (signed byte at +1), `entity` the
/// entity, `arg2` an auxiliary object, `weight` a blend weight and `flags`
/// control flags. A null entity or a negative key byte returns the object
/// unchanged. Otherwise the entity's live record is fetched through two
/// virtual calls (falling back to its stored record when the first answers
/// null); an empty or exhausted record returns as well. When the entity's
/// update bit is clear and the weight is nonzero, the auxiliary object
/// prepares two scratch blocks and the blend helper consumes them with the
/// record row; otherwise a simpler helper takes the row directly. A set
/// flag byte then runs two notification calls. Returns the last helper or
/// notification answer, or the incoming accumulator on early exits.
export!(thiscall, rw_00bf34b0(this_: *mut u8, entity: *mut u8, arg2: u32, weight: f32, flags: u32) -> u32 {
    unsafe {
        if entity.is_null() {
            return this_ as u32;
        }
        let key = *(this_.add(1) as *const i8);
        if key < 0 {
            return this_ as u32;
        }
        // First virtual call through the entity's table slot 0xa0.
        let vt = *(entity as *const u32);
        let slot1 = ((vt + 0xa0) as *const u32).read();
        let vcall1: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot1 as usize);
        let a = vcall1(entity as u32);
        let (rec, acc) = if a == 0 {
            (*(entity.add(0x100) as *const u32), 0u32)
        } else {
            let b = vcall1(entity as u32);
            let vt2 = *(b as *const u32);
            let slot2 = ((vt2 + 0xe0) as *const u32).read();
            let vcall2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot2 as usize);
            let r = vcall2(b);
            (r, r)
        };
        if rec == 0 {
            return 0;
        }
        if (*(rec as *const i32).add(4)) <= 0 {
            return acc;
        }
        let row = (*(rec as *const u32).add(5))
            .wrapping_add(((key as i32) << 6) as u32);
        let bit_set = (*(entity.add(0x24) as *const u32) & 0x400) != 0;
        let ans = if !bit_set && weight != 0.0 {
            if arg2 == 0 {
                // No call runs on this path, so the original returns its
                // accumulator with AH replaced by the weight comparison's
                // flags (via lahf): SF/AF are deterministically zero here
                // (the AND-masked test clears SF; the compare against zero
                // clears AF), ZF/PF/CF come from the float class.
                let ah = if weight.is_nan() {
                    0x47u32
                } else if weight == 0.0 {
                    0x42
                } else if weight < 0.0 {
                    0x03
                } else {
                    0x02
                };
                (acc & !0xFF00) | (ah << 8)
            } else {
                let mut s1 = [0u32; 4];
                let mut s2 = [0u32; 4];
                callee_thiscall!(1, u32, arg2, s2.as_mut_ptr() as u32, s1.as_mut_ptr() as u32);
                callee_thiscall!(2, u32, this_ as u32, row, s2.as_mut_ptr() as u32,
                    s1.as_mut_ptr() as u32, weight.to_bits())
            }
        } else {
            callee_thiscall!(3, u32, this_ as u32, row)
        };
        if (flags as u8) == 0 {
            return ans;
        }
        callee_cdecl!(4, u32, entity as u32, 0);
        callee_cdecl!(5, u32, entity as u32, 1)
    }
});
