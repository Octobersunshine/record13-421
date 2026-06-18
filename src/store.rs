use std::collections::HashMap;
use std::sync::RwLock;

use crate::model::Order;

pub struct OrderStore {
    inner: RwLock<HashMap<String, Order>>,
}

impl OrderStore {
    pub fn new() -> Self {
        let mut map: HashMap<String, Order> = HashMap::new();

        let order1 = Order {
            id: "1".to_string(),
            order_no: "ORD2026061800001".to_string(),
            amount: 1299.50,
            status: "已支付".to_string(),
            created_at: "2026-06-18T10:30:00Z".to_string(),
            customer_name: "张小明".to_string(),
            customer_phone: "13812345678".to_string(),
            customer_id_card: "110101199001011234".to_string(),
            customer_email: "zhangxiaoming@example.com".to_string(),
            bank_card_no: "6222021234567890123".to_string(),
            shipping_address: "北京市朝阳区建国路88号SOHO现代城A座1001室".to_string(),
        };

        let order2 = Order {
            id: "2".to_string(),
            order_no: "ORD2026061800002".to_string(),
            amount: 599.00,
            status: "待发货".to_string(),
            created_at: "2026-06-18T11:05:00Z".to_string(),
            customer_name: "李思琪".to_string(),
            customer_phone: "13998765432".to_string(),
            customer_id_card: "310101199505123456".to_string(),
            customer_email: "lisiqi@test.org".to_string(),
            bank_card_no: "6228480123456789012".to_string(),
            shipping_address: "上海市浦东新区陆家嘴环路1000号恒生银行大厦25楼".to_string(),
        };

        let order3 = Order {
            id: "3".to_string(),
            order_no: "ORD2026061700003".to_string(),
            amount: 3280.00,
            status: "已完成".to_string(),
            created_at: "2026-06-17T09:15:00Z".to_string(),
            customer_name: "王大伟".to_string(),
            customer_phone: "13600001111".to_string(),
            customer_id_card: "440101198805157788".to_string(),
            customer_email: "wangdawei@mail.cn".to_string(),
            bank_card_no: "6225880123456789".to_string(),
            shipping_address: "广东省广州市天河区珠江新城花城大道85号".to_string(),
        };

        let order4 = Order {
            id: "4".to_string(),
            order_no: "ORD2026061600004".to_string(),
            amount: 188.80,
            status: "已取消".to_string(),
            created_at: "2026-06-16T15:42:00Z".to_string(),
            customer_name: "赵丽娜".to_string(),
            customer_phone: "13722223333".to_string(),
            customer_id_card: "510101199212126677".to_string(),
            customer_email: "zhaolina@example.cn".to_string(),
            bank_card_no: "6217009876543210".to_string(),
            shipping_address: "四川省成都市锦江区春熙路东段1号IFS国际金融中心".to_string(),
        };

        map.insert(order1.id.clone(), order1);
        map.insert(order2.id.clone(), order2);
        map.insert(order3.id.clone(), order3);
        map.insert(order4.id.clone(), order4);

        OrderStore {
            inner: RwLock::new(map),
        }
    }

    pub fn get_by_id(&self, id: &str) -> Option<Order> {
        let guard = self.inner.read().ok()?;
        guard.get(id).cloned()
    }

    pub fn list_by_date_range(&self, start: &str, end: &str) -> Vec<Order> {
        let guard = match self.inner.read() {
            Ok(g) => g,
            Err(_) => return Vec::new(),
        };
        let start_norm = normalize_date(start);
        let end_norm = normalize_date(end);
        let mut results: Vec<Order> = guard
            .values()
            .filter(|o| {
                let order_date = extract_date(&o.created_at);
                match (&start_norm, &end_norm) {
                    (Some(s), Some(e)) => order_date.as_deref() >= Some(s.as_str()) && order_date.as_deref() <= Some(e.as_str()),
                    (Some(s), None) => order_date.as_deref() >= Some(s.as_str()),
                    (None, Some(e)) => order_date.as_deref() <= Some(e.as_str()),
                    (None, None) => true,
                }
            })
            .cloned()
            .collect();
        results.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        results
    }
}

fn normalize_date(s: &str) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.len() >= 10 {
        Some(trimmed[..10].to_string())
    } else {
        Some(trimmed.to_string())
    }
}

fn extract_date(created_at: &str) -> Option<String> {
    if created_at.len() >= 10 {
        Some(created_at[..10].to_string())
    } else {
        None
    }
}

impl Default for OrderStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_date() {
        assert_eq!(extract_date("2026-06-18T10:30:00Z"), Some("2026-06-18".to_string()));
    }

    #[test]
    fn test_normalize_date() {
        assert_eq!(normalize_date(""), None);
        assert_eq!(normalize_date("2026-06-18"), Some("2026-06-18".to_string()));
        assert_eq!(normalize_date("2026-06-18T10:30:00Z"), Some("2026-06-18".to_string()));
    }

    #[test]
    fn test_list_by_date_range() {
        let store = OrderStore::new();
        let all = store.list_by_date_range("2026-06-16", "2026-06-18");
        assert_eq!(all.len(), 4);

        let june18 = store.list_by_date_range("2026-06-18", "2026-06-18");
        assert_eq!(june18.len(), 2);

        let june16 = store.list_by_date_range("2026-06-16", "2026-06-16");
        assert_eq!(june16.len(), 1);
        assert_eq!(june16[0].id, "4");
    }
}
