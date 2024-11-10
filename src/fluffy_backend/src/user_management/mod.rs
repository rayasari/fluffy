pub struct User {
    pub user_id: String,
    // Info pengguna lainnya bisa ditambahkan di sini
}

pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager { users: vec![] }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn get_user(&self, user_id: &String) -> Option<&User> {
        self.users.iter().find(|u| &u.user_id == user_id)
    }
}
