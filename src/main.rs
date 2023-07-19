use std::{env, io::BufRead};

type River = Vec<Vec<char>>;

fn is_valid(river: &River, row: usize, col: usize) -> bool {
    let rows = river.len();
    let cols = river[0].len();
    row >= 0 as usize && row < rows && col >= 0 as usize && col < cols
}

fn is_river(cell: char) -> bool {
    cell == '1'
}

fn find_river(river: &mut River) -> Vec<usize> {
    let mut visited = Vec::<(usize, usize)>::new();
    let mut res = Vec::<usize>::new();
    river.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, &cell)| {
            if is_river(cell) && is_valid(river, x, y) && !visited.contains(&(x, y)) {
                let mut stack = Vec::<(usize, usize)>::new();
                stack.push((x, y));
                visited.push((x, y));
                let mut count = 1;
                while let Some((i, j)) = stack.pop() {
                    let dir = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                    for &(k, l) in dir.iter() {
                        let new_i = (i as isize + k) as usize;
                        let new_j = (j as isize + l) as usize;

                        if is_valid(river, new_i, new_j) && !visited.contains(&(new_i, new_j)) {
                            if is_river(river[new_i][new_j]) {
                                stack.push((new_i, new_j));
                                visited.push((new_i, new_j));
                                count += 1;
                            }
                        }
                    }
                }
                res.push(count);
            }
        })
    });
    res.sort();
    res
}

fn main() -> std::io::Result<()> {
    let mut river: River = Vec::new();

    let args = env::args().collect::<Vec<String>>();
    let fs = std::fs::OpenOptions::new()
        .read(true)
        .open(args.get(1).map_or("data.txt", |f| &*f))?;
    let f = std::io::BufReader::new(fs);
    f.lines().for_each(|line| match line {
        Ok(value) => {
            let c = value
                .chars()
                .enumerate()
                .filter_map(|(key, f)| if key % 2 == 0 { Some(f) } else { None })
                .collect::<Vec<char>>();
            river.push(c);
        }
        Err(e) => println!("ERROR {}", e),
    });
    find_river(&mut river);
    Ok(())
}
