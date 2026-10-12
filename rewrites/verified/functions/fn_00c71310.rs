// original: 0x00C71310 anim_list_array_free

/// Release each non-null list node, release a node's child buffer when its 16-bit child count is nonzero, release the pointer array, then clear the object's data pointer and both adjacent 16-bit fields. The list node's next pointer is at byte offset 0x0c.
lf_checker_rt::export!(thiscall, rw_0x00c71310(this: u32) -> u32 {
    const DATA:u32=0; const COUNT:u32=4; const NEXT:u32=0x0c; const CHILD:u32=4; const CHILD_COUNT:u32=0x0a;
    unsafe { let object=this as *mut u8; let array=object.cast::<u32>().read_unaligned(); let count=object.add(COUNT as usize).cast::<u16>().read_unaligned(); let mut i=0u16; while i<count { let mut node_ptr=(array as *const u32).add(i as usize).read_unaligned(); while node_ptr!=0 { let node=node_ptr as *const u8; let next=node.add(NEXT as usize).cast::<u32>().read_unaligned(); if node.add(CHILD_COUNT as usize).cast::<u16>().read_unaligned()!=0 { let child=node.add(CHILD as usize).cast::<u32>().read_unaligned(); let _=lf_checker_rt::callee_cdecl!(1,u32,child); } let _=lf_checker_rt::callee_cdecl!(1,u32,node_ptr); node_ptr=next; } i=i.wrapping_add(1); } let _=lf_checker_rt::callee_cdecl!(1,u32,array); object.cast::<u32>().write_unaligned(0); object.add(COUNT as usize).cast::<u32>().write_unaligned(0); 0 }
});
