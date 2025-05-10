use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{env, fmt, fs};

use aoc::*;
use clap::{Parser, Subcommand, ValueEnum};
use reqwest::cookie::Jar;
use reqwest::Url;
use scraper::{Html, Selector};

/// AoC CLI
#[derive(Debug, Parser)]
#[clap(name = "aoc-cli", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Download puzzle text and inputs
    Download {
        /// Year to download
        year: u32,

        /// Day to download
        day: u32,
    },

    /// Run solutions
    Run {
        /// Year to run
        #[arg(short, long, required = false)]
        year: Option<u32>,

        /// Day to run
        #[arg(short, long, required = false)]
        day: Option<u32>,

        /// Input file to use instead of default
        #[arg(short, long, required = false, requires = "year", requires = "day")]
        input_path_override: Option<PathBuf>,

        /// Print totals
        #[arg(short, long, required = false)]
        totals: bool,
    },

    /// Submit puzzle answer
    Submit {
        /// Year to submit
        year: u32,

        /// Day to submit
        day: u32,

        /// Part of the puzzle
        part: PuzzlePart,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum PuzzlePart {
    P1,
    P2,
}

fn main() {
    let args = App::parse();

    if let Err(err) = match args.command {
        Command::Download { year, day } => download(year, day),
        Command::Run { year, day, input_path_override, totals } => {
            run(year, day, input_path_override, totals)
        }
        Command::Submit { year, day, part } => submit(year, day, part),
    } {
        println!("Error: {err}");
    }
}

#[derive(Debug)]
struct SessionTokenError;

impl Error for SessionTokenError {}

impl fmt::Display for SessionTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to find environment variable: 'AOC_SESSION_TOKEN'")
    }
}

#[derive(Debug)]
struct ArticleNotFoundError(String);

impl Error for ArticleNotFoundError {}

impl fmt::Display for ArticleNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to find puzzle text in url: '{}'", self.0)
    }
}

fn download(year: u32, day: u32) -> Result<(), Box<dyn Error>> {
    let client = http_client()?;

    download_puzzle(&client, year, day, false)?;
    download_input(&client, year, day, false)?;

    Ok(())
}

fn download_puzzle(
    client: &reqwest::blocking::Client,
    year: u32,
    day: u32,
    force: bool,
) -> Result<(), Box<dyn Error>> {
    let puzzle_path = Path::new("puzzle")
        .join(format!("y{year}"))
        .join(format!("day{day:02}"))
        .with_extension("md");

    if force || !fs::exists(&puzzle_path)? {
        let puzzle_url = format!("https://adventofcode.com/{year}/day/{day}");

        let r = client.get(&puzzle_url).send()?;

        let html = Html::parse_document(&r.text()?);
        let selector = Selector::parse("article.day-desc")?;

        let file_content = html.select(&selector).fold(String::new(), |acc, article| {
            if let Ok(md) = htmd::convert(&article.html()) {
                if acc.is_empty() { acc + &md } else { acc + &format!("\n\n{md}") }
            } else {
                acc
            }
        });

        if file_content.is_empty() {
            return Err(ArticleNotFoundError(puzzle_url).into());
        }

        fs::create_dir_all(puzzle_path.parent().unwrap())?;
        fs::write(&puzzle_path, file_content)?;
    } else {
        println!("{} exists, skipping...", puzzle_path.display())
    }

    Ok(())
}

fn download_input(
    client: &reqwest::blocking::Client,
    year: u32,
    day: u32,
    force: bool,
) -> Result<(), Box<dyn Error>> {
    let input_path = Path::new("input")
        .join(format!("y{year}"))
        .join(format!("day{day:02}"))
        .with_extension("txt");

    if force || !fs::exists(&input_path)? {
        let input_url = format!("https://adventofcode.com/{year}/day/{day}/input");
        println!("{}", input_url);

        let r = client.get(&input_url).send()?;

        fs::create_dir_all(input_path.parent().unwrap())?;
        fs::write(input_path, r.text()?)?;
    } else {
        println!("{} exists, skipping...", input_path.display())
    }

    Ok(())
}

