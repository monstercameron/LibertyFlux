// original: 0x00e5f080 static_list_prepend_00e5f080
/// Prepend the static node at 0x0110EE08 (link field at +0x4) to the global singly-linked list headed at 0x017ACD24; returns the previous head.
export!(cdecl, rw_00e5f080() -> u32 {
    unsafe {
        let head = global::<u32>(0x017ACD24);
        let prev = *head;
        *global::<u32>(0x0110EE0C) = prev;
        *head = relocated(0x0110EE08);
        prev
    }
});
