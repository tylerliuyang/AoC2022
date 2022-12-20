pub fn main() {
    for i in 0..1000 {
        for j in 0..1000 {
            for k in 2..1000 {
                let a = (i + j) % (k - 1);
                let b = (((i + j) % k) + (i + j) / (k - 1)) % k;

                if a != b {
                    println!("{} {}", a, b);
                }
            }
        }
    }
}
