pub mod constants;
pub mod de;
pub mod error;
pub mod ser;
pub mod value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
