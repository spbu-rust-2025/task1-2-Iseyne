use std::io;

fn check_number(value: &str) -> bool {
    return value.trim().parse::<isize>().is_ok();
}

fn main() {
    let mut flag = 0;
    let mut sm: isize = 0;

    let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Reading error");

    while input != "-1\n" {
        if check_number(&input) == (true) {
            let num: isize = input.trim().parse().expect("");
            sm = sm + num;
        }
        else {
            flag = 1;
        }

        input = String::new();
	    io::stdin().read_line(&mut input).expect("Reading error");
    }

    if flag == 1 {
        println!("NaN");
    }
    else {
        println!("{}", sm);
    }
}
