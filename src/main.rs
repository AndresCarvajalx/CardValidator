use std::io;

struct Card {
    number: i64,
    is_valid: bool,
}

fn numberToVec(n: i64) -> Vec<i64> {
    let mut digits: Vec<i64> = Vec::new();
    let mut n: i64 = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

fn validator(number: i64, valid: bool) -> (i64, bool) {
    let mut valid: bool = valid;
    let mut numberVec: Vec<i64> = numberToVec(number);
    for i in 0..numberVec.len() - 1 {
        if i % 2 == 0 {
            numberVec[i] = numberVec[i] * 2;
        }
    }

    for j in 0..numberVec.len() - 1 {
        let two_digits: Vec<i64> = numberToVec(numberVec[j]);
        if two_digits.len() == 2 {
            numberVec[j] = two_digits[0] + two_digits[1];
        }
    }

    let mut sum: i64 = 0;
    for f in 1..numberVec.len() {
        sum = sum + numberVec[f - 1];
    } sum = sum + numberVec.last().unwrap();

    if sum % 10 == 0 {
        valid = true;
        print!("Credit Card valid ");
        return (sum, valid);
    }
    print!("Credit Card not valid ");
    (0, valid)
}

fn main() {
    let mut number = String::new();
    println!("Enter your card number: ");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    number = number.split_whitespace().collect();

    let numberCard: Card = Card {
        number: number.parse().expect("Please type a number!"),
        is_valid: false,
    };
    println!("{:?}", validator(numberCard.number, numberCard.is_valid));
}