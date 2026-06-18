use csv::Writer;
use zip::write::FileOptions;
use zip::ZipWriter;

use crate::desensitize::desensitize_order;
use crate::model::{Order, OrderResponse};

const CSV_HEADERS: &[&str] = &[
    "id",
    "order_no",
    "amount",
    "status",
    "created_at",
    "customer_name",
    "customer_phone",
    "customer_id_card",
    "customer_email",
    "bank_card_no",
    "shipping_address",
];

pub fn orders_to_csv_bytes(orders: &[Order]) -> Result<Vec<u8>, csv::Error> {
    let mut wtr = Writer::from_writer(Vec::new());
    wtr.write_record(CSV_HEADERS)?;
    for order in orders {
        let d = desensitize_order(order);
        wtr.write_field(&d.id)?;
        wtr.write_field(&d.order_no)?;
        wtr.write_field(&format!("{:.2}", d.amount))?;
        wtr.write_field(&d.status)?;
        wtr.write_field(&d.created_at)?;
        wtr.write_field(&d.customer_name)?;
        wtr.write_field(&d.customer_phone)?;
        wtr.write_field(&d.customer_id_card)?;
        wtr.write_field(&d.customer_email)?;
        wtr.write_field(&d.bank_card_no)?;
        wtr.write_field(&d.shipping_address)?;
        wtr.write_record(None::<&[u8]>)?;
    }
    wtr.flush()?;
    wtr.into_inner()
}

pub fn build_zip_from_csv(csv_bytes: &[u8], filename_stem: &str) -> Result<Vec<u8>, String> {
    let buf = Vec::new();
    let mut zip = ZipWriter::new(buf);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);
    let csv_name = format!("{}.csv", filename_stem);
    zip.start_file(csv_name, options).map_err(|e| e.to_string())?;
    zip.write_all(csv_bytes).map_err(|e| e.to_string())?;
    zip.finish().map_err(|e| e.to_string())
}

pub fn build_filename_stem(start: &str, end: &str) -> String {
    let s = if start.trim().is_empty() {
        "all".to_string()
    } else {
        start.trim()[..start.trim().len().min(10)].to_string()
    };
    let e = if end.trim().is_empty() {
        "now".to_string()
    } else {
        end.trim()[..end.trim().len().min(10)].to_string()
    };
    format!("orders_{}_to_{}", s, e)
}

pub fn build_order_responses(orders: &[Order]) -> Vec<OrderResponse> {
    orders.iter().map(|o| desensitize_order(o)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_orders() -> Vec<Order> {
        vec![
            Order {
                id: "1".to_string(),
                order_no: "ORD001".to_string(),
                amount: 99.50,
                status: "已支付".to_string(),
                created_at: "2026-06-18T10:00:00Z".to_string(),
                customer_name: "张小明".to_string(),
                customer_phone: "13812345678".to_string(),
                customer_id_card: "110101199001011234".to_string(),
                customer_email: "zhang@example.com".to_string(),
                bank_card_no: "6222021234567890123".to_string(),
                shipping_address: "北京市朝阳区建国路88号".to_string(),
            },
            Order {
                id: "2".to_string(),
                order_no: "ORD002".to_string(),
                amount: 199.00,
                status: "待发货".to_string(),
                created_at: "2026-06-18T11:00:00Z".to_string(),
                customer_name: "李思琪".to_string(),
                customer_phone: "13998765432".to_string(),
                customer_id_card: "310101199505123456".to_string(),
                customer_email: "li@test.org".to_string(),
                bank_card_no: "6228480123456789012".to_string(),
                shipping_address: "上海市浦东新区陆家嘴环路1000号".to_string(),
            },
        ]
    }

    #[test]
    fn test_orders_to_csv_bytes_contains_headers() {
        let orders = sample_orders();
        let bytes = orders_to_csv_bytes(&orders).unwrap();
        let csv = String::from_utf8(bytes).unwrap();
        assert!(csv.starts_with("id,order_no,amount,"));
    }

    #[test]
    fn test_orders_to_csv_bytes_desensitized() {
        let orders = sample_orders();
        let bytes = orders_to_csv_bytes(&orders).unwrap();
        let csv = String::from_utf8(bytes).unwrap();
        assert!(csv.contains("张*明"));
        assert!(csv.contains("13*********"));
        assert!(csv.contains("11***************4"));
        assert!(!csv.contains("13812345678"));
        assert!(!csv.contains("110101199001011234"));
    }

    #[test]
    fn test_build_filename_stem() {
        assert_eq!(
            build_filename_stem("2026-06-16", "2026-06-18"),
            "orders_2026-06-16_to_2026-06-18"
        );
        assert_eq!(build_filename_stem("", ""), "orders_all_to_now");
    }

    #[test]
    fn test_build_zip_from_csv() {
        let csv = b"id,name\n1,test\n";
        let zip_bytes = build_zip_from_csv(csv, "test").unwrap();
        assert!(zip_bytes.len() > 0);
        assert_eq!(&zip_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);
    }
}
