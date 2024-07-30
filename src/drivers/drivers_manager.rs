// drivers/driver_manager.rs

// Import necessary traits and driver implementations
use crate::drivers::{gpu::Gpu, keyboard::Keyboard, network::Network, storage::Storage};

pub struct DriverManager {
    gpu: Option<Box<dyn Gpu>>,
    keyboard: Option<Keyboard>,
    network: Option<Network>,
    storage: Option<Storage>,
}

impl DriverManager {
    // Initialize the DriverManager with default or detected drivers
    pub fn new() -> DriverManager {
        DriverManager {
            gpu: None,
            keyboard: None,
            network: None,
            storage: None,
        }
    }

    // Set or detect the GPU driver
    pub fn set_gpu(&mut self, gpu: Box<dyn Gpu>) {
        self.gpu = Some(gpu);
    }

    // Initialize GPU driver
    pub fn init_gpu(&mut self) {
        self.gpu = Some(drivers::gpu::init_gpu());
    }

    // Initialize keyboard driver
    pub fn init_keyboard(&mut self) {
        self.keyboard = Some(Keyboard::new());
    }

    // Initialize network driver
    pub fn init_network(&mut self) {
        self.network = Some(Network::new());
    }

    // Initialize storage driver
    pub fn init_storage(&mut self) {
        self.storage = Some(Storage::new());
    }

    // Accessor methods for drivers
    pub fn get_gpu(&self) -> Option<&Box<dyn Gpu>> {
        self.gpu.as_ref()
    }

    pub fn get_keyboard(&self) -> Option<&Keyboard> {
        self.keyboard.as_ref()
    }

    pub fn get_network(&self) -> Option<&Network> {
        self.network.as_ref()
    }

    pub fn get_storage(&self) -> Option<&Storage> {
        self.storage.as_ref()
    }
}
