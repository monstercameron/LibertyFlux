// original: 0x00a49680 vehicle_map_door_index
/// Map a door/seat index through the vehicle's kind flag.
///
/// When the dword at `this+0x1304` is 1, argument 0 maps to 1, argument 2
/// maps to 2 and every other argument maps to 1; otherwise the result is the
/// argument plus one, wrapping (thiscall, one stack word).
export!(thiscall, rw_00a49680(this: u32, a: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x1304;
        let flag = (this.wrapping_add(KIND_OFF) as *const u32).read_unaligned();
        if flag == 1 {
            if a == 0 {
                1
            } else if a == 2 {
                2
            } else {
                1
            }
        } else {
            a.wrapping_add(1)
        }
    }
});
