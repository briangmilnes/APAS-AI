
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPer.rs,1903
pub struct NodeP<T> {NodeP5,89
    pub value: T,value6,111
    pub next: Option<Box<NodeP<T>>>,next7,129
pub struct LinkedListPerS<T> {LinkedListPerS10,169
    head: Option<Box<NodeP<T>>>,head11,200
    len: N,len12,233
pub trait LinkedListPerTrait<T> {LinkedListPerTrait15,248
    fn empty() -> LinkedListPerS<T>;empty16,282
    fn new(length: N, init_value: T) -> LinkedListPerS<T>new17,319
    fn length(&self) -> N;length20,405
    fn nth(&self, index: N) -> &T;nth21,432
    fn isEmpty(&self) -> B;isEmpty22,467
    fn isSingleton(&self) -> B;isSingleton23,495
    fn singleton(item: T) -> LinkedListPerS<T>;singleton24,527
    fn set(&self, index: N, item: T) -> Result<LinkedListPerS<T>, &'static str>set28,770
    fn subseq_copy(&self, start: N, length: N) -> LinkedListPerS<T>subseq_copy31,878
impl<T> LinkedListPerS<T> {LinkedListPerS36,977
    fn push_front_node(&mut self, node: Box<NodeP<T>>) {push_front_node37,1005
    pub fn from_vec(v: Vec<T>) -> LinkedListPerS<T> {from_vec44,1182
impl<T> LinkedListPerTrait<T> for LinkedListPerS<T> {LinkedListPerS53,1432
    fn empty() -> LinkedListPerS<T> {empty54,1486
    fn new(length: N, init_value: T) -> LinkedListPerS<T>new57,1576
    fn length(&self) -> N {length79,2224
    fn nth(&self, index: N) -> &T {nth82,2275
    fn isEmpty(&self) -> B {isEmpty94,2594
    fn isSingleton(&self) -> B {isSingleton101,2724
    fn singleton(item: T) -> LinkedListPerS<T> {singleton108,2858
    fn set(&self, index: N, item: T) -> Result<LinkedListPerS<T>, &'static str>set117,3082
    fn subseq_copy(&self, start: N, length: N) -> LinkedListPerS<T>subseq_copy152,4125
impl<T: std::fmt::Debug> std::fmt::Debug for LinkedListPerS<T> {LinkedListPerS190,5171
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt191,5236
macro_rules! LinkedListPer {LinkedListPer203,5578

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/MathSeq.rs,2038
pub struct MathSeqS<T> {MathSeqS16,578
    pub data: Vec<T>,data17,603
pub trait MathSeqTrait<T> {MathSeqTrait21,660
    fn new(length: N, init_value: T) -> MathSeqS<T>new25,816
    fn empty() -> MathSeqS<T>;empty32,975
    fn singleton(item: T) -> MathSeqS<T>;singleton37,1105
    fn length(&self) -> N;length41,1219
    fn nth(&self, index: N) -> &T;nth45,1346
    fn set(&mut self, index: N, value: T) -> Result<&mut MathSeqS<T>, &'static str>;set49,1469
    fn add_last(&mut self, value: T) -> &mut MathSeqS<T>;add_last54,1827
    fn delete_last(&mut self) -> Option<T>;delete_last58,1960
    fn subseq(&self, start: N, length: N) -> &[T];subseq62,2112
    fn subseq_copy(&self, start: N, length: N) -> MathSeqS<T>subseq_copy67,2299
    fn isEmpty(&self) -> B;isEmpty74,2477
    fn isSingleton(&self) -> B;isSingleton79,2592
    fn domain(&self) -> Vec<N>;domain83,2702
    fn range(&self) -> Vec<T>range87,2870
    fn multiset_range(&self) -> Vec<(N, T)>multiset_range93,3072
impl<T> MathSeqTrait<T> for MathSeqS<T> {MathSeqS98,3159
    fn new(length: N, init_value: T) -> MathSeqS<T>new100,3240
    fn empty() -> MathSeqS<T> {empty110,3440
    fn singleton(item: T) -> MathSeqS<T> {singleton115,3551
    fn length(&self) -> N {length120,3673
    fn nth(&self, index: N) -> &T {nth125,3765
    fn set(&mut self, index: N, value: T) -> Result<&mut MathSeqS<T>, &'static str> {set130,3867
    fn add_last(&mut self, value: T) -> &mut MathSeqS<T> {add_last142,4374
    fn delete_last(&mut self) -> Option<T> {delete_last148,4517
    fn subseq(&self, start: N, length: N) -> &[T] {subseq153,4626
    fn subseq_copy(&self, start: N, length: N) -> MathSeqS<T>subseq_copy161,4865
    fn isEmpty(&self) -> B {isEmpty177,5271
    fn isSingleton(&self) -> B {isSingleton186,5443
    fn domain(&self) -> Vec<N> {domain195,5630
    fn range(&self) -> Vec<T>range200,5754
    fn multiset_range(&self) -> Vec<(N, T)>multiset_range215,6161
macro_rules! MathSeq {MathSeq237,6786

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/RustSemantics.txt,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEph.rs,2084
pub struct ArraySeqEphS<T> {ArraySeqEphS7,210
    pub data: Box<[T]>,data8,239
pub trait ArraySeqEphTrait<T> {ArraySeqEphTrait12,308
    fn new(length: N, init_value: T) -> ArraySeqEphS<T>new14,377
    fn length(&self) -> N;length18,493
    fn nth(&self, index: N) -> &T;nth20,552
    fn empty() -> ArraySeqEphS<T>;empty22,619
    fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqEphS<T>, &'static str>;set24,686
    fn singleton(item: T) -> ArraySeqEphS<T>;singleton26,806
    fn isEmpty(&self) -> B;isEmpty28,884
    fn isSingleton(&self) -> B;isSingleton30,944
    fn subseq_copy(&self, start: N, length: N) -> ArraySeqEphS<T>subseq_copy32,1019
impl<T> ArraySeqEphS<T> {ArraySeqEphS37,1121
    pub fn subseq(&self, start: N, length: N) -> &[T] {subseq38,1147
    pub fn subseq_copy(&self, start: N, length: N) -> ArraySeqEphS<T>subseq_copy44,1350
    pub fn from_vec(v: Vec<T>) -> ArraySeqEphS<T> {from_vec63,1976
    pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqEphS<T> {update68,2107
impl<T: Eq> PartialEq for ArraySeqEphS<T> {ArraySeqEphS76,2291
    fn eq(&self, other: &Self) -> bool {eq77,2335
impl<T: Eq> Eq for ArraySeqEphS<T> {}ArraySeqEphS89,2613
impl<T: std::fmt::Debug> std::fmt::Debug for ArraySeqEphS<T> {ArraySeqEphS91,2652
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt92,2715
impl<T> ArraySeqEphTrait<T> for ArraySeqEphS<T> {ArraySeqEphS98,2903
    fn new(length: N, init_value: T) -> ArraySeqEphS<T>new99,2953
    fn length(&self) -> N {length105,3106
    fn nth(&self, index: N) -> &T {nth108,3164
    fn empty() -> ArraySeqEphS<T> {empty111,3232
    fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqEphS<T>, &'static str> {set114,3317
    fn singleton(item: T) -> ArraySeqEphS<T> {singleton122,3573
    fn isEmpty(&self) -> B {isEmpty125,3669
    fn isSingleton(&self) -> B {isSingleton132,3806
    fn subseq_copy(&self, start: N, length: N) -> ArraySeqEphS<T>subseq_copy139,3947
macro_rules! ArraySeqEph {ArraySeqEph148,4117

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,96
pub type N = usize;N8,264
pub enum B {B12,395
    True,True13,408
    False,False14,418

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main7,236

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,752
pub mod Types;Types7,226
pub mod MathSeq;MathSeq9,242
pub mod LinkedListPer;LinkedListPer16,330
pub mod LinkedListPerChap18;LinkedListPerChap1818,421
pub mod LinkedListPerChap19;LinkedListPerChap1920,512
pub mod LinkedListEph;LinkedListEph23,604
pub mod ArraySeqPer;ArraySeqPer26,696
pub mod ArraySeqPerChap18;ArraySeqPerChap1829,777
pub mod ArraySeqPerChap19;ArraySeqPerChap1932,863
pub mod ArraySeqEph;ArraySeqEph35,949
pub mod ArraySeqEphChap18;ArraySeqEphChap1838,1033
pub mod ArraySeqEphChap19;ArraySeqEphChap1940,1118
pub mod AVLTreeSeqPer;AVLTreeSeqPer43,1204
pub mod AVLTreeSeqPerChap18;AVLTreeSeqPerChap1845,1295
pub mod AVLTreeSeqPerChap19;AVLTreeSeqPerChap1947,1386
pub mod AVLTreeSeqEph;AVLTreeSeqEph51,1538

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqEph.rs,3997
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
    fn empty() -> AVLTreeSeqEphS<T>;empty40,871
    fn new() -> AVLTreeSeqEphS<T>;new42,946
    fn length(&self) -> N;length44,1019
    fn nth(&self, index: N) -> &T;nth46,1092
    fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeSeqEphS<T>, &'static str>;set48,1173
    fn singleton(item: T) -> AVLTreeSeqEphS<T>;singleton50,1301
    fn isEmpty(&self) -> B;isEmpty52,1387
    fn isSingleton(&self) -> B;isSingleton54,1453
    fn subseq_copy(&self, start: N, length: N) -> AVLTreeSeqEphS<T>subseq_copy56,1543
impl<T: Copy + Debug> AVLTreeSeqEphS<T> {AVLTreeSeqEphS61,1647
    pub fn new_root() -> Self {new_root62,1689
    pub fn new() -> Self {new68,1811
    pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqEphS<T> {update71,1869
    pub fn from_vec(values: Vec<T>) -> AVLTreeSeqEphS<T>from_vec75,2054
    pub fn to_arrayseq(&self) -> ArraySeqEphS<T>to_arrayseq87,2429
    pub fn iter<'a>(&'a self) -> AVLTreeSeqIterEph<'a, T> {iter109,3074
    pub fn push_back(&mut self, value: T) {push_back112,3183
    pub fn contains_value(&self, target: &T) -> Bcontains_value117,3377
    pub fn insert_value(&mut self, value: T) {insert_value128,3604
    pub fn delete_value(&mut self, target: &T) -> booldelete_value131,3688
impl<T: Copy + Debug> AVLTreeSeqEphTrait<T> for AVLTreeSeqEphS<T> {AVLTreeSeqEphS159,4452
    fn empty() -> AVLTreeSeqEphS<T> {empty160,4520
    fn new() -> AVLTreeSeqEphS<T> {new164,4600
    fn length(&self) -> N {length168,4678
    fn nth(&self, index: N) -> &T {nth172,4743
    fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeSeqEphS<T>, &'static str> {set176,4822
    fn singleton(item: T) -> AVLTreeSeqEphS<T> {singleton181,4985
    fn isEmpty(&self) -> B {isEmpty187,5173
    fn isSingleton(&self) -> B {isSingleton195,5309
    fn subseq_copy(&self, start: N, length: N) -> AVLTreeSeqEphS<T>subseq_copy203,5449
pub struct AVLTreeSeqIterEph<'a, T: Copy + Debug> {AVLTreeSeqIterEph221,5958
    stack: Vec<&'a AVLTreeNode<T>>,stack222,6010
    current: Option<&'a AVLTreeNode<T>>,current223,6046
impl<'a, T: Copy + Debug> AVLTreeSeqIterEph<'a, T> {AVLTreeSeqIterEph226,6090
    fn new(root: &'a Link<T>) -> Self {new227,6143
    fn push_left(&mut self, link: &'a Link<T>) {push_left235,6338
impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIterEph<'a, T> {AVLTreeSeqIterEph244,6558
    type Item = &'a T;Item245,6624
    fn next(&mut self) -> Option<Self::Item> {next246,6647
fn h<T: Copy + Debug>(n: &Link<T>) -> N {h254,6843
fn size_link<T: Copy + Debug>(n: &Link<T>) -> N {size_link258,6927
fn update_meta<T: Copy + Debug>(n: &mut Box<AVLTreeNode<T>>) {update_meta266,7073
fn rotate_right<T: Copy + Debug>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right274,7299
fn rotate_left<T: Copy + Debug>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left285,7631
fn rebalance<T: Copy + Debug>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance296,7961
pub(crate) fn insert_at_link<T: Copy + Debug>(insert_at_link317,8655
fn nth_link<'a, T: Copy + Debug>(node: &'a Link<T>, index: N) -> &'a T {nth_link342,9399
fn set_link<T: Copy + Debug>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str>set_link354,9744

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPer.rs,3368
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
    fn empty() -> AVLTreeSeqPerS<T>;empty138,3993
    fn new() -> AVLTreeSeqPerS<T>;new140,4067
    fn length(&self) -> N;length142,4139
    fn nth(&self, index: N) -> &T;nth144,4211
    fn set(&self, index: N, item: T) -> Result<AVLTreeSeqPerS<T>, &'static str>;set146,4350
    fn singleton(item: T) -> AVLTreeSeqPerS<T>;singleton148,4468
    fn isEmpty(&self) -> B;isEmpty150,4553
    fn isSingleton(&self) -> B;isSingleton152,4618
    fn subseq_copy(&self, start: N, length: N) -> AVLTreeSeqPerS<T>subseq_copy154,4703
    fn from_vec(values: Vec<T>) -> AVLTreeSeqPerS<T>from_vec158,4863
    fn values_in_order(&self) -> Vec<T>values_in_order162,4984
impl<T: Copy + Debug> AVLTreeSeqPerTrait<T> for AVLTreeSeqPerS<T> {AVLTreeSeqPerS167,5055
    fn empty() -> AVLTreeSeqPerS<T> {empty168,5123
    fn new() -> AVLTreeSeqPerS<T> {new171,5205
    fn length(&self) -> N {length174,5296
    fn nth(&self, index: N) -> &T {nth177,5355
    fn set(&self, index: N, item: T) -> Result<AVLTreeSeqPerS<T>, &'static str> {set180,5432
    fn singleton(item: T) -> AVLTreeSeqPerS<T> {singleton185,5612
    fn isEmpty(&self) -> B {isEmpty190,5748
    fn isSingleton(&self) -> B {isSingleton197,5883
    fn subseq_copy(&self, start: N, length: N) -> AVLTreeSeqPerS<T>subseq_copy204,6022
    fn from_vec(values: Vec<T>) -> AVLTreeSeqPerS<T>from_vec221,6602
    fn values_in_order(&self) -> Vec<T>values_in_order229,6788
impl<T: Eq + Copy + Debug> PartialEq for AVLTreeSeqPerS<T> {AVLTreeSeqPerS239,6987
    fn eq(&self, other: &Self) -> bool {eq240,7048
impl<T: Eq + Copy + Debug> Eq for AVLTreeSeqPerS<T> {}AVLTreeSeqPerS252,7326
impl<T: Debug + Copy> std::fmt::Debug for AVLTreeSeqPerS<T> {AVLTreeSeqPerS254,7382
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt255,7444
impl<T: Copy + Debug> AVLTreeSeqPerS<T> {AVLTreeSeqPerS261,7648
    pub fn to_arrayseq(&self) -> ArrayPerS<T>to_arrayseq262,7690
macro_rules! AVLTreeSeqPer {AVLTreeSeqPer272,7898

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListEph.rs,1850
pub struct NodeE<T> {NodeE5,115
    pub value: T,value6,137
    pub next: Option<Box<NodeE<T>>>,next7,155
pub struct LinkedListEphS<T> {LinkedListEphS10,195
    head: Option<Box<NodeE<T>>>,head11,226
    len: N,len12,259
pub trait LinkedListEphTrait<T> {LinkedListEphTrait15,274
    fn empty() -> LinkedListEphS<T>;empty16,308
    fn new(length: N, init_value: T) -> LinkedListEphS<T>new17,345
    fn length(&self) -> N;length20,431
    fn nth(&self, index: N) -> &T;nth21,458
    fn isEmpty(&self) -> B;isEmpty22,493
    fn isSingleton(&self) -> B;isSingleton23,521
    fn singleton(item: T) -> LinkedListEphS<T>;singleton24,553
    fn set(&mut self, index: N, item: T) -> Result<&mut LinkedListEphS<T>, &'static str>;set25,601
    fn subseq_copy(&self, start: N, length: N) -> LinkedListEphS<T>subseq_copy26,691
impl<T> LinkedListEphS<T> {LinkedListEphS31,790
    fn push_front_node(&mut self, node: Box<NodeE<T>>) {push_front_node32,818
impl<T> LinkedListEphTrait<T> for LinkedListEphS<T> {LinkedListEphS40,997
    fn empty() -> LinkedListEphS<T> {empty41,1051
    fn new(length: N, init_value: T) -> LinkedListEphS<T>new44,1141
    fn length(&self) -> N {length66,1789
    fn nth(&self, index: N) -> &T {nth69,1840
    fn isEmpty(&self) -> B {isEmpty81,2159
    fn isSingleton(&self) -> B {isSingleton88,2289
    fn singleton(item: T) -> LinkedListEphS<T> {singleton95,2423
    fn set(&mut self, index: N, item: T) -> Result<&mut LinkedListEphS<T>, &'static str> {set104,2647
    fn subseq_copy(&self, start: N, length: N) -> LinkedListEphS<T>subseq_copy117,3050
impl<T: std::fmt::Debug> std::fmt::Debug for LinkedListEphS<T> {LinkedListEphS156,4133
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt157,4198
macro_rules! LinkedListEph {LinkedListEph169,4540

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPer.rs,2222
pub struct ArrayPerS<T> { pub data: Box<[T]> }ArrayPerS12,478
pub struct ArrayPerS<T> { pub data: Box<[T]> }data12,478
pub trait ArraySeqPerTrait<T> {ArraySeqPerTrait15,591
    fn new(length: N, init_value: T) -> ArrayPerS<T> where T: Clone;new17,665
    fn length(&self) -> N;length19,771
    fn nth(&self, index: N) -> &T;nth21,835
    fn empty() -> ArrayPerS<T>;empty23,907
    fn set(&self, index: N, item: T) -> Result<ArrayPerS<T>, &'static str> where T: Clone;set27,1150
    fn singleton(item: T) -> ArrayPerS<T>;singleton29,1278
    fn isEmpty(&self) -> B;isEmpty31,1358
    fn isSingleton(&self) -> B;isSingleton33,1423
    fn subseq_copy(&self, start: N, length: N) -> ArrayPerS<T> where T: Clone + Eq;subseq_copy35,1497
impl<T> ArrayPerS<T> {ArrayPerS38,1584
    pub fn subseq(&self, start: N, length: N) -> &[T] {subseq40,1644
    pub fn from_vec(v: Vec<T>) -> ArrayPerS<T> { ArrayPerS { data: v.into_boxed_slice() } }from_vec49,1975
impl<T: Eq> PartialEq for ArrayPerS<T> {ArrayPerS52,2070
    fn eq(&self, other: &Self) -> bool {eq53,2111
impl<T: Eq> Eq for ArrayPerS<T> {}ArrayPerS59,2321
impl<T: std::fmt::Debug> std::fmt::Debug for ArrayPerS<T> {ArrayPerS61,2357
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt62,2417
impl<T> ArraySeqPerTrait<T> for ArrayPerS<T> {ArrayPerS68,2605
    fn new(length: N, init_value: T) -> ArrayPerS<T> where T: Clone {new69,2652
    fn length(&self) -> N { self.data.len() }length72,2782
    fn nth(&self, index: N) -> &T { &self.data[index] }nth73,2828
    fn empty() -> ArrayPerS<T> { ArrayPerS::from_vec(Vec::new()) }empty74,2884
    fn set(&self, index: N, item: T) -> Result<ArrayPerS<T>, &'static str> where T: Clone {set75,2951
    fn singleton(item: T) -> ArrayPerS<T> { ArrayPerS::from_vec(vec![item]) }singleton81,3232
    fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }isEmpty82,3310
    fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton83,3395
    fn subseq_copy(&self, start: N, length: N) -> ArrayPerS<T> where T: Clone + Eq {subseq_copy84,3484
macro_rules! ArraySeqPer {ArraySeqPer96,3870

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/12_TestAVLTreeSeqEph.rs,123
fn test_insert_contains_delete() {test_insert_contains_delete7,143
fn test_iter_and_range() {test_iter_and_range19,599

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/13_TestAVLTreeSeqChap18.rs,208
fn test_tabulate_inorder() {test_tabulate_inorder9,271
fn test_map_increment() {test_map_increment15,481
fn test_append_union() {test_append_union22,796
fn test_filter_even() {test_filter_even30,1211

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/20_TestAVLTreeSeqEph.rs,63
fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic6,133

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/01_TestTypes.rs,148
fn test_boolean_eq_neq_and_ordering() {test_boolean_eq_neq_and_ordering4,40
fn test_ordering_on_n_naturals() {test_ordering_on_n_naturals16,387

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/04_TestLinkedListEph.rs,63
fn test_sll_ephemeral_basic() {test_sll_ephemeral_basic6,153

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/20_TestAVLTreeSeqPer.rs,85
fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate5,127

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/05_TestArraySeqChap19Eph.rs,2577
fn test_empty() {test_empty8,275
fn test_singleton() {test_singleton14,409
fn test_map() {test_map20,600
fn test_append() {test_append27,876
fn test_append2() {test_append235,1242
fn test_deflate_true() {test_deflate_true43,1610
fn test_deflate_false() {test_deflate_false49,1837
fn test_filter_even_numbers() {test_filter_even_numbers55,2040
fn test_filter_none() {test_filter_none62,2383
fn test_update_in_bounds() {test_update_in_bounds69,2711
fn test_update_out_of_bounds() {test_update_out_of_bounds76,3003
fn test_isEmpty() {test_isEmpty83,3292
fn test_isSingleton() {test_isSingleton93,3680
fn test_iterate_sum() {test_iterate_sum103,4084
fn test_iterate_concat() {test_iterate_concat110,4348
fn test_map_empty() {test_map_empty124,4749
fn test_append_with_empty() {test_append_with_empty131,5010
fn test_append2_equivalence() {test_append2_equivalence141,5558
fn test_filter_empty_sequence() {test_filter_empty_sequence150,5980
fn test_select_boundary() {test_select_boundary157,6258
fn test_subseq_basic() {test_subseq_basic168,6982
fn test_reduce_sum_basic_ch19() {test_reduce_sum_basic_ch19175,7270
fn test_scan_sum_basic_ch19() {test_scan_sum_basic_ch19189,7992
fn test_flatten_ch19() {test_flatten_ch19200,8412
fn test_atomic_write_lowest_wins_serial() {test_atomic_write_lowest_wins_serial209,8889
fn test_atomic_write_highest_wins_mutex() {test_atomic_write_highest_wins_mutex224,9660
fn test_iterate_empty_returns_acc() {test_iterate_empty_returns_acc248,10612
fn test_inject_parallel2_equivalence_leftmost_wins() {test_inject_parallel2_equivalence_leftmost_wins255,10908
fn test_inject_parallel2_out_of_bounds_ignored() {test_inject_parallel2_out_of_bounds_ignored264,11417
fn test_inject_parallel2_empty_values_and_empty_changes() {test_inject_parallel2_empty_values_and_empty_changes272,11845
fn test_inject_parallel2_empty_values_nonempty_changes() {test_inject_parallel2_empty_values_nonempty_changes280,12236
fn test_inject_parallel2_nonempty_values_empty_changes() {test_inject_parallel2_nonempty_values_empty_changes288,12642
fn test_actually_atomic_write_ordering() {test_actually_atomic_write_ordering296,13026
fn test_inject_first_wins_duplicate_index() {test_inject_first_wins_duplicate_index330,14316
fn test_inject_empty_updates_no_change() {test_inject_empty_updates_no_change343,14842
fn test_inject_parallel_equivalence() {test_inject_parallel_equivalence352,15248
fn test_ninject_parallel2_equivalence_last_wins() {test_ninject_parallel2_equivalence_last_wins362,15806

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/05_TestArraySeqEphChapPer18.rs,885
fn test_tabulate_fibonacci() {test_tabulate_fibonacci7,222
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }fib8,253
fn test_map_increment() {test_map_increment19,748
fn test_subseq() {test_subseq26,1002
fn test_append() {test_append37,1515
fn test_sequence_literals_and_append() {test_sequence_literals_and_append45,1810
fn test_filter_even() {test_filter_even58,2534
fn test_flatten() {test_flatten68,3102
fn test_update_sequence() {test_update_sequence82,3926
fn test_inject_and_ninject() {test_inject_and_ninject93,4514
fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan112,5857
fn test_iterate_sum_basic() {test_iterate_sum_basic131,6866
fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum139,7134
fn test_collect_groups_by_key() {test_collect_groups_by_key151,7575

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/23_TestAVLTreeSeqPer.rs,85
fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate5,127

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/06_TestArraySeqPer.rs,60
fn test_new_set_persistent() {test_new_set_persistent5,90

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/14_TestAVLTreeSeqChap19Eph.rs,180
fn test_tabulate_and_map() {test_tabulate_and_map8,184
fn test_select_and_append() {test_select_and_append15,452
fn test_deflate_and_filter() {test_deflate_and_filter26,1041

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/02_TestMathSeq.rs,1019
fn test_length_and_nth_basic() {test_length_and_nth_basic8,201
fn test_add_last_and_delete_last() {test_add_last_and_delete_last16,386
fn test_new_empty_singleton_and_predicates() {test_new_empty_singleton_and_predicates29,780
fn test_set_in_bounds_and_out_of_bounds() {test_set_in_bounds_and_out_of_bounds48,1448
fn test_subseq_view_bounds() {test_subseq_view_bounds59,1760
fn test_subseq_copy_bounds() {test_subseq_copy_bounds72,2102
fn test_domain() {test_domain82,2411
fn test_range_deduplicates_preserving_order() {test_range_deduplicates_preserving_order88,2525
fn test_multiset_range_counts_first_occurrence_order() {test_multiset_range_counts_first_occurrence_order95,2722
fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics103,2937
fn test_range_empty_returns_empty() {test_range_empty_returns_empty109,3040
fn test_multiset_range_empty_returns_empty() {test_multiset_range_empty_returns_empty116,3182
fn test_domain_empty_is_empty() {test_domain_empty_is_empty123,3347

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/23_TestAVLTreeSeqEph.rs,63
fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic6,133

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/16_TestAVLTreeSeqEph.rs,1336
fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates9,284
fn test_new_and_set() {test_new_and_set23,774
fn test_from_vec_and_to_arrayseq() {test_from_vec_and_to_arrayseq39,1571
fn test_subseq_copy_basic() {test_subseq_copy_basic48,1918
fn test_ordering_numbers_and_strings_parity() {test_ordering_numbers_and_strings_parity58,2321
fn test_out_of_bounds_behaviors() {test_out_of_bounds_behaviors68,2608
fn test_equality_and_debug() {test_equality_and_debug78,3062
fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers88,3333
fn test_sequence_equality_strings() {test_sequence_equality_strings97,3611
fn test_sequence_basic_booleans() {test_sequence_basic_booleans106,3938
fn test_from_vec_strings() {test_from_vec_strings116,4587
fn test_nth_upper_bound_panics_avl3() {test_nth_upper_bound_panics_avl3124,4805
fn test_nth_on_empty_panics_avl3() {test_nth_on_empty_panics_avl3131,4995
fn test_set_out_of_bounds_panics_on_unwrap_avl3() {test_set_out_of_bounds_panics_on_unwrap_avl3138,5205
fn test_eq_vs_partial_eq_difference_avl3() {test_eq_vs_partial_eq_difference_avl3144,5422
    struct PartialComparable { value: f64 }PartialComparable146,5512
    struct PartialComparable { value: f64 }value146,5512
fn test_iterator() {test_iterator158,6299

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/03_TestLinkedListPer.rs,251
fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates5,81
fn test_new_and_nth_set() {test_new_and_nth_set14,458
fn test_subseq_copy() {test_subseq_copy23,810
fn test_new_set_persistent() {test_new_set_persistent32,1156

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/22_TestArraySeqEph.rs,163
fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic7,233
fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter16,551

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/03_TestLinkedListEph.rs,189
fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates7,147
fn test_new_and_nth_set() {test_new_and_nth_set16,524
fn test_subseq_copy() {test_subseq_copy25,876

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/13_TestAVLTreeSeqChap18Eph.rs,208
fn test_tabulate_inorder() {test_tabulate_inorder8,184
fn test_map_increment() {test_map_increment14,372
fn test_append_union() {test_append_union21,650
fn test_filter_even() {test_filter_even29,1013

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/22_TestArraySeqPer.rs,60
fn test_new_set_persistent() {test_new_set_persistent5,90

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/19_TestArraySeqEph.rs,163
fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic7,233
fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter16,551

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/05_TestArraySeqEph.rs,1728
fn test_new_and_set() {test_new_and_set7,177
fn test_length_and_nth_basic() {test_length_and_nth_basic21,649
fn test_empty() {test_empty29,841
fn test_sequence_basic() {test_sequence_basic36,1025
fn test_singleton() {test_singleton49,1669
fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton57,1872
fn test_from_vec() {test_from_vec72,2351
fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers89,3022
fn test_sequence_equality_strings() {test_sequence_equality_strings114,3942
fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference139,4956
    struct PartialComparable { value: f64 }PartialComparable141,5035
    struct PartialComparable { value: f64 }value141,5035
    struct TotalComparable { value: N }TotalComparable150,5535
    struct TotalComparable { value: N }value150,5535
fn test_ordering_numbers_basic() {test_ordering_numbers_basic162,6062
fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal171,6277
fn test_ordering_strings_basic() {test_ordering_strings_basic177,6385
fn test_strings_equal_is_equal() {test_strings_equal_is_equal186,6598
fn test_nth_on_empty_panics() {test_nth_on_empty_panics193,6723
fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics200,6870
fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap207,6994
fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes213,7189
fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic222,7446
fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic232,7824
fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter241,8142
