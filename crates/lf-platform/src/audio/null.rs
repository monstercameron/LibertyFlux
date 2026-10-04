//! [`NullAudio`]: an output that accepts every stream and plays nothing.

use super::{
    AudioDeviceId, AudioError, AudioOutput, AudioRender, AudioStream, DeviceInfo, StreamConfig,
    StreamState, find_device, negotiate,
};

/// Sample rates the null device claims, ascending.
const NULL_RATES: [u32; 6] = [11_025, 22_050, 32_000, 44_100, 48_000, 96_000];

/// Channel counts the null device claims (mono to 7.1).
const NULL_CHANNELS: [u16; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

/// The null device's id.
pub const NULL_DEVICE: AudioDeviceId = AudioDeviceId(0);

/// An audio output for running without sound (servers, benchmarks, CI).
///
/// It lists one default device, negotiates like any other backend, and its
/// streams change state on play and pause, but the renderer is never called
/// and no time passes: [`AudioStream::frames_rendered`] stays zero. Code that
/// needs audio time to advance in a test uses [`super::CaptureAudio`].
#[derive(Debug, Default, Clone, Copy)]
pub struct NullAudio;

impl NullAudio {
    /// The null output.
    #[must_use]
    pub fn new() -> Self {
        NullAudio
    }

    fn device_info() -> DeviceInfo {
        DeviceInfo {
            id: NULL_DEVICE,
            name: "Null output (silent)".into(),
            is_default: true,
            sample_rates: NULL_RATES.to_vec(),
            channel_counts: NULL_CHANNELS.to_vec(),
            min_buffer_frames: 1,
            max_buffer_frames: 1 << 16,
            default_buffer_frames: 1024,
        }
    }
}

impl AudioOutput for NullAudio {
    fn devices(&self) -> Vec<DeviceInfo> {
        vec![Self::device_info()]
    }

    fn open_stream(
        &self,
        device: Option<AudioDeviceId>,
        wanted: StreamConfig,
        render: Box<dyn AudioRender>,
    ) -> Result<Box<dyn AudioStream>, AudioError> {
        let devices = self.devices();
        let info = find_device(&devices, device)?;
        let config = negotiate(info, wanted)?;
        // The renderer is dropped at once: it will never be called.
        drop(render);
        Ok(Box::new(NullStream {
            config,
            state: StreamState::Paused,
        }))
    }
}

/// A stream of the null output.
struct NullStream {
    config: StreamConfig,
    state: StreamState,
}

impl AudioStream for NullStream {
    fn device(&self) -> AudioDeviceId {
        NULL_DEVICE
    }

    fn config(&self) -> StreamConfig {
        self.config
    }

    fn state(&self) -> StreamState {
        self.state
    }

    fn frames_rendered(&self) -> u64 {
        0
    }

    fn play(&mut self) -> Result<(), AudioError> {
        self.state = StreamState::Playing;
        Ok(())
    }

    fn pause(&mut self) -> Result<(), AudioError> {
        self.state = StreamState::Paused;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::RenderInfo;

    #[test]
    fn null_output_negotiates_and_never_renders() {
        let out = NullAudio::new();
        assert_eq!(out.default_device(), Some(NULL_DEVICE));
        let render = |_: &mut [f32], _: &RenderInfo| panic!("the null device must not render");
        let mut stream = out
            .open_stream(
                None,
                StreamConfig {
                    sample_rate_hz: 44_100,
                    channels: 6,
                    buffer_frames: 0,
                },
                Box::new(render),
            )
            .unwrap();
        assert_eq!(
            stream.config(),
            StreamConfig {
                sample_rate_hz: 44_100,
                channels: 6,
                buffer_frames: 1024
            }
        );
        assert_eq!(stream.state(), StreamState::Paused);
        stream.play().unwrap();
        assert_eq!(stream.state(), StreamState::Playing);
        stream.pause().unwrap();
        assert_eq!(stream.state(), StreamState::Paused);
        assert_eq!(stream.frames_rendered(), 0);
        assert_eq!(stream.device(), NULL_DEVICE);
        assert!(
            out.open_stream(
                Some(AudioDeviceId(5)),
                StreamConfig::STEREO_48K,
                Box::new(render)
            )
            .is_err()
        );
    }
}
