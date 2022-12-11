#[derive(Debug)]
pub struct User {
    name: String,
    age: i32,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    pub fn new(name: String, age: i32, email: String, sign_in_count: u64, active: bool) -> Self {
        Self { name, age, email, sign_in_count, active }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn age(&self) -> i32 {
        self.age
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn sign_in_count(&self) -> u64 {
        self.sign_in_count
    }
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_age(&mut self, age: i32) {
        self.age = age;
    }
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
    pub fn set_sign_in_count(&mut self, sign_in_count: u64) {
        self.sign_in_count = sign_in_count;
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}