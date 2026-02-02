//! Billing model types

use serde::{Deserialize, Serialize};

use crate::Meta;

/// Billing history entry
#[derive(Serialize, Deserialize)]
pub struct BillingHistory {
    /// History entry ID
    pub id: Option<i64>,
    /// Date of the entry
    pub date: Option<String>,
    /// Type of entry (charges, payment, credit, etc.)
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Amount
    pub amount: Option<f64>,
    /// Balance after this entry
    pub balance: Option<f64>,
}

/// Response wrapper for billing history list
#[derive(Serialize, Deserialize)]
pub struct BillingHistoryResponse {
    pub billing_history: Vec<BillingHistory>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

/// Invoice information
#[derive(Serialize, Deserialize)]
pub struct Invoice {
    /// Invoice ID
    pub id: Option<i64>,
    /// Invoice date
    pub date: Option<String>,
    /// Invoice description
    pub description: Option<String>,
    /// Invoice amount
    pub amount: Option<f64>,
    /// Invoice balance
    pub balance: Option<f64>,
}

/// Response wrapper for invoice list
#[derive(Serialize, Deserialize)]
pub struct InvoicesResponse {
    pub billing_invoices: Vec<Invoice>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

/// Response wrapper for single invoice
#[derive(Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub billing_invoice: Invoice,
}

/// Invoice item (line item on an invoice)
#[derive(Serialize, Deserialize)]
pub struct InvoiceItem {
    /// Item description
    pub description: Option<String>,
    /// Product name
    pub product: Option<String>,
    /// Start date
    pub start_date: Option<String>,
    /// End date
    pub end_date: Option<String>,
    /// Units
    pub units: Option<i64>,
    /// Unit type
    pub unit_type: Option<String>,
    /// Unit price
    pub unit_price: Option<f64>,
    /// Total amount
    pub total: Option<f64>,
}

/// Response wrapper for invoice items
#[derive(Serialize, Deserialize)]
pub struct InvoiceItemsResponse {
    pub invoice_items: Vec<InvoiceItem>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

/// Pending charge
#[derive(Serialize, Deserialize)]
pub struct PendingCharge {
    /// Description
    pub description: Option<String>,
    /// Date of the charge
    pub date: Option<String>,
    /// Amount
    pub amount: Option<f64>,
}

/// Response wrapper for pending charges
#[derive(Serialize, Deserialize)]
pub struct PendingChargesResponse {
    pub pending_charges: Vec<PendingCharge>,
    #[serde(default)]
    pub meta: Option<Meta>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_history_deserialize() {
        let json = r#"{"id":123,"date":"2024-01-01","type":"charges","description":"Monthly charges","amount":10.50,"balance":89.50}"#;
        let history: BillingHistory = serde_json::from_str(json).unwrap();
        assert_eq!(history.id.unwrap(), 123);
        assert_eq!(history.entry_type.unwrap(), "charges");
        assert_eq!(history.amount.unwrap(), 10.50);
    }

    #[test]
    fn test_invoice_deserialize() {
        let json = r#"{"id":456,"date":"2024-01-01","description":"January Invoice","amount":100.00,"balance":0.00}"#;
        let invoice: Invoice = serde_json::from_str(json).unwrap();
        assert_eq!(invoice.id.unwrap(), 456);
        assert_eq!(invoice.description.unwrap(), "January Invoice");
    }

    #[test]
    fn test_invoice_item_deserialize() {
        let json = r#"{"description":"Cloud Compute","product":"vc2-1c-1gb","start_date":"2024-01-01","end_date":"2024-01-31","units":744,"unit_type":"hours","unit_price":0.007,"total":5.21}"#;
        let item: InvoiceItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.product.unwrap(), "vc2-1c-1gb");
        assert_eq!(item.total.unwrap(), 5.21);
    }

    #[test]
    fn test_pending_charge_deserialize() {
        let json = r#"{"description":"Instance charges","date":"2024-01-15","amount":2.50}"#;
        let charge: PendingCharge = serde_json::from_str(json).unwrap();
        assert_eq!(charge.amount.unwrap(), 2.50);
    }
}
