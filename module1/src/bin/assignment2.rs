fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    
    let numbers: [i32; 10] = [5, 2, 9, 10, 4, 22, 3, 6, 7, 21];

    
    for i in 0..10 {
        let n = numbers[i];

        if n % 3 == 0 && n % 5 == 0 {
            println!("{n}: FizzBuzz");
        } else if n % 3 == 0 {
            println!("{n}: Fizz");
        } else if n % 5 == 0 {
            println!("{n}: Buzz");
        } else {
            if is_even(n) {
                println!("{n}: even");
            } else {
                println!("{n}: odd");
            }
        }
    }

    // while loop for sum
    let mut sum: i32 = 0;
    let mut index: i32 = 0;

    while index < 10 {
        sum = sum + numbers[index as usize];
        index = index + 1;
    }

    println!("Sum = {sum}");

    // loop to find largest number
    let mut largest = numbers[0];
    let mut j: i32 = 1;

    loop {
        if j >= 10 {
            break;
        }
        let value = numbers[j as usize];
        if value > largest {
            largest = value;
        }
        j = j + 1;
    }

    println!("Largest = {largest}");
}
