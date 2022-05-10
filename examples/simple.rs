use h2s::FromHtml;

fn main() {
    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Page {
        #[h2s(select = "html", attr = "lang")]
        lang: String,
        #[h2s(select = "h1.blog-title")]
        blog_title: String,
        #[h2s(select = ".articles > div > .detail > .author")]
        authors: Vec<String>,
        #[h2s(select = ".articles > div")]
        articles: Vec<ArticleElem>,
    }

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct ArticleElem {
        #[h2s(attr = "data-index")]
        index: String,
        #[h2s(select = "h2")]
        title: String,
        #[h2s(select = ".detail > .author")]
        author: String,
        #[h2s(select = ".detail > .date")]
        date: String,
        #[h2s(select = ".detail > .modified-date")]
        modified_date: Option<String>,
    }

    let html = r#"
<!DOCTYPE html>
<html lang="en">
<body>
<h1 class="blog-title">My tech blog</h1>
<div class="articles">
    <div data-index="1">
        <h2>article1</h2>
        <div class="detail">
            <span class="author">ikeno</span>
            <span class="date">2020-01-01</span>
        </div>
    </div>
    <div data-index="2">
        <h2>article2</h2>
        <div class="detail" data-index="2">
            <span class="author">alice</span>
            <span class="date">2020-02-01</span>
        </div>
    </div>
    <div data-index="3">
        <h2>article3</h2>
        <div class="detail" data-index="3">
            <span class="author">bob</span>
            <span class="date">2020-03-01</span>
            <span class="modified-date">2020-05-01</span>
        </div>
    </div>
</div>
</body>
</html>
    "#;

    let res: Page = h2s::extract_from_html(html).unwrap();
    println!("{:#?}", res);
}
