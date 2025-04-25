#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, ele: String) {
        // Find the product in the store and add it to the cart
        if let Some(product) = store.products.iter().find(|(name, _)| name == &ele) {
            self.items.push((product.0.clone(), product.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.receipt.clear();
        
        if self.items.is_empty() {
            return self.receipt.clone();
        }
    
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
        let mut total_discount = 0.0;
        for chunk in prices.chunks(3) {
            if chunk.len() == 3 {
                total_discount += chunk[0];
            }
        }
    
        let total_price: f32 = prices.iter().sum();
        
        if total_discount == 0.0 {
            self.receipt = prices.iter().map(|x| (x * 100.0).round() / 100.0).collect();
            self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            return self.receipt.clone();
        }
    
        let target_total = total_price - total_discount;
        let scale = target_total / total_price;
    
        // Apply scale and round to 2 decimal places
        let mut adjusted: Vec<f32> = prices
            .iter()
            .map(|price| (price * scale * 100.0).round() / 100.0)
            .collect();
    
        // Adjust to ensure sum matches target_total
        let current_sum: f32 = adjusted.iter().sum();
        let difference = target_total - current_sum;
        let adjustment = difference / adjusted.len() as f32;
    
        // Apply adjustment and re-round
        adjusted = adjusted
            .iter()
            .map(|price| ((price + adjustment) * 100.0).round() / 100.0)
            .collect();
    
        self.receipt = adjusted;
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
        self.receipt.clone()
    }
}