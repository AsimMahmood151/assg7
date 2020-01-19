fn main() {
    hr::joining::greetings();
}
mod hr {
    pub mod joining {
        pub fn greetings() {
            println!("Hello! Good to see you guys here.");
        }
    }
}
