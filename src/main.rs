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

extern crate regex;

use std::io::{self, Write};
use std::process::Command;
use regex::Regex;

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

fn print_results(world_results: &mut Vec<WorldResult>) {
  sort_by_ping(world_results);

  for world_result in world_results.iter() {
    println!("World {} ({}ms)", world_result.world_id, world_result.average_ping);
  }
}

fn main() {
  let worlds = vec![1isize,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,65,66,67,68,69,70,73,74,75,76,77,78,81,82,83,84,85,86,93,94,95,96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,129,130,131,132,132,133,134,135,136,137,138,139,140];
 
  let avg_regex = Regex::new(r"min/avg/max/mdev = ([0-9\.]*)/([0-9\.]*)/([0-9\.]*)/([0-9\.]*)").unwrap();
  let mut world_results = Vec::new();

  for world_id in worlds.iter() {
    let target_server = &format!("world{}.runescape.com", world_id);
    let ping_result = Command::new("ping").args(&["-c", "3", target_server]).output().expect("failed to execute ping");
    let ping_text = String::from_utf8_lossy(&ping_result.stdout);
    
    for capture in avg_regex.captures_iter(&ping_text) {
      let ping = capture[2].parse::<f32>().unwrap();
      world_results.push(process_world(*world_id, ping));  
    }

    print_current_best(&mut world_results);
  }

  println!("");

  print_results(&mut world_results);
}

