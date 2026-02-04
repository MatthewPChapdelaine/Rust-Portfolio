/*!
 * Design Patterns in Rust
 * 
 * Demonstrates 5+ design patterns with idiomatic Rust implementations:
 * - Singleton (thread-safe)
 * - Factory (with trait objects)
 * - Observer (publish-subscribe)
 * - Strategy (runtime polymorphism)
 * - Decorator (wrapper pattern)
 * - Builder (fluent API)
 * 
 * # Compile and Run
 * ```bash
 * rustc design_patterns.rs -o design_patterns
 * ./design_patterns
 * ```
 */

use std::sync::{Arc, Mutex, Once};
use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// 1. SINGLETON PATTERN
// ============================================================================

/// Thread-safe Singleton using Once
static mut LOGGER_INSTANCE: Option<Logger> = None;
static LOGGER_INIT: Once = Once::new();

#[derive(Clone)]
struct Logger {
    log_count: Arc<Mutex<usize>>,
}

impl Logger {
    /// Get the singleton instance (thread-safe)
    fn instance() -> &'static Logger {
        unsafe {
            LOGGER_INIT.call_once(|| {
                LOGGER_INSTANCE = Some(Logger {
                    log_count: Arc::new(Mutex::new(0)),
                });
            });
            LOGGER_INSTANCE.as_ref().unwrap()
        }
    }

    fn log(&self, message: &str) {
        let mut count = self.log_count.lock().unwrap();
        *count += 1;
        println!("[LOG #{}] {}", count, message);
    }

    fn get_count(&self) -> usize {
        *self.log_count.lock().unwrap()
    }
}

// ============================================================================
// 2. FACTORY PATTERN
// ============================================================================

/// Product trait
trait Vehicle {
    fn drive(&self) -> String;
    fn capacity(&self) -> u32;
}

struct Car {
    model: String,
}

impl Vehicle for Car {
    fn drive(&self) -> String {
        format!("Driving a {} car on the road", self.model)
    }

    fn capacity(&self) -> u32 {
        5
    }
}

struct Truck {
    model: String,
}

impl Vehicle for Truck {
    fn drive(&self) -> String {
        format!("Driving a {} truck hauling cargo", self.model)
    }

    fn capacity(&self) -> u32 {
        2
    }
}

struct Motorcycle {
    model: String,
}

impl Vehicle for Motorcycle {
    fn drive(&self) -> String {
        format!("Riding a {} motorcycle", self.model)
    }

    fn capacity(&self) -> u32 {
        2
    }
}

/// Factory
enum VehicleType {
    Car,
    Truck,
    Motorcycle,
}

struct VehicleFactory;

impl VehicleFactory {
    fn create_vehicle(vehicle_type: VehicleType, model: String) -> Box<dyn Vehicle> {
        match vehicle_type {
            VehicleType::Car => Box::new(Car { model }),
            VehicleType::Truck => Box::new(Truck { model }),
            VehicleType::Motorcycle => Box::new(Motorcycle { model }),
        }
    }
}

// ============================================================================
// 3. OBSERVER PATTERN
// ============================================================================

/// Observer trait
trait Observer {
    fn update(&self, message: &str);
}

struct EmailSubscriber {
    email: String,
}

impl Observer for EmailSubscriber {
    fn update(&self, message: &str) {
        println!("Email to {}: {}", self.email, message);
    }
}

struct SMSSubscriber {
    phone: String,
}

impl Observer for SMSSubscriber {
    fn update(&self, message: &str) {
        println!("SMS to {}: {}", self.phone, message);
    }
}

/// Subject (Observable)
struct NewsPublisher {
    observers: RefCell<Vec<Rc<dyn Observer>>>,
}

impl NewsPublisher {
    fn new() -> Self {
        NewsPublisher {
            observers: RefCell::new(Vec::new()),
        }
    }

