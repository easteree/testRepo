//use rand::Rng;
//use rand::distrubutions::uniform::sampleRange;

// fn main() {
// println!("Hello, world!");
// }

// fn main() {
//     let mut rng = rand::thread_rng();
//     let mut numbers: Vec<i32> = (0..10).map(|_| rng.gen_range(1, 100)).collect();
//     numbers.sort();
//     println!("Sorted numbers: {:?}", numbers);

// }




// fn main() {
//     let mut rng = rand::thread_rng();
//     let x: i32 = rng.gen_range(1..10); // 生成1到9之间的随机整数
//     println!("Random number: {}", x);   
// }


// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let mut s2 = String::from("hello");     // s2 comes into scope

//     //let s3 = takes_and_gives_back(&mut s2);  // s2 is moved into
//     takes_and_gives_back(&mut s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
//     //s3 = String::from("oh");
//     //println!("s1: {}, s3: {}", s1, s3);
//     println!("s1: {}, s2: {}", s1, s2);
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // This function takes a String and returns one
// //fn takes_and_gives_back(a_string: &mut String) -> String { // a_string comes into
// fn takes_and_gives_back(a_string: &mut String){ // a_string comes into
//                                                       // scope
//     //let mut s3 = String::from("ha!"); // s3 comes into scope
//     let s3 ="ha!"; // s3 comes into scope
//     a_string.push_str(&s3); // a_string is a mutable reference
//     //a_string  // a_string is returned and moves out to the calling function
// }



fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}