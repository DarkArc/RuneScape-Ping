/*
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
#[macro_use]
extern crate lazy_static;

extern crate regex;
extern crate clap;

use std::io::{self, Write};
use std::process::Command;
use regex::Regex;
use clap::{Arg, App, ArgMatches};

struct WorldResult {
  world_id: isize,
  average_ping: f32
}

fn process_world(world_id: isize, average_ping: f32) -> WorldResult {
  WorldResult {
    world_id: world_id,
    average_ping: average_ping
  }
}

fn sort_by_ping(world_results: &mut Vec<WorldResult>) {
  world_results.sort_by( |a, b| a.average_ping.partial_cmp(&b.average_ping).unwrap() )
}

fn print_current_best(world_results: &mut Vec<WorldResult>) {
  sort_by_ping(world_results);

  match world_results.first() {
    Some(best_match) => print!("\rCurrent best match: World {} ({}ms); Checked {} servers", best_match.world_id, best_match.average_ping, world_results.len()),
    None => print!("No match found")
  }

  io::stdout().flush().unwrap();
}

fn print_results(world_results: &mut Vec<WorldResult>, count: &usize) {
  sort_by_ping(world_results);

  let num_of_worlds = std::cmp::max(1, std::cmp::min(world_results.len(), *count));

  for i in 0..num_of_worlds {
      println!("World {} ({}ms)", &world_results[i].world_id, &world_results[i].average_ping);
  }
}

fn parse_args() -> ArgMatches<'static> {
  return App::new("RuneScape Ping")
    .arg(Arg::with_name("members_only")
        .short("m")
        .long("members_only")
        .help("Only test member worlds"))
    .arg(Arg::with_name("ftp_only")
        .short("f")
        .long("ftp_only")
        .conflicts_with("members_only")
        .help("Only test free to play worlds"))
    .arg(Arg::with_name("worldset")
        .short("w")
        .long("worlds")
        .multiple(true)
        .takes_value(true)
        .conflicts_with("members_only")
        .conflicts_with("ftp_only")
        .help("Custom world list to test"))
    .arg(Arg::with_name("count")
        .short("c")
        .long("resultCount")
        .takes_value(true)
        .help("Number of results to display"))
    .get_matches();
}

fn get_target_worlds(args: &ArgMatches) -> Vec<isize> {
  let mut target_worlds = vec![];
  if args.is_present("members_only") {
      target_worlds.extend(MEMBER_WORLDS.iter());
  } else if args.is_present("ftp_only") {
      target_worlds.extend(FTP_WORLDS.iter());
  } else if args.is_present("worldset") {
      if let Some(worlds) = args.values_of("worldset") {
          for x in worlds {
              let temp: isize = x.parse::<isize>().unwrap();
              if MEMBER_WORLDS.contains(&temp) || FTP_WORLDS.contains(&temp) {
                  target_worlds.push(temp);
              } else {
                  println!("\"{:?}\" is not a valid world!", temp);
              }
          }
      }
  } else {
      target_worlds.extend(MEMBER_WORLDS.iter());
      target_worlds.extend(FTP_WORLDS.iter());
  }
  return target_worlds;
}


lazy_static! {
  static ref FTP_WORLDS: Vec<isize> = {
    let ftp = vec![
      3, 7, 8, 11, 17, 19, 20, 29, 33, 34, 38, 41, 43,
      55, 57, 61, 80, 81, 94, 101, 108, 120, 122, 135,
      136, 141
    ];
    ftp
  };

  static ref MEMBER_WORLDS: Vec<isize> = {
    let mem = vec![
      1, 2, 4, 5, 6, 9, 10, 12, 14, 15, 16, 18, 21,
      22, 23, 24, 25, 26, 27, 28, 30, 31, 32, 35, 36,
      37, 39, 40, 42, 44, 45, 46, 48, 49, 50, 51, 52,
      53, 54, 56, 58, 59, 60, 62, 63, 64, 65, 66, 67,
      68, 69, 70, 71, 72, 73, 74, 76, 77, 78, 79, 82,
      83, 84, 85, 86, 87, 88, 89, 91, 92, 96, 97, 98,
      99, 100, 103, 104, 105, 106, 114, 115, 116, 117,
      119, 120, 123, 124, 134, 137, 138, 139, 140
    ];
    mem
  };
}

fn main() {
  let args = parse_args();
  let target_worlds = get_target_worlds(&args);

  let avg_regex = Regex::new(r"min/avg/max/mdev = ([0-9\.]*)/([0-9\.]*)/([0-9\.]*)/([0-9\.]*)").unwrap();
  let mut world_results = Vec::new();

  for world_id in target_worlds.iter() {
    let target_server = format!("world{}.runescape.com", world_id);
    let ping_result = Command::new("ping").args(&["-c", "3", &target_server]).output().expect("failed to execute ping");
    let ping_text = String::from_utf8_lossy(&ping_result.stdout);

    for capture in avg_regex.captures_iter(&ping_text) {
      let ping = capture[2].parse::<f32>().unwrap();
      world_results.push(process_world(*world_id, ping));
    }

    print_current_best(&mut world_results);
  }

  println!("");
  let mut result_count = 5usize;
  if let Some(rc) = args.value_of("count") {
    result_count = rc.parse::<usize>().unwrap();
  }

  print_results(&mut world_results, &result_count);
}
