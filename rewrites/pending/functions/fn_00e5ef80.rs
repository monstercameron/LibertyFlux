// original: 0x00e5ef80 static_list_prepend_00e5ef80
/// Prepend the static node at 0x0110ECC0 (link field at +0x4) to the global singly-linked list headed at 0x017ACD24; returns the previous head.
export!(cdecl, rw_00e5ef80() -> u32 {
    unsafe {
        let head = global::<u32>(0x017ACD24);
        let prev = *head;
        *global::<u32>(0x0110ECC4) = prev;
        *head = relocated(0x0110ECC0);
        prev
    }
});
