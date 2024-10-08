use crate::elements_builder::{
    capsfilter, decodebin, filesrc, hlssink3, mpegtsmux, nvh264enc, xh264enc, H264EncBuilder,
};
use anyhow::{Context, Result};
use gst::prelude::*;
use gst::Element;

// Define an enum to encapsulate both encoder types
pub enum H264Encoder {
    Nvenc(nvh264enc::NVH264EncBuilder),
    X264(xh264enc::Xh264EncBuilder),
}

impl H264Encoder {
    /// Sets the bitrate for the encoder.
    pub fn with_bitrate(self, bitrate: u32) -> Self {
        match self {
            H264Encoder::Nvenc(mut builder) => H264Encoder::Nvenc(builder.with_bitrate(bitrate).clone()),
            H264Encoder::X264(mut builder) => H264Encoder::X264(builder.with_bitrate(bitrate).clone()),
        }
    }

    /// Builds the encoder element.
    pub fn build(self) -> Result<gst::Element> {
        match self {
            H264Encoder::Nvenc(builder) => builder.build(),
            H264Encoder::X264(builder) => builder.build(),
        }
    }
}

/// Represents a pipeline builder with configurations.
pub struct PipelineBuilder {
    input_file: String,
    variant_name: String,
    filesrc: filesrc::FileSrcBuilder,
    decodebin: decodebin::DecodeBinBuilder,
    capsfilter: capsfilter::CapsFilterBuilder,
    mpegtsmux: mpegtsmux::MpegTsMuxBuilder,
    video_encoder: H264Encoder,
    hlssink: hlssink3::HlsSink3Builder,

    // Enable NVENC acceleration if true.
    nvh: bool,
}

impl PipelineBuilder {
    /// Creates a new pipeline builder with the given configurations.
    pub fn new(
        input_file: String,
        variant_dir: String,
        variant_name: String,
        width: i32,
        height: i32,
        bitrate: u32,
        acceleration: bool,
    ) -> Self {
        gst::init().expect("Failed to initialize Gst");

        let variant_dir = format!("{}/{}", variant_dir, variant_name);
        let filesrc = filesrc::FileSrcBuilder::new(&input_file);
        let decodebin = decodebin::DecodeBinBuilder::new();
        let capsfilter = capsfilter::CapsFilterBuilder::new("video/x-raw")
            .with_width(width)
            .with_height(height)
            .with_profile("high");

        let mpegtsmux = mpegtsmux::MpegTsMuxBuilder::new()
            .with_alignment(1)
            .with_pat_interval(2000)
            .with_pcr_interval(40);

        // Choose encoder type based on the acceleration flag
        let video_encoder = if acceleration {
            H264Encoder::Nvenc(nvh264enc::NVH264EncBuilder::default())
        } else {
            H264Encoder::X264(xh264enc::Xh264EncBuilder::default())
        };
        let video_encoder = video_encoder
            .with_bitrate(bitrate);

        let hlssink = hlssink3::HlsSink3Builder::new(&variant_dir, &variant_dir);

        Self {
            input_file,
            variant_name,
            filesrc,
            decodebin,
            capsfilter,
            mpegtsmux,
            video_encoder,
            hlssink,
            nvh: acceleration,
        }
    }

    fn create_queue(name: &str) -> Result<Element> {
        gst::ElementFactory::make_with_name("queue", Some(name)).map_err(|err| err.into())
    }

    fn create_element(factory_name: &str) -> Result<Element> {
        gst::ElementFactory::make_with_name(factory_name, Some(factory_name))
            .map_err(|err| err.into())
    }

