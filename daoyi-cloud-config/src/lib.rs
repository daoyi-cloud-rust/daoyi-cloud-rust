pub mod common_config;

#[cfg(test)]
mod tests {
    use super::common_config;

    #[test]
    fn it_works() {
        common_config::init(None);
    }
}
