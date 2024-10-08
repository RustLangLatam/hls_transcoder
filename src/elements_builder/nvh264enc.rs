//! # `NVH264EncBuilder` Module
//!
//! This module provides a builder for creating and configuring the `nvh264enc` GStreamer element,
//! which is used for hardware-accelerated encoding of H.264 video using NVIDIA's NVENC technology.
//! The `nvh264enc` element offers various properties for adjusting encoding parameters like bitrate,
//! rate control, profile, and keyframe intervals.
//!
//! ## Properties Explained
//!
//! Below is a summary of the properties that can be set using `NVH264EncBuilder`:
//!
//! 1. **`bitrate`**: The target bitrate for encoding in bits per second.
//!    - **Description**: This property sets the average bitrate of the encoded video.
//!    - **Default Value**: 2000 Kbps.
//!    - **Documentation Reference**: [NVH264Enc Bitrate](https://gstreamer.freedesktop.org/documentation/nvcodec/nvh264enc.html?gi-language=c#GstNvH264Enc:bitrate)
//!
//! 2. **`gop-size`**: Sets the Group of Pictures (GOP) size.
//!    - **Description**: The interval between keyframes. A lower value results in more frequent keyframes, which increases bitrate but reduces seek time.
//!    - **Default Value**: 30 (frames).
//!    - **Documentation Reference**: [NVH264Enc GOP Size](https://gstreamer.freedesktop.org/documentation/nvcodec/nvh264enc.html?gi-language=c#GstNvH264Enc:gop-size)
//!
//! 3. **`preset`**: Sets the encoder quality preset.
//!    - **Description**: Defines the trade-off between encoding speed and quality. Presets include `ultrafast`, `fast`, `medium`, `slow`, etc.
//!    - **Default Value**: "medium".
//!    - **Documentation Reference**: [NVH264Enc Preset](https://gstreamer.freedesktop.org/documentation/nvcodec/nvh264enc.html?gi-language=c#GstNvH264Enc:preset)
//!
//! 4. **`profile`**: Sets the H.264 encoding profile.
//!    - **Description**: Defines the subset of H.264 features to be used, such as `baseline`, `main`, or `high`.
//!    - **Default Value**: "main".
//!    - **Documentation Reference**: [NVH264Enc Profile](https://gstreamer.freedesktop.org/documentation/nvcodec/nvh264enc.html?gi-language=c#GstNvH264Enc:profile)
//!
//! 5. **`rc-mode`**: Sets the rate control mode, such as `cbr` (constant bit rate) or `vbr` (variable bit rate).
//!    - **Description**: Defines how the bitrate is adjusted over time.
//!    - **Default Value**: "cbr".
//!    - **Documentation Reference**: [NVH264Enc RC Mode](https://gstreamer.freedesktop.org/documentation/nvcodec/nvh264enc.html?gi-language=c#GstNvH264Enc:rc-mode)
//!
//! For more information and advanced usage scenarios, refer to the [official GStreamer NVH264Enc Documentation](https://gstreamer.freedesktop.org/documentation/nvcodec/nvh264enc.html?gi-language=c#nvh264enc-page).

use crate::elements_builder::H264EncBuilder;
use anyhow::Result;
use gst::prelude::*;
use gst::ElementFactory;

/// Enum for valid preset values of `nvh264enc`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvPreset {
    Default,
    Hp,
    Hq,
    LowLatency,
    LowLatencyHq,
    LowLatencyHp,
    Lossless,
    LosslessHp,
}

impl NvPreset {
    pub fn as_str(&self) -> &'static str {
        match self {
            NvPreset::Default => "default",
            NvPreset::Hp => "hp",
            NvPreset::Hq => "hq",
            NvPreset::LowLatency => "low-latency",
            NvPreset::LowLatencyHq => "low-latency-hq",
            NvPreset::LowLatencyHp => "low-latency-hp",
            NvPreset::Lossless => "lossless",
            NvPreset::LosslessHp => "lossless-hp",
        }
    }
}

/// Enum for valid rate control modes of `nvh264enc`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvRateControl {
    Cqp,
    Vbr,
    Cbr,
    CbrLdHq,
    CbrHq,
    VbrHq,
}

impl NvRateControl {
    pub fn as_str(&self) -> &'static str {
        match self {
            NvRateControl::Cqp => "cqp",
            NvRateControl::Vbr => "vbr",
            NvRateControl::Cbr => "cbr",
            NvRateControl::CbrLdHq => "cbr-ld-hq",
            NvRateControl::CbrHq => "cbr-hq",
            NvRateControl::VbrHq => "vbr-hq",
        }
    }
}

/// A builder for configuring and creating the `nvh264enc` GStreamer element.
#[derive(Debug, Clone)]
pub struct NVH264EncBuilder {
    element: gst::Element,
}

