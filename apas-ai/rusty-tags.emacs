
/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPerChap19.rs,3798
pub trait ArraySeqPerChap19Trait {ArraySeqPerChap19Trait8,205
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T>;tabulate10,322
    fn map<T, U>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U>;map12,460
    fn select<'a, T>(a: &'a ArrayPerS<T>, b: &'a ArrayPerS<T>, i: N) -> Option<&'a T>;select14,570
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;append16,706
    fn append2<T: Clone>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;append218,837
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArrayPerS<T>;deflate20,952
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, f: impl Fn(&T) -> B) -> ArrayPerS<T>;filter22,1105
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate26,1358
    fn reduce<T: Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;reduce28,1496
    fn scan<T: Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> (ArrayPerS<T>, T) where F: Fn(&scan30,1634
    fn flatten<T: Clone + Eq>(s: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T>;flatten32,1803
    fn inject<T: Clone + Eq>(values: &ArrayPerS<T>, changes: &ArrayPerS<(N, T)>) -> ArrayPerS<T>inject36,2066
    fn atomicWrite<T: Clone + Eq>(values_with_change_number: &mut ArrayPerS<(T, N)>, changes: &AatomicWrite38,2201
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &ArrayPerS<T>, changes: &ArrayPerS<inject_parallel240,2392
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &ArrayAtomicWriteLowestChangeNumberWins41,2514
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &ArrayPerS<T>, changes: &ArrayPerSninject_parallel243,2737
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &ArraAtomicWriteHighestChangeNumberWins44,2860
impl<T2> ArraySeqPerChap19 for ArrayPerS<T2> {ArrayPerS47,3028
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T> {tabulate48,3075
    fn map<T, U>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U> {map52,3231
    fn select<'a, T>(a: &'a ArrayPerS<T>, b: &'a ArrayPerS<T>, i: N) -> Option<&'a T> {select55,3394
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {append58,3617
    fn append2<T: Clone>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {append261,3829
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArrayPerS<T> {deflate64,4037
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, f: impl Fn(&T) -> B) -> ArrayPerS<T> {filter67,4237
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate71,4550
    fn reduce<T: Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T {reduce78,4963
    fn scan<T: Clone + Eq, F>(a: &ArrayPerS<T>, f: &F, id: T) -> (ArrayPerS<T>, T) where F: Fn(&scan85,5530
    fn flatten<T: Clone + Eq>(s: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T> { <ArrayPerS<T> as Arflatten103,6594
    fn inject<T: Clone + Eq>(values: &ArrayPerS<T>, changes: &ArrayPerS<(N, T)>) -> ArrayPerS<T>inject104,6721
    fn atomicWrite<T: Clone + Eq>(values_with_change_number: &mut ArrayPerS<(T, N)>, changes: &AatomicWrite109,7161
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &ArrayPerS<T>, changes: &ArrayPerS<inject_parallel2118,7664
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &ArrayAtomicWriteLowestChangeNumberWins123,8284
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &ArrayPerS<T>, changes: &ArrayPerSninject_parallel2127,8741
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &ArraAtomicWriteHighestChangeNumberWins132,9363

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqEphChap19.rs,1370
pub trait AVLTreeSeqChap19Trait {AVLTreeSeqChap19Trait12,418
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeEphS<T>tabulate14,536
    fn map<T, U>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeEphS<U>map18,737
    fn select<'a, T>(a: &'a AVLTreeEphS<T>, b: &'a AVLTreeEphS<T>, i: N) -> Option<T>select23,980
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, b: &AVLTreeEphS<T>) -> AVLTreappend27,1187
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeEphS<T>;deflate29,1324
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> B) -> AVLTrfilter31,1506
impl<T2: Ord + Copy + Debug + Display> AVLTreeEphSeqChap19 for AVLTreeEphS<T2> {AVLTreeEphS34,1616
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeEphS<T>tabulate36,1781
    fn map<T, U>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeEphS<U>map42,2047
    fn select<'a, T>(a: &'a AVLTreeEphS<T>, b: &'a AVLTreeEphS<T>, i: N) -> Option<T>select49,2350
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, b: &AVLTreeEphS<T>) -> AVLTreappend63,2952
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeEphS<T> {deflate68,3159
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> B) -> AVLTrfilter73,3487

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPer.rs,2393
pub struct NodeP<T> { pub value: T, pub next: Option<Box<NodeP<T>>> }NodeP5,89
pub struct NodeP<T> { pub value: T, pub next: Option<Box<NodeP<T>>> }value5,89
pub struct NodeP<T> { pub value: T, pub next: Option<Box<NodeP<T>>> }next5,89
pub struct LinkedListPerS<T> { head: Option<Box<NodeP<T>>>, len: N }LinkedListPerS7,160
pub struct LinkedListPerS<T> { head: Option<Box<NodeP<T>>>, len: N }head7,160
pub struct LinkedListPerS<T> { head: Option<Box<NodeP<T>>>, len: N }len7,160
pub trait LinkedListPerTrait<T> {LinkedListPerTrait9,230
    fn empty() -> LinkedListPerS<T>;empty10,264
    fn new(length: N, init_value: T) -> LinkedListPerS<T> where T: Clone;new11,301
    fn length(&self) -> N;length12,375
    fn nth(&self, index: N) -> &T;nth13,402
    fn isEmpty(&self) -> B;isEmpty14,437
    fn isSingleton(&self) -> B;isSingleton15,465
    fn singleton(item: T) -> LinkedListPerS<T>;singleton16,497
    fn set(&self, index: N, item: T) -> Result<LinkedListPerS<T>, &'static str> where T: Clone;set20,740
    fn subseq_copy(&self, start: N, length: N) -> LinkedListPerS<T> where T: Clone;subseq_copy21,836
impl<T> LinkedListPerS<T> {LinkedListPerS24,923
    fn push_front_node(&mut self, node: Box<NodeP<T>>) {push_front_node25,951
    pub fn from_vec(v: Vec<T>) -> LinkedListPerS<T> {from_vec29,1104
impl<T> LinkedListPer<T> for LinkedListPerS<T> {LinkedListPerS38,1354
    fn empty() -> LinkedListPerS<T> { LinkedListPerS { head: None, len: 0 } }empty39,1403
    fn new(length: N, init_value: T) -> LinkedListPerS<T> where T: Clone {new40,1481
    fn length(&self) -> N { self.len }length48,1983
    fn nth(&self, index: N) -> &T nth49,2022
    fn isEmpty(&self) -> B { if self.len == 0 { B::True } else { B::False } }isEmpty52,2249
    fn isSingleton(&self) -> B { if self.len == 1 { B::True } else { B::False } }isSingleton53,2327
    fn singleton(item: T) -> LinkedListPerS<T> { LinkedListPerS { head: Some(Box::new(NodeP { vasingleton54,2409
    fn set(&self, index: N, item: T) -> Result<LinkedListPerS<T>, &'static str> where T: Clone {set55,2543
    fn subseq_copy(&self, start: N, length: N) -> LinkedListPerS<T> where T: Clone {subseq_copy71,3379
impl<T: std::fmt::Debug> std::fmt::Debug for LinkedListPerS<T> {LinkedListPerS82,4130
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { fmt83,4195

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/MathSeq.rs,2271
pub struct MathS<T> { pub data: Vec<T> }MathS16,578
pub struct MathS<T> { pub data: Vec<T> }data16,578
pub trait MathSeqTrait<T> {MathSeqTrait19,649
    fn new(length: N, init_value: T) -> MathS<T>new23,805
    fn empty() -> MathS<T>;empty30,961
    fn singleton(item: T) -> MathS<T>;singleton35,1088
    fn length(&self) -> N;length39,1199
    fn nth(&self, index: N) -> &T;nth43,1326
    fn set(&mut self, index: N, value: T) -> Result<&mut MathS<T>, &'static str>;set47,1449
    fn add_last(&mut self, value: T) -> &mut MathS<T>;add_last52,1804
    fn delete_last(&mut self) -> Option<T>;delete_last56,1934
    fn subseq(&self, start: N, length: N) -> &[T];subseq60,2086
    fn subseq_copy(&self, start: N, length: N) -> MathS<T>subseq_copy65,2273
    fn isEmpty(&self) -> B;isEmpty72,2448
    fn isSingleton(&self) -> B;isSingleton77,2563
    fn domain(&self) -> Vec<N>;domain81,2673
    fn range(&self) -> Vec<T>range85,2841
    fn multiset_range(&self) -> Vec<(N, T)>multiset_range91,3043
impl<T> MathSeq<T> for MathS<T> {MathS96,3130
    fn new(length: N, init_value: T) -> MathS<T>new98,3203
    fn empty() -> MathS<T> { MathS { data: Vec::new() } }empty104,3364
    fn singleton(item: T) -> MathS<T> { MathS { data: vec![item] } }singleton107,3457
    fn length(&self) -> N { self.data.len() }length110,3561
    fn nth(&self, index: N) -> &T { &self.data[index] }nth113,3641
    fn set(&mut self, index: N, value: T) -> Result<&mut MathS<T>, &'static str> {set116,3731
    fn add_last(&mut self, value: T) -> &mut MathS<T> { self.data.push(value); self }add_last123,4183
    fn delete_last(&mut self) -> Option<T> { self.data.pop() }delete_last126,4303
    fn subseq(&self, start: N, length: N) -> &[T] {subseq129,4400
    fn subseq_copy(&self, start: N, length: N) -> MathS<T>subseq_copy137,4639
    fn isEmpty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }isEmpty149,4995
    fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton152,5115
    fn domain(&self) -> Vec<N> { (0..self.data.len()).collect() }domain155,5250
    fn range(&self) -> Vec<T>range158,5362
    fn multiset_range(&self) -> Vec<(N, T)>multiset_range171,5741

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPerChap18.rs,2995
pub trait ArraySeqPerChap18Trait {ArraySeqPerChap18Trait6,129
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T>;tabulate8,246
    fn map<T, U: Clone>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U>;map11,385
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;append14,515
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, pred: impl Fn(&T) -> B) -> ArrayPerS<T>;filter17,691
    fn update<T: Clone + Eq>(a: &ArrayPerS<T>, item_at: (N, T)) -> ArrayPerS<T>;update22,930
    fn inject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T>;inject25,1085
    fn ninject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T>;ninject28,1234
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate31,1402
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A)iteratePrefixes34,1539
    fn reduce<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce37,1738
    fn scan<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayPerS<T>, scan40,1868
    fn flatten<T: Clone + Eq>(ss: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T>;flatten43,2035
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(a: &ArrayPerS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O)collect46,2188
impl<T2> ArraySeqPerChap18 for ArrayPerS<T2> {ArrayPerS49,2322
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T> {tabulate50,2369
    fn map<T, U: Clone>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U> {map54,2531
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {append62,2921
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, pred: impl Fn(&T) -> B) -> ArrayPerS<T> {filter70,3327
    fn update<T: Clone + Eq>(a: &ArrayPerS<T>, (index, item): (N, T)) -> ArrayPerS<T> {update75,3587
    fn inject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T> {inject78,3762
    fn ninject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T> {ninject87,4197
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate92,4502
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A)iteratePrefixes95,4685
    fn reduce<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce101,5023
        fn rec<T: Clone + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec102,5110
    fn scan<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayPerS<T>, scan108,5429
        fn rec<T: Clone + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec109,5530
    fn flatten<T: Clone + Eq>(ss: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T> {flatten119,6070
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(a: &ArrayPerS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O)collect124,6345

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPerChap19.rs,2714
pub trait LinkedListPerChap19Trait {LinkedListPerChap19Trait7,195
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T>;tabulate8,232
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U>;map9,299
    fn select<'a, T>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T> wherselect10,389
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;append11,497
    fn append2<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;append212,589
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T>;deflate13,682
    fn filter<T: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T>;filter14,757
    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate15,847
    fn reduce<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;reduce16,942
    fn scan<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T) where F:scan17,1037
    fn flatten<T: Clone>(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T>;flatten18,1151
    fn inject<T: Clone + Eq>(values: &LinkedListPerS<T>, changes: &LinkedListPerS<(N, T)>) -> Liinject19,1237
impl<T2> LinkedListPerChap19 for LinkedListPerS<T2> {LinkedListPerS22,1353
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T> { <LinkedListPerS<T> as ListPetabulate23,1407
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U> { <Linkmap24,1530
    fn select<'a, T>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T> wherselect25,1672
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> { <Liappend33,2138
    fn append2<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> { <Lappend235,2286
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T> { deflate37,2435
    fn filter<T: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T> { <Linkfilter41,2658
    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { iterate43,2804
    fn reduce<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T { reduce45,2959
    fn scan<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T) where F:scan47,3114
    fn flatten<T: Clone>(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T> { <LinkedLiflatten49,3286
    fn inject<T: Clone + Eq>(values: &LinkedListPerS<T>, changes: &LinkedListPerS<(N, T)>) -> Liinject51,3426

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListPerChap18.rs,2931
pub trait LinkedListPerChap18Trait {LinkedListPerChap18Trait6,140
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T>;tabulate8,252
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U>;map11,396
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;append14,536
    fn filter<T: Clone>(a: &LinkedListPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListPerS<T>;filter17,722
    fn update<T: Clone>(a: &LinkedListPerS<T>, item_at: (N, T)) -> LinkedListPerS<T>;update22,966
    fn inject<T: Clone + Eq>(a: &LinkedListPerS<T>, updates: &LinkedListPerS<(N, T)>) -> LinkedLinject25,1126
    fn ninject<T: Clone + Eq>(a: &LinkedListPerS<T>, updates: &LinkedListPerS<(N, T)>) -> Linkedninject28,1290
    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate31,1473
    fn iteratePrefixes<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A)iteratePrefixes34,1610
    fn reduce<T: Clone>(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce37,1814
    fn scan<T: Clone>(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListPerSscan40,1944
    fn flatten<T: Clone>(ss: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T>;flatten43,2116
    fn collect<A: Clone + Eq, Bv: Clone>(a: &LinkedListPerS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O)collect46,2279
impl<T2> LinkedListPerChap18 for LinkedListPerS<T2> {LinkedListPerS49,2423
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T> {tabulate50,2477
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U> {map55,2678
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {append60,3031
    fn filter<T: Clone>(a: &LinkedListPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListPerS<T> {filter67,3519
    fn update<T: Clone>(a: &LinkedListPerS<T>, (index, item): (N, T)) -> LinkedListPerS<T> {update75,3895
    fn inject<T: Clone + Eq>(a: &LinkedListPerS<T>, updates: &LinkedListPerS<(N, T)>) -> LinkedLinject83,4342
    fn ninject<T: Clone + Eq>(a: &LinkedListPerS<T>, updates: &LinkedListPerS<(N, T)>) -> Linkedninject97,5084
    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate111,5819
    fn iteratePrefixes<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A)iteratePrefixes114,6073
    fn reduce<T: Clone>(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce119,6518
    fn scan<T: Clone>(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListPerSscan125,7110
    fn flatten<T: Clone>(ss: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T> {flatten133,7770
    fn collect<A: Clone + Eq, Bv: Clone>(a: &LinkedListPerS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O)collect141,8298

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEph.rs,2367
pub struct ArrayEphS<T> { pub data: Box<[T]> }ArrayEphS7,210
pub struct ArrayEphS<T> { pub data: Box<[T]> }data7,210
pub trait ArraySeqEphTrait<T> {ArraySeqEphTrait10,297
    fn new(length: N, init_value: T) -> ArrayEphS<T> where T: Clone;new12,366
    fn length(&self) -> N;length14,467
    fn nth(&self, index: N) -> &T;nth16,526
    fn empty() -> ArrayEphS<T>;empty18,593
    fn set(&mut self, index: N, item: T) -> Result<&mut ArrayEphS<T>, &'static str>;set20,657
    fn singleton(item: T) -> ArrayEphS<T>;singleton22,774
    fn isEmpty(&self) -> B;isEmpty24,849
    fn isSingleton(&self) -> B;isSingleton26,909
    fn subseq_copy(&self, start: N, length: N) -> ArrayEphS<T> where T: Clone + Eq;subseq_copy28,984
impl<T> ArrayEphS<T> {ArrayEphS31,1071
    pub fn subseq(&self, start: N, length: N) -> &[T] {subseq32,1094
    pub fn subseq_copy(&self, start: N, length: N) -> ArrayEphS<T>subseq_copy38,1297
    pub fn from_vec(v: Vec<T>) -> ArrayEphS<T> { ArrayEphS { data: v.into_boxed_slice() } }from_vec51,1851
    pub fn update(&mut self, (index, item): (N, T)) -> &mut ArrayEphS<T> {update52,1943
impl<T: Eq> PartialEq for ArrayEphS<T> {ArrayEphS58,2104
    fn eq(&self, other: &Self) -> bool {eq59,2145
impl<T: Eq> Eq for ArrayEphS<T> {}ArrayEphS65,2355
impl<T: std::fmt::Debug> std::fmt::Debug for ArrayEphS<T> {ArrayEphS67,2391
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt68,2451
impl<T> ArraySeqEph<T> for ArrayEphS<T> {ArrayEphS74,2639
    fn new(length: N, init_value: T) -> ArrayEphS<T> where T: Clone { ArrayEphS::from_vec(vec![inew75,2681
    fn length(&self) -> N { self.data.len() }length76,2799
    fn nth(&self, index: N) -> &T { &self.data[index] }nth77,2845
    fn empty() -> ArrayEphS<T> { ArrayEphS::from_vec(Vec::new()) }empty78,2901
    fn set(&mut self, index: N, item: T) -> Result<&mut ArrayEphS<T>, &'static str> {set79,2968
    fn singleton(item: T) -> ArrayEphS<T> { ArrayEphS::from_vec(vec![item]) }singleton82,3169
    fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }isEmpty83,3247
    fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton84,3332
    fn subseq_copy(&self, start: N, length: N) -> ArrayEphS<T> where T: Clone + Eq { self.subseqsubseq_copy85,3421

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEphChap18.rs,2820
pub trait ArraySeqEphChap18Trait {ArraySeqEphChap18Trait6,132
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayEphS<T>;tabulate7,167
    fn map<T, U: Clone>(a: &ArrayEphS<T>, f: impl Fn(&T) -> U) -> ArrayEphS<U>;map8,229
    fn append<T: Clone + Eq>(a: &ArrayEphS<T>, b: &ArrayEphS<T>) -> ArrayEphS<T>;append9,309
    fn filter<T: Clone + Eq>(a: &ArrayEphS<T>, pred: impl Fn(&T) -> B) -> ArrayEphS<T>;filter10,391
    fn update<T: Clone + Eq>(a: &mut ArrayEphS<T>, item_at: (N, T)) -> &mut ArrayEphS<T>;update11,479
    fn inject<T: Clone + Eq>(a: &ArrayEphS<T>, updates: &ArrayEphS<(N, T)>) -> ArrayEphS<T>;inject12,569
    fn ninject<T: Clone + Eq>(a: &ArrayEphS<T>, updates: &ArrayEphS<(N, T)>) -> ArrayEphS<T>;ninject13,662
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate14,756
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &ArrayEphS<T>, f: impl Fn(&A, &T) -> A, x: A)iteratePrefixes15,851
    fn reduce<T: Clone + Eq>(a: &ArrayEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce16,970
    fn scan<T: Clone + Eq>(a: &ArrayEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayEphS<T>, scan17,1056
    fn flatten<T: Clone + Eq>(ss: &ArrayEphS<ArrayEphS<T>>) -> ArrayEphS<T>;flatten18,1156
    fn collect<A: Clone + Eq, Bv: Clone>(a: &ArrayEphS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> Acollect19,1233
impl<T2> ArraySeqEphChap18 for ArrayEphS<T2> {ArrayEphS22,1362
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayEphS<T> {tabulate23,1409
    fn map<T, U: Clone>(a: &ArrayEphS<T>, f: impl Fn(&T) -> U) -> ArrayEphS<U> {map28,1600
    fn append<T: Clone + Eq>(a: &ArrayEphS<T>, b: &ArrayEphS<T>) -> ArrayEphS<T> {append37,2013
    fn filter<T: Clone + Eq>(a: &ArrayEphS<T>, pred: impl Fn(&T) -> B) -> ArrayEphS<T> {filter46,2543
    fn update<T: Clone + Eq>(a: &mut ArrayEphS<T>, (index, item): (N, T)) -> &mut ArrayEphS<T> {update52,2852
    fn inject<T: Clone + Eq>(a: &ArrayEphS<T>, updates: &ArrayEphS<(N, T)>) -> ArrayEphS<T> {inject53,2975
    fn ninject<T: Clone + Eq>(a: &ArrayEphS<T>, updates: &ArrayEphS<(N, T)>) -> ArrayEphS<T> { Sninject58,3230
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate59,3352
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &ArrayEphS<T>, f: impl Fn(&A, &T) -> A, x: A)iteratePrefixes62,3535
    fn reduce<T: Clone + Eq>(a: &ArrayEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce70,3973
    fn scan<T: Clone + Eq>(a: &ArrayEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayEphS<T>, scan73,4148
    fn flatten<T: Clone + Eq>(ss: &ArrayEphS<ArrayEphS<T>>) -> ArrayEphS<T> {flatten79,4546
    fn collect<A: Clone + Eq, Bv: Clone>(a: &ArrayEphS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> Acollect89,5288

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,143
pub type N = usize;N8,264
pub enum B { True, False }B12,395
pub enum B { True, False }True12,395
pub enum B { True, False }False12,395

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqEphChap19.rs,2425
pub trait ArraySeqEphChap19Trait {ArraySeqEphChap19Trait7,183
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayEphS<T>;tabulate8,218
    fn map<T, U: Clone>(a: &ArrayEphS<T>, f: impl Fn(&T) -> U) -> ArrayEphS<U>;map9,280
    fn select<'a, T: Clone>(a: &'a ArrayEphS<T>, b: &'a ArrayEphS<T>, i: N) -> Option<T>;select10,360
    fn append<T: Clone + Eq>(a: &ArrayEphS<T>, b: &ArrayEphS<T>) -> ArrayEphS<T>;append11,450
    fn append2<T: Clone + Eq>(a: &ArrayEphS<T>, b: &ArrayEphS<T>) -> ArrayEphS<T>;append212,532
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArrayEphS<T>;deflate13,615
    fn filter<T: Clone + Eq>(a: &ArrayEphS<T>, f: impl Fn(&T) -> B) -> ArrayEphS<T>;filter14,685
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate15,770
    fn reduce<T: Clone + Eq, F>(a: &ArrayEphS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;reduce16,865
    fn scan<T: Clone + Eq, F>(a: &ArrayEphS<T>, f: &F, id: T) -> (ArrayEphS<T>, T) where F: Fn(&scan17,960
    fn flatten<T: Clone + Eq>(s: &ArrayEphS<ArrayEphS<T>>) -> ArrayEphS<T>;flatten18,1069
impl<T2> ArraySeqEphChap19 for ArrayEphS<T2> {ArrayEphS21,1148
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayEphS<T> { <ArrayEphS<T> as ArraySeqEphChap1tabulate22,1195
    fn map<T, U: Clone>(a: &ArrayEphS<T>, f: impl Fn(&T) -> U) -> ArrayEphS<U> { <ArrayEphS<T2> map23,1312
    fn select<'a, T: Clone>(a: &'a ArrayEphS<T>, b: &'a ArrayEphS<T>, i: N) -> Option<T> {select24,1443
    fn append<T: Clone + Eq>(a: &ArrayEphS<T>, b: &ArrayEphS<T>) -> ArrayEphS<T> { <ArrayEphS<T2append27,1691
    fn append2<T: Clone + Eq>(a: &ArrayEphS<T>, b: &ArrayEphS<T>) -> ArrayEphS<T> { <ArrayEphS<Tappend228,1827
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArrayEphS<T> { if f(x) == B::True { <Arrdeflate29,1964
    fn filter<T: Clone + Eq>(a: &ArrayEphS<T>, f: impl Fn(&T) -> B) -> ArrayEphS<T> { <ArrayEphSfilter30,2166
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { iterate31,2305
    fn reduce<T: Clone + Eq, F>(a: &ArrayEphS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T { reduce32,2458
    fn scan<T: Clone + Eq, F>(a: &ArrayEphS<T>, f: &F, id: T) -> (ArrayEphS<T>, T) where F: Fn(&scan33,2611
    fn flatten<T: Clone + Eq>(s: &ArrayEphS<ArrayEphS<T>>) -> ArrayEphS<T> { <ArrayEphS<T2> as Aflatten34,2776

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main7,236

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPerChap19.rs,1350
pub trait AVLTreeSeqPerChap19Trait {AVLTreeSeqPerChap19Trait8,215
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreePerS<T>tabulate9,252
    fn map<T, U>(a: &AVLTreePerS<T>, f: impl Fn(&T) -> U) -> AVLTreePerS<U>map11,358
    fn select<T>(a: &AVLTreePerS<T>, b: &AVLTreePerS<T>, i: N) -> Option<T>select13,510
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreePerS<T>, b: &AVLTreePerS<T>) -> AVLTreappend15,629
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreePerS<T>;deflate16,735
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreePerS<T>, f: impl Fn(&T) -> B) -> AVLTrfilter17,830
impl<T2: Ord + Copy + Debug + Display> AVLTreeSeqPerChap19 for AVLTreePerS<T2> {AVLTreePerS20,940
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreePerS<T>tabulate21,1021
    fn map<T, U>(a: &AVLTreePerS<T>, f: impl Fn(&T) -> U) -> AVLTreePerS<U>map23,1186
    fn select<T>(a: &AVLTreePerS<T>, b: &AVLTreePerS<T>, i: N) -> Option<T>select25,1392
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreePerS<T>, b: &AVLTreePerS<T>) -> AVLTreappend31,1856
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreePerS<T> {deflate32,2019
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreePerS<T>, f: impl Fn(&T) -> B) -> AVLTrfilter35,2259

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,1205
pub mod Types;Types7,226
pub mod MathSeq;MathSeq9,242
pub mod LinkedListPer;LinkedListPer13,324
pub mod LinkedListPerChap18;LinkedListPerChap1815,415
pub mod LinkedListPerChap19;LinkedListPerChap1917,506
pub mod LinkedListEph;LinkedListEph20,598
pub mod ArraySeqPer;ArraySeqPer23,690
pub mod ArraySeqPerChap18;ArraySeqPerChap1826,771
pub mod ArraySeqPerChap19;ArraySeqPerChap1928,856
pub mod ArraySeqEph;ArraySeqEph31,942
pub mod ArraySeqEphChap18;ArraySeqEphChap1833,1022
pub mod ArraySeqEphChap19;ArraySeqEphChap1935,1107
pub mod AVLTreeSeqPer;AVLTreeSeqPer38,1193
pub mod AVLTreeSeqPerChap18;AVLTreeSeqPerChap1840,1284
pub mod AVLTreeSeqPerChap19;AVLTreeSeqPerChap1942,1375
pub mod AVLTreeSeqEph;AVLTreeSeqEph45,1467
pub mod AVLTreeSeqEphChap18;AVLTreeSeqEphChap1847,1555
pub mod AVLTreeSeqEphChap19; AVLTreeSeqEphChap1949,1646
macro_rules! MathSeq {MathSeq53,1755
macro_rules! LinkedListEph {LinkedListEph60,2013
macro_rules! LinkedListPer {LinkedListPer75,2666
macro_rules! ArraySeqPer {ArraySeqPer84,3022
macro_rules! ArraySeqEph {ArraySeqEph91,3311
macro_rules! AVLTreeSeqEph {AVLTreeSeqEph98,3600
macro_rules! AVLTreeSeqPer {AVLTreeSeqPer113,4006

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqEph.rs,5566
type Link<T> = Option<Box<AVLTreeNode<T>>>;Link7,186
pub struct AVLTreeNode<T: Copy + Debug> {AVLTreeNode9,231
    pub value: T,value10,273
    pub height: N,height11,291
    pub left_size: N,left_size12,310
    pub right_size: N,right_size13,332
    pub left: Link<T>,left14,355
    pub right: Link<T>,right15,378
    pub index: N,index16,402
impl<T: Copy + Debug> AVLTreeNode<T> { fn new(value: T, index: N) -> Self { AVLTreeNode { value,AVLTreeNode19,423
impl<T: Copy + Debug> AVLTreeNode<T> { fn new(value: T, index: N) -> Self { AVLTreeNode { value,new19,423
pub struct AVLTreeEphS<T: Copy + Debug> { pub root: Link<T>, pub next_key: N }AVLTreeEphS21,598
pub struct AVLTreeEphS<T: Copy + Debug> { pub root: Link<T>, pub next_key: N }root21,598
pub struct AVLTreeEphS<T: Copy + Debug> { pub root: Link<T>, pub next_key: N }next_key21,598
pub trait AVLTreeSeqEphTrait<T: Copy + Debug> {AVLTreeSeqEphTrait23,678
    fn empty() -> AVLTreeEphS<T>;empty25,764
    fn new() -> AVLTreeEphS<T>;new27,836
    fn length(&self) -> N;length29,906
    fn nth(&self, index: N) -> &T;nth31,979
    fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeEphS<T>, &'static str>;set33,1060
    fn singleton(item: T) -> AVLTreeEphS<T>;singleton35,1185
    fn isEmpty(&self) -> B;isEmpty37,1268
    fn isSingleton(&self) -> B;isSingleton39,1334
    fn subseq_copy(&self, start: N, length: N) -> AVLTreeEphS<T> where T: Clone + Eq;subseq_copy41,1424
impl<T: Copy + Debug> AVLTreeEphS<T> {AVLTreeEphS44,1513
    pub fn new_root() -> Self { AVLTreeEphS { root: None, next_key: 0 } }new_root45,1552
    pub fn new() -> Self { Self::new_root() }new46,1626
    pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeEphS<T> { let _ = <AVLTreeEphupdate47,1672
    pub fn from_vec(values: Vec<T>) -> AVLTreeEphS<T> where T: Clone { let length = values.len()from_vec48,1826
    pub fn to_arrayseq(&self) -> ArrayS<T> where T: Clone { let len = self.length(); if len == 0to_arrayseq49,2114
    pub fn iter<'a>(&'a self) -> AVLTreeSeqIterEph<'a, T> { AVLTreeSeqIterEph::new(&self.root) }iter50,2506
    pub fn push_back(&mut self, value: T) { let len = self.length(); let node = insert_at_link(spush_back51,2603
    pub fn contains_value(&self, target: &T) -> B where T: PartialEq { for v in self.iter() { ifcontains_value52,2769
    pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value53,2911
    pub fn delete_value(&mut self, target: &T) -> bool where T: Clone + PartialEq { let len = sedelete_value54,2983
impl<T: Copy + Debug> AVLTreeSeqEph<T> for AVLTreeEphS<T> {AVLTreeEphS57,3483
    fn empty() -> AVLTreeEphS<T> { AVLTreeEphS::new_root() }empty58,3543
    fn new() -> AVLTreeEphS<T> { AVLTreeEphS::new_root() }new59,3604
    fn length(&self) -> N { size_link(&self.root) }length60,3663
    fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth61,3715
    fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeEphS<T>, &'static str> { set_linkset62,3781
    fn singleton(item: T) -> AVLTreeEphS<T> { let mut t = AVLTreeEphS::new_root(); t.root = insesingleton63,3920
    fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty64,4073
    fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton65,4156
    fn subseq_copy(&self, start: N, length: N) -> AVLTreeEphS<T> where T: Clone + Eq { let n = ssubseq_copy66,4243
pub struct AVLTreeSeqIterEph<'a, T: Copy + Debug> { stack: Vec<&'a AVLTreeNode<T>>, current: OptAVLTreeSeqIterEph69,4621
pub struct AVLTreeSeqIterEph<'a, T: Copy + Debug> { stack: Vec<&'a AVLTreeNode<T>>, current: Optstack69,4621
pub struct AVLTreeSeqIterEph<'a, T: Copy + Debug> { stack: Vec<&'a AVLTreeNode<T>>, current: Optcurrent69,4621
impl<'a, T: Copy + Debug> AVLTreeSeqIterEph<'a, T> { fn new(root: &'a Link<T>) -> Self { let mutAVLTreeSeqIterEph70,4743
impl<'a, T: Copy + Debug> AVLTreeSeqIterEph<'a, T> { fn new(root: &'a Link<T>) -> Self { let mutnew70,4743
impl<'a, T: Copy + Debug> AVLTreeSeqIterEph<'a, T> { fn new(root: &'a Link<T>) -> Self { let mutpush_left70,4743
impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIterEph<'a, T> { type Item = &'a T; fn next(&muAVLTreeSeqIterEph71,5089
impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIterEph<'a, T> { type Item = &'a T; fn next(&muItem71,5089
impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIterEph<'a, T> { type Item = &'a T; fn next(&munext71,5089
fn h<T: Copy + Debug>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h73,5330
fn size_link<T: Copy + Debug>(n: &Link<T>) -> N { if let Some(b) = n { 1 + b.left_size + b.rightsize_link74,5409
fn update_meta<T: Copy + Debug>(n: &mut Box<AVLTreeNode<T>>) { n.left_size = size_link(&n.left);update_meta75,5526
fn rotate_right<T: Copy + Debug>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> { let mut x rotate_right76,5731
fn rotate_left<T: Copy + Debug>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> { let mut y =rotate_left77,6030
fn rebalance<T: Copy + Debug>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> { update_meta(&rebalance78,6327
pub(crate) fn insert_at_link<T: Copy + Debug>(node: Link<T>, index: N, value: T, next_key: &mut insert_at_link79,6892
fn nth_link<'a, T: Copy + Debug>(node: &'a Link<T>, index: N) -> &'a T { let n = node.as_ref().enth_link80,7436
fn set_link<T: Copy + Debug>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str>set_link81,7736

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPerChap18.rs,948
pub trait AVLTreeSeqPerChap18Trait {AVLTreeSeqPerChap18Trait7,162
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreePerS<T>tabulate9,283
    fn map<T, U>(a: &AVLTreePerS<T>, f: impl Fn(&T) -> U) -> AVLTreePerS<U>map12,476
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreePerS<T>, b: &AVLTreePerS<T>) -> AVLTreappend15,698
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreePerS<T>, pred: impl Fn(&T) -> B) -> AVfilter17,897
impl<T2: Ord + Copy + Debug + Display> AVLTreeSeqPerChap18 for AVLTreePerS<T2> {AVLTreePerS20,1010
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreePerS<T>tabulate21,1091
    fn map<T, U>(a: &AVLTreePerS<T>, f: impl Fn(&T) -> U) -> AVLTreePerS<U>map27,1353
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreePerS<T>, b: &AVLTreePerS<T>) -> AVLTreappend33,1718
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreePerS<T>, pred: impl Fn(&T) -> B) -> AVfilter39,2112

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqPer.rs,3688
type Link<T> = Option<Rc<Node<T>>>;Link7,155
struct Node<T: Copy + Debug> {Node9,192
    value: T,value10,223
    height: N,height11,237
    size: N,size12,252
    left: Link<T>,left13,265
    right: Link<T>,right14,284
fn height<T: Copy + Debug>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.height) }height17,307
fn size<T: Copy + Debug>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.size) }size18,391
fn mk<T: Copy + Debug>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {mk20,472
fn rotate_right<T: Copy + Debug>(y: Rc<Node<T>>) -> Rc<Node<T>> {rotate_right26,729
fn rotate_left<T: Copy + Debug>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left33,1005
fn rebalance<T: Copy + Debug>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance40,1280
fn nth_ref<'a, T: Copy + Debug>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref55,1974
fn set_rec<T: Copy + Debug>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> set_rec65,2320
fn inorder_collect<T: Copy + Debug>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect83,3013
fn build_balanced_from_slice<T: Copy + Debug>(a: &[T]) -> Link<T> {build_balanced_from_slice91,3226
    fn rec<T: Copy + Debug>(a: &[T]) -> Link<T> {rec92,3294
pub struct AVLTreePerS<T: Copy + Debug> { root: Link<T> }AVLTreePerS102,3547
pub struct AVLTreePerS<T: Copy + Debug> { root: Link<T> }root102,3547
pub trait AVLTreeSeqPerTrait<T: Copy + Debug> {AVLTreeSeqPerTrait104,3606
    fn empty() -> AVLTreePerS<T>;empty106,3691
    fn new() -> AVLTreePerS<T>;new108,3762
    fn length(&self) -> N;length110,3831
    fn nth(&self, index: N) -> &T;nth112,3903
    fn set(&self, index: N, item: T) -> Result<AVLTreePerS<T>, &'static str>;set114,4042
    fn singleton(item: T) -> AVLTreePerS<T>;singleton116,4157
    fn isEmpty(&self) -> B;isEmpty118,4239
    fn isSingleton(&self) -> B;isSingleton120,4304
    fn subseq_copy(&self, start: N, length: N) -> AVLTreePerS<T> where T: Clone + Eq;subseq_copy122,4389
    fn from_vec(values: Vec<T>) -> AVLTreePerS<T> where T: Clone;from_vec124,4534
    fn values_in_order(&self) -> Vec<T> where T: Clone;values_in_order126,4640
impl<T: Copy + Debug> AVLTreeSeqPer<T> for AVLTreePerS<T> {AVLTreePerS129,4699
    fn empty() -> AVLTreePerS<T> { AVLTreePerS { root: None } }empty130,4759
    fn new() -> AVLTreePerS<T> { <Self as AVLTreeSeqPer<T>>::empty() }new131,4823
    fn length(&self) -> N { size(&self.root) }length132,4894
    fn nth(&self, index: N) -> &T { nth_ref(&self.root, index) }nth133,4941
    fn set(&self, index: N, item: T) -> Result<AVLTreePerS<T>, &'static str> {set134,5006
    fn singleton(item: T) -> AVLTreePerS<T> { AVLTreePerS { root: Some(mk(item, None, None)) } }singleton137,5159
    fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty138,5256
    fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton139,5339
    fn subseq_copy(&self, start: N, length: N) -> AVLTreePerS<T> where T: Clone + Eq {subseq_copy140,5426
    fn from_vec(values: Vec<T>) -> AVLTreePerS<T> where T: Clone { AVLTreePerS { root: build_balfrom_vec148,5915
    fn values_in_order(&self) -> Vec<T> where T: Clone { let mut out = Vec::with_capacity(self.lvalues_in_order149,6045
impl<T: Eq + Copy + Debug> PartialEq for AVLTreePerS<T> {AVLTreePerS152,6199
    fn eq(&self, other: &Self) -> bool {eq153,6257
impl<T: Eq + Copy + Debug> Eq for AVLTreePerS<T> {}AVLTreePerS159,6467
impl<T: Debug + Copy> std::fmt::Debug for AVLTreePerS<T> {AVLTreePerS161,6520
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt162,6579

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListEph.rs,2337
pub struct NodeE<T> { pub value: T, pub next: Option<Box<NodeE<T>>> }NodeE5,115
pub struct NodeE<T> { pub value: T, pub next: Option<Box<NodeE<T>>> }value5,115
pub struct NodeE<T> { pub value: T, pub next: Option<Box<NodeE<T>>> }next5,115
pub struct LinkedListEphS<T> { head: Option<Box<NodeE<T>>>, len: N }LinkedListEphS7,186
pub struct LinkedListEphS<T> { head: Option<Box<NodeE<T>>>, len: N }head7,186
pub struct LinkedListEphS<T> { head: Option<Box<NodeE<T>>>, len: N }len7,186
pub trait LinkedListEphTrait<T> {LinkedListEphTrait9,256
    fn empty() -> LinkedListEphS<T>;empty10,290
    fn new(length: N, init_value: T) -> LinkedListEphS<T> where T: Clone;new11,327
    fn length(&self) -> N;length12,401
    fn nth(&self, index: N) -> &T;nth13,428
    fn isEmpty(&self) -> B;isEmpty14,463
    fn isSingleton(&self) -> B;isSingleton15,491
    fn singleton(item: T) -> LinkedListEphS<T>;singleton16,523
    fn set(&mut self, index: N, item: T) -> Result<&mut LinkedListEphS<T>, &'static str>;set17,571
    fn subseq_copy(&self, start: N, length: N) -> LinkedListEphS<T> where T: Clone;subseq_copy18,661
impl<T> LinkedListEphS<T> {LinkedListEphS21,748
    fn push_front_node(&mut self, node: Box<NodeE<T>>) {push_front_node22,776
impl<T> LinkedListEph<T> for LinkedListEphS<T> {LinkedListEphS27,931
    fn empty() -> LinkedListEphS<T> { LinkedListEphS { head: None, len: 0 } }empty28,980
    fn new(length: N, init_value: T) -> LinkedListEphS<T> where T: Clone {new29,1058
    fn length(&self) -> N { self.len }length37,1560
    fn nth(&self, index: N) -> &T { nth38,1599
    fn isEmpty(&self) -> B { if self.len == 0 { B::True } else { B::False } }isEmpty42,1833
    fn isSingleton(&self) -> B { if self.len == 1 { B::True } else { B::False } }isSingleton43,1911
    fn singleton(item: T) -> LinkedListEphS<T> { LinkedListEphS { head: Some(Box::new(NodeE { vasingleton44,1993
    fn set(&mut self, index: N, item: T) -> Result<&mut LinkedListEphS<T>, &'static str> { set45,2127
    fn subseq_copy(&self, start: N, length: N) -> LinkedListEphS<T> where T: Clone {subseq_copy49,2425
impl<T: std::fmt::Debug> std::fmt::Debug for LinkedListEphS<T> {LinkedListEphS61,3245
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { let mut v = Vec::with_cfmt62,3310

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqPer.rs,2170
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
impl<T> ArraySeqPer<T> for ArrayPerS<T> {ArrayPerS68,2605
    fn new(length: N, init_value: T) -> ArrayPerS<T> where T: Clone {new69,2647
    fn length(&self) -> N { self.data.len() }length72,2777
    fn nth(&self, index: N) -> &T { &self.data[index] }nth73,2823
    fn empty() -> ArrayPerS<T> { ArrayPerS::from_vec(Vec::new()) }empty74,2879
    fn set(&self, index: N, item: T) -> Result<ArrayPerS<T>, &'static str> where T: Clone {set75,2946
    fn singleton(item: T) -> ArrayPerS<T> { ArrayPerS::from_vec(vec![item]) }singleton81,3227
    fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }isEmpty82,3305
    fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton83,3390
    fn subseq_copy(&self, start: N, length: N) -> ArrayPerS<T> where T: Clone + Eq {subseq_copy84,3479

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
/home/milnes/.rusty-tags/cache/criterion-9645543562806322964.emacs,include
/home/milnes/.rusty-tags/cache/tree_collections-7614627914474005019.emacs,include
