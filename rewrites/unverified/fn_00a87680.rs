// original: 0x00a87680 CRenderPhase::render
//
// Renders the phase packets for one render-phase object (thiscall, no stack
// arguments). It allocates and stamps an initial packet, then visits four
// 16-byte records in ascending order. Active records with a nonzero second
// word are submitted through the command sink; the others produce stamped
// packets. A mode byte enables a pointer packet before a virtual dispatch on
// the object. Active records are then visited in descending order to make
// descriptor packets. The mode byte also enables a cookie-tagged packet
// whose function-table slot is dispatched twice. A trailing packet is always
// made and stamped, and that last stamp is returned. Stamping combines two
// slot answers with the signed remainder-mod-16 rule and folds the result
// into the packet word at offset four. The object pointer is saved globally
// during the operation and cleared before return. If an allocation returns
// null at a site followed by an unconditional virtual dispatch, both sides
// fault at that dispatch.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const C_ALLOC_FIRST: u32 = 1;
const C_MAKE_FIRST: u32 = 2;
const C_ALLOC_ACTIVE: u32 = 3;
const C_MAKE_ACTIVE: u32 = 4;
const C_SINK_ACTIVE: u32 = 5;
const C_SINK_NULL: u32 = 6;
const C_ALLOC_IDLE: u32 = 7;
const C_MAKE_IDLE: u32 = 8;
const C_ALLOC_POINTER: u32 = 9;
const C_MAKE_POINTER: u32 = 10;
const C_ALLOC_DESCRIPTOR: u32 = 11;
const C_MAKE_DESCRIPTOR: u32 = 12;
const C_ALLOC_COOKIE: u32 = 13;
const C_ALLOC_TRAILING: u32 = 14;
const C_MAKE_TRAILING: u32 = 15;

const V_THIS_24: u32 = 16;
const V_FIRST_8: u32 = 17;
const V_IDLE_8: u32 = 18;
const V_POINTER_8: u32 = 19;
const V_DESCRIPTOR_8: u32 = 20;
const V_COOKIE_8: u32 = 21;
const V_TRAILING_8: u32 = 22;
const V_THIS_20: u32 = 23;

const GLOBAL_SAVED_THIS: u32 = 0x012F_B1B8;
const GLOBAL_COOKIE: u32 = 0x0103_27A0;
const IMAGE_PACKET_TEMP_TAG: u32 = 0x00E7_E048;
const IMAGE_PACKET_TABLE: u32 = 0x00E8_668C;

