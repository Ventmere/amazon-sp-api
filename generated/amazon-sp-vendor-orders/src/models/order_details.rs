/*
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderDetails : Details of an order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderDetails {
    /// The date the purchase order was placed. Must be in ISO-8601 date/time format.
    #[serde(default, rename = "purchaseOrderDate")]
    pub purchase_order_date: String,
    /// The date when purchase order was last changed by Amazon after the order was placed. This date will be greater than 'purchaseOrderDate'. This means the PO data was changed on that date and vendors are required to fulfill the  updated PO. The PO changes can be related to Item Quantity, Ship to Location, Ship Window etc. This field will not be present in orders that have not changed after creation. Must be in ISO-8601 date/time format.
    #[serde(default, rename = "purchaseOrderChangedDate", skip_serializing_if = "Option::is_none")]
    pub purchase_order_changed_date: Option<String>,
    /// The date when current purchase order state was changed. Current purchase order state is available in the field 'purchaseOrderState'. Must be in ISO-8601 date/time format.
    #[serde(default, rename = "purchaseOrderStateChangedDate")]
    pub purchase_order_state_changed_date: String,
    /// Type of purchase order.
    #[serde(default, rename = "purchaseOrderType", skip_serializing_if = "Option::is_none")]
    pub purchase_order_type: Option<PurchaseOrderType>,
    #[serde(default, rename = "importDetails", skip_serializing_if = "Option::is_none")]
    pub import_details: Option<Box<crate::models::ImportDetails>>,
    /// If requested by the recipient, this field will contain a promotional/deal number. The discount code line is optional. It is used to obtain a price discount on items on the order.
    #[serde(default, rename = "dealCode", skip_serializing_if = "Option::is_none")]
    pub deal_code: Option<String>,
    /// Payment method used.
    #[serde(default, rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    #[serde(default, rename = "buyingParty", skip_serializing_if = "Option::is_none")]
    pub buying_party: Option<Box<crate::models::PartyIdentification>>,
    #[serde(default, rename = "sellingParty", skip_serializing_if = "Option::is_none")]
    pub selling_party: Option<Box<crate::models::PartyIdentification>>,
    #[serde(default, rename = "shipToParty", skip_serializing_if = "Option::is_none")]
    pub ship_to_party: Option<Box<crate::models::PartyIdentification>>,
    #[serde(default, rename = "billToParty", skip_serializing_if = "Option::is_none")]
    pub bill_to_party: Option<Box<crate::models::PartyIdentification>>,
    /// Defines a date time interval according to ISO8601. Interval is separated by double hyphen (--).
    #[serde(default, rename = "shipWindow", skip_serializing_if = "Option::is_none")]
    pub ship_window: Option<String>,
    /// Defines a date time interval according to ISO8601. Interval is separated by double hyphen (--).
    #[serde(default, rename = "deliveryWindow", skip_serializing_if = "Option::is_none")]
    pub delivery_window: Option<String>,
    /// A list of items in this purchase order.
    #[serde(default, rename = "items")]
    pub items: Vec<crate::models::OrderItem>,
}

impl OrderDetails {
    /// Details of an order.
    pub fn new(purchase_order_date: String, purchase_order_state_changed_date: String, items: Vec<crate::models::OrderItem>) -> OrderDetails {
        OrderDetails {
            purchase_order_date,
            purchase_order_changed_date: None,
            purchase_order_state_changed_date,
            purchase_order_type: None,
            import_details: None,
            deal_code: None,
            payment_method: None,
            buying_party: None,
            selling_party: None,
            ship_to_party: None,
            bill_to_party: None,
            ship_window: None,
            delivery_window: None,
            items,
        }
    }
}

/// Type of purchase order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PurchaseOrderType {
    #[serde(rename = "RegularOrder")]
    RegularOrder,
    #[serde(rename = "ConsignedOrder")]
    ConsignedOrder,
    #[serde(rename = "NewProductIntroduction")]
    NewProductIntroduction,
    #[serde(rename = "RushOrder")]
    RushOrder,
}

impl Default for PurchaseOrderType {
    fn default() -> PurchaseOrderType {
        Self::RegularOrder
    }
}
/// Payment method used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentMethod {
    #[serde(rename = "Invoice")]
    Invoice,
    #[serde(rename = "Consignment")]
    Consignment,
    #[serde(rename = "CreditCard")]
    CreditCard,
    #[serde(rename = "Prepaid")]
    Prepaid,
}

impl Default for PaymentMethod {
    fn default() -> PaymentMethod {
        Self::Invoice
    }
}

