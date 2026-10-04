// original: 0x008758d0 crmt_node_link
/// Link a node into a list: build or reuse it, append, balance the refs.
///
/// thiscall/3. When an inner object is present, builds a fresh node from
/// the tag; otherwise reuses the auxiliary node. Either way the node is
/// reference-counted, appended to the list chain and, on the reuse path,
/// its extra reference is released again, destroying the node when the
/// count reaches zero. Returns the last helper answer on an empty list,
/// the append walk's terminating null otherwise, or 0xffff when the reuse
/// path keeps the node alive.
export!(thiscall, rw_008758d0(this: *mut u8, tag: u32, aux: u32, list: u32) -> u32 {
    unsafe {
        let inner = *(this.add(0x14) as *const u32);
        if inner != 0 {
            let node: u32 = callee_stdcall!(1, u32, tag, 0);
            let refs = (node as *mut u8).add(4) as *mut u16;
            *refs = (*refs).wrapping_add(1);
            let linked = callee_thiscall!(2, u32, node);
            let head = *((list as *const u8).add(0x1c) as *const u32);
            // The append walk leaves the terminating null in EAX on a
            // non-empty list and preserves the helper answer otherwise.
            let base_answer = if head == 0 {
                *((list as *mut u8).add(0x1c) as *mut u32) = node;
                linked
            } else {
                let mut prev = head;
                while *((prev as *const u8).add(0x10) as *const u32) != 0 {
                    prev = *((prev as *const u8).add(0x10) as *const u32);
                }
                *((prev as *mut u8).add(0x10) as *mut u32) = node;
                0
            };
            *((node as *mut u8).add(0x0c) as *mut u32) = list;
            if aux != 0 {
                callee_stdcall!(3, u32, list, 1)
            } else {
                base_answer
            }
        } else {
            let node = aux;
            let refs = (node as *mut u8).add(4) as *mut u16;
            *refs = (*refs).wrapping_add(1);
            callee_stdcall!(3, u32, list, 0);
            *refs = (*refs).wrapping_add(1);
            callee_thiscall!(2, u32, node);
            let head = *((list as *const u8).add(0x1c) as *const u32);
            if head == 0 {
                *((list as *mut u8).add(0x1c) as *mut u32) = node;
            } else {
                let mut prev = head;
                while *((prev as *const u8).add(0x10) as *const u32) != 0 {
                    prev = *((prev as *const u8).add(0x10) as *const u32);
                }
                *((prev as *mut u8).add(0x10) as *mut u32) = node;
            }
            *((node as *mut u8).add(0x0c) as *mut u32) = list;
            *refs = (*refs).wrapping_add(0xffff);
            if *refs != 0 {
                0xffff
            } else {
                let extra = *((node as *const u8).add(0x18) as *const u32);
                if extra != 0 {
                    callee_thiscall!(4, u32, extra, node)
                } else {
                    let vtable = *(node as *const u32);
                    let destroy: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(*(vtable as *const u32) as usize);
                    destroy(node, 1)
                }
            }
        }
    }
});
