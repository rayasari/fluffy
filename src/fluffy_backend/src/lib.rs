mod data_storage;
mod user_management;
mod data_visualization;
mod api_interface;
mod utils;

use data_storage::DataStorage;
use user_management::UserManager;

#[ic_cdk::init]
fn init() {
    ic_cdk::println!("Fluffy Backend Initialized");
}
