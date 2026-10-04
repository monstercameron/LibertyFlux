//! [`CaptureAudio`]: an in-memory output that records what streams render.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use super::{
    AudioDeviceId, AudioError, AudioOutput, AudioRender, AudioStream, DeviceInfo, RenderInfo,
    StreamConfig, StreamState, find_device, negotiate,
};

/// One stream's state, shared between the stream handle and the output.
struct StreamCore {
    device: AudioDeviceId,
    config: StreamConfig,
    state: StreamState,
    /// `None` once the stream handle is dropped.
    render: Option<Box<dyn AudioRender>>,
    position: u64,
    captured: Vec<f32>,
    /// Reused render buffer, so pumping allocates only while it grows.
    scratch: Vec<f32>,
}

type SharedStream = Arc<Mutex<StreamCore>>;

fn lock(stream: &SharedStream) -> MutexGuard<'_, StreamCore> {
    stream.lock().unwrap_or_else(PoisonError::into_inner)
}

/// A deterministic audio output for tests: the test is the sound card.
///
/// Devices are given at construction. Each opened stream is numbered in
/// open order (0, 1, ...). Nothing renders on its own: [`CaptureAudio::pump`]
/// asks every playing stream for a number of frames, in calls of at most the
/// stream's negotiated `buffer_frames`, exactly as a device would, and
/// appends the samples to that stream's capture. Captures outlive the stream
/// handle, so a test can drop the stream and still read what it produced.
/// [`CaptureAudio::disconnect`] simulates a device being unplugged.
pub struct CaptureAudio {
    devices: Vec<DeviceInfo>,
    streams: Mutex<Vec<SharedStream>>,
}

impl Default for CaptureAudio {
    /// One default stereo device at 44.1 or 48 kHz, 256-frame buffers.
    fn default() -> Self {
        CaptureAudio::new(vec![DeviceInfo {
            id: AudioDeviceId(1),
            name: "Capture (stereo)".into(),
            is_default: true,
            sample_rates: vec![44_100, 48_000],
            channel_counts: vec![2],
            min_buffer_frames: 16,
            max_buffer_frames: 4096,
            default_buffer_frames: 256,
        }])
    }
}

impl CaptureAudio {
    /// An output with exactly these devices.
    #[must_use]
    pub fn new(devices: Vec<DeviceInfo>) -> Self {
        CaptureAudio {
            devices,
            streams: Mutex::new(Vec::new()),
        }
    }

    fn streams(&self) -> MutexGuard<'_, Vec<SharedStream>> {
        self.streams.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// How many streams have been opened (including dropped ones).
    #[must_use]
    pub fn stream_count(&self) -> usize {
        self.streams().len()
    }

    /// Renders `frames` frames on every playing stream and returns how many
    /// streams rendered. Calls are at most `buffer_frames` long; the last
    /// one may be shorter.
    pub fn pump(&self, frames: u64) -> usize {
        let streams: Vec<SharedStream> = self.streams().clone();
        let mut rendered = 0;
        for stream in &streams {
            let mut core = lock(stream);
            if core.state != StreamState::Playing || core.render.is_none() {
                continue;
            }
            rendered += 1;
            let mut left = frames;
            while left > 0 {
                let chunk = left.min(u64::from(core.config.buffer_frames.max(1)));
                pump_once(&mut core, usize::try_from(chunk).unwrap_or(usize::MAX));
                left -= chunk;
            }
        }
        rendered
    }

    /// A copy of everything stream `index` has rendered.
    #[must_use]
    pub fn captured(&self, index: usize) -> Option<Vec<f32>> {
        self.streams().get(index).map(|s| lock(s).captured.clone())
    }

    /// Removes and returns everything stream `index` has rendered.
    pub fn take_captured(&self, index: usize) -> Option<Vec<f32>> {
        self.streams()
            .get(index)
            .map(|s| std::mem::take(&mut lock(s).captured))
    }

    /// The negotiated format of stream `index`.
    #[must_use]
    pub fn stream_config(&self, index: usize) -> Option<StreamConfig> {
        self.streams().get(index).map(|s| lock(s).config)
    }

