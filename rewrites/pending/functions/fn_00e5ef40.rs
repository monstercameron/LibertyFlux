// original: 0x00e5ef40 static_list_prepend_00e5ef40
/// Prepend the static node at 0x0110EC20 (link field at +0xC) to the global singly-linked list headed at 0x017AD1B8; returns the previous head.
export!(cdecl, rw_00e5ef40() -> u32 {
    unsafe {
        let head = global::<u32>(0x017AD1B8);
        let prev = *head;
        *global::<u32>(0x0110EC2C) = prev;
        *head = relocated(0x0110EC20);
        prev
    }
});
