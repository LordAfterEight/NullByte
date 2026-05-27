use creek::{ReadDiskStream, SymphoniaDecoder, SeekMode};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use ringbuf::traits::{Observer, Split};
use ringbuf::{HeapRb, traits::{Producer, Consumer}};

const RING_BUFFER_FRAMES: usize = 8192;

#[cfg(target_os = "windows")]
const VOLUME_MULTIPLIER: f32 = 0.5;

#[cfg(target_os = "linux")]
const VOLUME_MULTIPLIER: f32 = 1.0;

pub struct AudioManager {
    pub channels: Vec<Channel>,
    producer: ringbuf::HeapProd<f32>,
    consumer_len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    _stream: Stream,
}

impl AudioManager {
    pub fn init() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device");
        let config: StreamConfig = device.default_output_config().unwrap().into();

        assert_eq!(config.channels, 2, "Only stereo output supported");

        let rb = HeapRb::<f32>::new(RING_BUFFER_FRAMES * 2);
        let (producer, mut consumer) = rb.split();

        let consumer_len = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let consumer_len_clone = consumer_len.clone();

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                let available = consumer.pop_slice(data);
                data[available..].fill(0.0);
                consumer_len_clone.store(
                    consumer.occupied_len() / 2,
                    std::sync::atomic::Ordering::Relaxed,
                );
            },
            |err| eprintln!("cpal stream error: {err}"),
            None,
        ).unwrap();

        stream.play().unwrap();

        Self {
            channels: Vec::new(),
            producer,
            consumer_len,
            _stream: stream,
        }
    }

    pub fn new_channel(&mut self, src: &str, name: &str) {
        let mut stream = ReadDiskStream::<SymphoniaDecoder>::new(
            src,
            0,
            Default::default(),
        ).unwrap();

        stream.cache(0, 0).unwrap();
        stream.seek(0, SeekMode::Auto).unwrap();
        stream.block_until_ready().unwrap();

        self.channels.push(Channel {
            label: name.to_string(),
            read_disk_stream: stream,
            next: None,
            volume: 0.0,
            finished: false,
        });
    }

    pub fn is_finished(&self, label: &str) -> bool {
        !self.channels.iter().any(|c| c.label == label)
    }

    pub fn set_next(&mut self, label: &str, next_src: &str) {
        if let Some(c) = self.channels.iter_mut().find(|c| c.label == label) {
            let mut stream = ReadDiskStream::<SymphoniaDecoder>::new(
                next_src,
                0,
                Default::default(),
            ).unwrap();
            stream.cache(0, 0).unwrap();
            stream.seek(0, SeekMode::Auto).unwrap();
            stream.block_until_ready().unwrap();
            c.next = Some(stream);
        }
    }

    pub fn interrupt(&mut self, label: &str, next_src: &str) {
        if let Some(channel) = self.channels.iter_mut().find(|c| c.label == label) {
            let buffered = self.consumer_len.load(std::sync::atomic::Ordering::Relaxed);

            let mut stream = ReadDiskStream::<SymphoniaDecoder>::new(
                next_src,
                0,
                Default::default(),
            ).unwrap();

            stream.cache(0, 0).unwrap();
            stream.seek(buffered, SeekMode::Auto).unwrap();
            stream.block_until_ready().unwrap();

            channel.read_disk_stream = stream;
            channel.next = None;
            channel.finished = false;
        }
    }

    pub fn has_next(&self, label: &str) -> bool {
        if let Some(channel) = self.channels.iter().find(|c| c.label == label) {
            return channel.next.is_some()
        } else {
            return false;
        }
    }

    pub fn update(&mut self) {
        let writable_frames = self.producer.vacant_len() / 2;
        if writable_frames == 0 {
            return;
        }

        let mut mix = vec![0.0f32; writable_frames * 2];

        for channel in self.channels.iter_mut() {
            if channel.finished {
                continue;
            }
            if !channel.read_disk_stream.is_ready().unwrap() {
                continue;
            }

            let mut frames_written = 0;

            loop {
                let info = channel.read_disk_stream.info();
                let remaining_in_stream = info.num_frames.saturating_sub(channel.read_disk_stream.playhead());
                let frames_to_read = (writable_frames - frames_written).min(remaining_in_stream);

                if frames_to_read > 0 {
                    let read_data = channel.read_disk_stream.read(frames_to_read).unwrap();
                    let left = read_data.read_channel(0);
                    let right = read_data.read_channel(1);

                    for i in 0..read_data.num_frames() {
                        mix[(frames_written + i) * 2]     += left[i] * channel.volume * VOLUME_MULTIPLIER;
                        mix[(frames_written + i) * 2 + 1] += right[i] * channel.volume * VOLUME_MULTIPLIER;
                    }

                    frames_written += read_data.num_frames();
                }

                let info = channel.read_disk_stream.info();
                if channel.read_disk_stream.playhead() >= info.num_frames {
                    if let Some(next_stream) = channel.next.take() {
                        channel.read_disk_stream = next_stream;
                    } else {
                        channel.finished = true;
                        break;
                    }
                }

                if frames_written >= writable_frames {
                    break;
                }
            }
        }

        self.producer.push_slice(&mix);
        self.channels.retain(|c| !c.finished);
    }

    pub fn set_volume(&mut self, label: &str, volume: f32) {
        if let Some(c) = self.channels.iter_mut().find(|c| c.label == label) {
            c.volume = volume.clamp(0.0, 1.0);
        }
    }

    pub fn active_channels(&self) -> usize {
        self.channels.len()
    }
}

pub struct Channel {
    label: String,
    read_disk_stream: ReadDiskStream<SymphoniaDecoder>,
    next: Option<ReadDiskStream<SymphoniaDecoder>>,
    pub volume: f32,
    finished: bool,
}

impl Channel {
    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