    fn subscribe(&self, observer: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    fn publish(&self, news: &str) {
        println!("\n📰 Publishing: {}", news);
        for observer in self.observers.borrow().iter() {
            observer.update(news);
        }
    }
}

// ============================================================================
// 4. STRATEGY PATTERN
// ============================================================================

/// Strategy trait
trait PaymentStrategy {
    fn pay(&self, amount: f64) -> String;
}

struct CreditCard {
    card_number: String,
}

impl PaymentStrategy for CreditCard {
    fn pay(&self, amount: f64) -> String {
        format!("Paid ${:.2} with credit card ending in {}", 
                amount, 
                &self.card_number[self.card_number.len()-4..])
    }
}

struct PayPal {
    email: String,
}

impl PaymentStrategy for PayPal {
    fn pay(&self, amount: f64) -> String {
        format!("Paid ${:.2} via PayPal account {}", amount, self.email)
    }
}

struct Bitcoin {
    wallet_address: String,
}

impl PaymentStrategy for Bitcoin {
    fn pay(&self, amount: f64) -> String {
        format!("Paid ${:.2} with Bitcoin to wallet {}...", 
                amount, 
                &self.wallet_address[..8])
    }
}

/// Context
struct ShoppingCart {
    items: Vec<(String, f64)>,
}

impl ShoppingCart {
    fn new() -> Self {
        ShoppingCart { items: Vec::new() }
    }

    fn add_item(&mut self, name: String, price: f64) {
        self.items.push((name, price));
    }

    fn total(&self) -> f64 {
        self.items.iter().map(|(_, price)| price).sum()
    }

    fn checkout(&self, strategy: &dyn PaymentStrategy) -> String {
        let total = self.total();
        strategy.pay(total)
    }
}

// ============================================================================
// 5. DECORATOR PATTERN
// ============================================================================

/// Component trait
trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

/// Concrete component
struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        2.0
    }

    fn description(&self) -> String {
        "Simple Coffee".to_string()
    }
}

/// Decorator
struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5
    }

    fn description(&self) -> String {
        format!("{}, Milk", self.coffee.description())
    }
}

struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for SugarDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.2
    }

    fn description(&self) -> String {
        format!("{}, Sugar", self.coffee.description())
    }
}

struct WhipDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for WhipDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.7
    }

    fn description(&self) -> String {
        format!("{}, Whipped Cream", self.coffee.description())
    }
}

// ============================================================================
// 6. BUILDER PATTERN
// ============================================================================

#[derive(Debug, Clone)]
struct Computer {
    cpu: String,
    ram: u32,
    storage: u32,
    gpu: Option<String>,
    wifi: bool,
}

struct ComputerBuilder {
    cpu: String,
    ram: u32,
    storage: u32,
    gpu: Option<String>,
    wifi: bool,
}

impl ComputerBuilder {
    fn new() -> Self {
        ComputerBuilder {
            cpu: "Intel i5".to_string(),
            ram: 8,
            storage: 256,
            gpu: None,
            wifi: true,
        }
    }

    fn cpu(mut self, cpu: &str) -> Self {
        self.cpu = cpu.to_string();
        self
    }

    fn ram(mut self, ram: u32) -> Self {
        self.ram = ram;
        self
    }

    fn storage(mut self, storage: u32) -> Self {
        self.storage = storage;
        self
    }

    fn gpu(mut self, gpu: &str) -> Self {
        self.gpu = Some(gpu.to_string());
        self
    }

    fn wifi(mut self, wifi: bool) -> Self {
        self.wifi = wifi;
        self
    }

    fn build(self) -> Computer {
        Computer {
            cpu: self.cpu,
            ram: self.ram,
            storage: self.storage,
            gpu: self.gpu,
            wifi: self.wifi,
        }
    }
}

// ============================================================================
// DEMONSTRATIONS
// ============================================================================

fn demo_singleton() {
    println!("\n{:=^60}", " SINGLETON PATTERN ");
    
    let logger1 = Logger::instance();
    logger1.log("First log message");
    
    let logger2 = Logger::instance();
    logger2.log("Second log message");
    
    println!("Total logs: {}", logger1.get_count());
    println!("Same instance? {}", logger1.get_count() == logger2.get_count());
}

fn demo_factory() {
    println!("\n{:=^60}", " FACTORY PATTERN ");
    
    let vehicles: Vec<Box<dyn Vehicle>> = vec![
        VehicleFactory::create_vehicle(VehicleType::Car, "Tesla Model 3".to_string()),
        VehicleFactory::create_vehicle(VehicleType::Truck, "Ford F-150".to_string()),
        VehicleFactory::create_vehicle(VehicleType::Motorcycle, "Harley Davidson".to_string()),
    ];
    
    for vehicle in vehicles {
        println!("{} (capacity: {})", vehicle.drive(), vehicle.capacity());
    }
}

