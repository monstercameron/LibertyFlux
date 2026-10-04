// original: 0x00b57210 set_level_on_matched_entries_alt
// rs05f8/rs05f9: notify matching list entries of a new level.
//
// Walks the list at `this+0x1A28` (links at `+0x8C`) and, for every entry
// whose kind word is 1, whose flag at `+0x44` is set and whose id at `+0xC`
// equals `wanted`, forwards `level` to it (thiscall/1 with the entry body).
fn notify_matching_list(
    head: *const u8,
    wanted: u32,
    level_bits: u32,
    callee: extern "thiscall" fn(u32, u32) -> u32,
) {
    unsafe {
        let mut node = *(head.add(0x1A28) as *const *const u8);
        while !node.is_null() {
            let next = *(node.add(0x8C) as *const *const u8);
            let kind = *(node.add(0x48) as *const u16);
            let body = node.add(4);
            if kind == 1
                && *(body.add(0x40) as *const u32) != 0
                && *(body.add(8) as *const u32) == wanted
            {
                callee(body as u32, level_bits);
            }
            node = next;
        }
    }
}

// rs05f9: notify matching entries (second variant).
export!(thiscall, rw_b57210(this: *const u8, wanted: u32, level: f32) -> () {
    unsafe {
        let notify: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        notify_matching_list(this, wanted, level.to_bits(), notify);
    }
});
