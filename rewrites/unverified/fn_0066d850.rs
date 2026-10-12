// original: 0x0066D850 rb_tree_successor
//! Move a three-word tagged red-black-tree cursor to its adjacent node.
//!
//! The cursor stores a payload at byte 0, its current node at byte 4, and a
//! tagged header pointer at byte 8. A node has a tagged child link at byte 4,
//! a traversal link at byte 8, a parent at byte 12, and its payload at byte
//! 16. The original 32-bit layout reserves bit zero in tagged pointers.
//! From a current node, follow the child branch to its terminal node or climb
//! parents until the cursor changes side. An empty cursor starts at the tagged
//! header's root chain. When a next node exists, copy its payload into the
//! cursor. Finally copy the cursor's three words to the caller's output and
//! return that output pointer. `this` is in ECX and the output pointer is the
//! sole stack argument, which the callee removes.
const CURSOR_VALUE_WORD: usize = 0;
const CURSOR_NODE_WORD: usize = 1;
const CURSOR_HEADER_WORD: usize = 2;
const NODE_CHILD_WORD: usize = 1;
const NODE_TRAVERSE_WORD: usize = 2;
const NODE_PARENT_WORD: usize = 3;
const NODE_PAYLOAD_WORD: usize = 4;
const TAGGED_POINTER_MASK: u32 = !1;

#[no_mangle]
pub unsafe extern "thiscall" fn fn_0066D850(cursor: *mut u32, output: *mut u32) -> u32 {
    let current_address = unsafe { core::ptr::read_volatile(cursor.add(CURSOR_NODE_WORD)) };
    let mut next_address = current_address;

    if current_address != 0 {
        let current_node = current_address as *mut u32;
        let child_address = unsafe {
            core::ptr::read_volatile(current_node.add(NODE_CHILD_WORD)) & TAGGED_POINTER_MASK
        };

        if child_address != 0 {
            let mut terminal_address = child_address;
            let mut traverse_address = unsafe {
                core::ptr::read_volatile((child_address as *mut u32).add(NODE_TRAVERSE_WORD))
            };
            while traverse_address != 0 {
                terminal_address = traverse_address;
                traverse_address = unsafe {
                    core::ptr::read_volatile(
                        (traverse_address as *mut u32).add(NODE_TRAVERSE_WORD),
                    )
                };
            }
            next_address = terminal_address;
        } else {
            let mut child_on_path = current_address;
            let mut ancestor_address = unsafe {
                core::ptr::read_volatile(current_node.add(NODE_PARENT_WORD))
            };
            while ancestor_address != 0 {
                let ancestor = ancestor_address as *mut u32;
                let ancestor_child = unsafe {
                    core::ptr::read_volatile(ancestor.add(NODE_CHILD_WORD))
                        & TAGGED_POINTER_MASK
                };
                if child_on_path != ancestor_child {
                    break;
                }
                child_on_path = ancestor_address;
                ancestor_address = unsafe {
                    core::ptr::read_volatile(ancestor.add(NODE_PARENT_WORD))
                };
            }
            next_address = ancestor_address;
        }

        unsafe { core::ptr::write_volatile(cursor.add(CURSOR_NODE_WORD), next_address) };
    } else {
        let tagged_header = unsafe {
            core::ptr::read_volatile(cursor.add(CURSOR_HEADER_WORD))
        };
        let header_address = tagged_header & TAGGED_POINTER_MASK;
        if header_address != 0 && tagged_header & 1 != 0 {
            unsafe {
                core::ptr::write_volatile(cursor.add(CURSOR_HEADER_WORD), header_address);
            }
            let header = header_address as *mut u32;
            let root_address = unsafe { core::ptr::read_volatile(header) };
            let mut terminal_address = root_address;
            let mut traverse_address = unsafe {
                core::ptr::read_volatile((root_address as *mut u32).add(NODE_TRAVERSE_WORD))
            };
            while traverse_address != 0 {
                terminal_address = traverse_address;
                traverse_address = unsafe {
                    core::ptr::read_volatile(
                        (traverse_address as *mut u32).add(NODE_TRAVERSE_WORD),
                    )
                };
            }
            unsafe { core::ptr::write_volatile(cursor.add(CURSOR_NODE_WORD), terminal_address) };
            next_address = terminal_address;
        }
    }

    if next_address != 0 {
        let payload = unsafe {
            core::ptr::read_volatile((next_address as *mut u32).add(NODE_PAYLOAD_WORD))
        };
        unsafe { core::ptr::write_volatile(cursor.add(CURSOR_VALUE_WORD), payload) };
    }

    for word in 0..3 {
        let value = unsafe { core::ptr::read_volatile(cursor.add(word)) };
        unsafe { core::ptr::write_volatile(output.add(word), value) };
    }
    output as u32
}
