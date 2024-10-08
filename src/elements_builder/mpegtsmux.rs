use anyhow::Result;
use gst::prelude::*;
use gst::ElementFactory;

/// A builder for configuring and creating the `mpegtsmux` GStreamer element.
///
/// The builder provides an interface for setting properties such as `alignment`,
/// `pat-interval`, `pmt-interval`, `muxrate`, and `pcr-interval` to create an
/// optimized MPEG-TS muxer for streaming.
#[derive(Debug, Clone)]
pub struct MpegTsMuxBuilder {
    element: gst::Element,
}

impl MpegTsMuxBuilder {
    /// Creates a new `MpegTsMuxBuilder` instance with default properties.
    ///
    /// The `mpegtsmux` element is created and initialized with default properties
    /// for optimal performance.
    pub fn new() -> Self {
        let element = ElementFactory::make_with_name("mpegtsmux", Some("mpegtsmux"))
            .expect("Failed to create mpegtsmux element");

        // Set default properties
        element.set_property("alignment", 1); // Ensures alignment of TS packets
        element.set_property("pat-interval", 500u32); // Update PAT every 500 ms
        element.set_property("pmt-interval", 500u32); // Update PMT every 500 ms
        element.set_property("pcr-interval", 20u32); // PCR interval of 20 ms

        Self { element }
    }

    /// Sets the `alignment` property of the `mpegtsmux` element.
    pub fn with_alignment(mut self, alignment: i32) -> Self {
        self.element.set_property("alignment", alignment);
        self
    }

    /// Sets the `pat-interval` property of the `mpegtsmux` element.
    pub fn with_pat_interval(mut self, interval: u32) -> Self {
        self.element.set_property("pat-interval", interval);
        self
    }

    /// Sets the `pmt-interval` property of the `mpegtsmux` element.
    pub fn with_pmt_interval(mut self, interval: u32) -> Self {
        self.element.set_property("pmt-interval", interval);
        self
    }

    /// Sets the `pcr-interval` property of the `mpegtsmux` element.
    pub fn with_pcr_interval(mut self, interval: u32) -> Self {
        self.element.set_property("pcr-interval", interval);
        self
    }

    /// Enables or disables the `m2ts-mode` property of the `mpegtsmux` element.
    pub fn with_m2ts_mode(mut self, enabled: bool) -> Self {
        self.element.set_property("m2ts-mode", enabled);
        self
    }


    /// Builds and returns the configured `mpegtsmux` instance.
    pub fn build(self) -> Result<gst::Element> {
        Ok(self.element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst::init;

    #[test]
    fn test_mpegtsmux_builder_default() {
        init().unwrap();
        let muxer = MpegTsMuxBuilder::new().build().unwrap();

        assert_eq!(muxer.property::<i32>("alignment"), 1);
        assert_eq!(muxer.property::<u32>("pat-interval"), 500);
        assert_eq!(muxer.property::<u32>("pmt-interval"), 500);
        assert_eq!(muxer.property::<u32>("pcr-interval"), 20);
    }

    #[test]
    fn test_mpegtsmux_builder_custom_values() {
        init().unwrap();
        let muxer = MpegTsMuxBuilder::new()
            .with_alignment(1)
            .with_pat_interval(1000)
            .with_pmt_interval(1000)
            .with_pcr_interval(40)
            .build()
            .unwrap();

        assert_eq!(muxer.property::<i32>("alignment"), 1);
        assert_eq!(muxer.property::<u32>("pat-interval"), 1000);
        assert_eq!(muxer.property::<u32>("pmt-interval"), 1000);
        assert_eq!(muxer.property::<u32>("pcr-interval"), 40);
    }
}
