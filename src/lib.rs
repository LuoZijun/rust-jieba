#![feature(try_from, test)]
#![allow(non_snake_case)]

extern crate test;

pub mod hmm;
pub mod word_class;

use std::hash::Hasher;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;


#[derive(Debug)]
pub enum CutOption {
    // 分词模式
    MIX,                // 精确模式     (HiddenMarkovModel + MaxProbability)
    SEARCH,             // 搜索引擎模式  (先使用 `MIX` 方法切词, 对于切出来的较长的词再使用 `FULL` 方法)
    FULL,               // 枚举模式     切出所有字典里的词语 ( MaxProbability + HiddenMarkovModel + SEARCH )
    // 数据模型
    HiddenMarkovModel,  // 隐式马尔科夫模型(Hidden Markov Model)
    MaxProbability,     // 最大概率法(Max Probability)
}

#[allow(unused_imports, unused_variables)]
pub fn cut(text: &str, opt: CutOption) {
    unimplemented!()
}


/*
散列算法:

    *   `MurmurHash3 <https://github.com/mhallin/murmurhash3-rs>`_
    *   `Jenkins hash <https://github.com/torvalds/linux/blob/master/include/linux/jhash.h>`_

*/
#[allow(unused_imports, unused_variables)]
pub fn minhash(s: &str) -> u64 {
    unimplemented!()
}

// Google 去重算法
// Github: https://github.com/yanyiwu/simhash
//         https://yanyiwu.com/work/2014/01/30/simhash-shi-xian-xiang-jie.html
pub fn simhash(text: &str) -> u64 {
    let mut v = [0i32; 64];
    let mut number: u64 = 0;

    for block in text.split_whitespace() {
        let mut hasher = DefaultHasher::new();
        hasher.write(&block.as_bytes());
        let feature_hash: u64 = hasher.finish();

        for i in 0..64 {
            let bit = (feature_hash >> i) & 1;
            if bit == 1 {
                v[i] = v[i].saturating_add(1);
            } else {
                v[i] = v[i].saturating_sub(1);
            }
        }
    }

    for q in 0..64 {
        if v[q] > 0 {
            number |= 1 << q;
        }
    }

    number
}

pub fn similarity(text1: &str, text2: &str) -> f64 {
    let a: u64 = simhash(text1);
    let b: u64 = simhash(text2);

    // Bitwise hamming distance of two `u64` hashes
    let hamming_distance: f64 = (a ^ b).count_ones() as f64;
    let n = 1.0 - (hamming_distance / 64.0);

    n
}

pub fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
    let inter = a.intersection(&b).count();
    let union = a.union(&b).count();
    (inter as f64) / (union as f64)
}

