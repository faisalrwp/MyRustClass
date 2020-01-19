mod pakistan
{
    pub mod sindh
    {
        pub fn education() { println!("WE are in pakistan sindh education"); }
    } 
    pub mod punjab
    {
        pub fn education() { println!("WE are in pakistan punjab education"); }
    }
}

mod kashmir
{
    pub mod bagh
    {
        pub fn literacy() {println!("WE are in kashmir bagh literacy");}
    }
    pub mod muzaffarabad
    {
        pub fn literacy() {println!("WE are in kashmir muzaffarabad literacy");}
    }
}
mod usa;
use pakistan::sindh;
fn main() {
    println!("Hello, world!");
    crate::pakistan::sindh::education();
    sindh::education();
    crate::pakistan::punjab::education();
    crate::kashmir::bagh::literacy();
    crate::kashmir::muzaffarabad::literacy();
    usa::states::berkley::university();
}
