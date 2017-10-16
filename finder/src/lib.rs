pub mod scraper;

pub mod connector;

pub mod filterer;

pub mod util;

#[cfg(test)]
mod tests {
    use super::scraper;
    use super::connector;
    use super::filterer;
    #[test]
    fn it_works() {
      connector::connect();
    }
}
