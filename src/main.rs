pub enum Bracket {
    Open(char),
    Closed(char),
}

impl Bracket {
    fn to_char(c: char) -> Option<Bracket> {
        match c {
            '{' | '[' | '(' => Some(Bracket::Open(c)),
            '}' => Some(Bracket::Closed('{')),
            ']' => Some(Bracket::Closed('[')),
            ')' => Some(Bracket::Closed('(')),
            _ => None,
        }
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for char in string.chars() {
        match Bracket::to_char(char) {
            Some(Bracket::Open(open_char)) => stack.push(open_char),
            Some(Bracket::Closed(closing_char)) => {
                if stack.pop() != Some(closing_char) {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

fn main() {
    brackets_are_balanced("{[()]}");
}
