pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    spell_number(n)
}

fn spell_number(n: u64) -> String {
    // Handle numbers up to one million
    if n >= 1_000_000 {
        let millions = n / 1_000_000;
        let remainder = n % 1_000_000;
        
        if remainder == 0 {
            return format!("{} million", spell_number(millions));
        } else {
            return format!("{} million {}", spell_number(millions), spell_number(remainder));
        }
    }
    
    // Handle thousands
    if n >= 1_000 {
        let thousands = n / 1_000;
        let remainder = n % 1_000;
        
        if remainder == 0 {
            return format!("{} thousand", spell_number(thousands));
        } else {
            return format!("{} thousand {}", spell_number(thousands), spell_number(remainder));
        }
    }
    
    // Handle hundreds
    if n >= 100 {
        let hundreds = n / 100;
        let remainder = n % 100;
        
        if remainder == 0 {
            return format!("{} hundred", spell_number(hundreds));
        } else {
            return format!("{} hundred {}", spell_number(hundreds), spell_number(remainder));
        }
    }
    
    // Handle tens
    if n >= 20 {
        let tens = n / 10;
        let remainder = n % 10;
        
        let tens_word = match tens {
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => unreachable!(),
        };
        
        if remainder == 0 {
            return tens_word.to_string();
        } else {
            return format!("{}-{}", tens_word, spell_number(remainder));
        }
    }
    
    // Handle teens
    if n >= 10 {
        return match n {
            10 => "ten".to_string(),
            11 => "eleven".to_string(),
            12 => "twelve".to_string(),
            13 => "thirteen".to_string(),
            14 => "fourteen".to_string(),
            15 => "fifteen".to_string(),
            16 => "sixteen".to_string(),
            17 => "seventeen".to_string(),
            18 => "eighteen".to_string(),
            19 => "nineteen".to_string(),
            _ => unreachable!(),
        };
    }
    
    // Handle single digits
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => unreachable!(),
    }
}