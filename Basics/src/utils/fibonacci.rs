pub fn fib (num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    } else if num == 1 {
        return second;
    } else {
        for _ in 2..num {
            let temp = first + second;
            first = second;
            second = temp;
        }
    }
    return second;
}