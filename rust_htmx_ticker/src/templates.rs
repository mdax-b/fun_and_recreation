use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "results.html")]
pub struct SearchResults {
    pub search_results: Vec<SearchResult>,
}

pub struct SearchResult {
    pub name: String,
    pub ticker: String,
}

#[derive(Template)]
#[template(path = "values.html")]
pub struct TickerValues {
    pub ticker: String,
    pub values: Values,
}

pub struct Values {
    pub open: f64,
    pub high: f64,
    pub low: f64,
}

impl SearchResults {
    pub fn new(search_results: Vec<SearchResult>) -> Self {
        Self { search_results }
    }
}

impl Default for SearchResults {
    fn default() -> Self {
        Self {
            search_results: vec![SearchResult {
                name: "A company".to_string(),
                ticker: "XYZ".to_string(),
            }],
        }
    }
}

impl Default for Values {
    fn default() -> Self {
        Self {
            open: 330.123,
            high: 340.41,
            low: 320.79,
        }
    }
}
