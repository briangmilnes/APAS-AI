
/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPerChap19.rs,3286
pub trait ArraySeqPerChap19Trait {ArraySeqPerChap19Trait8,205
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T>;tabulate10,322
    fn map<T, U>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U>;map12,460
    fn select<'a, T>(a: &'a ArrayPerS<T>, b: &'a ArrayPerS<T>, i: N) -> Option<&'a T>;select14,570
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;append16,706
    fn append2<T: Clone>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;append218,837
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArrayPerS<T>;deflate20,952
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, f: impl Fn(&T) -> B) -> ArrayPerS<T>;filter22,1105
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate26,1358
    fn reduce<T: Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> Treduce28,1496
    fn scan<T: Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> (ArrayPerS<T>, T)scan32,1646
    fn flatten<T: Clone + Eq>(s: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T>;flatten36,1827
    fn inject<T: Clone + Eq>(values: &ArrayPerS<T>, changes: &ArrayPerS<(N, T)>) -> ArrayPerS<T>inject40,2090
    fn atomicWrite<T: Clone + Eq>(atomicWrite42,2225
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(inject_parallel248,2447
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(AtomicWriteLowestChangeNumberWins52,2592
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(ninject_parallel258,2846
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(AtomicWriteHighestChangeNumberWins62,2992
impl<T2> ArraySeqPerChap19Trait for ArrayPerS<T2> {ArrayPerS69,3191
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T> {tabulate70,3243
    fn map<T, U>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U> {map74,3399
    fn select<'a, T>(a: &'a ArrayPerS<T>, b: &'a ArrayPerS<T>, i: N) -> Option<&'a T> {select77,3567
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {append89,3898
    fn append2<T: Clone>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {append295,4150
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArrayPerS<T> {deflate101,4398
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, f: impl Fn(&T) -> B) -> ArrayPerS<T> {filter105,4611
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate113,4993
    fn reduce<T: Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> Treduce127,5490
    fn scan<T: Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> (ArrayPerS<T>, T)scan151,6226
    fn flatten<T: Clone + Eq>(s: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T> {flatten200,7871
    fn inject<T: Clone + Eq>(values: &ArrayPerS<T>, changes: &ArrayPerS<(N, T)>) -> ArrayPerS<T>inject203,8015
    fn atomicWrite<T: Clone + Eq>(atomicWrite215,8552
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(inject_parallel2230,9128
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(AtomicWriteLowestChangeNumberWins257,10043
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(ninject_parallel2273,10607
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(AtomicWriteHighestChangeNumberWins300,11524

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqEphChap19.rs,1370
pub trait AVLTreeSeqChap19Trait {AVLTreeSeqChap19Trait12,428
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeEphS<T>tabulate14,546
    fn map<T, U>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeEphS<U>map18,747
    fn select<'a, T>(a: &'a AVLTreeEphS<T>, b: &'a AVLTreeEphS<T>, i: N) -> Option<T>select23,990
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, b: &AVLTreeEphS<T>) -> AVLTreappend27,1197
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeEphS<T>;deflate29,1334
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> B) -> AVLTrfilter31,1516
impl<T2: Ord + Copy + Debug + Display> AVLTreeEphSeqChap19 for AVLTreeEphS<T2> {AVLTreeEphS34,1626
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeEphS<T>tabulate36,1791
    fn map<T, U>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeEphS<U>map42,2057
    fn select<'a, T>(a: &'a AVLTreeEphS<T>, b: &'a AVLTreeEphS<T>, i: N) -> Option<T>select49,2360
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, b: &AVLTreeEphS<T>) -> AVLTreappend63,2962
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeEphS<T> {deflate68,3169
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> B) -> AVLTrfilter73,3497

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPer.rs,2497
pub struct NodeP<T> {NodeP5,89
    pub value: T,value6,111
    pub next: Option<Box<NodeP<T>>>,next7,129
pub struct LinkedListPerS<T> {LinkedListPerS10,169
    head: Option<Box<NodeP<T>>>,head11,200
    len: N,len12,233
pub trait LinkedListPerTrait<T> {LinkedListPerTrait15,248
    fn empty() -> Self;empty16,282
    fn new(length: N, init_value: T) -> Selfnew17,306
    fn length(&self) -> N;length20,379
    fn nth(&self, index: N) -> &T;nth21,406
    fn isEmpty(&self) -> B;isEmpty22,441
    fn isSingleton(&self) -> B;isSingleton23,469
    fn singleton(item: T) -> Self;singleton24,501
    fn set(&self, index: N, item: T) -> Result<Self, &'static str>set28,731
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy32,847
impl<T> LinkedListPerS<T> {LinkedListPerS37,933
    fn push_front_node(&mut self, node: Box<NodeP<T>>) {push_front_node38,961
    pub fn from_vec(v: Vec<T>) -> Self {from_vec45,1138
    pub fn iter<'a>(&'a self) -> LinkedListPerIter<'a, T> {iter53,1373
impl<T> LinkedListPerTrait<T> for LinkedListPerS<T> {LinkedListPerS58,1501
    fn empty() -> Self {empty59,1555
    fn new(length: N, init_value: T) -> Selfnew62,1632
    fn length(&self) -> N {length84,2267
    fn nth(&self, index: N) -> &T {nth87,2318
    fn isEmpty(&self) -> B {isEmpty99,2637
    fn isSingleton(&self) -> B {isSingleton106,2767
    fn singleton(item: T) -> Self {singleton113,2901
    fn set(&self, index: N, item: T) -> Result<Self, &'static str>set122,3112
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy157,4142
impl<T: std::fmt::Debug> std::fmt::Debug for LinkedListPerS<T> {LinkedListPerS195,5175
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt196,5240
pub struct LinkedListPerIter<'a, T> {LinkedListPerIter207,5566
    cursor: Option<&'a NodeP<T>>,cursor208,5604
impl<'a, T> Iterator for LinkedListPerIter<'a, T> {LinkedListPerIter211,5641
    type Item = &'a T;Item212,5693
    fn next(&mut self) -> Option<Self::Item> {next213,5716
impl<T: PartialEq> PartialEq for LinkedListPerS<T> {LinkedListPerS221,5907
    fn eq(&self, other: &Self) -> bool {eq222,5960
impl<T: Eq> Eq for LinkedListPerS<T> {}LinkedListPerS239,6385
impl<T: std::fmt::Display> std::fmt::Display for LinkedListPerS<T> {LinkedListPerS241,6426
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt242,6495
macro_rules! LinkedListPer {LinkedListPer256,6899

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/MathSeq.rs,2556
pub struct MathSeqS<T> {MathSeqS16,578
    pub data: Vec<T>,data17,603
impl<T: PartialEq> PartialEq for MathSeqS<T> {MathSeqS20,628
    fn eq(&self, other: &Self) -> bool {eq21,675
impl<T: Eq> Eq for MathSeqS<T> {}MathSeqS26,757
impl<T: std::fmt::Debug> std::fmt::Debug for MathSeqS<T> {MathSeqS28,792
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt29,851
impl<T: std::fmt::Display> std::fmt::Display for MathSeqS<T> {MathSeqS38,1105
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt39,1168
impl<T> MathSeqS<T> {MathSeqS50,1469
    pub fn iter(&self) -> std::slice::Iter<'_, T> {iter51,1491
pub trait MathSeqTrait<T> {MathSeqTrait57,1609
    fn new(length: N, init_value: T) -> Selfnew61,1765
    fn empty() -> Self;empty68,1917
    fn singleton(item: T) -> Self;singleton73,2040
    fn length(&self) -> N;length77,2147
    fn nth(&self, index: N) -> &T;nth81,2274
    fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str>;set85,2397
    fn add_last(&mut self, value: T) -> &mut Self;add_last90,2748
    fn delete_last(&mut self) -> Option<T>;delete_last94,2874
    fn subseq(&self, start: N, length: N) -> &[T];subseq98,3026
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy103,3213
    fn isEmpty(&self) -> B;isEmpty110,3384
    fn isSingleton(&self) -> B;isSingleton115,3499
    fn domain(&self) -> Vec<N>;domain119,3609
    fn range(&self) -> Vec<T>range123,3777
    fn multiset_range(&self) -> Vec<(N, T)>multiset_range129,3979
impl<T> MathSeqTrait<T> for MathSeqS<T> {MathSeqS134,4066
    fn new(length: N, init_value: T) -> Selfnew136,4147
    fn empty() -> Self {empty146,4340
    fn singleton(item: T) -> Self {singleton151,4444
    fn length(&self) -> N {length156,4559
    fn nth(&self, index: N) -> &T {nth161,4651
    fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str> {set166,4753
    fn add_last(&mut self, value: T) -> &mut Self {add_last178,5253
    fn delete_last(&mut self) -> Option<T> {delete_last184,5389
    fn subseq(&self, start: N, length: N) -> &[T] {subseq189,5498
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy197,5737
    fn isEmpty(&self) -> B {isEmpty213,6136
    fn isSingleton(&self) -> B {isSingleton222,6308
    fn domain(&self) -> Vec<N> {domain231,6495
    fn range(&self) -> Vec<T>range236,6619
    fn multiset_range(&self) -> Vec<(N, T)>multiset_range251,7026
macro_rules! MathSeq {MathSeq273,7651

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPerChap18.rs,2808
pub trait ArraySeqPerChap18Trait {ArraySeqPerChap18Trait6,129
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T>;tabulate8,246
    fn map<T, U: Clone>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U>;map11,385
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;append14,515
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, pred: impl Fn(&T) -> B) -> ArrayPerS<T>;filter17,691
    fn update<T: Clone + Eq>(a: &ArrayPerS<T>, item_at: (N, T)) -> ArrayPerS<T>;update22,930
    fn inject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T>;inject25,1085
    fn ninject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T>;ninject28,1234
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate31,1402
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(iteratePrefixes34,1539
    fn reduce<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce41,1769
    fn scan<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayPerS<T>, scan44,1899
    fn flatten<T: Clone + Eq>(ss: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T>;flatten47,2066
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(collect50,2219
impl<T2> ArraySeqPerChap18Trait for ArrayPerS<T2> {ArrayPerS56,2376
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T> {tabulate57,2428
    fn map<T, U: Clone>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U> {map61,2590
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {append73,3025
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, pred: impl Fn(&T) -> B) -> ArrayPerS<T> {filter87,3496
    fn update<T: Clone + Eq>(a: &ArrayPerS<T>, (index, item): (N, T)) -> ArrayPerS<T> {update96,3804
    fn inject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T> {inject102,4012
    fn ninject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T> {ninject114,4491
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate124,4856
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(iteratePrefixes131,5075
    fn reduce<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce144,5476
        fn rec<T: Clone + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec145,5563
    fn scan<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayPerS<T>, scan160,5998
        fn rec<T: Clone + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec161,6099
    fn flatten<T: Clone + Eq>(ss: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T> {flatten182,6775
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(collect192,7110

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPerChap19.rs,2443
pub trait LinkedListPerChap19Trait {LinkedListPerChap19Trait7,195
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T>;tabulate8,232
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U>;map9,299
    fn select<'a, T>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T>select10,389
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;append13,509
    fn append2<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;append214,601
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T>;deflate15,694
    fn filter<T: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T>;filter16,769
    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate17,859
    fn reduce<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> Treduce18,954
    fn scan<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T)scan21,1061
    fn flatten<T: Clone>(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T>;flatten24,1187
    fn inject<T: Clone + Eq>(inject25,1273
impl<T2> LinkedListPerChap19Trait for LinkedListPerS<T2> {LinkedListPerS31,1412
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T> {tabulate32,1471
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U> {map35,1617
    fn select<'a, T>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T>select38,1782
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {append55,2389
    fn append2<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {append259,2560
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T> {deflate63,2732
    fn filter<T: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T> {filter71,3016
    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate75,3185
    fn reduce<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> Treduce79,3363
    fn scan<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T)scan86,3558
    fn flatten<T: Clone>(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T> {flatten93,3770
    fn inject<T: Clone + Eq>(inject97,3933

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPerChap18.rs,2309
pub trait LinkedListPerChap18Trait {LinkedListPerChap18Trait6,140
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T>;tabulate8,252
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U>;map11,396
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;append14,536
    fn filter<T: Clone>(a: &LinkedListPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListPerS<T>;filter17,722
    fn update<T: Clone>(a: &LinkedListPerS<T>, item_at: (N, T)) -> LinkedListPerS<T>;update22,966
    fn inject<T: Clone + Eq>(inject25,1126
    fn ninject<T: Clone + Eq>(ninject31,1313
    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate37,1519
    fn iteratePrefixes<T: Clone, A: Clone>(iteratePrefixes40,1656
    fn reduce<T: Clone>(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce47,1891
    fn scan<T: Clone>(scan50,2021
    fn flatten<T: Clone>(ss: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T>;flatten57,2224
    fn collect<A: Clone + Eq, Bv: Clone>(collect60,2387
impl<T2> LinkedListPerChap18Trait for LinkedListPerS<T2> {LinkedListPerS66,2554
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T> {tabulate67,2613
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U> {map75,2835
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {append83,3253
    fn filter<T: Clone>(a: &LinkedListPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListPerS<T> {filter95,3833
    fn update<T: Clone>(a: &LinkedListPerS<T>, (index, item): (N, T)) -> LinkedListPerS<T> {update105,4259
    fn inject<T: Clone + Eq>(inject114,4751
    fn ninject<T: Clone + Eq>(ninject137,5668
    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate159,6558
    fn iteratePrefixes<T: Clone, A: Clone>(iteratePrefixes169,6916
    fn reduce<T: Clone>(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce186,7523
    fn scan<T: Clone>(scan201,8261
    fn flatten<T: Clone>(ss: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T> {flatten223,9134
    fn collect<A: Clone + Eq, Bv: Clone>(collect239,9815

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEph.rs,2037
pub struct ArraySeqEphS<T> {ArraySeqEphS7,210
    pub data: Box<[T]>,data8,239
pub trait ArraySeqEphTrait<T> {ArraySeqEphTrait12,308
    fn new(length: N, init_value: T) -> Selfnew14,377
    fn length(&self) -> N;length18,482
    fn nth(&self, index: N) -> &T;nth20,541
    fn empty() -> Self;empty22,608
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set24,664
    fn singleton(item: T) -> Self;singleton26,773
    fn isEmpty(&self) -> B;isEmpty28,840
    fn isSingleton(&self) -> B;isSingleton30,900
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy32,975
impl<T> ArraySeqEphS<T> {ArraySeqEphS37,1066
    pub fn subseq(&self, start: N, length: N) -> &[T] {subseq38,1092
    pub fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy44,1295
    pub fn from_vec(v: Vec<T>) -> Self {from_vec63,1910
    pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqEphS<T> {update68,2030
    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter75,2212
impl<T: Eq> PartialEq for ArraySeqEphS<T> {ArraySeqEphS78,2286
    fn eq(&self, other: &Self) -> bool {eq79,2330
impl<T: Eq> Eq for ArraySeqEphS<T> {}ArraySeqEphS91,2608
impl<T: std::fmt::Debug> std::fmt::Debug for ArraySeqEphS<T> {ArraySeqEphS93,2647
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt94,2710
impl<T> ArraySeqEphTrait<T> for ArraySeqEphS<T> {ArraySeqEphS100,2898
    fn new(length: N, init_value: T) -> Selfnew101,2948
    fn length(&self) -> N {length107,3090
    fn nth(&self, index: N) -> &T {nth110,3148
    fn empty() -> Self {empty113,3216
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set116,3290
    fn singleton(item: T) -> Self {singleton124,3535
    fn isEmpty(&self) -> B {isEmpty127,3620
    fn isSingleton(&self) -> B {isSingleton134,3757
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy141,3898
macro_rules! ArraySeqEph {ArraySeqEph150,4057

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEphChap18.rs,2254
pub trait ArraySeqEphChap18Trait {ArraySeqEphChap18Trait6,135
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T>;tabulate7,170
    fn map<T, U: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U>;map8,235
    fn append<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;append9,321
    fn filter<T: Clone + Eq>(a: &ArraySeqEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqEphS<T>;filter10,412
    fn update<T: Clone + Eq>(a: &mut ArraySeqEphS<T>, item_at: (N, T)) -> &mut ArraySeqEphS<T>;update11,506
    fn inject<T: Clone + Eq>(inject12,602
    fn ninject<T: Clone + Eq>(ninject16,727
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> Aiterate20,853
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(iteratePrefixes21,951
    fn reduce<T: Clone + Eq>(a: &ArraySeqEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce26,1107
    fn scan<T: Clone + Eq>(scan27,1196
    fn flatten<T: Clone + Eq>(ss: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T>;flatten32,1333
    fn collect<A: Clone + Eq, Bv: Clone>(collect33,1419
impl<T2> ArraySeqEphChap18Trait for ArraySeqEphS<T2> {ArraySeqEphS39,1580
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T> {tabulate40,1635
    fn map<T, U: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U> {map47,1852
    fn append<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {append61,2327
    fn filter<T: Clone + Eq>(a: &ArraySeqEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqEphS<T> {filter82,2998
    fn update<T: Clone + Eq>(update94,3384
    fn inject<T: Clone + Eq>(inject100,3548
    fn ninject<T: Clone + Eq>(ninject111,3867
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> Aiterate117,4033
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(iteratePrefixes124,4255
    fn reduce<T: Clone + Eq>(a: &ArraySeqEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce141,4798
    fn scan<T: Clone + Eq>(scan148,5012
    fn flatten<T: Clone + Eq>(ss: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T> {flatten166,5543
    fn collect<A: Clone + Eq, Bv: Clone>(collect194,6518

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,273
pub mod Types {Types7,211
    pub type N = usize;N15,515
    pub enum B {B19,658
        True,True20,675
        False,False21,689
    impl std::fmt::Display for B {B28,922
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt29,957

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEphChap19.rs,2343
pub trait ArraySeqEphChap19Trait {ArraySeqEphChap19Trait7,186
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T>;tabulate8,221
    fn map<T, U: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U>;map9,286
    fn select<'a, T: Clone>(a: &'a ArraySeqEphS<T>, b: &'a ArraySeqEphS<T>, i: N) -> Option<T>;select10,372
    fn append<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;append11,468
    fn append2<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;append212,559
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArraySeqEphS<T>;deflate13,651
    fn filter<T: Clone + Eq>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqEphS<T>;filter14,724
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> Aiterate15,815
    fn reduce<T: Clone + Eq, F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> Treduce16,913
    fn scan<T: Clone + Eq, F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> (ArraySeqEphS<T>, T)scan19,1023
    fn flatten<T: Clone + Eq>(s: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T>;flatten22,1150
impl<T2> ArraySeqEphChap19Trait for ArraySeqEphS<T2> {ArraySeqEphS25,1238
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T> {tabulate26,1293
    fn map<T, U: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U> {map29,1433
    fn select<'a, T: Clone>(a: &'a ArraySeqEphS<T>, b: &'a ArraySeqEphS<T>, i: N) -> Option<T> {select32,1590
    fn append<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {append44,1952
    fn append2<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {append247,2117
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArraySeqEphS<T> {deflate50,2283
    fn filter<T: Clone + Eq>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqEphS<T> {filter57,2556
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> Aiterate60,2721
    fn reduce<T: Clone + Eq, F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> Treduce63,2897
    fn scan<T: Clone + Eq, F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> (ArraySeqEphS<T>, T)scan69,3090
    fn flatten<T: Clone + Eq>(s: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T> {flatten75,3298

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListEphChap19.rs,2739
pub trait LinkedListEphChap19Trait {LinkedListEphChap19Trait7,207
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T>;tabulate8,244
    fn map<T, U: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U>;map9,311
    fn select<'a, T: Clone>(a: &'a LinkedListEphS<T>, b: &'a LinkedListEphS<T>, i: N) -> Option<select10,401
    fn append<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;append11,501
    fn append2<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;append212,593
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListEphS<T>;deflate13,686
    fn filter<T: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> B) -> LinkedListEphS<T>;filter14,761
    fn iterate<T: Clone, A: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate15,851
    fn reduce<T: Clone, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;reduce16,946
    fn scan<T: Clone, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> (LinkedListEphS<T>, T) where F:scan17,1041
    fn flatten<T: Clone>(s: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T>;flatten18,1155
    fn inject<T: Clone + Eq>(values: &LinkedListEphS<T>, changes: &LinkedListEphS<(N, T)>) -> Liinject19,1241
impl<T2> LinkedListEphChap19Trait for LinkedListEphS<T2> {LinkedListEphS22,1357
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T> { <LinkedListEphS<T> as Linkedtabulate23,1416
    fn map<T, U: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U> { <Linkmap24,1550
    fn select<'a, T: Clone>(a: &'a LinkedListEphS<T>, b: &'a LinkedListEphS<T>, i: N) -> Option<select25,1703
    fn append<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> { <Liappend32,2197
    fn append2<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> { <Lappend233,2355
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListEphS<T> { if f(x) == B::True {deflate34,2514
    fn filter<T: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> B) -> LinkedListEphS<T> { <Linkfilter35,2745
    fn iterate<T: Clone, A: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { iterate36,2901
    fn reduce<T: Clone, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T { reduce37,3066
    fn scan<T: Clone, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> (LinkedListEphS<T>, T) where F:scan38,3231
    fn flatten<T: Clone>(s: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T> { <LinkedLiflatten39,3413
    fn inject<T: Clone + Eq>(values: &LinkedListEphS<T>, changes: &LinkedListEphS<(N, T)>) -> Liinject40,3563

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main7,236

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/SelfChanges.txt,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPerChap19.rs,1199
pub trait AVLTreeSeqPerChap19Trait {AVLTreeSeqPerChap19Trait8,231
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>tabulate9,268
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>map12,385
    fn select<T>(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>, i: N) -> Option<T>select16,559
    fn append<T: Ord + Copy + Debug + Display>(append19,692
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqPerS<T>deflate23,830
    fn filter<T: Ord + Copy + Debug + Display>(filter24,928
impl<T2: Ord + Copy + Debug + Display> AVLTreeSeqPerChap19Trait for AVLTreeSeqPerS<T2> {AVLTreeSeqPerS30,1067
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>tabulate31,1156
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>map37,1357
    fn select<T>(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>, i: N) -> Option<T>select44,1610
    fn append<T: Ord + Copy + Debug + Display>(append61,2237
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqPerS<T>deflate67,2452
    fn filter<T: Ord + Copy + Debug + Display>(filter74,2751

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,914
pub mod Types;Types7,226
pub mod MathSeq;MathSeq10,282
pub mod LinkedListPer;LinkedListPer16,337
pub mod LinkedListPerChap18;LinkedListPerChap1818,428
pub mod LinkedListPerChap19;LinkedListPerChap1920,519
pub mod LinkedListEph;LinkedListEph23,611
pub mod LinkedListEphChap18;LinkedListEphChap1825,702
pub mod LinkedListEphChap19;LinkedListEphChap1927,793
pub mod ArraySeqPer;ArraySeqPer30,885
pub mod ArraySeqPerChap18;ArraySeqPerChap1833,966
pub mod ArraySeqPerChap19;ArraySeqPerChap1936,1052
pub mod FooArraySeqPer;FooArraySeqPer39,1138
pub mod ArraySeqEph;ArraySeqEph41,1163
pub mod ArraySeqEphChap18;ArraySeqEphChap1844,1247
pub mod ArraySeqEphChap19;ArraySeqEphChap1946,1332
pub mod AVLTreeSeqPer;AVLTreeSeqPer49,1418
pub mod AVLTreeSeqPerChap18;AVLTreeSeqPerChap1851,1509
pub mod AVLTreeSeqPerChap19;AVLTreeSeqPerChap1953,1600
pub mod AVLTreeSeqEph;AVLTreeSeqEph57,1752

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqEph.rs,4139
type Link<T> = Option<Box<AVLTreeNode<T>>>;Link7,172
pub struct AVLTreeNode<T: Copy + Debug> {AVLTreeNode9,217
    pub value: T,value10,259
    pub height: N,height11,277
    pub left_size: N,left_size12,296
    pub right_size: N,right_size13,318
    pub left: Link<T>,left14,341
    pub right: Link<T>,right15,364
    pub index: N,index16,388
impl<T: Copy + Debug> AVLTreeNode<T> {AVLTreeNode19,409
    fn new(value: T, index: N) -> Self {new20,448
pub struct AVLTreeSeqEphS<T: Copy + Debug> {AVLTreeSeqEphS33,693
    pub root: Link<T>,root34,738
    pub next_key: N,next_key35,761
pub trait AVLTreeSeqEphTrait<T: Copy + Debug> {AVLTreeSeqEphTrait38,785
    fn empty() -> Self;empty40,871
    fn new() -> Self;new42,933
    fn length(&self) -> N;length44,993
    fn nth(&self, index: N) -> &T;nth46,1066
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set48,1147
    fn singleton(item: T) -> Self;singleton50,1262
    fn isEmpty(&self) -> B;isEmpty52,1335
    fn isSingleton(&self) -> B;isSingleton54,1401
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy56,1491
impl<T: Copy + Debug> AVLTreeSeqEphS<T> {AVLTreeSeqEphS61,1582
    pub fn new_root() -> Self {new_root62,1624
    pub fn new() -> Self {new68,1746
    pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqEphS<T> {update71,1804
    pub fn from_vec(values: Vec<T>) -> AVLTreeSeqEphS<T>from_vec75,1989
    pub fn to_arrayseq(&self) -> ArraySeqEphS<T>to_arrayseq87,2364
    pub fn iter<'a>(&'a self) -> AVLTreeSeqIterEph<'a, T> {iter109,3009
    pub fn push_back(&mut self, value: T) {push_back112,3118
    pub fn contains_value(&self, target: &T) -> Bcontains_value117,3312
    pub fn insert_value(&mut self, value: T) {insert_value128,3539
    pub fn delete_value(&mut self, target: &T) -> booldelete_value131,3623
impl<T: Copy + Debug> AVLTreeSeqEphTrait<T> for AVLTreeSeqEphS<T> {AVLTreeSeqEphS159,4387
    fn empty() -> Self {empty160,4455
    fn new() -> Self {new164,4522
    fn length(&self) -> N {length168,4587
    fn nth(&self, index: N) -> &T {nth172,4652
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set176,4731
    fn singleton(item: T) -> Self {singleton181,4881
    fn isEmpty(&self) -> B {isEmpty187,5056
    fn isSingleton(&self) -> B {isSingleton195,5192
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy203,5332
pub struct AVLTreeSeqIterEph<'a, T: Copy + Debug> {AVLTreeSeqIterEph221,5828
    stack: Vec<&'a AVLTreeNode<T>>,stack222,5880
    current: Option<&'a AVLTreeNode<T>>,current223,5916
impl<'a, T: Copy + Debug> AVLTreeSeqIterEph<'a, T> {AVLTreeSeqIterEph226,5960
    fn new(root: &'a Link<T>) -> Self {new227,6013
    fn push_left(&mut self, link: &'a Link<T>) {push_left235,6208
impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIterEph<'a, T> {AVLTreeSeqIterEph244,6428
    type Item = &'a T;Item245,6494
    fn next(&mut self) -> Option<Self::Item> {next246,6517
fn h<T: Copy + Debug>(n: &Link<T>) -> N {h254,6713
fn size_link<T: Copy + Debug>(n: &Link<T>) -> N {size_link258,6797
fn update_meta<T: Copy + Debug>(n: &mut Box<AVLTreeNode<T>>) {update_meta266,6943
fn rotate_right<T: Copy + Debug>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right274,7169
fn rotate_left<T: Copy + Debug>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left285,7501
fn rebalance<T: Copy + Debug>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance296,7831
pub(crate) fn insert_at_link<T: Copy + Debug>(insert_at_link317,8525
fn nth_link<'a, T: Copy + Debug>(node: &'a Link<T>, index: N) -> &'a T {nth_link342,9269
fn set_link<T: Copy + Debug>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str>set_link354,9614
macro_rules! AVLTreeSeqEph {AVLTreeSeqEph372,10161
impl<T: Eq + Copy + Debug> PartialEq for AVLTreeSeqEphS<T> {AVLTreeSeqEphS387,10606
    fn eq(&self, other: &Self) -> bool {eq388,10667
impl<T: Eq + Copy + Debug> Eq for AVLTreeSeqEphS<T> {}AVLTreeSeqEphS401,10946

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListEphChap18.rs,2941
pub trait LinkedListEphChap18Trait {LinkedListEphChap18Trait7,183
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T>;tabulate8,220
    fn map<T, U: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U>;map9,287
    fn append<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;append10,377
    fn filter<T: Clone>(a: &LinkedListEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListEphS<T>;filter11,469
    fn update<T: Clone>(a: &mut LinkedListEphS<T>, item_at: (N, T)) -> &mut LinkedListEphS<T>;update12,562
    fn inject<T: Clone + Eq>(a: &LinkedListEphS<T>, updates: &LinkedListEphS<(N, T)>) -> LinkedLinject13,657
    fn ninject<T: Clone + Eq>(a: &LinkedListEphS<T>, updates: &LinkedListEphS<(N, T)>) -> Linkedninject14,765
    fn iterate<T: Clone, A: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate15,874
    fn iteratePrefixes<T: Clone, A: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A)iteratePrefixes16,969
    fn reduce<T: Clone>(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,1093
    fn scan<T: Clone>(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListEphSscan18,1179
    fn flatten<T: Clone>(ss: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T>;flatten19,1284
    fn collect<A: Clone + Eq, Bv: Clone>(a: &LinkedListEphS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O)collect20,1371
impl<T2> LinkedListEphChap18Trait for LinkedListEphS<T2> {LinkedListEphS23,1515
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T> {tabulate24,1574
    fn map<T, U: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U> {map29,1775
    fn append<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> {append34,2161
    fn filter<T: Clone>(a: &LinkedListEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListEphS<T> {filter42,2701
    fn update<T: Clone>(a: &mut LinkedListEphS<T>, item_at: (N, T)) -> &mut LinkedListEphS<T> { update50,3099
    fn inject<T: Clone + Eq>(a: &LinkedListEphS<T>, updates: &LinkedListEphS<(N, T)>) -> LinkedLinject51,3262
    fn ninject<T: Clone + Eq>(a: &LinkedListEphS<T>, updates: &LinkedListEphS<(N, T)>) -> Linkedninject68,4064
    fn iterate<T: Clone, A: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate81,4716
    fn iteratePrefixes<T: Clone, A: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A)iteratePrefixes84,4991
    fn reduce<T: Clone>(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce90,5433
    fn scan<T: Clone>(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListEphSscan101,6129
    fn flatten<T: Clone>(ss: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T> {flatten109,6757
    fn collect<A: Clone + Eq, Bv: Clone>(a: &LinkedListEphS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O)collect119,7355

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPerChap18.rs,781
pub trait AVLTreeSeqPerChap18Trait {AVLTreeSeqPerChap18Trait7,170
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>tabulate9,291
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>map13,495
    fn append<T: Ord + Copy + Debug + Display>(append18,739
    fn filter<T: Ord + Copy + Debug + Display>(filter23,970
impl<T2: Ord + Copy + Debug + Display> AVLTreeSeqPerChap18Trait for AVLTreeSeqPerS<T2> {AVLTreeSeqPerS29,1112
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>tabulate30,1201
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>map40,1507
    fn append<T: Ord + Copy + Debug + Display>(append49,1915
    fn filter<T: Ord + Copy + Debug + Display>(filter62,2413

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPer.rs,3828
type Link<T> = Option<Rc<Node<T>>>;Link8,210
struct Node<T: Copy + Debug> {Node10,247
    value: T,value11,278
    height: N,height12,292
    size: N,size13,307
    left: Link<T>,left14,320
    right: Link<T>,right15,339
fn height<T: Copy + Debug>(n: &Link<T>) -> N {height18,362
fn size<T: Copy + Debug>(n: &Link<T>) -> N {size21,450
fn mk<T: Copy + Debug>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {mk25,535
fn rotate_right<T: Copy + Debug>(y: Rc<Node<T>>) -> Rc<Node<T>> {rotate_right38,841
fn rotate_left<T: Copy + Debug>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left45,1117
fn rebalance<T: Copy + Debug>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance56,1428
fn nth_ref<'a, T: Copy + Debug>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref78,2190
fn set_rec<T: Copy + Debug>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> set_rec93,2592
fn inorder_collect<T: Copy + Debug>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect111,3285
fn build_balanced_from_slice<T: Copy + Debug>(a: &[T]) -> Link<T> {build_balanced_from_slice119,3498
    fn rec<T: Copy + Debug>(a: &[T]) -> Link<T> {rec120,3566
pub struct AVLTreeSeqPerS<T: Copy + Debug> {AVLTreeSeqPerS132,3841
    root: Link<T>,root133,3886
pub trait AVLTreeSeqPerTrait<T: Copy + Debug> {AVLTreeSeqPerTrait136,3908
    fn empty() -> Self;empty138,3993
    fn new() -> Self;new140,4054
    fn length(&self) -> N;length142,4113
    fn nth(&self, index: N) -> &T;nth144,4185
    fn set(&self, index: N, item: T) -> Result<Self, &'static str> where Self: Sized;set146,4324
    fn singleton(item: T) -> Self;singleton148,4447
    fn isEmpty(&self) -> B;isEmpty150,4519
    fn isSingleton(&self) -> B;isSingleton152,4584
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy154,4669
    fn from_vec(values: Vec<T>) -> Selffrom_vec158,4816
    fn values_in_order(&self) -> Vec<T>values_in_order162,4924
impl<T: Copy + Debug> AVLTreeSeqPerTrait<T> for AVLTreeSeqPerS<T> {AVLTreeSeqPerS167,4995
    fn empty() -> Self {empty168,5063
    fn new() -> Self {new171,5132
    fn length(&self) -> N {length174,5210
    fn nth(&self, index: N) -> &T {nth177,5269
    fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set180,5346
    fn singleton(item: T) -> Self {singleton185,5513
    fn isEmpty(&self) -> B {isEmpty190,5636
    fn isSingleton(&self) -> B {isSingleton197,5771
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy204,5910
    fn from_vec(values: Vec<T>) -> Selffrom_vec221,6477
    fn values_in_order(&self) -> Vec<T>values_in_order229,6650
impl<T: Eq + Copy + Debug> PartialEq for AVLTreeSeqPerS<T> {AVLTreeSeqPerS239,6849
    fn eq(&self, other: &Self) -> bool {eq240,6910
impl<T: Eq + Copy + Debug> Eq for AVLTreeSeqPerS<T> {}AVLTreeSeqPerS252,7188
impl<T: Debug + Copy> std::fmt::Debug for AVLTreeSeqPerS<T> {AVLTreeSeqPerS254,7244
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt255,7306
impl<T: Copy + Debug> AVLTreeSeqPerS<T> {AVLTreeSeqPerS261,7510
    pub fn to_arrayseq(&self) -> ArrayPerS<T>to_arrayseq262,7552
    pub fn iter<'a>(&'a self) -> AVLTreeSeqPerIter<'a, T> {iter270,7742
pub struct AVLTreeSeqPerIter<'a, T: Copy + Debug> {AVLTreeSeqPerIter275,7890
    stack: Vec<&'a Node<T>>,stack276,7942
    current: Option<&'a Node<T>>,current277,7971
impl<'a, T: Copy + Debug> AVLTreeSeqPerIter<'a, T> {AVLTreeSeqPerIter280,8008
    fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {push_left281,8061
impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqPerIter<'a, T> {AVLTreeSeqPerIter289,8243
    type Item = &'a T;Item290,8309
    fn next(&mut self) -> Option<Self::Item> {next291,8332
macro_rules! AVLTreeSeqPer {AVLTreeSeqPer304,8676

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/FooArraySeqPer.rs,1882
pub struct FooArrayPerS<T> { pub data: Box<[T]> }FooArrayPerS12,478
pub struct FooArrayPerS<T> { pub data: Box<[T]> }data12,478
pub trait FooArraySeqPerTrait<T> {FooArraySeqPerTrait15,597
    fn new(length: N, init_value: T) -> FooArrayPerS<T> where T: Clone;new16,632
    fn new2(length: N, init_value: T) -> Self where T: Clone, Self: Sized;new217,704
    fn set(&self, index: N, item: T) -> Result<FooArrayPerS<T>, &'static str> where T: Clone;set18,779
    fn set2(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone, Self: Sized;set219,873
    fn singleton(item: T) -> Self where Self: Sized;singleton20,970
    fn empty() -> Self where Self: Sized;empty21,1023
impl<T> FooArrayPerS<T> {FooArrayPerS24,1068
    pub fn subseq(&self, start: N, length: N) -> &[T] {subseq25,1094
    pub fn from_vec(v: Vec<T>) -> FooArrayPerS<T> { FooArrayPerS { data: v.into_boxed_slice() } from_vec31,1297
    pub fn new(length: N, init_value: T) -> Self where T: Clone { Self::from_vec(vec![init_valuenew32,1395
    pub fn new_inherent(length: N, init_value: T) -> Self where T: Clone { Self::from_vec(vec![inew_inherent33,1504
impl<T> FooArraySeqPerTrait<T> for FooArrayPerS<T> {FooArrayPerS37,1626
    fn new(length: N, init_value: T) -> FooArrayPerS<T> where T: Clone {new38,1679
    fn new2(length: N, init_value: T) -> FooArrayPerS<T> where T: Clone {new241,1815
    fn empty() -> Self { FooArrayPerS::from_vec(Vec::new()) }empty44,1952
    fn set(&self, index: N, item: T) -> Result<FooArrayPerS<T>, &'static str> where T: Clone {set45,2014
    fn singleton(item: T) -> FooArrayPerS<T> { FooArrayPerS::from_vec(vec![item]) }singleton51,2301
    fn set2(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {set253,2386
pub fn foo_new2<T: Clone>(length: N, init_value: T) -> FooArrayPerS<T> {foo_new263,2747

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListEph.rs,2681
pub struct NodeE<T> {NodeE6,132
    pub value: T,value7,154
    pub next: Option<Box<NodeE<T>>>,next8,172
pub struct LinkedListEphS<T> {LinkedListEphS12,229
    head: Option<Box<NodeE<T>>>,head13,260
    len: N,len14,293
pub trait LinkedListEphTrait<T> {LinkedListEphTrait17,308
    fn empty() -> LinkedListEphS<T>;empty18,342
    fn new(length: N, init_value: T) -> Selfnew19,379
    fn length(&self) -> N;length22,452
    fn nth(&self, index: N) -> &T;nth23,479
    fn isEmpty(&self) -> B;isEmpty24,514
    fn isSingleton(&self) -> B;isSingleton25,542
    fn singleton(item: T) -> Self;singleton26,574
    fn update(&mut self, item_at: (N, T)) -> &mut Self;update27,609
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set28,665
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy29,742
impl<T> LinkedListEphS<T> {LinkedListEphS34,828
    fn push_front_node(&mut self, node: Box<NodeE<T>>) {push_front_node35,856
    pub fn from_vec(v: Vec<T>) -> Self {from_vec42,1033
    pub fn iter<'a>(&'a self) -> LinkedListEphIter<'a, T> {iter50,1268
impl<T> LinkedListEphTrait<T> for LinkedListEphS<T> {LinkedListEphS55,1396
    fn empty() -> Self {empty56,1450
    fn new(length: N, init_value: T) -> Selfnew59,1527
    fn length(&self) -> N {length81,2162
    fn nth(&self, index: N) -> &T {nth84,2213
    fn isEmpty(&self) -> B {isEmpty96,2532
    fn isSingleton(&self) -> B {isSingleton103,2662
    fn singleton(item: T) -> Self {singleton110,2796
    fn update(&mut self, (index, item): (N, T)) -> &mut Self {update119,3007
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set132,3350
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy145,3740
impl<T: std::fmt::Debug> std::fmt::Debug for LinkedListEphS<T> {LinkedListEphS184,4810
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt185,4875
pub struct LinkedListEphIter<'a, T> {LinkedListEphIter196,5201
    cursor: Option<&'a NodeE<T>>,cursor197,5239
impl<'a, T> Iterator for LinkedListEphIter<'a, T> {LinkedListEphIter200,5276
    type Item = &'a T;Item201,5328
    fn next(&mut self) -> Option<Self::Item> {next202,5351
impl<T: PartialEq> PartialEq for LinkedListEphS<T> {LinkedListEphS210,5542
    fn eq(&self, other: &Self) -> bool {eq211,5595
impl<T: Eq> Eq for LinkedListEphS<T> {}LinkedListEphS228,6020
impl<T: std::fmt::Display> std::fmt::Display for LinkedListEphS<T> {LinkedListEphS230,6061
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt231,6130
macro_rules! LinkedListEph {LinkedListEph245,6534

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPer.rs,2234
pub struct ArrayPerS<T> { pub data: Box<[T]> }ArrayPerS12,478
pub struct ArrayPerS<T> { pub data: Box<[T]> }data12,478
pub trait ArraySeqPerTrait<T> {ArraySeqPerTrait15,591
    fn new(length: N, init_value: T) -> Self where T: Clone;new17,665
    fn length(&self) -> N;length19,763
    fn nth(&self, index: N) -> &T;nth21,827
    fn empty() -> Self;empty23,899
    fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone, Self: Sized;set27,1134
    fn singleton(item: T) -> Self;singleton29,1267
    fn isEmpty(&self) -> B;isEmpty31,1339
    fn isSingleton(&self) -> B;isSingleton33,1404
    fn subseq_copy(&self, start: N, length: N) -> Self where T: Clone + Eq, Self: Sized;subseq_copy35,1478
impl<T> ArrayPerS<T> {ArrayPerS38,1570
    pub fn subseq(&self, start: N, length: N) -> &[T] {subseq40,1630
    pub fn from_vec(v: Vec<T>) -> Self { ArrayPerS { data: v.into_boxed_slice() } }from_vec49,1961
    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter51,2046
impl<T: Eq> PartialEq for ArrayPerS<T> {ArrayPerS54,2120
    fn eq(&self, other: &Self) -> bool {eq55,2161
impl<T: Eq> Eq for ArrayPerS<T> {}ArrayPerS61,2371
impl<T: std::fmt::Debug> std::fmt::Debug for ArrayPerS<T> {ArrayPerS63,2407
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt64,2467
impl<T> ArraySeqPerTrait<T> for ArrayPerS<T> {ArrayPerS70,2655
    fn new(length: N, init_value: T) -> Self where T: Clone {new71,2702
    fn length(&self) -> N { self.data.len() }length74,2819
    fn nth(&self, index: N) -> &T { &self.data[index] }nth75,2865
    fn empty() -> Self { Self::from_vec(Vec::new()) }empty76,2921
    fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {set77,2975
    fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton83,3243
    fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }isEmpty84,3308
    fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton85,3393
    fn subseq_copy(&self, start: N, length: N) -> Self where T: Clone + Eq {subseq_copy86,3482
macro_rules! ArraySeqPer {ArraySeqPer98,3847

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqEphChap18.rs,919
pub trait AVLTreeSeqChap18Trait {AVLTreeSeqChap18Trait14,539
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeS<T>tabulate17,712
    fn map<T, U>(a: &AVLTreeS<T>, f: impl Fn(&T) -> U) -> AVLTreeS<U>map23,983
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeS<T>, b: &AVLTreeS<T>) -> AVLTreeS<T>;append30,1253
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeS<T>, pred: impl Fn(&T) -> B) -> AVLTrfilter34,1491
impl<T2: Ord + Copy + Debug + Display> AVLTreeSeqChap18 for AVLTreeS<T2> {AVLTreeS37,1598
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeS<T>tabulate39,1757
    fn map<T, U>(a: &AVLTreeS<T>, f: impl Fn(&T) -> U) -> AVLTreeS<U>map49,2103
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeS<T>, b: &AVLTreeS<T>) -> AVLTreeS<T> append60,2490
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeS<T>, pred: impl Fn(&T) -> B) -> AVLTrfilter68,2921

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/12_TestArraySeqEph.rs,223
fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic8,235
fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter17,461
fn test_iterators_collect() {test_iterators_collect27,932

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/08_TestLinkedListEphChap19.rs,454
fn test_eph_set_and_nth() {test_eph_set_and_nth7,170
fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug14,321
fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1924,632
fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1931,830
fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1941,1423

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/07_TestLinkedListEphChap18.rs,742
fn test_construct_eph_from_vec() {test_construct_eph_from_vec9,277
fn test_eph_is_empty_and_singleton() {test_eph_is_empty_and_singleton15,410
fn test_eph_set_and_subseq_copy() {test_eph_set_and_subseq_copy23,657
fn test_iter_inorder_collect_eph_ch18() {test_iter_inorder_collect_eph_ch1832,884
fn test_tabulate_and_map_ch18() {test_tabulate_and_map_ch1839,1082
fn test_append_ch18() {test_append_ch1847,1422
fn test_filter_ch18() {test_filter_ch1856,1751
fn test_update_ch18() {test_update_ch1864,2054
fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1872,2355
fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1882,2854
fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch1894,3406

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/05_TestLinkedListPerChap19.rs,345
fn test_select() {test_select8,251
fn test_append_variants() {test_append_variants21,692
fn test_deflate() {test_deflate31,1059
fn test_map() {test_map39,1421
fn test_iterate_and_reduce() {test_iterate_and_reduce46,1610
fn test_scan() {test_scan55,1927
fn test_flatten() {test_flatten63,2174
fn test_inject() {test_inject73,2430

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/01_TestTypes.rs,677
pub mod TestTypes {TestTypes1,0
    fn test_boolean_eq_neq_and_ordering() {test_boolean_eq_neq_and_ordering5,67
    fn test_ordering_on_n_naturals() {test_ordering_on_n_naturals15,324
    fn test_cmp_on_b_returns_expected_ordering_variants() {test_cmp_on_b_returns_expected_ordering_variants24,571
    fn test_btree_set_orders_b_true_before_false() {test_btree_set_orders_b_true_before_false32,895
    fn test_n_aliases_usize_and_cmp_examples() {test_n_aliases_usize_and_cmp_examples42,1195
    fn test_debug_format_for_b_variants() {test_debug_format_for_b_variants57,1648
    fn test_display_format_for_b_variants() {test_display_format_for_b_variants63,1821

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/11_TestArraySeqPerChap19.rs,283
fn test_map_and_select_and_append() {test_map_and_select_and_append7,177
fn test_deflate_and_filter() {test_deflate_and_filter17,628
fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten26,1079
fn test_inject_and_parallel() {test_inject_and_parallel45,1962

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Problem21_1.rs,418
fn points2d(n: N) -> ArrayPerS<(N, N)> {points2d8,256
fn test_points2d_n3_example() {test_points2d_n3_example21,562
fn test_points2d_n1_empty() {test_points2d_n1_empty29,794
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values35,891
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order43,1065
fn test_points2d_debug_display_shapes() {test_points2d_debug_display_shapes51,1333

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/10_TestArraySeqPerChap18.rs,884
fn test_tabulate_fibonacci() {test_tabulate_fibonacci6,134
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }fib7,165
fn test_map_increment() {test_map_increment18,606
fn test_subseq() {test_subseq25,806
fn test_append() {test_append36,1217
fn test_sequence_literals_and_append() {test_sequence_literals_and_append44,1438
fn test_filter_even() {test_filter_even57,1999
fn test_flatten() {test_flatten67,2479
fn test_update_sequence() {test_update_sequence81,3073
fn test_inject_and_ninject() {test_inject_and_ninject91,3506
fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan110,4613
fn test_iterate_sum_basic() {test_iterate_sum_basic129,5505
fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum137,5739
fn test_collect_groups_by_key() {test_collect_groups_by_key149,6146

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercsise_21_9.rs,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/14_TestArraySeqEphChap19.rs,1272
fn test_empty() {test_empty8,254
fn test_singleton() {test_singleton14,377
fn test_map() {test_map20,533
fn test_append() {test_append27,767
fn test_append2() {test_append235,1080
fn test_deflate_true() {test_deflate_true43,1395
fn test_deflate_false() {test_deflate_false49,1591
fn test_filter_even_numbers() {test_filter_even_numbers55,1783
fn test_filter_none() {test_filter_none62,2084
fn test_update_in_bounds() {test_update_in_bounds69,2370
fn test_update_out_of_bounds() {test_update_out_of_bounds76,2626
fn test_isEmpty() {test_isEmpty83,2879
fn test_isSingleton() {test_isSingleton93,3234
fn test_iterate_sum() {test_iterate_sum103,3605
fn test_iterate_concat() {test_iterate_concat110,3847
fn test_map_empty() {test_map_empty124,4226
fn test_append_with_empty() {test_append_with_empty131,4461
fn test_append2_equivalence() {test_append2_equivalence141,4921
fn test_filter_empty_sequence() {test_filter_empty_sequence150,5299
fn test_select_boundary() {test_select_boundary157,5551
fn test_subseq_basic() {test_subseq_basic168,6194
fn test_reduce_sum_basic_ch19() {test_reduce_sum_basic_ch19175,6440
fn test_scan_sum_basic_ch19() {test_scan_sum_basic_ch19189,7092
fn test_flatten_ch19() {test_flatten_ch19200,7490

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Problem_21_4.rs,988
fn cartesian_loops(a: &ArrayPerS<N>, b: &ArrayPerS<&'static str>) -> ArrayPerS<(N, &'static str)cartesian_loops10,338
    let mut v: Vec<(N, &'static str)> = Vec::with_capacity(a.length() * b.length());str11,438
fn cartesian_tab_flat(a: &ArrayPerS<N>, b: &ArrayPerS<&'static str>) -> ArrayPerS<(N, &'static scartesian_tab_flat22,808
    let nested: ArrayPerS<ArrayPerS<(N, &'static str)>> =str23,911
        <ArrayPerS<ArrayPerS<(N, &'static str)>> as ArraySeqPerChap19Trait>::map(str24,969
            |x| <ArrayPerS<(N, &'static str)> as ArraySeqPerChap19Trait>::map(b, |y| (*x, *y)),str26,1066
    <ArrayPerS<(N, &'static str)> as ArraySeqPerChap18Trait>::flatten(&nested)str28,1173
fn test_cartesian_loops_basic() {test_cartesian_loops_basic32,1263
fn test_cartesian_tab_flat_basic() {test_cartesian_tab_flat_basic41,1593
fn test_cartesian_iterator_order() {test_cartesian_iterator_order50,1929
fn test_cartesian_debug_shape() {test_cartesian_debug_shape59,2233

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/06_TestLinkedListEph.rs,466
fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates7,187
fn test_new_and_nth_set() {test_new_and_nth_set16,473
fn test_subseq_copy() {test_subseq_copy25,674
fn test_linkedlisteph_basic() {test_linkedlisteph_basic34,875
fn test_debug_format_for_eph() {test_debug_format_for_eph43,1081
fn test_display_format_for_eph() {test_display_format_for_eph49,1211
fn test_iter_inorder_collect_eph() {test_iter_inorder_collect_eph55,1341

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/17_TestAVLTreeSeqPerChap19.rs,301
fn test_tabulate_and_map_ch19() {test_tabulate_and_map_ch197,155
fn test_select_and_append_ch19() {test_select_and_append_ch1915,479
fn test_deflate_and_filter_ch19() {test_deflate_and_filter_ch1927,1183
fn test_iter_inorder_after_pipeline_ch19() {test_iter_inorder_after_pipeline_ch1938,1758

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Problem_21_3.rs,475
fn points3d_loops(n: N) -> ArrayPerS<(N, N, N)> {points3d_loops8,311
fn test_points3d_loops_n0_empty() {test_points3d_loops_n0_empty23,682
fn test_points3d_loops_n1_single() {test_points3d_loops_n1_single29,791
fn test_points3d_loops_n2_values_and_order() {test_points3d_loops_n2_values_and_order36,952
fn test_points3d_loops_iterator_order() {test_points3d_loops_iterator_order49,1250
fn test_points3d_loops_debug_shape() {test_points3d_loops_debug_shape56,1497

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/19_TestAVLTreeSeqEphChap18.rs,209
fn test_tabulate_inorder() {test_tabulate_inorder10,263
fn test_map_increment() {test_map_increment16,427
fn test_append_union() {test_append_union24,728
fn test_filter_even() {test_filter_even34,1116

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/FooTestArraySeqPer.rs,167
fn test_new_and_set() {test_new_and_set7,209
fn test_singleton() {test_singleton17,653
fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton22,759

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/04_TestLinkedListPerChap18.rs,466
fn test_tabulate() {test_tabulate7,194
fn test_map() {test_map14,391
fn test_filter() {test_filter22,682
fn test_append() {test_append29,1002
fn test_update() {test_update37,1298
fn test_inject() {test_inject44,1525
fn test_ninject() {test_ninject52,1828
fn test_iterate() {test_iterate60,2135
fn test_reduce() {test_reduce67,2349
fn test_scan() {test_scan74,2558
fn test_flatten() {test_flatten82,2836
fn test_collect() {test_collect93,3164

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/18_TestAVLTreeSeqEph.rs,63
fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic6,133

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/09_TestArraySeqPer.rs,1704
fn test_new_and_set() {test_new_and_set8,203
fn test_length_and_nth_basic() {test_length_and_nth_basic22,675
fn test_empty() {test_empty30,854
fn test_sequence_basic() {test_sequence_basic37,1038
fn test_singleton() {test_singleton50,1656
fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton58,1859
fn test_from_vec() {test_from_vec73,2325
fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers88,2902
fn test_sequence_equality_strings() {test_sequence_equality_strings113,3692
fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference138,4576
    struct PartialComparable { value: f64 }PartialComparable140,4655
    struct PartialComparable { value: f64 }value140,4655
    struct TotalComparable { value: N }TotalComparable149,5103
    struct TotalComparable { value: N }value149,5103
fn test_ordering_numbers_basic() {test_ordering_numbers_basic161,5578
fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal170,5793
fn test_ordering_strings_basic() {test_ordering_strings_basic176,5901
fn test_strings_equal_is_equal() {test_strings_equal_is_equal185,6114
fn test_nth_on_empty_panics() {test_nth_on_empty_panics192,6239
fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics199,6386
fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap206,6497
fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes212,6692
fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic221,6949
fn test_new_set_persistent() {test_new_set_persistent230,7300
fn test_iterator_collects_in_order() {test_iterator_collects_in_order240,7585

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_7.rs,424
fn is_even(x: &N) -> B { if *x % 2 == 0 { B::True } else { B::False } }is_even9,301
fn is_vowel(c: &char) -> B {is_vowel10,373
fn pair_even_with_vowels(a: &ArrayPerS<N>, b: &ArrayPerS<char>) -> ArrayPerS<(N, char)> {pair_even_with_vowels19,640
fn test_pair_even_with_vowels_basic() {test_pair_even_with_vowels_basic30,1266
fn test_pair_even_with_vowels_debug_shape() {test_pair_even_with_vowels_debug_shape39,1576

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Algorithm_21_2.rs,515
fn points3d_tab_flat(n: N) -> ArrayPerS<(N, N, N)> {points3d_tab_flat11,480
fn test_points3d_tab_flat_n0_empty() {test_points3d_tab_flat_n0_empty32,1437
fn test_points3d_tab_flat_n1_single() {test_points3d_tab_flat_n1_single38,1552
fn test_points3d_tab_flat_n2_values_and_order() {test_points3d_tab_flat_n2_values_and_order45,1719
fn test_points3d_tab_flat_iterator_order() {test_points3d_tab_flat_iterator_order58,2023
fn test_points3d_tab_flat_debug_shape() {test_points3d_tab_flat_debug_shape65,2276

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/15_TestAVLTreeSeqPer.rs,157
fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate5,109
fn test_iterator_inorder_values() {test_iterator_inorder_values14,451

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Foo2TestArraySeqPer.rs,67
fn test_wildcard_import_usage() {test_wildcard_import_usage7,158

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_2.txt,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/13_TestArraySeqEphChap18.rs,982
fn test_tabulate_fibonacci() {test_tabulate_fibonacci8,230
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }fib9,261
fn test_map_increment() {test_map_increment20,705
fn test_subseq() {test_subseq27,908
fn test_append() {test_append38,1328
fn test_sequence_literals_and_append() {test_sequence_literals_and_append46,1552
fn test_filter_even() {test_filter_even59,2128
fn test_flatten() {test_flatten69,2614
fn test_update_sequence() {test_update_sequence83,3221
fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins95,3754
fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins108,4406
fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan124,5036
fn test_iterate_sum_basic() {test_iterate_sum_basic143,5946
fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum151,6183
fn test_collect_groups_by_key() {test_collect_groups_by_key163,6593

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Algorithm_21_6.rs,259
fn prime_sieve(n: N) -> ArrayPerS<N> {prime_sieve11,462
fn test_prime_sieve_small() {test_prime_sieve_small45,2153
fn test_prime_sieve_n2_empty() {test_prime_sieve_n2_empty52,2309
fn test_prime_sieve_debug_shape() {test_prime_sieve_debug_shape58,2412

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Algorithm_21_1.rs,421
fn points2d_tab_flat(n: N) -> ArrayPerS<(N, N)> {points2d_tab_flat10,401
fn test_points2d_n3_example() {test_points2d_n3_example21,831
fn test_points2d_n1_empty() {test_points2d_n1_empty28,1027
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values34,1133
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order42,1316
fn test_points2d_debug_shape() {test_points2d_debug_shape50,1593

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_8_and_Algorithm_21_5.rs,391
fn is_divisible(n: N, i: N) -> B { if n % i == 0 { B::True } else { B::False } }is_divisible9,282
fn is_prime(n: N) -> B {is_prime14,525
fn primes_bf(n: N) -> ArrayPerS<N> {primes_bf26,1098
fn test_is_prime_small_values() {test_is_prime_small_values34,1422
fn test_primes_bf_small() {test_primes_bf_small44,1699
fn test_primes_bf_debug_shape() {test_primes_bf_debug_shape51,1851

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/02_TestMathSeq.rs,1243
fn test_length_and_nth_basic() {test_length_and_nth_basic8,200
fn test_add_last_and_delete_last() {test_add_last_and_delete_last16,388
fn test_new_empty_singleton_and_predicates() {test_new_empty_singleton_and_predicates29,785
fn test_set_in_bounds_and_out_of_bounds() {test_set_in_bounds_and_out_of_bounds48,1307
fn test_subseq_view_bounds() {test_subseq_view_bounds59,1622
fn test_subseq_copy_bounds() {test_subseq_copy_bounds72,1967
fn test_domain() {test_domain82,2223
fn test_range_deduplicates_preserving_order() {test_range_deduplicates_preserving_order88,2340
fn test_debug_format_for_mathseq() {test_debug_format_for_mathseq95,2540
fn test_display_format_for_mathseq() {test_display_format_for_mathseq101,2681
fn test_multiset_range_counts_first_occurrence_order() {test_multiset_range_counts_first_occurrence_order107,2822
fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics115,3040
fn test_range_empty_returns_empty() {test_range_empty_returns_empty121,3146
fn test_multiset_range_empty_returns_empty() {test_multiset_range_empty_returns_empty128,3291
fn test_domain_empty_is_empty() {test_domain_empty_is_empty135,3459
fn test_iter_collect_and_sum() {test_iter_collect_and_sum142,3593

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/03_TestLinkedListPer.rs,458
fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates6,131
fn test_new_and_nth_set() {test_new_and_nth_set15,380
fn test_subseq_copy() {test_subseq_copy27,701
fn test_from_vec_and_debug_format() {test_from_vec_and_debug_format36,902
fn test_iter_inorder_collect() {test_iter_inorder_collect43,1068
fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics51,1260
fn test_display_impl() {test_display_impl57,1364

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_5_and_21_6.rs,381
fn all_contiguous_subseqs<T: Clone + Eq>(a: &ArrayPerS<T>) -> ArrayPerS<ArrayPerS<T>> {all_contiguous_subseqs11,417
fn test_all_contiguous_subseqs_n0() {test_all_contiguous_subseqs_n028,1035
fn test_all_contiguous_subseqs_n3_values() {test_all_contiguous_subseqs_n3_values35,1214
fn test_all_contiguous_subseqs_debug_shape() {test_all_contiguous_subseqs_debug_shape48,1587

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/16_TestAVLTreeSeqPerChap18.rs,209
fn test_tabulate_inorder() {test_tabulate_inorder10,297
fn test_map_increment() {test_map_increment16,494
fn test_append_union() {test_append_union23,796
fn test_filter_even() {test_filter_even31,1198

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/20_TestAVLTreeSeqEphChap19.rs,181
fn test_tabulate_and_map() {test_tabulate_and_map11,284
fn test_select_and_append() {test_select_and_append19,578
fn test_deflate_and_filter() {test_deflate_and_filter38,1374

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListPer.rs,80
fn bench_sll_persistent_ops(c: &mut Criterion) {bench_sll_persistent_ops8,223

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqPerChap19.rs,94
fn bench_build_and_read_persistent(c: &mut Criterion) {bench_build_and_read_persistent9,264

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqPerChap19.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch198,239

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchMathSeq.rs,72
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics8,220

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListPerChap18.rs,66
fn bench_ll_per_ch18(c: &mut Criterion) {bench_ll_per_ch187,224

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqEphChap19.rs,68
fn bench_avl_eph_ch19(c: &mut Criterion) {bench_avl_eph_ch196,200

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqEphChap19.rs,86
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch197,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListEphChap19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch197,224

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqPer.rs,80
fn bench_build_and_contains(c: &mut Criterion) {bench_build_and_contains9,277

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqPer.rs,477
struct LinearCongruentialGenerator32 { state: u32 }LinearCongruentialGenerator3210,348
struct LinearCongruentialGenerator32 { state: u32 }state10,348
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3212,401
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new13,438
    fn next_N(&mut self) -> N {next_N15,557
fn bench_build_random_s_persistent(c: &mut Criterion) {bench_build_random_s_persistent24,794

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqEph.rs,58
fn bench_avl_eph(c: &mut Criterion) {bench_avl_eph6,200

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch187,230

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqEphChap18.rs,68
fn bench_avl_eph_ch18(c: &mut Criterion) {bench_avl_eph_ch186,200

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListEph.rs,56
fn bench_ll_eph(c: &mut Criterion) {bench_ll_eph6,185

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListPerChap19.rs,66
fn bench_ll_per_ch19(c: &mut Criterion) {bench_ll_per_ch198,237

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListEphChap18.rs,66
fn bench_ll_eph_ch18(c: &mut Criterion) {bench_ll_eph_ch186,211

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqEph.rs,455
struct LinearCongruentialGenerator32 { state: u32 }LinearCongruentialGenerator3211,367
struct LinearCongruentialGenerator32 { state: u32 }state11,367
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3213,420
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new14,457
    fn next_N(&mut self) -> N {next_N16,576
fn bench_build_random_s(c: &mut Criterion) {bench_build_random_s25,813

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqEphChap18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map7,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqPerChap18.rs,68
fn bench_avl_per_ch18(c: &mut Criterion) {bench_avl_per_ch187,217
