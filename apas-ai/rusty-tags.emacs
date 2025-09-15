
/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPerChap19.rs,3181
pub mod ArraySeqStPerChap19 {ArraySeqStPerChap193,46
    pub trait ArraySeqStPerChap19Trait<T: MtT> {ArraySeqStPerChap19Trait10,246
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate12,381
        fn map<U: MtT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map14,526
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T>;select16,650
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append18,795
        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append220,925
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T>;deflate22,1044
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter24,1197
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate28,1455
        fn reduce<F>(a: &ArrayStPerS<T>, f: &F, id: T) -> Treduce30,1586
        fn scan<F>(a: &ArrayStPerS<T>, f: &F, id: T) -> (ArrayStPerS<T>, T)scan34,1739
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten38,1925
        fn inject(values: &ArrayStPerS<T>, changes: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject42,2195
        fn atomicWrite(atomicWrite44,2333
        fn inject_parallel2(values: &ArrayStPerS<T>, changes: &ArrayStPerS<Pair<N, T>>)inject_parallel250,2576
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins52,2695
        fn ninject_parallel2(ninject_parallel258,2963
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins62,3106
    impl<T: MtT> ArraySeqStPerChap19Trait<T> for ArrayStPerS<T> {ArrayStPerS69,3319
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate70,3385
        fn map<U: MtT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map74,3558
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T> {select77,3751
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append89,4131
        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append295,4405
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T> {deflate101,4680
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter108,4955
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate116,5378
        fn reduce<F>(a: &ArrayStPerS<T>, f: &F, id: T) -> Treduce130,5926
        fn scan<F>(a: &ArrayStPerS<T>, f: &F, id: T) -> (ArrayStPerS<T>, T)scan154,6765
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten203,8631
        fn inject(values: &ArrayStPerS<T>, changes: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> inject206,8785
        fn atomicWrite(atomicWrite218,9393
        fn inject_parallel2(inject_parallel2233,10038
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins260,11070
        fn ninject_parallel2(ninject_parallel2276,11696
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins303,12730

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStEphChap18.rs,2394
pub mod ArraySeqStEphChap18 {ArraySeqStEphChap183,51
    pub trait ArraySeqStEphChap18Trait<T: StT> {ArraySeqStEphChap18Trait7,162
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate8,211
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map9,279
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append10,368
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter11,454
        fn update(a: &mut ArraySeqStEphS<T>, item_at: (N, T)) -> &mut ArraySeqStEphS<T>;update12,541
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject13,630
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject14,731
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate15,833
        fn iteratePrefixes<A: StT>(iteratePrefixes16,920
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce21,1083
        fn scan(scan22,1163
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten27,1309
        fn collect<A: StT, Bv: StT>(collect28,1390
    impl<T: StT> ArraySeqStEphChap18Trait<T> for ArraySeqStEphS<T> {ArraySeqStEphS34,1576
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate35,1645
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map42,1891
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append55,2425
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter76,3179
        fn update(a: &mut ArraySeqStEphS<T>, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update88,3604
        fn inject(inject91,3746
        fn ninject(ninject102,4107
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate108,4292
        fn iteratePrefixes<A: StT>(iteratePrefixes115,4527
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce132,5133
        fn scan(scan139,5362
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten157,5962
        fn collect<A: StT, Bv: StT>(collect185,7048

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStEphChap19.rs,1424
pub mod AVLTreeSeqStEphChap19 {AVLTreeSeqStEphChap193,54
    pub trait AVLTreeSeqStEphChap19Trait<T: StT> {AVLTreeSeqStEphChap19Trait11,275
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T>;tabulate12,326
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U>;map13,396
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T>;select14,489
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T>;append15,577
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T>;deflate16,669
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T>;filter17,740
    impl<T: StT> AVLTreeSeqStEphChap19Trait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS20,835
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T> {tabulate21,908
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U> {map24,1072
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T> {select27,1254
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T> {append41,1899
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T> {deflate44,2083
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T> {filter51,2387

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStPer.rs,3950
pub mod AVLTreeSeqStPer {AVLTreeSeqStPer3,85
    type Link<T> = Option<Rc<Node<T>>>;Link9,238
    struct Node<T: StT> {Node11,279
        value: T,value12,305
        height: N,height13,323
        size: N,size14,342
        left: Link<T>,left15,359
        right: Link<T>,right16,382
    fn height<T: StT>(n: &Link<T>) -> N {height19,413
    fn size<T: StT>(n: &Link<T>) -> N {size22,504
    fn mk<T: StT>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {mk26,592
    fn rotate_right<T: StT>(y: Rc<Node<T>>) -> Rc<Node<T>> {rotate_right39,937
    fn rotate_left<T: StT>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left46,1228
    fn rebalance<T: StT>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance57,1570
    fn nth_ref<'a, T: StT>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref79,2407
    fn set_rec<T: StT>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> {set_rec94,2856
    fn inorder_collect<T: StT>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect112,3608
    fn build_balanced_from_slice<T: StT>(a: &[T]) -> Link<T> {build_balanced_from_slice120,3840
        fn rec<T: StT>(a: &[T]) -> Link<T> {rec121,3903
    pub struct AVLTreeSeqStPerS<T: StT> {AVLTreeSeqStPerS133,4213
        root: Link<T>,root134,4255
    pub trait AVLTreeSeqStPerTrait<T: StT> {AVLTreeSeqStPerTrait137,4285
        fn empty() -> Self;empty139,4371
        fn new() -> Self;new141,4440
        fn length(&self) -> N;length143,4507
        fn nth(&self, index: N) -> &T;nth145,4587
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set147,4736
        fn singleton(item: T) -> Self;singleton151,4887
        fn isEmpty(&self) -> B;isEmpty153,4967
        fn isSingleton(&self) -> B;isSingleton155,5040
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy157,5133
        fn from_vec(values: Vec<T>) -> Self;from_vec159,5256
        fn values_in_order(&self) -> Vec<T>;values_in_order161,5345
    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS164,5397
        fn empty() -> Self {empty165,5464
        fn new() -> Self {new168,5547
        fn length(&self) -> N {length171,5610
        fn nth(&self, index: N) -> &T {nth174,5681
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set177,5770
        fn singleton(item: T) -> Self {singleton182,5959
        fn isEmpty(&self) -> B {isEmpty187,6104
        fn isSingleton(&self) -> B {isSingleton194,6267
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy201,6434
        fn from_vec(values: Vec<T>) -> Self {from_vec215,6934
        fn values_in_order(&self) -> Vec<T> {values_in_order220,7097
    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS227,7288
        fn eq(&self, other: &Self) -> bool {eq228,7341
    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}AVLTreeSeqStPerS240,7667
    impl<T: StT> std::fmt::Debug for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS242,7715
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt243,7774
    impl<T: StT> AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS249,7966
        pub fn to_arrayseq(&self) -> ArrayStPerS<T> {to_arrayseq250,8005
        pub fn iter<'a>(&'a self) -> AVLTreeSeqStPerIter<'a, T> {iter255,8151
    pub struct AVLTreeSeqStPerIter<'a, T: StT> {AVLTreeSeqStPerIter263,8364
        stack: Vec<&'a Node<T>>,stack264,8413
        current: Option<&'a Node<T>>,current265,8446
    impl<'a, T: StT> AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter268,8491
        fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {push_left269,8541
    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter277,8751
        type Item = &'a T;Item278,8814
        fn next(&mut self) -> Option<Self::Item> {next279,8841
macro_rules! AVLTreeSeqStPer {AVLTreeSeqStPer293,9231

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/DirGraphStEphChap6_1.rs,3725
pub mod DirGraphStEphChap6_1 {DirGraphStEphChap6_13,77
pub struct DirGraphStEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {DirGraphStEph10,242
    V: Set<V>,V11,329
    A: Set<(V, V)>,A12,344
pub trait DirGraphStEphChap6_1Trait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> DirGraphStEphChap6_1Trait15,367
    fn empty() -> DirGraphStEph<V>;empty16,465
    fn FromSets(V: Set<V>, A: Set<(V, V)>) -> DirGraphStEph<V>;FromSets17,501
    fn vertices(&self) -> &Set<V>;vertices18,565
    fn arcs(&self) -> &Set<(V, V)>;arcs19,600
    fn sizeV(&self) -> N;sizeV20,636
    fn sizeA(&self) -> N;sizeA21,662
    fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor23,727
    fn NG(&self, v: &V) -> Set<V>;            // Out-neighbors by conventionNG24,770
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices25,847
    fn NPlus(&self, v: &V) -> Set<V>;         // Out-neighborsNPlus26,901
    fn NMinus(&self, v: &V) -> Set<V>;        // In-neighborsNMinus27,964
    fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices28,1026
    fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices29,1083
    fn Incident(&self, e: &(V, V), v: &V) -> B;Incident30,1141
    fn Degree(&self, v: &V) -> N;             // Out-degree by conventionDegree31,1189
    fn InDegree(&self, v: &V) -> N;InDegree32,1263
    fn OutDegree(&self, v: &V) -> N;OutDegree33,1299
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> DirGraphStEphChap6_1Trait<V> foDirGraphStEph36,1339
    fn empty() -> DirGraphStEph<V> { DirGraphStEph { V: SetLit![], A: SetLit![] } }empty37,1456
    fn FromSets(V: Set<V>, A: Set<(V, V)>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }FromSets38,1540
    fn vertices(&self) -> &Set<V> { &self.V }vertices39,1630
    fn arcs(&self) -> &Set<(V, V)> { &self.A }arcs40,1676
    fn sizeV(&self) -> N { self.V.size() }sizeV41,1723
    fn sizeA(&self) -> N { self.A.size() }sizeA42,1766
    fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor44,1810
    fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG49,2049
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices51,2102
    fn NPlus(&self, v: &V) -> Set<V> {NPlus60,2342
    fn NMinus(&self, v: &V) -> Set<V> {NMinus66,2536
    fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices72,2731
    fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices81,2981
    fn Incident(&self, e: &(V, V), v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } else { B:Incident90,3235
    fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree92,3343
    fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree93,3401
    fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree94,3462
impl<V: Eq + Hash + Clone + std::fmt::Debug + std::fmt::Display> std::fmt::Debug for DirGraphStEDirGraphStEph97,3526
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt98,3630
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for DirGraphSDirGraphStEph103,3801
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt104,3907
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for DirGraphStEph<V> DirGraphStEph109,4038
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for DirGraphStEph<V> eq109,4038
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for DirGraphStEph<V> {}DirGraphStEph110,4216
macro_rules! DirGraphLit {DirGraphLit117,4386
pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise125,4914

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStPer.rs,2775
pub mod LinkedListStPer {LinkedListStPer3,64
    pub struct NodeP<T: StT> {NodeP7,144
        pub value: T,value8,175
        pub next: Option<Box<NodeP<T>>>,next9,197
    pub struct LinkedListStPerS<T: StT> {LinkedListStPerS13,266
        head: Option<Box<NodeP<T>>>,head14,308
        len: N,len15,345
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait18,368
        fn empty() -> Self;empty19,413
        fn new(length: N, init_value: T) -> Self;new20,441
        fn length(&self) -> N;length21,491
        fn nth(&self, index: N) -> &T;nth22,522
        fn isEmpty(&self) -> B;isEmpty23,561
        fn isSingleton(&self) -> B;isSingleton24,593
        fn singleton(item: T) -> Self;singleton25,629
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set29,875
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy32,985
    impl<T: StT> LinkedListStPerS<T> {LinkedListStPerS35,1052
        fn push_front_node(&mut self, node: Box<NodeP<T>>) {push_front_node36,1091
        pub fn from_vec(v: Vec<T>) -> Self {from_vec43,1292
        pub fn iter<'a>(&'a self) -> LinkedListStPerIter<'a, T> {iter51,1557
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS58,1734
        fn empty() -> Self {empty59,1801
        fn new(length: N, init_value: T) -> Self {new62,1892
        fn length(&self) -> N {length81,2573
        fn nth(&self, index: N) -> &T {nth84,2636
        fn isEmpty(&self) -> B {isEmpty96,3003
        fn isSingleton(&self) -> B {isSingleton103,3161
        fn singleton(item: T) -> Self {singleton110,3323
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set119,3572
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy151,4700
    impl<T: StT> std::fmt::Debug for LinkedListStPerS<T> {LinkedListStPerS186,5841
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt187,5900
    pub struct LinkedListStPerIter<'a, T: StT> {LinkedListStPerIter198,6266
        cursor: Option<&'a NodeP<T>>,cursor199,6315
    impl<'a, T: StT> Iterator for LinkedListStPerIter<'a, T> {LinkedListStPerIter202,6360
        type Item = &'a T;Item203,6423
        fn next(&mut self) -> Option<Self::Item> {next204,6450
    impl<T: StT> PartialEq for LinkedListStPerS<T> {LinkedListStPerS214,6697
        fn eq(&self, other: &Self) -> bool {eq215,6750
    impl<T: StT> Eq for LinkedListStPerS<T> {}LinkedListStPerS232,7239
    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {LinkedListStPerS234,7287
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt235,7348
macro_rules! LinkedListStPer {LinkedListStPer254,7874

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStEphChap18.rs,1052
pub mod AVLTreeSeqStEphChap18 {AVLTreeSeqStEphChap183,74
    pub trait AVLTreeSeqStEphChap18Trait<T: StT> {AVLTreeSeqStEphChap18Trait9,228
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T>;tabulate12,426
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U>;map15,663
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T>;append18,871
        fn filter(a: &AVLTreeSeqStEphS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T>;filter21,1111
    impl<T: StT> AVLTreeSeqStEphChap18Trait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS24,1209
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T> {tabulate25,1282
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U> {map32,1558
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T> {append39,1867
        fn filter(a: &AVLTreeSeqStEphS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T> {filter51,2331

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEphChap18.rs,2957
pub mod LinkedListStEphChap18 {LinkedListStEphChap183,60
pub trait LinkedListStEphChap18Trait<T: StT> {LinkedListStEphChap18Trait8,200
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;tabulate9,247
    fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map10,313
    fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append11,402
    fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T>;filter12,490
    fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T>;update13,577
    fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEpinject14,670
    fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEninject15,773
    fn iterate< A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,877
    fn iteratePrefixes< A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkiteratePrefixes17,963
    fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce18,1080
    fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, Tscan19,1158
    fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten20,1257
    fn collect<A: StT, Bv: StT>(a: &LinkedListStEphS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O) ->collect21,1340
impl<T: StT> LinkedListStEphChap18Trait<T> for LinkedListStEphS<T> {LinkedListStEphS24,1489
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate25,1558
    fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map30,1760
    fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append35,2159
    fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter43,2713
    fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T> { <Lupdate51,3115
    fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEpinject52,3280
    fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEninject69,4117
    fn iterate< A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate82,4804
    fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkeiteratePrefixes85,5078
    fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce91,5522
    fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, Tscan102,6240
    fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten110,6886
    fn collect<A: StT, Bv: StT>(a: &LinkedListStEphS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O) ->collect120,7506

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/RelationStEphChap5_2.rs,1962
pub mod RelationStEphChap5_2 {RelationStEphChap5_23,59
pub struct Relation<A, B> {Relation14,315
    pairs: Set<(A, B)>,pairs15,343
pub trait RelationStEphChap5_2Trait<X: Eq + Hash + Display + Debug + Clone + Sized, RelationStEphChap5_2Trait18,370
    fn empty() -> Relation<X, Y>;empty20,539
    fn FromSet(pairs: Set<(X, Y)>) -> Relation<X, Y>;FromSet22,574
    fn size(&self) -> N;size24,629
    fn domain(&self) -> Set<X>domain26,655
    fn range(&self) -> Set<Y>range30,715
    fn mem(&self, a: &X, b: &Y) -> Bmem34,774
    fn iter(&self) -> Iter<'_, (X, Y)>;iter39,858
impl<A, B> Relation<A, B> {Relation42,901
    pub fn FromVec(v: Vec<(A, B)>) -> Relation<A, B>FromVec43,929
impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> PartialEq for Relation<A, BRelation52,1131
    fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq53,1231
impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> Eq for Relation<A, B> {}Relation56,1303
impl<A: Debug + Eq + Hash, B: Debug + Eq + Hash> Debug for Relation<A, B> {Relation58,1398
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt59,1474
impl<A: Display + Eq + Hash, B: Display + Eq + Hash> Display for Relation<A, B> {Relation64,1562
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt65,1644
impl<X: Eq + Hash + Display + Debug + Clone + Sized, Relation76,1948
    fn empty() -> Relation<X, Y> {empty78,2109
    fn FromSet(pairs: Set<(X, Y)>) -> Relation<X, Y> { Relation { pairs } }FromSet82,2189
    fn size(&self) -> N { self.pairs.size() }size84,2266
    fn domain(&self) -> Set<X>domain86,2313
    fn range(&self) -> Set<Y>range95,2516
    fn mem(&self, a: &X, b: &Y) -> Bmem104,2718
    fn iter(&self) -> Iter<'_, (X, Y)> { self.pairs.iter() }iter112,2906
macro_rules! RelationLit {RelationLit120,3047
pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise131,3608

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStPerChap19.rs,1418
pub mod AVLTreeSeqStPerChap19 {AVLTreeSeqStPerChap193,49
    pub trait AVLTreeSeqStPerChap19Trait<T: StT> {AVLTreeSeqStPerChap19Trait9,268
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T>;tabulate10,319
        fn map<U>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U>;map11,389
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T>;select12,477
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T>;append13,565
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T>;deflate14,657
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T>;filter15,728
    impl<T: StT> AVLTreeSeqStPerChap19Trait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS18,823
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T> {tabulate19,896
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U> {map22,1060
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T> {select25,1242
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T> {append39,1887
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T> {deflate42,2071
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T> {filter49,2375

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,1449
pub mod Types {Types4,45
    pub type N = usize;N9,156
    pub enum B {B13,299
        True,True14,316
        False,False15,330
    impl std::fmt::Display for B {B22,563
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt23,598
    pub trait StT: Eq + Clone + Display + Debug + Sized {}StT33,939
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}T34,998
    pub trait MtT: Clone + Eq + Sized + Send + Sync {}MtT38,1206
    impl<T> MtT for T where T: Clone + Eq + Sized + Send + Sync {}T39,1261
    pub struct Edge<V: StT>(pub V, pub V);Edge43,1468
    impl<V: StT> std::fmt::Display for Edge<V> {Edge45,1512
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt46,1561
    impl<V: StT> From<(V, V)> for Edge<V> {Edge51,1705
        fn from(t: (V, V)) -> Self {from52,1749
    impl<V: StT> From<Edge<V>> for (V, V) {V57,1830
        fn from(e: Edge<V>) -> (V, V) {from58,1874
    pub struct Pair<A, B>(pub A, pub B);Pair65,2091
    impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {Pair67,2133
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt68,2221
    impl<A, B> From<(A, B)> for Pair<A, B> {Pair73,2365
        fn from(t: (A, B)) -> Self {from74,2410
    impl<A, B> From<Pair<A, B>> for (A, B) {B79,2491
        fn from(p: Pair<A, B>) -> (A, B) {from80,2536

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStEph.rs,2375
pub mod ArraySeqStEph {ArraySeqStEph3,93
    pub struct ArraySeqStEphS<T: StT> {ArraySeqStEphS13,340
        pub data: Box<[T]>,data14,380
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait18,463
        fn new(length: N, init_value: T) -> Self;new20,547
        fn length(&self) -> N;length22,633
        fn nth(&self, index: N) -> &T;nth24,700
        fn empty() -> Self;empty26,775
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set28,839
        fn singleton(item: T) -> Self;singleton30,956
        fn isEmpty(&self) -> B;isEmpty32,1031
        fn isSingleton(&self) -> B;isSingleton34,1099
        fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy36,1182
    impl<T: StT> ArraySeqStEphS<T> {ArraySeqStEphS41,1282
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq42,1319
        pub fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy48,1546
        pub fn from_vec(v: Vec<T>) -> Self {from_vec67,2238
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update72,2380
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter79,2588
    impl<T: StT> PartialEq for ArraySeqStEphS<T> {ArraySeqStEphS82,2658
        fn eq(&self, other: &Self) -> bool {eq83,2709
    impl<T: StT> Eq for ArraySeqStEphS<T> {}ArraySeqStEphS96,3036
    impl<T: StT> Debug for ArraySeqStEphS<T> {ArraySeqStEphS98,3082
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt99,3129
    impl<T: StT> Display for ArraySeqStEphS<T> {ArraySeqStEphS105,3322
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt106,3371
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS116,3654
        fn new(length: N, init_value: T) -> Selfnew117,3717
        fn length(&self) -> N {length123,3883
        fn nth(&self, index: N) -> &T {nth126,3953
        fn empty() -> Self {empty129,4033
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set132,4121
        fn singleton(item: T) -> Self {singleton140,4398
        fn isEmpty(&self) -> B {isEmpty143,4497
        fn isSingleton(&self) -> B {isSingleton150,4662
        fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy157,4831
macro_rules! ArraySeqStEph {ArraySeqStEph168,5014

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStEphChap19.rs,2239
pub mod ArraySeqStEphChap19 {ArraySeqStEphChap193,51
pub trait ArraySeqStEphChap19Trait<T: StT> {ArraySeqStEphChap19Trait8,212
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate9,257
    fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map10,321
    fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T>;select11,406
    fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append12,496
    fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append213,578
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T>;deflate14,661
    fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter15,726
    fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,806
    fn reduce<F>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> Treduce17,889
    fn scan<F>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T)scan20,986
    fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten23,1102
impl<T: StT> ArraySeqStEphChap19Trait<T> for ArraySeqStEphS<T> {ArraySeqStEphS26,1181
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate27,1246
    fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map30,1392
    fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T> {select33,1554
    fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append45,1908
    fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append248,2070
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T> {deflate51,2233
    fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter58,2506
    fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate61,2666
    fn reduce<F>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> Treduce64,2833
    fn scan<F>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T)scan70,3019
    fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten76,3222

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPerChap18.rs,2618
pub mod ArraySeqStPerChap18 {ArraySeqStPerChap183,46
pub trait ArraySeqStPerChap18Trait<T: MtT> {ArraySeqStPerChap18Trait7,149
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate9,276
    fn map<U: MtT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map12,414
    fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append15,551
    fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter18,718
    fn update(a: &ArrayStPerS<T>, item_at: (N, T)) -> ArrayStPerS<T>;update23,946
    fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<(N, T)>) -> ArrayStPerS<T>;inject26,1090
    fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<(N, T)>) -> ArrayStPerS<T>;ninject29,1230
    fn iterate<A: MtT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate32,1389
    fn iteratePrefixes<A: MtT + Clone>(iteratePrefixes35,1511
    fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce42,1736
    fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan45,1853
    fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten48,2009
    fn collect(collect51,2153
impl<T: MtT> ArraySeqStPerChap18Trait<T> for ArrayStPerS<T> {ArrayStPerS57,2291
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate58,2353
    fn map<U: MtT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map62,2516
    fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append74,2964
    fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter88,3432
    fn update(a: &ArrayStPerS<T>, (index, item): (N, T)) -> ArrayStPerS<T> {update97,3731
    fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<(N, T)>) -> ArrayStPerS<T> {inject103,3928
    fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<(N, T)>) -> ArrayStPerS<T> {ninject115,4436
    fn iterate<A: MtT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate125,4830
    fn iteratePrefixes<A: MtT + Clone>(iteratePrefixes132,5034
    fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce145,5432
        fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec146,5506
    fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan161,5946
        fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec162,6036
    fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten181,6692
    fn collect(collect191,7020

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/MappingStEphChap5_5.rs,2373
pub mod MappingStEphChap5_5 {MappingStEphChap5_53,72
pub struct Mapping<A, B> {Mapping12,325
    rel: Relation<A, B>,rel13,352
pub trait MappingStEphChap5_5Trait<X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + MappingStEphChap5_5Trait16,380
    fn empty() -> Mapping<X, Y>;empty17,553
    fn FromVec(v: Vec<(X, Y)>) -> Mapping<X, Y>;FromVec19,587
    fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation21,637
    fn size(&self) -> N;size23,696
    fn domain(&self) -> Set<X>;domain25,722
    fn range(&self) -> Set<Y>;range27,755
    fn mem(&self, a: &X, b: &Y) -> B;mem29,787
    fn iter(&self) -> std::collections::hash_set::Iter<'_, (X, Y)>;iter31,826
impl<A, B> Mapping<A, B> {Mapping34,897
    fn unique_pairs_from_iter<I>(iter: I) -> Set<(A, B)>unique_pairs_from_iter35,924
impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std:Mapping48,1274
    fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq49,1413
impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std:Mapping51,1480
impl<A: std::fmt::Debug + Eq + Hash, B: std::fmt::Debug + Eq + Hash> std::fmt::Debug for MappingMapping53,1614
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }fmt54,1719
impl<A: std::fmt::Display + Eq + Hash, B: std::fmt::Display + Eq + Hash> std::fmt::Display for MMapping56,1812
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }fmt57,1923
impl<X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized, Y: Eq + Hash + std::fmtMapping60,2017
    fn empty() -> Mapping<X, Y> {empty61,2209
    fn FromVec(v: Vec<(X, Y)>) -> Mapping<X, Y> {FromVec65,2336
    fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation70,2539
    fn size(&self) -> N { self.rel.size() }size75,2767
    fn domain(&self) -> Set<X> { self.rel.domain() }domain77,2812
    fn range(&self) -> Set<Y> { self.rel.range() }range79,2866
    fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem81,2918
    fn iter(&self) -> std::collections::hash_set::Iter<'_, (X, Y)> { self.rel.iter() }iter83,2979
macro_rules! MappingLit {MappingLit91,3144
pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise102,3707

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main7,236

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStEph.rs,4065
pub mod AVLTreeSeqStEph {AVLTreeSeqStEph3,61
type Link<T> = Option<Box<AVLTreeNode<T>>>;Link8,181
pub struct AVLTreeNode<T: StT> {AVLTreeNode10,226
    pub value: T,value11,259
    pub height: N,height12,277
    pub left_size: N,left_size13,296
    pub right_size: N,right_size14,318
    pub left: Link<T>,left15,341
    pub right: Link<T>,right16,364
    pub index: N,index17,388
impl<T: StT> AVLTreeNode<T> {AVLTreeNode20,409
    fn new(value: T, index: N) -> Self {new21,439
pub struct AVLTreeSeqStEphS<T: StT> {AVLTreeSeqStEphS34,684
    pub root: Link<T>,root35,722
    pub next_key: N,next_key36,745
pub trait AVLTreeSeqStEphTrait<T: StT> {AVLTreeSeqStEphTrait39,769
    fn empty() -> Self;empty41,848
    fn new() -> Self;new43,910
    fn length(&self) -> N;length45,970
    fn nth(&self, index: N) -> &T;nth47,1043
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set49,1124
    fn singleton(item: T) -> Self;singleton51,1239
    fn isEmpty(&self) -> B;isEmpty53,1312
    fn isSingleton(&self) -> B;isSingleton55,1378
    fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy57,1468
impl<T: StT> AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS60,1527
    pub fn new_root() -> Self {new_root61,1562
    pub fn new() -> Self {new67,1686
    pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqStEphS<T> {update70,1744
    pub fn from_vec(values: Vec<T>) -> AVLTreeSeqStEphS<T>from_vec74,1935
    pub fn to_arrayseq(&self) -> ArraySeqStEphS<T>to_arrayseq84,2286
    pub fn iter<'a>(&'a self) -> AVLTreeSeqIterStEph<'a, T> {iter104,2913
    pub fn push_back(&mut self, value: T) {push_back107,3026
    pub fn contains_value(&self, target: &T) -> Bcontains_value112,3220
    pub fn insert_value(&mut self, value: T) {insert_value121,3415
    pub fn delete_value(&mut self, target: &T) -> booldelete_value124,3499
impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS150,4225
    fn empty() -> Self {empty151,4288
    fn new() -> Self {new155,4357
    fn length(&self) -> N {length159,4424
    fn nth(&self, index: N) -> &T {nth163,4489
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set167,4568
    fn singleton(item: T) -> Self {singleton172,4718
    fn isEmpty(&self) -> B {isEmpty178,4895
    fn isSingleton(&self) -> B {isSingleton186,5031
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy194,5171
pub struct AVLTreeSeqIterStEph<'a, T: StT> {AVLTreeSeqIterStEph210,5640
    stack: Vec<&'a AVLTreeNode<T>>,stack211,5685
    current: Option<&'a AVLTreeNode<T>>,current212,5721
impl<'a, T: StT> AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph215,5765
    fn new(root: &'a Link<T>) -> Self {new216,5811
    fn push_left(&mut self, link: &'a Link<T>) {push_left224,6008
impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph233,6228
    type Item = &'a T;Item234,6287
    fn next(&mut self) -> Option<Self::Item> {next235,6310
fn h<T: StT>(n: &Link<T>) -> N {h243,6506
fn size_link<T: StT>(n: &Link<T>) -> N {size_link247,6581
fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>) {update_meta255,6718
fn rotate_right<T: StT>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right263,6935
fn rotate_left<T: StT>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left274,7258
fn rebalance<T: StT>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance285,7579
pub(crate) fn insert_at_link<T: StT>(insert_at_link306,8264
fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> &'a T {nth_link331,8999
fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str> {set_link343,9335
    macro_rules! AVLTreeSeqStEph {AVLTreeSeqStEph361,9877
    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS375,10438
        fn eq(&self, other: &Self) -> bool {eq376,10491
    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}AVLTreeSeqStEphS389,10818

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStPerChap18.rs,2606
pub mod LinkedListStPerChap18 {LinkedListStPerChap183,48
    pub trait LinkedListStPerChap18Trait<T: StT> {LinkedListStPerChap18Trait7,165
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate9,295
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map12,446
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append15,593
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter18,783
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;update23,1037
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>,)inject26,1203
        fn ninject(ninject30,1380
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate36,1601
        fn iteratePrefixes<A: StT>(iteratePrefixes39,1736
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce46,1987
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan49,2117
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten52,2291
        fn collect<A: StT, Bv: StT>(collect55,2458
    impl<T: StT> LinkedListStPerChap18Trait<T> for LinkedListStPerS<T> {LinkedListStPerS61,2650
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate62,2723
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map70,2974
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append78,3437
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter90,4079
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T>update100,4549
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>,) inject109,5093
        fn ninject(ninject134,6194
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate160,7288
        fn iteratePrefixes<A: StT>(iteratePrefixes170,7684
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce187,8365
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan203,9199
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten224,10195
        fn collect<A: StT, Bv: StT>(collect239,10937

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,1218
pub mod Types;Types7,226
pub mod MathSeq;MathSeq10,274
pub mod SetStEphChap5_1;SetStEphChap5_113,351
pub mod RelationStEphChap5_2;RelationStEphChap5_216,429
pub mod MappingStEphChap5_5;MappingStEphChap5_519,522
pub mod DirGraphStEphChap6_1;DirGraphStEphChap6_122,612
pub mod UnDirGraphStEphChap6_1;UnDirGraphStEphChap6_124,704
pub mod LinkedListStPer;LinkedListStPer27,803
pub mod LinkedListStPerChap18;LinkedListStPerChap1829,880
pub mod LinkedListStPerChap19;LinkedListStPerChap1931,952
pub mod LinkedListStEph;LinkedListStEph34,1025
pub mod LinkedListStEphChap18;LinkedListStEphChap1836,1102
pub mod LinkedListStEphChap19;LinkedListStEphChap1938,1174
pub mod ArraySeqStPer;ArraySeqStPer41,1247
pub mod ArraySeqStPerChap18;ArraySeqStPerChap1844,1319
pub mod ArraySeqStPerChap19;ArraySeqStPerChap1947,1388
pub mod ArraySeqStEph;ArraySeqStEph50,1457
pub mod ArraySeqStEphChap18;ArraySeqStEphChap1853,1529
pub mod ArraySeqStEphChap19;ArraySeqStEphChap1955,1618
pub mod AVLTreeSeqStPer;AVLTreeSeqStPer58,1708
pub mod AVLTreeSeqStPerChap18;AVLTreeSeqStPerChap1860,1785
pub mod AVLTreeSeqStPerChap19;AVLTreeSeqStPerChap1962,1880
pub mod AVLTreeSeqStEph;AVLTreeSeqStEph65,1976

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStPerChap19.rs,2579
pub mod LinkedListStPerChap19 {LinkedListStPerChap193,48
pub trait LinkedListStPerChap19Trait<T: StT> {LinkedListStPerChap19Trait8,219
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate9,266
    fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map10,332
    fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T>;select11,421
    fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append12,515
    fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append213,603
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T>;deflate14,692
    fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter15,759
    fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,843
    fn reduce<F>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;reduce17,928
    fn scan<F>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<T>, T) where F: Fn(&Tscan18,1015
    fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten19,1123
    fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>,inject20,1205
impl<T: StT> LinkedListStPerChap19Trait<T> for LinkedListStPerS<T> {LinkedListStPerS24,1322
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate25,1391
    fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map28,1543
    fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T> {select31,1713
    fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append45,2305
    fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append249,2478
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T> {deflate53,2652
    fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter61,2936
    fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate65,3105
    fn reduce<F>(a: &LinkedListStPerS<T>, f: &F, id: T) -> Treduce69,3279
    fn scan<F>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<T>, T)scan76,3472
    fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten83,3684
    fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>,) inject87,3849

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/SetStEphChap5_1.rs,3255
pub mod SetStEphChap5_1 {SetStEphChap5_13,69
pub struct Set<T> { data: HashSet<T> }Set11,236
pub struct Set<T> { data: HashSet<T> }data11,236
pub trait SetStEphChap5_1Trait<T: Eq + Hash + Clone + Display + Debug + Sized> {SetStEphChap5_1Trait13,276
    fn empty() -> Set<T>;empty14,357
    fn singleton(x: T) -> Set<T>;singleton15,383
    fn size(&self) -> N;size16,417
    fn mem(&self, x: &T) -> B;mem17,442
    fn union(&self, other: &Set<T>) -> Set<T>union18,473
    fn intersection(&self, other: &Set<T>) -> Set<T>intersection21,546
    fn partition(&self, parts: &Set<Set<T>>) -> B;partition24,628
    fn CartesianProduct<U>(&self, other: &Set<U>) -> Set<(T, U)>CartesianProduct26,680
    fn insert(&mut self, x: T) -> &mut Self;insert31,795
    fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;iter33,841
    fn FromVec(v: Vec<T>) -> Set<T>FromVec34,904
impl<T: Eq + Hash> PartialEq for Set<T> {Set39,975
    fn eq(&self, other: &Self) -> bool { self.data == other.data }eq40,1017
impl<T: Eq + Hash> Eq for Set<T> {}Set43,1087
impl<T: Eq + Hash> std::fmt::Debug for Set<T>Set45,1124
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt49,1202
impl<T: Eq + Hash> std::fmt::Display for Set<T>Set54,1341
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt58,1423
impl<T: Eq + Hash > Hash for Set<T> {Set70,1813
    fn hash<H: Hasher>(&self, state: &mut H) {hash71,1851
impl<T: Eq + Hash> Set<T> {Set85,2335
    pub fn empty() -> Set<T> {empty86,2363
    pub fn singleton(x: T) -> Set<T> {singleton90,2438
    pub fn size(&self) -> N { self.data.len() }size96,2584
    pub fn mem(&self, x: &T) -> B {mem98,2633
    pub fn union(&self, other: &Set<T>) -> Set<T>union102,2739
    pub fn intersection(&self, other: &Set<T>) -> Set<T>intersection111,2951
    pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition120,3246
    pub fn CartesianProduct<U>(&self, other: &Set<U>) -> Set<(T, U)>CartesianProduct134,3661
    pub fn insert(&mut self, x: T) -> &mut Self {insert148,4034
    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter153,4141
    pub fn FromVec(v: Vec<T>) -> Set<T>FromVec155,4229
impl<T: Eq + Hash + Clone + Display + Debug + Sized> SetStEphChap5_1Trait<T> for Set<T> {Set165,4437
    fn empty() -> Set<T> {empty166,4527
    fn singleton(x: T) -> Set<T> {singleton170,4598
    fn size(&self) -> N { self.data.len() }size176,4740
    fn mem(&self, x: &T) -> B {mem178,4785
    fn union(&self, other: &Set<T>) -> Set<T>union182,4887
    fn intersection(&self, other: &Set<T>) -> Set<T>intersection191,5095
    fn partition(&self, parts: &Set<Set<T>>) -> B {partition200,5386
    fn CartesianProduct<U>(&self, other: &Set<U>) -> Set<(T, U)>CartesianProduct214,5797
    fn insert(&mut self, x: T) -> &mut Self {insert228,6166
    fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter233,6269
    fn FromVec(v: Vec<T>) -> Set<T>FromVec235,6353
macro_rules! SetLit {SetLit251,6625
pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise264,6936
    let _s0: Set<&'static str> = SetLit![];str266,7030

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEphChap19.rs,2569
pub mod LinkedListStEphChap19 {LinkedListStEphChap193,60
    pub trait LinkedListStEphChap19Trait<T: StT> {LinkedListStEphChap19Trait8,243
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;tabulate9,294
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map10,364
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>select11,457
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append12,555
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append213,647
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T>;deflate14,740
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T>;filter15,811
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,899
        fn reduce<F>(a: &LinkedListStEphS<T>, f: &F, id: T) -> Treduce17,988
        fn scan<F>(a: &LinkedListStEphS<T>, f: &F, id: T) -> (LinkedListStEphS<T>, T)scan20,1099
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten23,1231
        fn inject(inject24,1317
    impl<T: StT> LinkedListStEphChap19Trait<T> for LinkedListStEphS<T> {LinkedListStEphS30,1471
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate31,1544
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map34,1708
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>select37,1890
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append51,2538
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append254,2722
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T> {deflate57,2907
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter64,3218
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate67,3398
        fn reduce<F>(a: &LinkedListStEphS<T>, f: &F, id: T) -> Treduce70,3583
        fn scan<F>(a: &LinkedListStEphS<T>, f: &F, id: T) -> (LinkedListStEphS<T>, T)scan76,3799
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten82,4034
        fn inject(values: &LinkedListStEphS<T>,changes: &LinkedListStEphS<Pair<N, T>>,) inject85,4210

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEph.rs,2799
pub mod LinkedListStEph {LinkedListStEph3,90
pub struct NodeE<T: StT> {NodeE7,162
    pub value: T,value8,189
    pub next: Option<Box<NodeE<T>>>,next9,207
pub struct LinkedListStEphS<T: StT> {LinkedListStEphS13,264
    head: Option<Box<NodeE<T>>>,head14,302
    len: N,len15,335
pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait18,350
    fn empty() -> LinkedListStEphS<T>;empty19,391
    fn new(length: N, init_value: T) -> Self;new20,430
    fn length(&self) -> N;length21,476
    fn nth(&self, index: N) -> &T;nth22,503
    fn isEmpty(&self) -> B;isEmpty23,538
    fn isSingleton(&self) -> B;isSingleton24,566
    fn singleton(item: T) -> Self;singleton25,598
    fn update(&mut self, item_at: Pair<N, T>) -> &mut Self;update26,633
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set27,693
    fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy28,770
impl<T: StT> LinkedListStEphS<T> {LinkedListStEphS31,829
    fn push_front_node(&mut self, node: Box<NodeE<T>>) {push_front_node32,864
    pub fn from_vec(v: Vec<T>) -> Self {from_vec39,1041
    pub fn iter<'a>(&'a self) -> LinkedListStEphIter<'a, T> {iter47,1278
impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS52,1410
    fn empty() -> Self {empty53,1473
    fn new(length: N, init_value: T) -> Self {new56,1552
    fn length(&self) -> N {length75,2157
    fn nth(&self, index: N) -> &T {nth78,2208
    fn isEmpty(&self) -> B {isEmpty90,2527
    fn isSingleton(&self) -> B {isSingleton97,2657
    fn singleton(item: T) -> Self {singleton104,2791
    fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {update113,3004
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set126,3355
    fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy139,3745
impl<T: StT> std::fmt::Debug for LinkedListStEphS<T> {LinkedListStEphS175,4787
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt176,4842
pub struct LinkedListStEphIter<'a, T: StT> {LinkedListStEphIter187,5168
    cursor: Option<&'a NodeE<T>>,cursor188,5213
impl<'a, T: StT> Iterator for LinkedListStEphIter<'a, T> {LinkedListStEphIter191,5250
    type Item = &'a T;Item192,5309
    fn next(&mut self) -> Option<Self::Item> {next193,5332
impl<T: StT> PartialEq for LinkedListStEphS<T> {LinkedListStEphS201,5523
    fn eq(&self, other: &Self) -> bool {eq202,5572
impl<T: StT> Eq for LinkedListStEphS<T> {}LinkedListStEphS219,5997
impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {LinkedListStEphS221,6041
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt222,6098
macro_rules! LinkedListStEph {LinkedListStEph236,6502

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPer.rs,3336
pub mod ArraySeqStPer {ArraySeqStPer8,354
    pub struct ArrayStPerS<T: MtT> { data: Box<[T]> }ArrayStPerS17,618
    pub struct ArrayStPerS<T: MtT> { data: Box<[T]> }data17,618
    pub trait ArraySeqStPerTrait<T: MtT> {ArraySeqStPerTrait20,744
        fn new(length: N, init_value: T) -> Self where T: Clone;new22,833
        fn length(&self) -> N;length24,939
        fn nth(&self, index: N) -> &T;nth26,1011
        fn empty() -> Self;empty28,1091
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone, Self: Sizset32,1342
        fn singleton(item: T) -> Self;singleton34,1483
        fn isEmpty(&self) -> B;isEmpty36,1563
        fn isSingleton(&self) -> B;isSingleton38,1636
        fn subseq_copy(&self, start: N, length: N) -> Self where T: Clone, Self: Sized;subseq_copy40,1718
    impl<T: MtT> ArrayStPerS<T> {ArrayStPerS43,1813
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq45,1888
        pub fn from_vec(v: Vec<T>) -> Self { ArrayStPerS { data: v.into_boxed_slice() } }from_vec54,2251
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter56,2342
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }iter_mut57,2405
        pub fn empty() -> Self { ArrayStPerS { data: Vec::new().into_boxed_slice() } }empty59,2484
        pub fn singleton(item: T) -> Self { ArrayStPerS { data: vec![item].into_boxed_slice() } singleton60,2571
        pub fn new(length: N, init_value: T) -> Self where T: Clone { Self::from_vec(vec![init_vnew61,2669
        pub fn length(&self) -> N { self.data.len() }length62,2782
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth63,2836
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {set64,2900
    impl<T: MtT + Eq> PartialEq for ArrayStPerS<T> {ArrayStPerS72,3203
        fn eq(&self, other: &Self) -> bool {eq73,3256
    impl<T: MtT + Eq> Eq for ArrayStPerS<T> {}ArrayStPerS79,3490
    impl<T: MtT + Debug> Debug for ArrayStPerS<T> {ArrayStPerS81,3538
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt82,3590
    impl<T: MtT + Display> Display for ArrayStPerS<T> {ArrayStPerS88,3783
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt89,3839
    impl<T: MtT> ArraySeqStPerTrait<T> for ArrayStPerS<T> {ArrayStPerS99,4124
        fn new(length: N, init_value: T) -> Self where T: Clone {new100,4184
        fn length(&self) -> N { self.data.len() }length103,4313
        fn nth(&self, index: N) -> &T { &self.data[index] }nth104,4363
        fn empty() -> Self { Self::from_vec(Vec::new()) }empty105,4423
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {set106,4481
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton112,4773
        fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }isEmpty113,4842
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton114,4931
        fn subseq_copy(&self, start: N, length: N) -> Self where T: Clone {subseq_copy115,5024
    macro_rules! ArraySeqStPer {ArraySeqStPer127,5399
    fn _ArraySeqStPer_macro_type_checks() {_ArraySeqStPer_macro_type_checks134,5751

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStPerChap18.rs,1050
pub mod AVLTreeSeqStPerChap18 {AVLTreeSeqStPerChap183,46
    pub trait AVLTreeSeqStPerChap18Trait<T: StT> {AVLTreeSeqStPerChap18Trait8,199
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T>;tabulate10,338
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U>;map12,499
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T>;append14,666
        fn filter(a: &AVLTreeSeqStPerS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T>;filter16,855
    impl<T: StT> AVLTreeSeqStPerChap18Trait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS19,953
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T> {tabulate20,1026
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U> {map27,1308
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T> {append32,1654
        fn filter(a: &AVLTreeSeqStPerS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T> {filter42,2154

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/UnDirGraphStEphChap6_1.rs,2805
pub mod UnDirGraphStEphChap6_1 {UnDirGraphStEphChap6_13,80
pub struct UnDirGraphStEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {UnDirGraphStEph10,247
    V: Set<V>,V11,336
    E: Set<(V, V)>,E12,351
pub trait UnDirGraphStEphChap6_1Trait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::DebugUnDirGraphStEphChap6_1Trait15,374
    fn empty() -> UnDirGraphStEph<V>;empty16,474
    fn FromSets(V: Set<V>, E: Set<(V, V)>) -> UnDirGraphStEph<V>;FromSets17,512
    fn vertices(&self) -> &Set<V>;vertices18,578
    fn edges(&self) -> &Set<(V, V)>;edges19,613
    fn sizeV(&self) -> N;sizeV20,650
    fn sizeE(&self) -> N;sizeE21,676
    fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor23,754
    fn NG(&self, v: &V) -> Set<V>;NG24,797
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices25,832
    fn Incident(&self, e: &(V, V), v: &V) -> B;Incident26,886
    fn Degree(&self, v: &V) -> N;Degree27,934
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> UnDirGraphStEphChap6_1Trait<V> UnDirGraphStEph30,971
    fn empty() -> UnDirGraphStEph<V> { UnDirGraphStEph { V: SetLit![], E: SetLit![] } }empty31,1092
    fn FromSets(V: Set<V>, E: Set<(V, V)>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E } }FromSets32,1180
    fn vertices(&self) -> &Set<V> { &self.V }vertices33,1274
    fn edges(&self) -> &Set<(V, V)> { &self.E }edges34,1320
    fn sizeV(&self) -> N { self.V.size() }sizeV35,1368
    fn sizeE(&self) -> N { self.E.size() }sizeE36,1411
    fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor38,1455
    fn NG(&self, v: &V) -> Set<V> {NG43,1687
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices52,1957
    fn Incident(&self, e: &(V, V), v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } else { B:Incident61,2197
    fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree63,2305
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Debug for UnDirGraphSUnDirGraphStEph66,2363
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt67,2469
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for UnDirGrapUnDirGraphStEph72,2642
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt73,2750
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for UnDirGraphStEph<VUnDirGraphStEph78,2881
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for UnDirGraphStEph<Veq78,2881
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for UnDirGraphStEph<V> {}UnDirGraphStEph79,3061
macro_rules! UnDirGraphLit {UnDirGraphLit86,3237
pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise95,3781

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Problem21_1.rs,418
fn points2d(n: N) -> ArrayPerS<(N, N)> {points2d9,287
fn test_points2d_n3_example() {test_points2d_n3_example22,576
fn test_points2d_n1_empty() {test_points2d_n1_empty30,795
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values36,892
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order44,1066
fn test_points2d_debug_display_shapes() {test_points2d_debug_display_shapes52,1334

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/11_TestArraySeqStPerChap19.rs,342
pub mod TestArraySeqPerChap19 {TestArraySeqPerChap191,0
fn test_map_and_select_and_append() {test_map_and_select_and_append10,289
fn test_deflate_and_filter() {test_deflate_and_filter20,752
fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten29,1212
fn test_inject_and_parallel() {test_inject_and_parallel48,2116

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/09_TestArraySeqStPer.rs,1751
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/10_TestArraySeqStPerChap18.rs,943
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/20_TestAVLTreeSeqEphChap19.rs,244
pub mod TestAVLTreeSeqEphChap19 {TestAVLTreeSeqEphChap193,80
fn test_tabulate_and_map() {test_tabulate_and_map11,328
fn test_select_and_append() {test_select_and_append19,622
fn test_deflate_and_filter() {test_deflate_and_filter38,1418

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStEphChap19.rs,68
fn bench_avl_eph_ch19(c: &mut Criterion) {bench_avl_eph_ch196,187

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchUnDirGraphStEphChap6_1.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build7,247

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchMathSeq.rs,72
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics8,213

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStEphChap19.rs,86
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch198,307

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPer.rs,80
fn bench_sll_persistent_ops(c: &mut Criterion) {bench_sll_persistent_ops8,225

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStEph.rs,455
struct LinearCongruentialGenerator32 { state: u32 }LinearCongruentialGenerator3211,407
struct LinearCongruentialGenerator32 { state: u32 }state11,407
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3213,460
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new14,497
    fn next_N(&mut self) -> N {next_N16,616
fn bench_build_random_s(c: &mut Criterion) {bench_build_random_s25,853

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEphChap19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch197,247

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch187,239

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStPer.rs,477
struct LinearCongruentialGenerator32 { state: u32 }LinearCongruentialGenerator3211,415
struct LinearCongruentialGenerator32 { state: u32 }state11,415
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3213,468
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new14,505
    fn next_N(&mut self) -> N {next_N16,624
fn bench_build_random_s_persistent(c: &mut Criterion) {bench_build_random_s_persistent25,861

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEphChap18.rs,66
fn bench_ll_eph_ch18(c: &mut Criterion) {bench_ll_eph_ch187,247

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStEph.rs,58
fn bench_avl_eph(c: &mut Criterion) {bench_avl_eph6,187

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchDirGraphStEphChap6_1.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build7,243

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchRelationStEphChap5_2.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range8,264

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStEphChap18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map8,307

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchMappingStEphChap5_5.rs,70
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build8,297

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStPerChap19.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch198,281

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStPerChap19.rs,95
fn bench_build_and_read_persistent(c: &mut Criterion) {bench_build_and_read_persistent10,361

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStPer.rs,80
fn bench_build_and_contains(c: &mut Criterion) {bench_build_and_contains9,285

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPerChap19.rs,66
fn bench_ll_per_ch19(c: &mut Criterion) {bench_ll_per_ch198,323

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStPerChap18.rs,68
fn bench_avl_per_ch18(c: &mut Criterion) {bench_avl_per_ch187,247

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStEphChap18.rs,68
fn bench_avl_eph_ch18(c: &mut Criterion) {bench_avl_eph_ch186,187

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPerChap18.rs,66
fn bench_ll_per_ch18(c: &mut Criterion) {bench_ll_per_ch187,247

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEph.rs,56
fn bench_ll_eph(c: &mut Criterion) {bench_ll_eph6,187
