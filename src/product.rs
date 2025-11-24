use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct Product {
    id: u32,
    name: String,
    unity: String,
    address: String,
    quantity: f32
}
 
static CURRENT_ID: AtomicU32 = AtomicU32::new(1);
 
impl Product {
    pub fn new(name: String, unity: String, address: String) -> Self {
        Product {
            id: {
                let id = CURRENT_ID.load(Ordering::Relaxed);
                CURRENT_ID.store(id + 1, Ordering::Relaxed);
                id
            },
            name,
            unity,
            address: address.to_uppercase(),
            quantity: 0.0
        }
    }
    
    pub fn info(&self) -> String {
        format!(
            "ID: {}\nNome: {}\nUnidade: {}\nEndereço: {}\nQuantidade: {} {}\n",
            self.id, self.name, self.unity, self.address, self.quantity, self.unity
        )
    }
}