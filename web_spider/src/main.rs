use scraper::Selector;
use std::iter::zip;

fn main() {
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();
    let year_selector: Selector =
        scraper::Selector::parse("div.ratings-bar>div>strong").unwrap();
    let ratings = document.select(&year_selector).map(|x| x.inner_html());
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    // titles
    //     .zip(years)
    //     .for_each(|(item, number)| println!("{}. {}", number, item));

    // years
    //     .zip(1..101)
    //     .for_each(|(item, number)| println!("{}. {}", number, item));
    let  iter = zip(titles, ratings);
    iter.for_each(|(title, rate)| println!("movie title: {title} rating: {rate}" ));
    
    
}
