use ic_cdk_macros::{query, update};
use crate::data_storage::{DataPoint, DataStorage};
use crate::user_management::{User, UserManager};

static mut DATA_STORAGE: Option<DataStorage> = None;
static mut USER_MANAGER: Option<UserManager> = None;

#[update]
fn add_user(user_id: String) {
    unsafe {
        let manager = USER_MANAGER.get_or_insert(UserManager::new());
        manager.add_user(User { user_id });
    }
}

#[update]
fn add_data(user_id: String, timestamp: u64, value: f64) {
    let data_point = DataPoint { timestamp, value };
    unsafe {
        let storage = DATA_STORAGE.get_or_insert(DataStorage::new());
        storage.add_data(user_id, data_point);
    }
}

#[query]
fn get_data(user_id: String) -> Vec<(u64, f64)> {
    unsafe {
        let storage = DATA_STORAGE.as_ref().unwrap();
        if let Some(data) = storage.get_data(&user_id) {
            data.iter().map(|d| (d.timestamp, d.value)).collect()
        } else {
            vec![]
        }
    }
}