    /// Simulates unplugging a device: its streams become
    /// [`StreamState::Lost`] and never render again.
    pub fn disconnect(&self, device: AudioDeviceId) {
        for stream in self.streams().iter() {
            let mut core = lock(stream);
            if core.device == device {
                core.state = StreamState::Lost;
            }
        }
    }
}

/// One render call of `frames` frames: zero the buffer, render, record.
fn pump_once(core: &mut StreamCore, frames: usize) {
    let channels = usize::from(core.config.channels);
    let info = RenderInfo {
        config: core.config,
        frame_position: core.position,
        frames,
    };
    let mut scratch = std::mem::take(&mut core.scratch);
    scratch.clear();
    scratch.resize(frames * channels, 0.0);
    if let Some(render) = core.render.as_mut() {
        render.render(&mut scratch, &info);
    }
    core.captured.extend_from_slice(&scratch);
    core.position += frames as u64;
    core.scratch = scratch;
}

impl AudioOutput for CaptureAudio {
    fn devices(&self) -> Vec<DeviceInfo> {
        self.devices.clone()
    }

    fn open_stream(
        &self,
        device: Option<AudioDeviceId>,
        wanted: StreamConfig,
        render: Box<dyn AudioRender>,
    ) -> Result<Box<dyn AudioStream>, AudioError> {
        let info = find_device(&self.devices, device)?;
        let config = negotiate(info, wanted)?;
        let core = Arc::new(Mutex::new(StreamCore {
            device: info.id,
            config,
            state: StreamState::Paused,
            render: Some(render),
            position: 0,
            captured: Vec::new(),
            scratch: Vec::new(),
        }));
        self.streams().push(Arc::clone(&core));
        Ok(Box::new(CaptureStream { core }))
    }
}

/// A stream of the capture output.
struct CaptureStream {
    core: SharedStream,
}

impl AudioStream for CaptureStream {
    fn device(&self) -> AudioDeviceId {
        lock(&self.core).device
    }

    fn config(&self) -> StreamConfig {
        lock(&self.core).config
    }

    fn state(&self) -> StreamState {
        lock(&self.core).state
    }

    fn frames_rendered(&self) -> u64 {
        lock(&self.core).position
    }

    fn play(&mut self) -> Result<(), AudioError> {
        set_state(&self.core, StreamState::Playing)
    }

    fn pause(&mut self) -> Result<(), AudioError> {
        set_state(&self.core, StreamState::Paused)
    }
}

fn set_state(core: &SharedStream, state: StreamState) -> Result<(), AudioError> {
    let mut core = lock(core);
    if core.state == StreamState::Lost {
        return Err(AudioError::StreamLost);
    }
    core.state = state;
    Ok(())
}

