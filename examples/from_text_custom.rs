use h2s::FromHtml;
use h2s_core::from_text::FromText;
use std::num::ParseIntError;

fn main() {
    // You can define an external parseable type yourself
    // Currently you have to define a newtype for an external crate struct
    struct Duration(std::time::Duration);
    impl FromText for Duration {
        type Error = ParseIntError;

        fn from_text(s: &str) -> Result<Self, Self::Error> {
            let sec = s.parse()?;
            Ok(Duration(std::time::Duration::from_secs(sec)))
        }
    }

    #[derive(FromHtml)]
    struct MyStruct {
        #[h2s(select = "div")]
        duration1: Duration,
        #[h2s(select = "div", attr = "seconds")]
        duration2: Duration,
    }

    let my_struct = h2s::parse::<MyStruct>(r#"<div seconds="456">123</div>"#).unwrap();
    assert_eq!(my_struct.duration1.0, std::time::Duration::from_secs(123));
    assert_eq!(my_struct.duration2.0, std::time::Duration::from_secs(456));
}

#[test]
fn test() {
    main();
}
