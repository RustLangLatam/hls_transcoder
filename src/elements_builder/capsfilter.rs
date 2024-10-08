use anyhow::Result;
use gst::caps::{Builder, NoFeature};
use gst::prelude::*;
use gst::{Caps, Element, ElementFactory};

/// A builder for configuring and creating the `capsfilter` GStreamer element.
///
/// The builder provides an interface for setting properties like width, height, and format,
/// allowing for flexible configuration of the capabilities filter based on user requirements.
#[derive(Debug)]
pub struct CapsFilterBuilder {
    element: Element,
    caps: Builder<NoFeature>,
}

impl CapsFilterBuilder {
    /// Creates a new `CapsFilterBuilder` instance with the specified media type.
    ///
    /// # Arguments
    ///
    /// * `media_type`: The media type of the caps filter (e.g., "video/x-raw").
    pub fn new(media_type: &str) -> Self {
        // Create a new caps filter element
        let element = ElementFactory::make_with_name("capsfilter", Some("capsfilter"))
            .expect("Failed to create capsfilter element");

        // Initialize the caps builder with the specified media type
        let caps = Caps::builder(media_type);

        Self { element, caps }
    }

    /// Sets the width property for the `caps` of the `capsfilter` element.
    ///
    /// # Arguments
    ///
    /// * `width`: The width of the video in pixels.
    pub fn with_width(mut self, width: i32) -> Self {
        self.caps = self.caps.field("width", width);
        self
    }

    /// Sets the height property for the `caps` of the `capsfilter` element.
    ///
    /// # Arguments
    ///
    /// * `height`: The height of the video in pixels.
    pub fn with_height(mut self, height: i32) -> Self {
        self.caps = self.caps.field("height", height);
        self
    }

    /// Sets the framerate property for the `caps` of the `capsfilter` element.
    ///
    /// # Arguments
    ///
    /// * `framerate`: The framerate of the video as a tuple of (numerator, denominator).
    pub fn with_framerate(mut self, framerate: (i32, i32)) -> Self {
        self.caps = self.caps.field("framerate", gst::Fraction::new(framerate.0, framerate.1));
        self
    }

    /// Sets the profile property for the `caps` of the `capsfilter` element.
    ///
    /// # Arguments
    ///
    /// * `profile`: The profile to use for the video (e.g., "high", "baseline").
    pub fn with_profile(mut self, profile: &str) -> Self {
        self.caps = self.caps.field("profile", profile);
        self
    }

    /// Sets the format property for the `caps` of the `capsfilter` element.
    ///
    /// # Arguments
    ///
    /// * `format`: The pixel format of the video (e.g., "I420", "NV12").
    pub fn with_format(mut self, format: &str) -> Self {
        self.caps = self.caps.field("format", format);
        self
    }

    /// Builds and returns the configured `capsfilter` instance.
    ///
    /// The `caps` property of the element is set based on the configured values.
    ///
    /// # Returns
    ///
    /// A `Result` containing the built `gst::Element` instance or an error if the element could not be created.
    pub fn build(mut self) -> Result<Element> {
        // Build the caps and set it to the capsfilter element
        let caps = self.caps.build();
        self.element.set_property("caps", &caps);

        Ok(self.element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst::init;

    #[test]
    fn test_capsfilter_builder_with_default() {
        init().unwrap();
        let capsfilter = CapsFilterBuilder::new("video/x-raw")
            .with_width(1920)
            .with_height(1080)
            .with_format("I420")
            .build()
            .unwrap();

        assert_eq!(
            capsfilter.property::<gst::Caps>("caps").to_string(),
            "video/x-raw, width=(uint)1920, height=(uint)1080, format=(string)I420"
        );
    }

    #[test]
    fn test_capsfilter_builder_with_framerate() {
        init().unwrap();
        let capsfilter = CapsFilterBuilder::new("video/x-raw")
            .with_width(1280)
            .with_height(720)
            .with_format("NV12")
            .with_framerate((30, 1))
            .build()
            .unwrap();

        assert_eq!(
            capsfilter.property::<gst::Caps>("caps").to_string(),
            "video/x-raw, width=(uint)1280, height=(uint)720, format=(string)NV12, framerate=(fraction)30/1"
        );
    }

    #[test]
    fn test_capsfilter_builder_with_profile() {
        init().unwrap();
        let capsfilter = CapsFilterBuilder::new("video/x-h264")
            .with_width(640)
            .with_height(480)
            .with_profile("high")
            .build()
            .unwrap();

        assert_eq!(
            capsfilter.property::<gst::Caps>("caps").to_string(),
            "video/x-h264, width=(uint)640, height=(uint)480, profile=(string)high"
        );
    }

    #[test]
    fn test_capsfilter_builder_with_invalid_format() {
        init().unwrap();
        let capsfilter = CapsFilterBuilder::new("video/x-raw")
            .with_format("INVALID_FORMAT")
            .build();

        assert!(capsfilter.is_err(), "Invalid format should fail");
    }
}
