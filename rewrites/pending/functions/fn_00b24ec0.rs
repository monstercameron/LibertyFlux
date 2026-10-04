// original: 0x00b24ec0 negated_model_value
// s08_b24ec0: negated model value. thiscall/0 -> f32: looks up the model
// record for the signed model index at +0x2E in the global model table,
// loads the float at record+0x28 and flips its sign with the global sign
// mask. Returned on the x87 stack.
export!(thiscall, rw_b24ec0(this: *mut u8) -> f32 {
    unsafe {
        let idx = *(this.add(0x2E) as *const i16) as i32;
        let table = global::<u32>(0x1295CD8);
        let rec = *table.offset(idx as isize);
        let v = *((rec + 0x28) as *const f32);
        let mask = *global::<u32>(0xFE8FA0);
        f32::from_bits(v.to_bits() ^ mask)
    }
});
