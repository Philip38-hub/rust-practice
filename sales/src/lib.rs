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

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| *name == ele) {
            self.items.push(product.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        let mut receipt: Vec<f32> = vec![0.0; prices.len()];
        let mut total_discount = 0.0;
    
        let mut i = 0;
        while i + 2 < prices.len() {
            let mut indices = vec![i, i + 1, i + 2];
            indices.sort_by(|&a, &b| prices[a].partial_cmp(&prices[b]).unwrap());
            let cheapest_index = indices[0];
            total_discount += prices[cheapest_index];
            i += 3;
        }
    
        let sum: f32 = prices.iter().sum();
        for (i, &price) in prices.iter().enumerate() {
            let proportion = price / sum;
            let adjusted_price = price - proportion * total_discount;
            receipt[i] = (adjusted_price * 100.0).round() / 100.0;
        }
    
        receipt.sort_by(|a, b| a.partial_cmp(b).unwrap()); // 👈 Sort the receipt
        self.receipt = receipt.clone();
        receipt
    }
    
}
