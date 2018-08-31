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


分词速度
----------

:处理器: Intel(R) Core(TM) i7-4770HQ @2.2 GHz
:样本: 钱钟书 - 《围城》
:吞吐量: ~= 4MB/s


参考资料
----------

`国家语委现代汉语语料库 <http://www.cncorpus.org/index.aspx>`_
`互联网上开放的中文语料库有哪些 <https://www.zhihu.com/question/21177095>`_
`搜狗实验室_语料数据 <https://www.sogou.com/labs/resource/list_yuliao.php>`_
