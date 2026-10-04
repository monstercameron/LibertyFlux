// original: 0x009df690 tracker_node_attach
// fn_009df690: tracker node attach (thiscall/0).
//
// Stamps the object with its vtable, clears its scalar fields, links a fresh
// node from the node step onto the front of the global list, and finishes
// clearing the tail fields. Returns `this`.
export!(thiscall, rw_009df690(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xE98184);
        *(this.add(0x30) as *mut u32) = 0;
        *(this.add(0x38) as *mut u32) = 0;
        *this.add(0x45) = 0;
        *(this.add(0x3C) as *mut u16) = 0;
        *this.add(0x3F) = 0;
        *(this.add(0x34) as *mut u32) = 0;
        let fresh: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let node = fresh(*global::<u32>(0x12B4164));
        // The original links through the node unconditionally; a null answer
        // would fault on both sides, so contracts only script non-null nodes.
        *(node as *mut u32) = this as u32;
        *((node.wrapping_add(4)) as *mut u32) = *global::<u32>(0x12B41B0);
        *global::<u32>(0x12B41B0) = node;
        *this.add(0x44) = 0;
        *(this.add(0x40) as *mut u32) = 0;
        *(this.add(0x46) as *mut u16) = 0;
        *this.add(0x3E) = 0;
        this as u32
    }
});
