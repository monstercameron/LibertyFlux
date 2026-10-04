// original: 0x00b577a0 forward_record_level_to_owner
// rs05f19: forward a record's field to its owner's level setter.
//
// Reads the float at `rec+0x4C` and passes it to the level setter
// (thiscall/1) on `owner`. The entry ECX is unused scratch.
export!(thiscall, rw_b577a0(_this: u32, rec: *const u8, owner: u32) -> () {
    unsafe {
        let value = *(rec.add(0x4C) as *const f32);
        let set_level: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        set_level(owner, value.to_bits());
    }
});
