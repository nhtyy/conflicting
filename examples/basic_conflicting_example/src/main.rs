use conflicting::conflicting;

fn main() {
    println!("Hello, world!");
}

fn feat_string() -> &'static str {
     conflicting! {
         "a" => {
             "a"
         },
         "b" => {
             "b"
         },
         "c" => {
             "c"
         },
     }
 }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_feat_string() {
        assert_eq!(feat_string(), "a");
    }
}
