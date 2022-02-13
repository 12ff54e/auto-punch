use reqwest::Client;
use scraper::Html;

const ENTRY_URL: &str = "https://healthreport.zju.edu.cn/ncov/wap/default/index";

mod dom_helper {
  use scraper::{Html, Selector};

  pub fn title(dom: &Html) -> String {
    let title_selector = Selector::parse("title").unwrap();
    let title_string = dom.select(&title_selector).next().unwrap().inner_html();

    title_string.trim().to_string()
  }

  pub fn fill_login_form(dom: &Html, id: &str, passwd: &str) -> String {
    // select input with names, which will show up in formData
    let input_selector = Selector::parse("form input[name]").unwrap();

    let mut query_str = String::new();
    for input in dom.select(&input_selector) {
      let e = input.value();
      match e.attr("type").unwrap_or("") {
        "text" if e.attr("name").unwrap_or("") == "username" => {
          query_str.push_str(&format!("username={}&", id));
        }
        "password" => {
          query_str.push_str(&format!("{}={}&", e.attr("name").unwrap(), passwd));
        }
        "hidden" => {
          query_str.push_str(&format!(
            "{}={}&",
            e.attr("name").unwrap(),
            e.attr("value").unwrap()
          ));
        }
        _ => {}
      }
    }

    // get rid of last '&'
    query_str.pop().unwrap();
    query_str
  }
}

async fn check_login(client: &Client, id: &str, passwd: &str) -> reqwest::Result<()> {
  let response = client.get(ENTRY_URL).send().await?;
  let html = response.text().await?;

  let dom = Html::parse_document(&html.to_string());

  if dom_helper::title(&dom) == "统一身份认证平台" {
    let post_string = dom_helper::fill_login_form(&dom, id, passwd);
    // TODO: login
  }

  Ok(())
}
