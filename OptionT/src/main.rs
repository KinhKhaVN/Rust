fn get_input() -> Option<String> {
    Some("x26".to_string())
}

fn parse(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

fn perfect_square(n: i32) -> Option<i32> {

    (0..=n).find(|x| x * x == n)
}

fn compute_01() -> Option<i32> {
    let input = get_input()?;
    let number = parse(&input)?;
    perfect_square(number)
}

fn compute_02() -> Option<i32> {
    get_input()
        .and_then(|input| parse(&input))
        .and_then(perfect_square)
}

fn compute_03() -> Result<i32, String> {
    let input = get_input().ok_or("No input".to_string())?;
    let number = parse(&input).ok_or("Not a number".to_string())?;
    perfect_square(number).ok_or("Not a perfect square".to_string())
}

fn compute_04() -> Result<i32, String> {
    let input = get_input().ok_or("No input".to_string())?;

    let number = parse(&input).ok_or_else(|| {
        let mut msg = String::new();
        core::fmt::write(
            &mut msg,
            format_args!("{input} is not a number!")).unwrap();
        msg
    })?;
    perfect_square(number).ok_or("Not a perfect square".to_string())
}

fn main() {

    if let Some(r) = compute_01() {
        println!("{r} is a perfect square")
    }

    println!("-----compute_02-----");
    match compute_02() {
        Some(r) => println!("{r} is a perfect square"),
        None         => println!("Error!")
    }

    println!("-----compute_03-----");
    match compute_03() {
        Ok(r) => println!("{r}"),
        Err(e)         => println!("{e}")
    }

    println!("-----compute_04-----");
    match compute_04() {
        Ok(r) => println!("{r}"),
        Err(e)         => println!("{e}")
    }
}
