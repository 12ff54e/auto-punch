const ENTRY_URL: &str = "https://healthreport.zju.edu.cn/ncov/wap/default/index";

mod dom_helper {
  use reqwest::{Client, Url};
  use scraper::{Html, Selector};

  pub fn title(dom: &Html) -> String {
    let title_selector = Selector::parse("title").unwrap();
    let title_string = dom.select(&title_selector).next().unwrap().inner_html();

    title_string.trim().to_string()
  }

  pub async fn post_login_form(
    client: &Client,
    url: Url,
    dom: &Html,
    id: &str,
    passwd: &str,
  ) -> bool {
    // select input with names, which will show up in formData
    let form_selector = Selector::parse("form").unwrap();
    let input_selector = Selector::parse("input[name]").unwrap();

    let form = dom.select(&form_selector).next().unwrap();

    // let mut query_str = String::new();
    let mut params: Vec<(&str, &str)> = Vec::new();
    for input in form.select(&input_selector) {
      let e = input.value();
      match e.attr("type").unwrap_or("") {
        "text" if e.attr("name").unwrap_or("") == "username" => {
          // query_str.push_str(&format!("username={}&", id));
          params.push(("username", id));
        }
        "password" => {
          // query_str.push_str(&format!("{}={}&", e.attr("name").unwrap(), passwd));
          params.push((e.attr("name").unwrap(), passwd));
        }
        "hidden" => {
          // query_str.push_str(&format!(
          //   "{}={}&",
          //   e.attr("name").unwrap(),
          //   e.attr("value").unwrap()
          // ));
          params.push((e.attr("name").unwrap(), e.attr("value").unwrap()));
        }
        _ => {}
      }
    }

    let task = client.post(url).form(&params).send().await;
    match task {
      Ok(res) => true,
      Err(err) => false,
    }
  }
}

pub fn create_client() -> reqwest::Result<reqwest::Client> {
  use reqwest::{header, ClientBuilder};
  let mut headers = header::HeaderMap::new();
  headers.insert(header::ACCEPT,"text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse().unwrap());
  headers.insert(
    header::ACCEPT_LANGUAGE,
    "zh-CN,zh;q=0.9,en;q=0.8".parse().unwrap(),
  );
  headers.insert(header::PRAGMA, "no-cache".parse().unwrap());
  headers.insert(
    "sec-ch-ua",
    r#"" Not A;Brand";v="99", "Chromium";v="98", "Google Chrome";v="98""#
      .parse()
      .unwrap(),
  );
  headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
  headers.insert("sec-ch-ua-platform", "Windows".parse().unwrap());
  headers.insert(header::UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
  headers.insert(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.82 Safari/537.36".parse().unwrap());

  ClientBuilder::new()
    .cookie_store(true)
    .default_headers(headers)
    .gzip(true)
    .deflate(true)
    .brotli(true)
    .build()
}

async fn check_login(client: &reqwest::Client, id: &str, passwd: &str) -> reqwest::Result<()> {
  let response = client.get(ENTRY_URL).send().await?;
  let url = response.url().clone();
  let html = response.text().await?;

  let dom = scraper::Html::parse_document(&html.to_string());

  if dom_helper::title(&dom) == "统一身份认证平台" {
    // TODO: login
    let _login_result = dom_helper::post_login_form(client, url, &dom, id, passwd).await;
  }

  Ok(())
}

async fn punch_card() {}
