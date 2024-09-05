use crate::{
    anime::{Anime, AnimeIndex},
    spider::Spider,
};
use regex::Regex;
use scraper::{Html, Selector};

pub struct Bangumi;

impl Bangumi {
    const URL: &'static str = "https://bangumi.tv/";
    const ID: &'static str = "Bangumi";
}

impl Spider for Bangumi {
    fn get_id() -> String {
        Bangumi::ID.to_string()
    }

    fn net_is_ok() -> bool {
        let resp = tauri::async_runtime::block_on(async { reqwest::get(Bangumi::URL).await });

        if let Ok(r) = resp {
            r.status().is_success()
        } else {
            false
        }
    }

    async fn get_anime_index(key_words: &str) -> anyhow::Result<Vec<AnimeIndex>> {
        let url = format!("{}subject_search/{}?cat=2", Bangumi::URL, key_words);
        let resp = tauri::async_runtime::spawn(async { reqwest::get(url).await }).await??;
        let doc = Html::parse_document(&resp.text().await?);
        let selector = Selector::parse(".item h3>a").unwrap();

        let mut anime_names = vec![];
        let temp_anime_names = doc
            .select(&selector)
            .into_iter()
            .map(|e| {
                let name = e.inner_html();
                let id = e.attr("href").unwrap().to_string();
                AnimeIndex {
                    name,
                    id,
                    edit_distance: usize::default(),
                }
            })
            .collect::<Vec<_>>();
        anime_names.extend(temp_anime_names);

        let page_selector = Selector::parse("span.p_edge").unwrap();
        let page = if let Some(e) = doc.select(&page_selector).next() {
            let page_str = e.text().next().unwrap();
            let re = Regex::new(r"\(\s*\d+\s*/\s*(\d+)\s*\)").unwrap();
            let max_page = re
                .captures(page_str)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().parse::<usize>().unwrap()))
                .unwrap();
            if max_page > 5 {
                5
            } else {
                max_page
            }
        } else {
            return Ok(anime_names);
        };
        let mut handle_vec = vec![];
        for i in 2..=page {
            let url = format!(
                "{}subject_search/{}?cat=2&page={}",
                Bangumi::URL,
                key_words,
                i
            );
            let handle = tauri::async_runtime::spawn(async { reqwest::get(url).await });
            handle_vec.push(handle);
        }
        for handle in handle_vec {
            let resp = handle.await??;
            let doc = Html::parse_document(&resp.text().await?);
            let temp_anime_names = doc
                .select(&selector)
                .into_iter()
                .map(|e| {
                    let name = e.inner_html();
                    let id = e.attr("href").unwrap().to_string();
                    AnimeIndex {
                        name,
                        id,
                        edit_distance: usize::default(),
                    }
                })
                .collect::<Vec<AnimeIndex>>();
            anime_names.extend(temp_anime_names)
        }

        Ok(anime_names)
    }

    async fn get_anime_info(anime_index: AnimeIndex) -> anyhow::Result<Anime> {
        let url = format!("{}{}", Bangumi::URL, anime_index.id);
        let url_cv = format!("{}/characters", url);
        let client = reqwest::Client::new();
        let resp = client.get(&url).send().await?.text().await?;
        let resp_cv = client.get(&url_cv).send().await?.text().await?;
        let document = Html::parse_document(&resp);
        let selector = Selector::parse("#subject_summary").unwrap();
        let description = document
            .select(&selector)
            .map(|e| e.inner_html())
            .collect::<String>();
        let selector = Selector::parse("#infobox>li").unwrap();
        let staff = document
            .select(&selector)
            .map(|e| {
                let mut text = e.text();
                let staff_key = text.next().unwrap().to_string();
                let staff_value = text.collect::<Vec<_>>().join("");
                (staff_key, staff_value)
            })
            .collect::<Vec<_>>();
        let selector = Selector::parse("img.cover").unwrap();
        let cover_url = document
            .select(&selector)
            .map(|e| e.attr("src").unwrap().to_owned())
            .collect::<Vec<_>>();
        let document = Html::parse_document(&resp_cv);
        let selector = Selector::parse(".column>.light_odd").unwrap();
        let cast = document
            .select(&selector)
            .map(|e| {
                let key_selector = Selector::parse(".clearit>h2>span").unwrap();
                let value_selector = Selector::parse(".clearit>.actorBadge>p>small").unwrap();
                let cast_key = match e
                    .select(&key_selector)
                    .map(|e| e.inner_html())
                    .collect::<Vec<_>>()
                    .join("")
                    .replace("/", "")
                {
                    s if !s.is_empty() => s,
                    _ => "暂无信息".to_owned(),
                };
                let cast_value = match e
                    .select(&value_selector)
                    .map(|e| e.inner_html())
                    .collect::<Vec<_>>()
                    .join("")
                {
                    s if !s.is_empty() => s,
                    _ => "暂无信息".to_owned(),
                };
                (cast_key, cast_value)
            })
            .collect::<Vec<_>>();
        let anime = Anime {
            title: anime_index.name,
            description,
            staff,
            cast,
            cover: format!("https:{}", &cover_url[0]),
        };
        Ok(anime)
    }
}
