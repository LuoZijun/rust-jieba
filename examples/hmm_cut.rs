extern crate jieba;

use std::time;


fn main() {
    let text = "小明硕士毕业于中国科学院计算所";

    let now = time::Instant::now();
    let words = jieba::hmm::cut(text);
    let elapsed = now.elapsed();

    println!("text: {:?}", text);
    println!("words: {:?}", words);

    let milliseconds = elapsed.as_secs() * 1_000 + (elapsed.subsec_nanos() / 1_000_000) as u64;
    println!("elapsed: {:?} ms", milliseconds);
}
