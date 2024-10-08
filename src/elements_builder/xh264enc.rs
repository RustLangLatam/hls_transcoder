//! # `Xh264EncBuilder` Module
//!
//! This module provides a builder for creating and configuring the `nvh264enc` GStreamer element,
//! which is used for hardware-accelerated encoding of H.264 video using NVIDIA's NVENC technology.
//! The `nvh264enc` element offers various properties for adjusting encoding parameters like bitrate,
//! rate control, profile, and keyframe intervals.
//!
//! ## Properties Explained
//!
//! Below is a summary of the properties that can be set using `Xh264EncBuilder`:
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

/// Enum for the valid preset values of `nvh264enc`.
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
    /// Returns the corresponding string value for the preset.
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

/// Enum for the valid rate control modes of `nvh264enc`.
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
    /// Returns the corresponding string value for the rate control mode.
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
///
/// This builder provides an interface for setting properties like bitrate, GOP size,
/// rate control mode, preset, and profile, allowing for flexible configuration
/// of the encoder based on the user's requirements.
#[derive(Debug, Clone)]
pub struct Xh264EncBuilder {
    element: gst::Element,
}

impl Xh264EncBuilder {
    /// Creates a new `Xh264EncBuilder` instance with the default properties for `nvh264enc`.
    ///
    /// This method initializes the `nvh264enc` element without setting any specific properties,
    /// allowing the builder methods to be called to set properties as needed.
    pub fn new() -> Self {
        let element = gst::ElementFactory::make_with_name("x264enc", Some("video_encoder")).unwrap();
        element.set_property_from_str("speed-preset", "superfast"); // Use 'superfast' for fast encoding
        element.set_property_from_str("tune", "zerolatency"); // Minimize latency for live streaming
        element.set_property("threads", 16u32); // Set to the number of CPU cores, e.g., 4 threads
        element.set_property("key-int-max", 30u32);  // Sets keyframe interval to every 30 frames

        Self { element }
    }

    /// Creates a new `Xh264EncBuilder` instance with pre-configured default properties.
    ///
    /// This method sets default values for bitrate, rate control mode, B-frames, etc.
    pub fn default() -> Self {
        let mut builder = Self::new();

        // builder = builder
        // .with_bitrate(1000)
        // .with_gop_size(75)
        // .with_bframes(0);
        // .with_preset(NvPreset::Hp)
        // .with_rate_control(NvRateControl::Cbr)
        // .with_zero_latency(false);
        builder
    }

    /// Sets the `zerolatency` property of the `nvh264enc` element.
    pub fn with_zero_latency(mut self, zero_latency: bool) -> Self {
        self.element.set_property("zerolatency", zero_latency);
        self
    }
}

impl H264EncBuilder for Xh264EncBuilder {
    const VALID_PROFILES: &'static [&'static str] = &[];

    /// Sets the `bitrate` property of the `nvh264enc` element.
    fn with_bitrate(&mut self, bitrate: u32) -> &mut Self {
        // Ensure the bitrate does not exceed the maximum allowed value of 2,048,000.
        let capped_bitrate = if bitrate > 2_048_000 {
            2_048_000
        } else {
            bitrate
        };

        // If the bitrate is greater than 1,000,000 (1 Mbps), divide it by 1,000 to convert to kbps.
        let final_bitrate = if capped_bitrate > 1_000_000 {
            capped_bitrate / 1_000
        } else {
            capped_bitrate
        };

        self.element.set_property("bitrate", final_bitrate);
        self
    }

    /// Sets the `bframes` property of the `nvh264enc` element.
    fn with_bframes(&mut self, bframes: u32) -> &mut Self {
        // Ensure that the value of bframes is within the valid range of 0 to 4.
        let bframes = if bframes > 4 { 4 } else { bframes };

        self.element.set_property("bframes", bframes);
        self
    }

    /// Sets the `gop-size` property of the `nvh264enc` element.
    fn with_gop_size(&mut self, size: i32) -> &mut Self {
        // Cap the size value to ensure it is within the valid range of -1 to 2147483647.
        let gop_size = if size < -1 {
            -1
        } else if size > 2_147_483_647 {
            2_147_483_647
        } else {
            size
        };

        self.element.set_property("gop-size", gop_size);
        self
    }

    /// Sets the `rate-control` property of the `nvh264enc` element using a strongly-typed enum.
    fn with_rate_control_mode(&mut self, rate_control: &str) -> &mut Self {
        self.element.set_property_from_str("rc-mode", rate_control);
        self
    }
    /// Sets the `preset` property of the `nvh264enc` element using a strongly-typed enum.
    fn with_preset(&mut self, preset: &str) -> &mut Self {
        self.element.set_property_from_str("preset", preset);
        self
    }

    fn with_profile(&mut self, profile: &str) -> Result<&mut Self> {
        todo!()
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
        let builder = Xh264EncBuilder::default();
        assert_default_encoder_properties(&builder);
    }

    #[test]
    fn test_encoder_builder_with_bitrate() {
        init().unwrap();
        let mut builder = Xh264EncBuilder::default();
        builder.with_bitrate(1_000_000);
        assert_eq!(builder.element.property::<u32>("bitrate"), 1_000_000);
    }

    #[test]
    fn test_encoder_builder_with_gop_size() {
        init().unwrap();
        let mut builder = Xh264EncBuilder::default();
        builder.with_gop_size(30);
        assert_eq!(builder.element.property::<i32>("gop-size"), 30);
    }

    #[test]
    fn test_encoder_builder_with_preset() {
        init().unwrap();
        let mut builder = Xh264EncBuilder::default();
        builder.with_preset(crate::elements_builder::nvh264enc::NvPreset::Default.as_str());
        assert_preset_property(&builder, "default");
    }

    #[test]
    fn test_encoder_builder_with_rate_control() {
        init().unwrap();
        let mut builder = Xh264EncBuilder::default();
        builder.with_rate_control_mode(crate::elements_builder::nvh264enc::NvRateControl::Cbr.as_str());
        assert_rate_control_property(&builder, "cbr");
    }

    #[test]
    fn test_encoder_builder_build() {
        init().unwrap();
        let result = Xh264EncBuilder::default().build();
        assert!(result.is_ok());
    }

    fn assert_default_encoder_properties(builder: &Xh264EncBuilder) {
        assert_eq!(builder.element.property::<u32>("bitrate"), 1000);
        assert_eq!(builder.element.property::<i32>("gop-size"), 75);
        assert_preset_property(builder, "hp");
        assert_rate_control_property(builder, "cbr");
    }

    fn assert_preset_property(builder: &Xh264EncBuilder, expected: &str) {
        assert_eq!(
            builder
                .element
                .property::<Value>("preset")
                .serialize()
                .unwrap(),
            expected
        );
    }

    fn assert_rate_control_property(builder: &Xh264EncBuilder, expected: &str) {
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