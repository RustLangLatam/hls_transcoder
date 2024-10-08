use anyhow::Result;
use gst::prelude::*;
use gst::ElementFactory;
use gst_hlssink3::hlssink3::HlsSink3PlaylistType;

/// A builder for configuring and creating the `hlssink3` GStreamer element.
///
/// The builder provides an interface for setting properties like `playlist-location`,
/// `location`, `target-duration`, `playlist-length`, `max-files`, and `playlist-type`,
/// allowing for flexible configuration of the HLS encoder based on user requirements.
#[derive(Debug, Clone)]
pub struct HlsSink3Builder {
    element: gst::Element,
}

impl HlsSink3Builder {
    /// Creates a new `HlsSink3EncoderBuilder` instance with default properties.
    ///
    /// The `hlssink3` element is created and initialized with default properties
    /// suitable for general HLS encoding tasks.
    pub fn new(location: &str, playlist_location: &str) -> Self {
        let element = ElementFactory::make_with_name("hlssink", Some("hls_sink"))
            .expect("Failed to create hlssink3 element");


        // Set default values for HLS properties
        element.set_property("target-duration", 5u32); // Default segment duration is 5 seconds
        element.set_property("playlist-length", 0u32); // Default playlist length is 10 segments
        element.set_property("max-files", 0u32); // Keep a maximum of 5 segments at a time
        // element.set_property("playlist-type", HlsSink3PlaylistType::Vod); // Default playlist type is VoD
        element.set_property("location", format!("{}/segment_%02d.ts", location));
        element.set_property("playlist-location", format!("{}/playlist.m3u8", playlist_location));

        Self { element }
    }

    /// Sets the `playlist-location` property of the `hlssink3` element.
    ///
    /// # Arguments
    ///
    /// * `playlist_location`: The location (file path) of the main HLS playlist file.
    pub fn with_playlist_location(mut self, playlist_location: &str) -> Self {
        self.element
            .set_property("playlist-location", playlist_location);
        self
    }

    /// Sets the `location` property of the `hlssink3` element.
    ///
    /// # Arguments
    ///
    /// * `segment_location`: The location (file path) pattern for the HLS segments.
    ///
    /// This path should include a placeholder like `%05d.ts` to indicate the segment numbering.
    pub fn with_segment_location(mut self, segment_location: &str) -> Self {
        self.element.set_property("location", segment_location);
        self
    }

    /// Sets the `target-duration` property of the `hlssink3` element.
    ///
    /// # Arguments
    ///
    /// * `duration`: The target duration for each HLS segment in seconds.
    ///
    /// Specifies the approximate length of each HLS segment, typically in the range of 2-10 seconds.
    pub fn with_target_duration(mut self, duration: u32) -> Self {
        self.element.set_property("target-duration", duration);
        self
    }

    /// Sets the `playlist-length` property of the `hlssink3` element.
    ///
    /// # Arguments
    ///
    /// * `length`: The number of segments to be maintained in the playlist.
    ///
    /// Determines the number of media segments listed in the HLS playlist at any given time.
    pub fn with_playlist_length(mut self, length: u32) -> Self {
        self.element.set_property("playlist-length", length);
        self
    }

    /// Sets the `max-files` property of the `hlssink3` element.
    ///
    /// # Arguments
    ///
    /// * `max_files`: The maximum number of segments to keep at a time.
    ///
    /// Older segments will be deleted once this limit is reached, reducing storage space usage.
    pub fn with_max_files(mut self, max_files: u32) -> Self {
        self.element.set_property("max-files", max_files);
        self
    }

    /// Sets the `playlist-type` property of the `hlssink3` element.
    ///
    /// # Arguments
    ///
    /// * `playlist_type`: The type of playlist, either "event", "vod", or "live".
    ///
    /// The playlist type defines whether the playlist is for a live stream or video on demand.
    pub fn with_playlist_type(mut self, playlist_type: HlsSink3PlaylistType) -> Result<Self> {
        if playlist_type != HlsSink3PlaylistType::Unspecified {
            self.element
                .set_property("playlist-type", playlist_type);
            Ok(self)
        } else {
            Err(anyhow::anyhow!("Invalid playlist type: {:?}", playlist_type))
        }
    }