fn run(
    year: Option<u32>,
    day: Option<u32>,
    input_path_override: Option<PathBuf>,
    totals: bool,
) -> Result<(), Box<dyn Error>> {
    let solutions = filtered_solutions(year, day);

    let mut solved = 0;
    let mut duration = Duration::ZERO;

    for Solution { year, day, input_path, wrapper } in solutions {
        println!("{year} Day {day:02}");

        let filepath = &input_path_override.as_ref().unwrap_or(&input_path);

        let instant = Instant::now();
        match wrapper(filepath.to_str().unwrap()) {
            Ok((part1, part2)) => {
                let elapsed = instant.elapsed();

                solved += if part1.is_some() { 1 } else { 0 };
                solved += if part2.is_some() { 1 } else { 0 };
                duration += elapsed;

                println!("  Part 1: {}", part1.unwrap_or("unsolved".to_owned()));
                println!("  Part 2: {}", part2.unwrap_or("unsolved".to_owned()));
                println!("  Elapsed: {:.03} s", elapsed.as_nanos() as f64 / 1e9);
            }
            Err(err) => {
                if let Ok(parse_error) = err.downcast::<ParseError>() {
                    println!("  {parse_error}")
                } else {
                    println!("  Missing input!");
                    println!("  Place input file in {}", filepath.display());
                }
            }
        }

        println!();
    }

    if totals {
        println!("⭐ {solved}");
        println!("🕓 {:.03} s", duration.as_nanos() as f64 / 1e9);
        println!();
    }

    Ok(())
}

fn submit(year: u32, day: u32, part: PuzzlePart) -> Result<(), Box<dyn Error>> {
    let client = http_client()?;

    let answer = {
        let solutions = filtered_solutions(Some(year), Some(day));
        if solutions.is_empty() {
            Err(format!("No solution found for {year} Day {day:02}").into())
        } else if solutions.len() > 1 {
            Err(format!("Multiple solutions found for {year} Day {day:02}").into())
        } else {
            let Solution { input_path, wrapper, .. } = &solutions[0];
            match wrapper(input_path.to_str().unwrap()) {
                Ok((part1, part2)) => match part {
                    PuzzlePart::P1 => {
                        part1.ok_or(format!("Part 1 unsolved for {year} Day {day:02}").into())
                    }
                    PuzzlePart::P2 => {
                        part2.ok_or(format!("Part 2 unsolved for {year} Day {day:02}").into())
                    }
                },
                Err(err) => Err(err),
            }
        }
    }?;

    let mut params = HashMap::new();
    match part {
        PuzzlePart::P1 => params.insert("level", "1"),
        PuzzlePart::P2 => params.insert("level", "2"),
    };
    params.insert("answer", &answer);

    let submit_url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let r = client.post(&submit_url).form(&params).send()?;
    let url = r.url().clone();

    let html = Html::parse_document(&r.text()?);
    let selector = Selector::parse("article")?;

    let Some(puzzle_text) = html.select(&selector).next() else {
        return Err(ArticleNotFoundError(url.to_string()).into());
    };

    let md = htmd::convert(&puzzle_text.html())?;

    if md.contains("That's the right answer") {
        println!("That's the right answer! Refreshing puzzle description...");
        download_puzzle(&client, year, day, true)?;
        Ok(())
    } else if md.contains("That's not the right answer") {
        print!("That's not the right answer");
        if md.contains("too low") {
            println!(", too low");
        } else if md.contains("too high") {
            println!(", too high");
        }
        Ok(())
    } else if md.contains("You gave an answer too recently") {
        println!("You gave an answer too recently");
        Ok(())
    } else if md.contains("You don't seem to be solving the right level") {
        println!("You don't seem to be solving the right level");
        Ok(())
    } else {
        Err("Failed to parse AoC response".into())
    }
}

fn http_client() -> Result<reqwest::blocking::Client, Box<dyn Error>> {
    let Ok(session_token) = env::var("AOC_SESSION_TOKEN") else {
        return Err(SessionTokenError.into());
    };

    let cookie = format!("session={session_token}");
    let url: Url = "https://adventofcode.com".parse().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    reqwest::blocking::Client::builder()
        .user_agent("github.com/cynicalico/aoc cynicalico@pm.me")
        .cookie_provider(jar.into())
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.into())
}
