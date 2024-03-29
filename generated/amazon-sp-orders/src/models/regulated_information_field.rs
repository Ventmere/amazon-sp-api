/*
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RegulatedInformationField : A field collected from the regulatory form.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegulatedInformationField {
    /// The unique identifier for the field.
    #[serde(default, rename = "FieldId")]
    pub field_id: String,
    /// The name for the field.
    #[serde(default, rename = "FieldLabel")]
    pub field_label: String,
    /// The type of field.
    #[serde(default, rename = "FieldType")]
    pub field_type: FieldType,
    /// The content of the field as collected in regulatory form. Note that FileAttachment type fields will contain a URL to download the attachment here.
    #[serde(default, rename = "FieldValue")]
    pub field_value: String,
}

impl RegulatedInformationField {
    /// A field collected from the regulatory form.
    pub fn new(field_id: String, field_label: String, field_type: FieldType, field_value: String) -> RegulatedInformationField {
        RegulatedInformationField {
            field_id,
            field_label,
            field_type,
            field_value,
        }
    }
}

/// The type of field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(rename = "Text")]
    Text,
    #[serde(rename = "FileAttachment")]
    FileAttachment,
}

impl Default for FieldType {
    fn default() -> FieldType {
        Self::Text
    }
}

