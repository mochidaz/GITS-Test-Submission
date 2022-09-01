
const OPEN_BRACKETS: [char; 3] = ['(', '{', '['];

const CLOSE_BRACKETS: [char; 3] = [')', '}', ']'];

fn check_parentheses(string: String) -> &'static str {
    let mut stack = Vec::<char>::new();
    for i in string.chars() {
        if OPEN_BRACKETS.contains(&i) {
            stack.push(i);
        }
        else if CLOSE_BRACKETS.contains(&i) {
            match stack.pop() {
                Some(v) => {
                    let close_pos = CLOSE_BRACKETS.iter().position(|&x| x == i).unwrap(); 
                    let open_pos = OPEN_BRACKETS.iter().position(|&x| x == v).unwrap();

                    if close_pos != open_pos {
                        return "NO";
                    }
                }

                None => {
                    return "NO";
                }
            }
        }
    }

    if stack.is_empty() {
        return "YES";
    }
    "NO"
}

fn main() {
    let string1 = String::from("{ [ ( ) ] }");
    let string2 = String::from("{ [ ( ] ) }");
    let string3 = String::from("{ ( ( [ ] ) [ ] ) [ ] }");

    println!("{}", check_parentheses(string1));
    println!("{}", check_parentheses(string2));
    println!("{}", check_parentheses(string3));
}