const STAMP_MASK: u32 = 0x01FF_C000;
const FRAME_WORDS: [u32; 7] = [0x3F80_0000, 0x3F80_0000, 0xFF00_0000, 0, 0, 0, 1];

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(addr: u32, value: u32) {
    unsafe { (addr as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_u32(address: u32) -> u32 {
    unsafe { global::<u32>(address).read() }
}

#[inline(always)]
unsafe fn write_global_u32(address: u32, value: u32) {
    unsafe { global::<u32>(address).write(value) }
}

#[inline(always)]
unsafe fn read_byte(obj: u32, offset: u32) -> u8 {
    unsafe { ((obj.wrapping_add(offset)) as *const u8).read() }
}

/// Dispatch a zero-stack-argument virtual slot with the object in ECX.
#[inline(always)]
unsafe fn call_slot(obj: u32, slot: u32) -> u32 {
    unsafe {
        let table = rd32(obj);
        let target = rd32(table.wrapping_add(slot));
        let function: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        function(obj)
    }
}

/// C-style signed remainder, matching the original's sign-corrected mask.
#[inline(always)]
fn signed_mod_16(value: u32) -> i32 {
    (value as i32) % 16
}

/// Fold two slot answers into the stamp field using signed divide semantics.
#[inline(always)]
fn mix_slot_answers(first: u32, second: u32) -> u32 {
    let remainder = signed_mod_16(first);
    let adjustment = (16 - remainder) % 16;
    let sum = second.wrapping_add(adjustment as u32) as i32;
    (sum / 16).wrapping_shl(14) as u32
}

/// Update the packet's second word and return the masked change.
#[inline(always)]
unsafe fn stamp_packet(packet: u32) -> u32 {
    unsafe {
        let first = call_slot(packet, 8);
        let second = call_slot(packet, 8);
        let previous = rd32(packet.wrapping_add(4));
        let change = (mix_slot_answers(first, second) ^ previous) & STAMP_MASK;
        wr32(packet.wrapping_add(4), previous ^ change);
        change
    }
}

/// Install the image table and update the cookie word in the original order.
#[inline(always)]
unsafe fn cookie_tag_packet(packet: u32) {
    unsafe {
        let previous = rd32(packet.wrapping_add(4));
        wr32(packet, relocated(IMAGE_PACKET_TEMP_TAG));
        let cookie = (previous ^ read_global_u32(GLOBAL_COOKIE)) & 0x3FFF;
        wr32(packet.wrapping_add(4), previous ^ cookie);
        write_global_u32(GLOBAL_COOKIE, read_global_u32(GLOBAL_COOKIE).wrapping_add(1));
        wr32(packet, relocated(IMAGE_PACKET_TABLE));
    }
}

#[inline(always)]
unsafe fn make_and_stamp_idle(record_index: u32, record_word: u32) {
    unsafe {
        let allocation = callee_cdecl!(C_ALLOC_IDLE, u32, 0x18, 0);
        let packet = if allocation != 0 {
            callee_thiscall!(C_MAKE_IDLE, u32, allocation, record_index, record_word, 0, 0)
        } else {
            0
        };
        stamp_packet(packet);
    }
}

#[inline(always)]
unsafe fn make_and_stamp_descriptor(record_index: u32) {
    unsafe {
        let allocation = callee_cdecl!(C_ALLOC_DESCRIPTOR, u32, 0x2C, 0);
        let packet = if allocation != 0 {
            callee_thiscall!(
                C_MAKE_DESCRIPTOR,
                u32,
                allocation,
                record_index,
                FRAME_WORDS.as_ptr() as u32
            )
        } else {
            0
        };
        stamp_packet(packet);
    }
}

unsafe fn render_phase_impl(this_ptr: u32) -> u32 {
    unsafe {
        write_global_u32(GLOBAL_SAVED_THIS, this_ptr);

        let allocation = callee_cdecl!(C_ALLOC_FIRST, u32, 0x10, 1);
        let first_packet = if allocation != 0 {
            let value = call_slot(this_ptr, 0x24);
            callee_thiscall!(C_MAKE_FIRST, u32, allocation, value)
        } else {
            0
        };
        stamp_packet(first_packet);

        let record_base = this_ptr.wrapping_add(0x890);
        for record_index in 0..4u32 {
            let record = record_base.wrapping_add(record_index.wrapping_mul(0x10));
            let record_word = rd32(record);
            if record_word == 0 {
                continue;
            }

            let resource_word = rd32(record.wrapping_add(8));
            if resource_word != 0 {
                let allocation = callee_cdecl!(C_ALLOC_ACTIVE, u32, 0x18, 0);
                if allocation != 0 {
                    let answer = callee_thiscall!(
                        C_MAKE_ACTIVE,
                        u32,
                        allocation,
                        record_index,
                        record_word,
                        resource_word,
                        0
                    );
                    callee_cdecl!(C_SINK_ACTIVE, u32, answer);
                } else {
                    callee_cdecl!(C_SINK_NULL, u32, 0);
                }
            } else {
                make_and_stamp_idle(record_index, record_word);
            }
        }

        if read_byte(this_ptr, 0x1C) != 0 {
            let allocation = callee_cdecl!(C_ALLOC_POINTER, u32, 0x410, 0);
            let packet = if allocation != 0 {
                callee_thiscall!(
                    C_MAKE_POINTER,
                    u32,
                    allocation,
                    this_ptr.wrapping_add(0xB0)
                )
            } else {
                0
            };
            stamp_packet(packet);
        }

        call_slot(this_ptr, 0x20);

        for record_index in (0..4u32).rev() {
            let record = record_base.wrapping_add(record_index.wrapping_mul(0x10));
            if rd32(record) != 0 {
                make_and_stamp_descriptor(record_index);
            }
        }

        if read_byte(this_ptr, 0x1C) != 0 {
            let packet = callee_cdecl!(C_ALLOC_COOKIE, u32, 8, 0);
            if packet != 0 {
                cookie_tag_packet(packet);
            }
            stamp_packet(packet);
        }

        let allocation = callee_cdecl!(C_ALLOC_TRAILING, u32, 8, 0);
        let trailing_packet = if allocation != 0 {
            callee_thiscall!(C_MAKE_TRAILING, u32, allocation)
        } else {
            0
        };
        let result = stamp_packet(trailing_packet);

        write_global_u32(GLOBAL_SAVED_THIS, 0);
        result
    }
}

export!(thiscall, rw_a87680(this_ptr: u32) -> u32 {
    unsafe { render_phase_impl(this_ptr) }
});
