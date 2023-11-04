fn main() {
    println!("Hello, world!");

    basic_types_and_loops();
    println!();
    ownership();
    println!();
    slices();
    println!();

    let s: String = String::from("Bad is a test!");
    println!("First word: {}", first_word(&s));

    println!();
    structs();
}

fn basic_types_and_loops() {
    let x: char = 'A';
    println!("This is a char: {x}");

    let y: f64 = 10.2002;
    println!("This is a double: {y}");

    let tup: (i32, f64, char) = (500, 8.2, 'A');
    let (x, _y, _z) = tup;

    println!("This is the first val of a tuple: {x}");
    println!("This is the second val of a tuple: {}", tup.1);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("This is an array: {}", a[0]);

    let test: bool = false;
    if test {
        println!("True!");
    } else {
        println!("False!");
    }

    for item in a {
        print!("{item}");
    }
    println!();

    let mut i = 0;
    loop {
        i += 1;
        if i == 5 {
            println!("i = {i}");
            break
        }
    }
}

fn ownership() {
    // Creation and Copying
    // ---------------------------------------------------

    // Create a string variable
    let mut s: String = String::from("turbo");
    s.push_str(" boost!");
    println!("{s}");

    let s1 = String::from("test");
    // s1 gets moved, making s1 invalid
    // s2 takes ownership of s1 essentially, the data on the heap is not copied
    // The same functionality happens when passing s1 to a function, unless its a reference for borrowing purposes
    let s2 = s1;
    // Rust will never perform a deep copy unless requested using clone()
    let s3: String = s2.clone();

    // println!("{s1}") // COMPILE ERROR
    println!("{s2}");
    println!("{s3}");


    // References
    // ---------------------------------------------------

    let mut mut_s: String = String::from("wee");
    let sr: &String = &mut_s;
    // A mutable reference cannot exist while another reference (mutable or immutable) exists to our object
    // let mut_sr: &mut String = &mut mut_s;

    println!("{sr}");

    // References lifetime exists until they stop being used, so now we can make a mutable reference and use it:
    let mut_sr: &mut String = &mut mut_s;

    mut_sr.push_str("-blah");
    println!("{mut_sr}");
}

fn slices() {
    let s: String = String::from("This is a test");
    let slice1: &str = &s[0..5];
    let slice2: &str = &s[..5];
    let slice3: &str = &s[5..s.len()];
    let slice4: &str = &s[5..];

    println!("1: {slice1} | 2: {slice2} | 3: {slice3} | 4: {slice4}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slicea: &[i32] = &a[2..4];
    println!("Array Slice: {:?}", slicea);
}

// Write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }

    return &string[..];
}

// NOTE: Structs can hold references, but it requires the use of lifetimes and isn't covered yet.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /* If the function doesn't take self then it's like a static function in C++. Usually used to implement factory / constructor functions. */
    /* Self, is just an alias for Rectangle in it's impl block. */
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn structs() {
    let mut user1: User = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("test@test.com");

    println!("Active: {}, Username: {}, Email: {}, Count: {}", user1.active, user1.username, user1.email, user1.sign_in_count);

    let user2: User = User {
        email: String::from("Another@another.com"),
        ..user1 // Take all remaining fields from user1 (this would invalidate user1 because the Strings are moved)
    };

    println!("Email 2: {}", user2.email);

    // Debug printing structs
    let scale: u32 = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    println!("width: {}, height: {}", rect1.width, rect1.height);
    dbg!(&rect1);
    println!("println struct: {:#?}", rect1);

    println!("Area: {}", rect1.area());

    let r: Rectangle = Rectangle::square(3);
    println!("Square: {:?}", r);
}