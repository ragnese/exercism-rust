pub fn raindrops(n: u32) -> String {
    let div3 = n % 3 == 0;
    let div5 = n % 5 == 0;
    let div7 = n % 7 == 0;

    if !(div3 || div5 || div7) {
        return format!("{}", n);
    }

    let mut result = "".to_string();
    if div3 {
        result = result + "Pling";
    }
    if div5 {
        result = result + "Plang";
    }
    if div7 {
        result = result + "Plong";
    }

    result
}
