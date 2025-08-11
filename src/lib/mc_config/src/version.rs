const CODE:u16=772;
const TEXT:&str= "1.21.8";
pub fn version()->u16{
    CODE
}
pub fn version_text()->&'static str{
    TEXT
}