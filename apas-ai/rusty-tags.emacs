
/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPerChap19.rs,3517
pub mod ArraySeqPerChap19 {ArraySeqPerChap193,44
pub trait ArraySeqPerChap19Trait {ArraySeqPerChap19Trait10,218
    fn tabulate<T: MtT>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T>;tabulate12,335
    fn map<T: MtT, U: MtT>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U>;map14,478
    fn select<'a, T: MtT>(a: &'a ArrayPerS<T>, b: &'a ArrayPerS<T>, i: N) -> Option<&'a T>;select16,598
    fn append<T: MtT + Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;append18,739
    fn append2<T: MtT + Clone>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;append220,876
    fn deflate<T: MtT + Clone + Eq>(f: impl Fn(&T) -> B, x: &T) -> ArrayPerS<T>;deflate22,997
    fn filter<T: MtT + Clone + Eq>(a: &ArrayPerS<T>, f: impl Fn(&T) -> B) -> ArrayPerS<T>;filter24,1161
    fn iterate<T: MtT + Clone + Eq, A: StT>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> iterate28,1420
    fn reduce<T: MtT + Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> Treduce30,1562
    fn scan<T: MtT + Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> (ArrayPerS<T>, T)scan34,1718
    fn flatten<T: MtT + Clone + Eq>(s: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T>;flatten38,1905
    fn inject<T: MtT + Clone + Eq>(values: &ArrayPerS<T>, changes: &ArrayPerS<Pair<N, T>>) -> Arinject42,2174
    fn atomicWrite<T: MtT + Clone + Eq>(atomicWrite44,2319
    fn inject_parallel2<T: MtT + Clone + Eq + Send + Sync>(inject_parallel250,2555
    fn AtomicWriteLowestChangeNumberWins<T: MtT + Clone + Eq + Send>(AtomicWriteLowestChangeNumberWins54,2710
    fn ninject_parallel2<T: MtT + Clone + Eq + Send + Sync>(ninject_parallel260,2978
    fn AtomicWriteHighestChangeNumberWins<T: MtT + Clone + Eq + Send>(AtomicWriteHighestChangeNumberWins64,3134
impl ArraySeqPerChap19Trait<T> for ArrayPerS<T> {ArrayPerS71,3347
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T> {tabulate72,3397
    fn map<U: MtT>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U> {map76,3550
    fn select<'a, T: MtT>(a: &'a ArrayPerS<T>, b: &'a ArrayPerS<T>, i: N) -> Option<&'a T> {select79,3723
    fn append<T: MtT + Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {append91,4059
    fn append2<T: MtT + Clone>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {append297,4320
    fn deflate<T: MtT + Clone + Eq>(f: impl Fn(&T) -> B, x: &T) -> ArrayPerS<T> {deflate103,4577
    fn filter<T: MtT + Clone + Eq>(a: &ArrayPerS<T>, f: impl Fn(&T) -> B) -> ArrayPerS<T> {filter107,4804
    fn iterate<T: MtT + Clone + Eq, A: StT>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> iterate115,5198
    fn reduce<T: MtT + Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> Treduce129,5701
    fn scan<T: MtT + Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> (ArrayPerS<T>, T)scan153,6447
    fn flatten<T: MtT + Clone + Eq>(s: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T> {flatten202,8112
    fn inject<T: MtT + Clone + Eq>(values: &ArrayPerS<T>, changes: &ArrayPerS<Pair<N, T>>) -> Arinject205,8265
    fn atomicWrite<T: MtT + Clone + Eq>(atomicWrite217,8830
    fn inject_parallel2<T: MtT + Clone + Eq + Send + Sync>(inject_parallel2232,9432
    fn AtomicWriteLowestChangeNumberWins<T: MtT + Clone + Eq + Send>(AtomicWriteLowestChangeNumberWins259,10375
    fn ninject_parallel2<T: MtT + Clone + Eq + Send + Sync>(ninject_parallel2275,10961
    fn AtomicWriteHighestChangeNumberWins<T: MtT + Clone + Eq + Send>(AtomicWriteHighestChangeNumberWins302,11906

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqEphChap19.rs,1449
pub mod AVLTreeSeqEphChap19 {AVLTreeSeqEphChap193,52
pub trait AVLTreeSeqEphChap19Trait {AVLTreeSeqEphChap19Trait9,245
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T>tabulate10,282
    fn map<T, U>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U>map13,399
    fn select<T>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>, i: N) -> Option<T>select17,573
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> append20,706
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqEphS<T>deflate21,821
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> B) -> AVfilter22,919
impl<T: Ord + Copy + Debug + Display> AVLTreeSeqEphChap19Trait for AVLTreeSeqEphS<T> {AVLTreeSeqEphS25,1035
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T>tabulate26,1122
    fn map<T, U>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U>map32,1326
    fn select<T>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>, i: N) -> Option<T>select39,1582
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> append56,2209
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqEphS<T>deflate59,2404
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> B) -> AVfilter66,2703

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPer.rs,2602
pub mod LinkedListPer {LinkedListPer3,62
pub struct NodeP<T: StT> {NodeP7,132
    pub value: T,value8,159
    pub next: Option<Box<NodeP<T>>>,next9,177
pub struct LinkedListPerS<T: StT> {LinkedListPerS13,234
    head: Option<Box<NodeP<T>>>,head14,270
    len: N,len15,303
pub trait LinkedListPerTrait<T: StT> {LinkedListPerTrait18,318
    fn empty() -> Self;empty19,357
    fn new(length: N, init_value: T) -> Self;new20,381
    fn length(&self) -> N;length21,427
    fn nth(&self, index: N) -> &T;nth22,454
    fn isEmpty(&self) -> B;isEmpty23,489
    fn isSingleton(&self) -> B;isSingleton24,517
    fn singleton(item: T) -> Self;singleton25,549
    fn set(&self, index: N, item: T) -> Result<Self, &'static str>set29,779
    fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy32,877
impl<T: StT> LinkedListPerS<T> {LinkedListPerS35,936
    fn push_front_node(&mut self, node: Box<NodeP<T>>) {push_front_node36,969
    pub fn from_vec(v: Vec<T>) -> Self {from_vec43,1146
    pub fn iter<'a>(&'a self) -> LinkedListPerIter<'a, T> {iter51,1381
impl<T: StT> LinkedListPerTrait<T> for LinkedListPerS<T> {LinkedListPerS56,1509
    fn empty() -> Self {empty57,1568
    fn new(length: N, init_value: T) -> Selfnew60,1645
    fn length(&self) -> N {length80,2252
    fn nth(&self, index: N) -> &T {nth83,2303
    fn isEmpty(&self) -> B {isEmpty95,2622
    fn isSingleton(&self) -> B {isSingleton102,2752
    fn singleton(item: T) -> Self {singleton109,2886
    fn set(&self, index: N, item: T) -> Result<Self, &'static str>set118,3097
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy151,4099
impl<T: StT + std::fmt::Debug> std::fmt::Debug for LinkedListPerS<T> {LinkedListPerS187,5104
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt188,5175
pub struct LinkedListPerIter<'a, T: StT> {LinkedListPerIter199,5501
    cursor: Option<&'a NodeP<T>>,cursor200,5544
impl<'a, T: StT> Iterator for LinkedListPerIter<'a, T> {LinkedListPerIter203,5581
    type Item = &'a T;Item204,5638
    fn next(&mut self) -> Option<Self::Item> {next205,5661
impl<T: StT + PartialEq> PartialEq for LinkedListPerS<T> {LinkedListPerS213,5852
    fn eq(&self, other: &Self) -> bool {eq214,5911
impl<T: StT + Eq> Eq for LinkedListPerS<T> {}LinkedListPerS231,6336
impl<T: StT + std::fmt::Display> std::fmt::Display for LinkedListPerS<T> {LinkedListPerS233,6383
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt234,6458
macro_rules! LinkedListPer {LinkedListPer251,6866

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/MathSeq.rs,3979
pub mod MathSeq {MathSeq8,306
pub struct MathSeqS<T: StT> {MathSeqS17,564
    data: Vec<T>,data18,594
impl<T: StT> PartialEq for MathSeqS<T> {MathSeqS21,615
    fn eq(&self, other: &Self) -> bool {eq22,656
impl<T: StT> Eq for MathSeqS<T> {}MathSeqS27,738
impl<T: StT> std::fmt::Debug for MathSeqS<T> {MathSeqS29,774
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt30,821
impl<T: StT> std::fmt::Display for MathSeqS<T> {MathSeqS35,961
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt36,1010
impl<T: StT> MathSeqS<T> {MathSeqS47,1311
    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter48,1338
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }iter_mut49,1409
    pub fn empty() -> Self { Self { data: Vec::new() } }empty51,1496
    pub fn singleton(item: T) -> Self { Self { data: vec![item] } }singleton52,1553
    pub fn from_vec(data: Vec<T>) -> Self { Self { data } }from_vec53,1621
    pub fn with_len(length: N, init_value: T) -> Self { Self { data: vec![init_value; length] } with_len54,1681
impl<'a, T: StT> IntoIterator for &'a MathSeqS<T> {MathSeqS57,1782
    type Item = &'a T;Item58,1834
    type IntoIter = std::slice::Iter<'a, T>;IntoIter59,1857
    fn into_iter(self) -> Self::IntoIter { self.data.iter() }into_iter60,1902
impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {MathSeqS63,1967
    type Item = &'a mut T;Item64,2023
    type IntoIter = std::slice::IterMut<'a, T>;IntoIter65,2050
    fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }into_iter66,2098
impl<T: StT> IntoIterator for MathSeqS<T> {MathSeqS69,2167
    type Item = T;Item70,2211
    type IntoIter = std::vec::IntoIter<T>;IntoIter71,2230
    fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }into_iter72,2273
pub trait MathSeqTrait<T: StT> {MathSeqTrait76,2375
    fn new(length: N, init_value: T) -> Self;new80,2536
    fn empty() -> Self;empty85,2661
    fn singleton(item: T) -> Self;singleton90,2784
    fn length(&self) -> N;length94,2891
    fn nth(&self, index: N) -> &T;nth98,3018
    fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str>;set102,3141
    fn add_last(&mut self, value: T) -> &mut Self;add_last107,3492
    fn delete_last(&mut self) -> Option<T>;delete_last111,3618
    fn subseq(&self, start: N, length: N) -> &[T];subseq115,3770
    fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy120,3957
    fn isEmpty(&self) -> B;isEmpty125,4101
    fn isSingleton(&self) -> B;isSingleton130,4216
    fn domain(&self) -> Vec<N>;domain134,4326
    fn range(&self) -> Vec<T>range138,4494
    fn multiset_range(&self) -> Vec<(N, T)>multiset_range144,4683
impl<T: StT> MathSeqTrait<T> for MathSeqS<T> {MathSeqS149,4757
    fn new(length: N, init_value: T) -> Self { MathSeqS { data: vec![init_value; length] } }new151,4843
    fn empty() -> Self { MathSeqS { data: Vec::new() } }empty154,4971
    fn singleton(item: T) -> Self { MathSeqS { data: vec![item] } }singleton157,5063
    fn length(&self) -> N {length160,5166
    fn nth(&self, index: N) -> &T {nth165,5258
    fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str> {set170,5360
    fn add_last(&mut self, value: T) -> &mut Self {add_last182,5860
    fn delete_last(&mut self) -> Option<T> {delete_last188,5996
    fn subseq(&self, start: N, length: N) -> &[T] {subseq193,6105
    fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy201,6344
    fn isEmpty(&self) -> B {isEmpty214,6711
    fn isSingleton(&self) -> B {isSingleton223,6883
    fn domain(&self) -> Vec<N> {domain232,7070
    fn range(&self) -> Vec<T>range237,7194
    fn multiset_range(&self) -> Vec<(N, T)>multiset_range250,7573
macro_rules! MathSeq {MathSeq267,8120
fn _MathSeq_macro_type_checks() {_MathSeq_macro_type_checks274,8402

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPerChap18.rs,2522
pub mod ArraySeqPerChap18 {ArraySeqPerChap183,44
pub trait ArraySeqPerChap18Trait<T: MtT> {ArraySeqPerChap18Trait7,141
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T>;tabulate9,266
    fn map<U: MtT + Clone>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U>;map12,402
    fn append(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;append15,535
    fn filter(a: &ArrayPerS<T>, pred: impl Fn(&T) -> B) -> ArrayPerS<T>;filter18,696
    fn update(a: &ArrayPerS<T>, item_at: (N, T)) -> ArrayPerS<T>;update23,920
    fn inject(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T>;inject26,1060
    fn ninject(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T>;ninject29,1194
    fn iterate<A: MtT>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate32,1347
    fn iteratePrefixes<A: MtT + Clone>(iteratePrefixes35,1467
    fn reduce(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce42,1688
    fn scan(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayPerS<T>, T);scan45,1803
    fn flatten(ss: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T>;flatten48,1955
    fn collect(collect51,2093
impl<T: MtT> ArraySeqPerChap18Trait<T> for ArrayPerS<T> {ArrayPerS57,2225
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T> {tabulate58,2283
    fn map<U: MtT + Clone>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U> {map62,2442
    fn append(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {append74,2880
    fn filter(a: &ArrayPerS<T>, pred: impl Fn(&T) -> B) -> ArrayPerS<T> {filter88,3336
    fn update(a: &ArrayPerS<T>, (index, item): (N, T)) -> ArrayPerS<T> {update97,3629
    fn inject(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T> {inject103,3822
    fn ninject(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T> {ninject115,4322
    fn iterate<A: MtT>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate125,4708
    fn iteratePrefixes<A: MtT + Clone>(iteratePrefixes132,4910
    fn reduce(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce145,5302
        fn rec<T: StT + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec146,5374
    fn scan(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayPerS<T>, T) {scan161,5819
        fn rec<T: StT + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec162,5905
    fn flatten(ss: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T> {flatten181,6564
    fn collect(collect191,6884

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPerChap19.rs,2476
pub mod LinkedListPerChap19 {LinkedListPerChap193,46
pub trait LinkedListPerChap19Trait<T: StT> {LinkedListPerChap19Trait8,207
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T>;tabulate9,252
    fn map<U: StT>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U>;map10,316
    fn select<'a>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T>;select11,401
    fn append(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;append12,491
    fn append2(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;append213,573
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T>;deflate14,656
    fn filter(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T>;filter15,721
    fn iterate<A: StT>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,801
    fn reduce<F>(a: &LinkedListPerS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;reduce17,884
    fn scan<F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T) where F: Fn(&T, &Tscan18,969
    fn flatten(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T>;flatten19,1073
    fn inject(values: &LinkedListPerS<T>, changes: &LinkedListPerS<Pair<N, T>>,inject20,1149
impl<T: StT> LinkedListPerChap19Trait<T> for LinkedListPerS<T> {LinkedListPerS24,1260
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T> {tabulate25,1325
    fn map<U: StT>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U> {map28,1471
    fn select<'a>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T> {select31,1633
    fn append(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {append45,2205
    fn append2(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {append249,2368
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T> {deflate53,2532
    fn filter(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T> {filter61,2806
    fn iterate<A: StT>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate65,2967
    fn reduce<F>(a: &LinkedListPerS<T>, f: &F, id: T) -> Treduce69,3135
    fn scan<F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T)scan76,3322
    fn flatten(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T> {flatten83,3526
    fn inject(values: &LinkedListPerS<T>, changes: &LinkedListPerS<Pair<N, T>>,) inject87,3681

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPerChap18.rs,2846
pub mod LinkedListPerChap18 {LinkedListPerChap183,46
pub trait LinkedListPerChap18Trait<T: StT> {LinkedListPerChap18Trait7,149
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T>;tabulate9,269
    fn map<U: StT>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U>;map12,410
    fn append(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;append15,545
    fn filter(a: &LinkedListPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListPerS<T>;filter18,721
    fn update(a: &LinkedListPerS<T>, item_at: Pair<N, T>) -> LinkedListPerS<T>;update23,955
    fn inject( a: &LinkedListPerS<T>, updates: &LinkedListPerS<T>,) -> LinkedListPerS<T>;inject26,1109
    fn ninject(a: &LinkedListPerS<T>, updates: &LinkedListPerS<Pair<N, T>>,) -> LinkedListPerS<Tninject29,1255
    fn iterate<A: StT>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate32,1428
    fn iteratePrefixes<A: StT> (a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A,)iteratePrefixes35,1553
    fn reduce(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce39,1751
    fn scan(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T,) -> (LinkedListPerS<T>, T);scan42,1871
    fn flatten(ss: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T>;flatten45,2034
    fn collect<A: StT, Bv: StT> (a: &LinkedListPerS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O,) collect48,2187
impl<T: StT> LinkedListPerChap18Trait<T> for LinkedListPerS<T> {LinkedListPerS52,2338
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T> {tabulate53,2403
    fn map<U: StT>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U> {map61,2622
    fn append(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {append69,3035
    fn filter(a: &LinkedListPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListPerS<T> {filter81,3605
    fn update(a: &LinkedListPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListPerS<T> {update91,4021
    fn inject(a: &LinkedListPerS<T>, updates: &LinkedListPerS<Pair<N, T>>,) -> LinkedListPerS<T>inject100,4511
    fn ninject(a: &LinkedListPerS<T>, updates: &LinkedListPerS<Pair<N, T>>,) -> LinkedListPerS<Tninject121,5420
    fn iterate<A: StT>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate140,6297
    fn iteratePrefixes<A: StT>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A,) iteratePrefixes150,6643
    fn reduce(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce164,7217
    fn scan(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T,) -> (LinkedListPerS<T>, T) {scan179,7949
    fn flatten(ss: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T> {flatten197,8789
    fn collect<A: StT, Bv: StT>(a: &LinkedListPerS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O,) collect213,9460

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEph.rs,2359
pub mod ArraySeqEph {ArraySeqEph3,91
    pub struct ArraySeqEphS<T: StT> {ArraySeqEphS13,336
        pub data: Box<[T]>,data14,374
    pub trait ArraySeqEphTrait<T: StT> {ArraySeqEphTrait18,455
        fn new(length: N, init_value: T) -> Self;new20,537
        fn length(&self) -> N;length22,623
        fn nth(&self, index: N) -> &T;nth24,690
        fn empty() -> Self;empty26,765
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set28,829
        fn singleton(item: T) -> Self;singleton30,946
        fn isEmpty(&self) -> B;isEmpty32,1021
        fn isSingleton(&self) -> B;isSingleton34,1089
        fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy36,1172
    impl<T: StT> ArraySeqEphS<T> {ArraySeqEphS41,1277
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq42,1312
        pub fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy48,1539
        pub fn from_vec(v: Vec<T>) -> Self {from_vec67,2228
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqEphS<T> {update72,2368
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter79,2574
    impl<T: StT + Eq> PartialEq for ArraySeqEphS<T> {ArraySeqEphS82,2644
        fn eq(&self, other: &Self) -> bool {eq83,2698
    impl<T: StT + Eq> Eq for ArraySeqEphS<T> {}ArraySeqEphS96,3025
    impl<T: StT + Debug> Debug for ArraySeqEphS<T> {ArraySeqEphS98,3074
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt99,3127
    impl<T: StT + Display> Display for ArraySeqEphS<T> {ArraySeqEphS105,3320
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt106,3377
    impl<T: StT> ArraySeqEphTrait<T> for ArraySeqEphS<T> {ArraySeqEphS116,3660
        fn new(length: N, init_value: T) -> Selfnew117,3719
        fn length(&self) -> N {length123,3883
        fn nth(&self, index: N) -> &T {nth126,3953
        fn empty() -> Self {empty129,4033
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set132,4119
        fn singleton(item: T) -> Self {singleton140,4396
        fn isEmpty(&self) -> B {isEmpty143,4493
        fn isSingleton(&self) -> B {isSingleton150,4658
        fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy157,4827
macro_rules! ArraySeqEph {ArraySeqEph168,5015

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEphChap18.rs,2317
pub mod ArraySeqEphChap18 {ArraySeqEphChap183,49
    pub trait ArraySeqEphChap18Trait<T: StT> {ArraySeqEphChap18Trait7,154
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T>;tabulate8,201
        fn map<U: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U>;map9,267
        fn append(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;append10,352
        fn filter(a: &ArraySeqEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqEphS<T>;filter11,432
        fn update(a: &mut ArraySeqEphS<T>, item_at: (N, T)) -> &mut ArraySeqEphS<T>;update12,515
        fn inject(a: &ArraySeqEphS<T>, updates: &ArraySeqEphS<Pair<N, T>>) -> ArraySeqEphS<T>;inject13,600
        fn ninject(a: &ArraySeqEphS<T>, updates: &ArraySeqEphS<Pair<N, T>>) -> ArraySeqEphS<T>;ninject14,695
        fn iterate<A: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate15,791
        fn iteratePrefixes<A: StT>(iteratePrefixes16,876
        fn reduce(a: &ArraySeqEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce21,1035
        fn scan(scan22,1113
        fn flatten(ss: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T>;flatten27,1255
        fn collect<A: StT, Bv: StT>(collect28,1330
    impl<T: StT> ArraySeqEphChap18Trait<T> for ArraySeqEphS<T> {ArraySeqEphS34,1510
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T> {tabulate35,1575
        fn map<U: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U> {map42,1817
        fn append(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {append55,2339
        fn filter(a: &ArraySeqEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqEphS<T> {filter76,3079
        fn update(a: &mut ArraySeqEphS<T>, (index, item): (N, T)) -> &mut ArraySeqEphS<T> {update88,3498
        fn inject(inject91,3636
        fn ninject(ninject102,3991
        fn iterate<A: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate108,4170
        fn iteratePrefixes<A: StT>(iteratePrefixes115,4403
        fn reduce(a: &ArraySeqEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce132,4997
        fn scan(scan139,5224
        fn flatten(ss: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T> {flatten157,5812
        fn collect<A: StT, Bv: StT>(collect185,6884

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/UnDirGraphEphChap6_1.rs,2771
pub mod UnDirGraphEphChap6_1 {UnDirGraphEphChap6_13,80
pub struct UnDirGraphEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {UnDirGraphEph10,241
    V: Set<V>,V11,328
    E: Set<(V, V)>,E12,343
pub trait UnDirGraphEphChap6_1Trait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> UnDirGraphEphChap6_1Trait15,366
    fn empty() -> UnDirGraphEph<V>;empty16,464
    fn FromSets(V: Set<V>, E: Set<(V, V)>) -> UnDirGraphEph<V>;FromSets17,500
    fn vertices(&self) -> &Set<V>;vertices18,564
    fn edges(&self) -> &Set<(V, V)>;edges19,599
    fn sizeV(&self) -> N;sizeV20,636
    fn sizeE(&self) -> N;sizeE21,662
    fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor23,740
    fn NG(&self, v: &V) -> Set<V>;NG24,783
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices25,818
    fn Incident(&self, e: &(V, V), v: &V) -> B;Incident26,872
    fn Degree(&self, v: &V) -> N;Degree27,920
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> UnDirGraphEphChap6_1Trait<V> foUnDirGraphEph30,957
    fn empty() -> UnDirGraphEph<V> { UnDirGraphEph { V: SetLit![], E: SetLit![] } }empty31,1074
    fn FromSets(V: Set<V>, E: Set<(V, V)>) -> UnDirGraphEph<V> { UnDirGraphEph { V, E } }FromSets32,1158
    fn vertices(&self) -> &Set<V> { &self.V }vertices33,1248
    fn edges(&self) -> &Set<(V, V)> { &self.E }edges34,1294
    fn sizeV(&self) -> N { self.V.size() }sizeV35,1342
    fn sizeE(&self) -> N { self.E.size() }sizeE36,1385
    fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor38,1429
    fn NG(&self, v: &V) -> Set<V> {NG43,1661
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices52,1931
    fn Incident(&self, e: &(V, V), v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } else { B:Incident61,2171
    fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree63,2279
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Debug for UnDirGraphEUnDirGraphEph66,2337
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt67,2441
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for UnDirGrapUnDirGraphEph72,2612
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt73,2718
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for UnDirGraphEph<V> UnDirGraphEph78,2849
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for UnDirGraphEph<V> eq78,2849
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for UnDirGraphEph<V> {}UnDirGraphEph79,3027
macro_rules! UnDirGraphLit {UnDirGraphLit86,3197
pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise95,3721

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/MappingEphChap5_5.rs,2367
pub mod MappingEphChap5_5 {MappingEphChap5_53,72
pub struct Mapping<A, B> {Mapping12,315
    rel: Relation<A, B>,rel13,342
pub trait MappingEphChap5_5Trait<X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + SiMappingEphChap5_5Trait16,370
    fn empty() -> Mapping<X, Y>;empty17,541
    fn FromVec(v: Vec<(X, Y)>) -> Mapping<X, Y>;FromVec19,575
    fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation21,625
    fn size(&self) -> N;size23,684
    fn domain(&self) -> Set<X>;domain25,710
    fn range(&self) -> Set<Y>;range27,743
    fn mem(&self, a: &X, b: &Y) -> B;mem29,775
    fn iter(&self) -> std::collections::hash_set::Iter<'_, (X, Y)>;iter31,814
impl<A, B> Mapping<A, B> {Mapping34,885
    fn unique_pairs_from_iter<I>(iter: I) -> Set<(A, B)>unique_pairs_from_iter35,912
impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std:Mapping48,1262
    fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq49,1401
impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std:Mapping51,1468
impl<A: std::fmt::Debug + Eq + Hash, B: std::fmt::Debug + Eq + Hash> std::fmt::Debug for MappingMapping53,1602
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }fmt54,1707
impl<A: std::fmt::Display + Eq + Hash, B: std::fmt::Display + Eq + Hash> std::fmt::Display for MMapping56,1800
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }fmt57,1911
impl<X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized, Y: Eq + Hash + std::fmtMapping60,2005
    fn empty() -> Mapping<X, Y> {empty61,2195
    fn FromVec(v: Vec<(X, Y)>) -> Mapping<X, Y> {FromVec65,2320
    fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation70,2521
    fn size(&self) -> N { self.rel.size() }size75,2747
    fn domain(&self) -> Set<X> { self.rel.domain() }domain77,2792
    fn range(&self) -> Set<Y> { self.rel.range() }range79,2846
    fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem81,2898
    fn iter(&self) -> std::collections::hash_set::Iter<'_, (X, Y)> { self.rel.iter() }iter83,2959
macro_rules! MappingLit {MappingLit91,3120
pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise102,3663

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,1667
pub mod Types {Types4,45
    pub type N = usize;N9,156
    pub enum B {B13,299
        True,True14,316
        False,False15,330
    impl std::fmt::Display for B {B22,563
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt23,598
    pub trait StT: Eq + Clone + Display + Debug + Sized {}StT33,939
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}T34,998
    pub trait MtT: Sized + Send + Sync {}MtT38,1206
    impl<T> MtT for T where T: Sized + Send + Sync {}T39,1248
    pub struct Edge<V: StT + Sized + std::fmt::Display + std::fmt::Debug>(pub V, pub V);Edge43,1436
    impl<V: StT + Sized + std::fmt::Display + std::fmt::Debug> std::fmt::Display for Edge<V> {Edge45,1526
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt46,1621
    impl<V: StT + Sized + std::fmt::Display + std::fmt::Debug> From<(V, V)> for Edge<V> {Edge51,1765
        fn from(t: (V, V)) -> Self { Edge(t.0, t.1) }from52,1855
    impl<V: StT + Sized + std::fmt::Display + std::fmt::Debug> From<Edge<V>> for (V, V) {V55,1916
        fn from(e: Edge<V>) -> (V, V) { (e.0, e.1) }from56,2006
    pub struct Pair<A, B>(pub A, pub B);Pair61,2197
    impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {Pair63,2239
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt64,2327
    impl<A, B> From<(A, B)> for Pair<A, B> {Pair69,2471
        fn from(t: (A, B)) -> Self { Pair(t.0, t.1) }from70,2516
    impl<A, B> From<Pair<A, B>> for (A, B) {B73,2577
        fn from(p: Pair<A, B>) -> (A, B) { (p.0, p.1) }from74,2622

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/DirGraphEphChap6_1.rs,3690
pub mod DirGraphEphChap6_1 {DirGraphEphChap6_13,77
pub struct DirGraphEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {DirGraphEph10,236
    V: Set<V>,V11,321
    A: Set<(V, V)>,A12,336
pub trait DirGraphEphChap6_1Trait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {DirGraphEphChap6_1Trait15,359
    fn empty() -> DirGraphEph<V>;empty16,455
    fn FromSets(V: Set<V>, A: Set<(V, V)>) -> DirGraphEph<V>;FromSets17,489
    fn vertices(&self) -> &Set<V>;vertices18,551
    fn arcs(&self) -> &Set<(V, V)>;arcs19,586
    fn sizeV(&self) -> N;sizeV20,622
    fn sizeA(&self) -> N;sizeA21,648
    fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor23,713
    fn NG(&self, v: &V) -> Set<V>;            // Out-neighbors by conventionNG24,756
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices25,833
    fn NPlus(&self, v: &V) -> Set<V>;         // Out-neighborsNPlus26,887
    fn NMinus(&self, v: &V) -> Set<V>;        // In-neighborsNMinus27,950
    fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices28,1012
    fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices29,1069
    fn Incident(&self, e: &(V, V), v: &V) -> B;Incident30,1127
    fn Degree(&self, v: &V) -> N;             // Out-degree by conventionDegree31,1175
    fn InDegree(&self, v: &V) -> N;InDegree32,1249
    fn OutDegree(&self, v: &V) -> N;OutDegree33,1285
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> DirGraphEphChap6_1Trait<V> for DirGraphEph36,1325
    fn empty() -> DirGraphEph<V> { DirGraphEph { V: SetLit![], A: SetLit![] } }empty37,1438
    fn FromSets(V: Set<V>, A: Set<(V, V)>) -> DirGraphEph<V> { DirGraphEph { V, A } }FromSets38,1518
    fn vertices(&self) -> &Set<V> { &self.V }vertices39,1604
    fn arcs(&self) -> &Set<(V, V)> { &self.A }arcs40,1650
    fn sizeV(&self) -> N { self.V.size() }sizeV41,1697
    fn sizeA(&self) -> N { self.A.size() }sizeA42,1740
    fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor44,1784
    fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG49,2023
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices51,2076
    fn NPlus(&self, v: &V) -> Set<V> {NPlus60,2316
    fn NMinus(&self, v: &V) -> Set<V> {NMinus66,2510
    fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices72,2705
    fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices81,2955
    fn Incident(&self, e: &(V, V), v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } else { B:Incident90,3209
    fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree92,3317
    fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree93,3375
    fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree94,3436
impl<V: Eq + Hash + Clone + std::fmt::Debug + std::fmt::Display> std::fmt::Debug for DirGraphEphDirGraphEph97,3500
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt98,3602
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for DirGraphEDirGraphEph103,3771
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt104,3875
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for DirGraphEph<V> { DirGraphEph109,4006
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for DirGraphEph<V> { eq109,4006
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for DirGraphEph<V> {}DirGraphEph110,4182
macro_rules! DirGraphLit {DirGraphLit117,4346
pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise125,4854

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/SetEphChap5_1.rs,3245
pub mod SetEphChap5_1 {SetEphChap5_13,69
pub struct Set<T> { data: HashSet<T> }Set11,234
pub struct Set<T> { data: HashSet<T> }data11,234
pub trait SetEphChap5_1Trait<T: Eq + Hash + Clone + Display + Debug + Sized> {SetEphChap5_1Trait13,274
    fn empty() -> Set<T>;empty14,353
    fn singleton(x: T) -> Set<T>;singleton15,379
    fn size(&self) -> N;size16,413
    fn mem(&self, x: &T) -> B;mem17,438
    fn union(&self, other: &Set<T>) -> Set<T>union18,469
    fn intersection(&self, other: &Set<T>) -> Set<T>intersection21,542
    fn partition(&self, parts: &Set<Set<T>>) -> B;partition24,624
    fn CartesianProduct<U>(&self, other: &Set<U>) -> Set<(T, U)>CartesianProduct26,676
    fn insert(&mut self, x: T) -> &mut Self;insert31,796
    fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;iter33,842
    fn FromVec(v: Vec<T>) -> Set<T>FromVec34,905
impl<T: Eq + Hash> PartialEq for Set<T> {Set39,976
    fn eq(&self, other: &Self) -> bool { self.data == other.data }eq40,1018
impl<T: Eq + Hash> Eq for Set<T> {}Set43,1088
impl<T: Eq + Hash> std::fmt::Debug for Set<T>Set45,1125
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt49,1203
impl<T: Eq + Hash> std::fmt::Display for Set<T>Set54,1342
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt58,1424
impl<T: Eq + Hash > Hash for Set<T> {Set70,1814
    fn hash<H: Hasher>(&self, state: &mut H) {hash71,1852
impl<T: Eq + Hash> Set<T> {Set85,2336
    pub fn empty() -> Set<T> {empty86,2364
    pub fn singleton(x: T) -> Set<T> {singleton90,2439
    pub fn size(&self) -> N { self.data.len() }size96,2585
    pub fn mem(&self, x: &T) -> B {mem98,2634
    pub fn union(&self, other: &Set<T>) -> Set<T>union102,2740
    pub fn intersection(&self, other: &Set<T>) -> Set<T>intersection111,2952
    pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition120,3247
    pub fn CartesianProduct<U>(&self, other: &Set<U>) -> Set<(T, U)>CartesianProduct134,3662
    pub fn insert(&mut self, x: T) -> &mut Self {insert148,4040
    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter153,4147
    pub fn FromVec(v: Vec<T>) -> Set<T>FromVec155,4235
impl<T: Eq + Hash + Clone + Display + Debug + Sized> SetEphChap5_1Trait<T> for Set<T> {Set165,4443
    fn empty() -> Set<T> {empty166,4531
    fn singleton(x: T) -> Set<T> {singleton170,4602
    fn size(&self) -> N { self.data.len() }size176,4744
    fn mem(&self, x: &T) -> B {mem178,4789
    fn union(&self, other: &Set<T>) -> Set<T>union182,4891
    fn intersection(&self, other: &Set<T>) -> Set<T>intersection191,5099
    fn partition(&self, parts: &Set<Set<T>>) -> B {partition200,5390
    fn CartesianProduct<U>(&self, other: &Set<U>) -> Set<(T, U)>CartesianProduct214,5801
    fn insert(&mut self, x: T) -> &mut Self {insert228,6175
    fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter233,6278
    fn FromVec(v: Vec<T>) -> Set<T>FromVec235,6362
macro_rules! SetLit {SetLit251,6630
pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise264,6933
    let _s0: Set<&'static str> = SetLit![];str266,7023

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEphChap19.rs,2141
pub mod ArraySeqEphChap19 {ArraySeqEphChap193,49
pub trait ArraySeqEphChap19Trait<T: StT> {ArraySeqEphChap19Trait8,200
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T>;tabulate9,243
    fn map<U: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U>;map10,305
    fn select<'a>(a: &'a ArraySeqEphS<T>, b: &'a ArraySeqEphS<T>, i: N) -> Option<T>;select11,386
    fn append(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;append12,472
    fn append2(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;append213,548
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqEphS<T>;deflate14,625
    fn filter(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqEphS<T>;filter15,688
    fn iterate<A: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,764
    fn reduce<F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> Treduce17,845
    fn scan<F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> (ArraySeqEphS<T>, T)scan20,940
    fn flatten(s: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T>;flatten23,1052
impl<T: StT> ArraySeqEphChap19Trait<T> for ArraySeqEphS<T> {ArraySeqEphS26,1125
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T> {tabulate27,1186
    fn map<U: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U> {map30,1326
    fn select<'a>(a: &'a ArraySeqEphS<T>, b: &'a ArraySeqEphS<T>, i: N) -> Option<T> {select33,1480
    fn append(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {append45,1830
    fn append2(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {append248,1982
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqEphS<T> {deflate51,2135
    fn filter(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqEphS<T> {filter58,2398
    fn iterate<A: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate61,2550
    fn reduce<F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> Treduce64,2711
    fn scan<F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> (ArraySeqEphS<T>, T)scan70,2891
    fn flatten(s: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T> {flatten76,3086

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListEphChap19.rs,2729
pub mod LinkedListEphChap19 {LinkedListEphChap193,58
pub trait LinkedListEphChap19Trait {LinkedListEphChap19Trait8,219
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T>;tabulate9,256
    fn map< U: StT>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U>;map10,320
    fn select<'a, T: StT>(a: &'a LinkedListEphS<T>, b: &'a LinkedListEphS<T>, i: N) -> Option<T>select11,406
    fn append(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;append12,504
    fn append2(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;append213,586
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListEphS<T>;deflate14,669
    fn filter(a: &LinkedListEphS<T>, f: impl Fn(&T) -> B) -> LinkedListEphS<T>;filter15,734
    fn iterate<T: StT, A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,814
    fn reduce<T: StT, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;reduce17,905
    fn scan<T: StT, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> (LinkedListEphS<T>, T) where F: Fscan18,998
    fn flatten(s: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T>;flatten19,1110
    fn inject<T: StT>(values: &LinkedListEphS<T>, changes: &LinkedListEphS<Pair<N, T>>) -> Linkeinject20,1186
impl LinkedListEphChap19Trait<T> for LinkedListEphS<T> {LinkedListEphS23,1299
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T> { <LinkedListEphS<T> as LinkedListabulate24,1356
    fn map<U: StT>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U> { <LinkedLismap25,1490
    fn select<'a, T: StT>(a: &'a LinkedListEphS<T>, b: &'a LinkedListEphS<T>, i: N) -> Option<T>select26,1640
    fn append(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> { <LinkedListEpappend33,2132
    fn append2(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> { <LinkedListEappend234,2282
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListEphS<T> { if f(x) == B::True { <LinkedLideflate35,2433
    fn filter(a: &LinkedListEphS<T>, f: impl Fn(&T) -> B) -> LinkedListEphS<T> { <LinkedListEphSfilter36,2654
    fn iterate<A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { <LinkedListEiterate37,2802
    fn reduce<F>(a: &LinkedListEphS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T { <LinkedLisreduce38,2957
    fn scan<F>(a: &LinkedListEphS<T>, f: &F, id: T) -> (LinkedListEphS<T>, T) where F: Fn(&T, &Tscan39,3114
    fn flatten(s: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T> { <LinkedListEphS<T> flatten40,3288
    fn inject(values: &LinkedListEphS<T>, changes: &LinkedListEphS<Pair<N, T>>) -> LinkedListEphinject41,3430

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main7,236

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/RelationEphChap5_2.rs,1954
pub mod RelationEphChap5_2 {RelationEphChap5_23,59
pub struct Relation<A, B> {Relation14,309
    pairs: Set<(A, B)>,pairs15,337
pub trait RelationEphChap5_2Trait<X: Eq + Hash + Display + Debug + Clone + Sized, RelationEphChap5_2Trait18,364
    fn empty() -> Relation<X, Y>;empty20,531
    fn FromSet(pairs: Set<(X, Y)>) -> Relation<X, Y>;FromSet22,566
    fn size(&self) -> N;size24,621
    fn domain(&self) -> Set<X>domain26,647
    fn range(&self) -> Set<Y>range30,707
    fn mem(&self, a: &X, b: &Y) -> Bmem34,766
    fn iter(&self) -> Iter<'_, (X, Y)>;iter39,850
impl<A, B> Relation<A, B> {Relation42,893
    pub fn FromVec(v: Vec<(A, B)>) -> Relation<A, B>FromVec43,921
impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> PartialEq for Relation<A, BRelation52,1123
    fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq53,1223
impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> Eq for Relation<A, B> {}Relation56,1295
impl<A: Debug + Eq + Hash, B: Debug + Eq + Hash> Debug for Relation<A, B> {Relation58,1390
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt59,1466
impl<A: Display + Eq + Hash, B: Display + Eq + Hash> Display for Relation<A, B> {Relation64,1554
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt65,1636
impl<X: Eq + Hash + Display + Debug + Clone + Sized, Relation76,1940
    fn empty() -> Relation<X, Y> {empty78,2099
    fn FromSet(pairs: Set<(X, Y)>) -> Relation<X, Y> { Relation { pairs } }FromSet82,2179
    fn size(&self) -> N { self.pairs.size() }size84,2256
    fn domain(&self) -> Set<X>domain86,2303
    fn range(&self) -> Set<Y>range95,2506
    fn mem(&self, a: &X, b: &Y) -> Bmem104,2708
    fn iter(&self) -> Iter<'_, (X, Y)> { self.pairs.iter() }iter112,2896
macro_rules! RelationLit {RelationLit120,3033
pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise131,3574

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPerChap19.rs,1253
pub mod AVLTreeSeqPerChap19 {AVLTreeSeqPerChap193,47
pub trait AVLTreeSeqPerChap19Trait {AVLTreeSeqPerChap19Trait9,240
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>tabulate10,277
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>map13,394
    fn select<T>(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>, i: N) -> Option<T>select17,568
    fn append<T: Ord + Copy + Debug + Display>(append20,701
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqPerS<T>deflate24,839
    fn filter<T: Ord + Copy + Debug + Display>(filter25,937
impl<T: Ord + Copy + Debug + Display> AVLTreeSeqPerChap19Trait for AVLTreeSeqPerS<T> {AVLTreeSeqPerS31,1076
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>tabulate32,1163
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>map38,1367
    fn select<T>(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>, i: N) -> Option<T>select45,1623
    fn append<T: Ord + Copy + Debug + Display>(append62,2250
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqPerS<T>deflate68,2468
    fn filter<T: Ord + Copy + Debug + Display>(filter75,2767

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,1133
pub mod Types;Types7,226
pub mod MathSeq;MathSeq10,274
pub mod SetEphChap5_1;SetEphChap5_113,351
pub mod RelationEphChap5_2;RelationEphChap5_216,423
pub mod MappingEphChap5_5;MappingEphChap5_519,510
pub mod DirGraphEphChap6_1;DirGraphEphChap6_122,594
pub mod UnDirGraphEphChap6_1;UnDirGraphEphChap6_124,680
pub mod LinkedListPer;LinkedListPer27,773
pub mod LinkedListPerChap18;LinkedListPerChap1829,844
pub mod LinkedListPerChap19;LinkedListPerChap1931,912
pub mod LinkedListEph;LinkedListEph34,981
pub mod LinkedListEphChap18;LinkedListEphChap1836,1052
pub mod LinkedListEphChap19;LinkedListEphChap1938,1120
pub mod ArraySeqPer;ArraySeqPer41,1189
pub mod ArraySeqPerChap18;ArraySeqPerChap1844,1255
pub mod ArraySeqPerChap19;ArraySeqPerChap1947,1320
pub mod ArraySeqEph;ArraySeqEph50,1385
pub mod ArraySeqEphChap18;ArraySeqEphChap1853,1451
pub mod ArraySeqEphChap19;ArraySeqEphChap1955,1534
pub mod AVLTreeSeqPer;AVLTreeSeqPer58,1618
pub mod AVLTreeSeqPerChap18;AVLTreeSeqPerChap1860,1689
pub mod AVLTreeSeqPerChap19;AVLTreeSeqPerChap1962,1778
pub mod AVLTreeSeqEph;AVLTreeSeqEph65,1868

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqEph.rs,4199
pub mod AVLTreeSeqEph {AVLTreeSeqEph3,59
type Link<T> = Option<Box<AVLTreeNode<T>>>;Link8,173
pub struct AVLTreeNode<T: Copy + Debug> {AVLTreeNode10,218
    pub value: T,value11,260
    pub height: N,height12,278
    pub left_size: N,left_size13,297
    pub right_size: N,right_size14,319
    pub left: Link<T>,left15,342
    pub right: Link<T>,right16,365
    pub index: N,index17,389
impl<T: Copy + Debug> AVLTreeNode<T> {AVLTreeNode20,410
    fn new(value: T, index: N) -> Self {new21,449
pub struct AVLTreeSeqEphS<T: Copy + Debug> {AVLTreeSeqEphS34,694
    pub root: Link<T>,root35,739
    pub next_key: N,next_key36,762
pub trait AVLTreeSeqEphTrait<T: Copy + Debug> {AVLTreeSeqEphTrait39,786
    fn empty() -> Self;empty41,872
    fn new() -> Self;new43,934
    fn length(&self) -> N;length45,994
    fn nth(&self, index: N) -> &T;nth47,1067
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set49,1148
    fn singleton(item: T) -> Self;singleton51,1263
    fn isEmpty(&self) -> B;isEmpty53,1336
    fn isSingleton(&self) -> B;isSingleton55,1402
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy57,1492
impl<T: Copy + Debug> AVLTreeSeqEphS<T> {AVLTreeSeqEphS62,1583
    pub fn new_root() -> Self {new_root63,1625
    pub fn new() -> Self {new69,1747
    pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqEphS<T> {update72,1805
    pub fn from_vec(values: Vec<T>) -> AVLTreeSeqEphS<T>from_vec76,1990
    pub fn to_arrayseq(&self) -> ArraySeqEphS<T>to_arrayseq88,2365
    pub fn iter<'a>(&'a self) -> AVLTreeSeqIterEph<'a, T> {iter110,3016
    pub fn push_back(&mut self, value: T) {push_back113,3125
    pub fn contains_value(&self, target: &T) -> Bcontains_value118,3319
    pub fn insert_value(&mut self, value: T) {insert_value129,3546
    pub fn delete_value(&mut self, target: &T) -> booldelete_value132,3630
impl<T: Copy + Debug> AVLTreeSeqEphTrait<T> for AVLTreeSeqEphS<T> {AVLTreeSeqEphS160,4394
    fn empty() -> Self {empty161,4462
    fn new() -> Self {new165,4529
    fn length(&self) -> N {length169,4594
    fn nth(&self, index: N) -> &T {nth173,4659
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set177,4738
    fn singleton(item: T) -> Self {singleton182,4888
    fn isEmpty(&self) -> B {isEmpty188,5063
    fn isSingleton(&self) -> B {isSingleton196,5199
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy204,5339
pub struct AVLTreeSeqIterEph<'a, T: Copy + Debug> {AVLTreeSeqIterEph222,5835
    stack: Vec<&'a AVLTreeNode<T>>,stack223,5887
    current: Option<&'a AVLTreeNode<T>>,current224,5923
impl<'a, T: Copy + Debug> AVLTreeSeqIterEph<'a, T> {AVLTreeSeqIterEph227,5967
    fn new(root: &'a Link<T>) -> Self {new228,6020
    fn push_left(&mut self, link: &'a Link<T>) {push_left236,6215
impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIterEph<'a, T> {AVLTreeSeqIterEph245,6435
    type Item = &'a T;Item246,6501
    fn next(&mut self) -> Option<Self::Item> {next247,6524
fn h<T: Copy + Debug>(n: &Link<T>) -> N {h255,6720
fn size_link<T: Copy + Debug>(n: &Link<T>) -> N {size_link259,6804
fn update_meta<T: Copy + Debug>(n: &mut Box<AVLTreeNode<T>>) {update_meta267,6950
fn rotate_right<T: Copy + Debug>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right275,7176
fn rotate_left<T: Copy + Debug>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left286,7508
fn rebalance<T: Copy + Debug>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance297,7838
pub(crate) fn insert_at_link<T: Copy + Debug>(insert_at_link318,8532
fn nth_link<'a, T: Copy + Debug>(node: &'a Link<T>, index: N) -> &'a T {nth_link343,9276
fn set_link<T: Copy + Debug>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str>set_link355,9621
    macro_rules! AVLTreeSeqEph {AVLTreeSeqEph373,10172
    impl<T: Eq + Copy + Debug> PartialEq for AVLTreeSeqEphS<T> {AVLTreeSeqEphS387,10713
        fn eq(&self, other: &Self) -> bool {eq388,10778
    impl<T: Eq + Copy + Debug> Eq for AVLTreeSeqEphS<T> {}AVLTreeSeqEphS401,11105

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListEphChap18.rs,2875
pub mod LinkedListEphChap18 {LinkedListEphChap183,58
pub trait LinkedListEphChap18Trait<T: St> {LinkedListEphChap18Trait8,192
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T>;tabulate9,236
    fn map<U: StT>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U>;map10,300
    fn append(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;append11,385
    fn filter(a: &LinkedListEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListEphS<T>;filter12,467
    fn update(a: &mut LinkedListEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListEphS<T>;update13,550
    fn inject(a: &LinkedListEphS<T>, updates: &LinkedListEphS<Pair<N, T>>) -> LinkedListEphS<T>;inject14,639
    fn ninject(a: &LinkedListEphS<T>, updates: &LinkedListEphS<Pair<N, T>>) -> LinkedListEphS<T>ninject15,736
    fn iterate< A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,834
    fn iteratePrefixes< A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkediteratePrefixes17,918
    fn reduce(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce18,1031
    fn scan(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListEphS<T>, T);scan19,1107
    fn flatten(ss: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T>;flatten20,1202
    fn collect<A: StT, Bv: StT>(a: &LinkedListEphS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O) -> Lcollect21,1279
impl LinkedListEphChap18Trait<T> for LinkedListEphS<T> {LinkedListEphS24,1422
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T> {tabulate25,1479
    fn map<U: StT>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U> {map30,1677
    fn append(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> {append35,2058
    fn filter(a: &LinkedListEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListEphS<T> {filter43,2588
    fn update(a: &mut LinkedListEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListEphS<T> { <Linkeupdate51,2976
    fn inject(a: &LinkedListEphS<T>, updates: &LinkedListEphS<Pair<N, T>>) -> LinkedListEphS<T> inject52,3133
    fn ninject(a: &LinkedListEphS<T>, updates: &LinkedListEphS<Pair<N, T>>) -> LinkedListEphS<T>ninject69,3944
    fn iterate< A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate82,4605
    fn iteratePrefixes<A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedLiteratePrefixes85,4869
    fn reduce(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce91,5299
    fn scan(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListEphS<T>, T) {scan102,5991
    fn flatten(ss: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T> {flatten110,6615
    fn collect<A: StT, Bv: StT>(a: &LinkedListEphS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O) -> Lcollect120,7203

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPerChap18.rs,835
pub mod AVLTreeSeqPerChap18 {AVLTreeSeqPerChap183,44
pub trait AVLTreeSeqPerChap18Trait {AVLTreeSeqPerChap18Trait8,179
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>tabulate10,300
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>map14,504
    fn append<T: Ord + Copy + Debug + Display>(append19,748
    fn filter<T: Ord + Copy + Debug + Display>(filter24,979
impl<T: Ord + Copy + Debug + Display> AVLTreeSeqPerChap18Trait for AVLTreeSeqPerS<T> {AVLTreeSeqPerS30,1121
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>tabulate31,1208
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>map41,1514
    fn append<T: Ord + Copy + Debug + Display>(append50,1922
    fn filter<T: Ord + Copy + Debug + Display>(filter63,2420

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPer.rs,3871
pub mod AVLTreeSeqPer {AVLTreeSeqPer3,83
type Link<T> = Option<Rc<Node<T>>>;Link9,214
struct Node<T: Copy + Debug> {Node11,251
    value: T,value12,282
    height: N,height13,296
    size: N,size14,311
    left: Link<T>,left15,324
    right: Link<T>,right16,343
fn height<T: Copy + Debug>(n: &Link<T>) -> N {height19,366
fn size<T: Copy + Debug>(n: &Link<T>) -> N {size22,454
fn mk<T: Copy + Debug>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {mk26,539
fn rotate_right<T: Copy + Debug>(y: Rc<Node<T>>) -> Rc<Node<T>> {rotate_right39,845
fn rotate_left<T: Copy + Debug>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left46,1121
fn rebalance<T: Copy + Debug>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance57,1432
fn nth_ref<'a, T: Copy + Debug>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref79,2194
fn set_rec<T: Copy + Debug>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> set_rec94,2596
fn inorder_collect<T: Copy + Debug>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect112,3289
fn build_balanced_from_slice<T: Copy + Debug>(a: &[T]) -> Link<T> {build_balanced_from_slice120,3502
    fn rec<T: Copy + Debug>(a: &[T]) -> Link<T> {rec121,3570
pub struct AVLTreeSeqPerS<T: Copy + Debug> {AVLTreeSeqPerS133,3845
    root: Link<T>,root134,3890
pub trait AVLTreeSeqPerTrait<T: Copy + Debug> {AVLTreeSeqPerTrait137,3912
    fn empty() -> Self;empty139,3997
    fn new() -> Self;new141,4058
    fn length(&self) -> N;length143,4117
    fn nth(&self, index: N) -> &T;nth145,4189
    fn set(&self, index: N, item: T) -> Result<Self, &'static str> where Self: Sized;set147,4328
    fn singleton(item: T) -> Self;singleton149,4451
    fn isEmpty(&self) -> B;isEmpty151,4523
    fn isSingleton(&self) -> B;isSingleton153,4588
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy155,4673
    fn from_vec(values: Vec<T>) -> Selffrom_vec159,4820
    fn values_in_order(&self) -> Vec<T>values_in_order163,4928
impl<T: Copy + Debug> AVLTreeSeqPerTrait<T> for AVLTreeSeqPerS<T> {AVLTreeSeqPerS168,4999
    fn empty() -> Self {empty169,5067
    fn new() -> Self {new172,5136
    fn length(&self) -> N {length175,5187
    fn nth(&self, index: N) -> &T {nth178,5246
    fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set181,5323
    fn singleton(item: T) -> Self {singleton186,5490
    fn isEmpty(&self) -> B {isEmpty191,5613
    fn isSingleton(&self) -> B {isSingleton198,5748
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy205,5887
    fn from_vec(values: Vec<T>) -> Selffrom_vec222,6368
    fn values_in_order(&self) -> Vec<T>values_in_order230,6541
impl<T: Eq + Copy + Debug> PartialEq for AVLTreeSeqPerS<T> {AVLTreeSeqPerS240,6740
    fn eq(&self, other: &Self) -> bool {eq241,6801
impl<T: Eq + Copy + Debug> Eq for AVLTreeSeqPerS<T> {}AVLTreeSeqPerS253,7079
impl<T: Debug + Copy> std::fmt::Debug for AVLTreeSeqPerS<T> {AVLTreeSeqPerS255,7135
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt256,7197
impl<T: Copy + Debug> AVLTreeSeqPerS<T> {AVLTreeSeqPerS262,7369
    pub fn to_arrayseq(&self) -> ArrayPerS<T>to_arrayseq263,7411
    pub fn iter<'a>(&'a self) -> AVLTreeSeqPerIter<'a, T> {iter271,7575
pub struct AVLTreeSeqPerIter<'a, T: Copy + Debug> {AVLTreeSeqPerIter276,7723
    stack: Vec<&'a Node<T>>,stack277,7775
    current: Option<&'a Node<T>>,current278,7804
impl<'a, T: Copy + Debug> AVLTreeSeqPerIter<'a, T> {AVLTreeSeqPerIter281,7841
    fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {push_left282,7894
impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqPerIter<'a, T> {AVLTreeSeqPerIter290,8076
    type Item = &'a T;Item291,8142
    fn next(&mut self) -> Option<Self::Item> {next292,8165
macro_rules! AVLTreeSeqPer {AVLTreeSeqPer308,8513

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListEph.rs,2745
pub mod LinkedListEph {LinkedListEph3,88
pub struct NodeE<T: StT> {NodeE7,158
    pub value: T,value8,185
    pub next: Option<Box<NodeE<T>>>,next9,203
pub struct LinkedListEphS<T: StT> {LinkedListEphS13,260
    head: Option<Box<NodeE<T>>>,head14,296
    len: N,len15,329
pub trait LinkedListEphTrait<T: StT> {LinkedListEphTrait18,344
    fn empty() -> LinkedListEphS<T>;empty19,383
    fn new(length: N, init_value: T) -> Self;new20,420
    fn length(&self) -> N;length21,466
    fn nth(&self, index: N) -> &T;nth22,493
    fn isEmpty(&self) -> B;isEmpty23,528
    fn isSingleton(&self) -> B;isSingleton24,556
    fn singleton(item: T) -> Self;singleton25,588
    fn update(&mut self, item_at: Pair<N, T>) -> &mut Self;update26,623
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set27,683
    fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy28,760
impl<T: StT> LinkedListEphS<T> {LinkedListEphS31,819
    fn push_front_node(&mut self, node: Box<NodeE<T>>) {push_front_node32,852
    pub fn from_vec(v: Vec<T>) -> Self {from_vec39,1029
    pub fn iter<'a>(&'a self) -> LinkedListEphIter<'a, T> {iter47,1264
impl<T: StT> LinkedListEphTrait<T> for LinkedListEphS<T> {LinkedListEphS52,1392
    fn empty() -> Self {empty53,1451
    fn new(length: N, init_value: T) -> Self {new56,1528
    fn length(&self) -> N {length75,2131
    fn nth(&self, index: N) -> &T {nth78,2182
    fn isEmpty(&self) -> B {isEmpty90,2501
    fn isSingleton(&self) -> B {isSingleton97,2631
    fn singleton(item: T) -> Self {singleton104,2765
    fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {update113,2976
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set126,3327
    fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy139,3717
impl<T: StT> std::fmt::Debug for LinkedListEphS<T> {LinkedListEphS175,4755
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt176,4808
pub struct LinkedListEphIter<'a, T: StT> {LinkedListEphIter187,5134
    cursor: Option<&'a NodeE<T>>,cursor188,5177
impl<'a, T: StT> Iterator for LinkedListEphIter<'a, T> {LinkedListEphIter191,5214
    type Item = &'a T;Item192,5271
    fn next(&mut self) -> Option<Self::Item> {next193,5294
impl<T: StT> PartialEq for LinkedListEphS<T> {LinkedListEphS201,5485
    fn eq(&self, other: &Self) -> bool {eq202,5532
impl<T: StT> Eq for LinkedListEphS<T> {}LinkedListEphS219,5957
impl<T: StT> std::fmt::Display for LinkedListEphS<T> {LinkedListEphS221,5999
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt222,6054
macro_rules! LinkedListEph {LinkedListEph236,6458

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPer.rs,3282
pub mod ArraySeqPer {ArraySeqPer8,348
    pub struct ArrayPerS<T: MtT> { data: Box<[T]> }ArrayPerS17,610
    pub struct ArrayPerS<T: MtT> { data: Box<[T]> }data17,610
    pub trait ArraySeqPerTrait<T: MtT> {ArraySeqPerTrait20,732
        fn new(length: N, init_value: T) -> Self where T: Clone;new22,819
        fn length(&self) -> N;length24,925
        fn nth(&self, index: N) -> &T;nth26,997
        fn empty() -> Self;empty28,1077
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone, Self: Sizset32,1328
        fn singleton(item: T) -> Self;singleton34,1469
        fn isEmpty(&self) -> B;isEmpty36,1549
        fn isSingleton(&self) -> B;isSingleton38,1622
        fn subseq_copy(&self, start: N, length: N) -> Self where T: Clone, Self: Sized;subseq_copy40,1704
    impl<T: MtT> ArrayPerS<T> {ArrayPerS43,1799
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq45,1872
        pub fn from_vec(v: Vec<T>) -> Self { ArrayPerS { data: v.into_boxed_slice() } }from_vec54,2235
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter56,2324
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }iter_mut57,2387
        pub fn empty() -> Self { ArrayPerS { data: Vec::new().into_boxed_slice() } }empty59,2466
        pub fn singleton(item: T) -> Self { ArrayPerS { data: vec![item].into_boxed_slice() } }singleton60,2551
        pub fn new(length: N, init_value: T) -> Self where T: Clone { Self::from_vec(vec![init_vnew61,2647
        pub fn length(&self) -> N { self.data.len() }length62,2760
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth63,2814
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {set64,2878
    impl<T: MtT + Eq> PartialEq for ArrayPerS<T> {ArrayPerS72,3181
        fn eq(&self, other: &Self) -> bool {eq73,3232
    impl<T: MtT + Eq> Eq for ArrayPerS<T> {}ArrayPerS79,3466
    impl<T: MtT + Debug> Debug for ArrayPerS<T> {ArrayPerS81,3512
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt82,3562
    impl<T: MtT + Display> Display for ArrayPerS<T> {ArrayPerS88,3755
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt89,3809
    impl<T: MtT> ArraySeqPerTrait<T> for ArrayPerS<T> {ArrayPerS99,4094
        fn new(length: N, init_value: T) -> Self where T: Clone {new100,4150
        fn length(&self) -> N { self.data.len() }length103,4279
        fn nth(&self, index: N) -> &T { &self.data[index] }nth104,4329
        fn empty() -> Self { Self::from_vec(Vec::new()) }empty105,4389
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {set106,4447
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton112,4739
        fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }isEmpty113,4808
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton114,4897
        fn subseq_copy(&self, start: N, length: N) -> Self where T: Clone {subseq_copy115,4990
    macro_rules! ArraySeqPer {ArraySeqPer127,5365
    fn _ArraySeqPer_macro_type_checks() {_ArraySeqPer_macro_type_checks134,5697

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqEphChap18.rs,1032
pub mod AVLTreeSeqEphChap18 {AVLTreeSeqEphChap183,72
pub trait AVLTreeSeqEphChap18Trait {AVLTreeSeqEphChap18Trait8,207
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T>tabulate11,383
    fn map<T, U>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U>map17,660
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> append24,942
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, pred: impl Fn(&T) -> B) ->filter28,1198
impl<T: Ord + Copy + Debug + Display> AVLTreeSeqEphChap18Trait for AVLTreeSeqEphS<T> {AVLTreeSeqEphS31,1317
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T>tabulate32,1404
    fn map<T, U>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U>map40,1682
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> append49,2027
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, pred: impl Fn(&T) -> B) ->filter55,2396

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/12_TestArraySeqEph.rs,270
pub mod TestArraySeqEph {TestArraySeqEph1,0
fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic10,262
fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter19,488
fn test_iterators_collect() {test_iterators_collect29,971

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/08_TestLinkedListEphChap19.rs,516
pub mod TestLinkedListEphChap19 {TestLinkedListEphChap191,0
fn test_eph_set_and_nth() {test_eph_set_and_nth8,223
fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug15,374
fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1925,685
fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1932,883
fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1942,1488

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/07_TestLinkedListEphChap18.rs,805
pub mod TestLinkedListEphChap18 {TestLinkedListEphChap181,0
fn test_construct_eph_from_vec() {test_construct_eph_from_vec10,325
fn test_eph_is_empty_and_singleton() {test_eph_is_empty_and_singleton16,458
fn test_eph_set_and_subseq_copy() {test_eph_set_and_subseq_copy24,705
fn test_iter_inorder_collect_eph_ch18() {test_iter_inorder_collect_eph_ch1833,932
fn test_tabulate_and_map_ch18() {test_tabulate_and_map_ch1840,1130
fn test_append_ch18() {test_append_ch1848,1476
fn test_filter_ch18() {test_filter_ch1857,1808
fn test_update_ch18() {test_update_ch1865,2114
fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1873,2418
fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1883,2923
fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch1895,3484

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/05_TestLinkedListPerChap19.rs,407
pub mod TestLinkedListPerChap19 {TestLinkedListPerChap191,0
fn test_select() {test_select9,283
fn test_append_variants() {test_append_variants22,733
fn test_deflate() {test_deflate32,1106
fn test_map() {test_map40,1474
fn test_iterate_and_reduce() {test_iterate_and_reduce47,1666
fn test_scan() {test_scan56,1989
fn test_flatten() {test_flatten64,2239
fn test_inject() {test_inject74,2498

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/01_TestTypes.rs,677
pub mod TestTypes {TestTypes1,0
    fn test_boolean_eq_neq_and_ordering() {test_boolean_eq_neq_and_ordering5,67
    fn test_ordering_on_n_naturals() {test_ordering_on_n_naturals15,324
    fn test_cmp_on_b_returns_expected_ordering_variants() {test_cmp_on_b_returns_expected_ordering_variants24,571
    fn test_btree_set_orders_b_true_before_false() {test_btree_set_orders_b_true_before_false32,895
    fn test_n_aliases_usize_and_cmp_examples() {test_n_aliases_usize_and_cmp_examples42,1195
    fn test_debug_format_for_b_variants() {test_debug_format_for_b_variants57,1648
    fn test_display_format_for_b_variants() {test_display_format_for_b_variants63,1821

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/11_TestArraySeqPerChap19.rs,342
pub mod TestArraySeqPerChap19 {TestArraySeqPerChap191,0
fn test_map_and_select_and_append() {test_map_and_select_and_append10,289
fn test_deflate_and_filter() {test_deflate_and_filter20,752
fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten29,1212
fn test_inject_and_parallel() {test_inject_and_parallel48,2116

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Problem21_1.rs,418
fn points2d(n: N) -> ArrayPerS<(N, N)> {points2d9,287
fn test_points2d_n3_example() {test_points2d_n3_example22,576
fn test_points2d_n1_empty() {test_points2d_n1_empty30,795
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values36,892
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order44,1066
fn test_points2d_debug_display_shapes() {test_points2d_debug_display_shapes52,1334

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/10_TestArraySeqPerChap18.rs,943
pub mod TestArraySeqPerChap18 {TestArraySeqPerChap181,0
fn test_tabulate_fibonacci() {test_tabulate_fibonacci9,244
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }fib10,275
fn test_map_increment() {test_map_increment21,719
fn test_subseq() {test_subseq28,922
fn test_append() {test_append39,1333
fn test_sequence_literals_and_append() {test_sequence_literals_and_append47,1557
fn test_filter_even() {test_filter_even60,2127
fn test_flatten() {test_flatten70,2613
fn test_update_sequence() {test_update_sequence84,3213
fn test_inject_and_ninject() {test_inject_and_ninject94,3652
fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan113,4771
fn test_iterate_sum_basic() {test_iterate_sum_basic132,5675
fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum140,5912
fn test_collect_groups_by_key() {test_collect_groups_by_key152,6322

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercsise_21_9.rs,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/14_TestArraySeqEphChap19.rs,1332
pub mod TestArraySeqEphChap19 {TestArraySeqEphChap193,93
fn test_empty() {test_empty11,360
fn test_singleton() {test_singleton17,483
fn test_map() {test_map23,639
fn test_append() {test_append30,879
fn test_append2() {test_append238,1201
fn test_deflate_true() {test_deflate_true46,1525
fn test_deflate_false() {test_deflate_false52,1724
fn test_filter_even_numbers() {test_filter_even_numbers58,1919
fn test_filter_none() {test_filter_none65,2226
fn test_update_in_bounds() {test_update_in_bounds72,2518
fn test_update_out_of_bounds() {test_update_out_of_bounds79,2780
fn test_isEmpty() {test_isEmpty86,3039
fn test_isSingleton() {test_isSingleton96,3397
fn test_iterate_sum() {test_iterate_sum106,3771
fn test_iterate_concat() {test_iterate_concat113,4019
fn test_map_empty() {test_map_empty127,4404
fn test_append_with_empty() {test_append_with_empty134,4645
fn test_append2_equivalence() {test_append2_equivalence144,5117
fn test_filter_empty_sequence() {test_filter_empty_sequence153,5507
fn test_select_boundary() {test_select_boundary160,5765
fn test_subseq_basic() {test_subseq_basic171,6429
fn test_reduce_sum_basic_ch19() {test_reduce_sum_basic_ch19178,6678
fn test_scan_sum_basic_ch19() {test_scan_sum_basic_ch19192,7348
fn test_flatten_ch19() {test_flatten_ch19203,7752

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Problem_21_4.rs,996
fn cartesian_loops(a: &ArrayPerS<N>, b: &ArrayPerS<&'static str>) -> ArrayPerS<(N, &'static str)cartesian_loops11,369
    let mut v: Vec<(N, &'static str)> = Vec::with_capacity(a.length() * b.length());str12,469
fn cartesian_tab_flat(a: &ArrayPerS<N>, b: &ArrayPerS<&'static str>) -> ArrayPerS<(N, &'static scartesian_tab_flat23,839
    let nested: ArrayPerS<ArrayPerS<(N, &'static str)>> =str24,942
        <ArrayPerS<ArrayPerS<(N, &'static str)>> as ArraySeqPerChap19Trait<T>>::map(str25,1000
            |x| <ArrayPerS<(N, &'static str)> as ArraySeqPerChap19Trait<T>>::map(b, |y| (*x, *y)str27,1100
    <ArrayPerS<(N, &'static str)> as ArraySeqPerChap18Trait<T>>::flatten(&nested)str29,1210
fn test_cartesian_loops_basic() {test_cartesian_loops_basic33,1303
fn test_cartesian_tab_flat_basic() {test_cartesian_tab_flat_basic42,1594
fn test_cartesian_iterator_order() {test_cartesian_iterator_order51,1891
fn test_cartesian_debug_shape() {test_cartesian_debug_shape60,2169

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/06_TestLinkedListEph.rs,517
pub mod TestLinkedListEph {TestLinkedListEph2,56
fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates9,214
fn test_new_and_nth_set() {test_new_and_nth_set18,500
fn test_subseq_copy() {test_subseq_copy27,701
fn test_linkedlisteph_basic() {test_linkedlisteph_basic36,902
fn test_debug_format_for_eph() {test_debug_format_for_eph45,1108
fn test_display_format_for_eph() {test_display_format_for_eph51,1238
fn test_iter_inorder_collect_eph() {test_iter_inorder_collect_eph57,1368

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/17_TestAVLTreeSeqPerChap19.rs,363
pub mod TestAVLTreeSeqPerChap19 {TestAVLTreeSeqPerChap191,0
fn test_tabulate_and_map_ch19() {test_tabulate_and_map_ch198,223
fn test_select_and_append_ch19() {test_select_and_append_ch1916,553
fn test_deflate_and_filter_ch19() {test_deflate_and_filter_ch1928,1275
fn test_iter_inorder_after_pipeline_ch19() {test_iter_inorder_after_pipeline_ch1939,1859

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Problem_21_3.rs,475
fn points3d_loops(n: N) -> ArrayPerS<(N, N, N)> {points3d_loops9,342
fn test_points3d_loops_n0_empty() {test_points3d_loops_n0_empty24,696
fn test_points3d_loops_n1_single() {test_points3d_loops_n1_single30,805
fn test_points3d_loops_n2_values_and_order() {test_points3d_loops_n2_values_and_order37,953
fn test_points3d_loops_iterator_order() {test_points3d_loops_iterator_order50,1238
fn test_points3d_loops_debug_shape() {test_points3d_loops_debug_shape57,1485

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/19_TestAVLTreeSeqEphChap18.rs,272
pub mod TestAVLTreeSeqEphChap18 {TestAVLTreeSeqEphChap183,79
fn test_tabulate_inorder() {test_tabulate_inorder11,339
fn test_map_increment() {test_map_increment17,503
fn test_append_union() {test_append_union25,804
fn test_filter_even() {test_filter_even35,1192

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/04_TestLinkedListPerChap18.rs,527
pub mod TestLinkedListPerChap18 {TestLinkedListPerChap181,0
fn test_tabulate() {test_tabulate8,200
fn test_map() {test_map15,381
fn test_filter() {test_filter23,640
fn test_append() {test_append30,928
fn test_update() {test_update38,1189
fn test_inject() {test_inject45,1400
fn test_ninject() {test_ninject53,1663
fn test_iterate() {test_iterate61,1930
fn test_reduce() {test_reduce68,2128
fn test_scan() {test_scan75,2321
fn test_flatten() {test_flatten83,2583
fn test_collect() {test_collect94,2879

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/25_TestUnDirGraphEphChap6_1.rs,145
pub mod TestUnDirGraphEphChap6_1 {TestUnDirGraphEphChap6_11,0
fn test_undigraph_vertices_and_edges() {test_undigraph_vertices_and_edges8,201

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/18_TestAVLTreeSeqEph.rs,113
pub mod TestAVLTreeSeqEph {TestAVLTreeSeqEph1,0
fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic7,206

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/24_TestDirGraphEphChap6_1.rs,135
pub mod TestDirGraphEphChap6_1 {TestDirGraphEphChap6_11,0
fn test_digraph_vertices_and_arcs() {test_digraph_vertices_and_arcs8,195

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/09_TestArraySeqPer.rs,1751
pub mod TestArraySeqPer {TestArraySeqPer3,93
fn test_new_and_set() {test_new_and_set9,242
fn test_length_and_nth_basic() {test_length_and_nth_basic23,714
fn test_empty() {test_empty31,893
fn test_sequence_basic() {test_sequence_basic38,1077
fn test_singleton() {test_singleton51,1695
fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton59,1898
fn test_from_vec() {test_from_vec74,2364
fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers89,2941
fn test_sequence_equality_strings() {test_sequence_equality_strings114,3731
fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference139,4615
    struct PartialComparable { value: f64 }PartialComparable141,4694
    struct PartialComparable { value: f64 }value141,4694
    struct TotalComparable { value: N }TotalComparable150,5142
    struct TotalComparable { value: N }value150,5142
fn test_ordering_numbers_basic() {test_ordering_numbers_basic162,5617
fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal171,5832
fn test_ordering_strings_basic() {test_ordering_strings_basic177,5940
fn test_strings_equal_is_equal() {test_strings_equal_is_equal186,6153
fn test_nth_on_empty_panics() {test_nth_on_empty_panics193,6278
fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics200,6425
fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap207,6536
fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes213,6731
fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic222,6988
fn test_new_set_persistent() {test_new_set_persistent231,7339
fn test_iterator_collects_in_order() {test_iterator_collects_in_order241,7624

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_7.rs,424
fn is_even(x: &N) -> B { if *x % 2 == 0 { B::True } else { B::False } }is_even9,309
fn is_vowel(c: &char) -> B {is_vowel10,381
fn pair_even_with_vowels(a: &ArrayPerS<N>, b: &ArrayPerS<char>) -> ArrayPerS<(N, char)> {pair_even_with_vowels19,648
fn test_pair_even_with_vowels_basic() {test_pair_even_with_vowels_basic30,1289
fn test_pair_even_with_vowels_debug_shape() {test_pair_even_with_vowels_debug_shape39,1560

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Algorithm_21_2.rs,515
fn points3d_tab_flat(n: N) -> ArrayPerS<(N, N, N)> {points3d_tab_flat12,511
fn test_points3d_tab_flat_n0_empty() {test_points3d_tab_flat_n0_empty33,1466
fn test_points3d_tab_flat_n1_single() {test_points3d_tab_flat_n1_single39,1581
fn test_points3d_tab_flat_n2_values_and_order() {test_points3d_tab_flat_n2_values_and_order46,1735
fn test_points3d_tab_flat_iterator_order() {test_points3d_tab_flat_iterator_order59,2026
fn test_points3d_tab_flat_debug_shape() {test_points3d_tab_flat_debug_shape66,2279

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/15_TestAVLTreeSeqPer.rs,207
pub mod TestAVLTreeSeqPer {TestAVLTreeSeqPer1,0
fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate7,173
fn test_iterator_inorder_values() {test_iterator_inorder_values16,518

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_2.txt,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/13_TestArraySeqEphChap18.rs,1043
pub mod TestArraySeqEphChap18 {TestArraySeqEphChap183,93
fn test_tabulate_fibonacci() {test_tabulate_fibonacci10,304
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }fib11,335
fn test_map_increment() {test_map_increment22,782
fn test_subseq() {test_subseq29,988
fn test_append() {test_append40,1408
fn test_sequence_literals_and_append() {test_sequence_literals_and_append48,1635
fn test_filter_even() {test_filter_even61,2220
fn test_flatten() {test_flatten71,2712
fn test_update_sequence() {test_update_sequence85,3325
fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins97,3864
fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins110,4522
fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan126,5158
fn test_iterate_sum_basic() {test_iterate_sum_basic145,6080
fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum153,6320
fn test_collect_groups_by_key() {test_collect_groups_by_key165,6733

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Algorithm_21_6.rs,259
fn prime_sieve(n: N) -> ArrayPerS<N> {prime_sieve12,490
fn test_prime_sieve_small() {test_prime_sieve_small46,2188
fn test_prime_sieve_n2_empty() {test_prime_sieve_n2_empty53,2344
fn test_prime_sieve_debug_shape() {test_prime_sieve_debug_shape59,2447

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/21_TestSetEphChap5_1.rs,545
pub mod TestSetEphChap5_1 {TestSetEphChap5_11,0
fn macro_typecheck_exercise() {macro_typecheck_exercise8,163
    let _empty: Set<&'static str> = SetLit![];str9,195
fn test_cartesian_product_example_5_1() {test_cartesian_product_example_5_115,325
fn test_partition_example_5_2_true() {test_partition_example_5_2_true31,724
fn test_partition_example_5_2_false_due_to_overlap() {test_partition_example_5_2_false_due_to_overlap40,989
fn test_partition_false_due_to_missing_element() {test_partition_false_due_to_missing_element49,1302

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/22_TestRelationEphChap5_2.rs,143
pub mod TestRelationEphChap5_2 {TestRelationEphChap5_21,0
fn test_relation_domain_range_and_mem() {test_relation_domain_range_and_mem8,211

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Algorithm_21_1.rs,421
fn points2d_tab_flat(n: N) -> ArrayPerS<(N, N)> {points2d_tab_flat11,432
fn test_points2d_n3_example() {test_points2d_n3_example22,854
fn test_points2d_n1_empty() {test_points2d_n1_empty29,1037
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values35,1143
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order43,1326
fn test_points2d_debug_shape() {test_points2d_debug_shape51,1603

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_8_and_Algorithm_21_5.rs,391
fn is_divisible(n: N, i: N) -> B { if n % i == 0 { B::True } else { B::False } }is_divisible8,264
fn is_prime(n: N) -> B {is_prime13,507
fn primes_bf(n: N) -> ArrayPerS<N> {primes_bf25,1086
fn test_is_prime_small_values() {test_is_prime_small_values33,1416
fn test_primes_bf_small() {test_primes_bf_small43,1693
fn test_primes_bf_debug_shape() {test_primes_bf_debug_shape50,1845

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/02_TestMathSeq.rs,1282
pub mod TestMathSeq {TestMathSeq3,93
fn test_length_and_nth_basic() {test_length_and_nth_basic9,210
fn test_add_last_and_delete_last() {test_add_last_and_delete_last17,398
fn test_new_empty_singleton_and_predicates() {test_new_empty_singleton_and_predicates30,795
fn test_set_in_bounds_and_out_of_bounds() {test_set_in_bounds_and_out_of_bounds49,1317
fn test_subseq_view_bounds() {test_subseq_view_bounds60,1632
fn test_subseq_copy_bounds() {test_subseq_copy_bounds73,1977
fn test_domain() {test_domain83,2233
fn test_range_deduplicates_preserving_order() {test_range_deduplicates_preserving_order89,2350
fn test_debug_format_for_mathseq() {test_debug_format_for_mathseq96,2550
fn test_display_format_for_mathseq() {test_display_format_for_mathseq102,2691
fn test_multiset_range_counts_first_occurrence_order() {test_multiset_range_counts_first_occurrence_order108,2832
fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics116,3050
fn test_range_empty_returns_empty() {test_range_empty_returns_empty122,3156
fn test_multiset_range_empty_returns_empty() {test_multiset_range_empty_returns_empty129,3301
fn test_domain_empty_is_empty() {test_domain_empty_is_empty136,3469
fn test_iter_collect_and_sum() {test_iter_collect_and_sum143,3603

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/03_TestLinkedListPer.rs,508
pub mod TestLinkedListPer {TestLinkedListPer1,0
fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates7,157
fn test_new_and_nth_set() {test_new_and_nth_set16,406
fn test_subseq_copy() {test_subseq_copy28,727
fn test_from_vec_and_debug_format() {test_from_vec_and_debug_format37,928
fn test_iter_inorder_collect() {test_iter_inorder_collect44,1094
fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics52,1286
fn test_display_impl() {test_display_impl58,1390

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_5_and_21_6.rs,381
fn all_contiguous_subseqs<T: Clone + Eq>(a: &ArrayPerS<T>) -> ArrayPerS<ArrayPerS<T>> {all_contiguous_subseqs12,448
fn test_all_contiguous_subseqs_n0() {test_all_contiguous_subseqs_n029,1075
fn test_all_contiguous_subseqs_n3_values() {test_all_contiguous_subseqs_n3_values36,1241
fn test_all_contiguous_subseqs_debug_shape() {test_all_contiguous_subseqs_debug_shape49,1601

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/16_TestAVLTreeSeqPerChap18.rs,272
pub mod TestAVLTreeSeqPerChap18 {TestAVLTreeSeqPerChap183,49
fn test_tabulate_inorder() {test_tabulate_inorder12,359
fn test_map_increment() {test_map_increment18,556
fn test_append_union() {test_append_union25,858
fn test_filter_even() {test_filter_even33,1260

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/20_TestAVLTreeSeqEphChap19.rs,244
pub mod TestAVLTreeSeqEphChap19 {TestAVLTreeSeqEphChap193,80
fn test_tabulate_and_map() {test_tabulate_and_map11,328
fn test_select_and_append() {test_select_and_append19,622
fn test_deflate_and_filter() {test_deflate_and_filter38,1418

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListPer.rs,80
fn bench_sll_persistent_ops(c: &mut Criterion) {bench_sll_persistent_ops8,225

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqPerChap19.rs,95
fn bench_build_and_read_persistent(c: &mut Criterion) {bench_build_and_read_persistent10,361

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqPerChap19.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch198,281

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchMathSeq.rs,72
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics8,213

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListPerChap18.rs,66
fn bench_ll_per_ch18(c: &mut Criterion) {bench_ll_per_ch187,247

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchRelationEphChap5_2.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range8,264

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqEphChap19.rs,68
fn bench_avl_eph_ch19(c: &mut Criterion) {bench_avl_eph_ch196,187

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqEphChap19.rs,86
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch198,307

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListEphChap19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch197,247

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqPer.rs,80
fn bench_build_and_contains(c: &mut Criterion) {bench_build_and_contains9,285

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqPer.rs,477
struct LinearCongruentialGenerator32 { state: u32 }LinearCongruentialGenerator3211,415
struct LinearCongruentialGenerator32 { state: u32 }state11,415
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3213,468
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new14,505
    fn next_N(&mut self) -> N {next_N16,624
fn bench_build_random_s_persistent(c: &mut Criterion) {bench_build_random_s_persistent25,861

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqEph.rs,58
fn bench_avl_eph(c: &mut Criterion) {bench_avl_eph6,187

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchUnDirGraphEphChap6_1.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build7,247

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch187,239

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqEphChap18.rs,68
fn bench_avl_eph_ch18(c: &mut Criterion) {bench_avl_eph_ch186,187

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchMappingEphChap5_5.rs,70
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build8,297

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListEph.rs,56
fn bench_ll_eph(c: &mut Criterion) {bench_ll_eph6,187

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListPerChap19.rs,66
fn bench_ll_per_ch19(c: &mut Criterion) {bench_ll_per_ch198,323

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchDirGraphEphChap6_1.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build7,243

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListEphChap18.rs,66
fn bench_ll_eph_ch18(c: &mut Criterion) {bench_ll_eph_ch187,247

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqEph.rs,455
struct LinearCongruentialGenerator32 { state: u32 }LinearCongruentialGenerator3211,407
struct LinearCongruentialGenerator32 { state: u32 }state11,407
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3213,460
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new14,497
    fn next_N(&mut self) -> N {next_N16,616
fn bench_build_random_s(c: &mut Criterion) {bench_build_random_s25,853

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqEphChap18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map8,307

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqPerChap18.rs,68
fn bench_avl_per_ch18(c: &mut Criterion) {bench_avl_per_ch187,247
