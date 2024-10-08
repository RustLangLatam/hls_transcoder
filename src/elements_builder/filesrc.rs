//! # `FileSrcBuilder` Module
//!
//! This module provides a builder for creating and configuring the `filesrc` GStreamer element,
//! which is used to read data from a file in the filesystem. The `filesrc` element can be configured
//! with different properties such as location, block size, and `do-timestamp` to suit various file reading needs.
//!
//! ## Properties Explained
//!
//! Below is a summary of the properties that can be set using `FileSrcBuilder`:
//!
//! 1. **`location`**: Sets the location of the file to be read by `filesrc`.
//!    - **Description**: The path to the file that will be read by the element.
//!    - **Usage**: This property must be set before using the element. Otherwise, the pipeline will not work.
//!    - **Documentation Reference**: [FileSrc Location](https://gstreamer.freedesktop.org/documentation/coreelements/filesrc.html?gi-language=c#GstFileSrc:location)
//!
//! 2. **`blocksize`**: Configures the block size in bytes for reading data.
//!    - **Description**: Sets the size of blocks that `filesrc` will read from the file. If the value is 0, a default block size will be used.
//!    - **Default Value**: 4096 bytes.
//!    - **Documentation Reference**: [FileSrc Blocksize](https://gstreamer.freedesktop.org/documentation/coreelements/filesrc.html?gi-language=c#GstFileSrc:blocksize)
//!
//! 3. **`do-timestamp`**: Configures whether timestamps should be applied to buffers.
//!    - **Description**: When set to `true`, buffers are timestamped based on the clock, which is useful for live playback.
//!    - **Default Value**: `false` (timestamps are not applied).
//!    - **Documentation Reference**: [FileSrc Do-Timestamp](https://gstreamer.freedesktop.org/documentation/coreelements/filesrc.html?gi-language=c#GstFileSrc:do-timestamp)
//!
//! 4. **`num-buffers`**: Sets the number of buffers to output before sending EOS (End of Stream).
//!    - **Description**: This property is useful for limiting the number of buffers sent, for testing or debugging purposes.
//!    - **Default Value**: `-1` (infinite buffers).
//!
//! For more information and advanced usage scenarios, refer to the [official GStreamer FileSrc Documentation](https://gstreamer.freedesktop.org/documentation/coreelements/filesrc.html?gi-language=c).

use anyhow::Result;
use gst::prelude::*;

/// A builder for creating and configuring the `filesrc` GStreamer element.
#[derive(Debug, Clone)]
pub struct FileSrcBuilder {
    element: gst::Element,
}

impl FileSrcBuilder {
    /// Creates a new `FileSrcBuilder` instance with a required `location` parameter.
    ///
    /// The `location` parameter is the path to the file that the `filesrc` element will read from.
    ///
    /// # Arguments
    ///
    /// * `location`: The path to the file as a string.
    ///
    /// # Returns
    ///
    /// A new instance of `FileSrcBuilder` with the location set.
    pub fn new(location: &str) -> Self {
        let element = gst::ElementFactory::make_with_name("filesrc", Some("filesrc"))
            .expect("Failed to create filesrc element");
        element.set_property("blocksize", 65536u32);  // Use a larger block size for faster reading

        // Set the required property `location` during initialization.
        element.set_property("location", location);

        FileSrcBuilder { element }
    }

    /// Sets the `blocksize` property of the `filesrc` element.
    ///
    /// # Arguments
    ///
    /// * `size`: The size of the blocks in bytes to be read from the file.
    ///
    /// # Returns
    ///
    /// Returns the updated builder instance.
    pub fn with_blocksize(mut self, size: u32) -> Self {
        self.element.set_property("blocksize", size);
        self
    }

    /// Enables or disables timestamping for the buffers read from the file.
    ///
    /// # Arguments
    ///
    /// * `enabled`: A boolean indicating whether to enable timestamping.
    ///
    /// # Returns
    ///
    /// Returns the updated builder instance.
    pub fn with_timestamp(mut self, enabled: bool) -> Self {
        self.element.set_property("do-timestamp", enabled);
        self
    }

    /// Sets the number of buffers to be output before sending EOS (End of Stream).
    ///
    /// # Arguments
    ///
    /// * `num_buffers`: The number of buffers to output, or `-1` for unlimited buffers.
    ///
    /// # Returns
    ///
    /// Returns the updated builder instance.
    pub fn with_num_buffers(mut self, num_buffers: i32) -> Self {
        self.element.set_property("num-buffers", num_buffers);
        self
    }

    /// Builds and returns the configured `filesrc` instance.
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
    fn test_builder_with_required_parameter() {
        init().unwrap();

        // Test creating FileSrcBuilder with a mandatory location.
        let filesrc = FileSrcBuilder::new("/path/to/video.mp4")
            .with_blocksize(8192)
            .with_timestamp(true)
            .with_num_buffers(100)
            .build()
            .unwrap();

        // Check if the location is correctly set.
        assert_eq!(
            filesrc.property::<String>("location"),
            "/path/to/video.mp4"
        );

        // Verify the optional properties are correctly set.
        assert_eq!(filesrc.property::<u32>("blocksize"), 8192);
        assert_eq!(filesrc.property::<bool>("do-timestamp"), true);
        assert_eq!(filesrc.property::<i32>("num-buffers"), 100);
    }

    #[test]
    fn test_builder_default_properties() {
        init().unwrap();

        // Test builder with only the required parameter set.
        let filesrc = FileSrcBuilder::new("/path/to/video.mp4").build().unwrap();

        // Validate default property values.
        assert_eq!(filesrc.property::<String>("location"), "/path/to/video.mp4");
        assert_eq!(filesrc.property::<u32>("blocksize"), 4096); // Default block size is 4096 bytes.
        assert_eq!(filesrc.property::<bool>("do-timestamp"), false); // Timestamping is disabled by default.
        assert_eq!(filesrc.property::<i32>("num-buffers"), -1); // Default is unlimited buffers.
    }
}


