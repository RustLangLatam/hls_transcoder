# HLS Transcoder

**HLS Transcoder** is a high-performance Rust library designed for building GStreamer-based pipelines that transcode media files into HLS (HTTP Live Streaming) format. It supports both hardware and software video encoding, making it versatile for a wide range of use cases, including live streaming and video on demand (VoD).

## Overview

The main goal of this library is to simplify the creation of GStreamer pipelines for HLS transcoding. It provides a comprehensive API for configuring various elements, such as file sources, decoders, encoders, muxers, and sinks, along with support for both NVIDIA NVENC hardware acceleration and software-based x264 encoding.

## Key Features

- **GStreamer Integration:** The library leverages GStreamer, a powerful multimedia framework, to build complex pipelines for video and audio processing.
- **HLS Output:** Outputs media in HLS format, making it compatible with many video players and streaming platforms.
- **Hardware & Software Encoding:** Supports NVIDIA NVENC for hardware-accelerated encoding and x264 for software-based encoding.
- **Audio and Video Support:** Allows simultaneous handling of both audio and video streams, providing full HLS packaging.
- **Modular Pipeline Construction:** Uses a builder pattern to easily create and configure pipeline elements with optimal settings.

## Technologies Used

- **Rust**: Safe and fast programming language used to build the core library.
- **GStreamer**: Multimedia framework for building media processing pipelines.
- **NVIDIA NVENC**: Hardware-accelerated video encoding for high performance.
- **x264**: Software-based H.264 video encoding.
- **MPEG-TS Muxing**: Efficient packaging of audio and video streams for streaming.
- **HLS (HTTP Live Streaming)**: Apple’s streaming protocol for delivering video over HTTP.

## Getting Started

To use this library, ensure that you have Rust and GStreamer installed on your system. You can create a new transcoding pipeline by configuring various elements and linking them together using the builder pattern provided by the library.

## Example

```rust
use hls_transcoder::PipelineBuilder;

fn main() {
    let pipeline = PipelineBuilder::new(
        "input.mp4".to_string(),
        "output".to_string(),
        "variant".to_string(),
        1280, 720, 2_000_000, true,
    )
    .build()
    .expect("Failed to build pipeline");

    pipeline.set_state(gst::State::Playing).expect("Failed to set pipeline to playing state");
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests to help improve this library.
