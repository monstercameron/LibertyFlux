// original: 0x00E5CB30 timer_list_push_178
/// Push one static node onto the mainloop-timing list; return the old head.
///
/// Reads the shared list head, links it as this node's next pointer,
/// then points the head at this node. Takes no arguments; entry registers
/// are ignored.
export!(cdecl, rw_00e5cb30() -> u32 {
    unsafe {
        let head = *global::<u32>(0x017ACD24);
        *global::<u32>(0x0110E178) = head;
        *global::<u32>(0x017ACD24) = relocated(0x0110E174);
        head
    }
});
