mod asura;
mod flamescans;
mod lnreader;
mod manganato;
mod mangareader;
mod manhuanaus;
mod manhwafreak;
mod webtoon;
mod wuxiax;

use core::fmt;
use std::num::ParseFloatError;

use futures::StreamExt;

use crate::common_fn::get_site_as_string;
use crate::structs_and_types::{Entry, Site, SiteDomain};

#[derive(Debug)]
pub enum CheckError {
    Request(reqwest::Error),
    Parse(String),
    FloatErr(ParseFloatError),
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Checkerror")
    }
}

async fn site_handler(site: &Site) -> Result<f32, CheckError> {
    let site_text = get_site_as_string(&site.url)
        .await
        .map_err(|err| CheckError::Request(err))?;

    let parsed_text = {
        match site.domain {
            SiteDomain::Manganato => manganato::check(site_text, site.url.clone()),
            SiteDomain::Asura => asura::check(site_text, site.url.clone()),
            SiteDomain::Wuxiax => wuxiax::check(site_text, site.url.clone()),
            SiteDomain::Flamescans => flamescans::check(site_text, site.url.clone()),
            SiteDomain::Lnreader => lnreader::check(site_text, site.url.clone()),
            SiteDomain::Manhwafreak => manhwafreak::check(site_text, site.url.clone()),
            SiteDomain::Manhuaus => manhuanaus::check(site_text, site.url.clone()),
            SiteDomain::Webtoon => webtoon::check(site_text, site.url.clone()),
            SiteDomain::Mangareader => mangareader::check(site_text, site.url.clone()),
        }
    };

    let parsed_text = match parsed_text {
        Ok(chapter) => chapter,
        Err(err) => {
            println!("Error parsing site: {}", site.url);
            println!("{:?}", err);
            return Err(err);
        }
    };

    Ok(parsed_text)
}

pub async fn handle(entry: &Entry) -> Result<(Entry, f32), CheckError> {
    let mut highest_chapter_found: f32 = 0.0;

    let fetched_entries: Vec<Result<f32, CheckError>> =
        futures::stream::iter(entry.urls.iter().map(site_handler))
            .buffer_unordered(1)
            .collect()
            .await;

    for result in fetched_entries {
        if let Ok(chapter) = result {
            if highest_chapter_found < chapter {
                highest_chapter_found = chapter;
            }
        }
    }

    Ok((entry.clone(), highest_chapter_found))
}