impl NVH264EncBuilder {
    /// Creates a new `NVH264EncBuilder` instance with the default properties for `nvh264enc`.
    pub fn new() -> Self {
        let element = ElementFactory::make_with_name("nvh264enc", Some("video_encoder"))
            .expect("Failed to create nvh264enc element");
        Self { element }
    }

    /// Creates a new `NVH264EncBuilder` instance with pre-configured default properties.
    pub fn default() -> Self {
        let mut builder = Self::new();
        let element = builder
            .with_bitrate(1000)
            .with_gop_size(75)
            .with_bframes(0)
            .with_preset(NvPreset::Hp.as_str())
            .with_rate_control_mode(NvRateControl::Cbr.as_str());

        Self { element: element.element.clone() }
    }

    /// Sets the `zerolatency` property of the `nvh264enc` element.
    pub fn with_zero_latency(mut self, zero_latency: bool) -> Self {
        self.element.set_property("zerolatency", zero_latency);
        self
    }
}

impl H264EncBuilder for NVH264EncBuilder {
    const VALID_PROFILES: &'static [&'static str] = &[
        "main",
        "high",
        "high-4:4:4",
        "baseline",
        "constrained-baseline",
    ];

    fn with_bitrate(&mut self, bitrate: u32) -> &mut Self {
        let capped_bitrate = bitrate.min(2_048_000);
        let final_bitrate = if capped_bitrate > 1_000_000 {
            capped_bitrate / 1_000
        } else {
            capped_bitrate
        };
        self.element.set_property("bitrate", final_bitrate);
        self
    }

    fn with_bframes(&mut self, bframes: u32) -> &mut Self {
        let bframes = bframes.min(4);
        self.element.set_property("bframes", bframes);
        self
    }

    fn with_gop_size(&mut self, size: i32) -> &mut Self {
        let gop_size = size.clamp(-1, 2_147_483_647);
        self.element.set_property("gop-size", gop_size);
        self
    }

    fn with_rate_control_mode(&mut self, rate_control: &str) -> &mut Self {
        self.element.set_property_from_str("rc-mode", rate_control);
        self
    }

    fn with_preset(&mut self, preset: &str) -> &mut Self {
        self.element.set_property_from_str("preset", preset);
        self
    }

    fn with_profile(&mut self, profile: &str) -> Result<&mut Self> {
        if Self::VALID_PROFILES.contains(&profile) {
            self.element.set_property("profile", profile);
            Ok(self)
        } else {
            Err(anyhow::anyhow!(
                "Invalid profile: {}. Valid options are: {:?}",
                profile,
                Self::VALID_PROFILES
            ))
        }
    }

    fn build(&self) -> Result<gst::Element> {
        Ok(self.element.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst::glib::Value;
    use gst::init;

    #[test]
    fn test_default_encoder_builder() {
        init().unwrap();
        let builder = NVH264EncBuilder::default();
        assert_default_encoder_properties(&builder);
    }

    #[test]
    fn test_encoder_builder_with_bitrate() {
        init().unwrap();
        let mut builder = NVH264EncBuilder::default();
        builder.with_bitrate(1_000_000);
        assert_eq!(builder.element.property::<u32>("bitrate"), 1_000_000);
    }

    #[test]
    fn test_encoder_builder_with_gop_size() {
        init().unwrap();
        let mut builder = NVH264EncBuilder::default();
        builder.with_gop_size(30);
        assert_eq!(builder.element.property::<i32>("gop-size"), 30);
    }

    #[test]
    fn test_encoder_builder_with_preset() {
        init().unwrap();
        let mut builder = NVH264EncBuilder::default();
        builder.with_preset(NvPreset::Default.as_str());
        assert_preset_property(&builder, "default");
    }

    #[test]
    fn test_encoder_builder_with_rate_control() {
        init().unwrap();
        let mut builder = NVH264EncBuilder::default();
        builder.with_rate_control_mode(NvRateControl::Cbr.as_str());
        assert_rate_control_property(&builder, "cbr");
    }

    #[test]
    fn test_encoder_builder_build() {
        init().unwrap();
        let result = NVH264EncBuilder::default().build();
        assert!(result.is_ok());
    }

    fn assert_default_encoder_properties(builder: &NVH264EncBuilder) {
        assert_eq!(builder.element.property::<u32>("bitrate"), 1000);
        assert_eq!(builder.element.property::<i32>("gop-size"), 75);
        assert_preset_property(builder, "hp");
        assert_rate_control_property(builder, "cbr");
    }

    fn assert_preset_property(builder: &NVH264EncBuilder, expected: &str) {
        assert_eq!(
            builder
                .element
                .property::<Value>("preset")
                .serialize()
                .unwrap(),
            expected
        );
    }

    fn assert_rate_control_property(builder: &NVH264EncBuilder, expected: &str) {
        assert_eq!(
            builder
                .element
                .property::<Value>("rc-mode")
                .serialize()
                .unwrap(),
            expected
        );
    }
}

