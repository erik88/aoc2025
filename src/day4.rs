use crate::board::Board;

pub fn day4(lines: Vec<String>) {
    let mut board = Board::new(&lines);

    println!("Part 1: {}", p1(&board));
    println!("Part 2: {}", p2(&mut board));
}

fn p1(board: &Board) -> u64 {
    let mut sum = 0;
    for x in 0..board.width as i64 {
        for y in 0..board.height as i64 {
            if board.get(x, y) == Some('@') && is_movable(&board, x, y) {
                sum += 1;
            }
        }
    }
    sum
}

fn p2(board: &mut Board) -> u64 {
    let mut sum = 0;
    loop {
        let new_sum = sweep(board);
        if new_sum == 0 {
            break;
        }
        sum += new_sum;
    }
    sum
}

fn sweep(board: &mut Board) -> u64 {
    let mut sum = 0;
    for x in 0..board.width as i64 {
        for y in 0..board.height as i64 {
            if board.get(x, y) == Some('@') && is_movable(&board, x, y) {
                sum += 1;
                board.set(x, y, '.');
            }
        }
    }
    sum
}

fn is_movable(board: &Board, xmid: i64, ymid: i64) -> bool {
    let mut count = 0;
    for x in xmid-1..=xmid+1 {
        for y in ymid-1..=ymid+1 {
            if x == xmid && y == ymid {
                continue;
            }
            if board.get(x, y) == Some('@') {
                count += 1;
            }
        }
    }
    return count < 4;
}
