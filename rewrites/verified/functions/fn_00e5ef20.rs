// original: 0x00e5ef20 static_list_prepend_00e5ef20
/// Prepend the static node at 0x0110EBE8 (link field at +0xC) to the global singly-linked list headed at 0x017AD1B8; returns the previous head.
export!(cdecl, rw_00e5ef20() -> u32 {
    unsafe {
        let head = global::<u32>(0x017AD1B8);
        let prev = *head;
        *global::<u32>(0x0110EBF4) = prev;
        *head = relocated(0x0110EBE8);
        prev
    }
});
