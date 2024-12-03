fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const PRESENTS: [&str; 12] = [
        "partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut count: usize = 0;

    while count < DAYS.len() {
        println!("On the {} day of Christmas,", DAYS[count]);
        println!("My true love sent to me");

        let mut count_int: usize = count;

        while count_int > 0 {
            if count > 0 {
                println!("{},", PRESENTS[count_int]);
            }

            count_int -= 1;
        }

        if count == 0 {
            println!("A {}.", PRESENTS[count_int]);
            println!("");
        } else {
            println!("And a {}.", PRESENTS[count_int]);
            println!("");
        }

        count += 1;
    }
}
