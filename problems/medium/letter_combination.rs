use std::collections::HashMap;

fn combine_vectors(a: Vec<char>, b: Vec<char>) -> Vec<String> {
    let mut c: Vec<String> = Vec::new();
    for i in 0..a.len() - 1 {
        for j in 0..b.len() - 1 {
            let str = String::from(a[i] + b[j]);
            c.push(str);
        }
    }
    c
}

fn main() {



}

fn generate_map() -> HashMap<char, Vec<char>> {
    let mut map: HashMap<char, Vec<char>> = HashMap::new();
    map.insert('2', vec!['a', 'b', 'c']);
    map.insert('3', vec!['d', 'e', 'f']);
    map.insert('4', vec!['g', 'h', 'i']);
    map.insert('5', vec!['j', 'k', 'l']);
    map.insert('6', vec!['m', 'n', 'o']);
    map.insert('7', vec!['p', 'q', 'r', 's']);
    map.insert('8', vec!['t', 'u', 'v']);
    map.insert('9', vec!['w', 'x', 'y', 'z']);
    map
}