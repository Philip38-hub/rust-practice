pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Check main diagonal (top-left to bottom-right)
    let main_diagonal = table[0][0] == player && table[1][1] == player && table[2][2] == player;
    
    // Check other diagonal (top-right to bottom-left)
    let other_diagonal = table[0][2] == player && table[1][1] == player && table[2][0] == player;
    
    main_diagonal || other_diagonal
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    // Check each row
    for row in 0..3 {
        if table[row][0] == player && table[row][1] == player && table[row][2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    // Check each column
    for col in 0..3 {
        if table[0][col] == player && table[1][col] == player && table[2][col] == player {
            return true;
        }
    }
    false
}

pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    // Check if player X won
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return "player X won".to_string();
    }
    
    // Check if player O won
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return "player O won".to_string();
    }
    
    // If no one won, it's a tie
    "tie".to_string()
}