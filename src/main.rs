use clap::Parser;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::io;
use std::path::PathBuf;

use chrono::Datelike;
use chrono::Timelike;
use chrono::{NaiveDate, NaiveTime};
use chrono;

struct Post {
    title: String,
    date: String,
    categories: Option<Vec<String>>,
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---")?;
        writeln!(f, "layout: post")?;
        writeln!(f, "title: \"{}\"", self.title)?;
        //writeln!(f, "date: {}", self.date)?;
        if let Some(categories) = &self.categories {
            writeln!(f, "categories: {:?}", categories)?;
        }
        writeln!(f, "---")?;
        Ok(())
    }
}

fn new_date() -> String {
    let current_date = chrono::Utc::now().date_naive();
    let year = current_date.year().to_string();
    let mut month = current_date.month().to_string();
    let mut day = current_date.day().to_string();

    if month.parse::<u32>().unwrap() < 10 {
        month = format!("0{}", month);
    }

    if day.parse::<u32>().unwrap() < 10 {
        day = format!("0{}", day);
    }

    format!("{}-{}-{}", year, month, day)
}

fn new_post(project_root_path: &PathBuf, post_title: &str) -> io::Result<String> {
    let paths_dir = project_root_path.join("_posts");

    if !paths_dir.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No _posts directory found. Are you in the root of your Jekyll project?"));
    }

    let current_date = new_date();

    // Convert the title to a filename
    let file_title = post_title.to_lowercase().replace(" ", "-");
    let file_title = format!("{}-{}.md", current_date, file_title);

    let file_path = paths_dir.join(file_title.clone());
    let mut file = File::create(file_path)?;

    // Write the front matter
    let post = Post {
        title: post_title.to_string(),
        date: current_date,
        categories: None,
    };

    file.write_all(post.to_string().as_bytes())?;

    Ok(file_title)
}

/// Command line arguments
#[derive(clap::Parser)]
struct Args {
    /// Title of the post
    #[clap(default_value = "New Post")]
    post_title: String,
}

/// Entry point
fn main() {
    let args = Args::parse();

    let cwd = std::env::current_dir().unwrap();
    let res = new_post(&cwd, &args.post_title);

    match res {
        Ok(filename) => println!("Post created: {}", filename),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        },
    }
}
