use rand::Rng;
//use rand::distrubutions::uniform::sampleRange;

// fn main() {
// println!("Hello, world!");
/*fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i32> = (0..10).map(|_| rng.gen_range(1, 100)).collect();
    numbers.sort();
    println!("Sorted numbers: {:?}", numbers);

}*/
fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(1..10); // 生成1到9之间的随机整数
    println!("Random number: {}", x);
}