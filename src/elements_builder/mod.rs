pub mod capsfilter;
pub mod decodebin;
pub mod filesrc;
pub mod hlssink3;
pub mod mpegtsmux;
pub mod nvh264enc;
pub mod xh264enc;

pub trait H264EncBuilder {
    const VALID_PROFILES: &'static [&'static str];

    /// Sets the `bitrate` property of the `nvh264enc` element.
    fn with_bitrate(&mut self, bitrate: u32) -> &mut Self;

    /// Sets the `bframes` property of the `nvh264enc` element.
    fn with_bframes(&mut self, bframes: u32) -> &mut Self;

    /// Sets the `gop-size` property of the `nvh264enc` element.
    fn with_gop_size(&mut self, gop_size: i32) -> &mut Self;

    /// Sets the `rate-control` property of the `nvh264enc` element using a strongly-typed enum.
    fn with_rate_control_mode(&mut self, mode: &str) -> &mut Self;

    /// Sets the `preset` property of the `h264enc` element using a strongly-typed enum.
    fn with_preset(&mut self, preset: &str) -> &mut Self;

    /// Sets the `profile` property of the encoder element.
    ///
    /// Validates that the profile is one of the allowed values: `main`, `high`, `high-4:4:4`,
    /// `baseline`, or `constrained-baseline`. If the profile is not valid, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `profile`: The profile to set for the encoder element.
    ///
    /// # Returns
    ///
    /// Returns the mutable reference to the builder for method chaining.
    fn with_profile(&mut self, profile: &str) -> anyhow::Result<&mut Self>;

    /// Builds and returns the configured `nvh264enc` instance.
    fn build(&self) -> anyhow::Result<gst::Element>;
}
