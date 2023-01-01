/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImageComponent : A reference to an image, hosted in the A+ Content media library.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImageComponent {
    /// This identifier is provided by the Selling Partner API for Uploads.
    #[serde(default, rename = "uploadDestinationId")]
    pub upload_destination_id: String,
    #[serde(default, rename = "imageCropSpecification")]
    pub image_crop_specification: Box<crate::models::ImageCropSpecification>,
    /// The alternative text for the image.
    #[serde(default, rename = "altText")]
    pub alt_text: String,
}

impl ImageComponent {
    /// A reference to an image, hosted in the A+ Content media library.
    pub fn new(upload_destination_id: String, image_crop_specification: crate::models::ImageCropSpecification, alt_text: String) -> ImageComponent {
        ImageComponent {
            upload_destination_id,
            image_crop_specification: Box::new(image_crop_specification),
            alt_text,
        }
    }
}


