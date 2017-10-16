extern crate lib;
extern create util;

use lib::{connector, filterer, scraper};

fn main() {
  // Connect to a site.
  let connection = connector::connect();

  // Scrape some Audio.
  let mut media_vec: vec<Media> = scraper::getMedia();

  // filter City Pop Audio.
  let city_pop: vec<Media> = filterer::filter_by(util::Type::Genre, media_vec);
}
