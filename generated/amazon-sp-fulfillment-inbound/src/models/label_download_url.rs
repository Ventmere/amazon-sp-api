/*
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LabelDownloadUrl {
    /// URL to download the label for the package. Note: The URL will only be valid for 15 seconds
    #[serde(default, rename = "DownloadURL", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

impl LabelDownloadUrl {
    pub fn new() -> LabelDownloadUrl {
        LabelDownloadUrl {
            download_url: None,
        }
    }
}


