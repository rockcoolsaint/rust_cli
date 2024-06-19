mod theme;

use std::error::Error;

// use colour::{dark_green, yellow};
// use newsapi::{Articles, get_articles};
use dotenv::dotenv;
use newsapi::{Article, Country, Endpoint, NewsAPI};

fn render_articles(articles: &Vec<Article>) {
    // before termimad
    // for i in &articles.articles {
    //     dark_green!("> {}\n", i.title);
    //     yellow!("- {}\n\n", i.url);
    // }
    for i in articles {
        let theme = theme::default();
        theme.print_text(&format!("`{}`", i.title()));
        theme.print_text(&format!("> *{}*", i.url()));
        theme.print_text("---");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Us);

    // let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    
    // let url = format!("{}{}", url, api_key);

    // let articles = get_articles(&url)?; //old code
    let newsapi_response = newsapi.fetch_async().await?;
    render_articles(&newsapi_response.articles());
    Ok(())
}