impl Drop for CaptureStream {
    fn drop(&mut self) {
        // Drop the renderer now: the contract says no render call starts
        // after the stream is dropped. The capture stays readable.
        let mut core = lock(&self.core);
        core.render = None;
        if core.state == StreamState::Playing {
            core.state = StreamState::Paused;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A renderer that writes `frame_position + i` into channel 0 and its
    /// negation into channel 1, so every sample says where it came from.
    #[allow(clippy::cast_precision_loss)] // frame counts in tests stay far below 2^24
    fn ramp(out: &mut [f32], info: &RenderInfo) {
        let channels = usize::from(info.config.channels);
        assert_eq!(out.len(), info.frames * channels);
        assert!(out.iter().all(|&s| s == 0.0), "buffer must be zeroed");
        for (i, frame) in out.chunks_exact_mut(channels).enumerate() {
            let t = (info.frame_position + i as u64) as f32;
            frame[0] = t;
            frame[1] = -t;
        }
    }

    #[test]
    fn pump_renders_in_buffer_sized_calls() {
        let out = CaptureAudio::default();
        let calls = Arc::new(Mutex::new(Vec::new()));
        let c2 = Arc::clone(&calls);
        let mut stream = out
            .open_stream(
                None,
                StreamConfig {
                    sample_rate_hz: 48_000,
                    channels: 2,
                    buffer_frames: 100,
                },
                Box::new(move |buf: &mut [f32], info: &RenderInfo| {
                    c2.lock().unwrap().push((info.frame_position, info.frames));
                    ramp(buf, info);
                }),
            )
            .unwrap();
        assert_eq!(out.pump(50), 0, "paused streams do not render");
        stream.play().unwrap();
        assert_eq!(out.pump(250), 1);
        assert_eq!(
            *calls.lock().unwrap(),
            vec![(0, 100), (100, 100), (200, 50)]
        );
        assert_eq!(stream.frames_rendered(), 250);
        let samples = out.captured(0).unwrap();
        assert_eq!(samples.len(), 500);
        assert_eq!(&samples[..4], &[0.0, -0.0, 1.0, -1.0]);
        assert_eq!(&samples[498..], &[249.0, -249.0]);
        stream.pause().unwrap();
        assert_eq!(out.pump(10), 0);
        assert_eq!(stream.frames_rendered(), 250, "pause keeps the position");
    }

    #[test]
    fn negotiation_and_device_choice() {
        let out = CaptureAudio::default();
        let render = |_: &mut [f32], _: &RenderInfo| {};
        let stream = out
            .open_stream(
                Some(AudioDeviceId(1)),
                StreamConfig {
                    sample_rate_hz: 22_050,
                    channels: 1,
                    buffer_frames: 0,
                },
                Box::new(render),
            )
            .unwrap();
        assert_eq!(
            stream.config(),
            StreamConfig {
                sample_rate_hz: 44_100,
                channels: 2,
                buffer_frames: 256
            }
        );
        assert_eq!(out.stream_config(0), Some(stream.config()));
        assert_eq!(
            out.open_stream(
                Some(AudioDeviceId(7)),
                StreamConfig::STEREO_48K,
                Box::new(render)
            )
            .err(),
            Some(AudioError::UnknownDevice(AudioDeviceId(7)))
        );
        let none = CaptureAudio::new(Vec::new());
        assert_eq!(none.default_device(), None);
        assert_eq!(
            none.open_stream(None, StreamConfig::STEREO_48K, Box::new(render))
                .err(),
            Some(AudioError::NoDevice)
        );
    }

    #[test]
    fn dropped_streams_stop_and_keep_their_capture() {
        let out = CaptureAudio::default();
        let mut stream = out
            .open_stream(None, StreamConfig::STEREO_48K, Box::new(ramp))
            .unwrap();
        stream.play().unwrap();
        out.pump(10);
        drop(stream);
        assert_eq!(out.pump(10), 0);
        assert_eq!(out.take_captured(0).unwrap().len(), 20);
        assert_eq!(out.captured(0).unwrap().len(), 0);
        assert_eq!(out.captured(1), None);
        assert_eq!(out.stream_count(), 1);
    }

    #[test]
    fn disconnect_loses_the_stream() {
        let out = CaptureAudio::default();
        let mut stream = out
            .open_stream(None, StreamConfig::STEREO_48K, Box::new(ramp))
            .unwrap();
        stream.play().unwrap();
        out.disconnect(AudioDeviceId(1));
        assert_eq!(stream.state(), StreamState::Lost);
        assert_eq!(out.pump(10), 0);
        assert_eq!(stream.play(), Err(AudioError::StreamLost));
        assert_eq!(stream.pause(), Err(AudioError::StreamLost));
    }

    #[test]
    fn two_streams_mix_independently() {
        let out = CaptureAudio::default();
        let mut a = out
            .open_stream(None, StreamConfig::STEREO_48K, Box::new(ramp))
            .unwrap();
        let mut b = out
            .open_stream(
                None,
                StreamConfig::STEREO_48K,
                Box::new(|buf: &mut [f32], _: &RenderInfo| buf.fill(0.5)),
            )
            .unwrap();
        a.play().unwrap();
        b.play().unwrap();
        assert_eq!(out.pump(4), 2);
        assert_eq!(out.captured(1).unwrap(), vec![0.5; 8]);
        assert_eq!(out.captured(0).unwrap()[6].to_bits(), 3.0f32.to_bits());
    }
}
