// original: 0x00a4a9d0 vehicle_head_equals_arg
/// True when the head word of the object at `this` equals the argument.
///
/// The original loads `[this]`, compares it with its one stack argument and
/// returns the equality flag (thiscall, one stack word). EAX is fully
/// determined (zeroed before the set), so the whole register is compared.
export!(thiscall, rw_00a4a9d0(this: u32, want: u32) -> u32 {
    unsafe {
        let head = (this as *const u32).read_unaligned();
        (head == want) as u32
    }
});
