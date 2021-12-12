use std::collections::HashSet;

enum CaveKind {
    Start,
    End,
    Small,
    Large,
}
impl CaveKind {
    pub fn from(cave_name: &str) -> Self {
        match cave_name {
            "start" => Self::Start,
            "end" => Self::End,
            cave => {
                if cave.to_ascii_lowercase() == cave {
                    Self::Small
                } else {
                    Self::Large
                }
            }
        }
    }
}

struct Cave {
    name: String,
    links: Vec<String>,
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        Self {
            name: input.to_string(),
            links: Vec::new(),
        }
    }
}

fn parse_input(input: String) -> Vec<Cave> {
    let mut came_across: HashSet<&str> = HashSet::default();
    let mut caves = Vec::default();
    let lines: Vec<&str> = input.split('\n').collect();
    for line in &lines {
        for cave in line.split('-') {
            if came_across.contains(cave) {
                continue;
            }
            caves.push(Cave::from(cave));
            came_across.insert(cave);
        }
    }

    for line in &lines {
        if let Some((cave_a, cave_b)) = line.split_once('-') {
            caves
                .iter_mut()
                .filter(|c| c.name == cave_a)
                .for_each(|c| c.links.push(cave_b.to_string()));
            caves
                .iter_mut()
                .filter(|c| c.name == cave_b)
                .for_each(|c| c.links.push(cave_a.to_string()));
        }
    }

    return caves;
}

fn any_small_cave_revisited_twice(path: &String) -> bool {
    let mut visited: HashSet<&str> = HashSet::default();
    for small_cave in path.split('-').filter(|c| match CaveKind::from(c) {
        CaveKind::Small => true,
        _ => false,
    }) {
        if visited.contains(small_cave) {
            return true;
        }
        visited.insert(small_cave);
    }
    return false;
}

fn count_paths(caves: &Vec<Cave>, allow_revisits: bool) -> usize {
    let mut paths: HashSet<String> = HashSet::default();
    let mut to_visit: Vec<String> = Vec::default();
    to_visit.push("start".to_string());

    while to_visit.len() > 0 {
        let current_path = to_visit.pop().unwrap();
        let current_name = current_path.split("-").last().unwrap();
        let current_cave = caves
            .iter()
            .find(|c| c.name == current_name.to_string())
            .unwrap();

        for link in &current_cave.links {
            let new_path = format!("{}-{}", current_path, link);

            match CaveKind::from(&link) {
                CaveKind::Start => {} // Revisited start, path dies
                CaveKind::End => {
                    paths.insert(new_path);
                } // Visited end, path completed!
                CaveKind::Large => to_visit.push(new_path), // Visited large cave, path continues
                CaveKind::Small => {
                    let small_cave_visited_count =
                        current_path.split('-').filter(|c| c == link).count();

                    match small_cave_visited_count {
                        0 => to_visit.push(new_path),
                        1 if allow_revisits && !any_small_cave_revisited_twice(&current_path) => {
                            to_visit.push(new_path)
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    return paths.len();
}

pub fn part1(input: String) -> usize {
    let caves = parse_input(input);

    return count_paths(&caves, false);
}

pub fn part2(input: String) -> usize {
    let caves = parse_input(input);

    return count_paths(&caves, true);
}