    /// Builds and returns the configured `hlssink3` instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the built `gst::Element` instance or an error if the element could not be created.
    pub fn build(self) -> Result<gst::Element> {
        Ok(self.element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst::init;

    #[test]
    fn test_hlssink3_encoder_builder_new() {
        init().unwrap();
        let builder = HlsSink3Builder::new("output/segment_%05d.ts", "output/playlist.m3u8");
        assert_eq!(builder.element.property::<u32>("target-duration"), 5);
        assert_eq!(builder.element.property::<u32>("playlist-length"), 10);
        assert_eq!(builder.element.property::<u32>("max-files"), 5);
        assert_eq!(
            builder
                .element
                .property::<gst::glib::Value>("playlist-type")
                .serialize()
                .unwrap(),
            "vod"
        );
    }

    #[test]
    fn test_hlssink3_encoder_builder_with_playlist_location() {
        init().unwrap();
        let builder = HlsSink3Builder::new("output/segment_%05d.ts", "output/playlist.m3u8")
            .with_playlist_location("output/custom_playlist.m3u8");
        assert_eq!(
            builder.element.property::<String>("playlist-location"),
            "output/custom_playlist.m3u8"
        );
    }

    #[test]
    fn test_hlssink3_encoder_builder_with_segment_location() {
        init().unwrap();
        let builder = HlsSink3Builder::new("output/segment_%05d.ts", "output/playlist.m3u8")
            .with_segment_location("output/custom_segment_%03d.ts");
        assert_eq!(
            builder.element.property::<String>("location"),
            "output/custom_segment_%03d.ts"
        );
    }

    #[test]
    fn test_hlssink3_encoder_builder_with_target_duration() {
        init().unwrap();
        let builder = HlsSink3Builder::new("output/segment_%05d.ts", "output/playlist.m3u8")
            .with_target_duration(8);
        assert_eq!(builder.element.property::<u32>("target-duration"), 8);
    }

    #[test]
    fn test_hlssink3_encoder_builder_with_playlist_length() {
        init().unwrap();
        let builder = HlsSink3Builder::new("output/segment_%05d.ts", "output/playlist.m3u8")
            .with_playlist_length(20);
        assert_eq!(builder.element.property::<u32>("playlist-length"), 20);
    }

    #[test]
    fn test_hlssink3_encoder_builder_with_max_files() {
        init().unwrap();
        let builder = HlsSink3Builder::new("output/segment_%05d.ts", "output/playlist.m3u8")
            .with_max_files(3);
        assert_eq!(builder.element.property::<u32>("max-files"), 3);
    }

    #[test]
    fn test_hlssink3_encoder_builder_with_playlist_type_valid() {
        init().unwrap();
        let builder = HlsSink3Builder::new("output/segment_%05d.ts", "output/playlist.m3u8")
            .with_playlist_type(HlsSink3PlaylistType::Event)
            .unwrap();
        assert_eq!(
            builder
                .element
                .property::<HlsSink3PlaylistType>("playlist-type"), HlsSink3PlaylistType::Event
        );
    }

    #[test]
    fn test_hlssink3_encoder_builder_with_playlist_type_invalid() {
        init().unwrap();
        let result = HlsSink3Builder::new("output/segment_%05d.ts", "output/playlist.m3u8")
            .with_playlist_type(HlsSink3PlaylistType::Unspecified);
        assert!(result.is_err());
    }

    #[test]
    fn test_hlssink3_encoder_builder_build() {
        init().unwrap();
        let result =
            HlsSink3Builder::new("output/segment_%05d.ts", "output/playlist.m3u8").build();
        assert!(result.is_ok());
    }
}
