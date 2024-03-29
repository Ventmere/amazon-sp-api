/*
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImportDetails : Import details for an import order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImportDetails {
    /// If the recipient requests, contains the shipment method of payment. This is for import PO's only.
    #[serde(default, rename = "methodOfPayment", skip_serializing_if = "Option::is_none")]
    pub method_of_payment: Option<MethodOfPayment>,
    /// Incoterms (International Commercial Terms) are used to divide transaction costs and responsibilities between buyer and seller and reflect state-of-the-art transportation practices. This is for import purchase orders only. 
    #[serde(default, rename = "internationalCommercialTerms", skip_serializing_if = "Option::is_none")]
    pub international_commercial_terms: Option<InternationalCommercialTerms>,
    /// The port where goods on an import purchase order must be delivered by the vendor. This should only be specified when the internationalCommercialTerms is FOB.
    #[serde(default, rename = "portOfDelivery", skip_serializing_if = "Option::is_none")]
    pub port_of_delivery: Option<String>,
    /// Types and numbers of container(s) for import purchase orders. Can be a comma-separated list if the shipment has multiple containers. HC signifies a high-capacity container. Free-text field, limited to 64 characters. The format will be a comma-delimited list containing values of the type: $NUMBER_OF_CONTAINERS_OF_THIS_TYPE-$CONTAINER_TYPE. The list of values for the container type is: 40'(40-foot container), 40'HC (40-foot high-capacity container), 45', 45'HC, 30', 30'HC, 20', 20'HC.
    #[serde(default, rename = "importContainers", skip_serializing_if = "Option::is_none")]
    pub import_containers: Option<String>,
    /// Special instructions regarding the shipment. This field is for import purchase orders.
    #[serde(default, rename = "shippingInstructions", skip_serializing_if = "Option::is_none")]
    pub shipping_instructions: Option<String>,
}

impl ImportDetails {
    /// Import details for an import order.
    pub fn new() -> ImportDetails {
        ImportDetails {
            method_of_payment: None,
            international_commercial_terms: None,
            port_of_delivery: None,
            import_containers: None,
            shipping_instructions: None,
        }
    }
}

/// If the recipient requests, contains the shipment method of payment. This is for import PO's only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MethodOfPayment {
    #[serde(rename = "PaidByBuyer")]
    PaidByBuyer,
    #[serde(rename = "CollectOnDelivery")]
    CollectOnDelivery,
    #[serde(rename = "DefinedByBuyerAndSeller")]
    DefinedByBuyerAndSeller,
    #[serde(rename = "FOBPortOfCall")]
    FOBPortOfCall,
    #[serde(rename = "PrepaidBySeller")]
    PrepaidBySeller,
    #[serde(rename = "PaidBySeller")]
    PaidBySeller,
}

impl Default for MethodOfPayment {
    fn default() -> MethodOfPayment {
        Self::PaidByBuyer
    }
}
/// Incoterms (International Commercial Terms) are used to divide transaction costs and responsibilities between buyer and seller and reflect state-of-the-art transportation practices. This is for import purchase orders only. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InternationalCommercialTerms {
    #[serde(rename = "ExWorks")]
    ExWorks,
    #[serde(rename = "FreeCarrier")]
    FreeCarrier,
    #[serde(rename = "FreeOnBoard")]
    FreeOnBoard,
    #[serde(rename = "FreeAlongSideShip")]
    FreeAlongSideShip,
    #[serde(rename = "CarriagePaidTo")]
    CarriagePaidTo,
    #[serde(rename = "CostAndFreight")]
    CostAndFreight,
    #[serde(rename = "CarriageAndInsurancePaidTo")]
    CarriageAndInsurancePaidTo,
    #[serde(rename = "CostInsuranceAndFreight")]
    CostInsuranceAndFreight,
    #[serde(rename = "DeliveredAtTerminal")]
    DeliveredAtTerminal,
    #[serde(rename = "DeliveredAtPlace")]
    DeliveredAtPlace,
    #[serde(rename = "DeliverDutyPaid")]
    DeliverDutyPaid,
}

impl Default for InternationalCommercialTerms {
    fn default() -> InternationalCommercialTerms {
        Self::ExWorks
    }
}

