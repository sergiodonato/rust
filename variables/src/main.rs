fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("----------------------");
    let x = 2.0; // f64
    println!("The value of x is: {x}");
    let y: f32 = 3.0; // f32
    println!("The value of x is: {y}");

    println!("----------------------");
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");
    

    println!("----------------------");
    let t = true;
    println!("The value of t is: {t}");

    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {f}");


    println!("----------------------");
    let c = 'z';
    println!("The value of c is: {c}");
    let z: char = 'ℤ'; // with explicit type annotation
    println!("The value of z is: {z}");
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");


    println!("----------------------");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");


    println!("----------------------");
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");


    println!("----------------------");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a_zero = a[0];
    println!("The value of a is: {0}", a_zero);

/*     let a = [3; 5];
    print!("The value of a random is: {0}", a[0]); */


    println!("----------------------");

}
