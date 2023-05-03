use crate::{
    model::{Album, AlbumResponse},
    COLORS,
};
use chrono::Utc;
use colored::Colorize;
use std::{collections::HashMap, fmt::Display};

pub struct GroupedResults {
    items: Vec<(String, Vec<Album>)>,
}

impl From<AlbumResponse> for GroupedResults {
    fn from(value: AlbumResponse) -> Self {
        let mut grouped: HashMap<String, Vec<Album>> = HashMap::new();

        for item in value.results.iter() {
            if !grouped.contains_key(&item.artist_name) {
                grouped.insert(item.artist_name.clone(), Vec::new());
            }
        }

        for item in value.results.iter() {
            grouped
                .get_mut(&item.artist_name)
                .unwrap()
                .push(item.clone());
        }

        let mut sorted_groups = grouped.into_iter().collect::<Vec<_>>();
        sorted_groups.sort_by(|a, b| a.0.cmp(&b.to_owned().0));

        Self {
            items: sorted_groups,
        }
    }
}

impl Display for GroupedResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (artist, group) in self.items.iter() {
            writeln!(
                f,
                "{}",
                artist.truecolor(COLORS[1].0, COLORS[1].1, COLORS[1].2)
            )?;

            for album in group {
                let release_date: chrono::DateTime<Utc> = album.release_date.into();

                writeln!(
                    f,
                    "    {} {}",
                    release_date
                        .format("%Y")
                        .to_string()
                        .truecolor(COLORS[0].0, COLORS[0].1, COLORS[0].2)
                        .bold(),
                    album
                        .collection_name
                        .truecolor(COLORS[2].0, COLORS[2].1, COLORS[2].2)
                )?;
            }
        }

        Ok(())
    }
}
