/*
    Project Euler
    Problem #22

    Using 0022_names.txt, a 46K text file containing over five-thousand first names, each first
    is surrounded by double quotation marks. You should remove the double quotation marks.
    Begin by sorting it into alphabetical order. Then working out the alphabetical value for each name,
    multiply this value by its alphabetical position in the list to obtain a name score.

    For example, when the list is sorted into alphabetical order, COLIN, which is worth
    3 + 5 +12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of
    938 x 53 = 49714.

    What is the total of all the name scores in the file?
    Write solution in Rust
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;
use std::fs;
use std::result::Result;
use regex::Regex;

/*
    1. Read the names from the file and parse them.
    2. Sort the names alphabetically.
    3. Calculate the alphabetical value for each name.
    4. Multiply the alphabetical value by its position in the sorted list to get the name score.
    5. Sum all the name scores to get the total score.
 */
#[allow(dead_code)]
pub fn find_names_scores() -> u32 {
    // Read the file contents
    let contents = fs::read_to_string("0022_names.txt").expect("Error reading file");

    // Use a regular expression to extract names
    let re = Regex::new(r#""([^"]+)""#).unwrap();
    let mut names: Vec<String> = re.captures_iter(&contents)
        .map(|cap| cap[1].to_string())
        .collect();

    // Sort the names alphabetically
    names.sort();

    // Calculate the total score
    let total_score: u32 = names.iter()
        .enumerate()
        .map(|(index, name)| name_score(name) * (index as u32 + 1))
        .sum();

    //println!("The total of all the name scores in the file is {}", total_score);

    //Ok(())
    total_score
}

#[allow(dead_code)]
fn name_score(name: &str) -> u32 {
    name.chars().map(|c| (c as u32 - 'A' as u32 + 1)).sum()
}

/*
    1. Read the names from the file and parse them.
    2. Sort the names alphabetically.
    3. Calculate the alphabetical value for each name.
    4. Multiply the alphabetical value by its position in the sorted list to get the name score.
    5. Sum all the name scores to get the total score.
 */
#[allow(dead_code)]
pub fn find_names_scores_2() -> Result<i32, Box<dyn Error>> {
    let path = Path::new("0022_names.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut names: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut names_in_line = line.split(",");

        while let Some(name) = names_in_line.next() {
            // trim the surrounding quotes of name before pushing into names
            names.push(name.trim_matches('"').to_string());
        }
    }
    names.sort();
    let mut name_scores: Vec<i32> = Vec::new();
    for (i, name) in names.iter().enumerate() {
        let mut name_score = 0;
        for c in name.chars() {
            name_score += (c as i32) - ('A' as i32) + 1;
        }
        name_scores.push(name_score * (i as i32 + 1));
    }
    Ok(name_scores.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_names_scores() {
        let result = find_names_scores();
        assert_eq!(result, 871198282);
    }

    #[test]
    fn test_find_names_scores_2() {
        let result = find_names_scores_2().unwrap();
        assert_eq!(result, 871198282);
    }
}