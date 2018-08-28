Rust Jieba
================

:Date: 08/28 2018


.. contents::



Build
-----------

.. code:: bash
    
    git clone https://github.com/LuoZijun/rust-jieba.git
    cd rust-jieba

    cargo run --bin mk_hmm_dict > src/hmm_dict.rs
    cargo build

    cargo run --example hmm_cut

    cargo test
    cargo bench

