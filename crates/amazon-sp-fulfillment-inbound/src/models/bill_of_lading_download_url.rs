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
pub struct BillOfLadingDownloadUrl {
    /// URL to download the bill of lading for the package. Note: The URL will only be valid for 15 seconds
    #[serde(rename = "DownloadURL", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

impl BillOfLadingDownloadUrl {
    pub fn new() -> BillOfLadingDownloadUrl {
        BillOfLadingDownloadUrl {
            download_url: None,
        }
    }
}


