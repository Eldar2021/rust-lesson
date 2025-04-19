use std::io::{self, Write};

fn main() {
    let mut board = [[' '; 3]; 3];
    let mut current_player = 'X';

    loop {
        print_board(&board);
        let (row, col) = get_move(&current_player);

        if board[row][col] != ' ' {
            println!("Bu alan zaten dolu! Lütfen başka bir alan seçin.");
            continue;
        }

        board[row][col] = current_player;

        if has_won(&board, current_player) {
            print_board(&board);
            println!("{} kazandı!!!", current_player);
            break;
        }

        if is_board_full(&board) {
            print_board(&board);
            println!("Oyun berabere bitti!");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

fn print_board(board: &[[char; 3]; 3]) {
    for row in board {
        println!(" {} | {} | {} ", row[0], row[1], row[2]);
        if row != &board[2] {
            println!("-----------");
        }
    }
    println!();
}

fn get_move(player: &char) -> (usize, usize) {
    loop {
        println!(
            "{} oyuncusu sıra sende (satır ve sütun numarası girin, ör: 1 2):",
            player
        );
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Giriş okunamadı");

        let coords: Vec<&str> = input.trim().split_whitespace().collect();
        if coords.len() != 2 {
            println!("Lütfen iki sayı girin.");
            continue;
        }

        let row: usize = match coords[0].parse::<usize>() {
            Ok(num) if num > 0 && num <= 3 => num - 1,
            _ => {
                println!("Geçersiz satır numarası, 1-3 arasında bir sayı girin.");
                continue;
            }
        };

        let col: usize = match coords[1].parse::<usize>() {
            Ok(num) if num > 0 && num <= 3 => num - 1,
            _ => {
                println!("Geçersiz sütun numarası, 1-3 arasında bir sayı girin.");
                continue;
            }
        };

        return (row, col);
    }
}

fn has_won(board: &[[char; 3]; 3], player: char) -> bool {
    // Satırları kontrol et
    for row in board {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }

    // Sütunları kontrol et
    for col in 0..3 {
        if board[0][col] == player && board[1][col] == player && board[2][col] == player {
            return true;
        }
    }

    // Diyagonalleri kontrol et
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }

    false
}

fn is_board_full(board: &[[char; 3]; 3]) -> bool {
    board.iter().all(|row| row.iter().all(|&cell| cell != ' '))
}
