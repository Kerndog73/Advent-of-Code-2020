use adventofcode2020::*;
use std::collections::HashSet;

fn parse_input() -> HashSet::<(i32, i32, i32)> {
    let mut tiles = HashSet::new();

    lines_from_file("input/day_24.txt", |line| {
        let mut tile = (0, 0, 0);
        let mut char_iter = line.chars();

        loop {
            let ch = match char_iter.next() {
                Some(ch) => ch,
                None => break
            };

            // https://www.redblobgames.com/grids/hexagons/
            match ch {
                'e' => { tile.0 += 1; tile.1 -= 1; }
                'w' => { tile.0 -= 1; tile.1 += 1; }
                'n' => match char_iter.next().unwrap() {
                    'e' => { tile.0 += 1; tile.2 -= 1; }
                    'w' => { tile.1 += 1; tile.2 -= 1; }
                    _ => panic!()
                },
                's' => match char_iter.next().unwrap() {
                    'e' => { tile.1 -= 1; tile.2 += 1; }
                    'w' => { tile.0 -= 1; tile.2 += 1; }
                    _ => panic!()
                },
                _ => panic!()
            }
        }

        if !tiles.remove(&tile) {
            tiles.insert(tile);
        }
    });

    tiles
}

fn simulate(tiles: &mut HashSet<(i32, i32, i32)>) {
    let mut next_tiles = HashSet::<(i32, i32, i32)>::new();

    for _ in 0..100 {
        next_tiles.clear();

        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut min_z = i32::MAX;
        let mut max_z = i32::MIN;
        for tile in tiles.iter() {
            min_x = min_x.min(tile.0);
            max_x = max_x.max(tile.0);
            min_y = min_y.min(tile.1);
            max_y = max_y.max(tile.1);
            min_z = min_z.min(tile.2);
            max_z = max_z.max(tile.2);
        }

        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    if x + y + z != 0 {
                        continue;
                    }
                    let neighbors = [
                        (x + 1, y - 1, z    ),
                        (x - 1, y + 1, z    ),
                        (x + 1, y,     z - 1),
                        (x,     y + 1, z - 1),
                        (x,     y - 1, z + 1),
                        (x - 1, y,     z + 1),
                    ];
                    let black_count = neighbors.iter()
                        .filter(|neighbor| tiles.contains(neighbor))
                        .count();
                    if black_count == 2 || (black_count == 1 && tiles.contains(&(x, y, z))) {
                        next_tiles.insert((x, y, z));
                    }
                }
            }
        }

        std::mem::swap(tiles, &mut next_tiles);
    }
}

fn main() {
    let mut tiles = parse_input();
    println!("Part one: {}", tiles.len());
    simulate(&mut tiles);
    println!("Part two: {}", tiles.len());
}
