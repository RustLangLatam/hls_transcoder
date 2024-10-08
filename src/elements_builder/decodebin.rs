//! # `DecodeBinBuilder` Module
//!
//! This module provides a builder for creating and configuring the `decodebin` GStreamer element, which is
//! designed to automatically select appropriate decoders and demuxers for media playback and processing.
//!
//! The `DecodeBinBuilder` is a flexible and intuitive way to set up the `decodebin` element with various
//! properties such as `caps`, buffering, sink properties, and more. This builder adheres to GStreamer best practices
//! and provides detailed configuration options for different use cases.
//!
//! ## Properties Explained
//!
//! Below is a summary of the properties that can be set using `DecodeBinBuilder`:
//!
//! 1. **`caps`**: Sets the `caps` property to specify the acceptable media data type. This ensures that the `decodebin`
//!    element only processes streams that match the given capabilities (e.g., `video/x-raw`, `audio/x-raw`).
//!    - **Documentation Reference**: [GStreamer Caps](https://gstreamer.freedesktop.org/documentation/additional/design/caps.html?gi-language=c)
//!
//! 2. **`use-buffering`**: Enables or disables buffering within the `decodebin`. Buffering is used for handling live
//!    or network sources where data may not arrive consistently.
//!    - **Default Value**: `false` (buffering is disabled by default).
//!    - **Documentation Reference**: [GStreamer Decodebin Buffering](https://gstreamer.freedesktop.org/documentation/additional/design/buffering.html?gi-language=c)
//!
//! 3. **`expose-all-streams`**: Determines whether `decodebin` should expose all detected streams, even if they
//!    are not decodable by the current plugins. This is useful for inspecting streams or extracting metadata.
//!    - **Default Value**: `true` (all streams are exposed by default).
//!    - **Documentation Reference**: [GStreamer Decodebin Design](https://gstreamer.freedesktop.org/documentation/playback/decodebin.html?gi-language=c#decodebin:expose-all-streams)
//!
//! 4. **`sink-caps`**: Defines the format for the sink pads created by `decodebin`, such as `audio/x-raw` or `video/x-raw`.
//!    This is used to control the format and capabilities of the output stream.
//!
//! 5. **`sink-properties`**: Sets additional properties on the sink pads, which control pad-specific behavior.
//!
//! For a comprehensive overview of all properties and their effects, refer to the [official GStreamer Decodebin Documentation](https://gstreamer.freedesktop.org/documentation/additional/design/decodebin.html?gi-language=c).

use anyhow::Result;
use gst::prelude::*;

/// A builder for creating and configuring a `decodebin` GStreamer element.
///
/// The `DecodeBinBuilder` provides an easy-to-use interface for setting up and configuring
/// the `decodebin` element with various properties like caps, buffering, and sink options.
/// It follows the GStreamer best practices and allows users to build a tailored `decodebin`
/// element for different streaming and processing needs.
#[derive(Debug, Clone)]
pub struct DecodeBinBuilder {
    element: gst::Element,
}

impl DecodeBinBuilder {
    /// Creates a new `DecodeBinBuilder` instance with default properties.
    ///
    /// The `decodebin` element is created and initialized with default properties for optimal performance.
    ///
    /// Returns a new `DecodeBinBuilder` instance.
    pub fn new() -> Self {
        DecodeBinBuilder {
            element: gst::ElementFactory::make_with_name("decodebin", Some("decodebin"))
                .expect("Failed to create decodebin element"),
        }
    }

    /// Sets the `caps` property of the `decodebin` element to filter incoming streams.
    ///
    /// # Arguments
    ///
    /// * `caps`: A string representing the `gst::Caps` to set (e.g., "video/x-raw").
    ///
    /// This method configures the acceptable caps for the `decodebin` element,
    /// ensuring that it only processes streams matching these caps.
    pub fn with_caps(mut self, caps: &str) -> Self {
        self.element.set_property("caps", &gst::Caps::builder(caps).build());
        self
    }

    /// Enables or disables buffering for the `decodebin` element.
    ///
    /// # Arguments
    ///
    /// * `enabled`: A boolean indicating whether to use buffering.
    ///
    /// When enabled, buffering is used to handle live sources or network streams.
    pub fn with_buffering(mut self, enabled: bool) -> Self {
        self.element.set_property("use-buffering", enabled);
        self
    }

    /// Sets the `expose-all-streams` property to expose all detected streams.
    ///
    /// # Arguments
    ///
    /// * `expose_all`: A boolean indicating whether to expose all streams found.
    ///
    /// When enabled, `decodebin` will expose all streams found, including non-decoded ones.
    pub fn with_expose_all_streams(mut self, expose_all: bool) -> Self {
        self.element.set_property("expose-all-streams", expose_all);
        self
    }

    /// Sets the `sink-caps` property to control the format of the output streams.
    ///
    /// # Arguments
    ///
    /// * `caps`: A string representing the `gst::Caps` to set (e.g., "audio/x-raw").
    ///
    /// The caps define the format of the accepted sink pads.
    pub fn with_sink_caps(mut self, caps: &str) -> Self {
        self.element.set_property("sink-caps", &gst::Caps::builder(caps).build());
        self
    }

    /// Sets multiple sink properties on the `decodebin` element.
    ///
    /// # Arguments
    ///
    /// * `properties`: A vector of tuples containing the property name and value.
    pub fn with_sink_properties(mut self, properties: Vec<(String, gst::glib::Value)>) -> Self {
        for (name, value) in properties {
            self.element.set_property_from_value(&name, &value);
        }
        self
    }

    /// Builds and returns the configured `decodebin` instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the built `gst::Element` instance or an error if the element could not be created.
    pub fn build(self) -> Result<gst::Element, anyhow::Error> {
        Ok(self.element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst::init;

    #[test]
    fn test_builder_default() {
        init().unwrap();
        let decodebin = DecodeBinBuilder::new().build().unwrap();

        assert_eq!(decodebin.property::<bool>("use-buffering"), false);
        assert_eq!(decodebin.property::<bool>("expose-all-streams"), true);
    }

    #[test]
    fn test_builder_with_custom_values() {
        init().unwrap();
        let decodebin = DecodeBinBuilder::new()
            .with_caps("video/x-raw")
            .with_buffering(true)
            .with_expose_all_streams(false)
            .with_sink_caps("audio/x-raw")
            .build()
            .unwrap();

        assert_eq!(decodebin.property::<bool>("use-buffering"), true);
        assert_eq!(decodebin.property::<bool>("expose-all-streams"), false);
        assert_eq!(decodebin.property::<gst::Caps>("sink-caps").to_string(), "audio/x-raw");
    }
}
