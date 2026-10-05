// original: 0x00a47ab0 vehicle_select_subpart_by_class
/// Map a resolved value through class rules, then select the subpart.
///
/// Resolves `(this, arg)` through two callees (thiscall/1, thiscall/1 on
/// `(id1answer, this)`), forces 5 for kind 0x57, adjusts for class 1
/// (7->5, 8->6) and class 3 (5/7->5, 6/8->6), and returns the selector's
/// answer (thiscall/2 on `(this, value, 0)`; thiscall, one stack argument).
export!(thiscall, rw_00a47ab0(this: u32, arg: u32) -> u32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        let p: u32 = callee_thiscall!(1, u32, this, arg);
        let mut v: u32 = callee_thiscall!(2, u32, p, this);
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        if (row.wrapping_add(0xc4) as *const u32).read_unaligned() == 0x57 {
            v = 5;
        }
        let class = (this.wrapping_add(0x1304) as *const u32).read_unaligned();
        if class == 1 {
            if v == 7 {
                v = 5;
            } else if v == 8 {
                v = 6;
            }
        }
        if class == 3 {
            if v == 5 || v == 7 {
                v = 5;
            } else if v == 6 || v == 8 {
                v = 6;
            }
        }
        callee_thiscall!(3, u32, this, v, 0)
    }
});