    /// Builds and configures the GStreamer pipeline for both video and audio processing.
    pub fn build(self) -> Result<gst::Pipeline> {
        let pipeline_name = format!("pipeline_{}", self.variant_name);
        let pipeline = gst::Pipeline::with_name(&pipeline_name);

        let file_source = self
            .filesrc
            .build()
            .context("Failed to create FileSrc element")?;

        let decode_bin = self
            .decodebin
            .build()
            .context("Failed to create DecodeBin element")?;

        let video_queue_name = format!("video_queue_{}", self.variant_name);
        let video_queue = Self::create_queue(&video_queue_name)
            .context("Failed to create video queue element")?;

        let video_scaler =
            Self::create_element("videoscale").context("Failed to create video scaler element")?;

        let video_caps_filter = self
            .capsfilter
            .build()
            .context("Failed to create CapsFilter element")?;


        let video_encoder = self
            .video_encoder
            .build()
            .context("Failed to create video encoder element")?;

        let h264_parser =
            Self::create_element("h264parse").context("Failed to create h264parse element")?;

        let audio_queue =
            Self::create_queue("audio_queue").context("Failed to create audio queue")?;

        let audio_convert = Self::create_element("audioconvert")?;
        let audio_resample = Self::create_element("audioresample")?;

        let audio_identity = Self::create_element("identity")?;
        audio_identity.set_property("silent", false);

        let audio_encoder = Self::create_element("avenc_aac")?;
        let aac_parser = Self::create_element("aacparse")?;

        let muxer = self
            .mpegtsmux
            .build()
            .context("Failed to create MpegTsMux element")?;

        let hlssink = self
            .hlssink
            .build()
            .context("Failed to create HlsSink3 element")?;

        pipeline.add_many(&[
            &file_source,
            &decode_bin,
            &video_queue,
            &video_scaler,
            &video_caps_filter,
            &video_encoder,
            &h264_parser,
            &audio_queue,
            &audio_convert,
            &audio_resample,
            &audio_identity,
            &audio_encoder,
            &aac_parser,
            &muxer,
            &hlssink,
        ])?;

        file_source
            .link(&decode_bin)
            .context("Failed to link FileSrc to DecodeBin")?;

        let pipeline_weak = pipeline.downgrade();
        decode_bin.connect_pad_added(move |_, src_pad| {
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return,
            };

            let caps = src_pad.current_caps().unwrap();
            let structure = caps.structure(0).unwrap();
            let pad_type = structure.name();

            if pad_type.starts_with("video") {
                let video_queue = pipeline.by_name(&video_queue_name).unwrap();
                let queue_sink_pad = video_queue.static_pad("sink").unwrap();
                src_pad
                    .link(&queue_sink_pad)
                    .context("Failed to link decodebin to video queue")
                    .unwrap();
                gst::Element::link_many(&[
                    &video_queue,
                    &video_scaler,
                    &video_caps_filter,
                    &video_encoder,
                    &h264_parser,
                    &muxer,
                ])
                    .unwrap();
            } else if pad_type.starts_with("audio") {
                let audio_queue = pipeline.by_name("audio_queue").unwrap();
                let queue_sink_pad = audio_queue.static_pad("sink").unwrap();
                src_pad
                    .link(&queue_sink_pad)
                    .context("Failed to link decodebin to audio queue")
                    .unwrap();
                gst::Element::link_many(&[
                    &audio_queue,
                    &audio_convert,
                    &audio_resample,
                    &audio_identity,
                    &audio_encoder,
                    &aac_parser,
                    &muxer,
                ])
                    .unwrap();
            }
        });

        if let Some(muxer) = pipeline.by_name("mpegtsmux") {
            muxer
                .link(&hlssink)
                .context("Failed to link MpegTsMux to HlsSink3")?;
        }

        Ok(pipeline)
    }
}

/// Test suite for HLS pipeline construction and processing.
#[cfg(test)]
mod tests {
    use super::*;
    use gst::init;
    use std::path::PathBuf;
    use std::time::Duration;

    const INPUT_FILE: &str = "test_input.mp4";
    const VARIANT_NAME: &str = "test_variant";
    const PIPELINE_NAME: &str = "test_name";
    const WIDTH: i32 = 1280;
    const HEIGHT: i32 = 720;
    const BITRATE: u32 = 1000000;

