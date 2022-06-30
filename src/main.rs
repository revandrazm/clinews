mod theme;

use dotenv::dotenv;
use std::error::Error;

use newsapi::{get_articles, Articles};

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top Headlines\n\n");

    for article in &articles.articles {
        theme.print_text(&format!("`{}`", article.title));
        theme.print_text(&format!(
            "> *{}{}*",
            article.url,
            if article.url.chars().nth(article.url.len() - 1).unwrap() != '/' {
                '/'
            } else {
                '\0'
            }
        ));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    #![allow(unused_must_use)]
    dotenv();

    let api_key = std::env::var("API_KEY")?;

    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    let url = format!("{}{}", url, api_key);

    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}
