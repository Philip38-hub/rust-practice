pub fn get_diamond(c: char) -> Vec<String> {
    // Check if the input is a valid uppercase letter
    if !c.is_ascii_uppercase() {
        panic!("Input must be an uppercase ASCII letter");
    }
    
    // Special case for 'A'
    if c == 'A' {
        return vec!["A".to_string()];
    }
    
    // Calculate the size of the diamond
    let size = (c as u8 - 'A' as u8 + 1) as usize;
    let width = 2 * size - 1;
    
    // Create a vector to hold the diamond rows
    let mut diamond = Vec::with_capacity(2 * size - 1);
    
    // Generate the top half of the diamond (including the middle row)
    for i in 0..size {
        let current_char = (('A' as u8) + i as u8) as char;
        let mut row = vec![' '; width];
        
        if current_char == 'A' {
            // For 'A', we only have one letter in the middle
            row[size - 1] = 'A';
        } else {
            // For other letters, we have two of the same letter
            let outer_spaces = size - 1 - i;
            let inner_spaces = width - 2 * (outer_spaces + 1);
            
            row[outer_spaces] = current_char;
            row[width - outer_spaces - 1] = current_char;
        }
        
        diamond.push(row.iter().collect::<String>());
    }
    
    // Generate the bottom half of the diamond (excluding the middle row)
    for i in (0..size-1).rev() {
        diamond.push(diamond[i].clone());
    }
    
    diamond
}