    fn create_pipeline_builder() -> PipelineBuilder {
        PipelineBuilder::new(
            INPUT_FILE.to_string(),
            VARIANT_NAME.to_string(),
            PIPELINE_NAME.to_string(),
            WIDTH,
            HEIGHT,
            BITRATE,
            true,
        )
    }

    /// Simulates an HLS generation pipeline and validates its construction and segment generation.
    #[test]
    fn test_pipeline_construction() {
        init().unwrap();

        let pipeline = create_pipeline_builder().build().unwrap();

        assert_pipeline_elements(&pipeline);
    }

    /// Test to simulate a full HLS segment creation process.
    #[test]
    fn test_hls_segment_creation() {
        init().unwrap();

        let pipeline = create_pipeline_builder().build().unwrap();

        pipeline.set_state(gst::State::Playing).unwrap();
        std::thread::sleep(Duration::from_secs(5));

        let hls_sink = pipeline.by_name("hls_sink").unwrap();
        let segment_files = get_segment_files(VARIANT_NAME);

        assert!(segment_files.len() > 0, "No HLS segments were created");
    }

    /// Test to validate state transitions of the pipeline.
    #[test]
    fn test_pipeline_state_transitions() {
        init().unwrap();

        let pipeline = create_pipeline_builder().build().unwrap();

        transition_pipeline_states(&pipeline);
    }

    /// Test to validate captured data using the AppSink element.
    // #[test]
    // fn test_appsink_data_capture() {
    //     init().unwrap();
    //
    //     let pipeline = create_pipeline_builder().build().unwrap();
    //
    //     let appsink = pipeline.by_name("appsink").unwrap();
    //     let appsink = appsink.dynamic_cast::<gst_app::AppSink>().unwrap();
    //
    //     let state = Arc::new(Mutex::new(vec![]));
    //     let state_clone = state.clone();
    //
    //     appsink.set_callbacks(
    //         gst_app::AppSinkCallbacks::builder()
    //             .new_sample(move |sink| {
    //                 let sample = sink.pull_sample().unwrap();
    //                 let buffer = sample.buffer_owned().unwrap();
    //
    //                 state_clone.lock().unwrap().push(buffer);
    //                 Ok(gst::FlowSuccess::Ok)
    //             })
    //             .build(),
    //     );
    //
    //     pipeline.set_state(gst::State::Playing).unwrap();
    //     std::thread::sleep(Duration::from_secs(5));
    //
    //     assert!(
    //         state.lock().unwrap().len() > 0,
    //         "No data was captured in the AppSink"
    //     );
    //
    //     pipeline.set_state(gst::State::Null).unwrap();
    // }

    fn assert_pipeline_elements(pipeline: &gst::Pipeline) {
        assert!(pipeline.by_name("filesrc").is_some());
        assert!(pipeline.by_name("decodebin").is_some());
        assert!(pipeline.by_name("video_encoder").is_some());
        assert!(pipeline.by_name("capsfilter").is_some());
        assert!(pipeline.by_name("hls_sink").is_some());
    }

    fn get_segment_files(variant_name: &str) -> Vec<PathBuf> {
        std::fs::read_dir(variant_name)
            .unwrap()
            .map(|res| res.unwrap().path())
            .collect()
    }

    fn transition_pipeline_states(pipeline: &gst::Pipeline) {
        pipeline.set_state(gst::State::Ready).unwrap();
        std::thread::sleep(Duration::from_secs(1));

        pipeline.set_state(gst::State::Paused).unwrap();
        std::thread::sleep(Duration::from_secs(1));

        pipeline.set_state(gst::State::Playing).unwrap();
        std::thread::sleep(Duration::from_secs(2));

        pipeline.set_state(gst::State::Null).unwrap();
    }
}
