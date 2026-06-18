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

        map.insert(order1.id.clone(), order1);
        map.insert(order2.id.clone(), order2);

        OrderStore {
            inner: RwLock::new(map),
        }
    }

    pub fn get_by_id(&self, id: &str) -> Option<Order> {
        let guard = self.inner.read().ok()?;
        guard.get(id).cloned()
    }
}

impl Default for OrderStore {
    fn default() -> Self {
        Self::new()
    }
}
