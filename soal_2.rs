fn reverse(c: &mut Vec<char>, start: usize, end: usize) -> &mut Vec<char> {

    if start >= end {
        return c;
    }

    let s = c[start];

    let e = c[end];

    c[start] = e;
    c[end] = s;

    reverse(c, start + 1, end - 1)
}

fn main() {
    let mut a = vec!['h', 'e', 'l', 'l', 'o'];
    let length = a.clone();

    println!("{:?}", reverse(&mut a, 0, length.len() - 1))
}
