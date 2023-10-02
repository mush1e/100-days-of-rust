fn get_or_default(arg: Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.clone()
}
 fn main() {
    get_or_default(None);
 } 