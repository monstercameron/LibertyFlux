/// Audio span wrapper proven with a scripted byte-channel helper.
/// The helper body and its stack-pointer address identity are outside this proof.
/// All comparison families are enabled; conditional coverage uses deterministic
/// witnesses rather than instruction-edge instrumentation.
pub mod audio_span_q142 {
    use lf_checker_rt::callee_thiscall;

const AUDIO_DISABLED_FLAG: usize = 0x0d2;
const AUDIO_GROUP_INDEX: usize = 0x040;
const AUDIO_CHANNELS: usize = 0x048;
const AUDIO_END_POSITION: usize = 0x054;
const AUDIO_ITEM_COUNT: usize = 0x0c8;
const AUDIO_SAMPLE_SCALE: u32 = 0x0115d964;
const AUDIO_SAMPLE_TABLE: u32 = 0x0115d988;
const AUDIO_GROUP_BYTES: u32 = 0x6f40;
const AUDIO_GROUP_DATA: u32 = 0x6f10;

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

/// Advances an audio span through its byte channels. The first output is the
/// next channel index; the second is the end position when a channel reaches
/// the requested boundary or raises its local end flag. A disabled stream
/// clears both outputs. The stream's channel data comes from the engine table.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_0089e7c0(this_ptr: u32, mut channel_index_out: u32, end_position_out: u32) {
    unsafe {
        let index_out_address = channel_index_out;
        if read_u8(this_ptr.wrapping_add(AUDIO_DISABLED_FLAG as u32)) != 0 {
            write_u32(index_out_address, 0);
            write_u32(end_position_out, 0);
            return;
        }

        let end_position = read_u32(this_ptr.wrapping_add(AUDIO_END_POSITION as u32));
        let item_count = read_u32(this_ptr.wrapping_add(AUDIO_ITEM_COUNT as u32)) as i32;
        let group_index = u32::from(read_u8(this_ptr.wrapping_add(AUDIO_GROUP_INDEX as u32)));
        let scale = lf_checker_rt::global::<u32>(AUDIO_SAMPLE_SCALE).read_unaligned();
        let sample_table = lf_checker_rt::global::<u32>(AUDIO_SAMPLE_TABLE).read_unaligned();

        write_u32(index_out_address, 0);
        write_u32(end_position_out, 0);
        let mut accumulated_bytes = 0u32;

        while (read_u32(index_out_address) as i32) < item_count {
            let channel_index = read_u32(index_out_address);
            let channel = read_u8(
                this_ptr
                    .wrapping_add(AUDIO_CHANNELS as u32)
                    .wrapping_add(channel_index),
            );
            let sample_cursor = if channel == u8::MAX {
                0
            } else {
                let table_entry = sample_table
                    .wrapping_add(group_index.wrapping_mul(AUDIO_GROUP_BYTES))
                    .wrapping_add(AUDIO_GROUP_DATA);
                read_u32(table_entry).wrapping_add(u32::from(channel).wrapping_mul(scale))
            };

            let arg_word = &mut channel_index_out as *mut u32;
            (arg_word as *mut u8).write(0);
            let bytes_read = callee_thiscall!(1, u32, sample_cursor, arg_word as u32);
            if bytes_read == u32::MAX {
                write_u32(index_out_address, 0);
                break;
            }

            accumulated_bytes = accumulated_bytes.wrapping_add(bytes_read);
            if accumulated_bytes >= end_position || (arg_word as *const u8).read() != 0 {
                let wrapped_end = bytes_read
                    .wrapping_sub(accumulated_bytes)
                    .wrapping_add(end_position);
                write_u32(end_position_out, wrapped_end);
                break;
            }

            write_u32(channel_index_out, channel_index.wrapping_add(1));
        }

        if (read_u32(index_out_address) as i32) >= item_count {
            write_u32(channel_index_out, u32::MAX);
        }
    }
}
}

