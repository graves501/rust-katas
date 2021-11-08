fn main() {
    println!("{}", even_or_odd(2));
    println!("{}", litres(3.0));
    println!("{}", reverse("world"));
    println!("{}", remove_char("atta"));
    println!("{}", find_smallest_int(&[42,120,138]));

}

fn even_or_odd(i: i32) -> &'static str {
    if i % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

fn litres(time: f64) -> i32 {
    (time * 0.5).floor() as i32
}

fn reverse(phrase: &str) -> String {
    phrase.chars().rev().collect::<String>()
}

pub fn remove_char(s: &str) -> String {
    let mut s = s.to_string();
    s.remove(0);
    s.remove(s.chars().count()-1);
    s
}

fn find_smallest_int(arr: &[i32]) -> i32 {
    let smallest_int_vec = arr.to_vec();
    let min_value = smallest_int_vec.iter().min();
    match min_value {
        Some(min) => *min,
        None      => -1
    }
}
