use crate::model::{Order, OrderResponse};

pub fn mask_name(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    match chars.len() {
        0 => String::new(),
        1 => "*".to_string(),
        2 => {
            let mut s = String::new();
            s.push(chars[0]);
            s.push('*');
            s
        }
        len => {
            let mut s = String::new();
            s.push(chars[0]);
            for _ in 0..len - 2 {
                s.push('*');
            }
            s.push(chars[len - 1]);
            s
        }
    }
}

pub fn mask_phone(phone: &str) -> String {
    let chars: Vec<char> = phone.chars().collect();
    if chars.len() < 7 {
        return "*".repeat(chars.len());
    }
    let mut s = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i >= 3 && i < chars.len() - 4 {
            s.push('*');
        } else {
            s.push(*c);
        }
    }
    s
}

pub fn mask_id_card(id_card: &str) -> String {
    let chars: Vec<char> = id_card.chars().collect();
    if chars.len() < 8 {
        return "*".repeat(chars.len());
    }
    let mut s = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i >= 6 && i < chars.len() - 4 {
            s.push('*');
        } else {
            s.push(*c);
        }
    }
    s
}

pub fn mask_email(email: &str) -> String {
    if let Some(at_idx) = email.find('@') {
        let (local, domain) = email.split_at(at_idx);
        let local_chars: Vec<char> = local.chars().collect();
        let masked_local = match local_chars.len() {
            0 => String::new(),
            1 => "*".to_string(),
            2 => {
                let mut s = String::new();
                s.push(local_chars[0]);
                s.push('*');
                s
            }
            len => {
                let mut s = String::new();
                s.push(local_chars[0]);
                for _ in 0..len - 2 {
                    s.push('*');
                }
                s.push(local_chars[len - 1]);
                s
            }
        };
        format!("{}{}", masked_local, domain)
    } else {
        "*".repeat(email.len())
    }
}

pub fn mask_bank_card(card: &str) -> String {
    let chars: Vec<char> = card.chars().collect();
    if chars.len() < 8 {
        return "*".repeat(chars.len());
    }
    let mut s = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i >= 6 && i < chars.len() - 4 {
            s.push('*');
        } else {
            s.push(*c);
        }
    }
    s
}

pub fn mask_address(address: &str) -> String {
    let chars: Vec<char> = address.chars().collect();
    if chars.len() <= 6 {
        return "*".repeat(chars.len());
    }
    let keep = chars.len() / 3;
    let mut s = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i < keep {
            s.push(*c);
        } else {
            s.push('*');
        }
    }
    s
}

pub fn desensitize_order(order: &Order) -> OrderResponse {
    OrderResponse {
        id: order.id.clone(),
        order_no: order.order_no.clone(),
        amount: order.amount,
        status: order.status.clone(),
        created_at: order.created_at.clone(),
        customer_name: mask_name(&order.customer_name),
        customer_phone: mask_phone(&order.customer_phone),
        customer_id_card: mask_id_card(&order.customer_id_card),
        customer_email: mask_email(&order.customer_email),
        bank_card_no: mask_bank_card(&order.bank_card_no),
        shipping_address: mask_address(&order.shipping_address),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_name_chinese() {
        assert_eq!(mask_name("张三"), "张*");
        assert_eq!(mask_name("李小明"), "李*明");
        assert_eq!(mask_name("欧阳修文"), "欧**文");
    }

    #[test]
    fn test_mask_phone() {
        assert_eq!(mask_phone("13812345678"), "138****5678");
    }

    #[test]
    fn test_mask_id_card() {
        assert_eq!(mask_id_card("110101199001011234"), "110101********1234");
    }

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("zhangsan@example.com"), "z******n@example.com");
        assert_eq!(mask_email("ab@test.com"), "a*@test.com");
    }

    #[test]
    fn test_mask_bank_card() {
        assert_eq!(mask_bank_card("6222021234567890123"), "622202********0123");
    }

    #[test]
    fn test_mask_address() {
        let addr = "北京市朝阳区建国路88号SOHO现代城A座1001室";
        let result = mask_address(addr);
        assert!(result.starts_with("北京市"));
        assert!(result.ends_with("*"));
    }
}
