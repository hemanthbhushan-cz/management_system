// Scenario: Inventory Management System

// You are tasked with building a simple inventory management system for a small store. The store sells various products, and you need to create a Rust program to manage these products. The program should support the following operations:

// 1. **Add Product:**
//    - Allow the user to add a new product to the inventory.
//    - Each product should have a name, price, and quantity in stock.

// 2. **List Products:**
//    - Display a list of all products in the inventory, including their names, prices, and quantities.

// 3. **Update Stock:**
//    - Enable the user to update the stock quantity of an existing product.

// 4. **Search Product:**
//    - Allow the user to search for a product by name and display its details.

// 5. **Remove Product:**
//    - Provide an option to remove a product from the inventory.

// 6. **Total Stock Value:**
//    - Calculate and display the total value of the entire inventory (sum of the individual product values).

// Your program should use structs and enums to represent products and operations. Implement error handling for scenarios such as trying to update the stock of a non-existent product or adding a product with invalid details.

// Use the `match` statement for handling different user inputs and operations.
#[derive(Debug)]
struct ProductDetails {
    name: String,
    price: u32,
    quantity_in_stock: u32,
}

impl ProductDetails {
    fn new(name: String, price: u32, quantity_in_stock: u32) -> Self {
        Self {
            name,
            price,
            quantity_in_stock,
        }
    }
}
#[derive(Debug)]

struct Inventory {
    products: Vec<ProductDetails>,
}

impl Inventory {
    fn add_product(&mut self, product: ProductDetails) {
        self.products.push(product);
    }

    fn display_products(&self) {
        println!("The product list: {:?}", self.products);
    }

    fn update_products(&mut self, details: ProductDetails) {
        for item in &mut self.products {
            if details.name == item.name {
                item.price = details.price;
            }
        }
    }

    fn search_products(&mut self, name: &str) {
        for item in &mut self.products {
            if name == item.name {
                println!("name searched is : {:?}", item);
            }
        }
    }

    fn remove_product(&mut self, name: &str) {
        if let Some(index) = self
            .products
            .iter()
            .position(|product| product.name == name)
        {
            self.products.remove(index);
            println!("Product {} removed from inventory.", name);
        } else {
            println!("Product {} not found in inventory.", name);
        }
    }

    fn total_cost_products(&self) -> u32{
        let mut value: u32 = 0;
        for item in &self.products {
            value += item.price;
        }

        value
    }
}

fn main() {
    let mut inventory = Inventory {
        products: Vec::new(),
    };

    let product1 = ProductDetails::new("Rust".to_owned(), 2400, 100);
    let product2 = ProductDetails::new("C++".to_owned(), 500, 200);
    let product3 = ProductDetails::new("C".to_owned(), 1000, 300);

    inventory.add_product(product1);
    inventory.add_product(product2);
    inventory.add_product(product3);

    inventory.display_products();

    let temp = ProductDetails {
        name: "Rust".to_owned(),
        price: 34444444,
        quantity_in_stock: 777777,
    };

    inventory.update_products(temp);

    inventory.display_products();

    inventory.search_products("Rust");
    inventory.remove_product("Rust");

    inventory.display_products();


    println!("the ttoal value {}",inventory.total_cost_products());
}
