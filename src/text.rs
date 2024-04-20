pub fn generate_lines(chars: String) -> Vec<String> {
    let mut lines = Vec::new();
    for line_number in 0..5 {
        let mut line = Vec::new();
        for char in chars.chars() {
            line.push(get_line_part(char, line_number));
            line.push(" ".to_string());
        }

        lines.push(line.join(""));
    }

    lines
}

fn get_line_part(c: char, line_number: i32) -> String {
    let result = match c {
        '0' => match line_number {
            0 => "██████",
            1 => "██  ██",
            2 => "██  ██",
            3 => "██  ██",
            4 => "██████",
            _ => "",
        },
        '1' => match line_number {
            0 => "    ██",
            1 => "    ██",
            2 => "    ██",
            3 => "    ██",
            4 => "    ██",
            _ => "",
        },
        '2' => match line_number {
            0 => "██████",
            1 => "    ██",
            2 => "██████",
            3 => "██    ",
            4 => "██████",
            _ => "",
        },
        '3' => match line_number {
            0 => "██████",
            1 => "    ██",
            2 => "██████",
            3 => "    ██",
            4 => "██████",
            _ => "",
        },
        '4' => match line_number {
            0 => "██  ██",
            1 => "██  ██",
            2 => "██████",
            3 => "    ██",
            4 => "    ██",
            _ => "",
        },
        '5' => match line_number {
            0 => "██████",
            1 => "██    ",
            2 => "██████",
            3 => "    ██",
            4 => "██████",
            _ => "",
        },
        '6' => match line_number {
            0 => "██████",
            1 => "██    ",
            2 => "██████",
            3 => "██  ██",
            4 => "██████",
            _ => "",
        },
        '7' => match line_number {
            0 => "██████",
            1 => "    ██",
            2 => "    ██",
            3 => "    ██",
            4 => "    ██",
            _ => "",
        },
        '8' => match line_number {
            0 => "██████",
            1 => "██  ██",
            2 => "██████",
            3 => "██  ██",
            4 => "██████",
            _ => "",
        },
        '9' => match line_number {
            0 => "██████",
            1 => "██  ██",
            2 => "██████",
            3 => "    ██",
            4 => "██████",
            _ => "",
        },
        ':' => match line_number {
            0 => "      ",
            1 => "  ██  ",
            2 => "      ",
            3 => "  ██  ",
            4 => "      ",
            _ => "",
        },
        '.' => match line_number {
            0 => "      ",
            1 => "      ",
            2 => "      ",
            3 => "      ",
            4 => "  ██  ",
            _ => "",
        },
        _ => panic!("Invalid character {}", c),
    };
    result.to_string()
}
