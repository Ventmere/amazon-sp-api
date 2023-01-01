/*
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * The version of the OpenAPI document: 2020-11-01
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetContentDocumentResponseAllOf {
    #[serde(default, rename = "contentRecord")]
    pub content_record: Box<crate::models::ContentRecord>,
}

impl GetContentDocumentResponseAllOf {
    pub fn new(content_record: crate::models::ContentRecord) -> GetContentDocumentResponseAllOf {
        GetContentDocumentResponseAllOf {
            content_record: Box::new(content_record),
        }
    }
}


