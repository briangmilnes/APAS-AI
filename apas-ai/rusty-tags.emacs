
/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtPerChap19.rs,3519
pub mod ArraySeqMtPerChap19 {ArraySeqMtPerChap193,129
    pub trait ArraySeqMtPerChap19Trait<T: MtT> {ArraySeqMtPerChap19Trait11,363
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;tabulate15,625
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;map18,862
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;append21,1057
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;filter24,1340
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;update27,1521
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;ninject30,1723
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate32,1893
        fn iteratePrefixes<A: MtT>(iteratePrefixes34,2022
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce40,2262
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan42,2386
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten44,2549
        fn collect(collect46,2700
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;inject52,2901
        fn atomicWrite(atomicWrite54,3039
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arrayinject_parallel260,3282
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins61,3389
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arraninject_parallel267,3657
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins68,3765
    impl<T: MtT + StT + Send + Sync> ArraySeqMtPerChap19Trait<T> for ArrayMtPerS<T> {ArrayMtPerS75,3978
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate76,4064
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map80,4217
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append84,4383
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter88,4546
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T> {update92,4716
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject96,4886
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate100,5072
        fn iteratePrefixes<A: MtT>(iteratePrefixes104,5246
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce112,5501
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan116,5668
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten120,5849
        fn collect(collect131,6251
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> inject168,7679
        fn atomicWrite(atomicWrite172,7873
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arrayinject_parallel2180,8166
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins185,8446
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arraninject_parallel2204,9180
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins209,9463

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPerChap19.rs,2726
pub mod ArraySeqStPerChap19 {ArraySeqStPerChap193,46
    pub trait ArraySeqStPerChap19Trait<T: StT> {ArraySeqStPerChap19Trait8,217
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,352
        fn map<U: StT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map12,497
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T>;select14,621
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append16,766
        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append218,896
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T>;deflate20,1015
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1168
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate26,1426
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce28,1557
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan30,1681
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten32,1838
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject34,1968
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject36,2119
    impl<T: StT> ArraySeqStPerChap19Trait<T> for ArrayStPerS<T> {ArrayStPerS39,2219
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate40,2285
        fn map<U: StT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map44,2438
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T> {select47,2631
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append60,3012
        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append267,3287
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T> {deflate74,3563
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter81,3838
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate90,4275
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce104,4795
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan125,5542
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten162,7158
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject166,7313
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject170,7497

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStEphChap18.rs,2403
pub mod ArraySeqStEphChap18 {ArraySeqStEphChap183,51
    pub trait ArraySeqStEphChap18Trait<T: StT> {ArraySeqStEphChap18Trait7,162
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate10,303
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map13,467
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append16,664
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter19,846
        fn update(a: &mut ArraySeqStEphS<T>, item_at: (N, T)) -> &mut ArraySeqStEphS<T>;update22,1025
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject25,1234
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject28,1455
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate29,1557
        fn iteratePrefixes<A: StT>(iteratePrefixes30,1644
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce35,1807
        fn scan(scan36,1887
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten41,2033
        fn collect<A: StT, Bv: StT>(collect42,2114
    impl<T: StT> ArraySeqStEphChap18Trait<T> for ArraySeqStEphS<T> {ArraySeqStEphS48,2300
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate49,2369
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map56,2615
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append69,3149
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter90,3903
        fn update(a: &mut ArraySeqStEphS<T>, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update102,4328
        fn inject(inject105,4470
        fn ninject(ninject116,4831
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate122,5016
        fn iteratePrefixes<A: StT>(iteratePrefixes129,5251
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce146,5857
        fn scan(scan153,6086
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten171,6686
        fn collect<A: StT, Bv: StT>(collect199,7772

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStEphChap19.rs,1424
pub mod AVLTreeSeqStEphChap19 {AVLTreeSeqStEphChap193,54
    pub trait AVLTreeSeqStEphChap19Trait<T: StT> {AVLTreeSeqStEphChap19Trait11,273
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T>;tabulate12,324
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U>;map13,394
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T>;select14,487
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T>;append15,575
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T>;deflate16,667
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T>;filter17,738
    impl<T: StT> AVLTreeSeqStEphChap19Trait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS20,833
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T> {tabulate21,906
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U> {map24,1070
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T> {select27,1252
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T> {append41,1911
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T> {deflate44,2095
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T> {filter51,2406

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStPer.rs,4033
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
    fn rotate_left<T: StT>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left46,1244
    fn rebalance<T: StT>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance57,1602
    fn nth_ref<'a, T: StT>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref79,2455
    fn set_rec<T: StT>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> {set_rec94,2904
    fn inorder_collect<T: StT>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect112,3672
    fn build_balanced_from_slice<T: StT>(a: &[T]) -> Link<T> {build_balanced_from_slice120,3912
        fn rec<T: StT>(a: &[T]) -> Link<T> {rec121,3975
    pub struct AVLTreeSeqStPerS<T: StT> {AVLTreeSeqStPerS133,4293
        root: Link<T>,root134,4335
    pub trait AVLTreeSeqStPerTrait<T: StT> {AVLTreeSeqStPerTrait137,4365
        fn empty() -> Self;empty140,4502
        fn new() -> Self;new143,4622
        fn length(&self) -> N;length146,4740
        fn nth(&self, index: N) -> &T;nth149,4879
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set152,5085
        fn singleton(item: T) -> Self;singleton157,5287
        fn isEmpty(&self) -> B;isEmpty160,5418
        fn isSingleton(&self) -> B;isSingleton163,5542
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy166,5694
        fn from_vec(values: Vec<T>) -> Self;from_vec168,5817
        fn values_in_order(&self) -> Vec<T>;values_in_order170,5906
    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS173,5958
        fn empty() -> Self {empty174,6025
        fn new() -> Self {new177,6108
        fn length(&self) -> N {length180,6171
        fn nth(&self, index: N) -> &T {nth183,6242
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set186,6331
        fn singleton(item: T) -> Self {singleton191,6520
        fn isEmpty(&self) -> B {isEmpty196,6665
        fn isSingleton(&self) -> B {isSingleton203,6828
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy210,6995
        fn from_vec(values: Vec<T>) -> Self {from_vec224,7503
        fn values_in_order(&self) -> Vec<T> {values_in_order229,7666
    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS236,7857
        fn eq(&self, other: &Self) -> bool {eq237,7910
    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}AVLTreeSeqStPerS249,8236
    impl<T: StT> std::fmt::Debug for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS251,8284
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt252,8343
    impl<T: StT> AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS258,8535
        pub fn to_arrayseq(&self) -> ArrayStPerS<T> {to_arrayseq259,8574
        pub fn iter<'a>(&'a self) -> AVLTreeSeqStPerIter<'a, T> {iter264,8720
    pub struct AVLTreeSeqStPerIter<'a, T: StT> {AVLTreeSeqStPerIter272,8933
        stack: Vec<&'a Node<T>>,stack273,8982
        current: Option<&'a Node<T>>,current274,9015
    impl<'a, T: StT> AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter277,9060
        fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {push_left278,9110
    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter286,9320
        type Item = &'a T;Item287,9383
        fn next(&mut self) -> Option<Self::Item> {next288,9410
    macro_rules! AVLTreeSeqStPer {AVLTreeSeqStPer301,9802
    fn _AVLTreeSeqStPer_type_checks() {_AVLTreeSeqStPer_type_checks314,10532

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/MathSeq.rs,3929
pub mod MathSeq {MathSeq8,306
    pub struct MathSeqS<T: StT> {MathSeqS17,588
        data: Vec<T>,data18,622
    impl<T: StT> PartialEq for MathSeqS<T> {MathSeqS21,651
        fn eq(&self, other: &Self) -> bool {eq22,696
    impl<T: StT> Eq for MathSeqS<T> {}MathSeqS27,794
    impl<T: StT> std::fmt::Debug for MathSeqS<T> {MathSeqS29,834
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt30,885
    impl<T: StT> std::fmt::Display for MathSeqS<T> {MathSeqS35,1041
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt36,1094
    impl<T: StT> MathSeqS<T> {MathSeqS51,1507
        pub fn iter(&self) -> std::slice::Iter<'_, T> {iter54,1630
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {iter_mut59,1817
        pub fn empty() -> Self {empty65,2020
        pub fn singleton(item: T) -> Self {singleton70,2193
        pub fn from_vec(data: Vec<T>) -> Self {from_vec75,2387
        pub fn with_len(length: N, init_value: T) -> Self {with_len80,2573
    impl<'a, T: StT> IntoIterator for &'a MathSeqS<T> {MathSeqS87,2731
        type Item = &'a T;Item88,2787
        type IntoIter = std::slice::Iter<'a, T>;IntoIter89,2814
        fn into_iter(self) -> Self::IntoIter {into_iter90,2863
    impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {MathSeqS95,2956
        type Item = &'a mut T;Item96,3016
        type IntoIter = std::slice::IterMut<'a, T>;IntoIter97,3047
        fn into_iter(self) -> Self::IntoIter {into_iter98,3099
    impl<T: StT> IntoIterator for MathSeqS<T> {MathSeqS103,3196
        type Item = T;Item104,3244
        type IntoIter = std::vec::IntoIter<T>;IntoIter105,3267
        fn into_iter(self) -> Self::IntoIter {into_iter106,3314
    pub trait MathSeqTrait<T: StT + Hash> {MathSeqTrait112,3448
        fn new(length: N, init_value: T) -> Self;new115,3594
        fn empty() -> Self;empty119,3737
        fn singleton(item: T) -> Self;singleton123,3858
        fn length(&self) -> N;length127,3990
        fn nth(&self, index: N) -> &T;nth131,4114
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str>;set135,4246
        fn add_last(&mut self, value: T) -> &mut Self;add_last139,4533
        fn delete_last(&mut self) -> Option<T>;delete_last143,4681
        fn subseq(&self, start: N, length: N) -> &[T];subseq147,4822
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy151,4980
        fn isEmpty(&self) -> B;isEmpty155,5133
        fn isSingleton(&self) -> B;isSingleton159,5258
        fn domain(&self) -> Vec<N>;domain163,5391
        fn range(&self) -> Vec<T>;range167,5524
        fn multiset_range(&self) -> Vec<(N, T)>;multiset_range171,5656
    impl<T: StT + Hash> MathSeqTrait<T> for MathSeqS<T> {MathSeqS174,5712
        fn new(length: N, init_value: T) -> Self {new177,5872
        fn empty() -> Self {empty185,6111
        fn singleton(item: T) -> Self {singleton191,6285
        fn length(&self) -> N {length197,6470
        fn nth(&self, index: N) -> &T {nth203,6633
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str> {set209,6806
        fn add_last(&mut self, value: T) -> &mut Self {add_last220,7290
        fn delete_last(&mut self) -> Option<T> {delete_last227,7501
        fn subseq(&self, start: N, length: N) -> &[T] {subseq233,7681
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy242,8007
        fn isEmpty(&self) -> B {isEmpty256,8476
        fn isSingleton(&self) -> B {isSingleton266,8734
        fn domain(&self) -> Vec<N> {domain276,9000
        fn range(&self) -> Vec<T> {range282,9187
        fn multiset_range(&self) -> Vec<(N, T)> {multiset_range295,9654
    macro_rules! MathSeq {MathSeq314,10334
    fn _MathSeq_macro_type_checks() {_MathSeq_macro_type_checks321,10624

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/DirGraphStEphChap6_1.rs,3712
pub mod DirGraphStEphChap6_1 {DirGraphStEphChap6_13,77
pub struct DirGraphStEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {DirGraphStEph10,242
    V: Set<V>,V11,329
    A: Set<Pair<V, V>>,A12,344
pub trait DirGraphStEphChap6_1Trait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> DirGraphStEphChap6_1Trait15,371
    fn empty() -> DirGraphStEph<V>;empty18,553
    fn FromSets(V: Set<V>, A: Set<Pair<V, V>>) -> DirGraphStEph<V>;FromSets21,689
    fn vertices(&self) -> &Set<V>;vertices24,841
    fn arcs(&self) -> &Set<Pair<V, V>>;arcs27,960
    fn sizeV(&self) -> N;sizeV30,1084
    fn sizeA(&self) -> N;sizeA33,1194
    fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor36,1304
    fn NG(&self, v: &V) -> Set<V>;NG39,1435
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices42,1580
    fn NPlus(&self, v: &V) -> Set<V>;NPlus45,1722
    fn NMinus(&self, v: &V) -> Set<V>;NMinus48,1848
    fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices51,1997
    fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices54,2164
    fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident57,2306
    fn Degree(&self, v: &V) -> N;Degree60,2446
    fn InDegree(&self, v: &V) -> N;InDegree63,2568
    fn OutDegree(&self, v: &V) -> N;OutDegree66,2692
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> DirGraphStEphChap6_1Trait<V> foDirGraphStEph69,2732
    fn empty() -> DirGraphStEph<V> { DirGraphStEph { V: SetLit![], A: SetLit![] } }empty70,2849
    fn FromSets(V: Set<V>, A: Set<Pair<V, V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }FromSets71,2933
    fn vertices(&self) -> &Set<V> { &self.V }vertices72,3027
    fn arcs(&self) -> &Set<Pair<V, V>> { &self.A }arcs73,3073
    fn sizeV(&self) -> N { self.V.size() }sizeV74,3124
    fn sizeA(&self) -> N { self.A.size() }sizeA75,3167
    fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor77,3211
    fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG82,3458
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices84,3511
    fn NPlus(&self, v: &V) -> Set<V> {NPlus93,3751
    fn NMinus(&self, v: &V) -> Set<V> {NMinus99,3949
    fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices105,4148
    fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices114,4398
    fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } else Incident123,4652
    fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree125,4764
    fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree126,4822
    fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree127,4883
impl<V: Eq + Hash + Clone + std::fmt::Debug + std::fmt::Display> std::fmt::Debug for DirGraphStEDirGraphStEph130,4947
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt131,5051
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for DirGraphSDirGraphStEph136,5222
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt137,5328
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for DirGraphStEph<V> DirGraphStEph142,5459
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for DirGraphStEph<V> eq142,5459
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for DirGraphStEph<V> {}DirGraphStEph143,5637
    macro_rules! DirGraphLit {DirGraphLit146,5750
    fn _DirGraphLit_type_checks() {_DirGraphLit_type_checks163,6909
    pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise169,7175

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36Mt.rs,2179
pub mod Chapter36Mt {Chapter36Mt3,94
    pub trait Chapter36MtTrait<T: StT + Ord + Send + 'static> {Chapter36MtTrait14,315
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first15,379
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median316,432
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random17,487
        fn quick_sort_mt_first(&mut self);quick_sort_mt_first19,542
        fn quick_sort_mt_median3(&mut self);quick_sort_mt_median320,585
        fn quick_sort_mt_random(&mut self);quick_sort_mt_random21,630
    fn snapshot_mutex_vec<T: StT + Send>(a: &ArraySeqMtEphS<T>) -> Arc<Vec<Mutex<T>>> {snapshot_mutex_vec24,681
    fn commit_sorted<T: StT + Send>(a: &mut ArraySeqMtEphS<T>, data: &Arc<Vec<Mutex<T>>>) {commit_sorted30,918
    fn mutex_value_at<T: StT + Send>(data: &Arc<Vec<Mutex<T>>>, index: N) -> T {mutex_value_at38,1173
    fn swap_cells<T: StT + Send>(data: &Arc<Vec<Mutex<T>>>, i: N, j: N) {swap_cells42,1305
    fn partition<T: StT + Ord + Send>(partition54,1745
    fn pivot_arc_first<T: StT + Ord + Send>(data: &Arc<Vec<Mutex<T>>>, lo: N, _hi: N) -> T {pivot_arc_first79,2340
    fn pivot_arc_median3<T: StT + Ord + Send>(data: &Arc<Vec<Mutex<T>>>, lo: N, hi: N) -> T {pivot_arc_median383,2473
    fn pivot_arc_random<T: StT + Ord + Send>(data: &Arc<Vec<Mutex<T>>>, lo: N, hi: N) -> T {pivot_arc_random93,2905
    fn quick_sort_parallel<T: StT + Ord + Send + 'static>(quick_sort_parallel99,3108
    fn quick_sort_mt_impl<T: StT + Ord + Send + 'static>(quick_sort_mt_impl120,3899
    impl<T: StT + Ord + Send + 'static> Chapter36MtTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS131,4254
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first132,4338
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median3133,4415
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random142,4825
        fn quick_sort_mt_first(&mut self) {quick_sort_mt_first148,5001
        fn quick_sort_mt_median3(&mut self) {quick_sort_mt_median3151,5115
        fn quick_sort_mt_random(&mut self) {quick_sort_mt_random154,5233

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStPer.rs,2863
pub mod LinkedListStPer {LinkedListStPer3,64
    pub struct NodeP<T: StT> {NodeP7,144
        pub value: T,value8,175
        pub next: Option<Box<NodeP<T>>>,next9,197
    pub struct LinkedListStPerS<T: StT> {LinkedListStPerS13,266
        head: Option<Box<NodeP<T>>>,head14,308
        len: N,len15,345
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait18,368
        fn empty() -> Self;empty21,505
        fn new(length: N, init_value: T) -> Self;new24,635
        fn length(&self) -> N;length27,777
        fn nth(&self, index: N) -> &T;nth30,916
        fn isEmpty(&self) -> B;isEmpty33,1047
        fn isSingleton(&self) -> B;isSingleton36,1171
        fn singleton(item: T) -> Self;singleton39,1299
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set42,1483
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy47,1737
    impl<T: StT> LinkedListStPerS<T> {LinkedListStPerS50,1804
        fn push_front_node(&mut self, node: Box<NodeP<T>>) {push_front_node51,1843
        pub fn from_vec(v: Vec<T>) -> Self {from_vec58,2044
        pub fn iter<'a>(&'a self) -> LinkedListStPerIter<'a, T> {iter66,2309
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS73,2486
        fn empty() -> Self {empty74,2553
        fn new(length: N, init_value: T) -> Self {new77,2644
        fn length(&self) -> N {length96,3325
        fn nth(&self, index: N) -> &T {nth99,3388
        fn isEmpty(&self) -> B {isEmpty111,3755
        fn isSingleton(&self) -> B {isSingleton118,3913
        fn singleton(item: T) -> Self {singleton125,4075
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set134,4324
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy166,5452
    impl<T: StT> std::fmt::Debug for LinkedListStPerS<T> {LinkedListStPerS201,6593
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt202,6652
    pub struct LinkedListStPerIter<'a, T: StT> {LinkedListStPerIter213,7018
        cursor: Option<&'a NodeP<T>>,cursor214,7067
    impl<'a, T: StT> Iterator for LinkedListStPerIter<'a, T> {LinkedListStPerIter217,7112
        type Item = &'a T;Item218,7175
        fn next(&mut self) -> Option<Self::Item> {next219,7202
    impl<T: StT> PartialEq for LinkedListStPerS<T> {LinkedListStPerS229,7449
        fn eq(&self, other: &Self) -> bool {eq230,7502
    impl<T: StT> Eq for LinkedListStPerS<T> {}LinkedListStPerS247,7991
    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {LinkedListStPerS249,8039
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt250,8100
    macro_rules! LinkedListStPer {LinkedListStPer268,8628
    fn _LinkedListStPer_type_checks() {_LinkedListStPer_type_checks277,9120

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtEphChap18.rs,1663
pub mod ArraySeqMtEphChap18 {ArraySeqMtEphChap183,67
    pub trait ArraySeqMtEphChap18Trait<T: StT> {ArraySeqMtEphChap18Trait7,178
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate8,227
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map9,295
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append10,384
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter11,470
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;update12,557
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject13,646
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject14,747
    impl<T: StT> ArraySeqMtEphChap18Trait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS17,856
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate18,925
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map23,1143
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append32,1593
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter41,2143
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update46,2464
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject49,2612
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject54,2893

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
        fn filter(a: &AVLTreeSeqStEphS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T> {filter51,2345

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEphChap18.rs,2964
pub mod LinkedListStEphChap18 {LinkedListStEphChap183,60
pub trait LinkedListStEphChap18Trait<T: StT> {LinkedListStEphChap18Trait8,200
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;tabulate11,331
    fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map14,485
    fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append17,674
    fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T>;filter20,850
    fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T>;update23,1037
    fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEpinject26,1242
    fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEninject29,1457
    fn iterate< A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1561
    fn iteratePrefixes< A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkiteratePrefixes31,1647
    fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce32,1764
    fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, Tscan33,1842
    fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten34,1941
    fn collect<A: StT, Bv: StT>(a: &LinkedListStEphS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O) ->collect35,2024
impl<T: StT> LinkedListStEphChap18Trait<T> for LinkedListStEphS<T> {LinkedListStEphS38,2173
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate39,2242
    fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map44,2444
    fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append49,2843
    fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter57,3397
    fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T> { <Lupdate65,3799
    fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEpinject66,3964
    fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEninject83,4801
    fn iterate< A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate96,5488
    fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkeiteratePrefixes99,5762
    fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce105,6206
    fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, Tscan116,6924
    fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten124,7570
    fn collect<A: StT, Bv: StT>(a: &LinkedListStEphS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O) ->collect134,8190

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/RelationStEphChap5_2.rs,2071
pub mod RelationStEphChap5_2 {RelationStEphChap5_23,63
pub struct Relation<A, B> {Relation14,319
    pairs: Set<Pair<A, B>>,pairs15,347
pub trait RelationStEphChap5_2Trait<X: Eq + Hash + Display + Debug + Clone + Sized, RelationStEphChap5_2Trait18,378
    fn empty() -> Relation<X, Y>;empty22,631
    fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y>;FromSet26,762
    fn size(&self) -> N;size30,905
    fn domain(&self) -> Set<X>domain34,1019
    fn range(&self) -> Set<Y>range40,1167
    fn mem(&self, a: &X, b: &Y) -> Bmem46,1310
    fn iter(&self) -> Iter<'_, Pair<X, Y>>;iter51,1394
impl<A, B> Relation<A, B> {Relation54,1441
    pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B>FromVec55,1469
impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> PartialEq for Relation<A, BRelation64,1691
    fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq65,1791
impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> Eq for Relation<A, B> {}Relation68,1863
impl<A: Debug + Eq + Hash, B: Debug + Eq + Hash> Debug for Relation<A, B> {Relation70,1958
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt71,2034
impl<A: Display + Eq + Hash, B: Display + Eq + Hash> Display for Relation<A, B> {Relation76,2122
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt77,2204
impl<X: Eq + Hash + Display + Debug + Clone + Sized, Relation88,2512
    fn empty() -> Relation<X, Y> {empty90,2673
    fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y> { Relation { pairs } }FromSet94,2753
    fn size(&self) -> N { self.pairs.size() }size96,2834
    fn domain(&self) -> Set<X>domain98,2881
    fn range(&self) -> Set<Y>range107,3088
    fn mem(&self, a: &X, b: &Y) -> Bmem116,3294
    fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }iter124,3486
    macro_rules! RelationLit {RelationLit128,3574
    fn _RelationLit_type_checks() {_RelationLit_type_checks144,4507
    pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise150,4780

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStPerChap19.rs,1423
pub mod AVLTreeSeqStPerChap19 {AVLTreeSeqStPerChap193,49
    pub trait AVLTreeSeqStPerChap19Trait<T: StT> {AVLTreeSeqStPerChap19Trait9,266
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T>;tabulate10,317
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U>;map11,387
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T>;select12,480
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T>;append13,568
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T>;deflate14,660
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T>;filter15,731
    impl<T: StT> AVLTreeSeqStPerChap19Trait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS18,826
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T> {tabulate19,899
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U> {map22,1063
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T> {select25,1245
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T> {append39,1904
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T> {deflate42,2088
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T> {filter49,2399

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,4468
pub mod Types {Types4,45
    pub type N = usize;N9,156
    pub enum B {B13,299
        True,True14,316
        False,False15,330
    impl std::fmt::Display for B {B22,563
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt23,598
    pub trait StT: Eq + Clone + Display + Debug + Sized {}StT33,939
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}T34,998
    pub trait MtT: Sized + Send + Sync {MtT38,1206
        type Inner: StT;Inner39,1247
        fn clone_mt(&self) -> Self;clone_mt40,1272
        fn new_mt(inner: Self::Inner) -> Self;new_mt41,1308
    impl<T: StT + Send> MtT for std::sync::Mutex<T> {Mutex44,1366
        type Inner = T;Inner45,1420
        fn clone_mt(&self) -> Self {clone_mt46,1444
        fn new_mt(inner: Self::Inner) -> Self {new_mt50,1586
    impl<A: StT + Send + Sync, B: StT + Send + Sync> MtT for Pair<A, B> {Pair55,1696
        type Inner = Pair<A, B>;Inner56,1770
        fn clone_mt(&self) -> Self {clone_mt57,1803
        fn new_mt(inner: Self::Inner) -> Self {new_mt60,1875
    impl MtT for usize {usize66,2036
        type Inner = usize;Inner67,2061
        fn clone_mt(&self) -> Self { *self }clone_mt68,2089
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt69,2134
    impl MtT for isize {isize72,2197
        type Inner = isize;Inner73,2222
        fn clone_mt(&self) -> Self { *self }clone_mt74,2250
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt75,2295
    impl MtT for i32 {i3278,2358
        type Inner = i32;Inner79,2381
        fn clone_mt(&self) -> Self { *self }clone_mt80,2407
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt81,2452
    impl MtT for u32 {u3284,2515
        type Inner = u32;Inner85,2538
        fn clone_mt(&self) -> Self { *self }clone_mt86,2564
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt87,2609
    impl MtT for i64 {i6490,2672
        type Inner = i64;Inner91,2695
        fn clone_mt(&self) -> Self { *self }clone_mt92,2721
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt93,2766
    impl MtT for u64 {u6496,2829
        type Inner = u64;Inner97,2852
        fn clone_mt(&self) -> Self { *self }clone_mt98,2878
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt99,2923
    impl MtT for bool {bool103,2987
        type Inner = bool;Inner104,3011
        fn clone_mt(&self) -> Self { *self }clone_mt105,3038
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt106,3083
    impl MtT for char {char109,3146
        type Inner = char;Inner110,3170
        fn clone_mt(&self) -> Self { *self }clone_mt111,3197
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt112,3242
    impl MtT for String {String116,3359
        type Inner = String;Inner117,3385
        fn clone_mt(&self) -> Self { self.clone() }clone_mt118,3414
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt119,3466
    impl<'a> MtT for &'a str {str123,3564
        type Inner = &'a str;Inner124,3595
        fn clone_mt(&self) -> Self { *self }clone_mt125,3625
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt126,3670
    impl MtT for B {B130,3775
        type Inner = B;Inner131,3796
        fn clone_mt(&self) -> Self { *self }clone_mt132,3820
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt133,3865
    pub struct Edge<V: StT>(pub V, pub V);Edge138,4067
    impl<V: StT> std::fmt::Display for Edge<V> {Edge140,4111
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt141,4160
    impl<V: StT> From<(V, V)> for Edge<V> {Edge146,4304
        fn from(t: (V, V)) -> Self {from147,4348
    impl<V: StT> From<Edge<V>> for (V, V) {V152,4429
        fn from(e: Edge<V>) -> (V, V) {from153,4473
    pub struct Pair<A, B>(pub A, pub B);Pair160,4707
    impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {Pair162,4749
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt163,4837
    impl<A, B> From<(A, B)> for Pair<A, B> {Pair168,4981
        fn from(t: (A, B)) -> Self {from169,5026
    impl<A, B> From<Pair<A, B>> for (A, B) {B174,5107
        fn from(p: Pair<A, B>) -> (A, B) {from175,5152
    pub fn ArraySeqSetEq<T: PartialEq>(a_len: N, a_nth: impl Fn(N) -> T, b_len: N, b_nth: impl FArraySeqSetEq183,5483

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStEph.rs,2392
pub mod ArraySeqStEph {ArraySeqStEph3,93
    pub struct ArraySeqStEphS<T: StT> {ArraySeqStEphS13,340
        pub data: Box<[T]>,data14,380
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait18,463
        fn new(length: N, init_value: T) -> Self;new21,608
        fn length(&self) -> N;length24,750
        fn nth(&self, index: N) -> &T;nth27,873
        fn empty() -> Self;empty30,1004
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set33,1124
        fn singleton(item: T) -> Self;singleton36,1297
        fn isEmpty(&self) -> B;isEmpty39,1428
        fn isSingleton(&self) -> B;isSingleton42,1552
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy45,1690
    impl<T: StT> ArraySeqStEphS<T> {ArraySeqStEphS48,1757
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq51,1886
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy59,2215
        pub fn from_vec(v: Vec<T>) -> Self {from_vec77,2961
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update84,3195
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter93,3495
    impl<T: StT> PartialEq for ArraySeqStEphS<T> {ArraySeqStEphS96,3565
        fn eq(&self, other: &Self) -> bool {eq97,3616
    impl<T: StT> Eq for ArraySeqStEphS<T> {}ArraySeqStEphS110,3943
    impl<T: StT> Debug for ArraySeqStEphS<T> {ArraySeqStEphS112,3989
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt113,4036
    impl<T: StT> Display for ArraySeqStEphS<T> {ArraySeqStEphS119,4229
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt120,4278
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS130,4561
        fn new(length: N, init_value: T) -> Self {new133,4726
        fn length(&self) -> N {length138,4942
        fn nth(&self, index: N) -> &T {nth143,5104
        fn empty() -> Self {empty148,5276
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set153,5456
        fn singleton(item: T) -> Self {singleton163,5825
        fn isEmpty(&self) -> B {isEmpty168,6016
        fn isSingleton(&self) -> B {isSingleton177,6273
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy186,6544
    macro_rules! ArraySeqStEph {ArraySeqStEph192,6686

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtEphChap19.rs,2409
pub mod ArraySeqMtEphChap19 {ArraySeqMtEphChap193,67
    pub trait ArraySeqMtEphChap19Trait<T: StT> {ArraySeqMtEphChap19Trait8,238
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate9,287
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map10,355
        fn select<'a>(a: &'a ArraySeqMtEphS<T>, b: &'a ArraySeqMtEphS<T>, i: N) -> Option<T>;select11,444
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append12,538
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append213,624
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T>;deflate14,711
        fn filter(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter15,780
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,864
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,951
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtEphS<T>, Tscan18,1031
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;flatten19,1130
    impl<T: StT> ArraySeqMtEphChap19Trait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS22,1217
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate23,1286
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map26,1444
        fn select<'a>(a: &'a ArraySeqMtEphS<T>, b: &'a ArraySeqMtEphS<T>, i: N) -> Option<T> {select29,1618
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append35,1920
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append238,2094
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T> {deflate41,2269
        fn filter(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter44,2472
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, mut x: A) -> A {iterate47,2644
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> T {reduce50,2816
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> (ArraySeqMtEphS<Tscan53,2984
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {flatten59,3378

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStEphChap19.rs,2316
pub mod ArraySeqStEphChap19 {ArraySeqStEphChap193,51
pub trait ArraySeqStEphChap19Trait<T: StT> {ArraySeqStEphChap19Trait8,210
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate11,339
    fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map14,491
    fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T>;select17,660
    fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append20,850
    fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append223,1032
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T>;deflate26,1199
    fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter29,1352
    fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1432
    fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce31,1515
    fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, T);scan32,1591
    fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten33,1686
impl<T: StT> ArraySeqStEphChap19Trait<T> for ArraySeqStEphS<T> {ArraySeqStEphS36,1765
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate37,1830
    fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map40,1976
    fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T> {select43,2138
    fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append55,2492
    fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append258,2654
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T> {deflate61,2817
    fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter68,3090
    fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate71,3250
    fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce74,3417
    fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, T) {scan77,3577
    fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten80,3754

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPerChap18.rs,2650
pub mod ArraySeqStPerChap18 {ArraySeqStPerChap183,46
pub trait ArraySeqStPerChap18Trait<T: StT> {ArraySeqStPerChap18Trait7,149
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,368
    fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map14,594
    fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append18,786
    fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1058
    fn update(a: &ArrayStPerS<T>, item_at: Pair<N, T>) -> ArrayStPerS<T>;update26,1228
    fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject30,1455
    fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject34,1660
    fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate37,1823
    fn iteratePrefixes<A: StT + Clone>(iteratePrefixes40,1945
    fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce47,2170
    fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan50,2287
    fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten53,2443
    fn collect(collect56,2587
impl<T: StT> ArraySeqStPerChap18Trait<T> for ArrayStPerS<T> {ArrayStPerS62,2725
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate63,2787
    fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map67,2950
    fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append79,3398
    fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter93,3866
    fn update(a: &ArrayStPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayStPerS<T> {update102,4165
    fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject108,4370
    fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject120,4886
    fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate130,5288
    fn iteratePrefixes<A: StT + Clone>(iteratePrefixes137,5492
    fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce150,5890
        fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec151,5964
    fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan166,6404
        fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec167,6494
    fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten186,7150
    fn collect(collect196,7478

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/MappingStEphChap5_5.rs,2533
pub mod MappingStEphChap5_5 {MappingStEphChap5_53,72
pub struct Mapping<A, B> {Mapping12,325
    rel: Relation<A, B>,rel13,352
pub trait MappingStEphChap5_5Trait<X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + MappingStEphChap5_5Trait16,380
    fn empty() -> Mapping<X, Y>;empty19,637
    fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y>;FromVec23,759
    fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation27,901
    fn size(&self) -> N;size31,1044
    fn domain(&self) -> Set<X>;domain35,1158
    fn range(&self) -> Set<Y>;range39,1279
    fn mem(&self, a: &X, b: &Y) -> B;mem43,1395
    fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>>;iter45,1434
impl<A: Eq + Hash, B: Eq + Hash> Mapping<A, B> {Mapping48,1509
    fn unique_pairs_from_iter<I: IntoIterator<Item = Pair<A, B>>>(iter: I) -> Set<Pair<A, B>> {unique_pairs_from_iter49,1558
impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std:Mapping57,1878
    fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq58,2017
impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std:Mapping60,2084
impl<A: std::fmt::Debug + Eq + Hash, B: std::fmt::Debug + Eq + Hash> std::fmt::Debug for MappingMapping62,2218
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }fmt63,2323
impl<A: std::fmt::Display + Eq + Hash, B: std::fmt::Display + Eq + Hash> std::fmt::Display for MMapping65,2416
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }fmt66,2527
impl<X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized, Y: Eq + Hash + std::fmtMapping69,2621
    fn empty() -> Mapping<X, Y> {empty70,2813
    fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y> {FromVec74,2940
    fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation79,3147
    fn size(&self) -> N { self.rel.size() }size84,3375
    fn domain(&self) -> Set<X> { self.rel.domain() }domain86,3420
    fn range(&self) -> Set<Y> { self.rel.range() }range88,3474
    fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem90,3526
    fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.rel.iter() }iter92,3587
    macro_rules! MappingLit {MappingLit96,3701
    fn _MappingLit_type_checks() {_MappingLit_type_checks107,4304
    pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise113,4574

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main7,236

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtPerChap18.rs,2557
pub mod ArraySeqMtPerChap18 {ArraySeqMtPerChap183,82
    pub trait ArraySeqMtPerChap18Trait<T: MtT> {ArraySeqMtPerChap18Trait7,193
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;tabulate10,424
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;map13,661
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;append16,856
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;filter19,1139
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;update22,1320
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;inject25,1558
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;ninject28,1774
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1944
        fn iteratePrefixes<A: MtT>(iteratePrefixes32,2073
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce38,2313
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan40,2437
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten42,2600
        fn collect(collect44,2751
    impl<T: MtT> ArraySeqMtPerChap18Trait<T> for ArrayMtPerS<T> {ArrayMtPerS50,2909
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate51,2975
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map56,3155
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append60,3349
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter67,3625
        fn update(a: &ArrayMtPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayMtPerS<T> {update77,3982
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {inject81,4143
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject94,4792
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate105,5227
        fn iteratePrefixes<A: MtT>(iteratePrefixes113,5460
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce128,5946
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan137,6248
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten147,6632
        fn collect(collect158,7034

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtPer.rs,3798
pub mod ArraySeqMtPer {ArraySeqMtPer8,390
    pub struct ArrayMtPerS<T> { data: Box<[T]> }ArrayMtPerS16,633
    pub struct ArrayMtPerS<T> { data: Box<[T]> }data16,633
    pub trait ArraySeqMtPerTrait<T: MtT> {ArraySeqMtPerTrait19,754
        fn new(length: N, init_value: T) -> Self;new22,899
        fn length(&self) -> N;length25,1041
        fn nth(&self, index: N) -> &T;nth28,1164
        fn empty() -> Self;empty31,1295
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> where Self: Sized;set34,1484
        fn singleton(item: T) -> Self;singleton37,1666
        fn isEmpty(&self) -> B;isEmpty40,1797
        fn isSingleton(&self) -> B;isSingleton43,1921
        fn subseq_copy(&self, start: N, length: N) -> Self where Self: Sized;subseq_copy46,2059
    impl<T> ArrayMtPerS<T> {ArrayMtPerS49,2144
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq51,2214
        pub fn from_vec(v: Vec<T>) -> Self { ArrayMtPerS { data: v.into_boxed_slice() } }from_vec60,2577
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter62,2668
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }iter_mut63,2731
        pub fn empty() -> Self { ArrayMtPerS { data: Vec::new().into_boxed_slice() } }empty65,2810
        pub fn singleton(item: T) -> Self { ArrayMtPerS { data: vec![item].into_boxed_slice() } singleton66,2897
        pub fn new(length: N, init_value: T) -> Self where T: Clone { Self::from_vec(vec![init_vnew67,2995
        pub fn length(&self) -> N { self.data.len() }length68,3108
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth69,3162
    impl<T: MtT> ArrayMtPerS<T> {ArrayMtPerS72,3237
        pub fn new_mt(length: N, init_value: T) -> Self { new_mt73,3271
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str> set81,3517
        pub fn subseq_copy_mt(&self, start: N, length: N) -> Self {subseq_copy_mt89,3846
    impl<T: MtT> Clone for ArrayMtPerS<T> {ArrayMtPerS98,4173
        fn clone(&self) -> Self {clone99,4217
    impl<T: MtT + StT + Send + Sync> MtT for ArrayMtPerS<T> {ArrayMtPerS105,4427
        type Inner = ArrayMtPerS<T>;Inner106,4489
        fn clone_mt(&self) -> Self {clone_mt107,4526
        fn new_mt(inner: Self::Inner) -> Self {new_mt110,4598
    impl<T: Eq> PartialEq for ArrayMtPerS<T> {ArrayMtPerS116,4682
        fn eq(&self, other: &Self) -> bool {eq117,4729
    impl<T: Eq> Eq for ArrayMtPerS<T> {}ArrayMtPerS123,4963
    impl<T: Debug> Debug for ArrayMtPerS<T> {ArrayMtPerS125,5005
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt126,5051
    impl<T: Display> Display for ArrayMtPerS<T> {ArrayMtPerS132,5244
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt133,5294
    impl<T: MtT> ArraySeqMtPerTrait<T> for ArrayMtPerS<T> {ArrayMtPerS143,5579
        fn new(length: N, init_value: T) -> Self {new144,5639
        fn length(&self) -> N { self.data.len() }length147,5752
        fn nth(&self, index: N) -> &T { &self.data[index] }nth148,5802
        fn empty() -> Self { Self::from_vec(Vec::new()) }empty149,5862
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set150,5920
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton153,6037
        fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }isEmpty154,6106
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton155,6195
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy156,6288
    macro_rules! ArraySeqMtPer {ArraySeqMtPer163,6434
    fn _ArraySeqMtPer_macro_type_checks() {_ArraySeqMtPer_macro_type_checks170,6786

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStEph.rs,4183
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
    fn empty() -> Self;empty42,894
    fn new() -> Self;new45,1002
    fn length(&self) -> N;length48,1108
    fn nth(&self, index: N) -> &T;nth51,1235
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set54,1370
    fn singleton(item: T) -> Self;singleton57,1531
    fn isEmpty(&self) -> B;isEmpty60,1650
    fn isSingleton(&self) -> B;isSingleton63,1762
    fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy66,1918
impl<T: StT> AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS69,1977
    pub fn new_root() -> Self {new_root70,2012
    pub fn new() -> Self {new76,2136
    pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqStEphS<T> {update79,2194
    pub fn from_vec(values: Vec<T>) -> AVLTreeSeqStEphS<T>from_vec83,2385
    pub fn to_arrayseq(&self) -> ArraySeqStEphS<T>to_arrayseq93,2736
    pub fn iter<'a>(&'a self) -> AVLTreeSeqIterStEph<'a, T> {iter113,3363
    pub fn push_back(&mut self, value: T) {push_back116,3476
    pub fn contains_value(&self, target: &T) -> Bcontains_value121,3670
    pub fn insert_value(&mut self, value: T) {insert_value130,3865
    pub fn delete_value(&mut self, target: &T) -> booldelete_value133,3949
impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS159,4675
    fn empty() -> Self {empty160,4738
    fn new() -> Self {new164,4807
    fn length(&self) -> N {length168,4874
    fn nth(&self, index: N) -> &T {nth172,4939
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set176,5018
    fn singleton(item: T) -> Self {singleton181,5168
    fn isEmpty(&self) -> B {isEmpty187,5345
    fn isSingleton(&self) -> B {isSingleton195,5481
    fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy203,5621
pub struct AVLTreeSeqIterStEph<'a, T: StT> {AVLTreeSeqIterStEph219,6090
    stack: Vec<&'a AVLTreeNode<T>>,stack220,6135
    current: Option<&'a AVLTreeNode<T>>,current221,6171
impl<'a, T: StT> AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph224,6215
    fn new(root: &'a Link<T>) -> Self {new225,6261
    fn push_left(&mut self, link: &'a Link<T>) {push_left233,6458
impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph242,6678
    type Item = &'a T;Item243,6737
    fn next(&mut self) -> Option<Self::Item> {next244,6760
    fn h<T: StT>(n: &Link<T>) -> N {h253,7012
    fn size_link<T: StT>(n: &Link<T>) -> N {size_link257,7099
    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>) {update_meta265,7264
    fn rotate_right<T: StT>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right273,7509
    fn rotate_left<T: StT>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left284,7872
    fn rebalance<T: StT>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance295,8233
    fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> &'a T {nth_link316,8998
    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str> {set_link328,9378
    pub(crate) fn insert_at_link<T: StT>(insert_at_link345,9964
    macro_rules! AVLTreeSeqStEph {AVLTreeSeqStEph371,10815
    fn _AVLTreeSeqStEph_type_checks() {_AVLTreeSeqStEph_type_checks386,11400
    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS391,11659
        fn eq(&self, other: &Self) -> bool {eq392,11712
    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}AVLTreeSeqStEphS405,12039

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36St.rs,2088
pub mod Chapter36St {Chapter36St3,95
    pub trait Chapter36StTrait<T: StT + Ord> {Chapter36StTrait9,225
        fn pivot_st_first(&self, lo: N, hi: N) -> T;pivot_st_first10,272
        fn pivot_st_median3(&self, lo: N, hi: N) -> T;pivot_st_median311,325
        fn pivot_st_random(&self, lo: N, hi: N) -> T;pivot_st_random12,380
        fn quick_sort_st_first(&mut self);quick_sort_st_first14,435
        fn quick_sort_st_median3(&mut self);quick_sort_st_median315,478
        fn quick_sort_st_random(&mut self);quick_sort_st_random16,523
    impl<T: StT + Ord> Chapter36StTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS19,574
        fn pivot_st_first(&self, lo: N, _hi: N) -> T { self.nth(lo).clone() }pivot_st_first20,641
        fn pivot_st_median3(&self, lo: N, hi: N) -> T {pivot_st_median321,719
        fn pivot_st_random(&self, lo: N, hi: N) -> T {pivot_st_random30,1132
        fn quick_sort_st_first(&mut self) {quick_sort_st_first36,1309
            fn swap<T: StT>(a: &mut ArraySeqStEphS<T>, i: N, j: N) {swap37,1353
            fn partition<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N, p: &T) -> (N, N)partition44,1636
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort56,2135
        fn quick_sort_st_median3(&mut self) {quick_sort_st_median367,2512
            fn swap<T: StT>(a: &mut ArraySeqStEphS<T>, i: N, j: N) {swap68,2558
            fn median3<T: StT + Ord>(a: &ArraySeqStEphS<T>, lo: N, hi: N) -> T {median375,2841
            fn partition<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N, p: &T) -> (N, N)partition84,3302
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort96,3801
        fn quick_sort_st_random(&mut self) {quick_sort_st_random107,4179
            fn swap<T: StT>(a: &mut ArraySeqStEphS<T>, i: N, j: N) {swap108,4224
            fn partition<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N, p: &T) -> (N, N)partition115,4507
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort127,5006

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStPerChap18.rs,2608
pub mod LinkedListStPerChap18 {LinkedListStPerChap183,48
    pub trait LinkedListStPerChap18Trait<T: StT> {LinkedListStPerChap18Trait7,165
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate10,384
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map14,627
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append18,833
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter22,1132
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;update26,1324
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>,)inject29,1490
        fn ninject(ninject33,1667
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate39,1888
        fn iteratePrefixes<A: StT>(iteratePrefixes42,2023
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce49,2274
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan52,2404
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten55,2578
        fn collect<A: StT, Bv: StT>(collect58,2745
    impl<T: StT> LinkedListStPerChap18Trait<T> for LinkedListStPerS<T> {LinkedListStPerS64,2937
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate65,3010
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map73,3261
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append81,3724
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter93,4366
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T>update103,4836
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>,) inject112,5380
        fn ninject(ninject137,6481
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate163,7575
        fn iteratePrefixes<A: StT>(iteratePrefixes173,7971
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce190,8652
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan206,9486
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten227,10482
        fn collect<A: StT, Bv: StT>(collect242,11224

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,1740
pub mod Types;Types7,226
pub mod MathSeq;MathSeq10,274
pub mod SetStEphChap5_1;SetStEphChap5_113,351
pub mod RelationStEphChap5_2;RelationStEphChap5_216,429
pub mod MappingStEphChap5_5;MappingStEphChap5_519,522
pub mod DirGraphStEphChap6_1;DirGraphStEphChap6_122,612
pub mod UnDirGraphStEphChap6_1;UnDirGraphStEphChap6_124,704
pub mod LinkedListStPer;LinkedListStPer27,803
pub mod LinkedListStPerChap18;LinkedListStPerChap1829,880
pub mod LinkedListStPerChap19;LinkedListStPerChap1931,975
pub mod LinkedListStEph;LinkedListStEph34,1071
pub mod LinkedListStEphChap18;LinkedListStEphChap1836,1148
pub mod LinkedListStEphChap19;LinkedListStEphChap1938,1243
pub mod ArraySeqStPer;ArraySeqStPer41,1339
pub mod ArraySeqStPerChap18;ArraySeqStPerChap1844,1411
pub mod ArraySeqStPerChap19;ArraySeqStPerChap1947,1501
pub mod ArraySeqMtPer;ArraySeqMtPer51,1626
pub mod ArraySeqMtPerChap18;ArraySeqMtPerChap1853,1697
pub mod ArraySeqMtPerChap19;ArraySeqMtPerChap1955,1786
pub mod ArraySeqStEph;ArraySeqStEph58,1876
pub mod ArraySeqStEphChap18;ArraySeqStEphChap1861,1948
pub mod ArraySeqStEphChap19;ArraySeqStEphChap1963,2037
pub mod ArraySeqMtEph;ArraySeqMtEph67,2161
pub mod ArraySeqMtEphChap18;ArraySeqMtEphChap1869,2232
pub mod ArraySeqMtEphChap19;ArraySeqMtEphChap1971,2321
pub mod AVLTreeSeqStPer;AVLTreeSeqStPer74,2411
pub mod AVLTreeSeqStPerChap18;AVLTreeSeqStPerChap1876,2488
pub mod AVLTreeSeqStPerChap19;AVLTreeSeqStPerChap1978,2583
pub mod AVLTreeSeqStEph;AVLTreeSeqStEph81,2679
pub mod AVLTreeSeqStEphChap18;AVLTreeSeqStEphChap1883,2756
pub mod AVLTreeSeqStEphChap19;AVLTreeSeqStEphChap1985,2851
pub mod Chapter36St;Chapter36St89,2972
pub mod Chapter36Mt;Chapter36Mt91,3037

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStPerChap19.rs,2712
pub mod LinkedListStPerChap19 {LinkedListStPerChap193,48
    pub trait LinkedListStPerChap19Trait<T: StT> {LinkedListStPerChap19Trait8,229
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate9,280
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map10,350
        fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T>select11,443
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append12,541
        fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append213,633
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T>;deflate14,726
        fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter15,797
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,885
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,974
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan18,1056
        fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten19,1159
        fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>) -> Linkeinject20,1245
    impl<T: StT> LinkedListStPerChap19Trait<T> for LinkedListStPerS<T> {LinkedListStPerS23,1364
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate24,1437
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map27,1601
        fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T>select30,1783
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append44,2431
        fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append248,2616
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T> {deflate52,2802
        fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter60,3114
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate64,3295
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce68,3481
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan72,3660
        fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten76,3858
        fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>) -> Linkeinject80,4035

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/SetStEphChap5_1.rs,3489
pub mod SetStEphChap5_1 {SetStEphChap5_13,69
pub struct Set<T> { data: HashSet<T> }Set11,236
pub struct Set<T> { data: HashSet<T> }data11,236
pub trait SetStEphChap5_1Trait<T: Eq + Hash + Clone + Display + Debug + Sized> {SetStEphChap5_1Trait13,276
    fn empty() -> Set<T>;empty16,441
    fn singleton(x: T) -> Set<T>;singleton19,551
    fn size(&self) -> N;size22,669
    fn mem(&self, x: &T) -> B;mem25,778
    fn union(&self, other: &Set<T>) -> Set<T>;union28,909
    fn intersection(&self, other: &Set<T>) -> Set<T>;intersection31,1056
    fn partition(&self, parts: &Set<Set<T>>) -> B;partition34,1224
    fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>;CartesianProduct38,1378
    fn insert(&mut self, x: T) -> &mut Self;insert42,1545
    fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;iter46,1675
    fn FromVec(v: Vec<T>) -> Set<T>;FromVec49,1826
impl<T: Eq + Hash> PartialEq for Set<T> {Set52,1866
    fn eq(&self, other: &Self) -> bool { self.data == other.data }eq53,1908
impl<T: Eq + Hash> Eq for Set<T> {}Set56,1978
impl<T: Eq + Hash> std::fmt::Debug for Set<T>Set58,2015
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt62,2093
impl<T: Eq + Hash> std::fmt::Display for Set<T>Set67,2232
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt71,2314
impl<T: Eq + Hash > Hash for Set<T> {Set83,2704
    fn hash<H: Hasher>(&self, state: &mut H) {hash84,2742
impl<T: Eq + Hash> Set<T> {Set98,3226
    pub fn empty() -> Set<T> {empty99,3254
    pub fn singleton(x: T) -> Set<T> {singleton103,3329
    pub fn size(&self) -> N { self.data.len() }size109,3475
    pub fn mem(&self, x: &T) -> B {mem111,3524
    pub fn union(&self, other: &Set<T>) -> Set<T> where T: Clone {union115,3630
    pub fn intersection(&self, other: &Set<T>) -> Set<T> where T: Clone {intersection121,3827
    pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition127,4107
    pub fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>> where T: ClCartesianProduct141,4522
    pub fn insert(&mut self, x: T) -> &mut Self {insert151,4891
    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter156,4998
    pub fn FromVec(v: Vec<T>) -> Set<T> {FromVec158,5086
impl<T: Eq + Hash + Clone + Display + Debug + Sized> SetStEphChap5_1Trait<T> for Set<T> {Set165,5258
    fn empty() -> Set<T> {empty166,5348
    fn singleton(x: T) -> Set<T> {singleton170,5419
    fn size(&self) -> N { self.data.len() }size176,5561
    fn mem(&self, x: &T) -> B {mem178,5606
    fn union(&self, other: &Set<T>) -> Set<T> where T: Clone {union182,5708
    fn intersection(&self, other: &Set<T>) -> Set<T> where T: Clone {intersection188,5901
    fn partition(&self, parts: &Set<Set<T>>) -> B {partition194,6177
    fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>> where T: Clone,CartesianProduct208,6588
    fn insert(&mut self, x: T) -> &mut Self {insert218,6953
    fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter223,7056
    fn FromVec(v: Vec<T>) -> Set<T> {FromVec225,7140
    macro_rules! SetLit {SetLit234,7329
    fn _SetLit_type_checks() {_SetLit_type_checks246,7683
    pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise252,7939
        let _s0: Set<&'static str> = SetLit![];str253,7985

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEphChap19.rs,2622
pub mod LinkedListStEphChap19 {LinkedListStEphChap193,60
    pub trait LinkedListStEphChap19Trait<T: StT> {LinkedListStEphChap19Trait8,241
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;tabulate9,292
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map10,362
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>select11,455
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append12,553
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append213,645
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T>;deflate14,738
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T>;filter15,809
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,897
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,986
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan18,1068
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten19,1171
        fn inject(inject20,1257
    impl<T: StT> LinkedListStEphChap19Trait<T> for LinkedListStEphS<T> {LinkedListStEphS26,1411
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate27,1484
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map30,1648
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>select33,1830
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append47,2478
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append250,2662
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T> {deflate53,2847
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter60,3158
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate63,3338
    fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce66,3523
    fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, Tscan69,3689
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten72,3874
        fn inject(values: &LinkedListStEphS<T>,changes: &LinkedListStEphS<Pair<N, T>>,) inject75,4050

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtEph.rs,2820
pub mod ArraySeqMtEph {ArraySeqMtEph8,381
    pub struct ArraySeqMtEphS<T: StT> {ArraySeqMtEphS17,646
        data: Mutex<Box<[T]>>,data18,686
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait22,772
        fn new(length: N, init_value: T) -> Self;new24,861
        fn length(&self) -> N;length26,952
        fn nth_cloned(&self, index: N) -> T;nth_cloned29,1113
        fn empty() -> Self;empty31,1199
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set33,1268
        fn singleton(item: T) -> Self;singleton35,1390
        fn isEmpty(&self) -> B;isEmpty37,1470
        fn isSingleton(&self) -> B;isSingleton39,1543
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy41,1625
    impl<T: StT> ArraySeqMtEphS<T> {ArraySeqMtEphS44,1692
        pub fn from_vec(v: Vec<T>) -> Self {from_vec46,1770
        pub fn length(&self) -> N {length51,1937
        pub fn nth_cloned(&self, index: N) -> T {nth_cloned57,2100
        pub fn iter_snapshot(&self) -> Iter<'_, T> where T: 'static {iter_snapshot64,2369
        pub fn to_vec(&self) -> Vec<T> {to_vec74,2838
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy80,3031
    impl<T: StT> Clone for ArraySeqMtEphS<T> {ArraySeqMtEphS90,3395
        fn clone(&self) -> Self {clone91,3442
    impl<T: StT> PartialEq for ArraySeqMtEphS<T> {ArraySeqMtEphS96,3545
        fn eq(&self, other: &Self) -> bool {eq97,3596
    impl<T: StT> Eq for ArraySeqMtEphS<T> {}ArraySeqMtEphS103,3747
    impl<T: StT> Debug for ArraySeqMtEphS<T> {ArraySeqMtEphS105,3793
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt106,3840
    impl<T: StT> Display for ArraySeqMtEphS<T> {ArraySeqMtEphS112,4008
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt113,4057
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS124,4374
        fn new(length: N, init_value: T) -> Self {new125,4437
        fn length(&self) -> N { self.length() }length128,4561
        fn nth_cloned(&self, index: N) -> T { self.nth_cloned(index) }nth_cloned129,4609
        fn empty() -> Self { ArraySeqMtEphS::from_vec(Vec::new()) }empty130,4680
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set131,4748
        fn singleton(item: T) -> Self { ArraySeqMtEphS::from_vec(vec![item]) }singleton142,5128
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty143,5207
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton144,5294
        fn subseq_copy(&self, start: N, length: N) -> Self { self.subseq_copy(start, length) }subseq_copy145,5385
    macro_rules! ArraySeqMtEph {ArraySeqMtEph149,5507

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEph.rs,2892
pub mod LinkedListStEph {LinkedListStEph3,90
pub struct NodeE<T: StT> {NodeE7,162
    pub value: T,value8,189
    pub next: Option<Box<NodeE<T>>>,next9,207
pub struct LinkedListStEphS<T: StT> {LinkedListStEphS13,264
    head: Option<Box<NodeE<T>>>,head14,302
    len: N,len15,335
pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait18,350
    fn empty() -> LinkedListStEphS<T>;empty21,475
    fn new(length: N, init_value: T) -> Self;new24,608
    fn length(&self) -> N;length27,738
    fn nth(&self, index: N) -> &T;nth30,865
    fn isEmpty(&self) -> B;isEmpty33,984
    fn isSingleton(&self) -> B;isSingleton36,1096
    fn singleton(item: T) -> Self;singleton39,1212
    fn update(&mut self, item_at: Pair<N, T>) -> &mut Self;update42,1347
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set45,1507
    fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy48,1720
impl<T: StT> LinkedListStEphS<T> {LinkedListStEphS51,1779
    fn push_front_node(&mut self, node: Box<NodeE<T>>) {push_front_node54,1898
    pub fn from_vec(v: Vec<T>) -> Self {from_vec63,2163
    pub fn iter<'a>(&'a self) -> LinkedListStEphIter<'a, T> {iter73,2484
impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS78,2616
    fn empty() -> Self {empty81,2763
    fn new(length: N, init_value: T) -> Self {new86,2936
    fn length(&self) -> N {length107,3625
    fn nth(&self, index: N) -> &T {nth112,3776
    fn isEmpty(&self) -> B {isEmpty126,4179
    fn isSingleton(&self) -> B {isSingleton135,4393
    fn singleton(item: T) -> Self {singleton144,4611
    fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {update155,4924
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set170,5375
    fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy185,5901
impl<T: StT> std::fmt::Debug for LinkedListStEphS<T> {LinkedListStEphS221,6943
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt222,6998
pub struct LinkedListStEphIter<'a, T: StT> {LinkedListStEphIter233,7324
    cursor: Option<&'a NodeE<T>>,cursor234,7369
impl<'a, T: StT> Iterator for LinkedListStEphIter<'a, T> {LinkedListStEphIter237,7406
    type Item = &'a T;Item238,7465
    fn next(&mut self) -> Option<Self::Item> {next239,7488
impl<T: StT> PartialEq for LinkedListStEphS<T> {LinkedListStEphS247,7679
    fn eq(&self, other: &Self) -> bool {eq248,7728
impl<T: StT> Eq for LinkedListStEphS<T> {}LinkedListStEphS265,8153
impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {LinkedListStEphS267,8197
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt268,8254
    macro_rules! LinkedListStEph {LinkedListStEph283,8663
    fn _LinkedListStEph_type_checks() {_LinkedListStEph_type_checks305,9958

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPer.rs,3080
pub mod ArraySeqStPer {ArraySeqStPer8,354
    pub struct ArrayStPerS<T: StT> { data: Box<[T]> }ArrayStPerS17,650
    pub struct ArrayStPerS<T: StT> { data: Box<[T]> }data17,650
    pub trait ArraySeqStPerTrait<T: StT + Clone> {ArraySeqStPerTrait20,776
        fn new(length: N, init_value: T) -> Self;new23,929
        fn length(&self) -> N;length26,1071
        fn nth(&self, index: N) -> &T;nth29,1194
        fn empty() -> Self;empty32,1325
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> where Self: Sized;set35,1514
        fn singleton(item: T) -> Self;singleton38,1696
        fn isEmpty(&self) -> B;isEmpty41,1827
        fn isSingleton(&self) -> B;isSingleton44,1951
        fn subseq_copy(&self, start: N, length: N) -> Self where Self: Sized;subseq_copy47,2089
    impl<T: StT> ArrayStPerS<T> {ArrayStPerS50,2174
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq53,2300
        pub fn from_vec(v: Vec<T>) -> Self { ArrayStPerS { data: v.into_boxed_slice() } }from_vec62,2663
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter64,2754
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }iter_mut65,2817
        pub fn empty() -> Self { ArrayStPerS { data: Vec::new().into_boxed_slice() } }empty67,2896
        pub fn singleton(item: T) -> Self { ArrayStPerS { data: vec![item].into_boxed_slice() } singleton68,2983
        pub fn new(length: N, init_value: T) -> Self where T: Clone { Self::from_vec(vec![init_vnew69,3081
        pub fn length(&self) -> N { self.data.len() }length70,3194
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth71,3248
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {set72,3312
    impl<T: StT + Debug> Debug for ArrayStPerS<T> {ArrayStPerS81,3616
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt82,3668
    impl<T: StT + Display> Display for ArrayStPerS<T> {ArrayStPerS88,3861
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt89,3917
    impl<T: StT + Clone> ArraySeqStPerTrait<T> for ArrayStPerS<T> {ArrayStPerS99,4202
        fn new(length: N, init_value: T) -> Self {new100,4270
        fn length(&self) -> N { self.data.len() }length103,4384
        fn nth(&self, index: N) -> &T { &self.data[index] }nth104,4434
        fn empty() -> Self { Self::from_vec(Vec::new()) }empty105,4494
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set106,4552
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton112,4829
        fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }isEmpty113,4898
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton114,4987
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy115,5080
    macro_rules! ArraySeqStPer {ArraySeqStPer127,5440
    fn _ArraySeqStPer_macro_type_checks() {_ArraySeqStPer_macro_type_checks134,5792

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/UnDirGraphStEphChap6_1.rs,2921
pub mod UnDirGraphStEphChap6_1 {UnDirGraphStEphChap6_13,80
pub struct UnDirGraphStEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {UnDirGraphStEph10,247
    V: Set<V>,V11,336
    E: Set<Pair<V, V>>,E12,351
pub trait UnDirGraphStEphChap6_1Trait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::DebugUnDirGraphStEphChap6_1Trait15,378
    fn empty() -> UnDirGraphStEph<V>;empty18,562
    fn FromSets(V: Set<V>, E: Set<Pair<V, V>>) -> UnDirGraphStEph<V>;FromSets21,700
    fn vertices(&self) -> &Set<V>;vertices24,854
    fn edges(&self) -> &Set<Pair<V, V>>;edges27,973
    fn sizeV(&self) -> N;sizeV30,1098
    fn sizeE(&self) -> N;sizeE33,1208
    fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor36,1318
    fn NG(&self, v: &V) -> Set<V>;NG39,1449
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices42,1594
    fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident45,1732
    fn Degree(&self, v: &V) -> N;Degree48,1872
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> UnDirGraphStEphChap6_1Trait<V> UnDirGraphStEph51,1909
    fn empty() -> UnDirGraphStEph<V> { UnDirGraphStEph { V: SetLit![], E: SetLit![] } }empty52,2030
    fn FromSets(V: Set<V>, E: Set<Pair<V, V>>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E } FromSets53,2118
    fn vertices(&self) -> &Set<V> { &self.V }vertices54,2216
    fn edges(&self) -> &Set<Pair<V, V>> { &self.E }edges55,2262
    fn sizeV(&self) -> N { self.V.size() }sizeV56,2314
    fn sizeE(&self) -> N { self.E.size() }sizeE57,2357
    fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor59,2401
    fn NG(&self, v: &V) -> Set<V> {NG64,2641
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices73,2915
    fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } else Incident82,3155
    fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree84,3267
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Debug for UnDirGraphSUnDirGraphStEph87,3325
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt88,3431
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for UnDirGrapUnDirGraphStEph93,3604
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt94,3712
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for UnDirGraphStEph<VUnDirGraphStEph99,3843
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for UnDirGraphStEph<Veq99,3843
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for UnDirGraphStEph<V> {}UnDirGraphStEph100,4023
    macro_rules! UnDirGraphLit {UnDirGraphLit103,4138
    fn _UnDirGraphLit_type_checks() {_UnDirGraphLit_type_checks121,5329
    pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise127,5597

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test29Algorithm_21_1.rs,427
fn points2d_tab_flat(n: N) -> ArrayStPerS<Pair<N, N>> {points2d_tab_flat11,442
fn test_points2d_n3_example() {test_points2d_n3_example22,950
fn test_points2d_n1_empty() {test_points2d_n1_empty29,1159
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values35,1265
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order43,1456
fn test_points2d_debug_shape() {test_points2d_debug_shape51,1785

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test12ArraySeqStEph.rs,271
pub mod TestArraySeqEph {TestArraySeqEph1,0
fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic10,272
fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter19,502
fn test_iterators_collect() {test_iterators_collect29,1001

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test18AVLTreeSeqStEph.rs,113
pub mod TestAVLTreeSeqEph {TestAVLTreeSeqEph1,0
fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic7,218

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test07LinkedListStEphChap18.rs,809
pub mod TestLinkedListStEphChap18 {TestLinkedListStEphChap181,0
fn test_construct_eph_from_vec() {test_construct_eph_from_vec10,337
fn test_eph_is_empty_and_singleton() {test_eph_is_empty_and_singleton16,474
fn test_eph_set_and_subseq_copy() {test_eph_set_and_subseq_copy24,729
fn test_iter_inorder_collect_eph_ch18() {test_iter_inorder_collect_eph_ch1833,960
fn test_tabulate_and_map_ch18() {test_tabulate_and_map_ch1840,1162
fn test_append_ch18() {test_append_ch1848,1520
fn test_filter_ch18() {test_filter_ch1857,1866
fn test_update_ch18() {test_update_ch1865,2180
fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1873,2443
fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1883,2980
fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch1895,3557

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test10ArraySeqStPerChap18.rs,948
pub mod TestArraySeqStPerChap18 {TestArraySeqStPerChap181,0
fn test_tabulate_fibonacci() {test_tabulate_fibonacci10,293
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }fib11,324
fn test_map_increment() {test_map_increment22,756
fn test_subseq() {test_subseq29,971
fn test_append() {test_append40,1265
fn test_sequence_literals_and_append() {test_sequence_literals_and_append48,1503
fn test_filter_even() {test_filter_even61,2075
fn test_flatten() {test_flatten72,2660
fn test_update_sequence() {test_update_sequence86,3368
fn test_inject_and_ninject() {test_inject_and_ninject96,3837
fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan115,5050
fn test_iterate_sum_basic() {test_iterate_sum_basic134,5950
fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum142,6193
fn test_collect_groups_by_key() {test_collect_groups_by_key154,6609

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test01Types.rs,677
pub mod TestTypes {TestTypes1,0
    fn test_boolean_eq_neq_and_ordering() {test_boolean_eq_neq_and_ordering5,67
    fn test_ordering_on_n_naturals() {test_ordering_on_n_naturals15,324
    fn test_cmp_on_b_returns_expected_ordering_variants() {test_cmp_on_b_returns_expected_ordering_variants24,571
    fn test_btree_set_orders_b_true_before_false() {test_btree_set_orders_b_true_before_false32,895
    fn test_n_aliases_usize_and_cmp_examples() {test_n_aliases_usize_and_cmp_examples42,1195
    fn test_debug_format_for_b_variants() {test_debug_format_for_b_variants57,1648
    fn test_display_format_for_b_variants() {test_display_format_for_b_variants63,1821

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test22RelationStEphChap5_2.rs,147
pub mod TestRelationStEphChap5_2 {TestRelationStEphChap5_21,0
fn test_relation_domain_range_and_mem() {test_relation_domain_range_and_mem8,221

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test15AVLTreeSeqStPer.rs,207
pub mod TestAVLTreeSeqPer {TestAVLTreeSeqPer1,0
fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate7,179
fn test_iterator_inorder_values() {test_iterator_inorder_values16,536

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test39Chapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test13ArraySeqStEphChap18.rs,1048
pub mod TestArraySeqStEphChap18 {TestArraySeqStEphChap183,93
fn test_tabulate_fibonacci() {test_tabulate_fibonacci10,314
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }fib11,345
fn test_map_increment() {test_map_increment22,820
fn test_subseq() {test_subseq29,1034
fn test_append() {test_append40,1472
fn test_sequence_literals_and_append() {test_sequence_literals_and_append48,1709
fn test_filter_even() {test_filter_even61,2322
fn test_flatten() {test_flatten71,2828
fn test_update_sequence() {test_update_sequence85,3473
fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins97,3928
fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins110,4634
fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan126,5318
fn test_iterate_sum_basic() {test_iterate_sum_basic145,6266
fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum153,6512
fn test_collect_groups_by_key() {test_collect_groups_by_key165,6931

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test17AVLTreeSeqStPerChap19.rs,367
pub mod TestAVLTreeSeqStPerChap19 {TestAVLTreeSeqStPerChap191,0
fn test_tabulate_and_map_ch19() {test_tabulate_and_map_ch198,233
fn test_select_and_append_ch19() {test_select_and_append_ch1916,577
fn test_deflate_and_filter_ch19() {test_deflate_and_filter_ch1928,1331
fn test_iter_inorder_after_pipeline_ch19() {test_iter_inorder_after_pipeline_ch1939,1937

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test35Exercsise_21_9.rs,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test38Problem21_1.rs,424
fn points2d(n: N) -> ArrayStPerS<Pair<N, N>> {points2d9,293
fn test_points2d_n3_example() {test_points2d_n3_example22,600
fn test_points2d_n1_empty() {test_points2d_n1_empty30,845
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values36,942
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order44,1124
fn test_points2d_debug_display_shapes() {test_points2d_debug_display_shapes52,1444

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test14ArraySeqStEphChap19.rs,1335
pub mod TestArraySeqStEphChap19 {TestArraySeqStEphChap193,93
fn test_empty() {test_empty8,290
fn test_singleton() {test_singleton14,406
fn test_map() {test_map20,540
fn test_append() {test_append27,756
fn test_append2() {test_append235,1041
fn test_deflate_true() {test_deflate_true43,1328
fn test_deflate_false() {test_deflate_false49,1516
fn test_filter_even_numbers() {test_filter_even_numbers55,1698
fn test_filter_none() {test_filter_none62,1981
fn test_update_in_bounds() {test_update_in_bounds69,2249
fn test_update_out_of_bounds() {test_update_out_of_bounds76,2457
fn test_isEmpty() {test_isEmpty83,2662
fn test_isSingleton() {test_isSingleton93,2993
fn test_iterate_sum() {test_iterate_sum103,3340
fn test_iterate_concat() {test_iterate_concat110,3546
fn test_map_empty() {test_map_empty124,3891
fn test_append_with_empty() {test_append_with_empty131,4089
fn test_append2_equivalence() {test_append2_equivalence141,4496
fn test_filter_empty_sequence() {test_filter_empty_sequence150,4834
fn test_select_boundary() {test_select_boundary157,5049
fn test_subseq_basic() {test_subseq_basic168,5527
fn test_reduce_sum_basic_ch19() {test_reduce_sum_basic_ch19175,5721
fn test_scan_sum_basic_ch19() {test_scan_sum_basic_ch19189,6248
fn test_flatten_ch19() {test_flatten_ch19200,6607

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test19AVLTreeSeqStEphChap18.rs,276
pub mod TestAVLTreeSeqStEphChap18 {TestAVLTreeSeqStEphChap183,79
fn test_tabulate_inorder() {test_tabulate_inorder11,355
fn test_map_increment() {test_map_increment17,525
fn test_append_union() {test_append_union25,836
fn test_filter_even() {test_filter_even35,1236

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test41ArraySeqMtEph.rs,156
fn test_arrayseq_mteph_basic_ops() {test_arrayseq_mteph_basic_ops4,25
fn test_arrayseq_mteph_append_and_map() {test_arrayseq_mteph_append_and_map17,458

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test39Chapter36St.rs,499
trait ToVec<T: StT> { fn to_vec(&self) -> Vec<T>; }ToVec4,74
trait ToVec<T: StT> { fn to_vec(&self) -> Vec<T>; }to_vec4,74
impl<T: StT> ToVec<T> for ArraySeqStEphS<T> {ArraySeqStEphS5,126
    fn to_vec(&self) -> Vec<T> {to_vec6,172
fn quick_sort_variants_produce_sorted_output() {quick_sort_variants_produce_sorted_output12,288
fn quick_sort_handles_edge_cases() {quick_sort_handles_edge_cases27,785
fn pivot_strategies_match_expectations() {pivot_strategies_match_expectations46,1386

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test26ArraySeqMtPer.rs,849
pub mod Test26ArraySeqMtPer {Test26ArraySeqMtPer3,93
fn test_new_and_set() {test_new_and_set8,241
fn test_length_and_nth_basic() {test_length_and_nth_basic22,637
fn test_empty() {test_empty30,818
fn test_sequence_basic() {test_sequence_basic37,978
fn test_singleton() {test_singleton50,1453
fn test_from_vec() {test_from_vec58,1643
fn test_subseq_copy() {test_subseq_copy67,1837
fn test_subseq_view() {test_subseq_view77,2087
fn test_iterators() {test_iterators87,2319
fn test_set_out_of_bounds() {test_set_out_of_bounds100,2640
fn test_macro_literals() {test_macro_literals107,2794
fn test_equality_and_debug() {test_equality_and_debug127,3341
fn test_display_format() {test_display_format143,3734
fn test_string_sequences() {test_string_sequences152,3969
fn test_boolean_sequences() {test_boolean_sequences160,4158

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test08LinkedListStEphChap19.rs,520
pub mod TestLinkedListStEphChap19 {TestLinkedListStEphChap191,0
fn test_eph_set_and_nth() {test_eph_set_and_nth8,233
fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug15,388
fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1925,705
fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1932,907
fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1942,1532

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test21SetStEphChap5_1.rs,550
pub mod TestSetStEphChap5_1 {TestSetStEphChap5_11,0
fn macro_typecheck_exercise() {macro_typecheck_exercise8,169
    let _empty: Set<&'static str> = SetLit![];str9,201
fn test_cartesian_product_example_5_1() {test_cartesian_product_example_5_115,331
fn test_partition_example_5_2_true() {test_partition_example_5_2_true33,967
fn test_partition_example_5_2_false_due_to_overlap() {test_partition_example_5_2_false_due_to_overlap42,1232
fn test_partition_false_due_to_missing_element() {test_partition_false_due_to_missing_element51,1545

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test30Algorithm_21_2.rs,527
fn points3d_tab_flat(n: N) -> ArrayStPerS<Pair<N, Pair<N, N>>> {points3d_tab_flat12,521
fn test_points3d_tab_flat_n0_empty() {test_points3d_tab_flat_n0_empty33,1718
fn test_points3d_tab_flat_n1_single() {test_points3d_tab_flat_n1_single39,1833
fn test_points3d_tab_flat_n2_values_and_order() {test_points3d_tab_flat_n2_values_and_order46,1999
fn test_points3d_tab_flat_iterator_order() {test_points3d_tab_flat_iterator_order59,2372
fn test_points3d_tab_flat_debug_shape() {test_points3d_tab_flat_debug_shape66,2715

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test34Exercise_21_8_and_Algorithm_21_5.rs,393
fn is_divisible(n: N, i: N) -> B { if n % i == 0 { B::True } else { B::False } }is_divisible8,272
fn is_prime(n: N) -> B {is_prime13,515
fn primes_bf(n: N) -> ArrayStPerS<N> {primes_bf25,1106
fn test_is_prime_small_values() {test_is_prime_small_values33,1452
fn test_primes_bf_small() {test_primes_bf_small43,1729
fn test_primes_bf_debug_shape() {test_primes_bf_debug_shape50,1881

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_2.txt,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test11ArraySeqStPerChap19.rs,342
pub mod TestArraySeqPerChap19 {TestArraySeqPerChap191,0
fn test_map_and_select_and_append() {test_map_and_select_and_append10,261
fn test_deflate_and_filter() {test_deflate_and_filter20,744
fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten29,1220
fn test_inject_and_parallel() {test_inject_and_parallel48,2160

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test02MathSeq.rs,1282
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test03LinkedListStPer.rs,508
pub mod TestLinkedListPer {TestLinkedListPer1,0
fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates7,163
fn test_new_and_nth_set() {test_new_and_nth_set16,418
fn test_subseq_copy() {test_subseq_copy28,741
fn test_from_vec_and_debug_format() {test_from_vec_and_debug_format37,944
fn test_iter_inorder_collect() {test_iter_inorder_collect44,1112
fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics52,1306
fn test_display_impl() {test_display_impl58,1412

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test33Exercise_21_7.rs,431
fn is_even(x: &N) -> B { if *x % 2 == 0 { B::True } else { B::False } }is_even9,319
fn is_vowel(c: &char) -> B {is_vowel10,391
fn pair_even_with_vowels(a: &ArrayStPerS<N>, b: &ArrayStPerS<char>) -> ArrayStPerS<Pair<N, char>pair_even_with_vowels19,658
fn test_pair_even_with_vowels_basic() {test_pair_even_with_vowels_basic34,1343
fn test_pair_even_with_vowels_debug_shape() {test_pair_even_with_vowels_debug_shape43,1636

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test37Problem_21_4.rs,1362
fn cartesian_loops(a: &ArrayStPerS<N>, b: &ArrayStPerS<&'static str>) -> ArrayStPerS<Pair<N, &'scartesian_loops11,379
    let mut v: Vec<Pair<N, &'static str>> = Vec::with_capacity(a.length() * b.length());str12,489
fn cartesian_tab_flat(a: &ArrayStPerS<N>, b: &ArrayStPerS<&'static str>) -> ArrayStPerS<Pair<N, cartesian_tab_flat23,869
    let nested: ArrayStPerS<ArrayStPerS<Pair<N, &'static str>>> =str24,982
        <ArrayStPerS<ArrayStPerS<Pair<N, &'static str>>> as ArraySeqStPerChap19Trait<ArrayStPerSstr25,1048
        <ArrayStPerS<ArrayStPerS<Pair<N, &'static str>>> as ArraySeqStPerChap19Trait<ArrayStPerSstr25,1048
            |i| <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerChap19Trait<Pair<N, &'staticstr26,1181
            |i| <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerChap19Trait<Pair<N, &'staticstr26,1181
    <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerChap18Trait<Pair<N, &'static str>>>::flastr30,1392
    <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerChap18Trait<Pair<N, &'static str>>>::flastr30,1392
fn test_cartesian_loops_basic() {test_cartesian_loops_basic34,1513
fn test_cartesian_tab_flat_basic() {test_cartesian_tab_flat_basic43,1834
fn test_cartesian_iterator_order() {test_cartesian_iterator_order52,2161
fn test_cartesian_debug_shape() {test_cartesian_debug_shape61,2463

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test20AVLTreeSeqStEphChap19.rs,248
pub mod TestAVLTreeSeqStEphChap19 {TestAVLTreeSeqStEphChap193,80
fn test_tabulate_and_map() {test_tabulate_and_map11,342
fn test_select_and_append() {test_select_and_append19,646
fn test_deflate_and_filter() {test_deflate_and_filter38,1458

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test27ArraySeqMtPerChap18.rs,562
pub mod Test27ArraySeqMtPerChap18 {Test27ArraySeqMtPerChap183,61
fn test_tabulate_basic() {test_tabulate_basic10,284
fn test_tabulate_fibonacci() {test_tabulate_fibonacci24,583
    fn fib(n: N) -> N {fib25,614
fn test_tabulate_empty() {test_tabulate_empty56,1328
fn test_tabulate_single() {test_tabulate_single63,1512
fn test_tabulate_string() {test_tabulate_string70,1690
fn test_tabulate_boolean() {test_tabulate_boolean88,2178
fn test_tabulate_squares() {test_tabulate_squares106,2657
fn test_tabulate_large() {test_tabulate_large121,3019

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test05LinkedListStPerChap19.rs,407
pub mod TestLinkedListPerChap19 {TestLinkedListPerChap191,0
fn test_select() {test_select8,231
fn test_append_variants() {test_append_variants21,697
fn test_deflate() {test_deflate31,1086
fn test_map() {test_map39,1466
fn test_iterate_and_reduce() {test_iterate_and_reduce46,1666
fn test_scan() {test_scan55,1999
fn test_flatten() {test_flatten63,2257
fn test_inject() {test_inject73,2528

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test06LinkedListStEph.rs,517
pub mod TestLinkedListEph {TestLinkedListEph2,56
fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates9,220
fn test_new_and_nth_set() {test_new_and_nth_set18,514
fn test_subseq_copy() {test_subseq_copy27,717
fn test_linkedlisteph_basic() {test_linkedlisteph_basic36,920
fn test_debug_format_for_eph() {test_debug_format_for_eph45,1128
fn test_display_format_for_eph() {test_display_format_for_eph51,1260
fn test_iter_inorder_collect_eph() {test_iter_inorder_collect_eph57,1392

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test04LinkedListStPerChap18.rs,531
pub mod TestLinkedListStPerChap18 {TestLinkedListStPerChap181,0
fn test_tabulate() {test_tabulate8,233
fn test_map() {test_map15,418
fn test_filter() {test_filter23,685
fn test_append() {test_append30,983
fn test_update() {test_update38,1254
fn test_inject() {test_inject45,1477
fn test_ninject() {test_ninject53,1758
fn test_iterate() {test_iterate61,2043
fn test_reduce() {test_reduce68,2247
fn test_scan() {test_scan75,2446
fn test_flatten() {test_flatten83,2716
fn test_collect() {test_collect94,3026

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test09ArraySeqStPer.rs,2188
pub mod TestArraySeqStPer {TestArraySeqStPer3,93
fn test_new_and_set() {test_new_and_set8,239
fn test_length_and_nth_basic() {test_length_and_nth_basic22,635
fn test_empty() {test_empty30,816
fn test_sequence_basic() {test_sequence_basic37,976
fn test_singleton() {test_singleton50,1616
fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton58,1793
fn test_from_vec() {test_from_vec73,2211
fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers88,2804
fn test_sequence_equality_strings() {test_sequence_equality_strings113,3614
fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference138,4518
    struct PartialComparable { value: i32 } // Use i32 instead of f64 so Eq can be implementedPartialComparable140,4601
    struct PartialComparable { value: i32 } // Use i32 instead of f64 so Eq can be implementedvalue140,4601
    impl std::fmt::Display for PartialComparable {PartialComparable142,4701
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt143,4752
    struct TotalComparable { value: N }TotalComparable155,5336
    struct TotalComparable { value: N }value155,5336
    impl std::fmt::Display for TotalComparable {TotalComparable157,5381
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt158,5430
fn test_ordering_numbers_basic() {test_ordering_numbers_basic173,6023
fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal182,6238
fn test_ordering_strings_basic() {test_ordering_strings_basic188,6346
fn test_strings_equal_is_equal() {test_strings_equal_is_equal197,6559
fn test_nth_on_empty_panics() {test_nth_on_empty_panics204,6684
fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics211,6807
fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap218,6920
fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes224,7063
fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic233,7268
fn test_new_set_persistent() {test_new_set_persistent242,7575
fn test_iterator_collects_in_order() {test_iterator_collects_in_order252,7810

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test24DirGraphStEphChap6_1.rs,139
pub mod TestDirGraphStEphChap6_1 {TestDirGraphStEphChap6_11,0
fn test_digraph_vertices_and_arcs() {test_digraph_vertices_and_arcs8,205

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test25UnDirGraphStEphChap6_1.rs,149
pub mod TestUnDirGraphStEphChap6_1 {TestUnDirGraphStEphChap6_11,0
fn test_undigraph_vertices_and_edges() {test_undigraph_vertices_and_edges8,211

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test23MappingStEphChap5_5.rs,529
pub mod Test23MappingStEphChap5_5 {Test23MappingStEphChap5_53,55
fn test_empty_mapping() {test_empty_mapping11,319
fn test_from_vec_basic() {test_from_vec_basic19,508
fn test_from_vec_duplicate_keys() {test_from_vec_duplicate_keys31,959
fn test_from_relation() {test_from_relation42,1465
fn test_domain_and_range() {test_domain_and_range56,2178
fn test_iter() {test_iter75,2825
fn test_mem_comprehensive() {test_mem_comprehensive88,3261
fn test_empty_mapping_operations() {test_empty_mapping_operations107,3870

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test40Chapter36Mt.rs,325
fn to_vec<T: StT>(a: &ArraySeqMtEphS<T>) -> Vec<T> {to_vec4,74
fn quick_sort_mt_variants_produce_sorted_output() {quick_sort_mt_variants_produce_sorted_output9,193
fn quick_sort_mt_edge_cases() {quick_sort_mt_edge_cases27,696
fn pivot_mt_strategies_match_expectations() {pivot_mt_strategies_match_expectations46,1281

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test32Exercise_21_5_and_21_6.rs,380
fn all_contiguous_subseqs<T: StT>(a: &ArrayStPerS<T>) -> ArrayStPerS<ArrayStPerS<T>> {all_contiguous_subseqs12,458
fn test_all_contiguous_subseqs_n0() {test_all_contiguous_subseqs_n029,1168
fn test_all_contiguous_subseqs_n3_values() {test_all_contiguous_subseqs_n3_values36,1338
fn test_all_contiguous_subseqs_debug_shape() {test_all_contiguous_subseqs_debug_shape49,1700

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test36Problem_21_3.rs,488
fn points3d_loops(n: N) -> ArrayStPerS<Pair<N, Pair<N, N>>> {points3d_loops9,348
fn test_points3d_loops_n0_empty() {test_points3d_loops_n0_empty24,738
fn test_points3d_loops_n1_single() {test_points3d_loops_n1_single30,847
fn test_points3d_loops_n2_values_and_order() {test_points3d_loops_n2_values_and_order37,1007
fn test_points3d_loops_iterator_order() {test_points3d_loops_iterator_order50,1374
fn test_points3d_loops_debug_shape() {test_points3d_loops_debug_shape57,1711

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test28ArraySeqMtPerChap19.rs,557
pub mod Test28ArraySeqMtPerChap19 {Test28ArraySeqMtPerChap193,61
fn test_inject_basic() {test_inject_basic11,306
fn test_inject_conflicting_updates() {test_inject_conflicting_updates27,857
fn test_inject_out_of_bounds() {test_inject_out_of_bounds43,1463
fn test_inject_empty_changes() {test_inject_empty_changes55,1934
fn test_inject_empty_values() {test_inject_empty_values67,2320
fn test_atomic_write_migrated_from_st_test() {test_atomic_write_migrated_from_st_test81,2918
fn test_inject_string_values() {test_inject_string_values106,4015

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test31Algorithm_21_6.rs,261
fn prime_sieve(n: N) -> ArrayStPerS<N> {prime_sieve12,500
fn test_prime_sieve_small() {test_prime_sieve_small46,2297
fn test_prime_sieve_n2_empty() {test_prime_sieve_n2_empty53,2453
fn test_prime_sieve_debug_shape() {test_prime_sieve_debug_shape59,2556

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test16AVLTreeSeqStPerChap18.rs,276
pub mod TestAVLTreeSeqStPerChap18 {TestAVLTreeSeqStPerChap183,49
fn test_tabulate_inorder() {test_tabulate_inorder12,352
fn test_map_increment() {test_map_increment18,565
fn test_append_union() {test_append_union25,897
fn test_filter_even() {test_filter_even33,1343

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStEphChap19.rs,68
fn bench_avl_eph_ch19(c: &mut Criterion) {bench_avl_eph_ch196,191

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchUnDirGraphStEphChap6_1.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build7,255

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36Mt.rs,129
fn gen_data(n: usize) -> ArraySeqMtEphS<i32> {gen_data4,89
fn bench_quicksort_mt(c: &mut Criterion) {bench_quicksort_mt8,199

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchMathSeq.rs,72
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics8,213

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStEphChap19.rs,86
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch198,285

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPer.rs,80
fn bench_sll_persistent_ops(c: &mut Criterion) {bench_sll_persistent_ops8,229

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStEph.rs,455
struct LinearCongruentialGenerator32 { state: u32 }LinearCongruentialGenerator3211,377
struct LinearCongruentialGenerator32 { state: u32 }state11,377
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3213,430
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new14,467
    fn next_N(&mut self) -> N {next_N16,586
fn bench_build_random_s(c: &mut Criterion) {bench_build_random_s25,823

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEphChap19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch197,232

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch187,245

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStPer.rs,477
struct LinearCongruentialGenerator32 { state: u32 }LinearCongruentialGenerator3211,425
struct LinearCongruentialGenerator32 { state: u32 }state11,425
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3213,478
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new14,515
    fn next_N(&mut self) -> N {next_N16,634
fn bench_build_random_s_persistent(c: &mut Criterion) {bench_build_random_s_persistent25,871

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEphChap18.rs,66
fn bench_ll_eph_ch18(c: &mut Criterion) {bench_ll_eph_ch187,232

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStEph.rs,58
fn bench_avl_eph(c: &mut Criterion) {bench_avl_eph6,191

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqMtPerChap18.rs,90
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch187,245

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchDirGraphStEphChap6_1.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build7,251

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchRelationStEphChap5_2.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range8,272

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36St.rs,129
fn gen_data(n: usize) -> ArraySeqStEphS<i32> {gen_data4,89
fn bench_quicksort_st(c: &mut Criterion) {bench_quicksort_st8,199

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStEphChap18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map8,285

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchMappingStEphChap5_5.rs,70
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build8,309

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStPerChap19.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch197,245

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStPerChap19.rs,95
fn bench_build_and_read_persistent(c: &mut Criterion) {bench_build_and_read_persistent10,335

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStPer.rs,80
fn bench_build_and_contains(c: &mut Criterion) {bench_build_and_contains9,270

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqMtPerChap19.rs,90
fn bench_tabulate_map_mtper_ch19(c: &mut Criterion) {bench_tabulate_map_mtper_ch197,245

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPerChap19.rs,66
fn bench_ll_per_ch19(c: &mut Criterion) {bench_ll_per_ch198,297

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStPerChap18.rs,68
fn bench_avl_per_ch18(c: &mut Criterion) {bench_avl_per_ch187,232

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStEphChap18.rs,68
fn bench_avl_eph_ch18(c: &mut Criterion) {bench_avl_eph_ch186,191

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPerChap18.rs,66
fn bench_ll_per_ch18(c: &mut Criterion) {bench_ll_per_ch187,232

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqMtPer.rs,505
struct LinearCongruentialGenerator32 { state: u32 }LinearCongruentialGenerator3211,425
struct LinearCongruentialGenerator32 { state: u32 }state11,425
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3213,478
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new14,515
    fn next_N(&mut self) -> N {next_N16,634
fn bench_build_random_s_multithreaded_persistent(c: &mut Criterion) {bench_build_random_s_multithreaded_persistent25,871

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEph.rs,56
fn bench_ll_eph(c: &mut Criterion) {bench_ll_eph6,191
