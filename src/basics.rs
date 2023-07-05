
fn multiply( value: i32, multiplier: i32) -> i32 {
    let mut result: i32 = 0;
    for n in 1..multiplier {
        result += value;
    }
    result
}

fn divide(value: i32, divider: i32) -> i32 {
    let mut stacks: Vec<i32> = vec![0; divider as usize];
    let mut counter = value;

    loop {
        for n in 0..divider {
            let index: usize = n as usize;
            stacks[index] += 1;
        }
        counter -= divider;
        if counter < 1 {
            break;
        }
    }
    stacks[0]
}

fn main() {
    let four = multiply(2, 2);
}
