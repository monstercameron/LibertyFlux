// original: 0x00e5f0a0 static_list_prepend_00e5f0a0
/// Prepend the static node at 0x0110EE00 (link field at +0x4) to the global singly-linked list headed at 0x017ACD24; returns the previous head.
export!(cdecl, rw_00e5f0a0() -> u32 {
    unsafe {
        let head = global::<u32>(0x017ACD24);
        let prev = *head;
        *global::<u32>(0x0110EE04) = prev;
        *head = relocated(0x0110EE00);
        prev
    }
});
