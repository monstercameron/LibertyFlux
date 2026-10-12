// original: 0x0087B340 rage::crmtIterator::assign_from

/// Assign one iterator's current node to another iterator.
///
/// `this` is the destination iterator and `source` is the stack argument. The
/// method increments the source reference count before querying its current
/// node through vtable slot `+0x1c`. A present result is queried again and
/// passed to destination slot `+0x0c`; destination slot `+0x10` then receives
/// the source iterator. If `source+0x10` contains a node, the method adds and
/// then drops a reference around a second destination-slot `+0x0c` call. A
/// reference count reaching zero releases through the stored owner at `+0x18`
/// or through vtable slot 0. Finally, it drops the temporary source reference
/// with the same zero-count release rule. Reference counts are wrapping 16-bit
/// unsigned values. The function has no defined return value.
///
/// Original: thiscall with one stack pointer. Object-table calls are
/// intercepted through planted heap vtables; direct relink calls are patched.
lf_checker_rt::export!(thiscall, rw_0087b340(this: u32, source: u32) -> () {
    const REFCOUNT_OFFSET: u32 = 0x04;
    const SOURCE_NODE_OFFSET: u32 = 0x10;
    const OWNER_OFFSET: u32 = 0x18;
    const ITERATOR_CURRENT_SLOT: u32 = 0x1C;
    const DEST_SET_CURRENT_SLOT: u32 = 0x0C;
    const DEST_REPLACE_CHILDREN_SLOT: u32 = 0x10;
    const RELEASE_SLOT: u32 = 0;
    const LOCKED_RELINK: u32 = 5;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn call_vtable0(object: u32, slot_offset: u32) -> u32 {
        let vtable = unsafe { read_u32(object) };
        let target = unsafe { read_u32(vtable.wrapping_add(slot_offset)) };
        let call: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        call(object)
    }

    #[inline(always)]
    unsafe fn call_vtable1(object: u32, slot_offset: u32, argument: u32) -> u32 {
        let vtable = unsafe { read_u32(object) };
        let target = unsafe { read_u32(vtable.wrapping_add(slot_offset)) };
        let call: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        call(object, argument)
    }

    unsafe {
        let source_refcount = source.wrapping_add(REFCOUNT_OFFSET);
        write_u16(source_refcount, read_u16(source_refcount).wrapping_add(1));

        if call_vtable0(source, ITERATOR_CURRENT_SLOT) != 0 {
            let current_value = call_vtable0(source, ITERATOR_CURRENT_SLOT);
            let _ = call_vtable1(this, DEST_SET_CURRENT_SLOT, current_value);
        }

        let source_node = read_u32(source.wrapping_add(SOURCE_NODE_OFFSET));
        if source_node != 0 {
            let node_refcount = source_node.wrapping_add(REFCOUNT_OFFSET);
            write_u16(node_refcount, read_u16(node_refcount).wrapping_add(1));
        }

        let _ = call_vtable1(this, DEST_REPLACE_CHILDREN_SLOT, source);

        if source_node != 0 {
            let _ = call_vtable1(this, DEST_SET_CURRENT_SLOT, source_node);
            let node_refcount = source_node.wrapping_add(REFCOUNT_OFFSET);
            let count_after_release = read_u16(node_refcount).wrapping_sub(1);
            write_u16(node_refcount, count_after_release);
            if count_after_release == 0 {
                let owner = read_u32(source_node.wrapping_add(OWNER_OFFSET));
                if owner != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(LOCKED_RELINK, u32, owner, source_node);
                } else {
                    let _ = call_vtable1(source_node, RELEASE_SLOT, 1);
                }
            }
        }

        let count_after_release = read_u16(source_refcount).wrapping_sub(1);
        write_u16(source_refcount, count_after_release);
        if count_after_release == 0 {
            let owner = read_u32(source.wrapping_add(OWNER_OFFSET));
            if owner != 0 {
                let _ = lf_checker_rt::callee_thiscall!(LOCKED_RELINK, u32, owner, source);
            } else {
                let _ = call_vtable1(source, RELEASE_SLOT, 1);
            }
        }
    }
});