fn demo_observer() {
    println!("\n{:=^60}", " OBSERVER PATTERN ");
    
    let publisher = NewsPublisher::new();
    
    let email_sub1: Rc<dyn Observer> = Rc::new(EmailSubscriber {
        email: "alice@example.com".to_string(),
    });
    let email_sub2: Rc<dyn Observer> = Rc::new(EmailSubscriber {
        email: "bob@example.com".to_string(),
    });
    let sms_sub: Rc<dyn Observer> = Rc::new(SMSSubscriber {
        phone: "+1-555-0123".to_string(),
    });
    
    publisher.subscribe(email_sub1);
    publisher.subscribe(email_sub2);
    publisher.subscribe(sms_sub);
    
    publisher.publish("Breaking: Rust 2.0 Released!");
    publisher.publish("Weather Alert: Sunny skies ahead");
}

fn demo_strategy() {
    println!("\n{:=^60}", " STRATEGY PATTERN ");
    
    let mut cart = ShoppingCart::new();
    cart.add_item("Laptop".to_string(), 999.99);
    cart.add_item("Mouse".to_string(), 29.99);
    cart.add_item("Keyboard".to_string(), 79.99);
    
    println!("Cart total: ${:.2}", cart.total());
    
    let credit_card = CreditCard {
        card_number: "1234567812345678".to_string(),
    };
    println!("{}", cart.checkout(&credit_card));
    
    let paypal = PayPal {
        email: "user@example.com".to_string(),
    };
    println!("{}", cart.checkout(&paypal));
    
    let bitcoin = Bitcoin {
        wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    };
    println!("{}", cart.checkout(&bitcoin));
}

fn demo_decorator() {
    println!("\n{:=^60}", " DECORATOR PATTERN ");
    
    let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee);
    println!("{}: ${:.2}", coffee.description(), coffee.cost());
    
    let coffee = Box::new(MilkDecorator { coffee });
    println!("{}: ${:.2}", coffee.description(), coffee.cost());
    
    let coffee = Box::new(SugarDecorator { coffee });
    println!("{}: ${:.2}", coffee.description(), coffee.cost());
    
    let coffee = Box::new(WhipDecorator { coffee });
    println!("{}: ${:.2}", coffee.description(), coffee.cost());
}

fn demo_builder() {
    println!("\n{:=^60}", " BUILDER PATTERN ");
    
    let basic_computer = ComputerBuilder::new().build();
    println!("Basic: {:?}", basic_computer);
    
    let gaming_computer = ComputerBuilder::new()
        .cpu("AMD Ryzen 9")
        .ram(32)
        .storage(1024)
        .gpu("NVIDIA RTX 4090")
        .build();
    println!("Gaming: {:?}", gaming_computer);
    
    let office_computer = ComputerBuilder::new()
        .cpu("Intel i3")
        .ram(16)
        .storage(512)
        .wifi(true)
        .build();
    println!("Office: {:?}", office_computer);
}

fn main() {
    println!("\n🎨 Design Patterns in Rust 🎨\n");
    println!("Demonstrating 6 classic design patterns with idiomatic Rust");
    
    demo_singleton();
    demo_factory();
    demo_observer();
    demo_strategy();
    demo_decorator();
    demo_builder();
    
    println!("\n{:=^60}", " COMPLETE ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton() {
        let logger1 = Logger::instance();
        let count1 = logger1.get_count();
        Logger::instance().log("Test");
        let count2 = Logger::instance().get_count();
        assert!(count2 > count1);
    }

    #[test]
    fn test_factory() {
        let car = VehicleFactory::create_vehicle(VehicleType::Car, "Test".to_string());
        assert_eq!(car.capacity(), 5);
    }

    #[test]
    fn test_strategy() {
        let mut cart = ShoppingCart::new();
        cart.add_item("Test".to_string(), 10.0);
        assert_eq!(cart.total(), 10.0);
    }

    #[test]
    fn test_decorator() {
        let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee);
        assert_eq!(coffee.cost(), 2.0);
        
        let coffee = Box::new(MilkDecorator { coffee });
        assert_eq!(coffee.cost(), 2.5);
    }

    #[test]
    fn test_builder() {
        let computer = ComputerBuilder::new()
            .cpu("Test CPU")
            .ram(16)
            .build();
        assert_eq!(computer.cpu, "Test CPU");
        assert_eq!(computer.ram, 16);
    }
}
