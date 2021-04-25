pub fn clamp(num: i32, min: i32, max: i32) -> i32 {
    if num < min {
        return min;
    } else if num > max {
        return max;
    }

    num
}
