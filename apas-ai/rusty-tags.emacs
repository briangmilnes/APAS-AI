
/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap38/BSTParaStEph.rs,3103
pub mod BSTParaStEph {BSTParaStEph3,70
    pub enum Exposed<T: StT + Ord> {Exposed11,269
        Leaf,Leaf13,325
        Node(ParamBST<T>, T, ParamBST<T>),Node14,339
    struct NodeInner<T: StT + Ord> {NodeInner18,417
        key: T,key19,454
        size: N,size20,470
        left: ParamBST<T>,left21,487
        right: ParamBST<T>,right22,514
    pub struct ParamBST<T: StT + Ord> {ParamBST26,577
        root: Rc<RefCell<Option<Box<NodeInner<T>>>>>,root27,617
    pub trait ParamBSTTrait<T: StT + Ord>: Sized {ParamBSTTrait30,678
        fn new() -> Self;new31,729
        fn expose(&self) -> Exposed<T>;expose32,755
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid33,795
        fn size(&self) -> N;size34,845
        fn is_empty(&self) -> B;is_empty35,874
        fn insert(&self, key: T);insert36,907
        fn delete(&self, key: &T);delete37,941
        fn find(&self, key: &T) -> Option<T>;find38,976
        fn split(&self, key: &T) -> (Self, B, Self);split39,1022
        fn join_pair(&self, other: Self) -> Self;join_pair40,1075
        fn union(&self, other: &Self) -> Self;union41,1125
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order42,1172
    impl<T: StT + Ord> ParamBST<T> {ParamBST45,1228
        fn expose_internal(&self) -> Exposed<T> {expose_internal46,1265
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid54,1557
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner66,2009
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m85,2994
        fn min_key(tree: &Self) -> Option<T> {min_key87,3110
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner97,3454
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner108,3924
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order120,4431
    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {ParamBST132,4833
        fn new() -> Self {new133,4891
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose139,5017
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid141,5084
        fn size(&self) -> N { self.root.borrow().as_ref().map_or(0, |node| node.size) }size143,5166
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty145,5255
        fn insert(&self, key: T) {insert147,5342
        fn delete(&self, key: &T) {delete154,5632
        fn find(&self, key: &T) -> Option<T> {find161,5924
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split172,6421
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair174,6511
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union176,6612
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order178,6698
    macro_rules! ParamBSTLit {ParamBSTLit186,6942
    fn _ParamBSTLit_type_checks() {_ParamBSTLit_type_checks199,7465

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap38/BSTParaMtEph.rs,4169
pub mod BSTParaMtEph {BSTParaMtEph3,69
    pub enum Exposed<T: StTInMtT + Ord> {Exposed10,237
        Leaf,Leaf11,279
        Node(ParamBST<T>, T, ParamBST<T>),Node12,293
    struct NodeInner<T: StTInMtT + Ord> {NodeInner16,364
        key: T,key17,406
        size: N,size18,422
        left: ParamBST<T>,left19,439
        right: ParamBST<T>,right20,466
    pub struct ParamBST<T: StTInMtT + Ord> {ParamBST24,529
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root25,574
    pub trait ParamBSTTrait<T: StTInMtT + Ord + 'static>: Sized {ParamBSTTrait28,635
        fn new() -> Self;new31,792
        fn expose(&self) -> Exposed<T>;expose34,909
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid37,1040
        fn size(&self) -> N;size40,1181
        fn is_empty(&self) -> B;is_empty43,1301
        fn insert(&self, key: T);insert46,1445
        fn delete(&self, key: &T);delete49,1590
        fn find(&self, key: &T) -> Option<T>;find52,1736
        fn split(&self, key: &T) -> (Self, B, Self);split55,1893
        fn join_pair(&self, other: Self) -> Self;join_pair58,2105
        fn union(&self, other: &Self) -> Self;union61,2280
        fn intersect(&self, other: &Self) -> Self;intersect64,2452
        fn difference(&self, other: &Self) -> Self;difference67,2628
        fn filter<F>(&self, predicate: F) -> Selffilter70,2785
        fn reduce<F>(&self, op: F, base: T) -> Treduce75,3009
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order80,3225
    impl<T: StTInMtT + Ord + 'static> ParamBST<T> {ParamBST83,3281
        fn expose_internal(&self) -> Exposed<T> {expose_internal86,3424
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid96,3814
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner110,4377
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m131,5453
        fn min_key(tree: &Self) -> Option<T> {min_key135,5680
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner147,6195
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner160,6790
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner176,7507
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner196,8470
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner217,9446
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel241,10528
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner251,10890
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel275,11969
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order285,12308
    impl<T: StTInMtT + Ord + 'static> ParamBSTTrait<T> for ParamBST<T> {ParamBST297,12710
        fn new() -> Self {new300,12874
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose308,13091
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid312,13249
        fn size(&self) -> N {size316,13422
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty323,13660
        fn insert(&self, key: T) {insert327,13858
        fn delete(&self, key: &T) {delete337,14301
        fn find(&self, key: &T) -> Option<T> {find347,14746
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split360,15354
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair364,15603
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union368,15829
        fn intersect(&self, other: &Self) -> Self { ParamBST::intersect_inner(self, other) }intersect372,16040
        fn difference(&self, other: &Self) -> Self { ParamBST::difference_inner(self, other) }difference376,16259
        fn filter<F>(&self, predicate: F) -> Selffilter380,16460
        fn reduce<F>(&self, op: F, base: T) -> Treduce389,16760
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order398,17051

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEphSlice.rs,3569
pub mod ArraySeqMtEphSlice {ArraySeqMtEphSlice8,395
    struct Inner<T: StT> {Inner16,591
        data: Mutex<Box<[T]>>,data17,618
    impl<T: StT> Inner<T> {Inner20,656
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }new21,684
        fn len(&self) -> N {len23,761
    pub struct ArraySeqMtEphSliceS<T: StT> {ArraySeqMtEphSliceS31,970
        inner: Arc<Inner<T>>,inner32,1015
        range: Range<N>,range33,1045
    pub trait ArraySeqMtEphSliceTrait<T: StT> {ArraySeqMtEphSliceTrait37,1141
        fn new(length: N, init_value: T) -> Self;new38,1189
        fn length(&self) -> N;length39,1239
        fn nth_cloned(&self, index: N) -> T;nth_cloned40,1270
        fn empty() -> Self;empty41,1315
        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;update42,1343
        fn singleton(item: T) -> Self;singleton43,1427
        fn isEmpty(&self) -> B;isEmpty44,1466
        fn isSingleton(&self) -> B;isSingleton45,1498
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy46,1534
        fn slice(&self, start: N, length: N) -> Self;slice47,1594
    impl<T: StT> ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS50,1655
        pub fn from_box(data: Box<[T]>) -> Self {from_box52,1758
        pub fn from_vec(data: Vec<T>) -> Self { Self::from_box(data.into_boxed_slice()) }from_vec61,2060
        pub fn to_vec(&self) -> Vec<T> {to_vec64,2232
        pub fn with_exclusive<F, R>(&self, f: F) -> Rwith_exclusive70,2496
        fn len(&self) -> N { self.range.end - self.range.start }len80,2802
        fn clamp_subrange(&self, start: N, length: N) -> Range<N> {clamp_subrange82,2868
    impl<T: StT> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS91,3228
        fn new(length: N, init_value: T) -> Self {new92,3301
        fn length(&self) -> N { self.len() }length97,3466
        fn nth_cloned(&self, index: N) -> T {nth_cloned99,3512
        fn empty() -> Self { ArraySeqMtEphSliceS::from_vec(Vec::new()) }empty105,3705
        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {update107,3779
        fn singleton(item: T) -> Self { ArraySeqMtEphSliceS::from_vec(vec![item]) }singleton119,4178
        fn isEmpty(&self) -> B { if self.len() == 0 { B::True } else { B::False } }isEmpty121,4263
        fn isSingleton(&self) -> B { if self.len() == 1 { B::True } else { B::False } }isSingleton123,4348
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy125,4437
        fn slice(&self, start: N, length: N) -> Self {slice132,4756
    impl<T: StT> Clone for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS141,5010
        fn clone(&self) -> Self {clone142,5062
    impl<T: StT> PartialEq for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS150,5252
        fn eq(&self, other: &Self) -> bool {eq151,5308
    impl<T: StT> Eq for ArraySeqMtEphSliceS<T> {}ArraySeqMtEphSliceS164,5726
    impl<T: StT> Debug for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS166,5777
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt167,5829
    impl<T: StT> Display for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS175,6096
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt176,6150
    fn repeat_vec<T: StT>(length: N, init: T) -> Vec<T> {repeat_vec191,6617
    macro_rules! ArraySeqMtEphSliceSLit {ArraySeqMtEphSliceSLit200,6842
    fn _ArraySeqMtEphSliceSLit_type_checks() {_ArraySeqMtEphSliceSLit_type_checks207,7305

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStEph.rs,3446
pub mod ArraySeqStEph {ArraySeqStEph3,51
    pub type ArraySeqStEphS<T> = ArrayS<T>;ArraySeqStEphS8,172
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait10,217
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>new11,260
        fn empty() -> ArraySeqStEphS<T>;empty14,358
        fn singleton(item: T) -> ArraySeqStEphS<T>;singleton15,399
        fn length(&self) -> N;length16,451
        fn nth(&self, index: N) -> &T;nth17,482
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T>;subseq_copy18,521
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate22,687
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map25,851
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T>;select28,1032
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append31,1228
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append234,1422
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T>;deflate37,1601
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter40,1766
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate41,1853
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce42,1940
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan43,2020
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten44,2119
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS47,2206
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>new48,2269
        fn empty() -> ArraySeqStEphS<T> { ArraySeq::empty() }empty55,2434
        fn singleton(item: T) -> ArraySeqStEphS<T> { ArraySeq::singleton(item) }singleton57,2497
        fn length(&self) -> N { ArraySeq::length(self) }length59,2579
        fn nth(&self, index: N) -> &T { ArraySeq::nth(self, index) }nth61,2637
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T> { ArraySeq::subseq_copy(subseq_copy63,2707
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> { ArraySeq::tabulate(f, n) }tabulate65,2827
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> { ArraySmap67,2924
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T> {select69,3037
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> { ArraySeq:append83,3517
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> { ArraySeqappend285,3630
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T> {deflate87,3744
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> { ArraySeqfilter95,3974
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { ArraySeqiterate97,4091
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T { ArraySeq::reducreduce99,4209
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan101,4320
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> { ArraySeq::flattflatten105,4468

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtPer.rs,4911
pub mod ArraySeqMtPer {ArraySeqMtPer3,132
    pub trait ArraySeqMtPerTrait<T: MtT> {ArraySeqMtPerTrait9,335
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;new11,409
        fn empty() -> ArraySeqMtPerS<T>;empty12,472
        fn singleton(item: T) -> ArraySeqMtPerS<T>;singleton13,513
        fn length(&self) -> N;length14,565
        fn nth(&self, index: N) -> &T;nth15,596
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;subseq_copy16,635
        fn update(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str>;update17,708
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T>;tabulate19,797
        fn map<W: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W) -> ArraySeqMtPerS<W>;map20,865
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append21,954
        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtPerS<T>;filter22,1040
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T>;update23,1127
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject24,1211
        fn iterate<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate25,1313
        fn iteratePrefixes<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes26,1400
        fn reduce(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce27,1516
        fn scan(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtPerS<T>, Tscan28,1596
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;flatten29,1695
        fn collect(collect30,1776
        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMinject36,1978
        fn atomicWrite(atomicWrite37,2084
        fn inject_parallel2(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) ->inject_parallel242,2270
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins43,2386
        fn ninject_parallel2(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -ninject_parallel248,2597
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins49,2714
    impl<T: MtT> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {ArraySeqMtPerS56,2933
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerTraitChap18::new(lenew57,2996
        fn empty() -> ArraySeqMtPerS<T> { ArraySeqMtPerTraitChap18::empty() }empty59,3113
        fn singleton(item: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerTraitChap18::singleton(item) }singleton61,3192
        fn length(&self) -> N { ArraySeqMtPerTraitChap18::length(self) }length63,3290
        fn nth(&self, index: N) -> &T { ArraySeqMtPerTraitChap18::nth(self, index) }nth65,3364
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> {subseq_copy67,3450
        fn update(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str> {update71,3606
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T> { ArraySeqMtPerTraitChap18::ttabulate75,3770
        fn map<W: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W) -> ArraySeqMtPerS<W> {map77,3883
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append81,4032
        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtPerS<T> {filter85,4181
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T> {update89,4334
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject93,4487
        fn iterate<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate97,4659
        fn iteratePrefixes<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes101,4813
        fn reduce(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce105,5004
        fn scan(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtPerS<T>, Tscan109,5151
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {flatten113,5315
        fn collect(collect117,5458
        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMinject124,5684
        fn atomicWrite(atomicWrite128,5864
        fn inject_parallel2(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) ->inject_parallel2136,6163
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins140,6353
        fn ninject_parallel2(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -ninject_parallel2161,7236
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins165,7428

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEph.rs,3484
pub mod ArraySeqMtEph {ArraySeqMtEph3,67
    pub struct ArraySeqMtEphS<T: StT> {ArraySeqMtEphS10,262
        data: Mutex<Box<[T]>>,data11,302
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait14,340
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>new15,383
        fn empty() -> ArraySeqMtEphS<T>;empty18,481
        fn singleton(item: T) -> ArraySeqMtEphS<T>;singleton19,522
        fn length(&self) -> N;length20,574
        fn nth_cloned(&self, index: N) -> T;nth_cloned21,605
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;subseq_copy22,650
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate24,724
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map25,792
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T>;select26,881
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append27,969
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append228,1055
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T>;deflate29,1142
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter30,1211
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate31,1298
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce32,1385
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtEphS<T>, Tscan33,1465
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;flatten34,1564
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS37,1651
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>new38,1714
        fn empty() -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::empty() }empty45,1889
        fn singleton(item: T) -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::singleton(item) }singleton47,1962
        fn length(&self) -> N { ArraySeqMtEphTrait::length(self) }length49,2054
        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphTrait::nth_cloned(self, index) }nth_cloned51,2122
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {subseq_copy53,2215
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::tabulattabulate57,2365
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map59,2472
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T> {select63,2615
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append77,3133
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append281,3276
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T> {deflate85,3420
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter93,3670
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, mut x: A) -> A {iterate97,3817
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> T {reduce104,4070
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> (ArraySeqMtEphS<Tscan111,4319
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> { ArraySeqMtEphTrflatten125,4845

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStPer.rs,3911
pub mod ArraySeqStPer {ArraySeqStPer3,46
    pub type ArraySeqStPerS<T> = ArrayS<T>;ArraySeqStPerS7,166
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait9,211
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>new10,254
        fn empty() -> ArraySeqStPerS<T>;empty13,352
        fn singleton(item: T) -> ArraySeqStPerS<T>;singleton14,393
        fn length(&self) -> N;length15,445
        fn nth(&self, index: N) -> &T;nth16,476
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T>;subseq_copy17,515
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStPerS<T>;tabulate20,675
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U>;map22,823
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>select24,953
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append26,1104
        fn append2(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append228,1243
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStPerS<T>;deflate30,1371
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T>;filter32,1527
        fn iterate<A: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate33,1614
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce34,1701
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, Tscan35,1781
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;flatten36,1880
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject37,1960
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject38,2061
    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {ArraySeqStPerS41,2170
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>new42,2233
        fn empty() -> ArraySeqStPerS<T> { ArraySeq::empty() }empty49,2398
        fn singleton(item: T) -> ArraySeqStPerS<T> { ArraySeq::singleton(item) }singleton51,2461
        fn length(&self) -> N { ArraySeq::length(self) }length53,2543
        fn nth(&self, index: N) -> &T { ArraySeq::nth(self, index) }nth55,2601
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T> { ArraySeq::subseq_copy(subseq_copy57,2671
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStPerS<T> { ArraySeq::tabulate(f, n) }tabulate59,2791
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U> { ArraySmap61,2888
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>select63,3001
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> { ArraySeq:append77,3463
        fn append2(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> { ArraySeqappend279,3576
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStPerS<T> {deflate81,3690
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T> { ArraySeqfilter89,3920
        fn iterate<A: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { ArraySeqiterate91,4037
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T { ArraySeq::reducreduce93,4155
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, Tscan95,4266
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> { ArraySeq::flattflatten99,4414
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject101,4519
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject105,4673

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphFloat.rs,1258
pub mod WeightedUnDirGraphMtEphFloat {WeightedUnDirGraphMtEphFloat3,107
    pub type WeightedUnDirGraphMtEphFloat<V> = LabUnDirGraphMtEph<V, OrderedF64>;WeightedUnDirGraphMtEphFloat12,463
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphFloat<V> {WeightedUnDirGraphMtEphFloat15,652
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges17,774
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge32,1337
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight37,1538
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges42,1745
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted51,2106
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight64,2663
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree72,2950
    macro_rules! WeightedUnDirGraphMtEphFloatLit {WeightedUnDirGraphMtEphFloatLit76,3058
    pub fn __weighted_undir_graph_mt_float_macro_typecheck_exercise() {__weighted_undir_graph_mt_float_macro_typecheck_exercise88,3683

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphInt.rs,1316
pub mod WeightedUnDirGraphStEphInt {WeightedUnDirGraphStEphInt3,101
    pub type WeightedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;WeightedUnDirGraphStEphInt12,432
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphInt<V> {WeightedUnDirGraphStEphInt15,588
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,702
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(vadd_weighted_edge32,1251
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, vget_edge_weight35,1418
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges38,1591
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted47,1932
        pub fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum()total_weight60,2469
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree63,2635
        pub fn is_connected(&self) -> bool {is_connected66,2802
    macro_rules! WeightedUnDirGraphStEphIntLit {WeightedUnDirGraphStEphIntLit95,3814
    pub fn __weighted_undir_graph_st_int_macro_typecheck_exercise() {__weighted_undir_graph_st_int_macro_typecheck_exercise107,4410

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphStEph.rs,2676
pub mod UnDirGraphStEph {UnDirGraphStEph3,80
    pub struct UnDirGraphStEph<V: StT + Hash> {UnDirGraphStEph12,310
        V: Set<V>,V13,358
        E: Set<Edge<V>>,E14,377
    pub trait UnDirGraphStEphTrait<V: StT + Hash> {UnDirGraphStEphTrait17,409
        fn empty() -> UnDirGraphStEph<V>;empty20,553
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V>;FromSets23,703
        fn vertices(&self) -> &Set<V>;vertices26,866
        fn edges(&self) -> &Set<Edge<V>>;edges29,997
        fn sizeV(&self) -> N;sizeV32,1131
        fn sizeE(&self) -> N;sizeE35,1253
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1375
        fn NG(&self, v: &V) -> Set<V>;NG41,1518
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1675
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident47,1825
        fn Degree(&self, v: &V) -> N;Degree50,1974
    impl<V: StT + Hash> UnDirGraphStEphTrait<V> for UnDirGraphStEph<V> {UnDirGraphStEph53,2019
        fn empty() -> UnDirGraphStEph<V> {empty54,2092
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E }FromSets60,2249
        fn vertices(&self) -> &Set<V> { &self.V }vertices61,2348
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges62,2398
        fn sizeV(&self) -> N { self.V.size() }sizeV63,2451
        fn sizeE(&self) -> N { self.E.size() }sizeE64,2498
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor66,2546
        fn NG(&self, v: &V) -> Set<V> {NG76,2870
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices88,3232
        fn Incident(&self, e: &Edge<V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } elseIncident97,3504
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree99,3617
    impl<V: StT + Hash> Debug for UnDirGraphStEph<V> {UnDirGraphStEph102,3683
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt103,3738
    impl<V: StT + Hash> Display for UnDirGraphStEph<V> {UnDirGraphStEph111,3958
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} E={:?}", self.V, self.Efmt112,4015
    impl<V: StT + Hash> PartialEq for UnDirGraphStEph<V> {UnDirGraphStEph115,4122
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq116,4181
    impl<V: StT + Hash> Eq for UnDirGraphStEph<V> {}UnDirGraphStEph118,4273
    macro_rules! UnDirGraphStEphLit {UnDirGraphStEphLit121,4347
    fn _UnDirGraphStEphLit_type_checks() {_UnDirGraphStEphLit_type_checks139,5472
    pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise145,5735

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphInt.rs,1540
pub mod WeightedDirGraphStEphInt {WeightedDirGraphStEphInt3,99
    pub type WeightedDirGraphStEphInt<V> = LabDirGraphStEph<V, i32>;WeightedDirGraphStEphInt12,422
    impl<V: StT + Hash> WeightedDirGraphStEphInt<V> {WeightedDirGraphStEphInt15,572
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,678
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(add_weighted_edge32,1217
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(fromget_edge_weight35,1387
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges38,1565
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted47,1914
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted58,2327
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() total_weight69,2737
        pub fn edges_above_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_above_weight72,2893
        pub fn edges_below_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_below_weight83,3338
    macro_rules! WeightedDirGraphStEphIntLit {WeightedDirGraphStEphIntLit95,3755
    fn _WeightedDirGraphStEphIntLit_type_checks() {_WeightedDirGraphStEphIntLit_type_checks107,4341
    pub fn __weighted_dir_graph_st_int_macro_typecheck_exercise() {__weighted_dir_graph_st_int_macro_typecheck_exercise113,4633

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphFloat.rs,1530
pub mod WeightedUnDirGraphStEphFloat {WeightedUnDirGraphStEphFloat29,1110
    pub type WeightedUnDirGraphStEphFloat<V> = LabUnDirGraphStEph<V, OrderedF64>;WeightedUnDirGraphStEphFloat38,1450
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphFloat<V> {WeightedUnDirGraphStEphFloat41,1622
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges43,1738
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge58,2301
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight63,2502
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges68,2709
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted77,3064
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight90,3615
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree98,3902
        pub fn is_connected(&self) -> bool {is_connected101,4069
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge129,5095
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge137,5393
    macro_rules! WeightedUnDirGraphStEphFloatLit {WeightedUnDirGraphStEphFloatLit146,5677
    pub fn __weighted_undir_graph_st_float_macro_typecheck_exercise() {__weighted_undir_graph_st_float_macro_typecheck_exercise158,6302

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphInt.rs,1255
pub mod WeightedUnDirGraphMtEphInt {WeightedUnDirGraphMtEphInt3,100
    pub type WeightedUnDirGraphMtEphInt<V> = LabUnDirGraphMtEph<V, i32>;WeightedUnDirGraphMtEphInt12,447
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphInt<V> {WeightedUnDirGraphMtEphInt15,620
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,740
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(vadd_weighted_edge32,1289
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, vget_edge_weight35,1456
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges38,1629
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted47,1976
        pub fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum()total_weight60,2519
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree63,2685
    macro_rules! WeightedUnDirGraphMtEphIntLit {WeightedUnDirGraphMtEphIntLit67,2793
    pub fn __weighted_undir_graph_mt_int_macro_typecheck_exercise() {__weighted_undir_graph_mt_int_macro_typecheck_exercise79,3389

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphMtEph.rs,2685
pub mod LabUnDirGraphMtEph {LabUnDirGraphMtEph3,119
    pub struct LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph12,352
        vertices: Set<V>,vertices17,471
        labeled_edges: Set<LabEdge<V, L>>,labeled_edges18,497
    pub trait LabUnDirGraphMtEphTrait<V, L>LabUnDirGraphMtEphTrait21,547
        fn empty() -> Self;empty26,670
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges27,698
        fn vertices(&self) -> &Set<V>;vertices28,803
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;labeled_edges29,842
        fn edges(&self) -> Set<Edge<V>>;edges30,898
        fn add_vertex(&mut self, v: V);add_vertex31,939
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);add_labeled_edge32,979
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;get_edge_label33,1043
        fn has_edge(&self, v1: &V, v2: &V) -> bool;has_edge34,1107
        fn neighbors(&self, v: &V) -> Set<V>;neighbors35,1159
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;normalize_edge36,1205
    impl<V, L> LabUnDirGraphMtEphTrait<V, L> for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph39,1270
        fn empty() -> Self {empty44,1423
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges51,1595
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices58,1816
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> { &self.labeled_edges }labeled_edges60,1874
        fn edges(&self) -> Set<Edge<V>> {edges62,1954
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex70,2231
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge72,2300
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label83,2692
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge94,3129
        fn neighbors(&self, v: &V) -> Set<V> {neighbors104,3474
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge116,3913
    impl<V, L> Display for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph124,4333
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt129,4464
    impl<V, L> Debug for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph134,4626
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt139,4755
    macro_rules! LabUnDirGraphMtEphLit {LabUnDirGraphMtEphLit149,5027
    fn _LabUnDirGraphMtEphLit_type_checks() {_LabUnDirGraphMtEphLit_type_checks172,6233
    pub fn __lab_undir_graph_mt_macro_typecheck_exercise() {__lab_undir_graph_mt_macro_typecheck_exercise178,6512

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphFloat.rs,1253
pub mod WeightedDirGraphMtEphFloat {WeightedDirGraphMtEphFloat3,105
    pub type WeightedDirGraphMtEphFloat<V> = LabDirGraphMtEph<V, OrderedF64>;WeightedDirGraphMtEphFloat12,453
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphFloat<V> {WeightedDirGraphMtEphFloat15,636
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges17,750
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge32,1303
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight37,1507
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges42,1719
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted51,2088
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted62,2518
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight73,2945
    macro_rules! WeightedDirGraphMtEphFloatLit {WeightedDirGraphMtEphFloatLit82,3191
    pub fn __weighted_dir_graph_mt_float_macro_typecheck_exercise() {__weighted_dir_graph_mt_float_macro_typecheck_exercise94,3804

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphMtEph.rs,2765
pub mod UnDirGraphMtEph {UnDirGraphMtEph3,105
    pub struct UnDirGraphMtEph<V: StT + MtT + Hash> {UnDirGraphMtEph11,280
        V: Set<V>,V12,334
        E: Set<Edge<V>>,E13,353
    pub trait UnDirGraphMtEphTrait<V: StT + MtT + Hash> {UnDirGraphMtEphTrait16,385
        fn empty() -> UnDirGraphMtEph<V>;empty19,535
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V>;FromSets22,685
        fn vertices(&self) -> &Set<V>;vertices25,848
        fn edges(&self) -> &Set<Edge<V>>;edges28,979
        fn sizeV(&self) -> N;sizeV31,1113
        fn sizeE(&self) -> N;sizeE34,1235
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor37,1357
        fn NG(&self, v: &V) -> Set<V>;NG40,1500
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices43,1657
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident46,1807
        fn Degree(&self, v: &V) -> N;Degree49,1956
    impl<V: StT + MtT + Hash> UnDirGraphMtEphTrait<V> for UnDirGraphMtEph<V> {UnDirGraphMtEph52,2001
        fn empty() -> UnDirGraphMtEph<V> {empty53,2080
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V> { UnDirGraphMtEph { V, E }FromSets59,2237
        fn vertices(&self) -> &Set<V> { &self.V }vertices60,2336
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges61,2386
        fn sizeV(&self) -> N { self.V.size() }sizeV62,2439
        fn sizeE(&self) -> N { self.E.size() }sizeE63,2486
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor65,2534
        fn NG(&self, v: &V) -> Set<V> {NG76,2886
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices88,3254
        fn Incident(&self, e: &Edge<V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } elseIncident97,3526
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree99,3639
    impl<V: StT + MtT + Hash> std::fmt::Debug for UnDirGraphMtEph<V> {UnDirGraphMtEph102,3705
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt103,3776
    impl<V: StT + MtT + Hash> std::fmt::Display for UnDirGraphMtEph<V> {UnDirGraphMtEph111,4016
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} E={fmt112,4089
    impl<V: StT + MtT + Hash> PartialEq for UnDirGraphMtEph<V> {UnDirGraphMtEph115,4216
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq116,4281
    impl<V: StT + MtT + Hash> Eq for UnDirGraphMtEph<V> {}UnDirGraphMtEph118,4373
    macro_rules! UnDirGraphMtEphLit {UnDirGraphMtEphLit121,4453
    fn _UnDirGraphMtEphLit_type_checks() {_UnDirGraphMtEphLit_type_checks139,5578
    pub fn __undirgraph_mt_macro_typecheck_exercise() {__undirgraph_mt_macro_typecheck_exercise145,5830

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphStEph.rs,2617
pub mod LabDirGraphStEph {LabDirGraphStEph3,91
    pub struct LabDirGraphStEph<V, L>LabDirGraphStEph12,322
        vertices: Set<V>,vertices17,422
        labeled_arcs: Set<LabEdge<V, L>>,labeled_arcs18,448
    pub trait LabDirGraphStEphTrait<V, L>LabDirGraphStEphTrait21,497
        fn empty() -> Self;empty26,601
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs27,629
        fn vertices(&self) -> &Set<V>;vertices28,732
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;labeled_arcs29,771
        fn arcs(&self) -> Set<Edge<V>>;arcs30,826
        fn add_vertex(&mut self, v: V);add_vertex31,866
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);add_labeled_arc32,906
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;get_arc_label33,971
        fn has_arc(&self, from: &V, to: &V) -> bool;has_arc34,1036
        fn out_neighbors(&self, v: &V) -> Set<V>;out_neighbors35,1089
        fn in_neighbors(&self, v: &V) -> Set<V>;in_neighbors36,1139
    impl<V, L> LabDirGraphStEphTrait<V, L> for LabDirGraphStEph<V, L>LabDirGraphStEph39,1195
        fn empty() -> Self {empty44,1327
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs51,1496
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices55,1667
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }labeled_arcs57,1725
        fn arcs(&self) -> Set<Edge<V>> {arcs59,1803
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex67,2066
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc69,2135
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label75,2370
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc84,2671
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors93,2945
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors103,3267
    impl<V, L> Display for LabDirGraphStEph<V, L>LabDirGraphStEph114,3594
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt119,3731
    impl<V, L> Debug for LabDirGraphStEph<V, L>LabDirGraphStEph124,3890
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt129,4025
    macro_rules! LabDirGraphStEphLit {LabDirGraphStEphLit139,4293
    fn _LabDirGraphStEphLit_type_checks() {_LabDirGraphStEphLit_type_checks151,5085
    pub fn __lab_dir_graph_macro_typecheck_exercise() {__lab_dir_graph_macro_typecheck_exercise157,5356

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphStEph.rs,3505
pub mod DirGraphStEph {DirGraphStEph3,77
    pub struct DirGraphStEph<V: StT + Hash> {DirGraphStEph12,305
        V: Set<V>,V13,351
        A: Set<Edge<V>>,A14,370
    pub trait DirGraphStEphTrait<V: StT + Hash> {DirGraphStEphTrait17,402
        fn empty() -> DirGraphStEph<V>;empty20,544
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V>;FromSets23,692
        fn vertices(&self) -> &Set<V>;vertices26,853
        fn arcs(&self) -> &Set<Edge<V>>;arcs29,984
        fn sizeV(&self) -> N;sizeV32,1117
        fn sizeA(&self) -> N;sizeA35,1239
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1361
        fn NG(&self, v: &V) -> Set<V>;NG41,1504
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1661
        fn NPlus(&self, v: &V) -> Set<V>;NPlus47,1815
        fn NMinus(&self, v: &V) -> Set<V>;NMinus50,1953
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices53,2114
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices56,2293
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident59,2447
        fn Degree(&self, v: &V) -> N;Degree62,2599
        fn InDegree(&self, v: &V) -> N;InDegree65,2733
        fn OutDegree(&self, v: &V) -> N;OutDegree68,2869
    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {DirGraphStEph71,2917
        fn empty() -> DirGraphStEph<V> {empty72,2986
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }FromSets78,3139
        fn vertices(&self) -> &Set<V> { &self.V }vertices79,3234
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs80,3284
        fn sizeV(&self) -> N { self.V.size() }sizeV81,3336
        fn sizeA(&self) -> N { self.A.size() }sizeA82,3383
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor84,3431
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG93,3696
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices95,3753
        fn NPlus(&self, v: &V) -> Set<V> {NPlus104,4025
        fn NMinus(&self, v: &V) -> Set<V> {NMinus114,4307
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices124,4590
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices133,4872
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } eIncident142,5158
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree144,5274
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree145,5336
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree146,5401
    impl<V: StT + Hash> Debug for DirGraphStEph<V> {DirGraphStEph149,5473
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt150,5526
    impl<V: StT + Hash> Display for DirGraphStEph<V> {DirGraphStEph158,5744
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.Afmt159,5799
    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {DirGraphStEph162,5906
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq163,5963
    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}DirGraphStEph165,6055
    macro_rules! DirGraphStEphLit {DirGraphStEphLit168,6127
    fn _DirGraphStEphLit_type_checks() {_DirGraphStEphLit_type_checks185,7220
    pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise191,7475

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphMtEph.rs,3594
pub mod DirGraphMtEph {DirGraphMtEph3,102
    pub struct DirGraphMtEph<V: StT + MtT + Hash> {DirGraphMtEph11,275
        V: Set<V>,V12,327
        A: Set<Edge<V>>,A13,346
    pub trait DirGraphMtEphTrait<V: StT + MtT + Hash> {DirGraphMtEphTrait16,378
        fn empty() -> DirGraphMtEph<V>;empty19,526
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V>;FromSets22,674
        fn vertices(&self) -> &Set<V>;vertices25,835
        fn arcs(&self) -> &Set<Edge<V>>;arcs28,966
        fn sizeV(&self) -> N;sizeV31,1099
        fn sizeA(&self) -> N;sizeA34,1221
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor37,1343
        fn NG(&self, v: &V) -> Set<V>;NG40,1486
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices43,1643
        fn NPlus(&self, v: &V) -> Set<V>;NPlus46,1797
        fn NMinus(&self, v: &V) -> Set<V>;NMinus49,1935
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices52,2096
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices55,2275
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident58,2429
        fn Degree(&self, v: &V) -> N;Degree61,2581
        fn InDegree(&self, v: &V) -> N;InDegree64,2715
        fn OutDegree(&self, v: &V) -> N;OutDegree67,2851
    impl<V: StT + MtT + Hash> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {DirGraphMtEph70,2899
        fn empty() -> DirGraphMtEph<V> {empty71,2974
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V> { DirGraphMtEph { V, A } }FromSets77,3127
        fn vertices(&self) -> &Set<V> { &self.V }vertices78,3222
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs79,3272
        fn sizeV(&self) -> N { self.V.size() }sizeV80,3324
        fn sizeA(&self) -> N { self.A.size() }sizeA81,3371
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor83,3419
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG92,3690
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices94,3747
        fn NPlus(&self, v: &V) -> Set<V> {NPlus103,4019
        fn NMinus(&self, v: &V) -> Set<V> {NMinus113,4304
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices123,4590
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices132,4872
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } eIncident141,5158
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree143,5274
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree144,5336
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree145,5401
    impl<V: StT + MtT + Hash> std::fmt::Debug for DirGraphMtEph<V> {DirGraphMtEph148,5473
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt149,5542
    impl<V: StT + MtT + Hash> std::fmt::Display for DirGraphMtEph<V> {DirGraphMtEph157,5780
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} A={fmt158,5851
    impl<V: StT + MtT + Hash> PartialEq for DirGraphMtEph<V> {DirGraphMtEph161,5978
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq162,6041
    impl<V: StT + MtT + Hash> Eq for DirGraphMtEph<V> {}DirGraphMtEph164,6133
    macro_rules! DirGraphMtEphLit {DirGraphMtEphLit167,6211
    fn _DirGraphMtEphLit_type_checks() {_DirGraphMtEphLit_type_checks184,7304
    pub fn __dirgraph_mt_macro_typecheck_exercise() {__dirgraph_mt_macro_typecheck_exercise190,7548

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphFloat.rs,1906
pub mod WeightedDirGraphStEphFloat {WeightedDirGraphStEphFloat29,1075
    pub type WeightedDirGraphStEphFloat<V> = LabDirGraphStEph<V, OrderedF64>;WeightedDirGraphStEphFloat38,1407
    impl<V: StT + Hash> WeightedDirGraphStEphFloat<V> {WeightedDirGraphStEphFloat41,1573
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges43,1681
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge58,2234
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight63,2438
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges68,2650
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted77,3013
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted88,3440
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight99,3864
        pub fn edges_above_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_above_weight107,4141
        pub fn edges_below_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_below_weight118,4614
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge129,5073
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge137,5370
        pub fn scale_weights(&mut self, factor: OrderedFloat<f64>) {scale_weights145,5669
    macro_rules! WeightedDirGraphStEphFloatLit {WeightedDirGraphStEphFloatLit163,6292
    fn _WeightedDirGraphStEphFloatLit_type_checks() {_WeightedDirGraphStEphFloatLit_type_checks175,6905
    pub fn __weighted_dir_graph_st_float_macro_typecheck_exercise() {__weighted_dir_graph_st_float_macro_typecheck_exercise181,7207

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphInt.rs,1231
pub mod WeightedDirGraphMtEphInt {WeightedDirGraphMtEphInt3,98
    pub type WeightedDirGraphMtEphInt<V> = LabDirGraphMtEph<V, i32>;WeightedDirGraphMtEphInt12,437
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphInt<V> {WeightedDirGraphMtEphInt15,604
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,716
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(add_weighted_edge32,1255
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(fromget_edge_weight35,1425
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges38,1603
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted47,1958
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted58,2374
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() total_weight69,2787
    macro_rules! WeightedDirGraphMtEphIntLit {WeightedDirGraphMtEphIntLit73,2912
    pub fn __weighted_dir_graph_mt_int_macro_typecheck_exercise() {__weighted_dir_graph_mt_int_macro_typecheck_exercise85,3498

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphStEph.rs,2677
pub mod LabUnDirGraphStEph {LabUnDirGraphStEph3,94
    pub struct LabUnDirGraphStEph<V, L>LabUnDirGraphStEph12,327
        vertices: Set<V>,vertices17,435
        labeled_edges: Set<LabEdge<V, L>>,labeled_edges18,461
    pub trait LabUnDirGraphStEphTrait<V, L>LabUnDirGraphStEphTrait21,511
        fn empty() -> Self;empty26,623
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges27,651
        fn vertices(&self) -> &Set<V>;vertices28,756
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;labeled_edges29,795
        fn edges(&self) -> Set<Edge<V>>;edges30,851
        fn add_vertex(&mut self, v: V);add_vertex31,892
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);add_labeled_edge32,932
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;get_edge_label33,996
        fn has_edge(&self, v1: &V, v2: &V) -> bool;has_edge34,1060
        fn neighbors(&self, v: &V) -> Set<V>;neighbors35,1112
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;normalize_edge36,1158
    impl<V, L> LabUnDirGraphStEphTrait<V, L> for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph39,1223
        fn empty() -> Self {empty44,1365
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges51,1537
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices58,1758
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> { &self.labeled_edges }labeled_edges60,1816
        fn edges(&self) -> Set<Edge<V>> {edges62,1896
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex70,2167
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge72,2236
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label83,2622
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge94,3059
        fn neighbors(&self, v: &V) -> Set<V> {neighbors105,3468
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge117,3901
    impl<V, L> Display for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph125,4321
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt130,4441
    impl<V, L> Debug for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph135,4603
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt140,4721
    macro_rules! LabUnDirGraphStEphLit {LabUnDirGraphStEphLit150,4993
    fn _LabUnDirGraphStEphLit_type_checks() {_LabUnDirGraphStEphLit_type_checks173,6199
    pub fn __lab_undir_graph_macro_typecheck_exercise() {__lab_undir_graph_macro_typecheck_exercise179,6478

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphMtEph.rs,2625
pub mod LabDirGraphMtEph {LabDirGraphMtEph3,116
    pub struct LabDirGraphMtEph<V, L>LabDirGraphMtEph12,347
        vertices: Set<V>,vertices17,458
        labeled_arcs: Set<LabEdge<V, L>>,labeled_arcs18,484
    pub trait LabDirGraphMtEphTrait<V, L>LabDirGraphMtEphTrait21,533
        fn empty() -> Self;empty26,648
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs27,676
        fn vertices(&self) -> &Set<V>;vertices28,779
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;labeled_arcs29,818
        fn arcs(&self) -> Set<Edge<V>>;arcs30,873
        fn add_vertex(&mut self, v: V);add_vertex31,913
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);add_labeled_arc32,953
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;get_arc_label33,1018
        fn has_arc(&self, from: &V, to: &V) -> bool;has_arc34,1083
        fn out_neighbors(&self, v: &V) -> Set<V>;out_neighbors35,1136
        fn in_neighbors(&self, v: &V) -> Set<V>;in_neighbors36,1186
    impl<V, L> LabDirGraphMtEphTrait<V, L> for LabDirGraphMtEph<V, L>LabDirGraphMtEph39,1242
        fn empty() -> Self {empty44,1385
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs51,1554
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices55,1725
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }labeled_arcs57,1783
        fn arcs(&self) -> Set<Edge<V>> {arcs59,1861
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex67,2130
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc69,2199
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label75,2440
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc84,2741
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors93,3015
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors103,3340
    impl<V, L> Display for LabDirGraphMtEph<V, L>LabDirGraphMtEph114,3670
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt119,3793
    impl<V, L> Debug for LabDirGraphMtEph<V, L>LabDirGraphMtEph124,3952
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt129,4073
    macro_rules! LabDirGraphMtEphLit {LabDirGraphMtEphLit139,4341
    fn _LabDirGraphMtEphLit_type_checks() {_LabDirGraphMtEphLit_type_checks151,5133
    pub fn __lab_dir_graph_mt_macro_typecheck_exercise() {__lab_dir_graph_mt_macro_typecheck_exercise157,5404

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap11/FibonacciMt.rs,579
pub mod FibonacciMt {FibonacciMt4,131
    pub struct FibonacciMt;FibonacciMt7,198
    pub trait FibonacciMtTrait {FibonacciMtTrait10,261
        fn fib(n: N) -> N;fib11,294
    impl FibonacciMt {FibonacciMt14,328
        pub fn fib(n: N) -> N {fib15,351
    impl FibonacciMtTrait for FibonacciMt {FibonacciMt25,623
        fn fib(n: N) -> N {fib26,667
    mod tests {tests32,761
        fn fib_base_cases() {fib_base_cases37,882
        fn fib_small_values() {fib_small_values43,1035
        fn trait_and_inherent_agree() {trait_and_inherent_agree50,1239

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap03/InsertionSortSt.rs,310
pub mod InsertionSortSt {InsertionSortSt3,51
    pub trait InsertionSortStTrait<T: Ord + Clone> {InsertionSortStTrait5,78
        fn insSort(&self, slice: &mut [T]);insSort8,230
    impl<T: Ord + Clone> InsertionSortStTrait<T> for T {T11,281
        fn insSort(&self, slice: &mut [T]) {insSort12,338

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap17/MathSeq.rs,4403
pub mod MathSeq {MathSeq8,306
    pub struct MathSeqS<T: StT> {MathSeqS18,609
        data: Vec<T>,data19,643
    pub trait MathSeqTrait<T: StT + Hash> {MathSeqTrait23,708
        fn new(length: N, init_value: T) -> Self;new26,854
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str>;set30,997
        fn length(&self) -> N;length34,1172
        fn nth(&self, index: N) -> &T;nth38,1296
        fn empty() -> Self;empty42,1428
        fn singleton(item: T) -> Self;singleton46,1549
        fn subseq(&self, start: N, length: N) -> &[T];subseq50,1681
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy54,1839
        fn add_last(&mut self, value: T) -> &mut Self;add_last58,2104
        fn delete_last(&mut self) -> Option<T>;delete_last62,2252
        fn isEmpty(&self) -> B;isEmpty66,2393
        fn isSingleton(&self) -> B;isSingleton70,2518
        fn domain(&self) -> Vec<N>;domain74,2651
        fn range(&self) -> Vec<T>;range78,2784
        fn multiset_range(&self) -> Vec<(N, T)>;multiset_range82,2916
    impl<T: StT> PartialEq for MathSeqS<T> {MathSeqS85,2972
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq86,3017
    impl<T: StT> Eq for MathSeqS<T> {}MathSeqS89,3095
    impl<T: StT> std::fmt::Debug for MathSeqS<T> {MathSeqS91,3135
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt92,3186
    impl<T: StT> std::fmt::Display for MathSeqS<T> {MathSeqS97,3342
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt98,3395
    impl<T: StT> MathSeqS<T> {MathSeqS113,3808
        pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter116,3931
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }iter_mut119,4098
        pub fn empty() -> Self { Self { data: Vec::new() } }empty123,4281
        pub fn singleton(item: T) -> Self { Self { data: vec![item] } }singleton126,4434
        pub fn from_vec(data: Vec<T>) -> Self { Self { data } }from_vec129,4608
        pub fn with_len(length: N, init_value: T) -> Self {with_len132,4774
    impl<'a, T: StT> IntoIterator for &'a MathSeqS<T> {MathSeqS139,4932
        type Item = &'a T;Item140,4988
        type IntoIter = std::slice::Iter<'a, T>;IntoIter141,5015
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }into_iter142,5064
    impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {MathSeqS145,5137
        type Item = &'a mut T;Item146,5197
        type IntoIter = std::slice::IterMut<'a, T>;IntoIter147,5228
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }into_iter148,5280
    impl<T: StT> IntoIterator for MathSeqS<T> {MathSeqS151,5357
        type Item = T;Item152,5405
        type IntoIter = std::vec::IntoIter<T>;IntoIter153,5428
        fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }into_iter154,5475
    impl<T: StT + Hash> MathSeqTrait<T> for MathSeqS<T> {MathSeqS157,5553
        fn new(length: N, init_value: T) -> Self {new158,5611
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str> {set164,5758
        fn length(&self) -> N { self.data.len() }length173,6038
        fn nth(&self, index: N) -> &T { &self.data[index] }nth175,6089
        fn empty() -> Self { MathSeqS { data: Vec::new() } }empty177,6150
        fn singleton(item: T) -> Self { MathSeqS { data: vec![item] } }singleton179,6212
        fn subseq(&self, start: N, length: N) -> &[T] {subseq181,6285
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy188,6509
        fn add_last(&mut self, value: T) -> &mut Self {add_last200,6886
        fn delete_last(&mut self) -> Option<T> { self.data.pop() }delete_last205,7005
        fn isEmpty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }isEmpty207,7073
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton209,7163
        fn domain(&self) -> Vec<N> { (0..self.data.len()).collect() }domain211,7257
        fn range(&self) -> Vec<T> {range213,7328
        fn multiset_range(&self) -> Vec<(N, T)> {multiset_range224,7699
    macro_rules! MathSeqSLit {MathSeqSLit243,8383
    fn _MathSeqSLit_type_checks() {_MathSeqSLit_type_checks256,8777

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTTreapMtEph.rs,4433
pub mod BSTTreapMtEph {BSTTreapMtEph3,100
    type Link<T> = Option<Box<Node<T>>>;Link10,274
    struct Node<T: StTInMtT + Ord> {Node13,344
        key: T,key14,381
        priority: u64,priority15,397
        size: N,size16,420
        left: Link<T>,left17,437
        right: Link<T>,right18,460
    impl<T: StTInMtT + Ord> Node<T> {Node21,491
        fn new(key: T, priority: u64) -> Self {new22,529
    pub struct BSTTreapMtEph<T: StTInMtT + Ord> {BSTTreapMtEph34,784
        root: Arc<RwLock<Link<T>>>,root35,834
    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;BSTreeTreap38,877
    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTTreapMtEphTrait40,926
        fn new() -> Self;new41,987
        fn insert(&self, value: T);insert42,1013
        fn find(&self, target: &T) -> Option<T>;find43,1049
        fn contains(&self, target: &T) -> B;contains44,1098
        fn size(&self) -> N;size45,1143
        fn is_empty(&self) -> B;is_empty46,1172
        fn height(&self) -> N;height47,1205
        fn minimum(&self) -> Option<T>;minimum48,1236
        fn maximum(&self) -> Option<T>;maximum49,1276
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order50,1316
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order51,1365
    impl<T: StTInMtT + Ord> Default for BSTTreapMtEph<T> {BSTTreapMtEph54,1422
        fn default() -> Self { Self::new() }default55,1481
    impl<T: StTInMtT + Ord> BSTTreapMtEph<T> {BSTTreapMtEph58,1533
        pub fn new() -> Self {new59,1580
        pub fn size(&self) -> N {size65,1715
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty70,1848
        pub fn height(&self) -> N {height72,1939
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec73,1975
        pub fn insert(&self, value: T) {insert84,2325
        pub fn find(&self, target: &T) -> Option<T> {find90,2527
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains95,2697
        pub fn minimum(&self) -> Option<T> {minimum97,2811
        pub fn maximum(&self) -> Option<T> {maximum102,2963
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order107,3115
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order114,3400
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link121,3687
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate123,3770
        fn rotate_left(link: &mut Link<T>) {rotate_left125,3889
        fn rotate_right(link: &mut Link<T>) {rotate_right139,4344
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link153,4800
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link178,5820
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link193,6337
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link203,6658
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect213,6981
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect221,7268
    impl<T: StTInMtT + Ord> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {BSTTreapMtEph230,7564
        fn new() -> Self { BSTTreapMtEph::new() }new231,7637
        fn insert(&self, value: T) { BSTTreapMtEph::insert(self, value) }insert233,7688
        fn find(&self, target: &T) -> Option<T> { BSTTreapMtEph::find(self, target) }find235,7763
        fn contains(&self, target: &T) -> B { BSTTreapMtEph::contains(self, target) }contains237,7850
        fn size(&self) -> N { BSTTreapMtEph::size(self) }size239,7937
        fn is_empty(&self) -> B { BSTTreapMtEph::is_empty(self) }is_empty241,7996
        fn height(&self) -> N { BSTTreapMtEph::height(self) }height243,8063
        fn minimum(&self) -> Option<T> { BSTTreapMtEph::minimum(self) }minimum245,8126
        fn maximum(&self) -> Option<T> { BSTTreapMtEph::maximum(self) }maximum247,8199
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::in_order(self) }in_order249,8272
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::pre_order(self) }pre_order251,8355
    macro_rules! BSTTreapMtEphLit {BSTTreapMtEphLit255,8466
    fn _BSTTreapMtEphLit_type_checks() {_BSTTreapMtEphLit_type_checks267,8995

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTSetTreapMtEph.rs,5645
pub mod BSTSetTreapMtEph {BSTSetTreapMtEph3,75
    pub struct BSTSetTreapMtEph<T: StTInMtT + Ord> {BSTSetTreapMtEph11,312
        tree: BSTTreapMtEph<T>,tree12,365
    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;BSTSetTreapMt15,404
    pub trait BSTSetTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetTreapMtEphTrait17,458
        fn empty() -> Self;empty18,522
        fn singleton(value: T) -> Self;singleton19,550
        fn size(&self) -> N;size20,590
        fn is_empty(&self) -> B;is_empty21,619
        fn find(&self, value: &T) -> Option<T>;find22,652
        fn contains(&self, value: &T) -> B;contains23,700
        fn minimum(&self) -> Option<T>;minimum24,744
        fn maximum(&self) -> Option<T>;maximum25,784
        fn insert(&mut self, value: T);insert26,824
        fn delete(&mut self, target: &T);delete27,864
        fn union(&self, other: &Self) -> Self;union28,906
        fn intersection(&self, other: &Self) -> Self;intersection29,953
        fn difference(&self, other: &Self) -> Self;difference30,1007
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1059
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1114
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1169
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1231
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1301
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1369
        fn as_tree(&self) -> &BSTTreapMtEph<T>;as_tree37,1423
    impl<T: StTInMtT + Ord> BSTSetTreapMtEph<T> {BSTSetTreapMtEph40,1478
        pub fn empty() -> Self {empty41,1528
        pub fn singleton(value: T) -> Self {singleton47,1649
        pub fn size(&self) -> N { self.tree.size() }size53,1808
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1862
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1924
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,2002
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2080
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2148
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2216
        pub fn delete(&mut self, target: &T) {delete67,2289
        pub fn union(&self, other: &Self) -> Self {union75,2578
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2877
        pub fn difference(&self, other: &Self) -> Self {difference100,3455
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4032
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4724
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5037
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5393
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5802
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6066
        pub fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree179,6149
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6216
        fn rebuild_from_vec(values: Vec<T>) -> BSTTreapMtEph<T> {rebuild_from_vec183,6307
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6530
    impl<T: StTInMtT + Ord> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {BSTSetTreapMtEph203,6815
        fn empty() -> Self { Self::empty() }empty204,6894
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6940
        fn size(&self) -> N { self.tree.size() }size208,7007
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7057
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7115
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7189
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7263
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7327
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7391
        fn delete(&mut self, target: &T) { BSTSetTreapMtEph::delete(self, target) }delete222,7460
        fn union(&self, other: &Self) -> Self { BSTSetTreapMtEph::union(self, other) }union224,7545
        fn intersection(&self, other: &Self) -> Self { BSTSetTreapMtEph::intersection(self, otheintersection226,7633
        fn difference(&self, other: &Self) -> Self { BSTSetTreapMtEph::difference(self, other) }difference228,7735
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetTreapMtEph::split(self, pivot) }split230,7833
        fn join_pair(left: Self, right: Self) -> Self { BSTSetTreapMtEph::join_pair(left, right)join_pair232,7929
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetTreapMtEph::join_m(left, pijoin_m234,8029
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetTreapMtEph::filter(filter236,8140
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetTreapMtEph::reduce(sereduce238,8256
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8369
        fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree242,8448
    macro_rules! BSTSetTreapMtEphLit {BSTSetTreapMtEphLit246,8537
    fn _BSTSetTreapMtEphLit_type_checks() {_BSTSetTreapMtEphLit_type_checks258,9110

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTParaTreapMtEph.rs,4881
pub mod BSTParaTreapMtEph {BSTParaTreapMtEph3,96
    pub enum Exposed<T: StTInMtT + Ord> {Exposed12,329
        Leaf,Leaf13,371
        Node(ParamTreap<T>, T, ParamTreap<T>),Node14,385
    struct NodeInner<T: StTInMtT + Ord> {NodeInner18,460
        key: T,key19,502
        priority: i64,priority20,518
        size: N,size21,541
        left: ParamTreap<T>,left22,558
        right: ParamTreap<T>,right23,587
    pub struct ParamTreap<T: StTInMtT + Ord> {ParamTreap27,645
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root28,692
    fn priority_for<T: StTInMtT + Ord>(key: &T) -> i64 {priority_for31,753
    fn tree_priority<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> i64 {tree_priority39,1046
    fn tree_size<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> N {tree_size44,1233
    fn make_node<T: StTInMtT + Ord>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreamake_node49,1403
    impl<T: StTInMtT + Ord + 'static> ParamTreap<T> {ParamTreap62,1821
        fn expose_internal(&self) -> Exposed<T> {expose_internal65,1966
        pub fn expose_with_priority(&self) -> Option<(ParamTreap<T>, T, i64, ParamTreap<T>)> {expose_with_priority75,2356
        fn join_with_priority(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) join_with_priority84,2830
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid107,4007
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner119,4483
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner140,5614
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner154,6368
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner170,7112
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner190,8042
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner210,8955
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel234,10068
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner244,10432
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel268,11505
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order278,11846
    pub trait ParamTreapTrait<T: StTInMtT + Ord + 'static>: Sized {ParamTreapTrait290,12252
        fn new() -> Self;new293,12411
        fn expose(&self) -> Exposed<T>;expose296,12528
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid299,12659
        fn size(&self) -> N;size302,12800
        fn is_empty(&self) -> B;is_empty305,12920
        fn insert(&self, key: T);insert308,13064
        fn delete(&self, key: &T);delete311,13209
        fn find(&self, key: &T) -> Option<T>;find314,13355
        fn split(&self, key: &T) -> (Self, B, Self);split317,13512
        fn join_pair(&self, other: Self) -> Self;join_pair320,13724
        fn union(&self, other: &Self) -> Self;union323,13899
        fn intersect(&self, other: &Self) -> Self;intersect326,14071
        fn difference(&self, other: &Self) -> Self;difference329,14247
        fn filter<F>(&self, predicate: F) -> Selffilter332,14404
        fn reduce<F>(&self, op: F, base: T) -> Treduce337,14628
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order342,14844
    impl<T: StTInMtT + Ord + 'static> ParamTreapTrait<T> for ParamTreap<T> {ParamTreap345,14900
        fn new() -> Self {new348,15068
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose356,15287
        fn join_mid(exposed: Exposed<T>) -> Self { ParamTreap::join_mid(exposed) }join_mid360,15445
        fn size(&self) -> N { tree_size(self) }size364,15620
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty368,15760
        fn insert(&self, key: T) {insert372,15958
        fn delete(&self, key: &T) {delete383,16474
        fn find(&self, key: &T) -> Option<T> {find393,16923
        fn split(&self, key: &T) -> (Self, B, Self) { ParamTreap::split_inner(self, key) }split406,17535
        fn join_pair(&self, other: Self) -> Self { ParamTreap::join_pair_inner(self.clone(), othjoin_pair410,17786
        fn union(&self, other: &Self) -> Self { ParamTreap::union_inner(self, other) }union414,18014
        fn intersect(&self, other: &Self) -> Self { ParamTreap::intersect_inner(self, other) }intersect418,18227
        fn difference(&self, other: &Self) -> Self { ParamTreap::difference_inner(self, other) }difference422,18448
        fn filter<F>(&self, predicate: F) -> Selffilter426,18651
        fn reduce<F>(&self, op: F, base: T) -> Treduce435,18953
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order444,19246
    macro_rules! ParamTreapLit {ParamTreapLit452,19492
    fn _ParamTreapLit_type_checks() {_ParamTreapLit_type_checks464,20038

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTTreapStEph.rs,4539
pub mod BSTTreapStEph {BSTTreapStEph3,72
    type Link<T> = Option<Box<Node<T>>>;Link9,267
    struct Node<T: StT + Ord> {Node12,330
        key: T,key13,362
        priority: u64,priority14,378
        size: N,size15,401
        left: Link<T>,left16,418
        right: Link<T>,right17,441
    impl<T: StT + Ord> Node<T> {Node20,472
        fn new(key: T, priority: u64) -> Self {new21,505
    pub struct BSTTreapStEph<T: StT + Ord> {BSTTreapStEph33,760
        root: Link<T>,root34,805
    pub type BSTreeTreap<T> = BSTTreapStEph<T>;BSTreeTreap37,835
    pub trait BSTTreapStEphTrait<T: StT + Ord> {BSTTreapStEphTrait39,884
        fn new() -> Self;new40,933
        fn size(&self) -> N;size41,959
        fn is_empty(&self) -> B;is_empty42,988
        fn height(&self) -> N;height43,1021
        fn insert(&mut self, value: T);insert44,1052
        fn find(&self, target: &T) -> Option<&T>;find45,1092
        fn contains(&self, target: &T) -> B;contains46,1142
        fn minimum(&self) -> Option<&T>;minimum47,1187
        fn maximum(&self) -> Option<&T>;maximum48,1228
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order49,1269
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order50,1318
    impl<T: StT + Ord> Default for BSTreeTreap<T> {BSTreeTreap53,1375
        fn default() -> Self { Self::new() }default54,1427
    impl<T: StT + Ord> BSTTreapStEph<T> {BSTTreapStEph57,1479
        pub fn new() -> Self { BSTTreapStEph { root: None } }new58,1521
        pub fn size(&self) -> N { Self::size_link(&self.root) }size60,1584
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty62,1649
        pub fn height(&self) -> N {height64,1740
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec65,1776
        pub fn insert(&mut self, value: T) {insert74,2072
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find79,2221
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains81,2315
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum83,2429
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum85,2505
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order87,2581
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order93,2805
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link99,3031
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate101,3114
        fn rotate_left(link: &mut Link<T>) {rotate_left103,3233
        fn rotate_right(link: &mut Link<T>) {rotate_right117,3688
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link131,4144
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link156,5164
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link171,5681
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link181,6002
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect191,6325
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect199,6612
    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {BSTTreapStEph208,6908
        fn new() -> Self { BSTTreapStEph::new() }new209,6976
        fn size(&self) -> N { BSTTreapStEph::size(self) }size211,7027
        fn is_empty(&self) -> B { BSTTreapStEph::is_empty(self) }is_empty213,7086
        fn height(&self) -> N { BSTTreapStEph::height(self) }height215,7153
        fn insert(&mut self, value: T) { BSTTreapStEph::insert(self, value) }insert217,7216
        fn find(&self, target: &T) -> Option<&T> { BSTTreapStEph::find(self, target) }find219,7295
        fn contains(&self, target: &T) -> B { BSTTreapStEph::contains(self, target) }contains221,7383
        fn minimum(&self) -> Option<&T> { BSTTreapStEph::minimum(self) }minimum223,7470
        fn maximum(&self) -> Option<&T> { BSTTreapStEph::maximum(self) }maximum225,7544
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTTreapStEph::in_order(self) }in_order227,7618
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTTreapStEph::pre_order(self) }pre_order229,7701
    macro_rules! BSTTreapStEphLit {BSTTreapStEphLit233,7812
    fn _BSTTreapStEphLit_type_checks() {_BSTTreapStEphLit_type_checks245,8345

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,6135
pub mod Types {Types4,45
    pub type N = usize;N10,181
    pub enum B {B14,324
        True,True15,341
        False,False16,355
    impl std::fmt::Display for B {B23,588
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt24,623
    impl std::ops::Not for B {B32,854
        type Output = B;Output33,885
        fn not(self) -> B {not34,910
    pub trait StT: Eq + Clone + Display + Debug + Sized {}StT44,1186
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}T45,1245
    pub trait StTInMtT: StT + Send + Sync {}StTInMtT48,1409
    impl<T> StTInMtT for T where T: StT + Send + Sync {}T49,1454
    pub trait MtT: Sized + Send + Sync {MtT53,1648
        type Inner: StT;Inner54,1689
        fn clone_mt(&self) -> Self;clone_mt55,1714
        fn new_mt(inner: Self::Inner) -> Self;new_mt56,1750
    impl<T: StT + Send> MtT for std::sync::Mutex<T> {Mutex59,1804
        type Inner = T;Inner60,1858
        fn clone_mt(&self) -> Self {clone_mt61,1882
        fn new_mt(inner: Self::Inner) -> Self { std::sync::Mutex::new(inner) }new_mt65,2024
    impl<A: StT + Send + Sync, B: StT + Send + Sync> MtT for Pair<A, B> {Pair68,2110
        type Inner = Pair<A, B>;Inner69,2184
        fn clone_mt(&self) -> Self { self.clone() }clone_mt70,2217
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt71,2269
    impl MtT for usize {usize75,2410
        type Inner = usize;Inner76,2435
        fn clone_mt(&self) -> Self { *self }clone_mt77,2463
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt78,2508
    impl MtT for isize {isize81,2571
        type Inner = isize;Inner82,2596
        fn clone_mt(&self) -> Self { *self }clone_mt83,2624
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt84,2669
    impl MtT for i32 {i3287,2732
        type Inner = i32;Inner88,2755
        fn clone_mt(&self) -> Self { *self }clone_mt89,2781
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt90,2826
    impl MtT for u32 {u3293,2889
        type Inner = u32;Inner94,2912
        fn clone_mt(&self) -> Self { *self }clone_mt95,2938
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt96,2983
    impl MtT for i64 {i6499,3046
        type Inner = i64;Inner100,3069
        fn clone_mt(&self) -> Self { *self }clone_mt101,3095
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt102,3140
    impl MtT for u64 {u64105,3203
        type Inner = u64;Inner106,3226
        fn clone_mt(&self) -> Self { *self }clone_mt107,3252
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt108,3297
    impl MtT for bool {bool111,3360
        type Inner = bool;Inner112,3384
        fn clone_mt(&self) -> Self { *self }clone_mt113,3411
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt114,3456
    impl MtT for char {char117,3519
        type Inner = char;Inner118,3543
        fn clone_mt(&self) -> Self { *self }clone_mt119,3570
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt120,3615
    impl MtT for String {String124,3732
        type Inner = String;Inner125,3758
        fn clone_mt(&self) -> Self { self.clone() }clone_mt126,3787
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt127,3839
    impl<'a> MtT for &'a str {str131,3937
        type Inner = &'a str;Inner132,3968
        fn clone_mt(&self) -> Self { *self }clone_mt133,3998
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt134,4043
    impl MtT for B {B138,4148
        type Inner = B;Inner139,4169
        fn clone_mt(&self) -> Self { *self }clone_mt140,4193
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt141,4238
    pub struct Edge<V: StT>(pub V, pub V);Edge146,4440
    impl<V: StT> std::fmt::Display for Edge<V> {Edge148,4484
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})fmt149,4533
    impl<V: StT> From<(V, V)> for Edge<V> {Edge152,4657
        fn from(t: (V, V)) -> Self { Edge(t.0, t.1) }from153,4701
    impl<V: StT> From<Edge<V>> for (V, V) {V156,4762
        fn from(e: Edge<V>) -> (V, V) { (e.0, e.1) }from157,4806
    pub struct LabEdge<V: StT, L: StT + Hash>(pub V, pub V, pub L);LabEdge162,4979
    impl<V: StT, L: StT + Hash> std::fmt::Display for LabEdge<V, L> {LabEdge164,5048
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt165,5118
    impl<V: StT, L: StT + Hash> From<(V, V, L)> for LabEdge<V, L> {LabEdge170,5274
        fn from(t: (V, V, L)) -> Self { LabEdge(t.0, t.1, t.2) }from171,5342
    impl<V: StT, L: StT + Hash> From<LabEdge<V, L>> for (V, V, L) {L174,5414
        fn from(e: LabEdge<V, L>) -> (V, V, L) { (e.0, e.1, e.2) }from175,5482
    pub type OrderedF32 = OrderedFloat<f32>;OrderedF32182,5709
    pub type OrderedF64 = OrderedFloat<f64>;OrderedF64183,5754
    pub struct Pair<A, B>(pub A, pub B);Pair187,5954
    impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {Pair189,5996
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})fmt190,6084
    impl<A, B> From<(A, B)> for Pair<A, B> {Pair193,6208
        fn from(t: (A, B)) -> Self { Pair(t.0, t.1) }from194,6253
    impl<A, B> From<Pair<A, B>> for (A, B) {B197,6314
        fn from(p: Pair<A, B>) -> Self { (p.0, p.1) }from198,6359
    macro_rules! ParaPair {ParaPair202,6440
    fn _ParaPair_type_checks() {_ParaPair_type_checks213,6914
    pub fn ArraySeqSetEq<T: PartialEq>(a_len: N, a_nth: impl Fn(N) -> T, b_len: N, b_nth: impl FArraySeqSetEq222,7334
    macro_rules! EdgeLit {EdgeLit261,8392
    macro_rules! PairLit {PairLit268,8536
    macro_rules! EdgeList {EdgeList275,8680
    macro_rules! PairList {PairList285,8897
    fn _EdgeLit_type_checks() {_EdgeLit_type_checks295,9118
    fn _PairLit_type_checks() {_PairLit_type_checks301,9305
    fn _EdgeList_type_checks() {_EdgeList_type_checks307,9497
    fn _PairList_type_checks() {_PairList_type_checks313,9706

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_5.rs,1372
pub mod Exercise12_5 {Exercise12_53,86
    struct Node<T: StTInMtT> {Node10,260
        value: ManuallyDrop<T>,value11,291
        next: *mut Node<T>,next12,323
    pub struct ConcurrentStackMt<T: StTInMtT> {ConcurrentStackMt17,446
        head: AtomicPtr<Node<T>>,head18,494
    pub trait ConcurrentStackMtTrait<T: StTInMtT> {ConcurrentStackMtTrait21,535
        fn new() -> Self;new22,587
        fn push(&self, value: T);push23,613
        fn pop(&self) -> Option<T>;pop24,647
        fn is_empty(&self) -> bool;is_empty25,683
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt28,726
        fn raw_pop(&self) -> Option<*mut Node<T>> {raw_pop29,771
    impl<T: StTInMtT> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {ConcurrentStackMt47,1320
        fn new() -> Self {new48,1395
        fn push(&self, value: T) {push52,1500
        fn pop(&self) -> Option<T> {pop69,2137
        fn is_empty(&self) -> bool {is_empty76,2376
    impl<T: StTInMtT> Default for ConcurrentStackMt<T> {ConcurrentStackMt81,2486
        fn default() -> Self { Self::new() }default82,2543
    impl<T: StTInMtT> Drop for ConcurrentStackMt<T> {ConcurrentStackMt85,2595
        fn drop(&mut self) {drop86,2649
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt99,3053
        pub fn drain(&self) -> Vec<T> {drain101,3173

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_2.rs,477
pub mod Exercise12_2 {Exercise12_23,83
    pub trait FetchAddCasTrait {FetchAddCasTrait6,159
        fn fetch_add_cas(&self, delta: usize) -> usize;fetch_add_cas7,192
    impl FetchAddCasTrait for AtomicUsize {AtomicUsize10,255
        fn fetch_add_cas(&self, delta: usize) -> usize {fetch_add_cas11,299
    pub fn fetch_add_cas(target: &AtomicUsize, delta: usize) -> usize {fetch_add_cas23,753
    pub fn efficiency_note() -> &'static str {efficiency_note27,868

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_1.rs,1017
pub mod Exercise12_1 {Exercise12_13,72
    pub struct SpinLock {SpinLock12,283
        ticket: AtomicUsize,ticket13,309
        turn: AtomicUsize,turn14,338
    pub trait SpinLockTrait {SpinLockTrait17,372
        fn new() -> Self;new18,402
        fn lock(&self);lock19,428
        fn unlock(&self);unlock20,452
    impl SpinLock {SpinLock23,485
        pub fn new() -> Self {new24,505
        pub fn lock(&self) {lock28,627
        pub fn unlock(&self) {unlock35,850
        pub fn with_lock<T>(&self, action: impl FnOnce() -> T) -> T {with_lock39,947
    impl SpinLockTrait for SpinLock {SpinLock47,1140
        fn new() -> Self { SpinLock::new() }new48,1178
        fn lock(&self) { SpinLock::lock(self) }lock50,1224
        fn unlock(&self) { SpinLock::unlock(self) }unlock52,1273
    impl Default for SpinLock {SpinLock55,1332
        fn default() -> Self { SpinLock::new() }default56,1364
    pub fn parallel_increment(iterations: N) -> usize {parallel_increment59,1420

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortMtSlice.rs,1381
pub mod Chapter36MtSlice {Chapter36MtSlice3,98
    pub trait Chapter36MtSliceTrait<T: StT + Ord + Send> {Chapter36MtSliceTrait13,306
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first14,365
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median315,418
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random16,473
        fn quick_sort_mt_first(&self);quick_sort_mt_first18,528
        fn quick_sort_mt_median3(&self);quick_sort_mt_median319,567
        fn quick_sort_mt_random(&self);quick_sort_mt_random20,608
    impl<T: StT + Ord + Send> Chapter36MtSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS23,655
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first24,739
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median326,817
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random40,1288
        fn quick_sort_mt_first(&self) {quick_sort_mt_first46,1464
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort51,1624
        fn quick_sort_mt_median3(&self) {quick_sort_mt_median388,2987
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort93,3149
        fn quick_sort_mt_random(&self) {quick_sort_mt_random143,5077
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort148,5238

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortSt.rs,1495
pub mod Chapter36St {Chapter36St3,95
    pub trait Chapter36StTrait<T: StT + Ord> {Chapter36StTrait9,233
        fn pivot_st_first(&self, lo: N, hi: N) -> T;pivot_st_first10,280
        fn pivot_st_median3(&self, lo: N, hi: N) -> T;pivot_st_median311,333
        fn pivot_st_random(&self, lo: N, hi: N) -> T;pivot_st_random12,388
        fn quick_sort_st_first(&mut self);quick_sort_st_first14,443
        fn quick_sort_st_median3(&mut self);quick_sort_st_median315,486
        fn quick_sort_st_random(&mut self);quick_sort_st_random16,531
    impl<T: StT + Ord> Chapter36StTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS19,582
        fn pivot_st_first(&self, lo: N, _hi: N) -> T { self.nth(lo).clone() }pivot_st_first20,649
        fn pivot_st_median3(&self, lo: N, hi: N) -> T {pivot_st_median321,727
        fn pivot_st_random(&self, lo: N, hi: N) -> T {pivot_st_random34,1200
        fn quick_sort_st_first(&mut self) {quick_sort_st_first40,1377
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort41,1421
        fn quick_sort_st_median3(&mut self) {quick_sort_st_median375,2604
            fn median3<T: StT + Ord>(a: &ArraySeqStEphS<T>, lo: N, hi: N) -> T {median376,2650
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort89,3187
        fn quick_sort_st_random(&mut self) {quick_sort_st_random123,4371
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort124,4416

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortMt.rs,1405
pub mod Chapter36Mt {Chapter36Mt3,94
    pub trait Chapter36MtTrait<T: StT + Ord + Send> {Chapter36MtTrait25,1185
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first26,1239
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median327,1292
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random28,1347
        fn quick_sort_mt_first(&mut self);quick_sort_mt_first30,1402
        fn quick_sort_mt_median3(&mut self);quick_sort_mt_median331,1445
        fn quick_sort_mt_random(&mut self);quick_sort_mt_random32,1490
    impl<T: StT + Ord + Send> Chapter36MtTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS35,1541
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first36,1615
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median337,1692
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random50,2162
        fn quick_sort_mt_first(&mut self) {quick_sort_mt_first56,2338
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort61,2462
        fn quick_sort_mt_median3(&mut self) {quick_sort_mt_median399,3808
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort104,3934
        fn quick_sort_mt_random(&mut self) {quick_sort_mt_random154,5747
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort159,5872

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main7,236

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/MappingStEph.rs,2413
pub mod MappingStEph {MappingStEph3,72
    pub struct Mapping<A, B> {Mapping14,390
        rel: Relation<A, B>,rel15,421
    pub trait MappingStEphTrait<X: StT + Hash, Y: StT + Hash> {MappingStEphTrait18,457
        fn empty() -> Mapping<X, Y>;empty21,613
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y>;FromVec25,747
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation29,901
        fn size(&self) -> N;size33,1056
        fn domain(&self) -> Set<X>;domain37,1182
        fn range(&self) -> Set<Y>;range41,1315
        fn mem(&self, a: &X, b: &Y) -> B;mem45,1443
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>>;iter47,1486
    impl<A: Eq + Hash, B: Eq + Hash> Mapping<A, B> {Mapping50,1569
        fn unique_pairs_from_iter<I: IntoIterator<Item = Pair<A, B>>>(iter: I) -> Set<Pair<A, B>unique_pairs_from_iter51,1622
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Mapping<A, B> {Mapping61,2000
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq62,2069
    impl<A: StT + Hash, B: StT + Hash> Eq for Mapping<A, B> {}Mapping64,2144
    impl<A: StT + Hash, B: StT + Hash> Debug for Mapping<A, B> {Mapping66,2208
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.rel, f) }fmt67,2273
    impl<A: StT + Hash, B: StT + Hash> Display for Mapping<A, B> {Mapping69,2363
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.rel, f) }fmt70,2430
    impl<X: StT + Hash, Y: StT + Hash> MappingStEphTrait<X, Y> for Mapping<X, Y> {Mapping73,2523
        fn empty() -> Mapping<X, Y> {empty74,2606
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y> {FromVec80,2767
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation87,3012
        fn size(&self) -> N { self.rel.size() }size94,3278
        fn domain(&self) -> Set<X> { self.rel.domain() }domain96,3327
        fn range(&self) -> Set<Y> { self.rel.range() }range98,3385
        fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem100,3441
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.rel.iter() }iter102,3506
    macro_rules! MappingLit {MappingLit106,3628
    fn _MappingLit_type_checks() {_MappingLit_type_checks117,4193
    pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise123,4424

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/SetStEph.rs,3621
pub mod SetStEph {SetStEph3,69
    pub struct Set<T> {Set12,250
        data: HashSet<T>,data13,274
    pub trait SetStEphTrait<T: StT + Hash> {SetStEphTrait16,307
        fn empty() -> Set<T>;empty19,444
        fn singleton(x: T) -> Set<T>;singleton22,566
        fn size(&self) -> N;size25,696
        fn mem(&self, x: &T) -> B;mem28,817
        fn union(&self, other: &Set<T>) -> Set<T>;union31,960
        fn intersection(&self, other: &Set<T>) -> Set<T>;intersection34,1119
        fn partition(&self, parts: &Set<Set<T>>) -> B;partition37,1299
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>;CartesianProduct41,1465
        fn insert(&mut self, x: T) -> &mut Self;insert45,1644
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;iter49,1786
        fn FromVec(v: Vec<T>) -> Set<T>;FromVec52,1949
    impl<T: Eq + Hash> PartialEq for Set<T> {Set55,1997
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq56,2043
    impl<T: Eq + Hash> Eq for Set<T> {}Set59,2121
    impl<T: Eq + Hash> std::fmt::Debug for Set<T>Set61,2162
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt65,2256
    impl<T: Eq + Hash> std::fmt::Display for Set<T>Set70,2411
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt74,2509
    impl<T: Eq + Hash> Hash for Set<T> {Set90,3015
        fn hash<H: Hasher>(&self, state: &mut H) {hash91,3056
    impl<T: Eq + Hash> Set<T> {Set107,3620
        pub fn empty() -> Set<T> { Set { data: HashSet::new() } }empty108,3652
        pub fn singleton(x: T) -> Set<T> {singleton110,3719
        pub fn size(&self) -> N { self.data.len() }size116,3885
        pub fn mem(&self, x: &T) -> B { if self.data.contains(x) { B::True } else { B::False } }mem118,3938
        pub fn union(&self, other: &Set<T>) -> Set<T>union120,4036
        pub fn intersection(&self, other: &Set<T>) -> Set<T>intersection131,4310
        pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition142,4667
        pub fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct160,5222
        pub fn insert(&mut self, x: T) -> &mut Self {insert174,5668
        pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter179,5791
        pub fn FromVec(v: Vec<T>) -> Set<T> {FromVec181,5883
    impl<T: StT + Hash> SetStEphTrait<T> for Set<T> {Set190,6107
        fn empty() -> Set<T> { Set { data: HashSet::new() } }empty191,6161
        fn singleton(x: T) -> Set<T> {singleton193,6224
        fn size(&self) -> N { self.data.len() }size199,6386
        fn mem(&self, x: &T) -> B { if self.data.contains(x) { B::True } else { B::False } }mem201,6435
        fn union(&self, other: &Set<T>) -> Set<T>union203,6529
        fn intersection(&self, other: &Set<T>) -> Set<T>intersection214,6799
        fn partition(&self, parts: &Set<Set<T>>) -> B {partition225,7152
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct243,7703
        fn insert(&mut self, x: T) -> &mut Self {insert257,8145
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter262,8264
        fn FromVec(v: Vec<T>) -> Set<T> {FromVec264,8352
    macro_rules! SetLit {SetLit274,8592
    fn _SetLit_type_checks() {_SetLit_type_checks286,8934
    pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise292,9130
        let _s0: Set<&'static str> = SetLit![];str293,9176

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/RelationStEph.rs,2233
pub mod RelationStEph {RelationStEph3,63
    pub struct Relation<A, B> {Relation14,334
        pairs: Set<Pair<A, B>>,pairs15,366
    pub trait RelationStEphTrait<X: StT + Hash, Y: StT + Hash> {RelationStEphTrait18,405
        fn empty() -> Relation<X, Y>;empty21,562
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y>;FromSet25,705
        fn size(&self) -> N;size29,860
        fn domain(&self) -> Set<X>domain33,986
        fn range(&self) -> Set<Y>range39,1154
        fn mem(&self, a: &X, b: &Y) -> Bmem45,1317
        fn iter(&self) -> Iter<'_, Pair<X, Y>>;iter50,1417
    impl<A: StT + Hash, B: StT + Hash> Relation<A, B> {Relation53,1472
        pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B> { Relation { pairs: Set::FromVec(v)FromVec54,1528
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Relation<A, B> {Relation57,1636
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq58,1706
    impl<A: StT + Hash, B: StT + Hash> Eq for Relation<A, B> {}Relation61,1786
    impl<A: StT + Hash, B: StT + Hash> Debug for Relation<A, B> {Relation63,1851
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }fmt64,1917
    impl<A: StT + Hash, B: StT + Hash> Display for Relation<A, B> {Relation67,2020
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) fmt68,2088
    impl<X: StT + Hash, Y: StT + Hash> RelationStEphTrait<X, Y> for Relation<X, Y> {Relation71,2193
        fn empty() -> Relation<X, Y> { Relation { pairs: SetLit![] } }empty72,2278
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y> { Relation { pairs } }FromSet74,2350
        fn size(&self) -> N { self.pairs.size() }size76,2435
        fn domain(&self) -> Set<X>domain78,2486
        fn range(&self) -> Set<Y>range89,2753
        fn mem(&self, a: &X, b: &Y) -> Bmem100,3019
        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }iter112,3295
    macro_rules! RelationLit {RelationLit116,3391
    fn _RelationLit_type_checks() {_RelationLit_type_checks132,4268
    pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise138,4503

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap23/PrimTreeSeqSt.rs,984
pub mod PrimTreeSeqSt {PrimTreeSeqSt9,476
    pub enum PrimTreeSeqStTree<T: StT> {PrimTreeSeqStTree14,655
        Zero,Zero15,696
        One(T),One16,710
        Two(PrimTreeSeqStS<T>, PrimTreeSeqStS<T>),Two17,726
    pub struct PrimTreeSeqStS<T: StT> {PrimTreeSeqStS22,905
        data: Vec<T>,data23,945
    impl<T: StT> PrimTreeSeqStS<T> {PrimTreeSeqStS26,974
        pub fn empty() -> Self { Self { data: Vec::new() } }empty28,1050
        pub fn singleton(value: T) -> Self { Self { data: vec![value] } }singleton31,1171
        pub fn from_vec(vec: Vec<T>) -> Self { Self { data: vec } }from_vec34,1333
        pub fn into_vec(self) -> Vec<T> { self.data }into_vec37,1462
        pub fn as_slice(&self) -> &[T] { &self.data }as_slice40,1580
        pub fn length(&self) -> N { self.data.len() }length43,1695
        pub fn expose(&self) -> PrimTreeSeqStTree<T> {expose46,1827
        pub fn join(tree: PrimTreeSeqStTree<T>) -> Self {join60,2451

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap23/BBTEph.rs,2136
pub mod BBTEph {BBTEph3,56
    pub enum BBTree<T: StT> {BBTree9,266
        Leaf,Leaf10,296
        Node(Box<BBNode<T>>),Node11,310
    pub struct BBNode<T: StT> {BBNode15,390
        pub(crate) left: BBTree<T>,left16,422
        pub(crate) value: T,value17,458
        pub(crate) right: BBTree<T>,right18,487
    impl<T: StT> BBNode<T> {BBNode21,531
        fn new(left: BBTree<T>, value: T, right: BBTree<T>) -> Self { BBNode { left, value, righnew22,560
    pub trait BBTEphTrait<T: StT> {BBTEphTrait25,669
        fn leaf() -> Self;leaf26,705
        fn node(left: Self, value: T, right: Self) -> Self;node27,732
        fn is_leaf(&self) -> B;is_leaf28,792
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order29,824
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order30,873
        fn height(&self) -> N;height31,923
        fn size(&self) -> N;size32,954
    impl<T: StT> BBTree<T> {BBTree35,990
        pub fn leaf() -> Self { BBTree::Leaf }leaf36,1019
        pub fn node(left: BBTree<T>, value: T, right: BBTree<T>) -> Self {node38,1067
        pub fn is_leaf(&self) -> B {is_leaf42,1221
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order49,1398
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order62,2066
        pub fn height(&self) -> N {height75,2735
        pub fn size(&self) -> N {size86,3070
    impl<T: StT> BBTEphTrait<T> for BBTree<T> {BBTree94,3279
        fn leaf() -> Self { BBTree::leaf() }leaf95,3327
        fn node(left: Self, value: T, right: Self) -> Self { BBTree::node(left, value, right) }node97,3373
        fn is_leaf(&self) -> B { BBTree::is_leaf(self) }is_leaf99,3470
        fn in_order(&self) -> ArraySeqStPerS<T> { BBTree::in_order(self) }in_order101,3528
        fn pre_order(&self) -> ArraySeqStPerS<T> { BBTree::pre_order(self) }pre_order103,3604
        fn height(&self) -> N { BBTree::height(self) }height105,3682
        fn size(&self) -> N { BBTree::size(self) }size107,3738
    macro_rules! BBNodeLit {BBNodeLit111,3816
    fn _BBNodeLit_type_checks() {_BBNodeLit_type_checks118,4042

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeq.rs,6357
pub mod ArraySeq {ArraySeq4,150
    pub struct ArraySeqS<T> {ArraySeqS12,332
        data: Box<[T]>,data13,362
    pub type ArrayS<T> = ArraySeqS<T>;ArrayS16,393
    pub trait ArraySeq<T> {ArraySeq19,508
        fn new(length: N, init_value: T) -> ArraySeqS<T>new22,685
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str>;set28,882
        fn length(&self) -> N;length32,1085
        fn nth(&self, index: N) -> &T;nth36,1251
        fn empty() -> ArraySeqS<T>;empty40,1402
        fn singleton(item: T) -> ArraySeqS<T>;singleton44,1574
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqS<T>;tabulate48,1758
        fn map<U: Clone>(a: &ArraySeqS<T>, f: impl Fn(&T) -> U) -> ArraySeqS<U>;map52,1939
        fn subseq(a: &ArraySeqS<T>, start: N, length: N) -> ArraySeqS<T>subseq56,2176
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> ArraySeqS<T>append62,2404
        fn filter(a: &ArraySeqS<T>, pred: impl Fn(&T) -> B) -> ArraySeqS<T>filter68,2629
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> ArraySeqS<T>flatten74,2874
        fn update(a: &ArraySeqS<T>, update: Pair<N, T>) -> ArraySeqS<T>update80,3116
        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T>inject86,3373
        fn isEmpty(&self) -> B;isEmpty92,3620
        fn isSingleton(&self) -> B;isSingleton96,3781
        fn collect<K: Clone + Eq, V: Clone>(collect100,3983
        fn iterate<A>(a: &ArraySeqS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A;iterate106,4227
        fn reduce(a: &ArraySeqS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> Treduce110,4451
        fn scan(a: &ArraySeqS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqS<T>, T)scan116,4693
    impl<T> ArraySeqS<T> {ArraySeqS121,4824
        fn new(length: N, init_value: T) -> ArraySeqS<T>new122,4851
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {set131,5103
        fn length(&self) -> N { self.data.len() }length140,5389
        fn nth(&self, index: N) -> &T { &self.data[index] }nth142,5440
        fn empty() -> ArraySeqS<T> { ArraySeqS::from_vec(Vec::new()) }empty144,5501
        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::from_vec(vec![item]) }singleton146,5573
        fn isEmpty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }isEmpty148,5656
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton150,5746
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq155,6067
        pub fn subseq_copy(&self, start: N, length: N) -> ArraySeqS<T>subseq_copy164,6677
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqS<T> {update180,7395
        pub fn from_vec(elts: Vec<T>) -> ArraySeqS<T> {from_vec191,7925
        pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter197,8077
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }iter_mut199,8153
    impl<T> ArraySeq<T> for ArraySeqS<T> {ArraySeqS202,8250
        fn new(length: N, init_value: T) -> ArraySeqS<T>new203,8293
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {set210,8454
        fn length(&self) -> N { ArraySeqS::length(self) }length214,8601
        fn nth(&self, index: N) -> &T { ArraySeqS::nth(self, index) }nth216,8660
        fn empty() -> ArraySeqS<T> { ArraySeqS::empty() }empty218,8731
        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::singleton(item) }singleton220,8790
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqS<T> {tabulate222,8868
        fn map<U: Clone>(a: &ArraySeqS<T>, f: impl Fn(&T) -> U) -> ArraySeqS<U> {map230,9135
        fn subseq(a: &ArraySeqS<T>, start: N, length: N) -> ArraySeqS<T>subseq239,9450
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> ArraySeqS<T>append246,9621
        fn filter(a: &ArraySeqS<T>, pred: impl Fn(&T) -> B) -> ArraySeqS<T>filter264,10195
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> ArraySeqS<T>flatten278,10610
        fn update(a: &ArraySeqS<T>, Pair(index, item): Pair<N, T>) -> ArraySeqS<T>update292,11027
        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T>inject303,11387
        fn isEmpty(&self) -> B { ArraySeqS::isEmpty(self) }isEmpty318,11949
        fn isSingleton(&self) -> B { ArraySeqS::isSingleton(self) }isSingleton320,12010
        fn collect<K: Clone + Eq, V: Clone>(collect322,12079
        fn iterate<A>(a: &ArraySeqS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A {iterate345,12992
        fn reduce(a: &ArraySeqS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> Treduce353,13224
        fn scan(a: &ArraySeqS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqS<T>, T)scan364,13493
    impl<T: PartialEq> PartialEq for ArraySeqS<T> {ArraySeqS379,13950
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq380,14002
    impl<T: Eq> Eq for ArraySeqS<T> {}ArraySeqS383,14080
    impl<T: Debug> Debug for ArraySeqS<T> {ArraySeqS385,14120
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt386,14164
    impl<T: Display> Display for ArraySeqS<T> {ArraySeqS389,14283
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt390,14331
    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {ArraySeqS402,14661
        type Item = &'a T;Item403,14713
        type IntoIter = std::slice::Iter<'a, T>;IntoIter404,14740
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }into_iter406,14790
    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {ArraySeqS409,14863
        type Item = &'a mut T;Item410,14919
        type IntoIter = std::slice::IterMut<'a, T>;IntoIter411,14950
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }into_iter413,15003
    impl<T> IntoIterator for ArraySeqS<T> {ArraySeqS416,15080
        type Item = T;Item417,15124
        type IntoIter = std::vec::IntoIter<T>;IntoIter418,15147
        fn into_iter(self) -> Self::IntoIter { Vec::from(self.data).into_iter() }into_iter420,15195
    macro_rules! ArraySeqS {ArraySeqS424,15304
    fn _arrayseqs_macro_type_checks() {_arrayseqs_macro_type_checks437,15740

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStPer.rs,5144
pub mod LinkedListStPer {LinkedListStPer3,48
    pub struct NodeP<T: StT> {NodeP9,171
        pub value: T,value10,202
        pub next: Option<Box<NodeP<T>>>,next11,224
    pub struct LinkedListStPerS<T: StT> {LinkedListStPerS15,300
        head: Option<Box<NodeP<T>>>,head16,342
        len: N,len17,379
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait20,402
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>new21,447
        fn empty() -> LinkedListStPerS<T>;empty24,547
        fn singleton(item: T) -> LinkedListStPerS<T>;singleton25,590
        fn length(&self) -> N;length26,644
        fn nth(&self, index: N) -> &T;nth27,675
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T>;subseq_copy28,714
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate29,789
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map30,859
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append31,952
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter32,1044
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;update33,1135
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject34,1223
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject35,1330
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate36,1438
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes37,1527
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce38,1647
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan40,1730
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten42,1834
        fn collect<A: StT, Bv: StT>(collect44,1922
    impl<T: StT> LinkedListStPerS<T> {LinkedListStPerS50,2114
        pub fn empty() -> Self { LinkedListStPerS { head: None, len: 0 } }empty51,2153
        pub fn new(length: N, init_value: T) -> Selfnew53,2229
        pub fn singleton(item: T) -> Self { LinkedListStPerS::from_vec(vec![item]) }singleton60,2404
        pub fn from_vec(elts: Vec<T>) -> Self {from_vec62,2490
        pub fn length(&self) -> N { self.len }length72,2840
        pub fn nth(&self, index: N) -> &T {nth74,2888
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy80,3063
        fn node_at(&self, index: N) -> Option<&NodeP<T>> {node_at110,4145
    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {LinkedListStPerS127,4620
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt128,4681
    impl<T: StT> PartialEq for LinkedListStPerS<T> {LinkedListStPerS145,5214
        fn eq(&self, other: &Self) -> bool {eq146,5267
    impl<T: StT> Eq for LinkedListStPerS<T> {}LinkedListStPerS163,5779
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS165,5827
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>new166,5894
        fn empty() -> LinkedListStPerS<T> { LinkedListStPerS::empty() }empty173,6069
        fn singleton(item: T) -> LinkedListStPerS<T> { LinkedListStPerS::singleton(item) }singleton174,6141
        fn length(&self) -> N { LinkedListStPerS::length(self) }length175,6232
        fn nth(&self, index: N) -> &T { LinkedListStPerS::nth(self, index) }nth176,6297
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T> {subseq_copy177,6374
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate181,6524
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map189,6790
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append197,7104
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter208,7533
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T>update219,7925
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject232,8414
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject245,9008
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate256,9480
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes264,9718
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce274,10150
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan290,10773
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten305,11470
        fn collect<A: StT, Bv: StT>(collect316,11874

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStEph.rs,5663
pub mod ArraySeqStEph {ArraySeqStEph3,72
    pub struct ArraySeqStEphS<T: StT> {ArraySeqStEphS10,254
        data: Box<[T]>,data11,294
    pub type ArrayStEph<T> = ArraySeqStEphS<T>;ArrayStEph14,325
    impl<T: StT> ArraySeqStEphS<T> {ArraySeqStEphS16,374
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }from_vec17,411
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) new19,501
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }empty21,600
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton23,663
        pub fn length(&self) -> N { self.data.len() }length25,737
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth27,792
        pub fn subseq(&self, start: N, length: N) -> Self {subseq29,857
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set36,1150
        pub fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {update45,1432
        pub fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut Self {inject50,1582
    impl<T: StT> PartialEq for ArraySeqStEphS<T> {ArraySeqStEphS62,2016
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }eq63,2067
    impl<T: StT> Eq for ArraySeqStEphS<T> {}ArraySeqStEphS66,2153
    impl<T: StT> Debug for ArraySeqStEphS<T> {ArraySeqStEphS68,2199
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt69,2246
    impl<T: StT> Display for ArraySeqStEphS<T> {ArraySeqStEphS72,2365
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt73,2414
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait85,2744
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>;new86,2787
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str>;set87,2850
        fn length(&self) -> N;length88,2944
        fn nth(&self, index: N) -> &T;nth89,2975
        fn empty() -> ArraySeqStEphS<T>;empty90,3014
        fn singleton(item: T) -> ArraySeqStEphS<T>;singleton91,3055
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStEphS<T>;tabulate92,3107
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map93,3180
        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T>;subseq94,3269
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append95,3353
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter96,3439
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten97,3526
        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T>;update98,3606
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T>;inject99,3682
        fn isEmpty(&self) -> B;isEmpty100,3776
        fn isSingleton(&self) -> B;isSingleton101,3808
        fn collect<K: StT, V: StT>(collect102,3844
        fn iterate<A>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A;iterate106,4024
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce107,4109
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan108,4189
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS111,4295
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::new(length, initnew112,4358
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str> {set114,4465
        fn length(&self) -> N { ArraySeqStEphS::length(self) }length118,4622
        fn nth(&self, index: N) -> &T { ArraySeqStEphS::nth(self, index) }nth120,4686
        fn empty() -> ArraySeqStEphS<T> { ArraySeqStEphS::empty() }empty122,4762
        fn singleton(item: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::singleton(item) }singleton124,4831
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStEphS<T> {tabulate126,4919
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map134,5196
        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T> { a.subseq(stsubseq142,5504
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append144,5616
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter156,6068
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten167,6454
        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T> { ArraySeqStEphS::updupdate178,6847
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T> {inject180,6964
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty184,7120
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton186,7208
        fn collect<K: StT, V: StT>(collect188,7300
        fn iterate<A>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A {iterate210,8231
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce218,8468
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan226,8698
    macro_rules! ArraySeqStEphS {ArraySeqStEphS238,9126
    fn _arrayseqstephs_macro_type_checks() {_arrayseqstephs_macro_type_checks245,9536

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtPer.rs,5729
pub mod ArraySeqMtPer {ArraySeqMtPer3,82
    pub struct ArraySeqMtPerS<T: StTInMtT> {ArraySeqMtPerS10,272
        data: Box<[T]>,data11,317
    impl<T: StTInMtT> ArraySeqMtPerS<T> {ArraySeqMtPerS14,348
        pub fn empty() -> Self {empty15,390
        pub fn new(length: N, init_value: T) -> Self {new21,530
        pub fn singleton(item: T) -> Self { ArraySeqMtPerS::from_vec(vec![item]) }singleton29,802
        pub fn from_vec(values: Vec<T>) -> Self {from_vec31,886
        pub fn length(&self) -> N { self.data.len() }length37,1039
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth39,1094
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set41,1159
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy50,1524
        pub fn is_empty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }is_empty58,1849
        pub fn is_singleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } is_singleton60,1944
    impl<T: StTInMtT> Clone for ArraySeqMtPerS<T> {ArraySeqMtPerS63,2049
        fn clone(&self) -> Self {clone64,2101
    impl<T: StTInMtT + PartialEq> PartialEq for ArraySeqMtPerS<T> {ArraySeqMtPerS70,2267
        fn eq(&self, other: &Self) -> bool {eq71,2335
    impl<T: StTInMtT + Eq> Eq for ArraySeqMtPerS<T> {}ArraySeqMtPerS84,2662
    pub trait ArraySeqMtPerTrait<T: StTInMtT> {ArraySeqMtPerTrait86,2718
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;new87,2766
        fn length(&self) -> N;length88,2829
        fn nth(&self, index: N) -> &T;nth89,2860
        fn empty() -> ArraySeqMtPerS<T>;empty90,2899
        fn singleton(item: T) -> ArraySeqMtPerS<T>;singleton91,2940
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T>;tabulate94,3160
        fn map<W: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W) -> ArraySeqMtPerS<W>;map97,3400
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;subseq_copy98,3489
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append101,3674
        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtPerS<T>;filter104,3966
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;flatten106,4123
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T>;update109,4304
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerSinject112,4548
        fn isEmpty(&self) -> B;isEmpty113,4649
        fn isSingleton(&self) -> B;isSingleton114,4681
        fn collect(collect116,4796
        fn iterate<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate121,5033
        fn reduce(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce123,5203
        fn scan(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtPerS<T>, Tscan125,5330
        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str>;set126,5429
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject129,5638
        fn iteratePrefixes<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes131,5785
    impl<T: StTInMtT> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {ArraySeqMtPerS134,5908
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::new(length, initnew135,5976
        fn length(&self) -> N { ArraySeqMtPerS::length(self) }length137,6083
        fn nth(&self, index: N) -> &T { ArraySeqMtPerS::nth(self, index) }nth139,6147
        fn empty() -> ArraySeqMtPerS<T> { ArraySeqMtPerS::empty() }empty141,6223
        fn singleton(item: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::singleton(item) }singleton143,6292
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T> {tabulate145,6380
        fn map<W: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W) -> ArraySeqMtPerS<W> {map150,6570
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> {subseq_copy158,6878
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append162,7024
        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtPerS<T> {filter173,7448
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {flatten184,7834
        fn update(a: &ArraySeqMtPerS<T>, Pair(index, item): Pair<N, T>) -> ArraySeqMtPerS<T> {update195,8230
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerSinject199,8397
        fn isEmpty(&self) -> B { ArraySeqMtPerS::is_empty(self) }isEmpty211,8913
        fn isSingleton(&self) -> B { ArraySeqMtPerS::is_singleton(self) }isSingleton213,8980
        fn collect(collect215,9055
        fn iterate<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate242,10207
        fn reduce(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce250,10443
        fn scan(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtPerS<T>, Tscan265,11050
        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str> {set275,11454
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject279,11602
        fn iteratePrefixes<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes290,12035
    macro_rules! ArraySeqMtPerSLit {ArraySeqMtPerSLit303,12520
    fn _ArraySeqMtPerSLit_type_checks() {_ArraySeqMtPerSLit_type_checks310,12909

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtEph.rs,4473
pub mod ArraySeqMtEph {ArraySeqMtEph3,67
    pub struct ArraySeqMtEphS<T: StT> {ArraySeqMtEphS11,297
        data: Mutex<Box<[T]>>,data12,337
    impl<T: StT> ArraySeqMtEphS<T> {ArraySeqMtEphS15,375
        pub fn empty() -> Self {empty16,412
        pub fn new(length: N, init_value: T) -> Selfnew22,564
        pub fn singleton(item: T) -> Self { ArraySeqMtEphS::from_vec(vec![item]) }singleton29,737
        pub fn from_vec(values: Vec<T>) -> Self {from_vec31,821
        pub fn length(&self) -> N {length37,986
        pub fn nth_cloned(&self, index: N) -> T {nth_cloned42,1108
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set47,1253
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy57,1582
        pub fn to_vec(&self) -> Vec<T> {to_vec66,1950
    impl<T: StT> Clone for ArraySeqMtEphS<T> {ArraySeqMtEphS72,2103
        fn clone(&self) -> Self { ArraySeqMtEphS::from_vec(self.to_vec()) }clone73,2150
    impl<T: StT> PartialEq for ArraySeqMtEphS<T> {ArraySeqMtEphS76,2233
        fn eq(&self, other: &Self) -> bool { self.to_vec() == other.to_vec() }eq77,2284
    impl<T: StT> Eq for ArraySeqMtEphS<T> {}ArraySeqMtEphS80,2370
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait82,2416
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>new83,2459
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str>;set86,2557
        fn length(&self) -> N;length87,2651
        fn nth_cloned(&self, index: N) -> T;nth_cloned88,2682
        fn empty() -> ArraySeqMtEphS<T>;empty89,2727
        fn singleton(item: T) -> ArraySeqMtEphS<T>;singleton90,2768
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate91,2820
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map92,2888
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;subseq_copy93,2977
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append94,3050
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter95,3136
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;update96,3223
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject97,3312
        fn isEmpty(&self) -> B;isEmpty98,3413
        fn isSingleton(&self) -> B;isSingleton99,3445
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject100,3481
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS103,3590
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>new104,3653
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str> {set111,3824
        fn length(&self) -> N { ArraySeqMtEphS::length(self) }length115,3981
        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphS::nth_cloned(self, index) }nth_cloned117,4045
        fn empty() -> ArraySeqMtEphS<T> { ArraySeqMtEphS::empty() }empty119,4134
        fn singleton(item: T) -> ArraySeqMtEphS<T> { ArraySeqMtEphS::singleton(item) }singleton121,4203
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate123,4291
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map131,4553
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {subseq_copy143,4982
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append147,5128
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter160,5581
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update172,5990
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject177,6151
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty189,6592
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton191,6680
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject193,6772
    macro_rules! ArraySeqMtEphSLit {ArraySeqMtEphSLit204,7125
    fn _ArraySeqMtEphSLit_type_checks() {_ArraySeqMtEphSLit_type_checks211,7538

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStEph.rs,5066
pub mod LinkedListStEph {LinkedListStEph3,60
    pub struct NodeE<T: StT> {NodeE9,183
        pub value: T,value10,214
        pub next: Option<Box<NodeE<T>>>,next11,236
    pub struct LinkedListStEphS<T: StT> {LinkedListStEphS15,312
        head: Option<Box<NodeE<T>>>,head16,354
        len: N,len17,391
    impl<T: StT> LinkedListStEphS<T> {LinkedListStEphS20,414
        pub fn empty() -> Self { LinkedListStEphS { head: None, len: 0 } }empty21,453
        pub fn new(length: N, init_value: T) -> Selfnew23,529
        pub fn singleton(item: T) -> Self { LinkedListStEphS::from_vec(vec![item]) }singleton30,704
        pub fn from_vec(mut elts: Vec<T>) -> Self {from_vec32,790
        pub fn length(&self) -> N { self.len }length41,1117
        pub fn nth(&self, index: N) -> &T {nth43,1165
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set49,1340
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy59,1669
        fn node_at(&self, index: N) -> Option<&NodeE<T>> {node_at89,2751
        fn node_at_mut(&mut self, index: N) -> Option<&mut NodeE<T>> {node_at_mut105,3220
    impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {LinkedListStEphS122,3715
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt123,3776
    impl<T: StT> PartialEq for LinkedListStEphS<T> {LinkedListStEphS140,4309
        fn eq(&self, other: &Self) -> bool {eq141,4362
    impl<T: StT> Eq for LinkedListStEphS<T> {}LinkedListStEphS158,4874
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait160,4922
        fn new(length: N, init_value: T) -> Selfnew161,4967
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set165,5053
        fn length(&self) -> N;length167,5135
        fn nth(&self, index: N) -> &T;nth169,5167
        fn empty() -> Self;empty171,5207
        fn singleton(item: T) -> Self;singleton173,5236
        fn tabulate(f: impl Fn(N) -> T, n: N) -> Self;tabulate177,5368
        fn map<U: StT>(a: &Self, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map180,5519
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy181,5597
        fn append(a: &Self, b: &Self) -> Self;append184,5765
        fn filter(a: &Self, pred: impl Fn(&T) -> B) -> Self;filter187,5908
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten188,5969
        fn update(a: &mut Self, item_at: Pair<N, T>) -> &mut Self;update191,6164
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;inject194,6351
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;ninject197,6548
        fn collect<A: StT, Bv: StT>(collect198,6626
        fn iterate<A: StT>(a: &Self, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate202,6811
        fn iteratePrefixes<A: StT>(a: &Self, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListStEphSiteratePrefixes203,6885
        fn reduce(a: &Self, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce204,6990
        fn scan(a: &Self, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, T);scan205,7057
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS208,7152
        fn new(length: N, init_value: T) -> Selfnew209,7219
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set216,7379
        fn length(&self) -> N { LinkedListStEphS::length(self) }length220,7525
        fn nth(&self, index: N) -> &T { LinkedListStEphS::nth(self, index) }nth222,7591
        fn empty() -> Self { LinkedListStEphS::empty() }empty224,7669
        fn singleton(item: T) -> Self { LinkedListStEphS::singleton(item) }singleton226,7727
        fn tabulate(f: impl Fn(N) -> T, n: N) -> Self {tabulate228,7804
        fn map<U: StT>(a: &Self, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map236,8055
        fn subseq_copy(&self, start: N, length: N) -> Self { LinkedListStEphS::subseq_copy(self,subseq_copy244,8354
        fn append(a: &Self, b: &Self) -> Self {append246,8469
        fn filter(a: &Self, pred: impl Fn(&T) -> B) -> Self {filter257,8853
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten268,9215
        fn update(a: &mut Self, Pair(index, item): Pair<N, T>) -> &mut Self {update279,9619
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {inject284,9762
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {ninject296,10186
        fn collect<A: StT, Bv: StT>(collect305,10490
        fn iterate<A: StT>(a: &Self, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate325,11344
        fn iteratePrefixes<A: StT>(a: &Self, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListStEphSiteratePrefixes333,11567
        fn reduce(a: &Self, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce343,11984
        fn scan(a: &Self, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, T) {scan359,12592

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStPer.rs,4961
pub mod ArraySeqStPer {ArraySeqStPer3,79
    pub struct ArraySeqStPerS<T: StT> {ArraySeqStPerS11,347
        data: Box<[T]>,data12,387
    pub type ArrayStPer<T> = ArraySeqStPerS<T>;ArrayStPer15,418
    impl<T: StT> ArraySeqStPerS<T> {ArraySeqStPerS17,467
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }from_vec18,504
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) new19,593
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }empty20,691
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton21,753
        pub fn length(&self) -> N { self.data.len() }length22,826
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth23,880
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy24,944
    impl<T: StT> PartialEq for ArraySeqStPerS<T> {ArraySeqStPerS33,1287
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }eq34,1338
    impl<T: StT> Eq for ArraySeqStPerS<T> {}ArraySeqStPerS37,1424
    impl<T: StT> Debug for ArraySeqStPerS<T> {ArraySeqStPerS39,1470
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt40,1517
    impl<T: StT> Display for ArraySeqStPerS<T> {ArraySeqStPerS43,1636
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt44,1685
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait57,2098
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>;new58,2141
        fn length(&self) -> N;length59,2204
        fn nth(&self, index: N) -> &T;nth60,2235
        fn empty() -> ArraySeqStPerS<T>;empty61,2274
        fn singleton(item: T) -> ArraySeqStPerS<T>;singleton62,2315
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStPerS<T>;tabulate63,2367
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U>;map64,2440
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T>;subseq_copy65,2529
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append66,2618
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T>;filter67,2704
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;flatten68,2791
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject69,2871
        fn isEmpty(&self) -> B;isEmpty70,2972
        fn isSingleton(&self) -> B;isSingleton71,3004
        fn collect<K: StT, V: StT>(collect72,3040
        fn iterate<A>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A;iterate76,3220
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce77,3305
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, Tscan78,3385
    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {ArraySeqStPerS81,3491
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::new(length, initnew82,3554
        fn length(&self) -> N { ArraySeqStPerS::length(self) }length83,3660
        fn nth(&self, index: N) -> &T { ArraySeqStPerS::nth(self, index) }nth84,3723
        fn empty() -> ArraySeqStPerS<T> { ArraySeqStPerS::empty() }empty85,3798
        fn singleton(item: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::singleton(item) }singleton86,3866
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStPerS<T> {tabulate88,3954
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U> {map96,4231
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T> {subseq_copy104,4539
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {append108,4681
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T> {filter120,5133
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {flatten131,5519
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject142,5912
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty154,6462
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton156,6550
        fn collect<K: StT, V: StT>(collect158,6642
        fn iterate<A>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A {iterate180,7573
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce188,7810
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, Tscan196,8040
    macro_rules! ArraySeqStPerS {ArraySeqStPerS208,8468
    fn _arrayseqstpers_macro_type_checks() {_arrayseqstpers_macro_type_checks221,8954

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,3631
pub mod Types;Types5,148
pub mod Chap03 {Chap037,164
    pub mod InsertionSortSt;InsertionSortSt8,181
pub mod Chap05 {Chap0511,213
    pub mod MappingStEph;MappingStEph12,230
    pub mod RelationStEph;RelationStEph13,256
    pub mod SetStEph;SetStEph14,283
pub mod Chap06 {Chap0617,308
    pub mod DirGraphMtEph;DirGraphMtEph18,325
    pub mod DirGraphStEph;DirGraphStEph19,352
    pub mod LabDirGraphMtEph;LabDirGraphMtEph20,379
    pub mod LabDirGraphStEph;LabDirGraphStEph21,409
    pub mod LabUnDirGraphMtEph;LabUnDirGraphMtEph22,439
    pub mod LabUnDirGraphStEph;LabUnDirGraphStEph23,471
    pub mod UnDirGraphMtEph;UnDirGraphMtEph24,503
    pub mod UnDirGraphStEph;UnDirGraphStEph25,532
    pub mod WeightedDirGraphMtEphFloat;WeightedDirGraphMtEphFloat26,561
    pub mod WeightedDirGraphMtEphInt;WeightedDirGraphMtEphInt27,601
    pub mod WeightedDirGraphStEphFloat;WeightedDirGraphStEphFloat28,639
    pub mod WeightedDirGraphStEphInt;WeightedDirGraphStEphInt29,679
    pub mod WeightedUnDirGraphMtEphFloat;WeightedUnDirGraphMtEphFloat30,717
    pub mod WeightedUnDirGraphMtEphInt;WeightedUnDirGraphMtEphInt31,759
    pub mod WeightedUnDirGraphStEphFloat;WeightedUnDirGraphStEphFloat32,799
    pub mod WeightedUnDirGraphStEphInt;WeightedUnDirGraphStEphInt33,841
pub mod Chap17 {Chap1736,884
    pub mod MathSeq;MathSeq37,901
pub mod Chap18 {Chap1840,925
    pub mod ArraySeq;ArraySeq41,942
    pub mod ArraySeqMtEph;ArraySeqMtEph42,964
    pub mod ArraySeqMtPer;ArraySeqMtPer43,991
    pub mod ArraySeqStEph;ArraySeqStEph44,1018
    pub mod ArraySeqStPer;ArraySeqStPer45,1045
    pub mod LinkedListStEph;LinkedListStEph47,1073
    pub mod LinkedListStPer;LinkedListStPer48,1102
pub mod Chap19 {Chap1951,1134
    pub mod ArraySeqMtEph;ArraySeqMtEph52,1151
    pub mod ArraySeqMtEphSlice;ArraySeqMtEphSlice53,1178
    pub mod ArraySeqMtPer;ArraySeqMtPer54,1210
    pub mod ArraySeqStEph;ArraySeqStEph55,1237
    pub mod ArraySeqStPer;ArraySeqStPer56,1264
pub mod Chap23 {Chap2359,1294
    pub mod BBTEph;BBTEph60,1311
    pub mod PrimTreeSeqSt;PrimTreeSeqSt61,1331
pub mod Chap36 {Chap3664,1361
    pub mod QuickSortMt;QuickSortMt65,1378
    pub mod QuickSortMtSlice;QuickSortMtSlice66,1403
    pub mod QuickSortSt;QuickSortSt67,1433
pub mod Chap37 {Chap3770,1461
    pub mod AVLTreeSeq;AVLTreeSeq71,1478
    pub mod AVLTreeSeqStEph;AVLTreeSeqStEph72,1502
    pub mod AVLTreeSeqStPer;AVLTreeSeqStPer73,1531
    pub mod BSTAVLMtEph;BSTAVLMtEph74,1560
    pub mod BSTAVLStEph;BSTAVLStEph75,1585
    pub mod BSTBBAlphaMtEph;BSTBBAlphaMtEph76,1610
    pub mod BSTBBAlphaStEph;BSTBBAlphaStEph77,1639
    pub mod BSTPlainMtEph;BSTPlainMtEph78,1668
    pub mod BSTPlainStEph;BSTPlainStEph79,1695
    pub mod BSTRBMtEph;BSTRBMtEph80,1722
    pub mod BSTRBStEph;BSTRBStEph81,1746
    pub mod BSTSetAVLMtEph;BSTSetAVLMtEph82,1770
    pub mod BSTSetBBAlphaMtEph;BSTSetBBAlphaMtEph83,1798
    pub mod BSTSetPlainMtEph;BSTSetPlainMtEph84,1830
    pub mod BSTSetRBMtEph;BSTSetRBMtEph85,1860
    pub mod BSTSetSplayMtEph;BSTSetSplayMtEph86,1887
    pub mod BSTSplayMtEph;BSTSplayMtEph87,1917
    pub mod BSTSplayStEph;BSTSplayStEph88,1944
pub mod Chap38 {Chap3891,1974
    pub mod BSTParaMtEph;BSTParaMtEph92,1991
    pub mod BSTParaStEph;BSTParaStEph93,2017
pub mod Chap39 {Chap3996,2046
    pub mod BSTParaTreapMtEph;BSTParaTreapMtEph97,2063
    pub mod BSTSetTreapMtEph;BSTSetTreapMtEph98,2094
    pub mod BSTTreapMtEph;BSTTreapMtEph99,2124
    pub mod BSTTreapStEph;BSTTreapStEph100,2151

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetAVLMtEph.rs,5590
pub mod BSTSetAVLMtEph {BSTSetAVLMtEph3,73
    pub struct BSTSetAVLMtEph<T: StTInMtT + Ord> {BSTSetAVLMtEph11,314
        tree: BSTAVLMtEph<T>,tree12,365
    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;BSTSetAVLMt15,402
    pub trait BSTSetAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetAVLMtEphTrait17,452
        fn empty() -> Self;empty18,514
        fn singleton(value: T) -> Self;singleton19,542
        fn size(&self) -> N;size20,582
        fn is_empty(&self) -> B;is_empty21,611
        fn find(&self, value: &T) -> Option<T>;find22,644
        fn contains(&self, value: &T) -> B;contains23,692
        fn minimum(&self) -> Option<T>;minimum24,736
        fn maximum(&self) -> Option<T>;maximum25,776
        fn insert(&mut self, value: T);insert26,816
        fn delete(&mut self, target: &T);delete27,856
        fn union(&self, other: &Self) -> Self;union28,898
        fn intersection(&self, other: &Self) -> Self;intersection29,945
        fn difference(&self, other: &Self) -> Self;difference30,999
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1051
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1106
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1161
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1223
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1293
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1361
        fn as_tree(&self) -> &BSTAVLMtEph<T>;as_tree37,1415
    impl<T: StTInMtT + Ord> BSTSetAVLMtEph<T> {BSTSetAVLMtEph40,1468
        pub fn empty() -> Self {empty41,1516
        pub fn singleton(value: T) -> Self {singleton47,1635
        pub fn size(&self) -> N { self.tree.size() }size53,1792
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1846
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1908
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,1986
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2064
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2132
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2200
        pub fn delete(&mut self, target: &T) {delete67,2273
        pub fn union(&self, other: &Self) -> Self {union75,2562
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2861
        pub fn difference(&self, other: &Self) -> Self {difference100,3439
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4016
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4708
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5021
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5377
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5786
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6050
        pub fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree179,6133
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6198
        fn rebuild_from_vec(values: Vec<T>) -> BSTAVLMtEph<T> {rebuild_from_vec183,6289
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6508
    impl<T: StTInMtT + Ord> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {BSTSetAVLMtEph203,6791
        fn empty() -> Self { Self::empty() }empty204,6866
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6912
        fn size(&self) -> N { self.tree.size() }size208,6979
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7029
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7087
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7161
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7235
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7299
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7363
        fn delete(&mut self, target: &T) { BSTSetAVLMtEph::delete(self, target) }delete222,7432
        fn union(&self, other: &Self) -> Self { BSTSetAVLMtEph::union(self, other) }union224,7515
        fn intersection(&self, other: &Self) -> Self { BSTSetAVLMtEph::intersection(self, other)intersection226,7601
        fn difference(&self, other: &Self) -> Self { BSTSetAVLMtEph::difference(self, other) }difference228,7701
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetAVLMtEph::split(self, pivot) }split230,7797
        fn join_pair(left: Self, right: Self) -> Self { BSTSetAVLMtEph::join_pair(left, right) }join_pair232,7891
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetAVLMtEph::join_m(left, pivojoin_m234,7989
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetAVLMtEph::filter(sefilter236,8098
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetAVLMtEph::reduce(selfreduce238,8212
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8323
        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree242,8402
    macro_rules! BSTSetAVLMtEphLit {BSTSetAVLMtEphLit246,8489
    fn _BSTSetAVLMtEphLit_type_checks() {_BSTSetAVLMtEphLit_type_checks258,9036

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStPer.rs,4232
pub mod AVLTreeSeqStPer {AVLTreeSeqStPer3,85
    type Link<T> = Option<Rc<Node<T>>>;Link10,247
    struct Node<T: StT> {Node12,288
        value: T,value13,314
        height: N,height14,332
        size: N,size15,351
        left: Link<T>,left16,368
        right: Link<T>,right17,391
    fn height<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.height) }height20,422
    fn size<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.size) }size21,501
    fn mk<T: StT>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {mk23,577
    fn rotate_right<T: StT>(y: Rc<Node<T>>) -> Rc<Node<T>> {rotate_right36,922
    fn rotate_left<T: StT>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left43,1229
    fn rebalance<T: StT>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance50,1535
    fn nth_ref<'a, T: StT>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref72,2388
    fn set_rec<T: StT>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> {set_rec87,2837
    fn inorder_collect<T: StT>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect105,3609
    fn build_balanced_from_slice<T: StT>(a: &[T]) -> Link<T> {build_balanced_from_slice113,3849
        fn rec<T: StT>(a: &[T]) -> Link<T> {rec114,3912
    pub struct AVLTreeSeqStPerS<T: StT> {AVLTreeSeqStPerS126,4230
        root: Link<T>,root127,4272
    pub trait AVLTreeSeqStPerTrait<T: StT> {AVLTreeSeqStPerTrait130,4302
        fn empty() -> Self;empty132,4388
        fn new() -> Self;new134,4457
        fn length(&self) -> N;length136,4524
        fn nth(&self, index: N) -> &T;nth138,4604
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set140,4753
        fn singleton(item: T) -> Self;singleton144,4904
        fn isEmpty(&self) -> B;isEmpty146,4984
        fn isSingleton(&self) -> B;isSingleton148,5057
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy150,5150
        fn from_vec(values: Vec<T>) -> Self;from_vec152,5273
        fn values_in_order(&self) -> Vec<T>;values_in_order154,5362
    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS157,5414
        fn empty() -> Self { AVLTreeSeqStPerS { root: None } }empty158,5481
        fn new() -> Self { Self::empty() }new159,5544
        fn length(&self) -> N { size(&self.root) }length160,5587
        fn nth(&self, index: N) -> &T { nth_ref(&self.root, index) }nth161,5638
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set162,5707
        fn singleton(item: T) -> Self {singleton167,5896
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty172,6041
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton173,6128
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy174,6219
        fn from_vec(values: Vec<T>) -> Self {from_vec188,6727
        fn values_in_order(&self) -> Vec<T> {values_in_order193,6890
    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS200,7081
        fn eq(&self, other: &Self) -> bool {eq201,7134
    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}AVLTreeSeqStPerS213,7460
    impl<T: StT> std::fmt::Debug for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS215,7508
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt216,7567
    impl<T: StT> AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS222,7759
        pub fn to_arrayseq(&self) -> ArraySeqStPerS<T> {to_arrayseq223,7798
        pub fn iter<'a>(&'a self) -> AVLTreeSeqStPerIter<'a, T> {iter228,7950
    pub struct AVLTreeSeqStPerIter<'a, T: StT> {AVLTreeSeqStPerIter236,8163
        stack: Vec<&'a Node<T>>,stack237,8212
        current: Option<&'a Node<T>>,current238,8245
    impl<'a, T: StT> AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter241,8290
        fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {push_left242,8340
    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter250,8550
        type Item = &'a T;Item251,8613
        fn next(&mut self) -> Option<Self::Item> {next252,8640
macro_rules! AVLTreeSeqStPer {AVLTreeSeqStPer266,9030

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetPlainMtEph.rs,5277
pub mod BSTSetPlainMtEph {BSTSetPlainMtEph3,75
    pub struct BSTSetPlainMtEph<T: StTInMtT + Ord> {BSTSetPlainMtEph11,324
        tree: BSTPlainMtEph<T>,tree12,377
    pub type BSTSetPlainMt<T> = BSTSetPlainMtEph<T>;BSTSetPlainMt15,416
    pub trait BSTSetPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetPlainMtEphTrait17,470
        fn empty() -> Self;empty18,534
        fn singleton(value: T) -> Self;singleton19,562
        fn size(&self) -> N;size20,602
        fn is_empty(&self) -> B;is_empty21,631
        fn find(&self, value: &T) -> Option<T>;find22,664
        fn contains(&self, value: &T) -> B;contains23,712
        fn minimum(&self) -> Option<T>;minimum24,756
        fn maximum(&self) -> Option<T>;maximum25,796
        fn insert(&mut self, value: T);insert26,836
        fn delete(&mut self, target: &T);delete27,876
        fn union(&self, other: &Self) -> Self;union28,918
        fn intersection(&self, other: &Self) -> Self;intersection29,965
        fn difference(&self, other: &Self) -> Self;difference30,1019
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1071
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1126
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1181
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1243
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1313
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1381
        fn as_tree(&self) -> &BSTPlainMtEph<T>;as_tree37,1435
    impl<T: StTInMtT + Ord> BSTSetPlainMtEph<T> {BSTSetPlainMtEph40,1490
        pub fn empty() -> Self {empty41,1540
        pub fn singleton(value: T) -> Self {singleton47,1661
        pub fn size(&self) -> N { self.tree.size() }size53,1820
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1874
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1936
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,2014
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2092
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2160
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2228
        pub fn delete(&mut self, target: &T) {delete67,2301
        pub fn union(&self, other: &Self) -> Self {union75,2590
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2889
        pub fn difference(&self, other: &Self) -> Self {difference100,3467
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4044
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4736
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5049
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5405
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5814
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6078
        pub fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree179,6161
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6228
        fn rebuild_from_vec(values: Vec<T>) -> BSTPlainMtEph<T> {rebuild_from_vec183,6319
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6542
    impl<T: StTInMtT + Ord> BSTSetPlainMtEphTrait<T> for BSTSetPlainMtEph<T> {BSTSetPlainMtEph203,6827
        fn empty() -> Self { Self::empty() }empty204,6906
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6952
        fn size(&self) -> N { self.tree.size() }size208,7019
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7069
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7127
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7201
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7275
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7339
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7403
        fn delete(&mut self, target: &T) {delete222,7472
        fn union(&self, other: &Self) -> Self {union230,7757
        fn intersection(&self, other: &Self) -> Self {intersection238,8052
        fn difference(&self, other: &Self) -> Self {difference255,8626
        fn split(&self, pivot: &T) -> (Self, B, Self) {split272,9199
        fn join_pair(left: Self, right: Self) -> Self {join_pair292,9887
        fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m300,10196
        fn filter<F>(&self, predicate: F) -> Selffilter309,10548
        fn reduce<F>(&self, op: F, base: T) -> Treduce316,10709
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order323,10867
        fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree325,10946
    macro_rules! BSTSetPlainMtEphLit {BSTSetPlainMtEphLit329,11035
    fn _BSTSetPlainMtEphLit_type_checks() {_BSTSetPlainMtEphLit_type_checks341,11608

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBStEph.rs,4730
pub mod BSTRBStEph {BSTRBStEph3,93
    enum Color {Color9,301
        Red,Red10,318
        Black,Black11,331
    type Link<T> = Option<Box<Node<T>>>;Link14,353
    struct Node<T: StT + Ord> {Node17,423
        key: T,key18,455
        color: Color,color19,471
        size: N,size20,493
        left: Link<T>,left21,510
        right: Link<T>,right22,533
    impl<T: StT + Ord> Node<T> {Node25,564
        fn new(key: T) -> Self {new26,597
    pub struct BSTRBStEph<T: StT + Ord> {BSTRBStEph38,846
        root: Link<T>,root39,888
    pub type BSTreeRB<T> = BSTRBStEph<T>;BSTreeRB42,918
    pub trait BSTRBStEphTrait<T: StT + Ord> {BSTRBStEphTrait44,961
        fn new() -> Self;new45,1007
        fn size(&self) -> N;size46,1033
        fn is_empty(&self) -> B;is_empty47,1062
        fn height(&self) -> N;height48,1095
        fn insert(&mut self, value: T);insert49,1126
        fn find(&self, target: &T) -> Option<&T>;find50,1166
        fn contains(&self, target: &T) -> B;contains51,1216
        fn minimum(&self) -> Option<&T>;minimum52,1261
        fn maximum(&self) -> Option<&T>;maximum53,1302
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order54,1343
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order55,1392
    impl<T: StT + Ord> Default for BSTRBStEph<T> {BSTRBStEph58,1449
        fn default() -> Self { Self::new() }default59,1500
    impl<T: StT + Ord> BSTRBStEph<T> {BSTRBStEph62,1552
        pub fn new() -> Self { BSTRBStEph { root: None } }new63,1591
        pub fn size(&self) -> N { Self::size_link(&self.root) }size65,1651
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty67,1716
        pub fn height(&self) -> N {height69,1807
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec70,1843
        pub fn insert(&mut self, value: T) {insert79,2139
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find86,2359
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains88,2453
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum90,2567
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum92,2643
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order94,2719
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order100,2943
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red106,3169
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link108,3271
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate110,3354
        fn rotate_left(link: &mut Link<T>) {rotate_left112,3473
        fn rotate_right(link: &mut Link<T>) {rotate_right129,4069
        fn flip_colors(link: &mut Link<T>) {flip_colors146,4667
        fn fix_up(link: &mut Link<T>) {fix_up167,5472
        fn insert_link(link: &mut Link<T>, value: T) {insert_link184,6159
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link200,6701
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link215,7218
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link225,7539
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect235,7862
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect243,8149
    impl<T: StT + Ord> BSTRBStEphTrait<T> for BSTRBStEph<T> {BSTRBStEph252,8445
        fn new() -> Self { BSTRBStEph::new() }new253,8507
        fn size(&self) -> N { BSTRBStEph::size(self) }size255,8555
        fn is_empty(&self) -> B { BSTRBStEph::is_empty(self) }is_empty257,8611
        fn height(&self) -> N { BSTRBStEph::height(self) }height259,8675
        fn insert(&mut self, value: T) { BSTRBStEph::insert(self, value) }insert261,8735
        fn find(&self, target: &T) -> Option<&T> { BSTRBStEph::find(self, target) }find263,8811
        fn contains(&self, target: &T) -> B { BSTRBStEph::contains(self, target) }contains265,8896
        fn minimum(&self) -> Option<&T> { BSTRBStEph::minimum(self) }minimum267,8980
        fn maximum(&self) -> Option<&T> { BSTRBStEph::maximum(self) }maximum269,9051
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTRBStEph::in_order(self) }in_order271,9122
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTRBStEph::pre_order(self) }pre_order273,9202
    macro_rules! BSTRBStEphLit {BSTRBStEphLit277,9310
    fn _BSTRBStEphLit_type_checks() {_BSTRBStEphLit_type_checks289,9804

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeq.rs,5331
pub mod AVLTreeSeq {AVLTreeSeq9,536
    type Link<T> = Option<Box<AVLTreeNode<T>>>;Link15,666
    pub struct AVLTreeNode<T: Copy + Debug> {AVLTreeNode17,715
        pub value: T,value18,761
        pub height: N,height19,783
        pub left_size: N,left_size20,806
        pub right_size: N,right_size21,832
        pub left: Link<T>,left22,859
        pub right: Link<T>,right23,886
        pub index: N,index24,914
    impl<T: Copy + Debug> AVLTreeNode<T> {AVLTreeNode27,943
        fn new(value: T, index: N) -> Self {new28,986
    pub struct AVLTreeS<T: Copy + Debug> {AVLTreeS41,1279
        pub root: Link<T>,root42,1322
        pub next_key: N,next_key43,1349
    pub trait AVLTreeSeq<T: Copy + Debug> {AVLTreeSeq46,1381
        fn empty() -> AVLTreeS<T>;empty49,1504
        fn new() -> AVLTreeS<T>;new53,1627
        fn length(&self) -> N;length57,1742
        fn nth(&self, index: N) -> &T;nth61,1916
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str>;set65,2065
        fn singleton(item: T) -> AVLTreeS<T>;singleton69,2240
        fn isEmpty(&self) -> B;isEmpty73,2353
        fn isSingleton(&self) -> B;isSingleton74,2385
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>subseq_copy78,2552
    impl<T: Copy + Debug> AVLTreeS<T> {AVLTreeS83,2666
        pub fn new_root() -> Self {new_root84,2706
        pub fn new() -> Self { Self::new_root() }new90,2846
        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeS<T> {update92,2897
        pub fn from_vec(values: Vec<T>) -> AVLTreeS<T>from_vec97,3079
        pub fn to_arrayseq(&self) -> ArrayS<T>to_arrayseq110,3491
        pub fn iter<'a>(&'a self) -> AVLTreeSeqIter<'a, T> { AVLTreeSeqIter::new(&self.root) }iter129,4093
        pub fn push_back(&mut self, value: T) {push_back131,4189
        pub fn contains_value(&self, target: &T) -> Bcontains_value138,4497
        pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value150,4769
        pub fn delete_value(&mut self, target: &T) -> booldelete_value152,4846
        pub fn is_tree_empty(&self) -> bool { self.length() == 0 }is_tree_empty180,5790
        pub fn values_in_order(&self) -> Vec<T>values_in_order182,5858
    impl<T: Copy + Debug> AVLTreeSeq<T> for AVLTreeS<T> {AVLTreeS192,6094
        fn empty() -> AVLTreeS<T> { AVLTreeS::new_root() }empty194,6194
        fn new() -> AVLTreeS<T> { AVLTreeS::new_root() }new197,6296
        fn length(&self) -> N { size_link(&self.root) }length200,6396
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth203,6503
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str> {set206,6624
        fn singleton(item: T) -> AVLTreeS<T> {singleton212,6839
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty219,7077
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton221,7206
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>subseq_copy224,7360
    impl<T: Eq + Copy + Debug> PartialEq for AVLTreeS<T> {AVLTreeS242,7911
        fn eq(&self, other: &Self) -> bool {eq243,7970
    impl<T: Eq + Copy + Debug> Eq for AVLTreeS<T> {}AVLTreeS256,8297
    impl<T: Debug + Copy> std::fmt::Debug for AVLTreeS<T> {AVLTreeS258,8351
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt259,8411
    impl<T: std::fmt::Display + Copy + Debug> std::fmt::Display for AVLTreeS<T> {AVLTreeS265,8619
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt266,8701
    pub struct AVLTreeSeqIter<'a, T: Copy + Debug> {AVLTreeSeqIter282,9113
        stack: Vec<&'a AVLTreeNode<T>>,stack283,9166
        current: Option<&'a AVLTreeNode<T>>,current284,9206
    impl<'a, T: Copy + Debug> AVLTreeSeqIter<'a, T> {AVLTreeSeqIter287,9258
        fn new(root: &'a Link<T>) -> Self {new288,9312
        fn push_left(&mut self, link: &'a Link<T>) {push_left297,9537
    impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIter<'a, T> {AVLTreeSeqIter306,9789
        type Item = &'a T;Item307,9856
        fn next(&mut self) -> Option<Self::Item> {next308,9883
    fn h<T: Copy + Debug>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h318,10142
    fn size_link<T: Copy + Debug>(n: &Link<T>) -> N {size_link319,10225
    fn update_meta<T: Copy + Debug>(n: &mut Box<AVLTreeNode<T>>) {update_meta327,10399
    fn rotate_right<T: Copy + Debug>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right335,10653
    fn rotate_left<T: Copy + Debug>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left348,11027
    fn rebalance<T: Copy + Debug>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance361,11399
    pub(crate) fn insert_at_link<T: Copy + Debug>(node: Link<T>, index: N, value: T, next_key: &insert_at_link385,12299
    fn nth_link<'a, T: Copy + Debug>(node: &'a Link<T>, index: N) -> &'a T {nth_link405,13104
    fn set_link<T: Copy + Debug>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static set_link417,13493
    fn push_inorder<T: Copy + Debug>(link: &Link<T>, out: &mut Vec<T>)push_inorder434,14092

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetSplayMtEph.rs,5645
pub mod BSTSetSplayMtEph {BSTSetSplayMtEph3,75
    pub struct BSTSetSplayMtEph<T: StTInMtT + Ord> {BSTSetSplayMtEph11,324
        tree: BSTSplayMtEph<T>,tree12,377
    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;BSTSetSplayMt15,416
    pub trait BSTSetSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetSplayMtEphTrait17,470
        fn empty() -> Self;empty18,534
        fn singleton(value: T) -> Self;singleton19,562
        fn size(&self) -> N;size20,602
        fn is_empty(&self) -> B;is_empty21,631
        fn find(&self, value: &T) -> Option<T>;find22,664
        fn contains(&self, value: &T) -> B;contains23,712
        fn minimum(&self) -> Option<T>;minimum24,756
        fn maximum(&self) -> Option<T>;maximum25,796
        fn insert(&mut self, value: T);insert26,836
        fn delete(&mut self, target: &T);delete27,876
        fn union(&self, other: &Self) -> Self;union28,918
        fn intersection(&self, other: &Self) -> Self;intersection29,965
        fn difference(&self, other: &Self) -> Self;difference30,1019
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1071
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1126
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1181
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1243
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1313
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1381
        fn as_tree(&self) -> &BSTSplayMtEph<T>;as_tree37,1435
    impl<T: StTInMtT + Ord> BSTSetSplayMtEph<T> {BSTSetSplayMtEph40,1490
        pub fn empty() -> Self {empty41,1540
        pub fn singleton(value: T) -> Self {singleton47,1661
        pub fn size(&self) -> N { self.tree.size() }size53,1820
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1874
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1936
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,2014
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2092
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2160
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2228
        pub fn delete(&mut self, target: &T) {delete67,2301
        pub fn union(&self, other: &Self) -> Self {union75,2590
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2889
        pub fn difference(&self, other: &Self) -> Self {difference100,3467
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4044
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4736
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5049
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5405
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5814
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6078
        pub fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree179,6161
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6228
        fn rebuild_from_vec(values: Vec<T>) -> BSTSplayMtEph<T> {rebuild_from_vec183,6319
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6542
    impl<T: StTInMtT + Ord> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {BSTSetSplayMtEph203,6827
        fn empty() -> Self { Self::empty() }empty204,6906
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6952
        fn size(&self) -> N { self.tree.size() }size208,7019
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7069
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7127
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7201
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7275
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7339
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7403
        fn delete(&mut self, target: &T) { BSTSetSplayMtEph::delete(self, target) }delete222,7472
        fn union(&self, other: &Self) -> Self { BSTSetSplayMtEph::union(self, other) }union224,7557
        fn intersection(&self, other: &Self) -> Self { BSTSetSplayMtEph::intersection(self, otheintersection226,7645
        fn difference(&self, other: &Self) -> Self { BSTSetSplayMtEph::difference(self, other) }difference228,7747
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetSplayMtEph::split(self, pivot) }split230,7845
        fn join_pair(left: Self, right: Self) -> Self { BSTSetSplayMtEph::join_pair(left, right)join_pair232,7941
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetSplayMtEph::join_m(left, pijoin_m234,8041
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetSplayMtEph::filter(filter236,8152
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetSplayMtEph::reduce(sereduce238,8268
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8381
        fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree242,8460
    macro_rules! BSTSetSplayMtEphLit {BSTSetSplayMtEphLit246,8549
    fn _BSTSetSplayMtEphLit_type_checks() {_BSTSetSplayMtEphLit_type_checks258,9122

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaMtEph.rs,4630
pub mod BSTBBAlphaMtEph {BSTBBAlphaMtEph3,108
    type Link<T> = Option<Box<Node<T>>>;Link12,344
    struct Node<T: StTInMtT + Ord> {Node15,407
        key: T,key16,444
        size: N,size17,460
        left: Link<T>,left18,477
        right: Link<T>,right19,500
    impl<T: StTInMtT + Ord> Node<T> {Node22,531
        fn new(key: T) -> Self {new23,569
    pub struct BSTBBAlphaMtEph<T: StTInMtT + Ord> {BSTBBAlphaMtEph34,783
        root: Arc<RwLock<Link<T>>>,root35,835
    pub type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;BSTreeBBAlpha38,878
    pub trait BSTBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTBBAlphaMtEphTrait40,931
        fn new() -> Self;new41,994
        fn insert(&self, value: T);insert42,1020
        fn find(&self, target: &T) -> Option<T>;find43,1056
        fn contains(&self, target: &T) -> B;contains44,1105
        fn size(&self) -> N;size45,1150
        fn is_empty(&self) -> B;is_empty46,1179
        fn height(&self) -> N;height47,1212
        fn minimum(&self) -> Option<T>;minimum48,1243
        fn maximum(&self) -> Option<T>;maximum49,1283
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order50,1323
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order51,1372
    impl<T: StTInMtT + Ord> Default for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph54,1429
        fn default() -> Self { Self::new() }default55,1490
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph58,1542
        pub fn new() -> Self {new59,1591
        pub fn size(&self) -> N {size65,1728
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty70,1861
        pub fn height(&self) -> N {height72,1952
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec73,1988
        pub fn insert(&self, value: T) {insert84,2338
        pub fn find(&self, target: &T) -> Option<T> {find93,2669
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains98,2839
        pub fn minimum(&self) -> Option<T> {minimum100,2953
        pub fn maximum(&self) -> Option<T> {maximum105,3105
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order110,3257
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order117,3542
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link124,3829
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate126,3912
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link128,4031
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild150,4777
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed157,5059
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values167,5469
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced175,5750
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link187,6190
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link202,6707
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link212,7028
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect222,7351
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect230,7638
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph239,7934
        fn new() -> Self { BSTBBAlphaMtEph::new() }new240,8011
        fn insert(&self, value: T) { BSTBBAlphaMtEph::insert(self, value) }insert242,8064
        fn find(&self, target: &T) -> Option<T> { BSTBBAlphaMtEph::find(self, target) }find244,8141
        fn contains(&self, target: &T) -> B { BSTBBAlphaMtEph::contains(self, target) }contains246,8230
        fn size(&self) -> N { BSTBBAlphaMtEph::size(self) }size248,8319
        fn is_empty(&self) -> B { BSTBBAlphaMtEph::is_empty(self) }is_empty250,8380
        fn height(&self) -> N { BSTBBAlphaMtEph::height(self) }height252,8449
        fn minimum(&self) -> Option<T> { BSTBBAlphaMtEph::minimum(self) }minimum254,8514
        fn maximum(&self) -> Option<T> { BSTBBAlphaMtEph::maximum(self) }maximum256,8589
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaMtEph::in_order(self) }in_order258,8664
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaMtEph::pre_order(self) }pre_order260,8749
    macro_rules! BSTBBAlphaMtEphLit {BSTBBAlphaMtEphLit264,8862
    fn _BSTBBAlphaMtEphLit_type_checks() {_BSTBBAlphaMtEphLit_type_checks276,9417

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLStEph.rs,4549
pub mod BSTAVLStEph {BSTAVLStEph3,97
    type Link<T> = Option<Box<Node<T>>>;Link8,264
    struct Node<T: StT + Ord> {Node11,327
        key: T,key12,359
        height: i32,height13,375
        size: N,size14,396
        left: Link<T>,left15,413
        right: Link<T>,right16,436
    impl<T: StT + Ord> Node<T> {Node19,467
        fn new(key: T) -> Self {new20,500
    pub struct BSTAVLStEph<T: StT + Ord> {BSTAVLStEph32,741
        root: Link<T>,root33,784
    pub type BSTreeAVL<T> = BSTAVLStEph<T>;BSTreeAVL36,814
    pub trait BSTAVLStEphTrait<T: StT + Ord> {BSTAVLStEphTrait38,859
        fn new() -> Self;new39,906
        fn size(&self) -> N;size40,932
        fn is_empty(&self) -> B;is_empty41,961
        fn height(&self) -> N;height42,994
        fn insert(&mut self, value: T);insert43,1025
        fn find(&self, target: &T) -> Option<&T>;find44,1065
        fn contains(&self, target: &T) -> B;contains45,1115
        fn minimum(&self) -> Option<&T>;minimum46,1160
        fn maximum(&self) -> Option<&T>;maximum47,1201
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order48,1242
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order49,1291
    impl<T: StT + Ord> Default for BSTAVLStEph<T> {BSTAVLStEph52,1348
        fn default() -> Self { Self::new() }default53,1400
    impl<T: StT + Ord> BSTAVLStEph<T> {BSTAVLStEph56,1452
        pub fn new() -> Self { BSTAVLStEph { root: None } }new57,1492
        pub fn size(&self) -> N { Self::size_link(&self.root) }size59,1553
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty61,1618
        pub fn height(&self) -> N { Self::height_link(&self.root) as N }height63,1709
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert65,1783
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find67,1873
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains69,1967
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum71,2081
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum73,2157
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order75,2233
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order81,2457
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link87,2683
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link89,2772
        fn update(node: &mut Node<T>) {update91,2855
        fn rotate_right(link: &mut Link<T>) {rotate_right96,3091
        fn rotate_left(link: &mut Link<T>) {rotate_left110,3547
        fn rebalance(link: &mut Link<T>) {rebalance124,4002
        fn insert_link(link: &mut Link<T>, value: T) {insert_link149,5048
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link168,5683
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link183,6200
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link193,6521
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect203,6844
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect211,7131
    impl<T: StT + Ord> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {BSTAVLStEph220,7427
        fn new() -> Self { BSTAVLStEph::new() }new221,7491
        fn size(&self) -> N { BSTAVLStEph::size(self) }size223,7540
        fn is_empty(&self) -> B { BSTAVLStEph::is_empty(self) }is_empty225,7597
        fn height(&self) -> N { BSTAVLStEph::height(self) }height227,7662
        fn insert(&mut self, value: T) { BSTAVLStEph::insert(self, value) }insert229,7723
        fn find(&self, target: &T) -> Option<&T> { BSTAVLStEph::find(self, target) }find231,7800
        fn contains(&self, target: &T) -> B { BSTAVLStEph::contains(self, target) }contains233,7886
        fn minimum(&self) -> Option<&T> { BSTAVLStEph::minimum(self) }minimum235,7971
        fn maximum(&self) -> Option<&T> { BSTAVLStEph::maximum(self) }maximum237,8043
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::in_order(self) }in_order239,8115
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::pre_order(self) }pre_order241,8196
    macro_rules! BSTAVLStEphLit {BSTAVLStEphLit245,8305
    fn _BSTAVLStEphLit_type_checks() {_BSTAVLStEphLit_type_checks257,8812

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainMtEph.rs,3810
pub mod BSTPlainMtEph {BSTPlainMtEph3,90
    type Link<T> = Arc<RwLock<Option<Node<T>>>>;Link9,238
    struct Node<T: StTInMtT + Ord> {Node12,309
        key: T,key13,346
        height: i32,height14,362
        size: N,size15,383
        left: Link<T>,left16,400
        right: Link<T>,right17,423
    impl<T: StTInMtT + Ord> Node<T> {Node20,454
        fn new(key: T) -> Self {new21,492
        fn update(&mut self) {update31,745
    pub struct BSTPlainMtEph<T: StTInMtT + Ord> {BSTPlainMtEph40,1049
        root: Link<T>,root41,1099
    pub type BSTree<T> = BSTPlainMtEph<T>;BSTree44,1129
    pub trait BSTPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTPlainMtEphTrait46,1173
        fn new() -> Self;new47,1234
        fn insert(&self, value: T);insert48,1260
        fn find(&self, target: &T) -> Option<T>;find49,1296
        fn contains(&self, target: &T) -> B;contains50,1345
        fn size(&self) -> N;size51,1390
        fn is_empty(&self) -> B;is_empty52,1419
        fn height(&self) -> N;height53,1452
        fn minimum(&self) -> Option<T>;minimum54,1483
        fn maximum(&self) -> Option<T>;maximum55,1523
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order56,1563
    impl<T: StTInMtT + Ord> BSTPlainMtEph<T> {BSTPlainMtEph59,1619
        pub fn new() -> Self {new60,1666
        pub fn insert(&self, value: T) {insert66,1792
            fn descend<T: StTInMtT + Ord>(link: &Link<T>, value: T) -> bool {descend67,1833
        pub fn find(&self, target: &T) -> Option<T> {find101,3025
            fn find_rec<T: StTInMtT + Ord>(link: &Link<T>, target: &T) -> Option<T> {find_rec102,3079
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains121,3818
        pub fn size(&self) -> N {size122,3931
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty127,4082
        pub fn height(&self) -> N {height129,4173
        pub fn minimum(&self) -> Option<T> {minimum134,4333
            fn leftmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {leftmost135,4378
        pub fn maximum(&self) -> Option<T> {maximum156,5088
            fn rightmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {rightmost157,5133
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order178,5850
            fn traverse<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {traverse179,5904
    fn height_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> i32 { link.as_ref().map_or(0, |n|height_of198,6572
    fn size_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> N { link.as_ref().map_or(0, |n| n.ssize_of200,6682
    impl<T: StTInMtT + Ord> BSTPlainMtEphTrait<T> for BSTPlainMtEph<T> {BSTPlainMtEph202,6786
        fn new() -> Self { BSTPlainMtEph::new() }new203,6859
        fn insert(&self, value: T) { BSTPlainMtEph::insert(self, value) }insert204,6909
        fn find(&self, target: &T) -> Option<T> { BSTPlainMtEph::find(self, target) }find205,6983
        fn contains(&self, target: &T) -> B { BSTPlainMtEph::contains(self, target) }contains206,7069
        fn size(&self) -> N { BSTPlainMtEph::size(self) }size207,7155
        fn is_empty(&self) -> B { BSTPlainMtEph::is_empty(self) }is_empty208,7213
        fn height(&self) -> N { BSTPlainMtEph::height(self) }height209,7279
        fn minimum(&self) -> Option<T> { BSTPlainMtEph::minimum(self) }minimum210,7341
        fn maximum(&self) -> Option<T> { BSTPlainMtEph::maximum(self) }maximum211,7413
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTPlainMtEph::in_order(self) }in_order212,7485
    macro_rules! BSTPlainMtEphLit {BSTPlainMtEphLit216,7594
    fn _BSTPlainMtEphLit_type_checks() {_BSTPlainMtEphLit_type_checks231,8158

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaStEph.rs,4742
pub mod BSTBBAlphaStEph {BSTBBAlphaStEph3,80
    type Link<T> = Option<Box<Node<T>>>;Link10,281
    struct Node<T: StT + Ord> {Node13,351
        key: T,key14,383
        size: N,size15,399
        left: Link<T>,left16,416
        right: Link<T>,right17,439
    impl<T: StT + Ord> Node<T> {Node20,470
        fn new(key: T) -> Self {new21,503
    pub struct BSTBBAlphaStEph<T: StT + Ord> {BSTBBAlphaStEph32,717
        root: Link<T>,root33,764
    pub type BSTreeBBAlpha<T> = BSTBBAlphaStEph<T>;BSTreeBBAlpha36,794
    pub trait BSTBBAlphaStEphTrait<T: StT + Ord> {BSTBBAlphaStEphTrait38,847
        fn new() -> Self;new39,898
        fn size(&self) -> N;size40,924
        fn is_empty(&self) -> B;is_empty41,953
        fn height(&self) -> N;height42,986
        fn insert(&mut self, value: T);insert43,1017
        fn find(&self, target: &T) -> Option<&T>;find44,1057
        fn contains(&self, target: &T) -> B;contains45,1107
        fn minimum(&self) -> Option<&T>;minimum46,1152
        fn maximum(&self) -> Option<&T>;maximum47,1193
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order48,1234
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order49,1283
    impl<T: StT + Ord> Default for BSTBBAlphaStEph<T> {BSTBBAlphaStEph52,1340
        fn default() -> Self { Self::new() }default53,1396
    impl<T: StT + Ord> BSTBBAlphaStEph<T> {BSTBBAlphaStEph56,1448
        pub fn new() -> Self { BSTBBAlphaStEph { root: None } }new57,1492
        pub fn size(&self) -> N { Self::size_link(&self.root) }size59,1557
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty61,1622
        pub fn height(&self) -> N {height63,1713
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec64,1749
        pub fn insert(&mut self, value: T) {insert73,2045
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find81,2333
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains83,2427
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum85,2541
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum87,2617
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order89,2693
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order95,2917
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link101,3143
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate103,3226
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link105,3345
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild127,4091
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed134,4373
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values144,4783
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced152,5064
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link164,5504
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link179,6021
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link189,6342
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect199,6665
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect207,6952
    impl<T: StT + Ord> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {BSTBBAlphaStEph216,7248
        fn new() -> Self { BSTBBAlphaStEph::new() }new217,7320
        fn size(&self) -> N { BSTBBAlphaStEph::size(self) }size219,7373
        fn is_empty(&self) -> B { BSTBBAlphaStEph::is_empty(self) }is_empty221,7434
        fn height(&self) -> N { BSTBBAlphaStEph::height(self) }height223,7503
        fn insert(&mut self, value: T) { BSTBBAlphaStEph::insert(self, value) }insert225,7568
        fn find(&self, target: &T) -> Option<&T> { BSTBBAlphaStEph::find(self, target) }find227,7649
        fn contains(&self, target: &T) -> B { BSTBBAlphaStEph::contains(self, target) }contains229,7739
        fn minimum(&self) -> Option<&T> { BSTBBAlphaStEph::minimum(self) }minimum231,7828
        fn maximum(&self) -> Option<&T> { BSTBBAlphaStEph::maximum(self) }maximum233,7904
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaStEph::in_order(self) }in_order235,7980
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaStEph::pre_order(self) }pre_order237,8065
    macro_rules! BSTBBAlphaStEphLit {BSTBBAlphaStEphLit241,8178
    fn _BSTBBAlphaStEphLit_type_checks() {_BSTBBAlphaStEphLit_type_checks253,8737

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayMtEph.rs,4230
pub mod BSTSplayMtEph {BSTSplayMtEph3,99
    type Link<T> = Option<Box<Node<T>>>;Link10,303
    struct Node<T: StTInMtT + Ord> {Node13,373
        key: T,key14,410
        size: N,size15,426
        left: Link<T>,left16,443
        right: Link<T>,right17,466
    impl<T: StTInMtT + Ord> Node<T> {Node20,497
        fn new(key: T) -> Self {new21,535
    pub struct BSTSplayMtEph<T: StTInMtT + Ord> {BSTSplayMtEph32,749
        root: Arc<RwLock<Link<T>>>,root33,799
    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;BSTreeSplay36,842
    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSplayMtEphTrait38,891
        fn new() -> Self;new39,952
        fn insert(&self, value: T);insert40,978
        fn find(&self, target: &T) -> Option<T>;find41,1014
        fn contains(&self, target: &T) -> B;contains42,1063
        fn size(&self) -> N;size43,1108
        fn is_empty(&self) -> B;is_empty44,1137
        fn height(&self) -> N;height45,1170
        fn minimum(&self) -> Option<T>;minimum46,1201
        fn maximum(&self) -> Option<T>;maximum47,1241
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order48,1281
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order49,1330
    impl<T: StTInMtT + Ord> Default for BSTSplayMtEph<T> {BSTSplayMtEph52,1387
        fn default() -> Self { Self::new() }default53,1446
    impl<T: StTInMtT + Ord> BSTSplayMtEph<T> {BSTSplayMtEph56,1498
        pub fn new() -> Self {new57,1545
        pub fn size(&self) -> N {size63,1680
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty68,1813
        pub fn height(&self) -> N {height70,1904
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec71,1940
        pub fn insert(&self, value: T) {insert82,2290
        pub fn find(&self, target: &T) -> Option<T> {find87,2449
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains92,2619
        pub fn minimum(&self) -> Option<T> {minimum94,2733
        pub fn maximum(&self) -> Option<T> {maximum99,2885
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order104,3037
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order111,3322
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link118,3609
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate120,3692
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link122,3811
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link144,4557
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link159,5074
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link169,5395
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect179,5718
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect187,6005
    impl<T: StTInMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {BSTSplayMtEph196,6301
        fn new() -> Self { BSTSplayMtEph::new() }new197,6374
        fn insert(&self, value: T) { BSTSplayMtEph::insert(self, value) }insert199,6425
        fn find(&self, target: &T) -> Option<T> { BSTSplayMtEph::find(self, target) }find201,6500
        fn contains(&self, target: &T) -> B { BSTSplayMtEph::contains(self, target) }contains203,6587
        fn size(&self) -> N { BSTSplayMtEph::size(self) }size205,6674
        fn is_empty(&self) -> B { BSTSplayMtEph::is_empty(self) }is_empty207,6733
        fn height(&self) -> N { BSTSplayMtEph::height(self) }height209,6800
        fn minimum(&self) -> Option<T> { BSTSplayMtEph::minimum(self) }minimum211,6863
        fn maximum(&self) -> Option<T> { BSTSplayMtEph::maximum(self) }maximum213,6936
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTSplayMtEph::in_order(self) }in_order215,7009
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTSplayMtEph::pre_order(self) }pre_order217,7092
    macro_rules! BSTSplayMtEphLit {BSTSplayMtEphLit221,7203
    fn _BSTSplayMtEphLit_type_checks() {_BSTSplayMtEphLit_type_checks233,7732

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetRBMtEph.rs,5562
pub mod BSTSetRBMtEph {BSTSetRBMtEph3,79
    pub struct BSTSetRBMtEph<T: StTInMtT + Ord> {BSTSetRBMtEph11,316
        tree: BSTRBMtEph<T>,tree12,366
    pub type BSTSetRBMt<T> = BSTSetRBMtEph<T>;BSTSetRBMt15,402
    pub trait BSTSetRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetRBMtEphTrait17,450
        fn empty() -> Self;empty18,511
        fn singleton(value: T) -> Self;singleton19,539
        fn size(&self) -> N;size20,579
        fn is_empty(&self) -> B;is_empty21,608
        fn find(&self, value: &T) -> Option<T>;find22,641
        fn contains(&self, value: &T) -> B;contains23,689
        fn minimum(&self) -> Option<T>;minimum24,733
        fn maximum(&self) -> Option<T>;maximum25,773
        fn insert(&mut self, value: T);insert26,813
        fn delete(&mut self, target: &T);delete27,853
        fn union(&self, other: &Self) -> Self;union28,895
        fn intersection(&self, other: &Self) -> Self;intersection29,942
        fn difference(&self, other: &Self) -> Self;difference30,996
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1048
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1103
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1158
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1220
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1290
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1358
        fn as_tree(&self) -> &BSTRBMtEph<T>;as_tree37,1412
    impl<T: StTInMtT + Ord> BSTSetRBMtEph<T> {BSTSetRBMtEph40,1464
        pub fn empty() -> Self {empty41,1511
        pub fn singleton(value: T) -> Self {singleton47,1629
        pub fn size(&self) -> N { self.tree.size() }size53,1785
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1839
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1901
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,1979
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2057
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2125
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2193
        pub fn delete(&mut self, target: &T) {delete67,2266
        pub fn union(&self, other: &Self) -> Self {union75,2555
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2854
        pub fn difference(&self, other: &Self) -> Self {difference100,3432
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4009
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4701
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5014
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5370
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5779
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6043
        pub fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree179,6126
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6190
        fn rebuild_from_vec(values: Vec<T>) -> BSTRBMtEph<T> {rebuild_from_vec183,6281
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6498
    impl<T: StTInMtT + Ord> BSTSetRBMtEphTrait<T> for BSTSetRBMtEph<T> {BSTSetRBMtEph203,6780
        fn empty() -> Self { Self::empty() }empty204,6853
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6899
        fn size(&self) -> N { self.tree.size() }size208,6966
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7016
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7074
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7148
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7222
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7286
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7350
        fn delete(&mut self, target: &T) { BSTSetRBMtEph::delete(self, target) }delete222,7419
        fn union(&self, other: &Self) -> Self { BSTSetRBMtEph::union(self, other) }union224,7501
        fn intersection(&self, other: &Self) -> Self { BSTSetRBMtEph::intersection(self, other) intersection226,7586
        fn difference(&self, other: &Self) -> Self { BSTSetRBMtEph::difference(self, other) }difference228,7685
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetRBMtEph::split(self, pivot) }split230,7780
        fn join_pair(left: Self, right: Self) -> Self { BSTSetRBMtEph::join_pair(left, right) }join_pair232,7873
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetRBMtEph::join_m(left, pivotjoin_m234,7970
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetRBMtEph::filter(selfilter236,8078
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetRBMtEph::reduce(self,reduce238,8191
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8301
        fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree242,8380
    macro_rules! BSTSetRBMtEphLit {BSTSetRBMtEphLit246,8466
    fn _BSTSetRBMtEphLit_type_checks() {_BSTSetRBMtEphLit_type_checks258,9000

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStEph.rs,4720
pub mod AVLTreeSeqStEph {AVLTreeSeqStEph3,61
    type Link<T> = Option<Box<AVLTreeNode<T>>>;Link9,202
    pub struct AVLTreeNode<T: StT> {AVLTreeNode11,251
        pub value: T,value12,288
        pub height: N,height13,310
        pub left_size: N,left_size14,333
        pub right_size: N,right_size15,359
        pub left: Link<T>,left16,386
        pub right: Link<T>,right17,413
        pub index: N,index18,441
    impl<T: StT> AVLTreeNode<T> {AVLTreeNode21,470
        fn new(value: T, index: N) -> Self {new22,504
    pub struct AVLTreeSeqStEphS<T: StT> {AVLTreeSeqStEphS35,797
        pub root: Link<T>,root36,839
        pub next_key: N,next_key37,866
    pub trait AVLTreeSeqStEphTrait<T: StT> {AVLTreeSeqStEphTrait40,898
        fn empty() -> Self;empty42,985
        fn new() -> Self;new44,1055
        fn length(&self) -> N;length46,1123
        fn nth(&self, index: N) -> &T;nth48,1204
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set50,1293
        fn singleton(item: T) -> Self;singleton52,1416
        fn isEmpty(&self) -> B;isEmpty54,1497
        fn isSingleton(&self) -> B;isSingleton56,1571
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy58,1669
    impl<T: StT> AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS61,1736
        pub fn new_root() -> Self {new_root62,1775
        pub fn new() -> Self { Self::new_root() }new68,1923
        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqStEphS<T> {update69,1973
        pub fn from_vec(values: Vec<T>) -> AVLTreeSeqStEphS<T> {from_vec73,2181
        pub fn to_arrayseq(&self) -> ArraySeqStEphS<T> {to_arrayseq82,2564
        pub fn iter<'a>(&'a self) -> AVLTreeSeqIterStEph<'a, T> { AVLTreeSeqIterStEph::new(&selfiter98,3218
        pub fn push_back(&mut self, value: T) {push_back99,3323
        pub fn contains_value(&self, target: &T) -> B {contains_value104,3537
        pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value112,3760
        pub fn delete_value(&mut self, target: &T) -> bool {delete_value113,3836
    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS138,4654
        fn empty() -> Self { AVLTreeSeqStEphS::new_root() }empty139,4721
        fn new() -> Self { AVLTreeSeqStEphS::new_root() }new141,4782
        fn length(&self) -> N { size_link(&self.root) }length143,4841
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth145,4898
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set147,4969
        fn singleton(item: T) -> Self {singleton152,5135
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty158,5332
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton160,5420
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy162,5512
    pub struct AVLTreeSeqIterStEph<'a, T: StT> {AVLTreeSeqIterStEph177,6033
        stack: Vec<&'a AVLTreeNode<T>>,stack178,6082
        current: Option<&'a AVLTreeNode<T>>,current179,6122
    impl<'a, T: StT> AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph182,6174
        fn new(root: &'a Link<T>) -> Self {new183,6224
        fn push_left(&mut self, link: &'a Link<T>) {push_left191,6453
    impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph200,6705
        type Item = &'a T;Item201,6768
        fn next(&mut self) -> Option<Self::Item> {next202,6795
    fn h<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h210,7019
    fn size_link<T: StT>(n: &Link<T>) -> N {size_link212,7094
    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>) {update_meta220,7259
    fn rotate_right<T: StT>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right228,7504
    fn rotate_left<T: StT>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left239,7867
    fn rebalance<T: StT>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance250,8228
    pub(crate) fn insert_at_link<T: StT>(node: Link<T>, index: N, value: T, next_key: &mut N) ->insert_at_link271,8993
    fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> &'a T {nth_link291,9789
    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str> {set_link303,10169
    macro_rules! AVLTreeSeqStEph {AVLTreeSeqStEph321,10779
    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS335,11340
        fn eq(&self, other: &Self) -> bool {eq336,11393
    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}AVLTreeSeqStEphS349,11720

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBMtEph.rs,4631
pub mod BSTRBMtEph {BSTRBMtEph3,102
    enum Color {Color11,345
        Red,Red12,362
        Black,Black13,375
    type Link<T> = Option<Box<Node<T>>>;Link16,397
    struct Node<T: StTInMtT + Ord> {Node19,460
        key: T,key20,497
        color: Color,color21,513
        size: N,size22,535
        left: Link<T>,left23,552
        right: Link<T>,right24,575
    impl<T: StTInMtT + Ord> Node<T> {Node27,606
        fn new(key: T) -> Self {new28,644
    pub struct BSTRBMtEph<T: StTInMtT + Ord> {BSTRBMtEph40,893
        root: Arc<RwLock<Link<T>>>,root41,940
    pub type BSTreeRB<T> = BSTRBMtEph<T>;BSTreeRB44,983
    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTRBMtEphTrait46,1026
        fn new() -> Self;new47,1084
        fn insert(&self, value: T);insert48,1110
        fn find(&self, target: &T) -> Option<T>;find49,1146
        fn contains(&self, target: &T) -> B;contains50,1195
        fn size(&self) -> N;size51,1240
        fn is_empty(&self) -> B;is_empty52,1269
        fn height(&self) -> N;height53,1302
        fn minimum(&self) -> Option<T>;minimum54,1333
        fn maximum(&self) -> Option<T>;maximum55,1373
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order56,1413
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order57,1462
    impl<T: StTInMtT + Ord> Default for BSTRBMtEph<T> {BSTRBMtEph60,1519
        fn default() -> Self { Self::new() }default61,1575
    impl<T: StTInMtT + Ord> BSTRBMtEph<T> {BSTRBMtEph64,1627
        pub fn new() -> Self {new65,1671
        pub fn size(&self) -> N {size71,1803
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty76,1936
        pub fn height(&self) -> N {height78,2027
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec79,2063
        pub fn insert(&self, value: T) {insert90,2413
        pub fn find(&self, target: &T) -> Option<T> {find98,2678
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains103,2848
        pub fn minimum(&self) -> Option<T> {minimum105,2962
        pub fn maximum(&self) -> Option<T> {maximum110,3114
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order115,3266
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order122,3551
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red129,3838
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link131,3940
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate133,4023
        fn rotate_left(link: &mut Link<T>) {rotate_left135,4142
        fn rotate_right(link: &mut Link<T>) {rotate_right154,4802
        fn flip_colors(link: &mut Link<T>) {flip_colors173,5466
        fn fix_up(link: &mut Link<T>) {fix_up194,6271
        fn insert_link(link: &mut Link<T>, value: T) {insert_link230,7425
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link246,7967
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link261,8484
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link271,8805
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect281,9128
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect289,9415
    impl<T: StTInMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {BSTRBMtEph298,9711
        fn new() -> Self { BSTRBMtEph::new() }new299,9778
        fn insert(&self, value: T) { BSTRBMtEph::insert(self, value) }insert301,9826
        fn find(&self, target: &T) -> Option<T> { BSTRBMtEph::find(self, target) }find303,9898
        fn contains(&self, target: &T) -> B { BSTRBMtEph::contains(self, target) }contains305,9982
        fn size(&self) -> N { BSTRBMtEph::size(self) }size307,10066
        fn is_empty(&self) -> B { BSTRBMtEph::is_empty(self) }is_empty309,10122
        fn height(&self) -> N { BSTRBMtEph::height(self) }height311,10186
        fn minimum(&self) -> Option<T> { BSTRBMtEph::minimum(self) }minimum313,10246
        fn maximum(&self) -> Option<T> { BSTRBMtEph::maximum(self) }maximum315,10316
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTRBMtEph::in_order(self) }in_order317,10386
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTRBMtEph::pre_order(self) }pre_order319,10466
    macro_rules! BSTRBMtEphLit {BSTRBMtEphLit323,10574
    fn _BSTRBMtEphLit_type_checks() {_BSTRBMtEphLit_type_checks335,11064

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayStEph.rs,4383
pub mod BSTSplayStEph {BSTSplayStEph3,84
    type Link<T> = Option<Box<Node<T>>>;Link8,253
    struct Node<T: StT + Ord> {Node11,316
        key: T,key12,348
        size: N,size13,364
        left: Link<T>,left14,381
        right: Link<T>,right15,404
    impl<T: StT + Ord> Node<T> {Node18,435
        fn new(key: T) -> Self {new19,468
    pub struct BSTSplayStEph<T: StT + Ord> {BSTSplayStEph30,682
        root: Link<T>,root31,727
    pub type BSTreeSplay<T> = BSTSplayStEph<T>;BSTreeSplay34,757
    pub trait BSTSplayStEphTrait<T: StT + Ord> {BSTSplayStEphTrait36,806
        fn new() -> Self;new37,855
        fn size(&self) -> N;size38,881
        fn is_empty(&self) -> B;is_empty39,910
        fn height(&self) -> N;height40,943
        fn insert(&mut self, value: T);insert41,974
        fn find(&self, target: &T) -> Option<&T>;find42,1014
        fn contains(&self, target: &T) -> B;contains43,1064
        fn minimum(&self) -> Option<&T>;minimum44,1109
        fn maximum(&self) -> Option<&T>;maximum45,1150
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order46,1191
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order47,1240
    impl<T: StT + Ord> Default for BSTSplayStEph<T> {BSTSplayStEph50,1297
        fn default() -> Self { Self::new() }default51,1351
    impl<T: StT + Ord> BSTSplayStEph<T> {BSTSplayStEph54,1403
        pub fn new() -> Self { BSTSplayStEph { root: None } }new55,1445
        pub fn size(&self) -> N { Self::size_link(&self.root) }size57,1508
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty59,1573
        pub fn height(&self) -> N {height61,1664
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec62,1700
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert71,1996
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find73,2086
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains75,2180
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum77,2294
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum79,2370
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order81,2446
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order87,2670
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link93,2896
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate95,2979
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link97,3098
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link119,3844
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link134,4361
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link144,4682
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect154,5005
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect162,5292
    impl<T: StT + Ord> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {BSTSplayStEph171,5588
        fn new() -> Self { BSTSplayStEph::new() }new172,5656
        fn size(&self) -> N { BSTSplayStEph::size(self) }size174,5707
        fn is_empty(&self) -> B { BSTSplayStEph::is_empty(self) }is_empty176,5766
        fn height(&self) -> N { BSTSplayStEph::height(self) }height178,5833
        fn insert(&mut self, value: T) { BSTSplayStEph::insert(self, value) }insert180,5896
        fn find(&self, target: &T) -> Option<&T> { BSTSplayStEph::find(self, target) }find182,5975
        fn contains(&self, target: &T) -> B { BSTSplayStEph::contains(self, target) }contains184,6063
        fn minimum(&self) -> Option<&T> { BSTSplayStEph::minimum(self) }minimum186,6150
        fn maximum(&self) -> Option<&T> { BSTSplayStEph::maximum(self) }maximum188,6224
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTSplayStEph::in_order(self) }in_order190,6298
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTSplayStEph::pre_order(self) }pre_order192,6381
    macro_rules! BSTSplayStEphLit {BSTSplayStEphLit196,6492
    fn _BSTSplayStEphLit_type_checks() {_BSTSplayStEphLit_type_checks208,7025

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLMtEph.rs,4356
pub mod BSTAVLMtEph {BSTAVLMtEph3,96
    type Link<T> = Option<Box<Node<T>>>;Link10,298
    struct Node<T: StTInMtT + Ord> {Node13,368
        key: T,key14,405
        height: i32,height15,421
        size: N,size16,442
        left: Link<T>,left17,459
        right: Link<T>,right18,482
    impl<T: StTInMtT + Ord> Node<T> {Node21,513
        fn new(key: T) -> Self {new22,551
    pub struct BSTAVLMtEph<T: StTInMtT + Ord> {BSTAVLMtEph34,792
        root: Arc<RwLock<Link<T>>>,root35,840
    pub type BSTreeAVL<T> = BSTAVLMtEph<T>;BSTreeAVL38,883
    pub trait BSTAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTAVLMtEphTrait40,928
        fn new() -> Self;new41,987
        fn insert(&self, value: T);insert42,1013
        fn find(&self, target: &T) -> Option<T>;find43,1049
        fn contains(&self, target: &T) -> B;contains44,1098
        fn size(&self) -> N;size45,1143
        fn is_empty(&self) -> B;is_empty46,1172
        fn height(&self) -> N;height47,1205
        fn minimum(&self) -> Option<T>;minimum48,1236
        fn maximum(&self) -> Option<T>;maximum49,1276
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order50,1316
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order51,1365
    impl<T: StTInMtT + Ord> Default for BSTAVLMtEph<T> {BSTAVLMtEph54,1422
        fn default() -> Self { Self::new() }default55,1479
    impl<T: StTInMtT + Ord> BSTAVLMtEph<T> {BSTAVLMtEph58,1531
        pub fn new() -> Self {new59,1576
        pub fn size(&self) -> N {size65,1709
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty70,1842
        pub fn height(&self) -> N {height72,1933
        pub fn insert(&self, value: T) {insert77,2075
        pub fn find(&self, target: &T) -> Option<T> {find82,2234
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains87,2404
        pub fn minimum(&self) -> Option<T> {minimum89,2518
        pub fn maximum(&self) -> Option<T> {maximum94,2670
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order99,2822
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order106,3107
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link113,3394
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link115,3483
        fn update(node: &mut Node<T>) {update117,3566
        fn rotate_right(link: &mut Link<T>) {rotate_right122,3802
        fn rotate_left(link: &mut Link<T>) {rotate_left136,4258
        fn rebalance(link: &mut Link<T>) {rebalance150,4713
        fn insert_link(link: &mut Link<T>, value: T) {insert_link175,5759
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link194,6394
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link209,6911
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link219,7233
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect229,7557
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect237,7844
    impl<T: StTInMtT + Ord> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {BSTAVLMtEph246,8140
        fn new() -> Self { BSTAVLMtEph::new() }new247,8209
        fn insert(&self, value: T) { BSTAVLMtEph::insert(self, value) }insert249,8258
        fn find(&self, target: &T) -> Option<T> { BSTAVLMtEph::find(self, target) }find251,8331
        fn contains(&self, target: &T) -> B { BSTAVLMtEph::contains(self, target) }contains253,8416
        fn size(&self) -> N { BSTAVLMtEph::size(self) }size255,8501
        fn is_empty(&self) -> B { BSTAVLMtEph::is_empty(self) }is_empty257,8558
        fn height(&self) -> N { BSTAVLMtEph::height(self) }height259,8623
        fn minimum(&self) -> Option<T> { BSTAVLMtEph::minimum(self) }minimum261,8684
        fn maximum(&self) -> Option<T> { BSTAVLMtEph::maximum(self) }maximum263,8755
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTAVLMtEph::in_order(self) }in_order265,8826
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTAVLMtEph::pre_order(self) }pre_order267,8907
    macro_rules! BSTAVLMtEphLit {BSTAVLMtEphLit271,9016
    fn _BSTAVLMtEphLit_type_checks() {_BSTAVLMtEphLit_type_checks283,9519

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainStEph.rs,3570
pub mod BSTPlainStEph {BSTPlainStEph3,64
    pub struct BSTPlainStEph<T: StT + Ord> {BSTPlainStEph9,252
        root: BBTree<T>,root10,297
    pub type BSTree<T> = BSTPlainStEph<T>;BSTree13,329
    pub trait BSTPlainStEphTrait<T: StT + Ord> {BSTPlainStEphTrait15,373
        fn new() -> Self;new16,422
        fn size(&self) -> N;size17,448
        fn is_empty(&self) -> B;is_empty18,477
        fn height(&self) -> N;height19,510
        fn insert(&mut self, value: T);insert20,541
        fn find(&self, target: &T) -> Option<&T>;find21,581
        fn contains(&self, target: &T) -> B;contains22,631
        fn minimum(&self) -> Option<&T>;minimum23,676
        fn maximum(&self) -> Option<&T>;maximum24,717
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order25,758
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order26,807
    impl<T: StT + Ord> BSTPlainStEph<T> {BSTPlainStEph29,864
        pub fn new() -> Self { BSTPlainStEph { root: BBTree::leaf() } }new30,906
        pub fn size(&self) -> N { self.root.size() }size32,979
        pub fn is_empty(&self) -> B { self.root.is_leaf() }is_empty34,1033
        pub fn height(&self) -> N { self.root.height() }height36,1094
        pub fn insert(&mut self, value: T) { insert_node(&mut self.root, value); }insert38,1152
        pub fn find(&self, target: &T) -> Option<&T> { find_node(&self.root, target) }find40,1236
        pub fn contains(&self, target: &T) -> B { contains_node(&self.root, target) }contains42,1324
        pub fn minimum(&self) -> Option<&T> { min_node(&self.root) }minimum44,1411
        pub fn maximum(&self) -> Option<&T> { max_node(&self.root) }maximum46,1481
        pub fn in_order(&self) -> ArraySeqStPerS<T> { self.root.in_order() }in_order48,1551
        pub fn pre_order(&self) -> ArraySeqStPerS<T> { self.root.pre_order() }pre_order50,1629
    impl<T: StT + Ord> BSTPlainStEphTrait<T> for BSTPlainStEph<T> {BSTPlainStEph53,1715
        fn new() -> Self { BSTPlainStEph::new() }new54,1783
        fn size(&self) -> N { BSTPlainStEph::size(self) }size56,1834
        fn is_empty(&self) -> B { BSTPlainStEph::is_empty(self) }is_empty58,1893
        fn height(&self) -> N { BSTPlainStEph::height(self) }height60,1960
        fn insert(&mut self, value: T) { BSTPlainStEph::insert(self, value) }insert62,2023
        fn find(&self, target: &T) -> Option<&T> { BSTPlainStEph::find(self, target) }find64,2102
        fn contains(&self, target: &T) -> B { BSTPlainStEph::contains(self, target) }contains66,2190
        fn minimum(&self) -> Option<&T> { BSTPlainStEph::minimum(self) }minimum68,2277
        fn maximum(&self) -> Option<&T> { BSTPlainStEph::maximum(self) }maximum70,2351
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTPlainStEph::in_order(self) }in_order72,2425
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTPlainStEph::pre_order(self) }pre_order74,2508
    fn insert_node<T: StT + Ord>(node: &mut BBTree<T>, value: T) {insert_node77,2599
    fn contains_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> B {contains_node92,3102
    fn find_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> Option<&'a T> {find_node107,3588
    fn min_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {min_node122,4081
    fn max_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {max_node132,4401
    macro_rules! BSTPlainStEphLit {BSTPlainStEphLit143,4743
    fn _BSTPlainStEphLit_type_checks() {_BSTPlainStEphLit_type_checks158,5315

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetBBAlphaMtEph.rs,5697
pub mod BSTSetBBAlphaMtEph {BSTSetBBAlphaMtEph3,78
    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord> {BSTSetBBAlphaMtEph11,335
        tree: BSTBBAlphaMtEph<T>,tree12,390
    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;BSTSetBBAlphaMt15,431
    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetBBAlphaMtEphTrait17,489
        fn empty() -> Self;empty18,555
        fn singleton(value: T) -> Self;singleton19,583
        fn size(&self) -> N;size20,623
        fn is_empty(&self) -> B;is_empty21,652
        fn find(&self, value: &T) -> Option<T>;find22,685
        fn contains(&self, value: &T) -> B;contains23,733
        fn minimum(&self) -> Option<T>;minimum24,777
        fn maximum(&self) -> Option<T>;maximum25,817
        fn insert(&mut self, value: T);insert26,857
        fn delete(&mut self, target: &T);delete27,897
        fn union(&self, other: &Self) -> Self;union28,939
        fn intersection(&self, other: &Self) -> Self;intersection29,986
        fn difference(&self, other: &Self) -> Self;difference30,1040
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1092
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1147
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1202
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1264
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1334
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1402
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T>;as_tree37,1456
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph40,1513
        pub fn empty() -> Self {empty41,1565
        pub fn singleton(value: T) -> Self {singleton47,1688
        pub fn size(&self) -> N { self.tree.size() }size53,1849
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1903
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1965
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,2043
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2121
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2189
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2257
        pub fn delete(&mut self, target: &T) {delete67,2330
        pub fn union(&self, other: &Self) -> Self {union75,2619
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2918
        pub fn difference(&self, other: &Self) -> Self {difference100,3496
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4073
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4765
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5078
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5434
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5843
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6107
        pub fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree179,6190
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6259
        fn rebuild_from_vec(values: Vec<T>) -> BSTBBAlphaMtEph<T> {rebuild_from_vec183,6350
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6577
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph203,6864
        fn empty() -> Self { Self::empty() }empty204,6947
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6993
        fn size(&self) -> N { self.tree.size() }size208,7060
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7110
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7168
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7242
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7316
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7380
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7444
        fn delete(&mut self, target: &T) { BSTSetBBAlphaMtEph::delete(self, target) }delete222,7513
        fn union(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::union(self, other) }union224,7600
        fn intersection(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::intersection(self, otintersection226,7690
        fn difference(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::difference(self, other)difference228,7794
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetBBAlphaMtEph::split(self, pivot) }split230,7894
        fn join_pair(left: Self, right: Self) -> Self { BSTSetBBAlphaMtEph::join_pair(left, righjoin_pair232,7992
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetBBAlphaMtEph::join_m(left, join_m234,8094
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetBBAlphaMtEph::filtefilter236,8207
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetBBAlphaMtEph::reduce(reduce238,8325
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8440
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree242,8519
    macro_rules! BSTSetBBAlphaMtEphLit {BSTSetBBAlphaMtEphLit246,8610
    fn _BSTSetBBAlphaMtEphLit_type_checks() {_BSTSetBBAlphaMtEphLit_type_checks258,9209

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap03/InsertionSortSt.rs,310
pub mod InsertionSortSt {InsertionSortSt3,51
    pub trait InsertionSortStTrait<T: Ord + Clone> {InsertionSortStTrait5,78
        fn insSort(&self, slice: &mut [T]);insSort8,230
    impl<T: Ord + Clone> InsertionSortStTrait<T> for T {T11,281
        fn insSort(&self, slice: &mut [T]) {insSort12,338

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/MappingStEph.rs,2413
pub mod MappingStEph {MappingStEph3,72
    pub struct Mapping<A, B> {Mapping14,390
        rel: Relation<A, B>,rel15,421
    pub trait MappingStEphTrait<X: StT + Hash, Y: StT + Hash> {MappingStEphTrait18,457
        fn empty() -> Mapping<X, Y>;empty21,613
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y>;FromVec25,747
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation29,901
        fn size(&self) -> N;size33,1056
        fn domain(&self) -> Set<X>;domain37,1182
        fn range(&self) -> Set<Y>;range41,1315
        fn mem(&self, a: &X, b: &Y) -> B;mem45,1443
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>>;iter47,1486
    impl<A: Eq + Hash, B: Eq + Hash> Mapping<A, B> {Mapping50,1569
        fn unique_pairs_from_iter<I: IntoIterator<Item = Pair<A, B>>>(iter: I) -> Set<Pair<A, B>unique_pairs_from_iter51,1622
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Mapping<A, B> {Mapping61,2000
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq62,2069
    impl<A: StT + Hash, B: StT + Hash> Eq for Mapping<A, B> {}Mapping64,2144
    impl<A: StT + Hash, B: StT + Hash> Debug for Mapping<A, B> {Mapping66,2208
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.rel, f) }fmt67,2273
    impl<A: StT + Hash, B: StT + Hash> Display for Mapping<A, B> {Mapping69,2363
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.rel, f) }fmt70,2430
    impl<X: StT + Hash, Y: StT + Hash> MappingStEphTrait<X, Y> for Mapping<X, Y> {Mapping73,2523
        fn empty() -> Mapping<X, Y> {empty74,2606
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y> {FromVec80,2767
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation87,3012
        fn size(&self) -> N { self.rel.size() }size94,3278
        fn domain(&self) -> Set<X> { self.rel.domain() }domain96,3327
        fn range(&self) -> Set<Y> { self.rel.range() }range98,3385
        fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem100,3441
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.rel.iter() }iter102,3506
    macro_rules! MappingLit {MappingLit106,3628
    fn _MappingLit_type_checks() {_MappingLit_type_checks117,4193
    pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise123,4424

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/SetStEph.rs,3621
pub mod SetStEph {SetStEph3,69
    pub struct Set<T> {Set12,250
        data: HashSet<T>,data13,274
    pub trait SetStEphTrait<T: StT + Hash> {SetStEphTrait16,307
        fn empty() -> Set<T>;empty19,444
        fn singleton(x: T) -> Set<T>;singleton22,566
        fn size(&self) -> N;size25,696
        fn mem(&self, x: &T) -> B;mem28,817
        fn union(&self, other: &Set<T>) -> Set<T>;union31,960
        fn intersection(&self, other: &Set<T>) -> Set<T>;intersection34,1119
        fn partition(&self, parts: &Set<Set<T>>) -> B;partition37,1299
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>;CartesianProduct41,1465
        fn insert(&mut self, x: T) -> &mut Self;insert45,1644
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;iter49,1786
        fn FromVec(v: Vec<T>) -> Set<T>;FromVec52,1949
    impl<T: Eq + Hash> PartialEq for Set<T> {Set55,1997
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq56,2043
    impl<T: Eq + Hash> Eq for Set<T> {}Set59,2121
    impl<T: Eq + Hash> std::fmt::Debug for Set<T>Set61,2162
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt65,2256
    impl<T: Eq + Hash> std::fmt::Display for Set<T>Set70,2411
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt74,2509
    impl<T: Eq + Hash> Hash for Set<T> {Set90,3015
        fn hash<H: Hasher>(&self, state: &mut H) {hash91,3056
    impl<T: Eq + Hash> Set<T> {Set107,3620
        pub fn empty() -> Set<T> { Set { data: HashSet::new() } }empty108,3652
        pub fn singleton(x: T) -> Set<T> {singleton110,3719
        pub fn size(&self) -> N { self.data.len() }size116,3885
        pub fn mem(&self, x: &T) -> B { if self.data.contains(x) { B::True } else { B::False } }mem118,3938
        pub fn union(&self, other: &Set<T>) -> Set<T>union120,4036
        pub fn intersection(&self, other: &Set<T>) -> Set<T>intersection131,4310
        pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition142,4667
        pub fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct160,5222
        pub fn insert(&mut self, x: T) -> &mut Self {insert174,5668
        pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter179,5791
        pub fn FromVec(v: Vec<T>) -> Set<T> {FromVec181,5883
    impl<T: StT + Hash> SetStEphTrait<T> for Set<T> {Set190,6107
        fn empty() -> Set<T> { Set { data: HashSet::new() } }empty191,6161
        fn singleton(x: T) -> Set<T> {singleton193,6224
        fn size(&self) -> N { self.data.len() }size199,6386
        fn mem(&self, x: &T) -> B { if self.data.contains(x) { B::True } else { B::False } }mem201,6435
        fn union(&self, other: &Set<T>) -> Set<T>union203,6529
        fn intersection(&self, other: &Set<T>) -> Set<T>intersection214,6799
        fn partition(&self, parts: &Set<Set<T>>) -> B {partition225,7152
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct243,7703
        fn insert(&mut self, x: T) -> &mut Self {insert257,8145
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter262,8264
        fn FromVec(v: Vec<T>) -> Set<T> {FromVec264,8352
    macro_rules! SetLit {SetLit274,8592
    fn _SetLit_type_checks() {_SetLit_type_checks286,8934
    pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise292,9130
        let _s0: Set<&'static str> = SetLit![];str293,9176

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/RelationStEph.rs,2233
pub mod RelationStEph {RelationStEph3,63
    pub struct Relation<A, B> {Relation14,334
        pairs: Set<Pair<A, B>>,pairs15,366
    pub trait RelationStEphTrait<X: StT + Hash, Y: StT + Hash> {RelationStEphTrait18,405
        fn empty() -> Relation<X, Y>;empty21,562
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y>;FromSet25,705
        fn size(&self) -> N;size29,860
        fn domain(&self) -> Set<X>domain33,986
        fn range(&self) -> Set<Y>range39,1154
        fn mem(&self, a: &X, b: &Y) -> Bmem45,1317
        fn iter(&self) -> Iter<'_, Pair<X, Y>>;iter50,1417
    impl<A: StT + Hash, B: StT + Hash> Relation<A, B> {Relation53,1472
        pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B> { Relation { pairs: Set::FromVec(v)FromVec54,1528
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Relation<A, B> {Relation57,1636
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq58,1706
    impl<A: StT + Hash, B: StT + Hash> Eq for Relation<A, B> {}Relation61,1786
    impl<A: StT + Hash, B: StT + Hash> Debug for Relation<A, B> {Relation63,1851
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }fmt64,1917
    impl<A: StT + Hash, B: StT + Hash> Display for Relation<A, B> {Relation67,2020
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) fmt68,2088
    impl<X: StT + Hash, Y: StT + Hash> RelationStEphTrait<X, Y> for Relation<X, Y> {Relation71,2193
        fn empty() -> Relation<X, Y> { Relation { pairs: SetLit![] } }empty72,2278
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y> { Relation { pairs } }FromSet74,2350
        fn size(&self) -> N { self.pairs.size() }size76,2435
        fn domain(&self) -> Set<X>domain78,2486
        fn range(&self) -> Set<Y>range89,2753
        fn mem(&self, a: &X, b: &Y) -> Bmem100,3019
        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }iter112,3295
    macro_rules! RelationLit {RelationLit116,3391
    fn _RelationLit_type_checks() {_RelationLit_type_checks132,4268
    pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise138,4503

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphFloat.rs,1258
pub mod WeightedUnDirGraphMtEphFloat {WeightedUnDirGraphMtEphFloat3,107
    pub type WeightedUnDirGraphMtEphFloat<V> = LabUnDirGraphMtEph<V, OrderedF64>;WeightedUnDirGraphMtEphFloat12,463
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphFloat<V> {WeightedUnDirGraphMtEphFloat15,652
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges17,774
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge32,1337
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight37,1538
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges42,1745
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted51,2106
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight64,2663
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree72,2950
    macro_rules! WeightedUnDirGraphMtEphFloatLit {WeightedUnDirGraphMtEphFloatLit76,3058
    pub fn __weighted_undir_graph_mt_float_macro_typecheck_exercise() {__weighted_undir_graph_mt_float_macro_typecheck_exercise88,3683

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphInt.rs,1316
pub mod WeightedUnDirGraphStEphInt {WeightedUnDirGraphStEphInt3,101
    pub type WeightedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;WeightedUnDirGraphStEphInt12,432
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphInt<V> {WeightedUnDirGraphStEphInt15,588
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,702
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(vadd_weighted_edge32,1251
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, vget_edge_weight35,1418
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges38,1591
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted47,1932
        pub fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum()total_weight60,2469
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree63,2635
        pub fn is_connected(&self) -> bool {is_connected66,2802
    macro_rules! WeightedUnDirGraphStEphIntLit {WeightedUnDirGraphStEphIntLit95,3814
    pub fn __weighted_undir_graph_st_int_macro_typecheck_exercise() {__weighted_undir_graph_st_int_macro_typecheck_exercise107,4410

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphStEph.rs,2676
pub mod UnDirGraphStEph {UnDirGraphStEph3,80
    pub struct UnDirGraphStEph<V: StT + Hash> {UnDirGraphStEph12,310
        V: Set<V>,V13,358
        E: Set<Edge<V>>,E14,377
    pub trait UnDirGraphStEphTrait<V: StT + Hash> {UnDirGraphStEphTrait17,409
        fn empty() -> UnDirGraphStEph<V>;empty20,553
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V>;FromSets23,703
        fn vertices(&self) -> &Set<V>;vertices26,866
        fn edges(&self) -> &Set<Edge<V>>;edges29,997
        fn sizeV(&self) -> N;sizeV32,1131
        fn sizeE(&self) -> N;sizeE35,1253
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1375
        fn NG(&self, v: &V) -> Set<V>;NG41,1518
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1675
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident47,1825
        fn Degree(&self, v: &V) -> N;Degree50,1974
    impl<V: StT + Hash> UnDirGraphStEphTrait<V> for UnDirGraphStEph<V> {UnDirGraphStEph53,2019
        fn empty() -> UnDirGraphStEph<V> {empty54,2092
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E }FromSets60,2249
        fn vertices(&self) -> &Set<V> { &self.V }vertices61,2348
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges62,2398
        fn sizeV(&self) -> N { self.V.size() }sizeV63,2451
        fn sizeE(&self) -> N { self.E.size() }sizeE64,2498
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor66,2546
        fn NG(&self, v: &V) -> Set<V> {NG76,2870
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices88,3232
        fn Incident(&self, e: &Edge<V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } elseIncident97,3504
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree99,3617
    impl<V: StT + Hash> Debug for UnDirGraphStEph<V> {UnDirGraphStEph102,3683
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt103,3738
    impl<V: StT + Hash> Display for UnDirGraphStEph<V> {UnDirGraphStEph111,3958
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} E={:?}", self.V, self.Efmt112,4015
    impl<V: StT + Hash> PartialEq for UnDirGraphStEph<V> {UnDirGraphStEph115,4122
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq116,4181
    impl<V: StT + Hash> Eq for UnDirGraphStEph<V> {}UnDirGraphStEph118,4273
    macro_rules! UnDirGraphStEphLit {UnDirGraphStEphLit121,4347
    fn _UnDirGraphStEphLit_type_checks() {_UnDirGraphStEphLit_type_checks139,5472
    pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise145,5735

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphInt.rs,1540
pub mod WeightedDirGraphStEphInt {WeightedDirGraphStEphInt3,99
    pub type WeightedDirGraphStEphInt<V> = LabDirGraphStEph<V, i32>;WeightedDirGraphStEphInt12,422
    impl<V: StT + Hash> WeightedDirGraphStEphInt<V> {WeightedDirGraphStEphInt15,572
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,678
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(add_weighted_edge32,1217
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(fromget_edge_weight35,1387
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges38,1565
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted47,1914
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted58,2327
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() total_weight69,2737
        pub fn edges_above_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_above_weight72,2893
        pub fn edges_below_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_below_weight83,3338
    macro_rules! WeightedDirGraphStEphIntLit {WeightedDirGraphStEphIntLit95,3755
    fn _WeightedDirGraphStEphIntLit_type_checks() {_WeightedDirGraphStEphIntLit_type_checks107,4341
    pub fn __weighted_dir_graph_st_int_macro_typecheck_exercise() {__weighted_dir_graph_st_int_macro_typecheck_exercise113,4633

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphFloat.rs,1530
pub mod WeightedUnDirGraphStEphFloat {WeightedUnDirGraphStEphFloat29,1110
    pub type WeightedUnDirGraphStEphFloat<V> = LabUnDirGraphStEph<V, OrderedF64>;WeightedUnDirGraphStEphFloat38,1450
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphFloat<V> {WeightedUnDirGraphStEphFloat41,1622
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges43,1738
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge58,2301
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight63,2502
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges68,2709
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted77,3064
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight90,3615
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree98,3902
        pub fn is_connected(&self) -> bool {is_connected101,4069
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge129,5095
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge137,5393
    macro_rules! WeightedUnDirGraphStEphFloatLit {WeightedUnDirGraphStEphFloatLit146,5677
    pub fn __weighted_undir_graph_st_float_macro_typecheck_exercise() {__weighted_undir_graph_st_float_macro_typecheck_exercise158,6302

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphInt.rs,1255
pub mod WeightedUnDirGraphMtEphInt {WeightedUnDirGraphMtEphInt3,100
    pub type WeightedUnDirGraphMtEphInt<V> = LabUnDirGraphMtEph<V, i32>;WeightedUnDirGraphMtEphInt12,447
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphInt<V> {WeightedUnDirGraphMtEphInt15,620
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,740
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(vadd_weighted_edge32,1289
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, vget_edge_weight35,1456
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges38,1629
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted47,1976
        pub fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum()total_weight60,2519
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree63,2685
    macro_rules! WeightedUnDirGraphMtEphIntLit {WeightedUnDirGraphMtEphIntLit67,2793
    pub fn __weighted_undir_graph_mt_int_macro_typecheck_exercise() {__weighted_undir_graph_mt_int_macro_typecheck_exercise79,3389

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphMtEph.rs,2685
pub mod LabUnDirGraphMtEph {LabUnDirGraphMtEph3,119
    pub struct LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph12,352
        vertices: Set<V>,vertices17,471
        labeled_edges: Set<LabEdge<V, L>>,labeled_edges18,497
    pub trait LabUnDirGraphMtEphTrait<V, L>LabUnDirGraphMtEphTrait21,547
        fn empty() -> Self;empty26,670
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges27,698
        fn vertices(&self) -> &Set<V>;vertices28,803
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;labeled_edges29,842
        fn edges(&self) -> Set<Edge<V>>;edges30,898
        fn add_vertex(&mut self, v: V);add_vertex31,939
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);add_labeled_edge32,979
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;get_edge_label33,1043
        fn has_edge(&self, v1: &V, v2: &V) -> bool;has_edge34,1107
        fn neighbors(&self, v: &V) -> Set<V>;neighbors35,1159
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;normalize_edge36,1205
    impl<V, L> LabUnDirGraphMtEphTrait<V, L> for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph39,1270
        fn empty() -> Self {empty44,1423
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges51,1595
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices58,1816
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> { &self.labeled_edges }labeled_edges60,1874
        fn edges(&self) -> Set<Edge<V>> {edges62,1954
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex70,2231
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge72,2300
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label83,2692
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge94,3129
        fn neighbors(&self, v: &V) -> Set<V> {neighbors104,3474
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge116,3913
    impl<V, L> Display for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph124,4333
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt129,4464
    impl<V, L> Debug for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph134,4626
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt139,4755
    macro_rules! LabUnDirGraphMtEphLit {LabUnDirGraphMtEphLit149,5027
    fn _LabUnDirGraphMtEphLit_type_checks() {_LabUnDirGraphMtEphLit_type_checks172,6233
    pub fn __lab_undir_graph_mt_macro_typecheck_exercise() {__lab_undir_graph_mt_macro_typecheck_exercise178,6512

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphFloat.rs,1253
pub mod WeightedDirGraphMtEphFloat {WeightedDirGraphMtEphFloat3,105
    pub type WeightedDirGraphMtEphFloat<V> = LabDirGraphMtEph<V, OrderedF64>;WeightedDirGraphMtEphFloat12,453
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphFloat<V> {WeightedDirGraphMtEphFloat15,636
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges17,750
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge32,1303
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight37,1507
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges42,1719
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted51,2088
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted62,2518
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight73,2945
    macro_rules! WeightedDirGraphMtEphFloatLit {WeightedDirGraphMtEphFloatLit82,3191
    pub fn __weighted_dir_graph_mt_float_macro_typecheck_exercise() {__weighted_dir_graph_mt_float_macro_typecheck_exercise94,3804

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphMtEph.rs,2765
pub mod UnDirGraphMtEph {UnDirGraphMtEph3,105
    pub struct UnDirGraphMtEph<V: StT + MtT + Hash> {UnDirGraphMtEph11,280
        V: Set<V>,V12,334
        E: Set<Edge<V>>,E13,353
    pub trait UnDirGraphMtEphTrait<V: StT + MtT + Hash> {UnDirGraphMtEphTrait16,385
        fn empty() -> UnDirGraphMtEph<V>;empty19,535
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V>;FromSets22,685
        fn vertices(&self) -> &Set<V>;vertices25,848
        fn edges(&self) -> &Set<Edge<V>>;edges28,979
        fn sizeV(&self) -> N;sizeV31,1113
        fn sizeE(&self) -> N;sizeE34,1235
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor37,1357
        fn NG(&self, v: &V) -> Set<V>;NG40,1500
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices43,1657
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident46,1807
        fn Degree(&self, v: &V) -> N;Degree49,1956
    impl<V: StT + MtT + Hash> UnDirGraphMtEphTrait<V> for UnDirGraphMtEph<V> {UnDirGraphMtEph52,2001
        fn empty() -> UnDirGraphMtEph<V> {empty53,2080
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V> { UnDirGraphMtEph { V, E }FromSets59,2237
        fn vertices(&self) -> &Set<V> { &self.V }vertices60,2336
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges61,2386
        fn sizeV(&self) -> N { self.V.size() }sizeV62,2439
        fn sizeE(&self) -> N { self.E.size() }sizeE63,2486
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor65,2534
        fn NG(&self, v: &V) -> Set<V> {NG76,2886
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices88,3254
        fn Incident(&self, e: &Edge<V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } elseIncident97,3526
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree99,3639
    impl<V: StT + MtT + Hash> std::fmt::Debug for UnDirGraphMtEph<V> {UnDirGraphMtEph102,3705
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt103,3776
    impl<V: StT + MtT + Hash> std::fmt::Display for UnDirGraphMtEph<V> {UnDirGraphMtEph111,4016
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} E={fmt112,4089
    impl<V: StT + MtT + Hash> PartialEq for UnDirGraphMtEph<V> {UnDirGraphMtEph115,4216
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq116,4281
    impl<V: StT + MtT + Hash> Eq for UnDirGraphMtEph<V> {}UnDirGraphMtEph118,4373
    macro_rules! UnDirGraphMtEphLit {UnDirGraphMtEphLit121,4453
    fn _UnDirGraphMtEphLit_type_checks() {_UnDirGraphMtEphLit_type_checks139,5578
    pub fn __undirgraph_mt_macro_typecheck_exercise() {__undirgraph_mt_macro_typecheck_exercise145,5830

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphStEph.rs,2617
pub mod LabDirGraphStEph {LabDirGraphStEph3,91
    pub struct LabDirGraphStEph<V, L>LabDirGraphStEph12,322
        vertices: Set<V>,vertices17,422
        labeled_arcs: Set<LabEdge<V, L>>,labeled_arcs18,448
    pub trait LabDirGraphStEphTrait<V, L>LabDirGraphStEphTrait21,497
        fn empty() -> Self;empty26,601
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs27,629
        fn vertices(&self) -> &Set<V>;vertices28,732
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;labeled_arcs29,771
        fn arcs(&self) -> Set<Edge<V>>;arcs30,826
        fn add_vertex(&mut self, v: V);add_vertex31,866
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);add_labeled_arc32,906
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;get_arc_label33,971
        fn has_arc(&self, from: &V, to: &V) -> bool;has_arc34,1036
        fn out_neighbors(&self, v: &V) -> Set<V>;out_neighbors35,1089
        fn in_neighbors(&self, v: &V) -> Set<V>;in_neighbors36,1139
    impl<V, L> LabDirGraphStEphTrait<V, L> for LabDirGraphStEph<V, L>LabDirGraphStEph39,1195
        fn empty() -> Self {empty44,1327
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs51,1496
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices55,1667
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }labeled_arcs57,1725
        fn arcs(&self) -> Set<Edge<V>> {arcs59,1803
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex67,2066
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc69,2135
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label75,2370
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc84,2671
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors93,2945
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors103,3267
    impl<V, L> Display for LabDirGraphStEph<V, L>LabDirGraphStEph114,3594
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt119,3731
    impl<V, L> Debug for LabDirGraphStEph<V, L>LabDirGraphStEph124,3890
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt129,4025
    macro_rules! LabDirGraphStEphLit {LabDirGraphStEphLit139,4293
    fn _LabDirGraphStEphLit_type_checks() {_LabDirGraphStEphLit_type_checks151,5085
    pub fn __lab_dir_graph_macro_typecheck_exercise() {__lab_dir_graph_macro_typecheck_exercise157,5356

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphStEph.rs,3505
pub mod DirGraphStEph {DirGraphStEph3,77
    pub struct DirGraphStEph<V: StT + Hash> {DirGraphStEph12,305
        V: Set<V>,V13,351
        A: Set<Edge<V>>,A14,370
    pub trait DirGraphStEphTrait<V: StT + Hash> {DirGraphStEphTrait17,402
        fn empty() -> DirGraphStEph<V>;empty20,544
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V>;FromSets23,692
        fn vertices(&self) -> &Set<V>;vertices26,853
        fn arcs(&self) -> &Set<Edge<V>>;arcs29,984
        fn sizeV(&self) -> N;sizeV32,1117
        fn sizeA(&self) -> N;sizeA35,1239
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1361
        fn NG(&self, v: &V) -> Set<V>;NG41,1504
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1661
        fn NPlus(&self, v: &V) -> Set<V>;NPlus47,1815
        fn NMinus(&self, v: &V) -> Set<V>;NMinus50,1953
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices53,2114
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices56,2293
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident59,2447
        fn Degree(&self, v: &V) -> N;Degree62,2599
        fn InDegree(&self, v: &V) -> N;InDegree65,2733
        fn OutDegree(&self, v: &V) -> N;OutDegree68,2869
    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {DirGraphStEph71,2917
        fn empty() -> DirGraphStEph<V> {empty72,2986
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }FromSets78,3139
        fn vertices(&self) -> &Set<V> { &self.V }vertices79,3234
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs80,3284
        fn sizeV(&self) -> N { self.V.size() }sizeV81,3336
        fn sizeA(&self) -> N { self.A.size() }sizeA82,3383
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor84,3431
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG93,3696
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices95,3753
        fn NPlus(&self, v: &V) -> Set<V> {NPlus104,4025
        fn NMinus(&self, v: &V) -> Set<V> {NMinus114,4307
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices124,4590
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices133,4872
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } eIncident142,5158
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree144,5274
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree145,5336
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree146,5401
    impl<V: StT + Hash> Debug for DirGraphStEph<V> {DirGraphStEph149,5473
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt150,5526
    impl<V: StT + Hash> Display for DirGraphStEph<V> {DirGraphStEph158,5744
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.Afmt159,5799
    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {DirGraphStEph162,5906
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq163,5963
    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}DirGraphStEph165,6055
    macro_rules! DirGraphStEphLit {DirGraphStEphLit168,6127
    fn _DirGraphStEphLit_type_checks() {_DirGraphStEphLit_type_checks185,7220
    pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise191,7475

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphMtEph.rs,3594
pub mod DirGraphMtEph {DirGraphMtEph3,102
    pub struct DirGraphMtEph<V: StT + MtT + Hash> {DirGraphMtEph11,275
        V: Set<V>,V12,327
        A: Set<Edge<V>>,A13,346
    pub trait DirGraphMtEphTrait<V: StT + MtT + Hash> {DirGraphMtEphTrait16,378
        fn empty() -> DirGraphMtEph<V>;empty19,526
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V>;FromSets22,674
        fn vertices(&self) -> &Set<V>;vertices25,835
        fn arcs(&self) -> &Set<Edge<V>>;arcs28,966
        fn sizeV(&self) -> N;sizeV31,1099
        fn sizeA(&self) -> N;sizeA34,1221
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor37,1343
        fn NG(&self, v: &V) -> Set<V>;NG40,1486
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices43,1643
        fn NPlus(&self, v: &V) -> Set<V>;NPlus46,1797
        fn NMinus(&self, v: &V) -> Set<V>;NMinus49,1935
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices52,2096
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices55,2275
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident58,2429
        fn Degree(&self, v: &V) -> N;Degree61,2581
        fn InDegree(&self, v: &V) -> N;InDegree64,2715
        fn OutDegree(&self, v: &V) -> N;OutDegree67,2851
    impl<V: StT + MtT + Hash> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {DirGraphMtEph70,2899
        fn empty() -> DirGraphMtEph<V> {empty71,2974
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V> { DirGraphMtEph { V, A } }FromSets77,3127
        fn vertices(&self) -> &Set<V> { &self.V }vertices78,3222
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs79,3272
        fn sizeV(&self) -> N { self.V.size() }sizeV80,3324
        fn sizeA(&self) -> N { self.A.size() }sizeA81,3371
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor83,3419
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG92,3690
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices94,3747
        fn NPlus(&self, v: &V) -> Set<V> {NPlus103,4019
        fn NMinus(&self, v: &V) -> Set<V> {NMinus113,4304
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices123,4590
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices132,4872
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } eIncident141,5158
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree143,5274
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree144,5336
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree145,5401
    impl<V: StT + MtT + Hash> std::fmt::Debug for DirGraphMtEph<V> {DirGraphMtEph148,5473
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt149,5542
    impl<V: StT + MtT + Hash> std::fmt::Display for DirGraphMtEph<V> {DirGraphMtEph157,5780
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} A={fmt158,5851
    impl<V: StT + MtT + Hash> PartialEq for DirGraphMtEph<V> {DirGraphMtEph161,5978
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq162,6041
    impl<V: StT + MtT + Hash> Eq for DirGraphMtEph<V> {}DirGraphMtEph164,6133
    macro_rules! DirGraphMtEphLit {DirGraphMtEphLit167,6211
    fn _DirGraphMtEphLit_type_checks() {_DirGraphMtEphLit_type_checks184,7304
    pub fn __dirgraph_mt_macro_typecheck_exercise() {__dirgraph_mt_macro_typecheck_exercise190,7548

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphFloat.rs,1906
pub mod WeightedDirGraphStEphFloat {WeightedDirGraphStEphFloat29,1075
    pub type WeightedDirGraphStEphFloat<V> = LabDirGraphStEph<V, OrderedF64>;WeightedDirGraphStEphFloat38,1407
    impl<V: StT + Hash> WeightedDirGraphStEphFloat<V> {WeightedDirGraphStEphFloat41,1573
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges43,1681
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge58,2234
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight63,2438
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges68,2650
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted77,3013
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted88,3440
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight99,3864
        pub fn edges_above_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_above_weight107,4141
        pub fn edges_below_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_below_weight118,4614
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge129,5073
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge137,5370
        pub fn scale_weights(&mut self, factor: OrderedFloat<f64>) {scale_weights145,5669
    macro_rules! WeightedDirGraphStEphFloatLit {WeightedDirGraphStEphFloatLit163,6292
    fn _WeightedDirGraphStEphFloatLit_type_checks() {_WeightedDirGraphStEphFloatLit_type_checks175,6905
    pub fn __weighted_dir_graph_st_float_macro_typecheck_exercise() {__weighted_dir_graph_st_float_macro_typecheck_exercise181,7207

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphInt.rs,1231
pub mod WeightedDirGraphMtEphInt {WeightedDirGraphMtEphInt3,98
    pub type WeightedDirGraphMtEphInt<V> = LabDirGraphMtEph<V, i32>;WeightedDirGraphMtEphInt12,437
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphInt<V> {WeightedDirGraphMtEphInt15,604
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,716
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(add_weighted_edge32,1255
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(fromget_edge_weight35,1425
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges38,1603
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted47,1958
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted58,2374
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() total_weight69,2787
    macro_rules! WeightedDirGraphMtEphIntLit {WeightedDirGraphMtEphIntLit73,2912
    pub fn __weighted_dir_graph_mt_int_macro_typecheck_exercise() {__weighted_dir_graph_mt_int_macro_typecheck_exercise85,3498

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphStEph.rs,2677
pub mod LabUnDirGraphStEph {LabUnDirGraphStEph3,94
    pub struct LabUnDirGraphStEph<V, L>LabUnDirGraphStEph12,327
        vertices: Set<V>,vertices17,435
        labeled_edges: Set<LabEdge<V, L>>,labeled_edges18,461
    pub trait LabUnDirGraphStEphTrait<V, L>LabUnDirGraphStEphTrait21,511
        fn empty() -> Self;empty26,623
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges27,651
        fn vertices(&self) -> &Set<V>;vertices28,756
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;labeled_edges29,795
        fn edges(&self) -> Set<Edge<V>>;edges30,851
        fn add_vertex(&mut self, v: V);add_vertex31,892
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);add_labeled_edge32,932
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;get_edge_label33,996
        fn has_edge(&self, v1: &V, v2: &V) -> bool;has_edge34,1060
        fn neighbors(&self, v: &V) -> Set<V>;neighbors35,1112
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;normalize_edge36,1158
    impl<V, L> LabUnDirGraphStEphTrait<V, L> for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph39,1223
        fn empty() -> Self {empty44,1365
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges51,1537
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices58,1758
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> { &self.labeled_edges }labeled_edges60,1816
        fn edges(&self) -> Set<Edge<V>> {edges62,1896
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex70,2167
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge72,2236
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label83,2622
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge94,3059
        fn neighbors(&self, v: &V) -> Set<V> {neighbors105,3468
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge117,3901
    impl<V, L> Display for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph125,4321
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt130,4441
    impl<V, L> Debug for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph135,4603
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt140,4721
    macro_rules! LabUnDirGraphStEphLit {LabUnDirGraphStEphLit150,4993
    fn _LabUnDirGraphStEphLit_type_checks() {_LabUnDirGraphStEphLit_type_checks173,6199
    pub fn __lab_undir_graph_macro_typecheck_exercise() {__lab_undir_graph_macro_typecheck_exercise179,6478

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphMtEph.rs,2625
pub mod LabDirGraphMtEph {LabDirGraphMtEph3,116
    pub struct LabDirGraphMtEph<V, L>LabDirGraphMtEph12,347
        vertices: Set<V>,vertices17,458
        labeled_arcs: Set<LabEdge<V, L>>,labeled_arcs18,484
    pub trait LabDirGraphMtEphTrait<V, L>LabDirGraphMtEphTrait21,533
        fn empty() -> Self;empty26,648
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs27,676
        fn vertices(&self) -> &Set<V>;vertices28,779
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;labeled_arcs29,818
        fn arcs(&self) -> Set<Edge<V>>;arcs30,873
        fn add_vertex(&mut self, v: V);add_vertex31,913
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);add_labeled_arc32,953
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;get_arc_label33,1018
        fn has_arc(&self, from: &V, to: &V) -> bool;has_arc34,1083
        fn out_neighbors(&self, v: &V) -> Set<V>;out_neighbors35,1136
        fn in_neighbors(&self, v: &V) -> Set<V>;in_neighbors36,1186
    impl<V, L> LabDirGraphMtEphTrait<V, L> for LabDirGraphMtEph<V, L>LabDirGraphMtEph39,1242
        fn empty() -> Self {empty44,1385
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs51,1554
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices55,1725
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }labeled_arcs57,1783
        fn arcs(&self) -> Set<Edge<V>> {arcs59,1861
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex67,2130
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc69,2199
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label75,2440
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc84,2741
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors93,3015
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors103,3340
    impl<V, L> Display for LabDirGraphMtEph<V, L>LabDirGraphMtEph114,3670
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt119,3793
    impl<V, L> Debug for LabDirGraphMtEph<V, L>LabDirGraphMtEph124,3952
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt129,4073
    macro_rules! LabDirGraphMtEphLit {LabDirGraphMtEphLit139,4341
    fn _LabDirGraphMtEphLit_type_checks() {_LabDirGraphMtEphLit_type_checks151,5133
    pub fn __lab_dir_graph_mt_macro_typecheck_exercise() {__lab_dir_graph_mt_macro_typecheck_exercise157,5404

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap11/FibonacciMt.rs,579
pub mod FibonacciMt {FibonacciMt4,131
    pub struct FibonacciMt;FibonacciMt7,198
    pub trait FibonacciMtTrait {FibonacciMtTrait10,261
        fn fib(n: N) -> N;fib11,294
    impl FibonacciMt {FibonacciMt14,328
        pub fn fib(n: N) -> N {fib15,351
    impl FibonacciMtTrait for FibonacciMt {FibonacciMt25,623
        fn fib(n: N) -> N {fib26,667
    mod tests {tests32,761
        fn fib_base_cases() {fib_base_cases37,882
        fn fib_small_values() {fib_small_values43,1035
        fn trait_and_inherent_agree() {trait_and_inherent_agree50,1239

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_5.rs,1372
pub mod Exercise12_5 {Exercise12_53,86
    struct Node<T: StTInMtT> {Node10,260
        value: ManuallyDrop<T>,value11,291
        next: *mut Node<T>,next12,323
    pub struct ConcurrentStackMt<T: StTInMtT> {ConcurrentStackMt17,446
        head: AtomicPtr<Node<T>>,head18,494
    pub trait ConcurrentStackMtTrait<T: StTInMtT> {ConcurrentStackMtTrait21,535
        fn new() -> Self;new22,587
        fn push(&self, value: T);push23,613
        fn pop(&self) -> Option<T>;pop24,647
        fn is_empty(&self) -> bool;is_empty25,683
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt28,726
        fn raw_pop(&self) -> Option<*mut Node<T>> {raw_pop29,771
    impl<T: StTInMtT> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {ConcurrentStackMt47,1320
        fn new() -> Self {new48,1395
        fn push(&self, value: T) {push52,1500
        fn pop(&self) -> Option<T> {pop69,2137
        fn is_empty(&self) -> bool {is_empty76,2376
    impl<T: StTInMtT> Default for ConcurrentStackMt<T> {ConcurrentStackMt81,2486
        fn default() -> Self { Self::new() }default82,2543
    impl<T: StTInMtT> Drop for ConcurrentStackMt<T> {ConcurrentStackMt85,2595
        fn drop(&mut self) {drop86,2649
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt99,3053
        pub fn drain(&self) -> Vec<T> {drain101,3173

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_2.rs,477
pub mod Exercise12_2 {Exercise12_23,83
    pub trait FetchAddCasTrait {FetchAddCasTrait6,159
        fn fetch_add_cas(&self, delta: usize) -> usize;fetch_add_cas7,192
    impl FetchAddCasTrait for AtomicUsize {AtomicUsize10,255
        fn fetch_add_cas(&self, delta: usize) -> usize {fetch_add_cas11,299
    pub fn fetch_add_cas(target: &AtomicUsize, delta: usize) -> usize {fetch_add_cas23,753
    pub fn efficiency_note() -> &'static str {efficiency_note27,868

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_1.rs,1017
pub mod Exercise12_1 {Exercise12_13,72
    pub struct SpinLock {SpinLock12,283
        ticket: AtomicUsize,ticket13,309
        turn: AtomicUsize,turn14,338
    pub trait SpinLockTrait {SpinLockTrait17,372
        fn new() -> Self;new18,402
        fn lock(&self);lock19,428
        fn unlock(&self);unlock20,452
    impl SpinLock {SpinLock23,485
        pub fn new() -> Self {new24,505
        pub fn lock(&self) {lock28,627
        pub fn unlock(&self) {unlock35,850
        pub fn with_lock<T>(&self, action: impl FnOnce() -> T) -> T {with_lock39,947
    impl SpinLockTrait for SpinLock {SpinLock47,1140
        fn new() -> Self { SpinLock::new() }new48,1178
        fn lock(&self) { SpinLock::lock(self) }lock50,1224
        fn unlock(&self) { SpinLock::unlock(self) }unlock52,1273
    impl Default for SpinLock {SpinLock55,1332
        fn default() -> Self { SpinLock::new() }default56,1364
    pub fn parallel_increment(iterations: N) -> usize {parallel_increment59,1420

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap17/MathSeq.rs,4403
pub mod MathSeq {MathSeq8,306
    pub struct MathSeqS<T: StT> {MathSeqS18,609
        data: Vec<T>,data19,643
    pub trait MathSeqTrait<T: StT + Hash> {MathSeqTrait23,708
        fn new(length: N, init_value: T) -> Self;new26,854
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str>;set30,997
        fn length(&self) -> N;length34,1172
        fn nth(&self, index: N) -> &T;nth38,1296
        fn empty() -> Self;empty42,1428
        fn singleton(item: T) -> Self;singleton46,1549
        fn subseq(&self, start: N, length: N) -> &[T];subseq50,1681
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy54,1839
        fn add_last(&mut self, value: T) -> &mut Self;add_last58,2104
        fn delete_last(&mut self) -> Option<T>;delete_last62,2252
        fn isEmpty(&self) -> B;isEmpty66,2393
        fn isSingleton(&self) -> B;isSingleton70,2518
        fn domain(&self) -> Vec<N>;domain74,2651
        fn range(&self) -> Vec<T>;range78,2784
        fn multiset_range(&self) -> Vec<(N, T)>;multiset_range82,2916
    impl<T: StT> PartialEq for MathSeqS<T> {MathSeqS85,2972
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq86,3017
    impl<T: StT> Eq for MathSeqS<T> {}MathSeqS89,3095
    impl<T: StT> std::fmt::Debug for MathSeqS<T> {MathSeqS91,3135
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt92,3186
    impl<T: StT> std::fmt::Display for MathSeqS<T> {MathSeqS97,3342
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt98,3395
    impl<T: StT> MathSeqS<T> {MathSeqS113,3808
        pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter116,3931
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }iter_mut119,4098
        pub fn empty() -> Self { Self { data: Vec::new() } }empty123,4281
        pub fn singleton(item: T) -> Self { Self { data: vec![item] } }singleton126,4434
        pub fn from_vec(data: Vec<T>) -> Self { Self { data } }from_vec129,4608
        pub fn with_len(length: N, init_value: T) -> Self {with_len132,4774
    impl<'a, T: StT> IntoIterator for &'a MathSeqS<T> {MathSeqS139,4932
        type Item = &'a T;Item140,4988
        type IntoIter = std::slice::Iter<'a, T>;IntoIter141,5015
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }into_iter142,5064
    impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {MathSeqS145,5137
        type Item = &'a mut T;Item146,5197
        type IntoIter = std::slice::IterMut<'a, T>;IntoIter147,5228
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }into_iter148,5280
    impl<T: StT> IntoIterator for MathSeqS<T> {MathSeqS151,5357
        type Item = T;Item152,5405
        type IntoIter = std::vec::IntoIter<T>;IntoIter153,5428
        fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }into_iter154,5475
    impl<T: StT + Hash> MathSeqTrait<T> for MathSeqS<T> {MathSeqS157,5553
        fn new(length: N, init_value: T) -> Self {new158,5611
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str> {set164,5758
        fn length(&self) -> N { self.data.len() }length173,6038
        fn nth(&self, index: N) -> &T { &self.data[index] }nth175,6089
        fn empty() -> Self { MathSeqS { data: Vec::new() } }empty177,6150
        fn singleton(item: T) -> Self { MathSeqS { data: vec![item] } }singleton179,6212
        fn subseq(&self, start: N, length: N) -> &[T] {subseq181,6285
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy188,6509
        fn add_last(&mut self, value: T) -> &mut Self {add_last200,6886
        fn delete_last(&mut self) -> Option<T> { self.data.pop() }delete_last205,7005
        fn isEmpty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }isEmpty207,7073
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton209,7163
        fn domain(&self) -> Vec<N> { (0..self.data.len()).collect() }domain211,7257
        fn range(&self) -> Vec<T> {range213,7328
        fn multiset_range(&self) -> Vec<(N, T)> {multiset_range224,7699
    macro_rules! MathSeqSLit {MathSeqSLit243,8383
    fn _MathSeqSLit_type_checks() {_MathSeqSLit_type_checks256,8777

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeq.rs,6357
pub mod ArraySeq {ArraySeq4,150
    pub struct ArraySeqS<T> {ArraySeqS12,332
        data: Box<[T]>,data13,362
    pub type ArrayS<T> = ArraySeqS<T>;ArrayS16,393
    pub trait ArraySeq<T> {ArraySeq19,508
        fn new(length: N, init_value: T) -> ArraySeqS<T>new22,685
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str>;set28,882
        fn length(&self) -> N;length32,1085
        fn nth(&self, index: N) -> &T;nth36,1251
        fn empty() -> ArraySeqS<T>;empty40,1402
        fn singleton(item: T) -> ArraySeqS<T>;singleton44,1574
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqS<T>;tabulate48,1758
        fn map<U: Clone>(a: &ArraySeqS<T>, f: impl Fn(&T) -> U) -> ArraySeqS<U>;map52,1939
        fn subseq(a: &ArraySeqS<T>, start: N, length: N) -> ArraySeqS<T>subseq56,2176
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> ArraySeqS<T>append62,2404
        fn filter(a: &ArraySeqS<T>, pred: impl Fn(&T) -> B) -> ArraySeqS<T>filter68,2629
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> ArraySeqS<T>flatten74,2874
        fn update(a: &ArraySeqS<T>, update: Pair<N, T>) -> ArraySeqS<T>update80,3116
        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T>inject86,3373
        fn isEmpty(&self) -> B;isEmpty92,3620
        fn isSingleton(&self) -> B;isSingleton96,3781
        fn collect<K: Clone + Eq, V: Clone>(collect100,3983
        fn iterate<A>(a: &ArraySeqS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A;iterate106,4227
        fn reduce(a: &ArraySeqS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> Treduce110,4451
        fn scan(a: &ArraySeqS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqS<T>, T)scan116,4693
    impl<T> ArraySeqS<T> {ArraySeqS121,4824
        fn new(length: N, init_value: T) -> ArraySeqS<T>new122,4851
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {set131,5103
        fn length(&self) -> N { self.data.len() }length140,5389
        fn nth(&self, index: N) -> &T { &self.data[index] }nth142,5440
        fn empty() -> ArraySeqS<T> { ArraySeqS::from_vec(Vec::new()) }empty144,5501
        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::from_vec(vec![item]) }singleton146,5573
        fn isEmpty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }isEmpty148,5656
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton150,5746
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq155,6067
        pub fn subseq_copy(&self, start: N, length: N) -> ArraySeqS<T>subseq_copy164,6677
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqS<T> {update180,7395
        pub fn from_vec(elts: Vec<T>) -> ArraySeqS<T> {from_vec191,7925
        pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter197,8077
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }iter_mut199,8153
    impl<T> ArraySeq<T> for ArraySeqS<T> {ArraySeqS202,8250
        fn new(length: N, init_value: T) -> ArraySeqS<T>new203,8293
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {set210,8454
        fn length(&self) -> N { ArraySeqS::length(self) }length214,8601
        fn nth(&self, index: N) -> &T { ArraySeqS::nth(self, index) }nth216,8660
        fn empty() -> ArraySeqS<T> { ArraySeqS::empty() }empty218,8731
        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::singleton(item) }singleton220,8790
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqS<T> {tabulate222,8868
        fn map<U: Clone>(a: &ArraySeqS<T>, f: impl Fn(&T) -> U) -> ArraySeqS<U> {map230,9135
        fn subseq(a: &ArraySeqS<T>, start: N, length: N) -> ArraySeqS<T>subseq239,9450
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> ArraySeqS<T>append246,9621
        fn filter(a: &ArraySeqS<T>, pred: impl Fn(&T) -> B) -> ArraySeqS<T>filter264,10195
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> ArraySeqS<T>flatten278,10610
        fn update(a: &ArraySeqS<T>, Pair(index, item): Pair<N, T>) -> ArraySeqS<T>update292,11027
        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T>inject303,11387
        fn isEmpty(&self) -> B { ArraySeqS::isEmpty(self) }isEmpty318,11949
        fn isSingleton(&self) -> B { ArraySeqS::isSingleton(self) }isSingleton320,12010
        fn collect<K: Clone + Eq, V: Clone>(collect322,12079
        fn iterate<A>(a: &ArraySeqS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A {iterate345,12992
        fn reduce(a: &ArraySeqS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> Treduce353,13224
        fn scan(a: &ArraySeqS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqS<T>, T)scan364,13493
    impl<T: PartialEq> PartialEq for ArraySeqS<T> {ArraySeqS379,13950
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq380,14002
    impl<T: Eq> Eq for ArraySeqS<T> {}ArraySeqS383,14080
    impl<T: Debug> Debug for ArraySeqS<T> {ArraySeqS385,14120
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt386,14164
    impl<T: Display> Display for ArraySeqS<T> {ArraySeqS389,14283
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt390,14331
    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {ArraySeqS402,14661
        type Item = &'a T;Item403,14713
        type IntoIter = std::slice::Iter<'a, T>;IntoIter404,14740
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }into_iter406,14790
    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {ArraySeqS409,14863
        type Item = &'a mut T;Item410,14919
        type IntoIter = std::slice::IterMut<'a, T>;IntoIter411,14950
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }into_iter413,15003
    impl<T> IntoIterator for ArraySeqS<T> {ArraySeqS416,15080
        type Item = T;Item417,15124
        type IntoIter = std::vec::IntoIter<T>;IntoIter418,15147
        fn into_iter(self) -> Self::IntoIter { Vec::from(self.data).into_iter() }into_iter420,15195
    macro_rules! ArraySeqS {ArraySeqS424,15304
    fn _arrayseqs_macro_type_checks() {_arrayseqs_macro_type_checks437,15740

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStPer.rs,5144
pub mod LinkedListStPer {LinkedListStPer3,48
    pub struct NodeP<T: StT> {NodeP9,171
        pub value: T,value10,202
        pub next: Option<Box<NodeP<T>>>,next11,224
    pub struct LinkedListStPerS<T: StT> {LinkedListStPerS15,300
        head: Option<Box<NodeP<T>>>,head16,342
        len: N,len17,379
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait20,402
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>new21,447
        fn empty() -> LinkedListStPerS<T>;empty24,547
        fn singleton(item: T) -> LinkedListStPerS<T>;singleton25,590
        fn length(&self) -> N;length26,644
        fn nth(&self, index: N) -> &T;nth27,675
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T>;subseq_copy28,714
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate29,789
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map30,859
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append31,952
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter32,1044
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;update33,1135
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject34,1223
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject35,1330
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate36,1438
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes37,1527
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce38,1647
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan40,1730
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten42,1834
        fn collect<A: StT, Bv: StT>(collect44,1922
    impl<T: StT> LinkedListStPerS<T> {LinkedListStPerS50,2114
        pub fn empty() -> Self { LinkedListStPerS { head: None, len: 0 } }empty51,2153
        pub fn new(length: N, init_value: T) -> Selfnew53,2229
        pub fn singleton(item: T) -> Self { LinkedListStPerS::from_vec(vec![item]) }singleton60,2404
        pub fn from_vec(elts: Vec<T>) -> Self {from_vec62,2490
        pub fn length(&self) -> N { self.len }length72,2840
        pub fn nth(&self, index: N) -> &T {nth74,2888
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy80,3063
        fn node_at(&self, index: N) -> Option<&NodeP<T>> {node_at110,4145
    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {LinkedListStPerS127,4620
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt128,4681
    impl<T: StT> PartialEq for LinkedListStPerS<T> {LinkedListStPerS145,5214
        fn eq(&self, other: &Self) -> bool {eq146,5267
    impl<T: StT> Eq for LinkedListStPerS<T> {}LinkedListStPerS163,5779
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS165,5827
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>new166,5894
        fn empty() -> LinkedListStPerS<T> { LinkedListStPerS::empty() }empty173,6069
        fn singleton(item: T) -> LinkedListStPerS<T> { LinkedListStPerS::singleton(item) }singleton174,6141
        fn length(&self) -> N { LinkedListStPerS::length(self) }length175,6232
        fn nth(&self, index: N) -> &T { LinkedListStPerS::nth(self, index) }nth176,6297
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T> {subseq_copy177,6374
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate181,6524
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map189,6790
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append197,7104
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter208,7533
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T>update219,7925
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject232,8414
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject245,9008
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate256,9480
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes264,9718
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce274,10150
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan290,10773
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten305,11470
        fn collect<A: StT, Bv: StT>(collect316,11874

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStEph.rs,5663
pub mod ArraySeqStEph {ArraySeqStEph3,72
    pub struct ArraySeqStEphS<T: StT> {ArraySeqStEphS10,254
        data: Box<[T]>,data11,294
    pub type ArrayStEph<T> = ArraySeqStEphS<T>;ArrayStEph14,325
    impl<T: StT> ArraySeqStEphS<T> {ArraySeqStEphS16,374
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }from_vec17,411
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) new19,501
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }empty21,600
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton23,663
        pub fn length(&self) -> N { self.data.len() }length25,737
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth27,792
        pub fn subseq(&self, start: N, length: N) -> Self {subseq29,857
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set36,1150
        pub fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {update45,1432
        pub fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut Self {inject50,1582
    impl<T: StT> PartialEq for ArraySeqStEphS<T> {ArraySeqStEphS62,2016
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }eq63,2067
    impl<T: StT> Eq for ArraySeqStEphS<T> {}ArraySeqStEphS66,2153
    impl<T: StT> Debug for ArraySeqStEphS<T> {ArraySeqStEphS68,2199
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt69,2246
    impl<T: StT> Display for ArraySeqStEphS<T> {ArraySeqStEphS72,2365
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt73,2414
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait85,2744
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>;new86,2787
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str>;set87,2850
        fn length(&self) -> N;length88,2944
        fn nth(&self, index: N) -> &T;nth89,2975
        fn empty() -> ArraySeqStEphS<T>;empty90,3014
        fn singleton(item: T) -> ArraySeqStEphS<T>;singleton91,3055
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStEphS<T>;tabulate92,3107
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map93,3180
        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T>;subseq94,3269
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append95,3353
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter96,3439
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten97,3526
        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T>;update98,3606
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T>;inject99,3682
        fn isEmpty(&self) -> B;isEmpty100,3776
        fn isSingleton(&self) -> B;isSingleton101,3808
        fn collect<K: StT, V: StT>(collect102,3844
        fn iterate<A>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A;iterate106,4024
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce107,4109
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan108,4189
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS111,4295
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::new(length, initnew112,4358
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str> {set114,4465
        fn length(&self) -> N { ArraySeqStEphS::length(self) }length118,4622
        fn nth(&self, index: N) -> &T { ArraySeqStEphS::nth(self, index) }nth120,4686
        fn empty() -> ArraySeqStEphS<T> { ArraySeqStEphS::empty() }empty122,4762
        fn singleton(item: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::singleton(item) }singleton124,4831
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStEphS<T> {tabulate126,4919
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map134,5196
        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T> { a.subseq(stsubseq142,5504
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append144,5616
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter156,6068
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten167,6454
        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T> { ArraySeqStEphS::updupdate178,6847
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T> {inject180,6964
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty184,7120
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton186,7208
        fn collect<K: StT, V: StT>(collect188,7300
        fn iterate<A>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A {iterate210,8231
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce218,8468
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan226,8698
    macro_rules! ArraySeqStEphS {ArraySeqStEphS238,9126
    fn _arrayseqstephs_macro_type_checks() {_arrayseqstephs_macro_type_checks245,9536

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtPer.rs,5729
pub mod ArraySeqMtPer {ArraySeqMtPer3,82
    pub struct ArraySeqMtPerS<T: StTInMtT> {ArraySeqMtPerS10,272
        data: Box<[T]>,data11,317
    impl<T: StTInMtT> ArraySeqMtPerS<T> {ArraySeqMtPerS14,348
        pub fn empty() -> Self {empty15,390
        pub fn new(length: N, init_value: T) -> Self {new21,530
        pub fn singleton(item: T) -> Self { ArraySeqMtPerS::from_vec(vec![item]) }singleton29,802
        pub fn from_vec(values: Vec<T>) -> Self {from_vec31,886
        pub fn length(&self) -> N { self.data.len() }length37,1039
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth39,1094
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set41,1159
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy50,1524
        pub fn is_empty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }is_empty58,1849
        pub fn is_singleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } is_singleton60,1944
    impl<T: StTInMtT> Clone for ArraySeqMtPerS<T> {ArraySeqMtPerS63,2049
        fn clone(&self) -> Self {clone64,2101
    impl<T: StTInMtT + PartialEq> PartialEq for ArraySeqMtPerS<T> {ArraySeqMtPerS70,2267
        fn eq(&self, other: &Self) -> bool {eq71,2335
    impl<T: StTInMtT + Eq> Eq for ArraySeqMtPerS<T> {}ArraySeqMtPerS84,2662
    pub trait ArraySeqMtPerTrait<T: StTInMtT> {ArraySeqMtPerTrait86,2718
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;new87,2766
        fn length(&self) -> N;length88,2829
        fn nth(&self, index: N) -> &T;nth89,2860
        fn empty() -> ArraySeqMtPerS<T>;empty90,2899
        fn singleton(item: T) -> ArraySeqMtPerS<T>;singleton91,2940
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T>;tabulate94,3160
        fn map<W: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W) -> ArraySeqMtPerS<W>;map97,3400
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;subseq_copy98,3489
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append101,3674
        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtPerS<T>;filter104,3966
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;flatten106,4123
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T>;update109,4304
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerSinject112,4548
        fn isEmpty(&self) -> B;isEmpty113,4649
        fn isSingleton(&self) -> B;isSingleton114,4681
        fn collect(collect116,4796
        fn iterate<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate121,5033
        fn reduce(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce123,5203
        fn scan(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtPerS<T>, Tscan125,5330
        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str>;set126,5429
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject129,5638
        fn iteratePrefixes<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes131,5785
    impl<T: StTInMtT> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {ArraySeqMtPerS134,5908
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::new(length, initnew135,5976
        fn length(&self) -> N { ArraySeqMtPerS::length(self) }length137,6083
        fn nth(&self, index: N) -> &T { ArraySeqMtPerS::nth(self, index) }nth139,6147
        fn empty() -> ArraySeqMtPerS<T> { ArraySeqMtPerS::empty() }empty141,6223
        fn singleton(item: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::singleton(item) }singleton143,6292
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T> {tabulate145,6380
        fn map<W: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W) -> ArraySeqMtPerS<W> {map150,6570
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> {subseq_copy158,6878
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append162,7024
        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtPerS<T> {filter173,7448
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {flatten184,7834
        fn update(a: &ArraySeqMtPerS<T>, Pair(index, item): Pair<N, T>) -> ArraySeqMtPerS<T> {update195,8230
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerSinject199,8397
        fn isEmpty(&self) -> B { ArraySeqMtPerS::is_empty(self) }isEmpty211,8913
        fn isSingleton(&self) -> B { ArraySeqMtPerS::is_singleton(self) }isSingleton213,8980
        fn collect(collect215,9055
        fn iterate<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate242,10207
        fn reduce(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce250,10443
        fn scan(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtPerS<T>, Tscan265,11050
        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str> {set275,11454
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject279,11602
        fn iteratePrefixes<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes290,12035
    macro_rules! ArraySeqMtPerSLit {ArraySeqMtPerSLit303,12520
    fn _ArraySeqMtPerSLit_type_checks() {_ArraySeqMtPerSLit_type_checks310,12909

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtEph.rs,4473
pub mod ArraySeqMtEph {ArraySeqMtEph3,67
    pub struct ArraySeqMtEphS<T: StT> {ArraySeqMtEphS11,297
        data: Mutex<Box<[T]>>,data12,337
    impl<T: StT> ArraySeqMtEphS<T> {ArraySeqMtEphS15,375
        pub fn empty() -> Self {empty16,412
        pub fn new(length: N, init_value: T) -> Selfnew22,564
        pub fn singleton(item: T) -> Self { ArraySeqMtEphS::from_vec(vec![item]) }singleton29,737
        pub fn from_vec(values: Vec<T>) -> Self {from_vec31,821
        pub fn length(&self) -> N {length37,986
        pub fn nth_cloned(&self, index: N) -> T {nth_cloned42,1108
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set47,1253
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy57,1582
        pub fn to_vec(&self) -> Vec<T> {to_vec66,1950
    impl<T: StT> Clone for ArraySeqMtEphS<T> {ArraySeqMtEphS72,2103
        fn clone(&self) -> Self { ArraySeqMtEphS::from_vec(self.to_vec()) }clone73,2150
    impl<T: StT> PartialEq for ArraySeqMtEphS<T> {ArraySeqMtEphS76,2233
        fn eq(&self, other: &Self) -> bool { self.to_vec() == other.to_vec() }eq77,2284
    impl<T: StT> Eq for ArraySeqMtEphS<T> {}ArraySeqMtEphS80,2370
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait82,2416
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>new83,2459
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str>;set86,2557
        fn length(&self) -> N;length87,2651
        fn nth_cloned(&self, index: N) -> T;nth_cloned88,2682
        fn empty() -> ArraySeqMtEphS<T>;empty89,2727
        fn singleton(item: T) -> ArraySeqMtEphS<T>;singleton90,2768
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate91,2820
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map92,2888
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;subseq_copy93,2977
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append94,3050
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter95,3136
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;update96,3223
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject97,3312
        fn isEmpty(&self) -> B;isEmpty98,3413
        fn isSingleton(&self) -> B;isSingleton99,3445
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject100,3481
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS103,3590
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>new104,3653
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str> {set111,3824
        fn length(&self) -> N { ArraySeqMtEphS::length(self) }length115,3981
        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphS::nth_cloned(self, index) }nth_cloned117,4045
        fn empty() -> ArraySeqMtEphS<T> { ArraySeqMtEphS::empty() }empty119,4134
        fn singleton(item: T) -> ArraySeqMtEphS<T> { ArraySeqMtEphS::singleton(item) }singleton121,4203
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate123,4291
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map131,4553
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {subseq_copy143,4982
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append147,5128
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter160,5581
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update172,5990
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject177,6151
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty189,6592
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton191,6680
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject193,6772
    macro_rules! ArraySeqMtEphSLit {ArraySeqMtEphSLit204,7125
    fn _ArraySeqMtEphSLit_type_checks() {_ArraySeqMtEphSLit_type_checks211,7538

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStEph.rs,5066
pub mod LinkedListStEph {LinkedListStEph3,60
    pub struct NodeE<T: StT> {NodeE9,183
        pub value: T,value10,214
        pub next: Option<Box<NodeE<T>>>,next11,236
    pub struct LinkedListStEphS<T: StT> {LinkedListStEphS15,312
        head: Option<Box<NodeE<T>>>,head16,354
        len: N,len17,391
    impl<T: StT> LinkedListStEphS<T> {LinkedListStEphS20,414
        pub fn empty() -> Self { LinkedListStEphS { head: None, len: 0 } }empty21,453
        pub fn new(length: N, init_value: T) -> Selfnew23,529
        pub fn singleton(item: T) -> Self { LinkedListStEphS::from_vec(vec![item]) }singleton30,704
        pub fn from_vec(mut elts: Vec<T>) -> Self {from_vec32,790
        pub fn length(&self) -> N { self.len }length41,1117
        pub fn nth(&self, index: N) -> &T {nth43,1165
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set49,1340
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy59,1669
        fn node_at(&self, index: N) -> Option<&NodeE<T>> {node_at89,2751
        fn node_at_mut(&mut self, index: N) -> Option<&mut NodeE<T>> {node_at_mut105,3220
    impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {LinkedListStEphS122,3715
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt123,3776
    impl<T: StT> PartialEq for LinkedListStEphS<T> {LinkedListStEphS140,4309
        fn eq(&self, other: &Self) -> bool {eq141,4362
    impl<T: StT> Eq for LinkedListStEphS<T> {}LinkedListStEphS158,4874
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait160,4922
        fn new(length: N, init_value: T) -> Selfnew161,4967
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set165,5053
        fn length(&self) -> N;length167,5135
        fn nth(&self, index: N) -> &T;nth169,5167
        fn empty() -> Self;empty171,5207
        fn singleton(item: T) -> Self;singleton173,5236
        fn tabulate(f: impl Fn(N) -> T, n: N) -> Self;tabulate177,5368
        fn map<U: StT>(a: &Self, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map180,5519
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy181,5597
        fn append(a: &Self, b: &Self) -> Self;append184,5765
        fn filter(a: &Self, pred: impl Fn(&T) -> B) -> Self;filter187,5908
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten188,5969
        fn update(a: &mut Self, item_at: Pair<N, T>) -> &mut Self;update191,6164
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;inject194,6351
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;ninject197,6548
        fn collect<A: StT, Bv: StT>(collect198,6626
        fn iterate<A: StT>(a: &Self, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate202,6811
        fn iteratePrefixes<A: StT>(a: &Self, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListStEphSiteratePrefixes203,6885
        fn reduce(a: &Self, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce204,6990
        fn scan(a: &Self, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, T);scan205,7057
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS208,7152
        fn new(length: N, init_value: T) -> Selfnew209,7219
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set216,7379
        fn length(&self) -> N { LinkedListStEphS::length(self) }length220,7525
        fn nth(&self, index: N) -> &T { LinkedListStEphS::nth(self, index) }nth222,7591
        fn empty() -> Self { LinkedListStEphS::empty() }empty224,7669
        fn singleton(item: T) -> Self { LinkedListStEphS::singleton(item) }singleton226,7727
        fn tabulate(f: impl Fn(N) -> T, n: N) -> Self {tabulate228,7804
        fn map<U: StT>(a: &Self, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map236,8055
        fn subseq_copy(&self, start: N, length: N) -> Self { LinkedListStEphS::subseq_copy(self,subseq_copy244,8354
        fn append(a: &Self, b: &Self) -> Self {append246,8469
        fn filter(a: &Self, pred: impl Fn(&T) -> B) -> Self {filter257,8853
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten268,9215
        fn update(a: &mut Self, Pair(index, item): Pair<N, T>) -> &mut Self {update279,9619
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {inject284,9762
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {ninject296,10186
        fn collect<A: StT, Bv: StT>(collect305,10490
        fn iterate<A: StT>(a: &Self, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate325,11344
        fn iteratePrefixes<A: StT>(a: &Self, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListStEphSiteratePrefixes333,11567
        fn reduce(a: &Self, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce343,11984
        fn scan(a: &Self, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, T) {scan359,12592

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStPer.rs,4961
pub mod ArraySeqStPer {ArraySeqStPer3,79
    pub struct ArraySeqStPerS<T: StT> {ArraySeqStPerS11,347
        data: Box<[T]>,data12,387
    pub type ArrayStPer<T> = ArraySeqStPerS<T>;ArrayStPer15,418
    impl<T: StT> ArraySeqStPerS<T> {ArraySeqStPerS17,467
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }from_vec18,504
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) new19,593
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }empty20,691
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton21,753
        pub fn length(&self) -> N { self.data.len() }length22,826
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth23,880
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy24,944
    impl<T: StT> PartialEq for ArraySeqStPerS<T> {ArraySeqStPerS33,1287
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }eq34,1338
    impl<T: StT> Eq for ArraySeqStPerS<T> {}ArraySeqStPerS37,1424
    impl<T: StT> Debug for ArraySeqStPerS<T> {ArraySeqStPerS39,1470
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt40,1517
    impl<T: StT> Display for ArraySeqStPerS<T> {ArraySeqStPerS43,1636
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt44,1685
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait57,2098
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>;new58,2141
        fn length(&self) -> N;length59,2204
        fn nth(&self, index: N) -> &T;nth60,2235
        fn empty() -> ArraySeqStPerS<T>;empty61,2274
        fn singleton(item: T) -> ArraySeqStPerS<T>;singleton62,2315
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStPerS<T>;tabulate63,2367
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U>;map64,2440
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T>;subseq_copy65,2529
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append66,2618
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T>;filter67,2704
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;flatten68,2791
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject69,2871
        fn isEmpty(&self) -> B;isEmpty70,2972
        fn isSingleton(&self) -> B;isSingleton71,3004
        fn collect<K: StT, V: StT>(collect72,3040
        fn iterate<A>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A;iterate76,3220
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce77,3305
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, Tscan78,3385
    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {ArraySeqStPerS81,3491
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::new(length, initnew82,3554
        fn length(&self) -> N { ArraySeqStPerS::length(self) }length83,3660
        fn nth(&self, index: N) -> &T { ArraySeqStPerS::nth(self, index) }nth84,3723
        fn empty() -> ArraySeqStPerS<T> { ArraySeqStPerS::empty() }empty85,3798
        fn singleton(item: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::singleton(item) }singleton86,3866
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStPerS<T> {tabulate88,3954
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U> {map96,4231
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T> {subseq_copy104,4539
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {append108,4681
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T> {filter120,5133
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {flatten131,5519
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject142,5912
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty154,6462
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton156,6550
        fn collect<K: StT, V: StT>(collect158,6642
        fn iterate<A>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A {iterate180,7573
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce188,7810
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, Tscan196,8040
    macro_rules! ArraySeqStPerS {ArraySeqStPerS208,8468
    fn _arrayseqstpers_macro_type_checks() {_arrayseqstpers_macro_type_checks221,8954

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEphSlice.rs,3569
pub mod ArraySeqMtEphSlice {ArraySeqMtEphSlice8,395
    struct Inner<T: StT> {Inner16,591
        data: Mutex<Box<[T]>>,data17,618
    impl<T: StT> Inner<T> {Inner20,656
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }new21,684
        fn len(&self) -> N {len23,761
    pub struct ArraySeqMtEphSliceS<T: StT> {ArraySeqMtEphSliceS31,970
        inner: Arc<Inner<T>>,inner32,1015
        range: Range<N>,range33,1045
    pub trait ArraySeqMtEphSliceTrait<T: StT> {ArraySeqMtEphSliceTrait37,1141
        fn new(length: N, init_value: T) -> Self;new38,1189
        fn length(&self) -> N;length39,1239
        fn nth_cloned(&self, index: N) -> T;nth_cloned40,1270
        fn empty() -> Self;empty41,1315
        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;update42,1343
        fn singleton(item: T) -> Self;singleton43,1427
        fn isEmpty(&self) -> B;isEmpty44,1466
        fn isSingleton(&self) -> B;isSingleton45,1498
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy46,1534
        fn slice(&self, start: N, length: N) -> Self;slice47,1594
    impl<T: StT> ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS50,1655
        pub fn from_box(data: Box<[T]>) -> Self {from_box52,1758
        pub fn from_vec(data: Vec<T>) -> Self { Self::from_box(data.into_boxed_slice()) }from_vec61,2060
        pub fn to_vec(&self) -> Vec<T> {to_vec64,2232
        pub fn with_exclusive<F, R>(&self, f: F) -> Rwith_exclusive70,2496
        fn len(&self) -> N { self.range.end - self.range.start }len80,2802
        fn clamp_subrange(&self, start: N, length: N) -> Range<N> {clamp_subrange82,2868
    impl<T: StT> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS91,3228
        fn new(length: N, init_value: T) -> Self {new92,3301
        fn length(&self) -> N { self.len() }length97,3466
        fn nth_cloned(&self, index: N) -> T {nth_cloned99,3512
        fn empty() -> Self { ArraySeqMtEphSliceS::from_vec(Vec::new()) }empty105,3705
        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {update107,3779
        fn singleton(item: T) -> Self { ArraySeqMtEphSliceS::from_vec(vec![item]) }singleton119,4178
        fn isEmpty(&self) -> B { if self.len() == 0 { B::True } else { B::False } }isEmpty121,4263
        fn isSingleton(&self) -> B { if self.len() == 1 { B::True } else { B::False } }isSingleton123,4348
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy125,4437
        fn slice(&self, start: N, length: N) -> Self {slice132,4756
    impl<T: StT> Clone for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS141,5010
        fn clone(&self) -> Self {clone142,5062
    impl<T: StT> PartialEq for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS150,5252
        fn eq(&self, other: &Self) -> bool {eq151,5308
    impl<T: StT> Eq for ArraySeqMtEphSliceS<T> {}ArraySeqMtEphSliceS164,5726
    impl<T: StT> Debug for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS166,5777
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt167,5829
    impl<T: StT> Display for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS175,6096
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt176,6150
    fn repeat_vec<T: StT>(length: N, init: T) -> Vec<T> {repeat_vec191,6617
    macro_rules! ArraySeqMtEphSliceSLit {ArraySeqMtEphSliceSLit200,6842
    fn _ArraySeqMtEphSliceSLit_type_checks() {_ArraySeqMtEphSliceSLit_type_checks207,7305

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStEph.rs,3446
pub mod ArraySeqStEph {ArraySeqStEph3,51
    pub type ArraySeqStEphS<T> = ArrayS<T>;ArraySeqStEphS8,172
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait10,217
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>new11,260
        fn empty() -> ArraySeqStEphS<T>;empty14,358
        fn singleton(item: T) -> ArraySeqStEphS<T>;singleton15,399
        fn length(&self) -> N;length16,451
        fn nth(&self, index: N) -> &T;nth17,482
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T>;subseq_copy18,521
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate22,687
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map25,851
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T>;select28,1032
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append31,1228
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append234,1422
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T>;deflate37,1601
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter40,1766
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate41,1853
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce42,1940
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan43,2020
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten44,2119
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS47,2206
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>new48,2269
        fn empty() -> ArraySeqStEphS<T> { ArraySeq::empty() }empty55,2434
        fn singleton(item: T) -> ArraySeqStEphS<T> { ArraySeq::singleton(item) }singleton57,2497
        fn length(&self) -> N { ArraySeq::length(self) }length59,2579
        fn nth(&self, index: N) -> &T { ArraySeq::nth(self, index) }nth61,2637
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T> { ArraySeq::subseq_copy(subseq_copy63,2707
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> { ArraySeq::tabulate(f, n) }tabulate65,2827
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> { ArraySmap67,2924
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T> {select69,3037
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> { ArraySeq:append83,3517
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> { ArraySeqappend285,3630
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T> {deflate87,3744
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> { ArraySeqfilter95,3974
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { ArraySeqiterate97,4091
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T { ArraySeq::reducreduce99,4209
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan101,4320
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> { ArraySeq::flattflatten105,4468

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtPer.rs,4911
pub mod ArraySeqMtPer {ArraySeqMtPer3,132
    pub trait ArraySeqMtPerTrait<T: MtT> {ArraySeqMtPerTrait9,335
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;new11,409
        fn empty() -> ArraySeqMtPerS<T>;empty12,472
        fn singleton(item: T) -> ArraySeqMtPerS<T>;singleton13,513
        fn length(&self) -> N;length14,565
        fn nth(&self, index: N) -> &T;nth15,596
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;subseq_copy16,635
        fn update(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str>;update17,708
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T>;tabulate19,797
        fn map<W: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W) -> ArraySeqMtPerS<W>;map20,865
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append21,954
        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtPerS<T>;filter22,1040
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T>;update23,1127
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject24,1211
        fn iterate<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate25,1313
        fn iteratePrefixes<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes26,1400
        fn reduce(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce27,1516
        fn scan(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtPerS<T>, Tscan28,1596
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;flatten29,1695
        fn collect(collect30,1776
        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMinject36,1978
        fn atomicWrite(atomicWrite37,2084
        fn inject_parallel2(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) ->inject_parallel242,2270
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins43,2386
        fn ninject_parallel2(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -ninject_parallel248,2597
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins49,2714
    impl<T: MtT> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {ArraySeqMtPerS56,2933
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerTraitChap18::new(lenew57,2996
        fn empty() -> ArraySeqMtPerS<T> { ArraySeqMtPerTraitChap18::empty() }empty59,3113
        fn singleton(item: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerTraitChap18::singleton(item) }singleton61,3192
        fn length(&self) -> N { ArraySeqMtPerTraitChap18::length(self) }length63,3290
        fn nth(&self, index: N) -> &T { ArraySeqMtPerTraitChap18::nth(self, index) }nth65,3364
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> {subseq_copy67,3450
        fn update(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str> {update71,3606
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T> { ArraySeqMtPerTraitChap18::ttabulate75,3770
        fn map<W: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W) -> ArraySeqMtPerS<W> {map77,3883
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append81,4032
        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtPerS<T> {filter85,4181
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T> {update89,4334
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject93,4487
        fn iterate<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate97,4659
        fn iteratePrefixes<A: MtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes101,4813
        fn reduce(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce105,5004
        fn scan(a: &ArraySeqMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtPerS<T>, Tscan109,5151
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {flatten113,5315
        fn collect(collect117,5458
        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMinject124,5684
        fn atomicWrite(atomicWrite128,5864
        fn inject_parallel2(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) ->inject_parallel2136,6163
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins140,6353
        fn ninject_parallel2(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -ninject_parallel2161,7236
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins165,7428

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEph.rs,3484
pub mod ArraySeqMtEph {ArraySeqMtEph3,67
    pub struct ArraySeqMtEphS<T: StT> {ArraySeqMtEphS10,262
        data: Mutex<Box<[T]>>,data11,302
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait14,340
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>new15,383
        fn empty() -> ArraySeqMtEphS<T>;empty18,481
        fn singleton(item: T) -> ArraySeqMtEphS<T>;singleton19,522
        fn length(&self) -> N;length20,574
        fn nth_cloned(&self, index: N) -> T;nth_cloned21,605
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;subseq_copy22,650
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate24,724
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map25,792
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T>;select26,881
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append27,969
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append228,1055
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T>;deflate29,1142
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter30,1211
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate31,1298
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce32,1385
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtEphS<T>, Tscan33,1465
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;flatten34,1564
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS37,1651
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>new38,1714
        fn empty() -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::empty() }empty45,1889
        fn singleton(item: T) -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::singleton(item) }singleton47,1962
        fn length(&self) -> N { ArraySeqMtEphTrait::length(self) }length49,2054
        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphTrait::nth_cloned(self, index) }nth_cloned51,2122
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {subseq_copy53,2215
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::tabulattabulate57,2365
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map59,2472
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T> {select63,2615
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append77,3133
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append281,3276
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T> {deflate85,3420
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter93,3670
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, mut x: A) -> A {iterate97,3817
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> T {reduce104,4070
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> (ArraySeqMtEphS<Tscan111,4319
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> { ArraySeqMtEphTrflatten125,4845

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStPer.rs,3911
pub mod ArraySeqStPer {ArraySeqStPer3,46
    pub type ArraySeqStPerS<T> = ArrayS<T>;ArraySeqStPerS7,166
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait9,211
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>new10,254
        fn empty() -> ArraySeqStPerS<T>;empty13,352
        fn singleton(item: T) -> ArraySeqStPerS<T>;singleton14,393
        fn length(&self) -> N;length15,445
        fn nth(&self, index: N) -> &T;nth16,476
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T>;subseq_copy17,515
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStPerS<T>;tabulate20,675
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U>;map22,823
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>select24,953
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append26,1104
        fn append2(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append228,1243
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStPerS<T>;deflate30,1371
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T>;filter32,1527
        fn iterate<A: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate33,1614
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce34,1701
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, Tscan35,1781
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;flatten36,1880
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject37,1960
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject38,2061
    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {ArraySeqStPerS41,2170
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>new42,2233
        fn empty() -> ArraySeqStPerS<T> { ArraySeq::empty() }empty49,2398
        fn singleton(item: T) -> ArraySeqStPerS<T> { ArraySeq::singleton(item) }singleton51,2461
        fn length(&self) -> N { ArraySeq::length(self) }length53,2543
        fn nth(&self, index: N) -> &T { ArraySeq::nth(self, index) }nth55,2601
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T> { ArraySeq::subseq_copy(subseq_copy57,2671
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStPerS<T> { ArraySeq::tabulate(f, n) }tabulate59,2791
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U> { ArraySmap61,2888
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>select63,3001
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> { ArraySeq:append77,3463
        fn append2(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> { ArraySeqappend279,3576
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStPerS<T> {deflate81,3690
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T> { ArraySeqfilter89,3920
        fn iterate<A: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { ArraySeqiterate91,4037
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T { ArraySeq::reducreduce93,4155
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, Tscan95,4266
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> { ArraySeq::flattflatten99,4414
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject101,4519
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject105,4673

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap23/PrimTreeSeqSt.rs,984
pub mod PrimTreeSeqSt {PrimTreeSeqSt9,476
    pub enum PrimTreeSeqStTree<T: StT> {PrimTreeSeqStTree14,655
        Zero,Zero15,696
        One(T),One16,710
        Two(PrimTreeSeqStS<T>, PrimTreeSeqStS<T>),Two17,726
    pub struct PrimTreeSeqStS<T: StT> {PrimTreeSeqStS22,905
        data: Vec<T>,data23,945
    impl<T: StT> PrimTreeSeqStS<T> {PrimTreeSeqStS26,974
        pub fn empty() -> Self { Self { data: Vec::new() } }empty28,1050
        pub fn singleton(value: T) -> Self { Self { data: vec![value] } }singleton31,1171
        pub fn from_vec(vec: Vec<T>) -> Self { Self { data: vec } }from_vec34,1333
        pub fn into_vec(self) -> Vec<T> { self.data }into_vec37,1462
        pub fn as_slice(&self) -> &[T] { &self.data }as_slice40,1580
        pub fn length(&self) -> N { self.data.len() }length43,1695
        pub fn expose(&self) -> PrimTreeSeqStTree<T> {expose46,1827
        pub fn join(tree: PrimTreeSeqStTree<T>) -> Self {join60,2451

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap23/BBTEph.rs,2136
pub mod BBTEph {BBTEph3,56
    pub enum BBTree<T: StT> {BBTree9,266
        Leaf,Leaf10,296
        Node(Box<BBNode<T>>),Node11,310
    pub struct BBNode<T: StT> {BBNode15,390
        pub(crate) left: BBTree<T>,left16,422
        pub(crate) value: T,value17,458
        pub(crate) right: BBTree<T>,right18,487
    impl<T: StT> BBNode<T> {BBNode21,531
        fn new(left: BBTree<T>, value: T, right: BBTree<T>) -> Self { BBNode { left, value, righnew22,560
    pub trait BBTEphTrait<T: StT> {BBTEphTrait25,669
        fn leaf() -> Self;leaf26,705
        fn node(left: Self, value: T, right: Self) -> Self;node27,732
        fn is_leaf(&self) -> B;is_leaf28,792
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order29,824
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order30,873
        fn height(&self) -> N;height31,923
        fn size(&self) -> N;size32,954
    impl<T: StT> BBTree<T> {BBTree35,990
        pub fn leaf() -> Self { BBTree::Leaf }leaf36,1019
        pub fn node(left: BBTree<T>, value: T, right: BBTree<T>) -> Self {node38,1067
        pub fn is_leaf(&self) -> B {is_leaf42,1221
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order49,1398
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order62,2066
        pub fn height(&self) -> N {height75,2735
        pub fn size(&self) -> N {size86,3070
    impl<T: StT> BBTEphTrait<T> for BBTree<T> {BBTree94,3279
        fn leaf() -> Self { BBTree::leaf() }leaf95,3327
        fn node(left: Self, value: T, right: Self) -> Self { BBTree::node(left, value, right) }node97,3373
        fn is_leaf(&self) -> B { BBTree::is_leaf(self) }is_leaf99,3470
        fn in_order(&self) -> ArraySeqStPerS<T> { BBTree::in_order(self) }in_order101,3528
        fn pre_order(&self) -> ArraySeqStPerS<T> { BBTree::pre_order(self) }pre_order103,3604
        fn height(&self) -> N { BBTree::height(self) }height105,3682
        fn size(&self) -> N { BBTree::size(self) }size107,3738
    macro_rules! BBNodeLit {BBNodeLit111,3816
    fn _BBNodeLit_type_checks() {_BBNodeLit_type_checks118,4042

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortMtSlice.rs,1381
pub mod Chapter36MtSlice {Chapter36MtSlice3,98
    pub trait Chapter36MtSliceTrait<T: StT + Ord + Send> {Chapter36MtSliceTrait13,306
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first14,365
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median315,418
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random16,473
        fn quick_sort_mt_first(&self);quick_sort_mt_first18,528
        fn quick_sort_mt_median3(&self);quick_sort_mt_median319,567
        fn quick_sort_mt_random(&self);quick_sort_mt_random20,608
    impl<T: StT + Ord + Send> Chapter36MtSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS23,655
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first24,739
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median326,817
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random40,1288
        fn quick_sort_mt_first(&self) {quick_sort_mt_first46,1464
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort51,1624
        fn quick_sort_mt_median3(&self) {quick_sort_mt_median388,2987
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort93,3149
        fn quick_sort_mt_random(&self) {quick_sort_mt_random143,5077
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort148,5238

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortSt.rs,1495
pub mod Chapter36St {Chapter36St3,95
    pub trait Chapter36StTrait<T: StT + Ord> {Chapter36StTrait9,233
        fn pivot_st_first(&self, lo: N, hi: N) -> T;pivot_st_first10,280
        fn pivot_st_median3(&self, lo: N, hi: N) -> T;pivot_st_median311,333
        fn pivot_st_random(&self, lo: N, hi: N) -> T;pivot_st_random12,388
        fn quick_sort_st_first(&mut self);quick_sort_st_first14,443
        fn quick_sort_st_median3(&mut self);quick_sort_st_median315,486
        fn quick_sort_st_random(&mut self);quick_sort_st_random16,531
    impl<T: StT + Ord> Chapter36StTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS19,582
        fn pivot_st_first(&self, lo: N, _hi: N) -> T { self.nth(lo).clone() }pivot_st_first20,649
        fn pivot_st_median3(&self, lo: N, hi: N) -> T {pivot_st_median321,727
        fn pivot_st_random(&self, lo: N, hi: N) -> T {pivot_st_random34,1200
        fn quick_sort_st_first(&mut self) {quick_sort_st_first40,1377
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort41,1421
        fn quick_sort_st_median3(&mut self) {quick_sort_st_median375,2604
            fn median3<T: StT + Ord>(a: &ArraySeqStEphS<T>, lo: N, hi: N) -> T {median376,2650
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort89,3187
        fn quick_sort_st_random(&mut self) {quick_sort_st_random123,4371
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort124,4416

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortMt.rs,1405
pub mod Chapter36Mt {Chapter36Mt3,94
    pub trait Chapter36MtTrait<T: StT + Ord + Send> {Chapter36MtTrait25,1185
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first26,1239
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median327,1292
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random28,1347
        fn quick_sort_mt_first(&mut self);quick_sort_mt_first30,1402
        fn quick_sort_mt_median3(&mut self);quick_sort_mt_median331,1445
        fn quick_sort_mt_random(&mut self);quick_sort_mt_random32,1490
    impl<T: StT + Ord + Send> Chapter36MtTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS35,1541
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first36,1615
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median337,1692
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random50,2162
        fn quick_sort_mt_first(&mut self) {quick_sort_mt_first56,2338
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort61,2462
        fn quick_sort_mt_median3(&mut self) {quick_sort_mt_median399,3808
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort104,3934
        fn quick_sort_mt_random(&mut self) {quick_sort_mt_random154,5747
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort159,5872

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetAVLMtEph.rs,5590
pub mod BSTSetAVLMtEph {BSTSetAVLMtEph3,73
    pub struct BSTSetAVLMtEph<T: StTInMtT + Ord> {BSTSetAVLMtEph11,314
        tree: BSTAVLMtEph<T>,tree12,365
    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;BSTSetAVLMt15,402
    pub trait BSTSetAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetAVLMtEphTrait17,452
        fn empty() -> Self;empty18,514
        fn singleton(value: T) -> Self;singleton19,542
        fn size(&self) -> N;size20,582
        fn is_empty(&self) -> B;is_empty21,611
        fn find(&self, value: &T) -> Option<T>;find22,644
        fn contains(&self, value: &T) -> B;contains23,692
        fn minimum(&self) -> Option<T>;minimum24,736
        fn maximum(&self) -> Option<T>;maximum25,776
        fn insert(&mut self, value: T);insert26,816
        fn delete(&mut self, target: &T);delete27,856
        fn union(&self, other: &Self) -> Self;union28,898
        fn intersection(&self, other: &Self) -> Self;intersection29,945
        fn difference(&self, other: &Self) -> Self;difference30,999
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1051
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1106
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1161
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1223
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1293
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1361
        fn as_tree(&self) -> &BSTAVLMtEph<T>;as_tree37,1415
    impl<T: StTInMtT + Ord> BSTSetAVLMtEph<T> {BSTSetAVLMtEph40,1468
        pub fn empty() -> Self {empty41,1516
        pub fn singleton(value: T) -> Self {singleton47,1635
        pub fn size(&self) -> N { self.tree.size() }size53,1792
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1846
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1908
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,1986
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2064
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2132
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2200
        pub fn delete(&mut self, target: &T) {delete67,2273
        pub fn union(&self, other: &Self) -> Self {union75,2562
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2861
        pub fn difference(&self, other: &Self) -> Self {difference100,3439
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4016
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4708
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5021
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5377
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5786
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6050
        pub fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree179,6133
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6198
        fn rebuild_from_vec(values: Vec<T>) -> BSTAVLMtEph<T> {rebuild_from_vec183,6289
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6508
    impl<T: StTInMtT + Ord> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {BSTSetAVLMtEph203,6791
        fn empty() -> Self { Self::empty() }empty204,6866
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6912
        fn size(&self) -> N { self.tree.size() }size208,6979
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7029
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7087
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7161
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7235
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7299
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7363
        fn delete(&mut self, target: &T) { BSTSetAVLMtEph::delete(self, target) }delete222,7432
        fn union(&self, other: &Self) -> Self { BSTSetAVLMtEph::union(self, other) }union224,7515
        fn intersection(&self, other: &Self) -> Self { BSTSetAVLMtEph::intersection(self, other)intersection226,7601
        fn difference(&self, other: &Self) -> Self { BSTSetAVLMtEph::difference(self, other) }difference228,7701
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetAVLMtEph::split(self, pivot) }split230,7797
        fn join_pair(left: Self, right: Self) -> Self { BSTSetAVLMtEph::join_pair(left, right) }join_pair232,7891
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetAVLMtEph::join_m(left, pivojoin_m234,7989
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetAVLMtEph::filter(sefilter236,8098
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetAVLMtEph::reduce(selfreduce238,8212
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8323
        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree242,8402
    macro_rules! BSTSetAVLMtEphLit {BSTSetAVLMtEphLit246,8489
    fn _BSTSetAVLMtEphLit_type_checks() {_BSTSetAVLMtEphLit_type_checks258,9036

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStPer.rs,4232
pub mod AVLTreeSeqStPer {AVLTreeSeqStPer3,85
    type Link<T> = Option<Rc<Node<T>>>;Link10,247
    struct Node<T: StT> {Node12,288
        value: T,value13,314
        height: N,height14,332
        size: N,size15,351
        left: Link<T>,left16,368
        right: Link<T>,right17,391
    fn height<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.height) }height20,422
    fn size<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.size) }size21,501
    fn mk<T: StT>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {mk23,577
    fn rotate_right<T: StT>(y: Rc<Node<T>>) -> Rc<Node<T>> {rotate_right36,922
    fn rotate_left<T: StT>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left43,1229
    fn rebalance<T: StT>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance50,1535
    fn nth_ref<'a, T: StT>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref72,2388
    fn set_rec<T: StT>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> {set_rec87,2837
    fn inorder_collect<T: StT>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect105,3609
    fn build_balanced_from_slice<T: StT>(a: &[T]) -> Link<T> {build_balanced_from_slice113,3849
        fn rec<T: StT>(a: &[T]) -> Link<T> {rec114,3912
    pub struct AVLTreeSeqStPerS<T: StT> {AVLTreeSeqStPerS126,4230
        root: Link<T>,root127,4272
    pub trait AVLTreeSeqStPerTrait<T: StT> {AVLTreeSeqStPerTrait130,4302
        fn empty() -> Self;empty132,4388
        fn new() -> Self;new134,4457
        fn length(&self) -> N;length136,4524
        fn nth(&self, index: N) -> &T;nth138,4604
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set140,4753
        fn singleton(item: T) -> Self;singleton144,4904
        fn isEmpty(&self) -> B;isEmpty146,4984
        fn isSingleton(&self) -> B;isSingleton148,5057
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy150,5150
        fn from_vec(values: Vec<T>) -> Self;from_vec152,5273
        fn values_in_order(&self) -> Vec<T>;values_in_order154,5362
    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS157,5414
        fn empty() -> Self { AVLTreeSeqStPerS { root: None } }empty158,5481
        fn new() -> Self { Self::empty() }new159,5544
        fn length(&self) -> N { size(&self.root) }length160,5587
        fn nth(&self, index: N) -> &T { nth_ref(&self.root, index) }nth161,5638
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set162,5707
        fn singleton(item: T) -> Self {singleton167,5896
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty172,6041
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton173,6128
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy174,6219
        fn from_vec(values: Vec<T>) -> Self {from_vec188,6727
        fn values_in_order(&self) -> Vec<T> {values_in_order193,6890
    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS200,7081
        fn eq(&self, other: &Self) -> bool {eq201,7134
    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}AVLTreeSeqStPerS213,7460
    impl<T: StT> std::fmt::Debug for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS215,7508
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt216,7567
    impl<T: StT> AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS222,7759
        pub fn to_arrayseq(&self) -> ArraySeqStPerS<T> {to_arrayseq223,7798
        pub fn iter<'a>(&'a self) -> AVLTreeSeqStPerIter<'a, T> {iter228,7950
    pub struct AVLTreeSeqStPerIter<'a, T: StT> {AVLTreeSeqStPerIter236,8163
        stack: Vec<&'a Node<T>>,stack237,8212
        current: Option<&'a Node<T>>,current238,8245
    impl<'a, T: StT> AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter241,8290
        fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {push_left242,8340
    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter250,8550
        type Item = &'a T;Item251,8613
        fn next(&mut self) -> Option<Self::Item> {next252,8640
macro_rules! AVLTreeSeqStPer {AVLTreeSeqStPer266,9030

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetPlainMtEph.rs,5277
pub mod BSTSetPlainMtEph {BSTSetPlainMtEph3,75
    pub struct BSTSetPlainMtEph<T: StTInMtT + Ord> {BSTSetPlainMtEph11,324
        tree: BSTPlainMtEph<T>,tree12,377
    pub type BSTSetPlainMt<T> = BSTSetPlainMtEph<T>;BSTSetPlainMt15,416
    pub trait BSTSetPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetPlainMtEphTrait17,470
        fn empty() -> Self;empty18,534
        fn singleton(value: T) -> Self;singleton19,562
        fn size(&self) -> N;size20,602
        fn is_empty(&self) -> B;is_empty21,631
        fn find(&self, value: &T) -> Option<T>;find22,664
        fn contains(&self, value: &T) -> B;contains23,712
        fn minimum(&self) -> Option<T>;minimum24,756
        fn maximum(&self) -> Option<T>;maximum25,796
        fn insert(&mut self, value: T);insert26,836
        fn delete(&mut self, target: &T);delete27,876
        fn union(&self, other: &Self) -> Self;union28,918
        fn intersection(&self, other: &Self) -> Self;intersection29,965
        fn difference(&self, other: &Self) -> Self;difference30,1019
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1071
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1126
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1181
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1243
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1313
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1381
        fn as_tree(&self) -> &BSTPlainMtEph<T>;as_tree37,1435
    impl<T: StTInMtT + Ord> BSTSetPlainMtEph<T> {BSTSetPlainMtEph40,1490
        pub fn empty() -> Self {empty41,1540
        pub fn singleton(value: T) -> Self {singleton47,1661
        pub fn size(&self) -> N { self.tree.size() }size53,1820
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1874
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1936
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,2014
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2092
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2160
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2228
        pub fn delete(&mut self, target: &T) {delete67,2301
        pub fn union(&self, other: &Self) -> Self {union75,2590
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2889
        pub fn difference(&self, other: &Self) -> Self {difference100,3467
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4044
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4736
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5049
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5405
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5814
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6078
        pub fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree179,6161
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6228
        fn rebuild_from_vec(values: Vec<T>) -> BSTPlainMtEph<T> {rebuild_from_vec183,6319
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6542
    impl<T: StTInMtT + Ord> BSTSetPlainMtEphTrait<T> for BSTSetPlainMtEph<T> {BSTSetPlainMtEph203,6827
        fn empty() -> Self { Self::empty() }empty204,6906
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6952
        fn size(&self) -> N { self.tree.size() }size208,7019
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7069
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7127
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7201
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7275
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7339
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7403
        fn delete(&mut self, target: &T) {delete222,7472
        fn union(&self, other: &Self) -> Self {union230,7757
        fn intersection(&self, other: &Self) -> Self {intersection238,8052
        fn difference(&self, other: &Self) -> Self {difference255,8626
        fn split(&self, pivot: &T) -> (Self, B, Self) {split272,9199
        fn join_pair(left: Self, right: Self) -> Self {join_pair292,9887
        fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m300,10196
        fn filter<F>(&self, predicate: F) -> Selffilter309,10548
        fn reduce<F>(&self, op: F, base: T) -> Treduce316,10709
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order323,10867
        fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree325,10946
    macro_rules! BSTSetPlainMtEphLit {BSTSetPlainMtEphLit329,11035
    fn _BSTSetPlainMtEphLit_type_checks() {_BSTSetPlainMtEphLit_type_checks341,11608

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBStEph.rs,4730
pub mod BSTRBStEph {BSTRBStEph3,93
    enum Color {Color9,301
        Red,Red10,318
        Black,Black11,331
    type Link<T> = Option<Box<Node<T>>>;Link14,353
    struct Node<T: StT + Ord> {Node17,423
        key: T,key18,455
        color: Color,color19,471
        size: N,size20,493
        left: Link<T>,left21,510
        right: Link<T>,right22,533
    impl<T: StT + Ord> Node<T> {Node25,564
        fn new(key: T) -> Self {new26,597
    pub struct BSTRBStEph<T: StT + Ord> {BSTRBStEph38,846
        root: Link<T>,root39,888
    pub type BSTreeRB<T> = BSTRBStEph<T>;BSTreeRB42,918
    pub trait BSTRBStEphTrait<T: StT + Ord> {BSTRBStEphTrait44,961
        fn new() -> Self;new45,1007
        fn size(&self) -> N;size46,1033
        fn is_empty(&self) -> B;is_empty47,1062
        fn height(&self) -> N;height48,1095
        fn insert(&mut self, value: T);insert49,1126
        fn find(&self, target: &T) -> Option<&T>;find50,1166
        fn contains(&self, target: &T) -> B;contains51,1216
        fn minimum(&self) -> Option<&T>;minimum52,1261
        fn maximum(&self) -> Option<&T>;maximum53,1302
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order54,1343
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order55,1392
    impl<T: StT + Ord> Default for BSTRBStEph<T> {BSTRBStEph58,1449
        fn default() -> Self { Self::new() }default59,1500
    impl<T: StT + Ord> BSTRBStEph<T> {BSTRBStEph62,1552
        pub fn new() -> Self { BSTRBStEph { root: None } }new63,1591
        pub fn size(&self) -> N { Self::size_link(&self.root) }size65,1651
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty67,1716
        pub fn height(&self) -> N {height69,1807
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec70,1843
        pub fn insert(&mut self, value: T) {insert79,2139
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find86,2359
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains88,2453
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum90,2567
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum92,2643
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order94,2719
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order100,2943
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red106,3169
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link108,3271
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate110,3354
        fn rotate_left(link: &mut Link<T>) {rotate_left112,3473
        fn rotate_right(link: &mut Link<T>) {rotate_right129,4069
        fn flip_colors(link: &mut Link<T>) {flip_colors146,4667
        fn fix_up(link: &mut Link<T>) {fix_up167,5472
        fn insert_link(link: &mut Link<T>, value: T) {insert_link184,6159
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link200,6701
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link215,7218
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link225,7539
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect235,7862
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect243,8149
    impl<T: StT + Ord> BSTRBStEphTrait<T> for BSTRBStEph<T> {BSTRBStEph252,8445
        fn new() -> Self { BSTRBStEph::new() }new253,8507
        fn size(&self) -> N { BSTRBStEph::size(self) }size255,8555
        fn is_empty(&self) -> B { BSTRBStEph::is_empty(self) }is_empty257,8611
        fn height(&self) -> N { BSTRBStEph::height(self) }height259,8675
        fn insert(&mut self, value: T) { BSTRBStEph::insert(self, value) }insert261,8735
        fn find(&self, target: &T) -> Option<&T> { BSTRBStEph::find(self, target) }find263,8811
        fn contains(&self, target: &T) -> B { BSTRBStEph::contains(self, target) }contains265,8896
        fn minimum(&self) -> Option<&T> { BSTRBStEph::minimum(self) }minimum267,8980
        fn maximum(&self) -> Option<&T> { BSTRBStEph::maximum(self) }maximum269,9051
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTRBStEph::in_order(self) }in_order271,9122
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTRBStEph::pre_order(self) }pre_order273,9202
    macro_rules! BSTRBStEphLit {BSTRBStEphLit277,9310
    fn _BSTRBStEphLit_type_checks() {_BSTRBStEphLit_type_checks289,9804

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeq.rs,5331
pub mod AVLTreeSeq {AVLTreeSeq9,536
    type Link<T> = Option<Box<AVLTreeNode<T>>>;Link15,666
    pub struct AVLTreeNode<T: Copy + Debug> {AVLTreeNode17,715
        pub value: T,value18,761
        pub height: N,height19,783
        pub left_size: N,left_size20,806
        pub right_size: N,right_size21,832
        pub left: Link<T>,left22,859
        pub right: Link<T>,right23,886
        pub index: N,index24,914
    impl<T: Copy + Debug> AVLTreeNode<T> {AVLTreeNode27,943
        fn new(value: T, index: N) -> Self {new28,986
    pub struct AVLTreeS<T: Copy + Debug> {AVLTreeS41,1279
        pub root: Link<T>,root42,1322
        pub next_key: N,next_key43,1349
    pub trait AVLTreeSeq<T: Copy + Debug> {AVLTreeSeq46,1381
        fn empty() -> AVLTreeS<T>;empty49,1504
        fn new() -> AVLTreeS<T>;new53,1627
        fn length(&self) -> N;length57,1742
        fn nth(&self, index: N) -> &T;nth61,1916
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str>;set65,2065
        fn singleton(item: T) -> AVLTreeS<T>;singleton69,2240
        fn isEmpty(&self) -> B;isEmpty73,2353
        fn isSingleton(&self) -> B;isSingleton74,2385
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>subseq_copy78,2552
    impl<T: Copy + Debug> AVLTreeS<T> {AVLTreeS83,2666
        pub fn new_root() -> Self {new_root84,2706
        pub fn new() -> Self { Self::new_root() }new90,2846
        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeS<T> {update92,2897
        pub fn from_vec(values: Vec<T>) -> AVLTreeS<T>from_vec97,3079
        pub fn to_arrayseq(&self) -> ArrayS<T>to_arrayseq110,3491
        pub fn iter<'a>(&'a self) -> AVLTreeSeqIter<'a, T> { AVLTreeSeqIter::new(&self.root) }iter129,4093
        pub fn push_back(&mut self, value: T) {push_back131,4189
        pub fn contains_value(&self, target: &T) -> Bcontains_value138,4497
        pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value150,4769
        pub fn delete_value(&mut self, target: &T) -> booldelete_value152,4846
        pub fn is_tree_empty(&self) -> bool { self.length() == 0 }is_tree_empty180,5790
        pub fn values_in_order(&self) -> Vec<T>values_in_order182,5858
    impl<T: Copy + Debug> AVLTreeSeq<T> for AVLTreeS<T> {AVLTreeS192,6094
        fn empty() -> AVLTreeS<T> { AVLTreeS::new_root() }empty194,6194
        fn new() -> AVLTreeS<T> { AVLTreeS::new_root() }new197,6296
        fn length(&self) -> N { size_link(&self.root) }length200,6396
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth203,6503
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str> {set206,6624
        fn singleton(item: T) -> AVLTreeS<T> {singleton212,6839
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty219,7077
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton221,7206
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>subseq_copy224,7360
    impl<T: Eq + Copy + Debug> PartialEq for AVLTreeS<T> {AVLTreeS242,7911
        fn eq(&self, other: &Self) -> bool {eq243,7970
    impl<T: Eq + Copy + Debug> Eq for AVLTreeS<T> {}AVLTreeS256,8297
    impl<T: Debug + Copy> std::fmt::Debug for AVLTreeS<T> {AVLTreeS258,8351
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt259,8411
    impl<T: std::fmt::Display + Copy + Debug> std::fmt::Display for AVLTreeS<T> {AVLTreeS265,8619
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt266,8701
    pub struct AVLTreeSeqIter<'a, T: Copy + Debug> {AVLTreeSeqIter282,9113
        stack: Vec<&'a AVLTreeNode<T>>,stack283,9166
        current: Option<&'a AVLTreeNode<T>>,current284,9206
    impl<'a, T: Copy + Debug> AVLTreeSeqIter<'a, T> {AVLTreeSeqIter287,9258
        fn new(root: &'a Link<T>) -> Self {new288,9312
        fn push_left(&mut self, link: &'a Link<T>) {push_left297,9537
    impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIter<'a, T> {AVLTreeSeqIter306,9789
        type Item = &'a T;Item307,9856
        fn next(&mut self) -> Option<Self::Item> {next308,9883
    fn h<T: Copy + Debug>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h318,10142
    fn size_link<T: Copy + Debug>(n: &Link<T>) -> N {size_link319,10225
    fn update_meta<T: Copy + Debug>(n: &mut Box<AVLTreeNode<T>>) {update_meta327,10399
    fn rotate_right<T: Copy + Debug>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right335,10653
    fn rotate_left<T: Copy + Debug>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left348,11027
    fn rebalance<T: Copy + Debug>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance361,11399
    pub(crate) fn insert_at_link<T: Copy + Debug>(node: Link<T>, index: N, value: T, next_key: &insert_at_link385,12299
    fn nth_link<'a, T: Copy + Debug>(node: &'a Link<T>, index: N) -> &'a T {nth_link405,13104
    fn set_link<T: Copy + Debug>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static set_link417,13493
    fn push_inorder<T: Copy + Debug>(link: &Link<T>, out: &mut Vec<T>)push_inorder434,14092

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetSplayMtEph.rs,5645
pub mod BSTSetSplayMtEph {BSTSetSplayMtEph3,75
    pub struct BSTSetSplayMtEph<T: StTInMtT + Ord> {BSTSetSplayMtEph11,324
        tree: BSTSplayMtEph<T>,tree12,377
    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;BSTSetSplayMt15,416
    pub trait BSTSetSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetSplayMtEphTrait17,470
        fn empty() -> Self;empty18,534
        fn singleton(value: T) -> Self;singleton19,562
        fn size(&self) -> N;size20,602
        fn is_empty(&self) -> B;is_empty21,631
        fn find(&self, value: &T) -> Option<T>;find22,664
        fn contains(&self, value: &T) -> B;contains23,712
        fn minimum(&self) -> Option<T>;minimum24,756
        fn maximum(&self) -> Option<T>;maximum25,796
        fn insert(&mut self, value: T);insert26,836
        fn delete(&mut self, target: &T);delete27,876
        fn union(&self, other: &Self) -> Self;union28,918
        fn intersection(&self, other: &Self) -> Self;intersection29,965
        fn difference(&self, other: &Self) -> Self;difference30,1019
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1071
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1126
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1181
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1243
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1313
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1381
        fn as_tree(&self) -> &BSTSplayMtEph<T>;as_tree37,1435
    impl<T: StTInMtT + Ord> BSTSetSplayMtEph<T> {BSTSetSplayMtEph40,1490
        pub fn empty() -> Self {empty41,1540
        pub fn singleton(value: T) -> Self {singleton47,1661
        pub fn size(&self) -> N { self.tree.size() }size53,1820
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1874
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1936
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,2014
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2092
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2160
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2228
        pub fn delete(&mut self, target: &T) {delete67,2301
        pub fn union(&self, other: &Self) -> Self {union75,2590
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2889
        pub fn difference(&self, other: &Self) -> Self {difference100,3467
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4044
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4736
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5049
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5405
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5814
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6078
        pub fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree179,6161
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6228
        fn rebuild_from_vec(values: Vec<T>) -> BSTSplayMtEph<T> {rebuild_from_vec183,6319
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6542
    impl<T: StTInMtT + Ord> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {BSTSetSplayMtEph203,6827
        fn empty() -> Self { Self::empty() }empty204,6906
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6952
        fn size(&self) -> N { self.tree.size() }size208,7019
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7069
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7127
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7201
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7275
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7339
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7403
        fn delete(&mut self, target: &T) { BSTSetSplayMtEph::delete(self, target) }delete222,7472
        fn union(&self, other: &Self) -> Self { BSTSetSplayMtEph::union(self, other) }union224,7557
        fn intersection(&self, other: &Self) -> Self { BSTSetSplayMtEph::intersection(self, otheintersection226,7645
        fn difference(&self, other: &Self) -> Self { BSTSetSplayMtEph::difference(self, other) }difference228,7747
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetSplayMtEph::split(self, pivot) }split230,7845
        fn join_pair(left: Self, right: Self) -> Self { BSTSetSplayMtEph::join_pair(left, right)join_pair232,7941
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetSplayMtEph::join_m(left, pijoin_m234,8041
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetSplayMtEph::filter(filter236,8152
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetSplayMtEph::reduce(sereduce238,8268
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8381
        fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree242,8460
    macro_rules! BSTSetSplayMtEphLit {BSTSetSplayMtEphLit246,8549
    fn _BSTSetSplayMtEphLit_type_checks() {_BSTSetSplayMtEphLit_type_checks258,9122

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaMtEph.rs,4630
pub mod BSTBBAlphaMtEph {BSTBBAlphaMtEph3,108
    type Link<T> = Option<Box<Node<T>>>;Link12,344
    struct Node<T: StTInMtT + Ord> {Node15,407
        key: T,key16,444
        size: N,size17,460
        left: Link<T>,left18,477
        right: Link<T>,right19,500
    impl<T: StTInMtT + Ord> Node<T> {Node22,531
        fn new(key: T) -> Self {new23,569
    pub struct BSTBBAlphaMtEph<T: StTInMtT + Ord> {BSTBBAlphaMtEph34,783
        root: Arc<RwLock<Link<T>>>,root35,835
    pub type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;BSTreeBBAlpha38,878
    pub trait BSTBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTBBAlphaMtEphTrait40,931
        fn new() -> Self;new41,994
        fn insert(&self, value: T);insert42,1020
        fn find(&self, target: &T) -> Option<T>;find43,1056
        fn contains(&self, target: &T) -> B;contains44,1105
        fn size(&self) -> N;size45,1150
        fn is_empty(&self) -> B;is_empty46,1179
        fn height(&self) -> N;height47,1212
        fn minimum(&self) -> Option<T>;minimum48,1243
        fn maximum(&self) -> Option<T>;maximum49,1283
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order50,1323
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order51,1372
    impl<T: StTInMtT + Ord> Default for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph54,1429
        fn default() -> Self { Self::new() }default55,1490
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph58,1542
        pub fn new() -> Self {new59,1591
        pub fn size(&self) -> N {size65,1728
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty70,1861
        pub fn height(&self) -> N {height72,1952
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec73,1988
        pub fn insert(&self, value: T) {insert84,2338
        pub fn find(&self, target: &T) -> Option<T> {find93,2669
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains98,2839
        pub fn minimum(&self) -> Option<T> {minimum100,2953
        pub fn maximum(&self) -> Option<T> {maximum105,3105
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order110,3257
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order117,3542
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link124,3829
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate126,3912
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link128,4031
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild150,4777
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed157,5059
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values167,5469
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced175,5750
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link187,6190
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link202,6707
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link212,7028
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect222,7351
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect230,7638
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph239,7934
        fn new() -> Self { BSTBBAlphaMtEph::new() }new240,8011
        fn insert(&self, value: T) { BSTBBAlphaMtEph::insert(self, value) }insert242,8064
        fn find(&self, target: &T) -> Option<T> { BSTBBAlphaMtEph::find(self, target) }find244,8141
        fn contains(&self, target: &T) -> B { BSTBBAlphaMtEph::contains(self, target) }contains246,8230
        fn size(&self) -> N { BSTBBAlphaMtEph::size(self) }size248,8319
        fn is_empty(&self) -> B { BSTBBAlphaMtEph::is_empty(self) }is_empty250,8380
        fn height(&self) -> N { BSTBBAlphaMtEph::height(self) }height252,8449
        fn minimum(&self) -> Option<T> { BSTBBAlphaMtEph::minimum(self) }minimum254,8514
        fn maximum(&self) -> Option<T> { BSTBBAlphaMtEph::maximum(self) }maximum256,8589
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaMtEph::in_order(self) }in_order258,8664
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaMtEph::pre_order(self) }pre_order260,8749
    macro_rules! BSTBBAlphaMtEphLit {BSTBBAlphaMtEphLit264,8862
    fn _BSTBBAlphaMtEphLit_type_checks() {_BSTBBAlphaMtEphLit_type_checks276,9417

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLStEph.rs,4549
pub mod BSTAVLStEph {BSTAVLStEph3,97
    type Link<T> = Option<Box<Node<T>>>;Link8,264
    struct Node<T: StT + Ord> {Node11,327
        key: T,key12,359
        height: i32,height13,375
        size: N,size14,396
        left: Link<T>,left15,413
        right: Link<T>,right16,436
    impl<T: StT + Ord> Node<T> {Node19,467
        fn new(key: T) -> Self {new20,500
    pub struct BSTAVLStEph<T: StT + Ord> {BSTAVLStEph32,741
        root: Link<T>,root33,784
    pub type BSTreeAVL<T> = BSTAVLStEph<T>;BSTreeAVL36,814
    pub trait BSTAVLStEphTrait<T: StT + Ord> {BSTAVLStEphTrait38,859
        fn new() -> Self;new39,906
        fn size(&self) -> N;size40,932
        fn is_empty(&self) -> B;is_empty41,961
        fn height(&self) -> N;height42,994
        fn insert(&mut self, value: T);insert43,1025
        fn find(&self, target: &T) -> Option<&T>;find44,1065
        fn contains(&self, target: &T) -> B;contains45,1115
        fn minimum(&self) -> Option<&T>;minimum46,1160
        fn maximum(&self) -> Option<&T>;maximum47,1201
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order48,1242
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order49,1291
    impl<T: StT + Ord> Default for BSTAVLStEph<T> {BSTAVLStEph52,1348
        fn default() -> Self { Self::new() }default53,1400
    impl<T: StT + Ord> BSTAVLStEph<T> {BSTAVLStEph56,1452
        pub fn new() -> Self { BSTAVLStEph { root: None } }new57,1492
        pub fn size(&self) -> N { Self::size_link(&self.root) }size59,1553
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty61,1618
        pub fn height(&self) -> N { Self::height_link(&self.root) as N }height63,1709
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert65,1783
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find67,1873
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains69,1967
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum71,2081
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum73,2157
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order75,2233
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order81,2457
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link87,2683
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link89,2772
        fn update(node: &mut Node<T>) {update91,2855
        fn rotate_right(link: &mut Link<T>) {rotate_right96,3091
        fn rotate_left(link: &mut Link<T>) {rotate_left110,3547
        fn rebalance(link: &mut Link<T>) {rebalance124,4002
        fn insert_link(link: &mut Link<T>, value: T) {insert_link149,5048
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link168,5683
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link183,6200
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link193,6521
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect203,6844
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect211,7131
    impl<T: StT + Ord> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {BSTAVLStEph220,7427
        fn new() -> Self { BSTAVLStEph::new() }new221,7491
        fn size(&self) -> N { BSTAVLStEph::size(self) }size223,7540
        fn is_empty(&self) -> B { BSTAVLStEph::is_empty(self) }is_empty225,7597
        fn height(&self) -> N { BSTAVLStEph::height(self) }height227,7662
        fn insert(&mut self, value: T) { BSTAVLStEph::insert(self, value) }insert229,7723
        fn find(&self, target: &T) -> Option<&T> { BSTAVLStEph::find(self, target) }find231,7800
        fn contains(&self, target: &T) -> B { BSTAVLStEph::contains(self, target) }contains233,7886
        fn minimum(&self) -> Option<&T> { BSTAVLStEph::minimum(self) }minimum235,7971
        fn maximum(&self) -> Option<&T> { BSTAVLStEph::maximum(self) }maximum237,8043
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::in_order(self) }in_order239,8115
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::pre_order(self) }pre_order241,8196
    macro_rules! BSTAVLStEphLit {BSTAVLStEphLit245,8305
    fn _BSTAVLStEphLit_type_checks() {_BSTAVLStEphLit_type_checks257,8812

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainMtEph.rs,3810
pub mod BSTPlainMtEph {BSTPlainMtEph3,90
    type Link<T> = Arc<RwLock<Option<Node<T>>>>;Link9,238
    struct Node<T: StTInMtT + Ord> {Node12,309
        key: T,key13,346
        height: i32,height14,362
        size: N,size15,383
        left: Link<T>,left16,400
        right: Link<T>,right17,423
    impl<T: StTInMtT + Ord> Node<T> {Node20,454
        fn new(key: T) -> Self {new21,492
        fn update(&mut self) {update31,745
    pub struct BSTPlainMtEph<T: StTInMtT + Ord> {BSTPlainMtEph40,1049
        root: Link<T>,root41,1099
    pub type BSTree<T> = BSTPlainMtEph<T>;BSTree44,1129
    pub trait BSTPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTPlainMtEphTrait46,1173
        fn new() -> Self;new47,1234
        fn insert(&self, value: T);insert48,1260
        fn find(&self, target: &T) -> Option<T>;find49,1296
        fn contains(&self, target: &T) -> B;contains50,1345
        fn size(&self) -> N;size51,1390
        fn is_empty(&self) -> B;is_empty52,1419
        fn height(&self) -> N;height53,1452
        fn minimum(&self) -> Option<T>;minimum54,1483
        fn maximum(&self) -> Option<T>;maximum55,1523
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order56,1563
    impl<T: StTInMtT + Ord> BSTPlainMtEph<T> {BSTPlainMtEph59,1619
        pub fn new() -> Self {new60,1666
        pub fn insert(&self, value: T) {insert66,1792
            fn descend<T: StTInMtT + Ord>(link: &Link<T>, value: T) -> bool {descend67,1833
        pub fn find(&self, target: &T) -> Option<T> {find101,3025
            fn find_rec<T: StTInMtT + Ord>(link: &Link<T>, target: &T) -> Option<T> {find_rec102,3079
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains121,3818
        pub fn size(&self) -> N {size122,3931
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty127,4082
        pub fn height(&self) -> N {height129,4173
        pub fn minimum(&self) -> Option<T> {minimum134,4333
            fn leftmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {leftmost135,4378
        pub fn maximum(&self) -> Option<T> {maximum156,5088
            fn rightmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {rightmost157,5133
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order178,5850
            fn traverse<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {traverse179,5904
    fn height_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> i32 { link.as_ref().map_or(0, |n|height_of198,6572
    fn size_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> N { link.as_ref().map_or(0, |n| n.ssize_of200,6682
    impl<T: StTInMtT + Ord> BSTPlainMtEphTrait<T> for BSTPlainMtEph<T> {BSTPlainMtEph202,6786
        fn new() -> Self { BSTPlainMtEph::new() }new203,6859
        fn insert(&self, value: T) { BSTPlainMtEph::insert(self, value) }insert204,6909
        fn find(&self, target: &T) -> Option<T> { BSTPlainMtEph::find(self, target) }find205,6983
        fn contains(&self, target: &T) -> B { BSTPlainMtEph::contains(self, target) }contains206,7069
        fn size(&self) -> N { BSTPlainMtEph::size(self) }size207,7155
        fn is_empty(&self) -> B { BSTPlainMtEph::is_empty(self) }is_empty208,7213
        fn height(&self) -> N { BSTPlainMtEph::height(self) }height209,7279
        fn minimum(&self) -> Option<T> { BSTPlainMtEph::minimum(self) }minimum210,7341
        fn maximum(&self) -> Option<T> { BSTPlainMtEph::maximum(self) }maximum211,7413
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTPlainMtEph::in_order(self) }in_order212,7485
    macro_rules! BSTPlainMtEphLit {BSTPlainMtEphLit216,7594
    fn _BSTPlainMtEphLit_type_checks() {_BSTPlainMtEphLit_type_checks231,8158

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaStEph.rs,4742
pub mod BSTBBAlphaStEph {BSTBBAlphaStEph3,80
    type Link<T> = Option<Box<Node<T>>>;Link10,281
    struct Node<T: StT + Ord> {Node13,351
        key: T,key14,383
        size: N,size15,399
        left: Link<T>,left16,416
        right: Link<T>,right17,439
    impl<T: StT + Ord> Node<T> {Node20,470
        fn new(key: T) -> Self {new21,503
    pub struct BSTBBAlphaStEph<T: StT + Ord> {BSTBBAlphaStEph32,717
        root: Link<T>,root33,764
    pub type BSTreeBBAlpha<T> = BSTBBAlphaStEph<T>;BSTreeBBAlpha36,794
    pub trait BSTBBAlphaStEphTrait<T: StT + Ord> {BSTBBAlphaStEphTrait38,847
        fn new() -> Self;new39,898
        fn size(&self) -> N;size40,924
        fn is_empty(&self) -> B;is_empty41,953
        fn height(&self) -> N;height42,986
        fn insert(&mut self, value: T);insert43,1017
        fn find(&self, target: &T) -> Option<&T>;find44,1057
        fn contains(&self, target: &T) -> B;contains45,1107
        fn minimum(&self) -> Option<&T>;minimum46,1152
        fn maximum(&self) -> Option<&T>;maximum47,1193
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order48,1234
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order49,1283
    impl<T: StT + Ord> Default for BSTBBAlphaStEph<T> {BSTBBAlphaStEph52,1340
        fn default() -> Self { Self::new() }default53,1396
    impl<T: StT + Ord> BSTBBAlphaStEph<T> {BSTBBAlphaStEph56,1448
        pub fn new() -> Self { BSTBBAlphaStEph { root: None } }new57,1492
        pub fn size(&self) -> N { Self::size_link(&self.root) }size59,1557
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty61,1622
        pub fn height(&self) -> N {height63,1713
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec64,1749
        pub fn insert(&mut self, value: T) {insert73,2045
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find81,2333
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains83,2427
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum85,2541
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum87,2617
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order89,2693
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order95,2917
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link101,3143
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate103,3226
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link105,3345
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild127,4091
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed134,4373
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values144,4783
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced152,5064
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link164,5504
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link179,6021
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link189,6342
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect199,6665
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect207,6952
    impl<T: StT + Ord> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {BSTBBAlphaStEph216,7248
        fn new() -> Self { BSTBBAlphaStEph::new() }new217,7320
        fn size(&self) -> N { BSTBBAlphaStEph::size(self) }size219,7373
        fn is_empty(&self) -> B { BSTBBAlphaStEph::is_empty(self) }is_empty221,7434
        fn height(&self) -> N { BSTBBAlphaStEph::height(self) }height223,7503
        fn insert(&mut self, value: T) { BSTBBAlphaStEph::insert(self, value) }insert225,7568
        fn find(&self, target: &T) -> Option<&T> { BSTBBAlphaStEph::find(self, target) }find227,7649
        fn contains(&self, target: &T) -> B { BSTBBAlphaStEph::contains(self, target) }contains229,7739
        fn minimum(&self) -> Option<&T> { BSTBBAlphaStEph::minimum(self) }minimum231,7828
        fn maximum(&self) -> Option<&T> { BSTBBAlphaStEph::maximum(self) }maximum233,7904
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaStEph::in_order(self) }in_order235,7980
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaStEph::pre_order(self) }pre_order237,8065
    macro_rules! BSTBBAlphaStEphLit {BSTBBAlphaStEphLit241,8178
    fn _BSTBBAlphaStEphLit_type_checks() {_BSTBBAlphaStEphLit_type_checks253,8737

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayMtEph.rs,4230
pub mod BSTSplayMtEph {BSTSplayMtEph3,99
    type Link<T> = Option<Box<Node<T>>>;Link10,303
    struct Node<T: StTInMtT + Ord> {Node13,373
        key: T,key14,410
        size: N,size15,426
        left: Link<T>,left16,443
        right: Link<T>,right17,466
    impl<T: StTInMtT + Ord> Node<T> {Node20,497
        fn new(key: T) -> Self {new21,535
    pub struct BSTSplayMtEph<T: StTInMtT + Ord> {BSTSplayMtEph32,749
        root: Arc<RwLock<Link<T>>>,root33,799
    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;BSTreeSplay36,842
    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSplayMtEphTrait38,891
        fn new() -> Self;new39,952
        fn insert(&self, value: T);insert40,978
        fn find(&self, target: &T) -> Option<T>;find41,1014
        fn contains(&self, target: &T) -> B;contains42,1063
        fn size(&self) -> N;size43,1108
        fn is_empty(&self) -> B;is_empty44,1137
        fn height(&self) -> N;height45,1170
        fn minimum(&self) -> Option<T>;minimum46,1201
        fn maximum(&self) -> Option<T>;maximum47,1241
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order48,1281
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order49,1330
    impl<T: StTInMtT + Ord> Default for BSTSplayMtEph<T> {BSTSplayMtEph52,1387
        fn default() -> Self { Self::new() }default53,1446
    impl<T: StTInMtT + Ord> BSTSplayMtEph<T> {BSTSplayMtEph56,1498
        pub fn new() -> Self {new57,1545
        pub fn size(&self) -> N {size63,1680
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty68,1813
        pub fn height(&self) -> N {height70,1904
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec71,1940
        pub fn insert(&self, value: T) {insert82,2290
        pub fn find(&self, target: &T) -> Option<T> {find87,2449
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains92,2619
        pub fn minimum(&self) -> Option<T> {minimum94,2733
        pub fn maximum(&self) -> Option<T> {maximum99,2885
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order104,3037
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order111,3322
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link118,3609
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate120,3692
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link122,3811
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link144,4557
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link159,5074
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link169,5395
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect179,5718
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect187,6005
    impl<T: StTInMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {BSTSplayMtEph196,6301
        fn new() -> Self { BSTSplayMtEph::new() }new197,6374
        fn insert(&self, value: T) { BSTSplayMtEph::insert(self, value) }insert199,6425
        fn find(&self, target: &T) -> Option<T> { BSTSplayMtEph::find(self, target) }find201,6500
        fn contains(&self, target: &T) -> B { BSTSplayMtEph::contains(self, target) }contains203,6587
        fn size(&self) -> N { BSTSplayMtEph::size(self) }size205,6674
        fn is_empty(&self) -> B { BSTSplayMtEph::is_empty(self) }is_empty207,6733
        fn height(&self) -> N { BSTSplayMtEph::height(self) }height209,6800
        fn minimum(&self) -> Option<T> { BSTSplayMtEph::minimum(self) }minimum211,6863
        fn maximum(&self) -> Option<T> { BSTSplayMtEph::maximum(self) }maximum213,6936
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTSplayMtEph::in_order(self) }in_order215,7009
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTSplayMtEph::pre_order(self) }pre_order217,7092
    macro_rules! BSTSplayMtEphLit {BSTSplayMtEphLit221,7203
    fn _BSTSplayMtEphLit_type_checks() {_BSTSplayMtEphLit_type_checks233,7732

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetRBMtEph.rs,5562
pub mod BSTSetRBMtEph {BSTSetRBMtEph3,79
    pub struct BSTSetRBMtEph<T: StTInMtT + Ord> {BSTSetRBMtEph11,316
        tree: BSTRBMtEph<T>,tree12,366
    pub type BSTSetRBMt<T> = BSTSetRBMtEph<T>;BSTSetRBMt15,402
    pub trait BSTSetRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetRBMtEphTrait17,450
        fn empty() -> Self;empty18,511
        fn singleton(value: T) -> Self;singleton19,539
        fn size(&self) -> N;size20,579
        fn is_empty(&self) -> B;is_empty21,608
        fn find(&self, value: &T) -> Option<T>;find22,641
        fn contains(&self, value: &T) -> B;contains23,689
        fn minimum(&self) -> Option<T>;minimum24,733
        fn maximum(&self) -> Option<T>;maximum25,773
        fn insert(&mut self, value: T);insert26,813
        fn delete(&mut self, target: &T);delete27,853
        fn union(&self, other: &Self) -> Self;union28,895
        fn intersection(&self, other: &Self) -> Self;intersection29,942
        fn difference(&self, other: &Self) -> Self;difference30,996
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1048
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1103
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1158
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1220
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1290
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1358
        fn as_tree(&self) -> &BSTRBMtEph<T>;as_tree37,1412
    impl<T: StTInMtT + Ord> BSTSetRBMtEph<T> {BSTSetRBMtEph40,1464
        pub fn empty() -> Self {empty41,1511
        pub fn singleton(value: T) -> Self {singleton47,1629
        pub fn size(&self) -> N { self.tree.size() }size53,1785
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1839
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1901
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,1979
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2057
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2125
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2193
        pub fn delete(&mut self, target: &T) {delete67,2266
        pub fn union(&self, other: &Self) -> Self {union75,2555
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2854
        pub fn difference(&self, other: &Self) -> Self {difference100,3432
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4009
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4701
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5014
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5370
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5779
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6043
        pub fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree179,6126
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6190
        fn rebuild_from_vec(values: Vec<T>) -> BSTRBMtEph<T> {rebuild_from_vec183,6281
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6498
    impl<T: StTInMtT + Ord> BSTSetRBMtEphTrait<T> for BSTSetRBMtEph<T> {BSTSetRBMtEph203,6780
        fn empty() -> Self { Self::empty() }empty204,6853
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6899
        fn size(&self) -> N { self.tree.size() }size208,6966
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7016
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7074
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7148
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7222
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7286
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7350
        fn delete(&mut self, target: &T) { BSTSetRBMtEph::delete(self, target) }delete222,7419
        fn union(&self, other: &Self) -> Self { BSTSetRBMtEph::union(self, other) }union224,7501
        fn intersection(&self, other: &Self) -> Self { BSTSetRBMtEph::intersection(self, other) intersection226,7586
        fn difference(&self, other: &Self) -> Self { BSTSetRBMtEph::difference(self, other) }difference228,7685
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetRBMtEph::split(self, pivot) }split230,7780
        fn join_pair(left: Self, right: Self) -> Self { BSTSetRBMtEph::join_pair(left, right) }join_pair232,7873
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetRBMtEph::join_m(left, pivotjoin_m234,7970
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetRBMtEph::filter(selfilter236,8078
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetRBMtEph::reduce(self,reduce238,8191
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8301
        fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree242,8380
    macro_rules! BSTSetRBMtEphLit {BSTSetRBMtEphLit246,8466
    fn _BSTSetRBMtEphLit_type_checks() {_BSTSetRBMtEphLit_type_checks258,9000

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStEph.rs,4720
pub mod AVLTreeSeqStEph {AVLTreeSeqStEph3,61
    type Link<T> = Option<Box<AVLTreeNode<T>>>;Link9,202
    pub struct AVLTreeNode<T: StT> {AVLTreeNode11,251
        pub value: T,value12,288
        pub height: N,height13,310
        pub left_size: N,left_size14,333
        pub right_size: N,right_size15,359
        pub left: Link<T>,left16,386
        pub right: Link<T>,right17,413
        pub index: N,index18,441
    impl<T: StT> AVLTreeNode<T> {AVLTreeNode21,470
        fn new(value: T, index: N) -> Self {new22,504
    pub struct AVLTreeSeqStEphS<T: StT> {AVLTreeSeqStEphS35,797
        pub root: Link<T>,root36,839
        pub next_key: N,next_key37,866
    pub trait AVLTreeSeqStEphTrait<T: StT> {AVLTreeSeqStEphTrait40,898
        fn empty() -> Self;empty42,985
        fn new() -> Self;new44,1055
        fn length(&self) -> N;length46,1123
        fn nth(&self, index: N) -> &T;nth48,1204
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set50,1293
        fn singleton(item: T) -> Self;singleton52,1416
        fn isEmpty(&self) -> B;isEmpty54,1497
        fn isSingleton(&self) -> B;isSingleton56,1571
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy58,1669
    impl<T: StT> AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS61,1736
        pub fn new_root() -> Self {new_root62,1775
        pub fn new() -> Self { Self::new_root() }new68,1923
        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqStEphS<T> {update69,1973
        pub fn from_vec(values: Vec<T>) -> AVLTreeSeqStEphS<T> {from_vec73,2181
        pub fn to_arrayseq(&self) -> ArraySeqStEphS<T> {to_arrayseq82,2564
        pub fn iter<'a>(&'a self) -> AVLTreeSeqIterStEph<'a, T> { AVLTreeSeqIterStEph::new(&selfiter98,3218
        pub fn push_back(&mut self, value: T) {push_back99,3323
        pub fn contains_value(&self, target: &T) -> B {contains_value104,3537
        pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value112,3760
        pub fn delete_value(&mut self, target: &T) -> bool {delete_value113,3836
    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS138,4654
        fn empty() -> Self { AVLTreeSeqStEphS::new_root() }empty139,4721
        fn new() -> Self { AVLTreeSeqStEphS::new_root() }new141,4782
        fn length(&self) -> N { size_link(&self.root) }length143,4841
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth145,4898
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set147,4969
        fn singleton(item: T) -> Self {singleton152,5135
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty158,5332
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton160,5420
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy162,5512
    pub struct AVLTreeSeqIterStEph<'a, T: StT> {AVLTreeSeqIterStEph177,6033
        stack: Vec<&'a AVLTreeNode<T>>,stack178,6082
        current: Option<&'a AVLTreeNode<T>>,current179,6122
    impl<'a, T: StT> AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph182,6174
        fn new(root: &'a Link<T>) -> Self {new183,6224
        fn push_left(&mut self, link: &'a Link<T>) {push_left191,6453
    impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph200,6705
        type Item = &'a T;Item201,6768
        fn next(&mut self) -> Option<Self::Item> {next202,6795
    fn h<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h210,7019
    fn size_link<T: StT>(n: &Link<T>) -> N {size_link212,7094
    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>) {update_meta220,7259
    fn rotate_right<T: StT>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right228,7504
    fn rotate_left<T: StT>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left239,7867
    fn rebalance<T: StT>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance250,8228
    pub(crate) fn insert_at_link<T: StT>(node: Link<T>, index: N, value: T, next_key: &mut N) ->insert_at_link271,8993
    fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> &'a T {nth_link291,9789
    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str> {set_link303,10169
    macro_rules! AVLTreeSeqStEph {AVLTreeSeqStEph321,10779
    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS335,11340
        fn eq(&self, other: &Self) -> bool {eq336,11393
    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}AVLTreeSeqStEphS349,11720

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBMtEph.rs,4631
pub mod BSTRBMtEph {BSTRBMtEph3,102
    enum Color {Color11,345
        Red,Red12,362
        Black,Black13,375
    type Link<T> = Option<Box<Node<T>>>;Link16,397
    struct Node<T: StTInMtT + Ord> {Node19,460
        key: T,key20,497
        color: Color,color21,513
        size: N,size22,535
        left: Link<T>,left23,552
        right: Link<T>,right24,575
    impl<T: StTInMtT + Ord> Node<T> {Node27,606
        fn new(key: T) -> Self {new28,644
    pub struct BSTRBMtEph<T: StTInMtT + Ord> {BSTRBMtEph40,893
        root: Arc<RwLock<Link<T>>>,root41,940
    pub type BSTreeRB<T> = BSTRBMtEph<T>;BSTreeRB44,983
    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTRBMtEphTrait46,1026
        fn new() -> Self;new47,1084
        fn insert(&self, value: T);insert48,1110
        fn find(&self, target: &T) -> Option<T>;find49,1146
        fn contains(&self, target: &T) -> B;contains50,1195
        fn size(&self) -> N;size51,1240
        fn is_empty(&self) -> B;is_empty52,1269
        fn height(&self) -> N;height53,1302
        fn minimum(&self) -> Option<T>;minimum54,1333
        fn maximum(&self) -> Option<T>;maximum55,1373
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order56,1413
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order57,1462
    impl<T: StTInMtT + Ord> Default for BSTRBMtEph<T> {BSTRBMtEph60,1519
        fn default() -> Self { Self::new() }default61,1575
    impl<T: StTInMtT + Ord> BSTRBMtEph<T> {BSTRBMtEph64,1627
        pub fn new() -> Self {new65,1671
        pub fn size(&self) -> N {size71,1803
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty76,1936
        pub fn height(&self) -> N {height78,2027
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec79,2063
        pub fn insert(&self, value: T) {insert90,2413
        pub fn find(&self, target: &T) -> Option<T> {find98,2678
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains103,2848
        pub fn minimum(&self) -> Option<T> {minimum105,2962
        pub fn maximum(&self) -> Option<T> {maximum110,3114
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order115,3266
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order122,3551
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red129,3838
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link131,3940
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate133,4023
        fn rotate_left(link: &mut Link<T>) {rotate_left135,4142
        fn rotate_right(link: &mut Link<T>) {rotate_right154,4802
        fn flip_colors(link: &mut Link<T>) {flip_colors173,5466
        fn fix_up(link: &mut Link<T>) {fix_up194,6271
        fn insert_link(link: &mut Link<T>, value: T) {insert_link230,7425
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link246,7967
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link261,8484
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link271,8805
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect281,9128
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect289,9415
    impl<T: StTInMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {BSTRBMtEph298,9711
        fn new() -> Self { BSTRBMtEph::new() }new299,9778
        fn insert(&self, value: T) { BSTRBMtEph::insert(self, value) }insert301,9826
        fn find(&self, target: &T) -> Option<T> { BSTRBMtEph::find(self, target) }find303,9898
        fn contains(&self, target: &T) -> B { BSTRBMtEph::contains(self, target) }contains305,9982
        fn size(&self) -> N { BSTRBMtEph::size(self) }size307,10066
        fn is_empty(&self) -> B { BSTRBMtEph::is_empty(self) }is_empty309,10122
        fn height(&self) -> N { BSTRBMtEph::height(self) }height311,10186
        fn minimum(&self) -> Option<T> { BSTRBMtEph::minimum(self) }minimum313,10246
        fn maximum(&self) -> Option<T> { BSTRBMtEph::maximum(self) }maximum315,10316
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTRBMtEph::in_order(self) }in_order317,10386
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTRBMtEph::pre_order(self) }pre_order319,10466
    macro_rules! BSTRBMtEphLit {BSTRBMtEphLit323,10574
    fn _BSTRBMtEphLit_type_checks() {_BSTRBMtEphLit_type_checks335,11064

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayStEph.rs,4383
pub mod BSTSplayStEph {BSTSplayStEph3,84
    type Link<T> = Option<Box<Node<T>>>;Link8,253
    struct Node<T: StT + Ord> {Node11,316
        key: T,key12,348
        size: N,size13,364
        left: Link<T>,left14,381
        right: Link<T>,right15,404
    impl<T: StT + Ord> Node<T> {Node18,435
        fn new(key: T) -> Self {new19,468
    pub struct BSTSplayStEph<T: StT + Ord> {BSTSplayStEph30,682
        root: Link<T>,root31,727
    pub type BSTreeSplay<T> = BSTSplayStEph<T>;BSTreeSplay34,757
    pub trait BSTSplayStEphTrait<T: StT + Ord> {BSTSplayStEphTrait36,806
        fn new() -> Self;new37,855
        fn size(&self) -> N;size38,881
        fn is_empty(&self) -> B;is_empty39,910
        fn height(&self) -> N;height40,943
        fn insert(&mut self, value: T);insert41,974
        fn find(&self, target: &T) -> Option<&T>;find42,1014
        fn contains(&self, target: &T) -> B;contains43,1064
        fn minimum(&self) -> Option<&T>;minimum44,1109
        fn maximum(&self) -> Option<&T>;maximum45,1150
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order46,1191
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order47,1240
    impl<T: StT + Ord> Default for BSTSplayStEph<T> {BSTSplayStEph50,1297
        fn default() -> Self { Self::new() }default51,1351
    impl<T: StT + Ord> BSTSplayStEph<T> {BSTSplayStEph54,1403
        pub fn new() -> Self { BSTSplayStEph { root: None } }new55,1445
        pub fn size(&self) -> N { Self::size_link(&self.root) }size57,1508
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty59,1573
        pub fn height(&self) -> N {height61,1664
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec62,1700
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert71,1996
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find73,2086
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains75,2180
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum77,2294
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum79,2370
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order81,2446
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order87,2670
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link93,2896
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate95,2979
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link97,3098
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link119,3844
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link134,4361
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link144,4682
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect154,5005
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect162,5292
    impl<T: StT + Ord> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {BSTSplayStEph171,5588
        fn new() -> Self { BSTSplayStEph::new() }new172,5656
        fn size(&self) -> N { BSTSplayStEph::size(self) }size174,5707
        fn is_empty(&self) -> B { BSTSplayStEph::is_empty(self) }is_empty176,5766
        fn height(&self) -> N { BSTSplayStEph::height(self) }height178,5833
        fn insert(&mut self, value: T) { BSTSplayStEph::insert(self, value) }insert180,5896
        fn find(&self, target: &T) -> Option<&T> { BSTSplayStEph::find(self, target) }find182,5975
        fn contains(&self, target: &T) -> B { BSTSplayStEph::contains(self, target) }contains184,6063
        fn minimum(&self) -> Option<&T> { BSTSplayStEph::minimum(self) }minimum186,6150
        fn maximum(&self) -> Option<&T> { BSTSplayStEph::maximum(self) }maximum188,6224
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTSplayStEph::in_order(self) }in_order190,6298
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTSplayStEph::pre_order(self) }pre_order192,6381
    macro_rules! BSTSplayStEphLit {BSTSplayStEphLit196,6492
    fn _BSTSplayStEphLit_type_checks() {_BSTSplayStEphLit_type_checks208,7025

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLMtEph.rs,4356
pub mod BSTAVLMtEph {BSTAVLMtEph3,96
    type Link<T> = Option<Box<Node<T>>>;Link10,298
    struct Node<T: StTInMtT + Ord> {Node13,368
        key: T,key14,405
        height: i32,height15,421
        size: N,size16,442
        left: Link<T>,left17,459
        right: Link<T>,right18,482
    impl<T: StTInMtT + Ord> Node<T> {Node21,513
        fn new(key: T) -> Self {new22,551
    pub struct BSTAVLMtEph<T: StTInMtT + Ord> {BSTAVLMtEph34,792
        root: Arc<RwLock<Link<T>>>,root35,840
    pub type BSTreeAVL<T> = BSTAVLMtEph<T>;BSTreeAVL38,883
    pub trait BSTAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTAVLMtEphTrait40,928
        fn new() -> Self;new41,987
        fn insert(&self, value: T);insert42,1013
        fn find(&self, target: &T) -> Option<T>;find43,1049
        fn contains(&self, target: &T) -> B;contains44,1098
        fn size(&self) -> N;size45,1143
        fn is_empty(&self) -> B;is_empty46,1172
        fn height(&self) -> N;height47,1205
        fn minimum(&self) -> Option<T>;minimum48,1236
        fn maximum(&self) -> Option<T>;maximum49,1276
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order50,1316
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order51,1365
    impl<T: StTInMtT + Ord> Default for BSTAVLMtEph<T> {BSTAVLMtEph54,1422
        fn default() -> Self { Self::new() }default55,1479
    impl<T: StTInMtT + Ord> BSTAVLMtEph<T> {BSTAVLMtEph58,1531
        pub fn new() -> Self {new59,1576
        pub fn size(&self) -> N {size65,1709
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty70,1842
        pub fn height(&self) -> N {height72,1933
        pub fn insert(&self, value: T) {insert77,2075
        pub fn find(&self, target: &T) -> Option<T> {find82,2234
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains87,2404
        pub fn minimum(&self) -> Option<T> {minimum89,2518
        pub fn maximum(&self) -> Option<T> {maximum94,2670
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order99,2822
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order106,3107
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link113,3394
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link115,3483
        fn update(node: &mut Node<T>) {update117,3566
        fn rotate_right(link: &mut Link<T>) {rotate_right122,3802
        fn rotate_left(link: &mut Link<T>) {rotate_left136,4258
        fn rebalance(link: &mut Link<T>) {rebalance150,4713
        fn insert_link(link: &mut Link<T>, value: T) {insert_link175,5759
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link194,6394
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link209,6911
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link219,7233
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect229,7557
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect237,7844
    impl<T: StTInMtT + Ord> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {BSTAVLMtEph246,8140
        fn new() -> Self { BSTAVLMtEph::new() }new247,8209
        fn insert(&self, value: T) { BSTAVLMtEph::insert(self, value) }insert249,8258
        fn find(&self, target: &T) -> Option<T> { BSTAVLMtEph::find(self, target) }find251,8331
        fn contains(&self, target: &T) -> B { BSTAVLMtEph::contains(self, target) }contains253,8416
        fn size(&self) -> N { BSTAVLMtEph::size(self) }size255,8501
        fn is_empty(&self) -> B { BSTAVLMtEph::is_empty(self) }is_empty257,8558
        fn height(&self) -> N { BSTAVLMtEph::height(self) }height259,8623
        fn minimum(&self) -> Option<T> { BSTAVLMtEph::minimum(self) }minimum261,8684
        fn maximum(&self) -> Option<T> { BSTAVLMtEph::maximum(self) }maximum263,8755
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTAVLMtEph::in_order(self) }in_order265,8826
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTAVLMtEph::pre_order(self) }pre_order267,8907
    macro_rules! BSTAVLMtEphLit {BSTAVLMtEphLit271,9016
    fn _BSTAVLMtEphLit_type_checks() {_BSTAVLMtEphLit_type_checks283,9519

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainStEph.rs,3570
pub mod BSTPlainStEph {BSTPlainStEph3,64
    pub struct BSTPlainStEph<T: StT + Ord> {BSTPlainStEph9,252
        root: BBTree<T>,root10,297
    pub type BSTree<T> = BSTPlainStEph<T>;BSTree13,329
    pub trait BSTPlainStEphTrait<T: StT + Ord> {BSTPlainStEphTrait15,373
        fn new() -> Self;new16,422
        fn size(&self) -> N;size17,448
        fn is_empty(&self) -> B;is_empty18,477
        fn height(&self) -> N;height19,510
        fn insert(&mut self, value: T);insert20,541
        fn find(&self, target: &T) -> Option<&T>;find21,581
        fn contains(&self, target: &T) -> B;contains22,631
        fn minimum(&self) -> Option<&T>;minimum23,676
        fn maximum(&self) -> Option<&T>;maximum24,717
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order25,758
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order26,807
    impl<T: StT + Ord> BSTPlainStEph<T> {BSTPlainStEph29,864
        pub fn new() -> Self { BSTPlainStEph { root: BBTree::leaf() } }new30,906
        pub fn size(&self) -> N { self.root.size() }size32,979
        pub fn is_empty(&self) -> B { self.root.is_leaf() }is_empty34,1033
        pub fn height(&self) -> N { self.root.height() }height36,1094
        pub fn insert(&mut self, value: T) { insert_node(&mut self.root, value); }insert38,1152
        pub fn find(&self, target: &T) -> Option<&T> { find_node(&self.root, target) }find40,1236
        pub fn contains(&self, target: &T) -> B { contains_node(&self.root, target) }contains42,1324
        pub fn minimum(&self) -> Option<&T> { min_node(&self.root) }minimum44,1411
        pub fn maximum(&self) -> Option<&T> { max_node(&self.root) }maximum46,1481
        pub fn in_order(&self) -> ArraySeqStPerS<T> { self.root.in_order() }in_order48,1551
        pub fn pre_order(&self) -> ArraySeqStPerS<T> { self.root.pre_order() }pre_order50,1629
    impl<T: StT + Ord> BSTPlainStEphTrait<T> for BSTPlainStEph<T> {BSTPlainStEph53,1715
        fn new() -> Self { BSTPlainStEph::new() }new54,1783
        fn size(&self) -> N { BSTPlainStEph::size(self) }size56,1834
        fn is_empty(&self) -> B { BSTPlainStEph::is_empty(self) }is_empty58,1893
        fn height(&self) -> N { BSTPlainStEph::height(self) }height60,1960
        fn insert(&mut self, value: T) { BSTPlainStEph::insert(self, value) }insert62,2023
        fn find(&self, target: &T) -> Option<&T> { BSTPlainStEph::find(self, target) }find64,2102
        fn contains(&self, target: &T) -> B { BSTPlainStEph::contains(self, target) }contains66,2190
        fn minimum(&self) -> Option<&T> { BSTPlainStEph::minimum(self) }minimum68,2277
        fn maximum(&self) -> Option<&T> { BSTPlainStEph::maximum(self) }maximum70,2351
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTPlainStEph::in_order(self) }in_order72,2425
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTPlainStEph::pre_order(self) }pre_order74,2508
    fn insert_node<T: StT + Ord>(node: &mut BBTree<T>, value: T) {insert_node77,2599
    fn contains_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> B {contains_node92,3102
    fn find_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> Option<&'a T> {find_node107,3588
    fn min_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {min_node122,4081
    fn max_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {max_node132,4401
    macro_rules! BSTPlainStEphLit {BSTPlainStEphLit143,4743
    fn _BSTPlainStEphLit_type_checks() {_BSTPlainStEphLit_type_checks158,5315

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetBBAlphaMtEph.rs,5697
pub mod BSTSetBBAlphaMtEph {BSTSetBBAlphaMtEph3,78
    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord> {BSTSetBBAlphaMtEph11,335
        tree: BSTBBAlphaMtEph<T>,tree12,390
    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;BSTSetBBAlphaMt15,431
    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetBBAlphaMtEphTrait17,489
        fn empty() -> Self;empty18,555
        fn singleton(value: T) -> Self;singleton19,583
        fn size(&self) -> N;size20,623
        fn is_empty(&self) -> B;is_empty21,652
        fn find(&self, value: &T) -> Option<T>;find22,685
        fn contains(&self, value: &T) -> B;contains23,733
        fn minimum(&self) -> Option<T>;minimum24,777
        fn maximum(&self) -> Option<T>;maximum25,817
        fn insert(&mut self, value: T);insert26,857
        fn delete(&mut self, target: &T);delete27,897
        fn union(&self, other: &Self) -> Self;union28,939
        fn intersection(&self, other: &Self) -> Self;intersection29,986
        fn difference(&self, other: &Self) -> Self;difference30,1040
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1092
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1147
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1202
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1264
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1334
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1402
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T>;as_tree37,1456
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph40,1513
        pub fn empty() -> Self {empty41,1565
        pub fn singleton(value: T) -> Self {singleton47,1688
        pub fn size(&self) -> N { self.tree.size() }size53,1849
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1903
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1965
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,2043
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2121
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2189
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2257
        pub fn delete(&mut self, target: &T) {delete67,2330
        pub fn union(&self, other: &Self) -> Self {union75,2619
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2918
        pub fn difference(&self, other: &Self) -> Self {difference100,3496
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4073
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4765
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5078
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5434
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5843
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6107
        pub fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree179,6190
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6259
        fn rebuild_from_vec(values: Vec<T>) -> BSTBBAlphaMtEph<T> {rebuild_from_vec183,6350
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6577
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph203,6864
        fn empty() -> Self { Self::empty() }empty204,6947
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6993
        fn size(&self) -> N { self.tree.size() }size208,7060
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7110
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7168
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7242
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7316
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7380
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7444
        fn delete(&mut self, target: &T) { BSTSetBBAlphaMtEph::delete(self, target) }delete222,7513
        fn union(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::union(self, other) }union224,7600
        fn intersection(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::intersection(self, otintersection226,7690
        fn difference(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::difference(self, other)difference228,7794
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetBBAlphaMtEph::split(self, pivot) }split230,7894
        fn join_pair(left: Self, right: Self) -> Self { BSTSetBBAlphaMtEph::join_pair(left, righjoin_pair232,7992
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetBBAlphaMtEph::join_m(left, join_m234,8094
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetBBAlphaMtEph::filtefilter236,8207
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetBBAlphaMtEph::reduce(reduce238,8325
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8440
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree242,8519
    macro_rules! BSTSetBBAlphaMtEphLit {BSTSetBBAlphaMtEphLit246,8610
    fn _BSTSetBBAlphaMtEphLit_type_checks() {_BSTSetBBAlphaMtEphLit_type_checks258,9209

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap38/BSTParaStEph.rs,3103
pub mod BSTParaStEph {BSTParaStEph3,70
    pub enum Exposed<T: StT + Ord> {Exposed11,269
        Leaf,Leaf13,325
        Node(ParamBST<T>, T, ParamBST<T>),Node14,339
    struct NodeInner<T: StT + Ord> {NodeInner18,417
        key: T,key19,454
        size: N,size20,470
        left: ParamBST<T>,left21,487
        right: ParamBST<T>,right22,514
    pub struct ParamBST<T: StT + Ord> {ParamBST26,577
        root: Rc<RefCell<Option<Box<NodeInner<T>>>>>,root27,617
    pub trait ParamBSTTrait<T: StT + Ord>: Sized {ParamBSTTrait30,678
        fn new() -> Self;new31,729
        fn expose(&self) -> Exposed<T>;expose32,755
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid33,795
        fn size(&self) -> N;size34,845
        fn is_empty(&self) -> B;is_empty35,874
        fn insert(&self, key: T);insert36,907
        fn delete(&self, key: &T);delete37,941
        fn find(&self, key: &T) -> Option<T>;find38,976
        fn split(&self, key: &T) -> (Self, B, Self);split39,1022
        fn join_pair(&self, other: Self) -> Self;join_pair40,1075
        fn union(&self, other: &Self) -> Self;union41,1125
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order42,1172
    impl<T: StT + Ord> ParamBST<T> {ParamBST45,1228
        fn expose_internal(&self) -> Exposed<T> {expose_internal46,1265
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid54,1557
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner66,2009
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m85,2994
        fn min_key(tree: &Self) -> Option<T> {min_key87,3110
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner97,3454
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner108,3924
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order120,4431
    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {ParamBST132,4833
        fn new() -> Self {new133,4891
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose139,5017
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid141,5084
        fn size(&self) -> N { self.root.borrow().as_ref().map_or(0, |node| node.size) }size143,5166
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty145,5255
        fn insert(&self, key: T) {insert147,5342
        fn delete(&self, key: &T) {delete154,5632
        fn find(&self, key: &T) -> Option<T> {find161,5924
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split172,6421
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair174,6511
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union176,6612
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order178,6698
    macro_rules! ParamBSTLit {ParamBSTLit186,6942
    fn _ParamBSTLit_type_checks() {_ParamBSTLit_type_checks199,7465

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap38/BSTParaMtEph.rs,4169
pub mod BSTParaMtEph {BSTParaMtEph3,69
    pub enum Exposed<T: StTInMtT + Ord> {Exposed10,237
        Leaf,Leaf11,279
        Node(ParamBST<T>, T, ParamBST<T>),Node12,293
    struct NodeInner<T: StTInMtT + Ord> {NodeInner16,364
        key: T,key17,406
        size: N,size18,422
        left: ParamBST<T>,left19,439
        right: ParamBST<T>,right20,466
    pub struct ParamBST<T: StTInMtT + Ord> {ParamBST24,529
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root25,574
    pub trait ParamBSTTrait<T: StTInMtT + Ord + 'static>: Sized {ParamBSTTrait28,635
        fn new() -> Self;new31,792
        fn expose(&self) -> Exposed<T>;expose34,909
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid37,1040
        fn size(&self) -> N;size40,1181
        fn is_empty(&self) -> B;is_empty43,1301
        fn insert(&self, key: T);insert46,1445
        fn delete(&self, key: &T);delete49,1590
        fn find(&self, key: &T) -> Option<T>;find52,1736
        fn split(&self, key: &T) -> (Self, B, Self);split55,1893
        fn join_pair(&self, other: Self) -> Self;join_pair58,2105
        fn union(&self, other: &Self) -> Self;union61,2280
        fn intersect(&self, other: &Self) -> Self;intersect64,2452
        fn difference(&self, other: &Self) -> Self;difference67,2628
        fn filter<F>(&self, predicate: F) -> Selffilter70,2785
        fn reduce<F>(&self, op: F, base: T) -> Treduce75,3009
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order80,3225
    impl<T: StTInMtT + Ord + 'static> ParamBST<T> {ParamBST83,3281
        fn expose_internal(&self) -> Exposed<T> {expose_internal86,3424
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid96,3814
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner110,4377
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m131,5453
        fn min_key(tree: &Self) -> Option<T> {min_key135,5680
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner147,6195
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner160,6790
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner176,7507
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner196,8470
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner217,9446
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel241,10528
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner251,10890
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel275,11969
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order285,12308
    impl<T: StTInMtT + Ord + 'static> ParamBSTTrait<T> for ParamBST<T> {ParamBST297,12710
        fn new() -> Self {new300,12874
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose308,13091
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid312,13249
        fn size(&self) -> N {size316,13422
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty323,13660
        fn insert(&self, key: T) {insert327,13858
        fn delete(&self, key: &T) {delete337,14301
        fn find(&self, key: &T) -> Option<T> {find347,14746
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split360,15354
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair364,15603
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union368,15829
        fn intersect(&self, other: &Self) -> Self { ParamBST::intersect_inner(self, other) }intersect372,16040
        fn difference(&self, other: &Self) -> Self { ParamBST::difference_inner(self, other) }difference376,16259
        fn filter<F>(&self, predicate: F) -> Selffilter380,16460
        fn reduce<F>(&self, op: F, base: T) -> Treduce389,16760
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order398,17051

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTTreapMtEph.rs,4433
pub mod BSTTreapMtEph {BSTTreapMtEph3,100
    type Link<T> = Option<Box<Node<T>>>;Link10,274
    struct Node<T: StTInMtT + Ord> {Node13,344
        key: T,key14,381
        priority: u64,priority15,397
        size: N,size16,420
        left: Link<T>,left17,437
        right: Link<T>,right18,460
    impl<T: StTInMtT + Ord> Node<T> {Node21,491
        fn new(key: T, priority: u64) -> Self {new22,529
    pub struct BSTTreapMtEph<T: StTInMtT + Ord> {BSTTreapMtEph34,784
        root: Arc<RwLock<Link<T>>>,root35,834
    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;BSTreeTreap38,877
    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTTreapMtEphTrait40,926
        fn new() -> Self;new41,987
        fn insert(&self, value: T);insert42,1013
        fn find(&self, target: &T) -> Option<T>;find43,1049
        fn contains(&self, target: &T) -> B;contains44,1098
        fn size(&self) -> N;size45,1143
        fn is_empty(&self) -> B;is_empty46,1172
        fn height(&self) -> N;height47,1205
        fn minimum(&self) -> Option<T>;minimum48,1236
        fn maximum(&self) -> Option<T>;maximum49,1276
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order50,1316
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order51,1365
    impl<T: StTInMtT + Ord> Default for BSTTreapMtEph<T> {BSTTreapMtEph54,1422
        fn default() -> Self { Self::new() }default55,1481
    impl<T: StTInMtT + Ord> BSTTreapMtEph<T> {BSTTreapMtEph58,1533
        pub fn new() -> Self {new59,1580
        pub fn size(&self) -> N {size65,1715
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty70,1848
        pub fn height(&self) -> N {height72,1939
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec73,1975
        pub fn insert(&self, value: T) {insert84,2325
        pub fn find(&self, target: &T) -> Option<T> {find90,2527
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains95,2697
        pub fn minimum(&self) -> Option<T> {minimum97,2811
        pub fn maximum(&self) -> Option<T> {maximum102,2963
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order107,3115
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order114,3400
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link121,3687
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate123,3770
        fn rotate_left(link: &mut Link<T>) {rotate_left125,3889
        fn rotate_right(link: &mut Link<T>) {rotate_right139,4344
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link153,4800
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link178,5820
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link193,6337
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link203,6658
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect213,6981
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect221,7268
    impl<T: StTInMtT + Ord> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {BSTTreapMtEph230,7564
        fn new() -> Self { BSTTreapMtEph::new() }new231,7637
        fn insert(&self, value: T) { BSTTreapMtEph::insert(self, value) }insert233,7688
        fn find(&self, target: &T) -> Option<T> { BSTTreapMtEph::find(self, target) }find235,7763
        fn contains(&self, target: &T) -> B { BSTTreapMtEph::contains(self, target) }contains237,7850
        fn size(&self) -> N { BSTTreapMtEph::size(self) }size239,7937
        fn is_empty(&self) -> B { BSTTreapMtEph::is_empty(self) }is_empty241,7996
        fn height(&self) -> N { BSTTreapMtEph::height(self) }height243,8063
        fn minimum(&self) -> Option<T> { BSTTreapMtEph::minimum(self) }minimum245,8126
        fn maximum(&self) -> Option<T> { BSTTreapMtEph::maximum(self) }maximum247,8199
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::in_order(self) }in_order249,8272
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::pre_order(self) }pre_order251,8355
    macro_rules! BSTTreapMtEphLit {BSTTreapMtEphLit255,8466
    fn _BSTTreapMtEphLit_type_checks() {_BSTTreapMtEphLit_type_checks267,8995

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTSetTreapMtEph.rs,5645
pub mod BSTSetTreapMtEph {BSTSetTreapMtEph3,75
    pub struct BSTSetTreapMtEph<T: StTInMtT + Ord> {BSTSetTreapMtEph11,312
        tree: BSTTreapMtEph<T>,tree12,365
    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;BSTSetTreapMt15,404
    pub trait BSTSetTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetTreapMtEphTrait17,458
        fn empty() -> Self;empty18,522
        fn singleton(value: T) -> Self;singleton19,550
        fn size(&self) -> N;size20,590
        fn is_empty(&self) -> B;is_empty21,619
        fn find(&self, value: &T) -> Option<T>;find22,652
        fn contains(&self, value: &T) -> B;contains23,700
        fn minimum(&self) -> Option<T>;minimum24,744
        fn maximum(&self) -> Option<T>;maximum25,784
        fn insert(&mut self, value: T);insert26,824
        fn delete(&mut self, target: &T);delete27,864
        fn union(&self, other: &Self) -> Self;union28,906
        fn intersection(&self, other: &Self) -> Self;intersection29,953
        fn difference(&self, other: &Self) -> Self;difference30,1007
        fn split(&self, pivot: &T) -> (Self, B, Self);split31,1059
        fn join_pair(left: Self, right: Self) -> Self;join_pair32,1114
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m33,1169
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter34,1231
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce35,1301
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order36,1369
        fn as_tree(&self) -> &BSTTreapMtEph<T>;as_tree37,1423
    impl<T: StTInMtT + Ord> BSTSetTreapMtEph<T> {BSTSetTreapMtEph40,1478
        pub fn empty() -> Self {empty41,1528
        pub fn singleton(value: T) -> Self {singleton47,1649
        pub fn size(&self) -> N { self.tree.size() }size53,1808
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty55,1862
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find57,1924
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains59,2002
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum61,2080
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum63,2148
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert65,2216
        pub fn delete(&mut self, target: &T) {delete67,2289
        pub fn union(&self, other: &Self) -> Self {union75,2578
        pub fn intersection(&self, other: &Self) -> Self {intersection83,2877
        pub fn difference(&self, other: &Self) -> Self {difference100,3455
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split117,4032
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair137,4724
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m145,5037
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter154,5393
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce167,5802
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order177,6066
        pub fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree179,6149
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec181,6216
        fn rebuild_from_vec(values: Vec<T>) -> BSTTreapMtEph<T> {rebuild_from_vec183,6307
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter191,6530
    impl<T: StTInMtT + Ord> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {BSTSetTreapMtEph203,6815
        fn empty() -> Self { Self::empty() }empty204,6894
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton206,6940
        fn size(&self) -> N { self.tree.size() }size208,7007
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty210,7057
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find212,7115
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains214,7189
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum216,7263
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum218,7327
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert220,7391
        fn delete(&mut self, target: &T) { BSTSetTreapMtEph::delete(self, target) }delete222,7460
        fn union(&self, other: &Self) -> Self { BSTSetTreapMtEph::union(self, other) }union224,7545
        fn intersection(&self, other: &Self) -> Self { BSTSetTreapMtEph::intersection(self, otheintersection226,7633
        fn difference(&self, other: &Self) -> Self { BSTSetTreapMtEph::difference(self, other) }difference228,7735
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetTreapMtEph::split(self, pivot) }split230,7833
        fn join_pair(left: Self, right: Self) -> Self { BSTSetTreapMtEph::join_pair(left, right)join_pair232,7929
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetTreapMtEph::join_m(left, pijoin_m234,8029
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetTreapMtEph::filter(filter236,8140
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetTreapMtEph::reduce(sereduce238,8256
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order240,8369
        fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree242,8448
    macro_rules! BSTSetTreapMtEphLit {BSTSetTreapMtEphLit246,8537
    fn _BSTSetTreapMtEphLit_type_checks() {_BSTSetTreapMtEphLit_type_checks258,9110

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTParaTreapMtEph.rs,4881
pub mod BSTParaTreapMtEph {BSTParaTreapMtEph3,96
    pub enum Exposed<T: StTInMtT + Ord> {Exposed12,329
        Leaf,Leaf13,371
        Node(ParamTreap<T>, T, ParamTreap<T>),Node14,385
    struct NodeInner<T: StTInMtT + Ord> {NodeInner18,460
        key: T,key19,502
        priority: i64,priority20,518
        size: N,size21,541
        left: ParamTreap<T>,left22,558
        right: ParamTreap<T>,right23,587
    pub struct ParamTreap<T: StTInMtT + Ord> {ParamTreap27,645
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root28,692
    fn priority_for<T: StTInMtT + Ord>(key: &T) -> i64 {priority_for31,753
    fn tree_priority<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> i64 {tree_priority39,1046
    fn tree_size<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> N {tree_size44,1233
    fn make_node<T: StTInMtT + Ord>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreamake_node49,1403
    impl<T: StTInMtT + Ord + 'static> ParamTreap<T> {ParamTreap62,1821
        fn expose_internal(&self) -> Exposed<T> {expose_internal65,1966
        pub fn expose_with_priority(&self) -> Option<(ParamTreap<T>, T, i64, ParamTreap<T>)> {expose_with_priority75,2356
        fn join_with_priority(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) join_with_priority84,2830
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid107,4007
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner119,4483
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner140,5614
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner154,6368
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner170,7112
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner190,8042
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner210,8955
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel234,10068
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner244,10432
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel268,11505
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order278,11846
    pub trait ParamTreapTrait<T: StTInMtT + Ord + 'static>: Sized {ParamTreapTrait290,12252
        fn new() -> Self;new293,12411
        fn expose(&self) -> Exposed<T>;expose296,12528
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid299,12659
        fn size(&self) -> N;size302,12800
        fn is_empty(&self) -> B;is_empty305,12920
        fn insert(&self, key: T);insert308,13064
        fn delete(&self, key: &T);delete311,13209
        fn find(&self, key: &T) -> Option<T>;find314,13355
        fn split(&self, key: &T) -> (Self, B, Self);split317,13512
        fn join_pair(&self, other: Self) -> Self;join_pair320,13724
        fn union(&self, other: &Self) -> Self;union323,13899
        fn intersect(&self, other: &Self) -> Self;intersect326,14071
        fn difference(&self, other: &Self) -> Self;difference329,14247
        fn filter<F>(&self, predicate: F) -> Selffilter332,14404
        fn reduce<F>(&self, op: F, base: T) -> Treduce337,14628
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order342,14844
    impl<T: StTInMtT + Ord + 'static> ParamTreapTrait<T> for ParamTreap<T> {ParamTreap345,14900
        fn new() -> Self {new348,15068
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose356,15287
        fn join_mid(exposed: Exposed<T>) -> Self { ParamTreap::join_mid(exposed) }join_mid360,15445
        fn size(&self) -> N { tree_size(self) }size364,15620
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty368,15760
        fn insert(&self, key: T) {insert372,15958
        fn delete(&self, key: &T) {delete383,16474
        fn find(&self, key: &T) -> Option<T> {find393,16923
        fn split(&self, key: &T) -> (Self, B, Self) { ParamTreap::split_inner(self, key) }split406,17535
        fn join_pair(&self, other: Self) -> Self { ParamTreap::join_pair_inner(self.clone(), othjoin_pair410,17786
        fn union(&self, other: &Self) -> Self { ParamTreap::union_inner(self, other) }union414,18014
        fn intersect(&self, other: &Self) -> Self { ParamTreap::intersect_inner(self, other) }intersect418,18227
        fn difference(&self, other: &Self) -> Self { ParamTreap::difference_inner(self, other) }difference422,18448
        fn filter<F>(&self, predicate: F) -> Selffilter426,18651
        fn reduce<F>(&self, op: F, base: T) -> Treduce435,18953
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order444,19246
    macro_rules! ParamTreapLit {ParamTreapLit452,19492
    fn _ParamTreapLit_type_checks() {_ParamTreapLit_type_checks464,20038

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTTreapStEph.rs,4539
pub mod BSTTreapStEph {BSTTreapStEph3,72
    type Link<T> = Option<Box<Node<T>>>;Link9,267
    struct Node<T: StT + Ord> {Node12,330
        key: T,key13,362
        priority: u64,priority14,378
        size: N,size15,401
        left: Link<T>,left16,418
        right: Link<T>,right17,441
    impl<T: StT + Ord> Node<T> {Node20,472
        fn new(key: T, priority: u64) -> Self {new21,505
    pub struct BSTTreapStEph<T: StT + Ord> {BSTTreapStEph33,760
        root: Link<T>,root34,805
    pub type BSTreeTreap<T> = BSTTreapStEph<T>;BSTreeTreap37,835
    pub trait BSTTreapStEphTrait<T: StT + Ord> {BSTTreapStEphTrait39,884
        fn new() -> Self;new40,933
        fn size(&self) -> N;size41,959
        fn is_empty(&self) -> B;is_empty42,988
        fn height(&self) -> N;height43,1021
        fn insert(&mut self, value: T);insert44,1052
        fn find(&self, target: &T) -> Option<&T>;find45,1092
        fn contains(&self, target: &T) -> B;contains46,1142
        fn minimum(&self) -> Option<&T>;minimum47,1187
        fn maximum(&self) -> Option<&T>;maximum48,1228
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order49,1269
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order50,1318
    impl<T: StT + Ord> Default for BSTreeTreap<T> {BSTreeTreap53,1375
        fn default() -> Self { Self::new() }default54,1427
    impl<T: StT + Ord> BSTTreapStEph<T> {BSTTreapStEph57,1479
        pub fn new() -> Self { BSTTreapStEph { root: None } }new58,1521
        pub fn size(&self) -> N { Self::size_link(&self.root) }size60,1584
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty62,1649
        pub fn height(&self) -> N {height64,1740
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec65,1776
        pub fn insert(&mut self, value: T) {insert74,2072
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find79,2221
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains81,2315
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum83,2429
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum85,2505
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order87,2581
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order93,2805
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link99,3031
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate101,3114
        fn rotate_left(link: &mut Link<T>) {rotate_left103,3233
        fn rotate_right(link: &mut Link<T>) {rotate_right117,3688
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link131,4144
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link156,5164
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link171,5681
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link181,6002
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect191,6325
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect199,6612
    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {BSTTreapStEph208,6908
        fn new() -> Self { BSTTreapStEph::new() }new209,6976
        fn size(&self) -> N { BSTTreapStEph::size(self) }size211,7027
        fn is_empty(&self) -> B { BSTTreapStEph::is_empty(self) }is_empty213,7086
        fn height(&self) -> N { BSTTreapStEph::height(self) }height215,7153
        fn insert(&mut self, value: T) { BSTTreapStEph::insert(self, value) }insert217,7216
        fn find(&self, target: &T) -> Option<&T> { BSTTreapStEph::find(self, target) }find219,7295
        fn contains(&self, target: &T) -> B { BSTTreapStEph::contains(self, target) }contains221,7383
        fn minimum(&self) -> Option<&T> { BSTTreapStEph::minimum(self) }minimum223,7470
        fn maximum(&self) -> Option<&T> { BSTTreapStEph::maximum(self) }maximum225,7544
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTTreapStEph::in_order(self) }in_order227,7618
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTTreapStEph::pre_order(self) }pre_order229,7701
    macro_rules! BSTTreapStEphLit {BSTTreapStEphLit233,7812
    fn _BSTTreapStEphLit_type_checks() {_BSTTreapStEphLit_type_checks245,8345

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test42BBTEph.rs,189
fn inorder_and_preorder_traversals_match_definitions() {inorder_and_preorder_traversals_match_definitions7,143
fn bst_insert_and_search_behavior() {bst_insert_and_search_behavior22,619

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test29Algorithm_21_1.rs,431
fn points2d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, N>> {points2d_tab_flat13,472
fn test_points2d_n3_example() {test_points2d_n3_example26,1006
fn test_points2d_n1_empty() {test_points2d_n1_empty40,1297
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values46,1403
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order54,1602
fn test_points2d_debug_shape() {test_points2d_debug_shape75,2105

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test01Types.rs,677
pub mod TestTypes {TestTypes1,0
    fn test_boolean_eq_neq_and_ordering() {test_boolean_eq_neq_and_ordering5,67
    fn test_ordering_on_n_naturals() {test_ordering_on_n_naturals15,324
    fn test_cmp_on_b_returns_expected_ordering_variants() {test_cmp_on_b_returns_expected_ordering_variants24,571
    fn test_btree_set_orders_b_true_before_false() {test_btree_set_orders_b_true_before_false32,895
    fn test_n_aliases_usize_and_cmp_examples() {test_n_aliases_usize_and_cmp_examples44,1344
    fn test_debug_format_for_b_variants() {test_debug_format_for_b_variants59,1797
    fn test_display_format_for_b_variants() {test_display_format_for_b_variants65,1970

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test12ArraySeqStEph.rs,283
pub mod TestArraySeqEph {TestArraySeqEph1,0
    fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic11,308
    fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter20,574
    fn test_iterators_collect() {test_iterators_collect31,1097

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test14ArraySeqStEph.rs,1419
pub mod TestArraySeqStEph {TestArraySeqStEph3,93
    fn test_empty() {test_empty9,320
    fn test_singleton() {test_singleton15,456
    fn test_map() {test_map21,614
    fn test_append() {test_append28,858
    fn test_append2() {test_append236,1175
    fn test_deflate_true() {test_deflate_true44,1494
    fn test_deflate_false() {test_deflate_false50,1706
    fn test_filter_even_numbers() {test_filter_even_numbers56,1908
    fn test_filter_none() {test_filter_none63,2219
    fn test_update_in_bounds() {test_update_in_bounds70,2515
    fn test_update_out_of_bounds() {test_update_out_of_bounds77,2751
    fn test_isEmpty() {test_isEmpty84,2984
    fn test_isSingleton() {test_isSingleton94,3351
    fn test_iterate_sum() {test_iterate_sum104,3734
    fn test_iterate_concat() {test_iterate_concat111,3964
    fn test_map_empty() {test_map_empty125,4381
    fn test_append_with_empty() {test_append_with_empty132,4603
    fn test_append2_equivalence() {test_append2_equivalence142,5054
    fn test_filter_empty_sequence() {test_filter_empty_sequence151,5424
    fn test_select_boundary() {test_select_boundary158,5663
    fn test_subseq_basic() {test_subseq_basic169,6181
    fn test_reduce_sum_basic_ch19() {test_reduce_sum_basic_ch19176,6403
    fn test_scan_sum_basic_ch19() {test_scan_sum_basic_ch19190,6982
    fn test_flatten_ch19() {test_flatten_ch19201,7381

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test13ArraySeqStEph18.rs,1040
pub mod TestArraySeqStEph {TestArraySeqStEph3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,352
        fn fib(n: N) -> N {fib12,387
    fn test_map_increment() {test_map_increment37,1067
    fn test_subseq() {test_subseq44,1307
    fn test_append() {test_append55,1797
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,2068
    fn test_filter_even() {test_filter_even76,2735
    fn test_flatten() {test_flatten94,3412
    fn test_update_sequence() {test_update_sequence108,4133
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins120,4648
    fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins133,5406
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan149,6158
    fn test_iterate_sum_basic() {test_iterate_sum_basic168,7162
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum176,7434
    fn test_collect_groups_by_key() {test_collect_groups_by_key188,7895

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test10ArraySeqStPer18.rs,937
pub mod TestArraySeqStPer {TestArraySeqStPer1,0
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,333
        fn fib(n: N) -> N {fib12,368
    fn test_map_increment() {test_map_increment37,1037
    fn test_subseq() {test_subseq44,1281
    fn test_append() {test_append55,1627
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1902
    fn test_filter_even() {test_filter_even76,2543
    fn test_flatten() {test_flatten95,3315
    fn test_update_sequence() {test_update_sequence110,4145
    fn test_inject_and_ninject() {test_inject_and_ninject120,4660
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan145,6031
    fn test_iterate_sum_basic() {test_iterate_sum_basic164,7005
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum172,7277
    fn test_collect_groups_by_key() {test_collect_groups_by_key184,7738

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test28ArraySeqMtPer.rs,573
pub mod Test28ArraySeqMtPer {Test28ArraySeqMtPer3,55
    fn test_inject_basic() {test_inject_basic13,365
    fn test_inject_conflicting_updates() {test_inject_conflicting_updates29,972
    fn test_inject_out_of_bounds() {test_inject_out_of_bounds45,1638
    fn test_inject_empty_changes() {test_inject_empty_changes57,2148
    fn test_inject_empty_values() {test_inject_empty_values69,2566
    fn test_atomic_write_migrated_from_st_test() {test_atomic_write_migrated_from_st_test83,3204
    fn test_inject_string_values() {test_inject_string_values108,4400

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test41ArraySeqMtEph.rs,156
fn test_arrayseq_mteph_basic_ops() {test_arrayseq_mteph_basic_ops5,57
fn test_arrayseq_mteph_append_and_map() {test_arrayseq_mteph_append_and_map22,518

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test26ArraySeqMtPer.rs,910
pub mod Test26ArraySeqMtPer {Test26ArraySeqMtPer3,93
    fn test_new_and_set() {test_new_and_set9,286
    fn test_length_and_nth_basic() {test_length_and_nth_basic23,733
    fn test_empty() {test_empty31,943
    fn test_sequence_basic() {test_sequence_basic38,1124
    fn test_singleton() {test_singleton51,1648
    fn test_from_vec() {test_from_vec59,1866
    fn test_subseq_copy() {test_subseq_copy67,2061
    fn test_subseq_view() {test_subseq_view77,2348
    fn test_iterators() {test_iterators87,2617
    fn test_set_out_of_bounds() {test_set_out_of_bounds100,2986
    fn test_macro_literals() {test_macro_literals107,3165
    fn test_equality_and_debug() {test_equality_and_debug127,3768
    fn test_display_format() {test_display_format143,4208
    fn test_string_sequences() {test_string_sequences152,4476
    fn test_boolean_sequences() {test_boolean_sequences160,4694

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test11ArraySeqStPer.rs,346
pub mod TestArraySeqPer {TestArraySeqPer1,0
    fn test_map_and_select_and_append() {test_map_and_select_and_append11,307
    fn test_deflate_and_filter() {test_deflate_and_filter24,856
    fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten37,1410
    fn test_inject_and_parallel() {test_inject_and_parallel56,2415

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test27ArraySeqMtPer18.rs,585
pub mod Test27ArraySeqMtPer {Test27ArraySeqMtPer3,55
    fn test_tabulate_basic() {test_tabulate_basic10,280
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci19,565
        fn fib(n: N) -> N {fib20,600
    fn test_tabulate_empty() {test_tabulate_empty51,1424
    fn test_tabulate_single() {test_tabulate_single58,1632
    fn test_tabulate_string() {test_tabulate_string65,1834
    fn test_tabulate_boolean() {test_tabulate_boolean80,2347
    fn test_tabulate_squares() {test_tabulate_squares95,2851
    fn test_tabulate_large() {test_tabulate_large110,3261

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test09ArraySeqStPer.rs,2133
pub mod TestArraySeqStPer {TestArraySeqStPer3,93
    fn test_new_and_set() {test_new_and_set9,287
    fn test_length_and_nth_basic() {test_length_and_nth_basic23,747
    fn test_empty() {test_empty31,960
    fn test_sequence_basic() {test_sequence_basic38,1150
    fn test_singleton() {test_singleton51,1861
    fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton59,2069
    fn test_from_vec() {test_from_vec74,2548
    fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers89,3223
    fn test_sequence_equality_strings() {test_sequence_equality_strings114,4165
    fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference139,5201
        struct PartialComparable {PartialComparable141,5292
            value: i32,value142,5327
        impl std::fmt::Display for PartialComparable {PartialComparable145,5413
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt146,5468
        struct TotalComparable {TotalComparable158,6112
            value: N,value159,6145
        impl std::fmt::Display for TotalComparable {TotalComparable162,6178
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt163,6231
    fn test_ordering_numbers_basic() {test_ordering_numbers_basic178,6896
    fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal187,7143
    fn test_ordering_strings_basic() {test_ordering_strings_basic193,7271
    fn test_strings_equal_is_equal() {test_strings_equal_is_equal202,7516
    fn test_nth_on_empty_panics() {test_nth_on_empty_panics209,7665
    fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics216,7818
    fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap223,7959
    fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes229,8128
    fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic238,8371
    fn test_new_set_persistent() {test_new_set_persistent247,8730
    fn test_iterator_collects_in_order() {test_iterator_collects_in_order256,9005

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test53BSTParaTreapMtEph.rs,612
fn make_tree(values: &[i32]) -> ParamTreap<i32> {make_tree5,103
fn make_range_tree(start: i32, end: i32) -> ParamTreap<i32> {make_range_tree13,260
fn treap_basic_insert_find() {treap_basic_insert_find22,440
fn treap_split_join_pair() {treap_split_join_pair32,758
fn treap_union_intersect_difference() {treap_union_intersect_difference44,1165
fn treap_filter_reduce() {treap_filter_reduce62,1926
fn treap_join_mid_roundtrip() {treap_join_mid_roundtrip75,2322
fn treap_invariants_priority_heap() {treap_invariants_priority_heap94,2916
    fn check_heap(tree: &ParamTreap<i32>) {check_heap95,2954

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test24DirGraphStEph.rs,129
pub mod TestDirGraphStEph {TestDirGraphStEph1,0
    fn test_digraph_vertices_and_arcs() {test_digraph_vertices_and_arcs8,205

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test26LabDirGraphStEph.rs,1037
pub mod TestLabDirGraphStEph {TestLabDirGraphStEph1,0
    fn test_labelled_dir_graph_empty() {test_labelled_dir_graph_empty8,237
    fn test_labelled_dir_graph_add_vertex() {test_labelled_dir_graph_add_vertex16,528
    fn test_labelled_dir_graph_add_labeled_arc() {test_labelled_dir_graph_add_labeled_arc28,912
    fn test_labelled_dir_graph_neighbors() {test_labelled_dir_graph_neighbors45,1522
    fn test_labelled_dir_graph_arcs() {test_labelled_dir_graph_arcs69,2376
    fn test_labelled_dir_graph_macro_empty() {test_labelled_dir_graph_macro_empty81,2765
    fn test_labelled_dir_graph_macro_with_data() {test_labelled_dir_graph_macro_with_data88,2992
    fn test_labelled_dir_graph_different_label_types() {test_labelled_dir_graph_different_label_types103,3476
    fn test_labelled_dir_graph_display() {test_labelled_dir_graph_display120,3962
    fn test_labelled_dir_graph_debug() {test_labelled_dir_graph_debug132,4316
    fn test_labelled_dir_graph_self_loop() {test_labelled_dir_graph_self_loop144,4675

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test27LabUnDirGraphStEph.rs,1329
pub mod TestLabUnDirGraphStEph {TestLabUnDirGraphStEph1,0
    fn test_labelled_undir_graph_empty() {test_labelled_undir_graph_empty8,245
    fn test_labelled_undir_graph_add_vertex() {test_labelled_undir_graph_add_vertex16,545
    fn test_labelled_undir_graph_add_labeled_edge() {test_labelled_undir_graph_add_labeled_edge28,936
    fn test_labelled_undir_graph_neighbors() {test_labelled_undir_graph_neighbors49,1854
    fn test_labelled_undir_graph_edges() {test_labelled_undir_graph_edges72,2672
    fn test_labelled_undir_graph_macro_empty() {test_labelled_undir_graph_macro_empty86,3189
    fn test_labelled_undir_graph_macro_with_data() {test_labelled_undir_graph_macro_with_data93,3423
    fn test_labelled_undir_graph_edge_normalization() {test_labelled_undir_graph_edge_normalization110,4018
    fn test_labelled_undir_graph_different_label_types() {test_labelled_undir_graph_different_label_types124,4535
    fn test_labelled_undir_graph_display() {test_labelled_undir_graph_display143,5182
    fn test_labelled_undir_graph_debug() {test_labelled_undir_graph_debug155,5542
    fn test_labelled_undir_graph_self_loop() {test_labelled_undir_graph_self_loop167,5908
    fn test_labelled_undir_graph_multiple_edges_same_vertices() {test_labelled_undir_graph_multiple_edges_same_vertices180,6335

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test25UnDirGraphStEph.rs,139
pub mod TestUnDirGraphStEph {TestUnDirGraphStEph1,0
    fn test_undigraph_vertices_and_edges() {test_undigraph_vertices_and_edges8,211

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/TestExercise12_5.rs,316
fn push_pop_lifo_single_thread() {push_pop_lifo_single_thread7,179
fn pop_on_empty_returns_none() {pop_on_empty_returns_none21,517
fn multi_thread_push_collects_all_items() {multi_thread_push_collects_all_items27,665
fn multi_thread_pop_consumes_all_elements() {multi_thread_pop_consumes_all_elements55,1485

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap03/TestInsertionSortSt.rs,727
fn sort_and_assert(mut data: Vec<i32>, expected: &[i32]) {sort_and_assert3,97
fn insertion_sort_handles_empty() {insertion_sort_handles_empty9,250
fn insertion_sort_single_element() { sort_and_assert(vec![42], &[42]); }insertion_sort_single_element16,419
fn insertion_sort_already_sorted() { sort_and_assert(vec![1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]); }insertion_sort_already_sorted19,501
fn insertion_sort_reverse_order() { sort_and_assert(vec![5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]); }insertion_sort_reverse_order22,605
fn insertion_sort_with_duplicates() { sort_and_assert(vec![3, 1, 2, 3, 1], &[1, 1, 2, 3, 3]); }insertion_sort_with_duplicates25,708
fn insertion_sort_random_slice() {insertion_sort_random_slice28,813

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test39Chapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test35Exercsise_21_9.rs,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test38Problem21_1.rs,429
fn points2d(n: N) -> ArraySeqStPerS<Pair<N, N>> {points2d10,325
fn test_points2d_n3_example() {test_points2d_n3_example25,654
fn test_points2d_n1_empty() {test_points2d_n1_empty33,903
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values39,1000
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order47,1182
fn test_points2d_debug_display_shapes() {test_points2d_debug_display_shapes68,1628

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test39Chapter36St.rs,610
trait ToVec<T: StT> {ToVec5,106
    fn to_vec(&self) -> Vec<T>;to_vec6,128
impl<T: StT> ToVec<T> for ArraySeqStEphS<T> {ArraySeqStEphS8,162
    fn to_vec(&self) -> Vec<T> { (0..self.length()).map(|i| self.nth(i).clone()).collect() }to_vec9,208
fn quick_sort_variants_produce_sorted_output() {quick_sort_variants_produce_sorted_output13,312
fn quick_sort_handles_edge_cases() {quick_sort_handles_edge_cases28,836
fn pivot_strategies_match_expectations() {pivot_strategies_match_expectations51,1611
fn quick_sort_small_inputs_use_shared_pivots() {quick_sort_small_inputs_use_shared_pivots76,2386

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test30Algorithm_21_2.rs,530
fn points3d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {points3d_tab_flat13,529
fn test_points3d_tab_flat_n0_empty() {test_points3d_tab_flat_n0_empty42,1892
fn test_points3d_tab_flat_n1_single() {test_points3d_tab_flat_n1_single48,2007
fn test_points3d_tab_flat_n2_values_and_order() {test_points3d_tab_flat_n2_values_and_order55,2177
fn test_points3d_tab_flat_iterator_order() {test_points3d_tab_flat_iterator_order72,2606
fn test_points3d_tab_flat_debug_shape() {test_points3d_tab_flat_debug_shape91,3100

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test34Exercise_21_8_and_Algorithm_21_5.rs,396
fn is_divisible(n: N, i: N) -> B { if n % i == 0 { B::True } else { B::False } }is_divisible8,248
fn is_prime(n: N) -> B {is_prime13,491
fn primes_bf(n: N) -> ArraySeqStPerS<N> {primes_bf27,1094
fn test_is_prime_small_values() {test_is_prime_small_values37,1458
fn test_primes_bf_small() {test_primes_bf_small47,1735
fn test_primes_bf_debug_shape() {test_primes_bf_debug_shape54,1887

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_2.txt,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test03LinkedListStPer.rs,537
pub mod TestLinkedListPer {TestLinkedListPer1,0
    fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates8,217
    fn test_new_and_nth_set() {test_new_and_nth_set17,512
    fn test_subseq_copy() {test_subseq_copy29,883
    fn test_from_vec_and_debug_format() {test_from_vec_and_debug_format38,1122
    fn test_iter_inorder_collect() {test_iter_inorder_collect45,1318
    fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics53,1544
    fn test_display_impl() {test_display_impl59,1674

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test33Exercise_21_7.rs,432
fn is_even(x: &N) -> B { if *x % 2 == 0 { B::True } else { B::False } }is_even10,327
fn is_vowel(c: &char) -> B {is_vowel11,399
fn pair_even_with_vowels(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<char>) -> ArraySeqStPerS<Pairpair_even_with_vowels20,688
fn test_pair_even_with_vowels_basic() {test_pair_even_with_vowels_basic39,1561
fn test_pair_even_with_vowels_debug_shape() {test_pair_even_with_vowels_debug_shape48,1866

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test23MappingStEph.rs,562
pub mod Test23MappingStEphChap5_5 {Test23MappingStEphChap5_53,55
    fn test_empty_mapping() {test_empty_mapping12,365
    fn test_from_vec_basic() {test_from_vec_basic20,579
    fn test_from_vec_duplicate_keys() {test_from_vec_duplicate_keys32,1086
    fn test_from_relation() {test_from_relation43,1644
    fn test_domain_and_range() {test_domain_and_range57,2413
    fn test_iter() {test_iter76,3128
    fn test_mem_comprehensive() {test_mem_comprehensive89,3612
    fn test_empty_mapping_operations() {test_empty_mapping_operations108,4281

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test22RelationStEph.rs,151
pub mod TestRelationStEphChap5_2 {TestRelationStEphChap5_21,0
    fn test_relation_domain_range_and_mem() {test_relation_domain_range_and_mem9,270

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test21SetStEph.rs,575
pub mod TestSetStEphChap5_1 {TestSetStEphChap5_11,0
    fn macro_typecheck_exercise() {macro_typecheck_exercise9,205
        let _empty: Set<&'static str> = SetLit![];str10,241
    fn test_cartesian_product_example_5_1() {test_cartesian_product_example_5_116,391
    fn test_partition_example_5_2_true() {test_partition_example_5_2_true36,961
    fn test_partition_example_5_2_false_due_to_overlap() {test_partition_example_5_2_false_due_to_overlap45,1258
    fn test_partition_false_due_to_missing_element() {test_partition_false_due_to_missing_element54,1603

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/TestExercise12_2.rs,239
fn fetch_add_cas_returns_previous_value() {fetch_add_cas_returns_previous_value7,178
fn trait_impl_matches_free_function() {trait_impl_matches_free_function14,368
fn fetch_add_cas_is_thread_safe() {fetch_add_cas_is_thread_safe24,654

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test48BSTTreapStEph.rs,166
fn treap_insert_find_stays_balanced() {treap_insert_find_stays_balanced5,71
fn treap_duplicate_insert_is_idempotent() {treap_duplicate_insert_is_idempotent25,698

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test44Chapter36MtSlice.rs,769
fn to_vec<T: StT>(a: &ArraySeqMtEphSliceS<T>) -> Vec<T> { a.to_vec() }to_vec6,112
fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }is_sorted8,184
fn mk_seq(data: &[i32]) -> ArraySeqMtEphSliceS<i32> { ArraySeqMtEphSliceS::from_vec(data.to_vec(mk_seq10,278
fn quick_sort_slice_variants_produce_sorted_output() {quick_sort_slice_variants_produce_sorted_output13,388
fn quick_sort_slice_edge_cases() {quick_sort_slice_edge_cases31,877
fn quick_sort_slice_large_inputs() {quick_sort_slice_large_inputs54,1555
fn slice_pivot_strategies_match_expectations() {slice_pivot_strategies_match_expectations67,2040
fn quick_sort_slice_small_inputs_use_shared_pivots() {quick_sort_slice_small_inputs_use_shared_pivots87,2665

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap23/TestPrimTreeSeqSt.rs,494
pub mod TestPrimTreeSeqSt {TestPrimTreeSeqSt1,0
    fn expose_zero_returns_zero() {expose_zero_returns_zero6,133
    fn expose_one_returns_one() {expose_one_returns_one12,316
    fn expose_two_splits_sequence() {expose_two_splits_sequence21,593
    fn join_zero_creates_empty_sequence() {join_zero_creates_empty_sequence36,1155
    fn join_two_concatenates_sequences() {join_two_concatenates_sequences42,1339
    fn expose_then_join_roundtrip() {expose_then_join_roundtrip51,1741

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test37Problem_21_4.rs,1238
fn cartesian_loops(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<&'static str>) -> ArraySeqStPerS<Pacartesian_loops12,387
    let mut v: Vec<Pair<N, &'static str>> = Vec::with_capacity(a.length() * b.length());str13,506
fn cartesian_tab_flat(cartesian_tab_flat24,889
    let nested: ArraySeqStPerS<ArraySeqStPerS<Pair<N, &'static str>>> =str28,1022
        <ArraySeqStPerS<ArraySeqStPerS<Pair<N, &'static str>>> as ArraySeqStPerTrait<str29,1094
            ArraySeqStPerS<Pair<N, &'static str>>,str30,1180
                <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static ststr33,1271
                <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static ststr33,1271
    <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::flattestr40,1535
    <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::flattestr40,1535
fn test_cartesian_loops_basic() {test_cartesian_loops_basic44,1653
fn test_cartesian_tab_flat_basic() {test_cartesian_tab_flat_basic60,2040
fn test_cartesian_iterator_order() {test_cartesian_iterator_order76,2433
fn test_cartesian_debug_shape() {test_cartesian_debug_shape85,2743

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test08LinkedListStEph19.rs,614
pub mod TestLinkedListStEph {TestLinkedListStEph1,0
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list8,269
    fn test_eph_set_and_nth() {test_eph_set_and_nth17,530
    fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug24,713
    fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1934,1074
    fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1940,1244
    fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1953,1841

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test01ArraySeqMacro.rs,594
fn arrayseqs_empty_macro() {arrayseqs_empty_macro6,132
fn arrayseqs_literal_macro_keeps_order() {arrayseqs_literal_macro_keeps_order13,320
fn arrayseqs_repeat_macro_clones_element() {arrayseqs_repeat_macro_clones_element21,564
fn arrayseq_tabulate_and_map_work() {arrayseq_tabulate_and_map_work31,862
fn arrayseq_subseq_append_filter_flatten() {arrayseq_subseq_append_filter_flatten42,1263
fn arrayseq_update_and_inject_preserve_original() {arrayseq_update_and_inject_preserve_original68,2262
fn arrayseq_collect_iterate_reduce_scan() {arrayseq_collect_iterate_reduce_scan83,2854

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test12ArraySeqStEphMacro.rs,141
fn arrayseq_steph_basic_macros() {arrayseq_steph_basic_macros6,147
fn arrayseq_steph_full_pipeline() {arrayseq_steph_full_pipeline15,490

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test10ArraySeqStPerChap18.rs,945
pub mod TestArraySeqStPerChap {TestArraySeqStPerChap1,0
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,355
        fn fib(n: N) -> N {fib12,390
    fn test_map_increment() {test_map_increment37,1059
    fn test_subseq() {test_subseq44,1309
    fn test_append() {test_append55,1655
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1936
    fn test_filter_even() {test_filter_even76,2595
    fn test_flatten() {test_flatten91,3304
    fn test_update_sequence() {test_update_sequence105,4134
    fn test_inject_and_ninject() {test_inject_and_ninject115,4661
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan140,6056
    fn test_iterate_sum_basic() {test_iterate_sum_basic159,7054
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum167,7332
    fn test_collect_groups_by_key() {test_collect_groups_by_key179,7799

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test13ArraySeqStEphChap18.rs,1048
pub mod TestArraySeqStEphChap {TestArraySeqStEphChap3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,368
        fn fib(n: N) -> N {fib12,403
    fn test_map_increment() {test_map_increment37,1089
    fn test_subseq() {test_subseq44,1335
    fn test_append() {test_append55,1825
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,2102
    fn test_filter_even() {test_filter_even76,2787
    fn test_flatten() {test_flatten90,3401
    fn test_update_sequence() {test_update_sequence104,4134
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins116,4649
    fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins129,5419
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan145,6183
    fn test_iterate_sum_basic() {test_iterate_sum_basic164,7211
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum172,7489
    fn test_collect_groups_by_key() {test_collect_groups_by_key185,7968

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test05LinkedListStPer19.rs,427
pub mod TestLinkedListPer {TestLinkedListPer1,0
    fn test_select() {test_select9,279
    fn test_append_variants() {test_append_variants22,779
    fn test_deflate() {test_deflate32,1208
    fn test_map() {test_map42,1633
    fn test_iterate_and_reduce() {test_iterate_and_reduce49,1859
    fn test_scan() {test_scan58,2216
    fn test_flatten() {test_flatten66,2504
    fn test_inject() {test_inject73,2787

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test02MathSeq.rs,1347
pub mod TestMathSeq {TestMathSeq3,93
    fn test_length_and_nth_basic() {test_length_and_nth_basic10,256
    fn test_add_last_and_delete_last() {test_add_last_and_delete_last18,476
    fn test_new_empty_singleton_and_predicates() {test_new_empty_singleton_and_predicates31,925
    fn test_set_in_bounds_and_out_of_bounds() {test_set_in_bounds_and_out_of_bounds50,1523
    fn test_subseq_view_bounds() {test_subseq_view_bounds61,1882
    fn test_subseq_copy_bounds() {test_subseq_copy_bounds74,2279
    fn test_domain() {test_domain84,2575
    fn test_range_deduplicates_preserving_order() {test_range_deduplicates_preserving_order90,2716
    fn test_debug_format_for_mathseq() {test_debug_format_for_mathseq97,2944
    fn test_display_format_for_mathseq() {test_display_format_for_mathseq103,3109
    fn test_multiset_range_counts_first_occurrence_order() {test_multiset_range_counts_first_occurrence_order109,3274
    fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics117,3524
    fn test_range_empty_returns_empty() {test_range_empty_returns_empty123,3654
    fn test_multiset_range_empty_returns_empty() {test_multiset_range_empty_returns_empty130,3827
    fn test_domain_empty_is_empty() {test_domain_empty_is_empty137,4023
    fn test_iter_collect_and_sum() {test_iter_collect_and_sum144,4185

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test04LinkedListStPer.rs,569
pub mod TestLinkedListStPer {TestLinkedListStPer1,0
    fn test_tabulate() {test_tabulate9,281
    fn test_map() {test_map16,484
    fn test_filter() {test_filter24,767
    fn test_append() {test_append37,1180
    fn test_update() {test_update45,1485
    fn test_inject() {test_inject52,1734
    fn test_ninject() {test_ninject60,2049
    fn test_iterate() {test_iterate68,2368
    fn test_reduce() {test_reduce75,2594
    fn test_scan() {test_scan82,2815
    fn test_flatten() {test_flatten90,3115
    fn test_collect() {test_collect101,3479

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test27ArraySeqMtPerChap18.rs,593
pub mod Test27ArraySeqMtPerChap {Test27ArraySeqMtPerChap3,61
    fn test_tabulate_basic() {test_tabulate_basic10,302
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci19,587
        fn fib(n: N) -> N {fib20,622
    fn test_tabulate_empty() {test_tabulate_empty51,1446
    fn test_tabulate_single() {test_tabulate_single58,1654
    fn test_tabulate_string() {test_tabulate_string65,1856
    fn test_tabulate_boolean() {test_tabulate_boolean80,2369
    fn test_tabulate_squares() {test_tabulate_squares95,2873
    fn test_tabulate_large() {test_tabulate_large110,3283

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test07LinkedListStEph.rs,927
pub mod TestLinkedListStEph {TestLinkedListStEph1,0
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list8,269
    fn test_construct_eph_from_vec() {test_construct_eph_from_vec19,638
    fn test_eph_is_empty_and_singleton() {test_eph_is_empty_and_singleton25,799
    fn test_eph_set_and_subseq_copy() {test_eph_set_and_subseq_copy33,1086
    fn test_iter_inorder_collect_eph_ch18() {test_iter_inorder_collect_eph_ch1842,1353
    fn test_tabulate_and_map_ch18() {test_tabulate_and_map_ch1848,1523
    fn test_append_ch18() {test_append_ch1855,1837
    fn test_filter_ch18() {test_filter_ch1863,2157
    fn test_update_ch18() {test_update_ch1876,2536
    fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1883,2767
    fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1893,3262
    fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch18105,3832

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test11ArraySeqStPerMacro.rs,275
fn arrayseq_stper_macro_empty() {arrayseq_stper_macro_empty6,172
fn arrayseq_stper_macro_literal() {arrayseq_stper_macro_literal13,387
fn arrayseq_stper_macro_repeat() {arrayseq_stper_macro_repeat20,595
fn arrayseq_stper_operations() {arrayseq_stper_operations29,951

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test06LinkedListStEph.rs,547
pub mod TestLinkedListEph {TestLinkedListEph2,56
    fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates10,274
    fn test_new_and_nth_set() {test_new_and_nth_set19,569
    fn test_subseq_copy() {test_subseq_copy28,808
    fn test_linkedlisteph_basic() {test_linkedlisteph_basic37,1047
    fn test_debug_format_for_eph() {test_debug_format_for_eph46,1291
    fn test_display_format_for_eph() {test_display_format_for_eph52,1447
    fn test_iter_inorder_collect_eph() {test_iter_inorder_collect_eph58,1603

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test40Chapter36Mt.rs,654
fn to_vec<T: StT>(a: &ArraySeqMtEphS<T>) -> Vec<T> { (0..a.length()).map(|i| a.nth_cloned(i)).coto_vec7,129
fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }is_sorted9,236
fn quick_sort_mt_variants_produce_sorted_output() {quick_sort_mt_variants_produce_sorted_output12,338
fn quick_sort_mt_edge_cases() {quick_sort_mt_edge_cases30,845
fn pivot_mt_strategies_match_expectations() {pivot_mt_strategies_match_expectations53,1569
fn quick_sort_mt_large_inputs() {quick_sort_mt_large_inputs74,2276
fn quick_sort_mt_small_inputs_use_shared_pivots() {quick_sort_mt_small_inputs_use_shared_pivots87,2756

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test51BSTSetMtEph.rs,11026
trait TestSet: Sized {TestSet4,49
    fn empty() -> Self;empty5,72
    fn insert(&mut self, value: i32);insert6,96
    fn delete(&mut self, value: &i32);delete7,134
    fn size(&self) -> usize;size8,173
    fn is_empty(&self) -> B;is_empty9,202
    fn contains(&self, value: &i32) -> B;contains10,231
    fn minimum(&self) -> Option<i32>;minimum11,273
    fn maximum(&self) -> Option<i32>;maximum12,311
    fn union(&self, other: &Self) -> Self;union13,349
    fn intersection(&self, other: &Self) -> Self;intersection14,392
    fn difference(&self, other: &Self) -> Self;difference15,442
    fn split(&self, pivot: &i32) -> (Self, B, Self);split16,490
    fn join_pair(left: Self, right: Self) -> Self;join_pair17,543
    fn join_m(left: Self, pivot: i32, right: Self) -> Self;join_m18,594
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self;filter19,654
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32;reduce20,722
    fn iter_seq(&self) -> ArraySeqStPerS<i32>;iter_seq21,796
impl TestSet for apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32> {BSTSetPlainMt24,846
    fn empty() -> Self { Self::empty() }empty25,937
    fn insert(&mut self, value: i32) { self.insert(value); }insert27,979
    fn delete(&mut self, value: &i32) { self.delete(value); }delete29,1041
    fn size(&self) -> usize { self.size() }size31,1104
    fn is_empty(&self) -> B { self.is_empty() }is_empty33,1149
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains35,1198
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum37,1265
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum39,1322
    fn union(&self, other: &Self) -> Self { self.union(other) }union41,1379
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection43,1444
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference45,1523
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split47,1598
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair49,1673
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m51,1757
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter53,1854
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce55,1949
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq57,2049
impl TestSet for apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32> {BSTSetAVLMt60,2123
    fn empty() -> Self { Self::empty() }empty61,2208
    fn insert(&mut self, value: i32) { self.insert(value); }insert63,2250
    fn delete(&mut self, value: &i32) { self.delete(value); }delete65,2312
    fn size(&self) -> usize { self.size() }size67,2375
    fn is_empty(&self) -> B { self.is_empty() }is_empty69,2420
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains71,2469
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum73,2536
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum75,2593
    fn union(&self, other: &Self) -> Self { self.union(other) }union77,2650
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection79,2715
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference81,2794
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split83,2869
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair85,2944
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m87,3028
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter89,3125
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce91,3220
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq93,3320
impl TestSet for apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32> {BSTSetRBMt96,3394
    fn empty() -> Self { Self::empty() }empty97,3476
    fn insert(&mut self, value: i32) { self.insert(value); }insert99,3518
    fn delete(&mut self, value: &i32) { self.delete(value); }delete101,3580
    fn size(&self) -> usize { self.size() }size103,3643
    fn is_empty(&self) -> B { self.is_empty() }is_empty105,3688
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains107,3737
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum109,3804
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum111,3861
    fn union(&self, other: &Self) -> Self { self.union(other) }union113,3918
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection115,3983
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference117,4062
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split119,4137
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair121,4212
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m123,4296
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter125,4393
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce127,4488
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq129,4588
impl TestSet for apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32> {BSTSetBBAlphaMt132,4662
    fn empty() -> Self { Self::empty() }empty133,4759
    fn insert(&mut self, value: i32) { self.insert(value); }insert135,4801
    fn delete(&mut self, value: &i32) { self.delete(value); }delete137,4863
    fn size(&self) -> usize { self.size() }size139,4926
    fn is_empty(&self) -> B { self.is_empty() }is_empty141,4971
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains143,5020
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum145,5087
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum147,5144
    fn union(&self, other: &Self) -> Self { self.union(other) }union149,5201
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection151,5266
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference153,5345
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split155,5420
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair157,5495
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m159,5579
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter161,5676
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce163,5771
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq165,5871
impl TestSet for apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32> {BSTSetTreapMt168,5945
    fn empty() -> Self { Self::empty() }empty169,6028
    fn insert(&mut self, value: i32) { self.insert(value); }insert171,6070
    fn delete(&mut self, value: &i32) { self.delete(value); }delete173,6132
    fn size(&self) -> usize { self.size() }size175,6195
    fn is_empty(&self) -> B { self.is_empty() }is_empty177,6240
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains179,6289
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum181,6356
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum183,6413
    fn union(&self, other: &Self) -> Self { self.union(other) }union185,6470
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection187,6535
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference189,6614
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split191,6689
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair193,6764
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m195,6848
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter197,6945
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce199,7040
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq201,7140
impl TestSet for apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32> {BSTSetSplayMt204,7214
    fn empty() -> Self { Self::empty() }empty205,7305
    fn insert(&mut self, value: i32) { self.insert(value); }insert207,7347
    fn delete(&mut self, value: &i32) { self.delete(value); }delete209,7409
    fn size(&self) -> usize { self.size() }size211,7472
    fn is_empty(&self) -> B { self.is_empty() }is_empty213,7517
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains215,7566
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum217,7633
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum219,7690
    fn union(&self, other: &Self) -> Self { self.union(other) }union221,7747
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection223,7812
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference225,7891
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split227,7966
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair229,8041
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m231,8125
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter233,8222
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce235,8317
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq237,8417
fn exercise_set<S: TestSet>() {exercise_set240,8491
fn test_plain_bst_set_ops() {test_plain_bst_set_ops300,10249
fn test_avl_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTStest_avl_bst_set_ops305,10385
fn test_rb_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRtest_rb_bst_set_ops308,10509
fn test_bbalpha_bst_set_ops() {test_bbalpha_bst_set_ops311,10629
fn test_treap_bst_set_ops() { exercise_set::<apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSettest_treap_bst_set_ops316,10773
fn test_splay_bst_set_ops() {test_splay_bst_set_ops319,10897

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test18AVLTreeSeqStEph.rs,117
pub mod TestAVLTreeSeqEph {TestAVLTreeSeqEph1,0
    fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic7,250

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test47BSTBBAlphaStEph.rs,162
fn bbalpha_insert_find_balances() {bbalpha_insert_find_balances5,83
fn bbalpha_duplicate_insert_is_idempotent() {bbalpha_duplicate_insert_is_idempotent25,703

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test19AVLTreeSeqStEph18.rs,281
pub mod TestAVLTreeSeqStEph {TestAVLTreeSeqStEph3,79
    fn test_tabulate_inorder() {test_tabulate_inorder13,463
    fn test_map_increment() {test_map_increment19,661
    fn test_append_union() {test_append_union27,1008
    fn test_filter_even() {test_filter_even41,1504

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test46BSTRBStEph.rs,146
fn rb_insert_find_and_bounds() {rb_insert_find_and_bounds5,73
fn rb_duplicate_insert_is_idempotent() {rb_duplicate_insert_is_idempotent25,691

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test15AVLTreeSeqStPer.rs,215
pub mod TestAVLTreeSeqPer {TestAVLTreeSeqPer1,0
    fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate8,237
    fn test_iterator_inorder_values() {test_iterator_inorder_values17,567

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test45BSTAVLStEph.rs,150
fn avl_insert_find_and_bounds() {avl_insert_find_and_bounds5,75
fn avl_duplicate_insert_is_idempotent() {avl_duplicate_insert_is_idempotent27,782

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test49BSTSplayStEph.rs,144
fn splay_basic_behaviour() {splay_basic_behaviour5,79
fn splay_duplicate_insert_is_idempotent() {splay_duplicate_insert_is_idempotent24,651

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test20AVLTreeSeqStEph.rs,248
pub mod TestAVLTreeSeqStEph {TestAVLTreeSeqStEph3,80
    fn test_tabulate_and_map() {test_tabulate_and_map13,449
    fn test_select_and_append() {test_select_and_append21,789
    fn test_deflate_and_filter() {test_deflate_and_filter47,1777

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test50BSTMtEph.rs,309
fn mt_plain_basic_ops() {mt_plain_basic_ops10,335
fn mt_avl_basic_ops() {mt_avl_basic_ops23,676
fn mt_rb_basic_ops() {mt_rb_basic_ops34,943
fn mt_bbalpha_basic_ops() {mt_bbalpha_basic_ops44,1144
fn mt_treap_basic_ops() {mt_treap_basic_ops54,1357
fn mt_splay_basic_ops() {mt_splay_basic_ops64,1566

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test16AVLTreeSeqStPer18.rs,280
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer3,49
    fn test_tabulate_inorder() {test_tabulate_inorder13,417
    fn test_map_increment() {test_map_increment19,648
    fn test_append_union() {test_append_union26,996
    fn test_filter_even() {test_filter_even34,1456

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaMtEph.rs,783
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,101
fn make_range_tree(start: i32, end: i32) -> ParamBST<i32> {make_range_tree13,254
fn para_basic_insert_find() {para_basic_insert_find22,430
fn para_split_and_join_pair() {para_split_and_join_pair32,747
fn para_union_and_delete() {para_union_and_delete44,1163
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip58,1588
fn para_intersect_and_difference() {para_intersect_and_difference80,2254
fn para_filter_and_reduce() {para_filter_and_reduce92,2606
fn para_union_large_balanced() {para_union_large_balanced106,2972
fn para_intersect_and_difference_large() {para_intersect_and_difference_large117,3272
fn para_filter_and_reduce_edge_cases() {para_filter_and_reduce_edge_cases133,3875

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test17AVLTreeSeqStPer19.rs,371
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer1,0
    fn test_tabulate_and_map_ch19() {test_tabulate_and_map_ch199,297
    fn test_select_and_append_ch19() {test_select_and_append_ch1917,661
    fn test_deflate_and_filter_ch19() {test_deflate_and_filter_ch1938,1529
    fn test_iter_inorder_after_pipeline_ch19() {test_iter_inorder_after_pipeline_ch1953,2212

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaStEph.rs,323
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,101
fn para_basic_insert_find() {para_basic_insert_find14,262
fn para_split_and_join_pair() {para_split_and_join_pair24,579
fn para_union_and_delete() {para_union_and_delete36,995
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip50,1420

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test32Exercise_21_5_and_21_6.rs,389
fn all_contiguous_subseqs<T: StT>(a: &ArraySeqStPerS<T>) -> ArraySeqStPerS<ArraySeqStPerS<T>> {all_contiguous_subseqs13,466
fn test_all_contiguous_subseqs_n0() {test_all_contiguous_subseqs_n032,1232
fn test_all_contiguous_subseqs_n3_values() {test_all_contiguous_subseqs_n3_values39,1409
fn test_all_contiguous_subseqs_debug_shape() {test_all_contiguous_subseqs_debug_shape48,1748

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test36Problem_21_3.rs,492
fn points3d_loops(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {points3d_loops10,380
fn test_points3d_loops_n0_empty() {test_points3d_loops_n0_empty27,792
fn test_points3d_loops_n1_single() {test_points3d_loops_n1_single33,901
fn test_points3d_loops_n2_values_and_order() {test_points3d_loops_n2_values_and_order40,1065
fn test_points3d_loops_iterator_order() {test_points3d_loops_iterator_order57,1488
fn test_points3d_loops_debug_shape() {test_points3d_loops_debug_shape76,1976

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/TestExercise12_1.rs,348
fn spin_lock_excludes_parallel_threads() {spin_lock_excludes_parallel_threads8,183
fn spin_lock_with_lock_helper_executes_body() {spin_lock_with_lock_helper_executes_body36,1001
fn parallel_increment_counts_all_iterations() {parallel_increment_counts_all_iterations44,1234
fn spin_lock_is_non_reentrant() {spin_lock_is_non_reentrant49,1343

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test31Algorithm_21_6.rs,264
fn prime_sieve(n: N) -> ArraySeqStPerS<N> {prime_sieve13,511
fn test_prime_sieve_small() {test_prime_sieve_small61,2472
fn test_prime_sieve_n2_empty() {test_prime_sieve_n2_empty67,2593
fn test_prime_sieve_debug_shape() {test_prime_sieve_debug_shape73,2696

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap03/TestInsertionSortSt.rs,727
fn sort_and_assert(mut data: Vec<i32>, expected: &[i32]) {sort_and_assert3,97
fn insertion_sort_handles_empty() {insertion_sort_handles_empty9,250
fn insertion_sort_single_element() { sort_and_assert(vec![42], &[42]); }insertion_sort_single_element16,419
fn insertion_sort_already_sorted() { sort_and_assert(vec![1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]); }insertion_sort_already_sorted19,501
fn insertion_sort_reverse_order() { sort_and_assert(vec![5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]); }insertion_sort_reverse_order22,605
fn insertion_sort_with_duplicates() { sort_and_assert(vec![3, 1, 2, 3, 1], &[1, 1, 2, 3, 3]); }insertion_sort_with_duplicates25,708
fn insertion_sort_random_slice() {insertion_sort_random_slice28,813

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test23MappingStEph.rs,562
pub mod Test23MappingStEphChap5_5 {Test23MappingStEphChap5_53,55
    fn test_empty_mapping() {test_empty_mapping12,365
    fn test_from_vec_basic() {test_from_vec_basic20,579
    fn test_from_vec_duplicate_keys() {test_from_vec_duplicate_keys32,1086
    fn test_from_relation() {test_from_relation43,1644
    fn test_domain_and_range() {test_domain_and_range57,2413
    fn test_iter() {test_iter76,3128
    fn test_mem_comprehensive() {test_mem_comprehensive89,3612
    fn test_empty_mapping_operations() {test_empty_mapping_operations108,4281

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test22RelationStEph.rs,151
pub mod TestRelationStEphChap5_2 {TestRelationStEphChap5_21,0
    fn test_relation_domain_range_and_mem() {test_relation_domain_range_and_mem9,270

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test21SetStEph.rs,575
pub mod TestSetStEphChap5_1 {TestSetStEphChap5_11,0
    fn macro_typecheck_exercise() {macro_typecheck_exercise9,205
        let _empty: Set<&'static str> = SetLit![];str10,241
    fn test_cartesian_product_example_5_1() {test_cartesian_product_example_5_116,391
    fn test_partition_example_5_2_true() {test_partition_example_5_2_true36,961
    fn test_partition_example_5_2_false_due_to_overlap() {test_partition_example_5_2_false_due_to_overlap45,1258
    fn test_partition_false_due_to_missing_element() {test_partition_false_due_to_missing_element54,1603

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test24DirGraphStEph.rs,129
pub mod TestDirGraphStEph {TestDirGraphStEph1,0
    fn test_digraph_vertices_and_arcs() {test_digraph_vertices_and_arcs8,205

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test26LabDirGraphStEph.rs,1037
pub mod TestLabDirGraphStEph {TestLabDirGraphStEph1,0
    fn test_labelled_dir_graph_empty() {test_labelled_dir_graph_empty8,237
    fn test_labelled_dir_graph_add_vertex() {test_labelled_dir_graph_add_vertex16,528
    fn test_labelled_dir_graph_add_labeled_arc() {test_labelled_dir_graph_add_labeled_arc28,912
    fn test_labelled_dir_graph_neighbors() {test_labelled_dir_graph_neighbors45,1522
    fn test_labelled_dir_graph_arcs() {test_labelled_dir_graph_arcs69,2376
    fn test_labelled_dir_graph_macro_empty() {test_labelled_dir_graph_macro_empty81,2765
    fn test_labelled_dir_graph_macro_with_data() {test_labelled_dir_graph_macro_with_data88,2992
    fn test_labelled_dir_graph_different_label_types() {test_labelled_dir_graph_different_label_types103,3476
    fn test_labelled_dir_graph_display() {test_labelled_dir_graph_display120,3962
    fn test_labelled_dir_graph_debug() {test_labelled_dir_graph_debug132,4316
    fn test_labelled_dir_graph_self_loop() {test_labelled_dir_graph_self_loop144,4675

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test27LabUnDirGraphStEph.rs,1329
pub mod TestLabUnDirGraphStEph {TestLabUnDirGraphStEph1,0
    fn test_labelled_undir_graph_empty() {test_labelled_undir_graph_empty8,245
    fn test_labelled_undir_graph_add_vertex() {test_labelled_undir_graph_add_vertex16,545
    fn test_labelled_undir_graph_add_labeled_edge() {test_labelled_undir_graph_add_labeled_edge28,936
    fn test_labelled_undir_graph_neighbors() {test_labelled_undir_graph_neighbors49,1854
    fn test_labelled_undir_graph_edges() {test_labelled_undir_graph_edges72,2672
    fn test_labelled_undir_graph_macro_empty() {test_labelled_undir_graph_macro_empty86,3189
    fn test_labelled_undir_graph_macro_with_data() {test_labelled_undir_graph_macro_with_data93,3423
    fn test_labelled_undir_graph_edge_normalization() {test_labelled_undir_graph_edge_normalization110,4018
    fn test_labelled_undir_graph_different_label_types() {test_labelled_undir_graph_different_label_types124,4535
    fn test_labelled_undir_graph_display() {test_labelled_undir_graph_display143,5182
    fn test_labelled_undir_graph_debug() {test_labelled_undir_graph_debug155,5542
    fn test_labelled_undir_graph_self_loop() {test_labelled_undir_graph_self_loop167,5908
    fn test_labelled_undir_graph_multiple_edges_same_vertices() {test_labelled_undir_graph_multiple_edges_same_vertices180,6335

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test25UnDirGraphStEph.rs,139
pub mod TestUnDirGraphStEph {TestUnDirGraphStEph1,0
    fn test_undigraph_vertices_and_edges() {test_undigraph_vertices_and_edges8,211

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test08LinkedListStEph19.rs,614
pub mod TestLinkedListStEph {TestLinkedListStEph1,0
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list8,269
    fn test_eph_set_and_nth() {test_eph_set_and_nth17,530
    fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug24,713
    fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1934,1074
    fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1940,1244
    fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1953,1841

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test01ArraySeqMacro.rs,594
fn arrayseqs_empty_macro() {arrayseqs_empty_macro6,132
fn arrayseqs_literal_macro_keeps_order() {arrayseqs_literal_macro_keeps_order13,320
fn arrayseqs_repeat_macro_clones_element() {arrayseqs_repeat_macro_clones_element21,564
fn arrayseq_tabulate_and_map_work() {arrayseq_tabulate_and_map_work31,862
fn arrayseq_subseq_append_filter_flatten() {arrayseq_subseq_append_filter_flatten42,1263
fn arrayseq_update_and_inject_preserve_original() {arrayseq_update_and_inject_preserve_original68,2262
fn arrayseq_collect_iterate_reduce_scan() {arrayseq_collect_iterate_reduce_scan83,2854

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test12ArraySeqStEphMacro.rs,141
fn arrayseq_steph_basic_macros() {arrayseq_steph_basic_macros6,147
fn arrayseq_steph_full_pipeline() {arrayseq_steph_full_pipeline15,490

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test10ArraySeqStPerChap18.rs,945
pub mod TestArraySeqStPerChap {TestArraySeqStPerChap1,0
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,355
        fn fib(n: N) -> N {fib12,390
    fn test_map_increment() {test_map_increment37,1059
    fn test_subseq() {test_subseq44,1309
    fn test_append() {test_append55,1655
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1936
    fn test_filter_even() {test_filter_even76,2595
    fn test_flatten() {test_flatten91,3304
    fn test_update_sequence() {test_update_sequence105,4134
    fn test_inject_and_ninject() {test_inject_and_ninject115,4661
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan140,6056
    fn test_iterate_sum_basic() {test_iterate_sum_basic159,7054
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum167,7332
    fn test_collect_groups_by_key() {test_collect_groups_by_key179,7799

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test13ArraySeqStEphChap18.rs,1048
pub mod TestArraySeqStEphChap {TestArraySeqStEphChap3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,368
        fn fib(n: N) -> N {fib12,403
    fn test_map_increment() {test_map_increment37,1089
    fn test_subseq() {test_subseq44,1335
    fn test_append() {test_append55,1825
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,2102
    fn test_filter_even() {test_filter_even76,2787
    fn test_flatten() {test_flatten90,3401
    fn test_update_sequence() {test_update_sequence104,4134
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins116,4649
    fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins129,5419
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan145,6183
    fn test_iterate_sum_basic() {test_iterate_sum_basic164,7211
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum172,7489
    fn test_collect_groups_by_key() {test_collect_groups_by_key185,7968

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test05LinkedListStPer19.rs,427
pub mod TestLinkedListPer {TestLinkedListPer1,0
    fn test_select() {test_select9,279
    fn test_append_variants() {test_append_variants22,779
    fn test_deflate() {test_deflate32,1208
    fn test_map() {test_map42,1633
    fn test_iterate_and_reduce() {test_iterate_and_reduce49,1859
    fn test_scan() {test_scan58,2216
    fn test_flatten() {test_flatten66,2504
    fn test_inject() {test_inject73,2787

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test02MathSeq.rs,1347
pub mod TestMathSeq {TestMathSeq3,93
    fn test_length_and_nth_basic() {test_length_and_nth_basic10,256
    fn test_add_last_and_delete_last() {test_add_last_and_delete_last18,476
    fn test_new_empty_singleton_and_predicates() {test_new_empty_singleton_and_predicates31,925
    fn test_set_in_bounds_and_out_of_bounds() {test_set_in_bounds_and_out_of_bounds50,1523
    fn test_subseq_view_bounds() {test_subseq_view_bounds61,1882
    fn test_subseq_copy_bounds() {test_subseq_copy_bounds74,2279
    fn test_domain() {test_domain84,2575
    fn test_range_deduplicates_preserving_order() {test_range_deduplicates_preserving_order90,2716
    fn test_debug_format_for_mathseq() {test_debug_format_for_mathseq97,2944
    fn test_display_format_for_mathseq() {test_display_format_for_mathseq103,3109
    fn test_multiset_range_counts_first_occurrence_order() {test_multiset_range_counts_first_occurrence_order109,3274
    fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics117,3524
    fn test_range_empty_returns_empty() {test_range_empty_returns_empty123,3654
    fn test_multiset_range_empty_returns_empty() {test_multiset_range_empty_returns_empty130,3827
    fn test_domain_empty_is_empty() {test_domain_empty_is_empty137,4023
    fn test_iter_collect_and_sum() {test_iter_collect_and_sum144,4185

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test04LinkedListStPer.rs,569
pub mod TestLinkedListStPer {TestLinkedListStPer1,0
    fn test_tabulate() {test_tabulate9,281
    fn test_map() {test_map16,484
    fn test_filter() {test_filter24,767
    fn test_append() {test_append37,1180
    fn test_update() {test_update45,1485
    fn test_inject() {test_inject52,1734
    fn test_ninject() {test_ninject60,2049
    fn test_iterate() {test_iterate68,2368
    fn test_reduce() {test_reduce75,2594
    fn test_scan() {test_scan82,2815
    fn test_flatten() {test_flatten90,3115
    fn test_collect() {test_collect101,3479

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test27ArraySeqMtPerChap18.rs,593
pub mod Test27ArraySeqMtPerChap {Test27ArraySeqMtPerChap3,61
    fn test_tabulate_basic() {test_tabulate_basic10,302
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci19,587
        fn fib(n: N) -> N {fib20,622
    fn test_tabulate_empty() {test_tabulate_empty51,1446
    fn test_tabulate_single() {test_tabulate_single58,1654
    fn test_tabulate_string() {test_tabulate_string65,1856
    fn test_tabulate_boolean() {test_tabulate_boolean80,2369
    fn test_tabulate_squares() {test_tabulate_squares95,2873
    fn test_tabulate_large() {test_tabulate_large110,3283

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test07LinkedListStEph.rs,927
pub mod TestLinkedListStEph {TestLinkedListStEph1,0
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list8,269
    fn test_construct_eph_from_vec() {test_construct_eph_from_vec19,638
    fn test_eph_is_empty_and_singleton() {test_eph_is_empty_and_singleton25,799
    fn test_eph_set_and_subseq_copy() {test_eph_set_and_subseq_copy33,1086
    fn test_iter_inorder_collect_eph_ch18() {test_iter_inorder_collect_eph_ch1842,1353
    fn test_tabulate_and_map_ch18() {test_tabulate_and_map_ch1848,1523
    fn test_append_ch18() {test_append_ch1855,1837
    fn test_filter_ch18() {test_filter_ch1863,2157
    fn test_update_ch18() {test_update_ch1876,2536
    fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1883,2767
    fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1893,3262
    fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch18105,3832

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test11ArraySeqStPerMacro.rs,275
fn arrayseq_stper_macro_empty() {arrayseq_stper_macro_empty6,172
fn arrayseq_stper_macro_literal() {arrayseq_stper_macro_literal13,387
fn arrayseq_stper_macro_repeat() {arrayseq_stper_macro_repeat20,595
fn arrayseq_stper_operations() {arrayseq_stper_operations29,951

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test12ArraySeqStEph.rs,283
pub mod TestArraySeqEph {TestArraySeqEph1,0
    fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic11,308
    fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter20,574
    fn test_iterators_collect() {test_iterators_collect31,1097

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test14ArraySeqStEph.rs,1419
pub mod TestArraySeqStEph {TestArraySeqStEph3,93
    fn test_empty() {test_empty9,320
    fn test_singleton() {test_singleton15,456
    fn test_map() {test_map21,614
    fn test_append() {test_append28,858
    fn test_append2() {test_append236,1175
    fn test_deflate_true() {test_deflate_true44,1494
    fn test_deflate_false() {test_deflate_false50,1706
    fn test_filter_even_numbers() {test_filter_even_numbers56,1908
    fn test_filter_none() {test_filter_none63,2219
    fn test_update_in_bounds() {test_update_in_bounds70,2515
    fn test_update_out_of_bounds() {test_update_out_of_bounds77,2751
    fn test_isEmpty() {test_isEmpty84,2984
    fn test_isSingleton() {test_isSingleton94,3351
    fn test_iterate_sum() {test_iterate_sum104,3734
    fn test_iterate_concat() {test_iterate_concat111,3964
    fn test_map_empty() {test_map_empty125,4381
    fn test_append_with_empty() {test_append_with_empty132,4603
    fn test_append2_equivalence() {test_append2_equivalence142,5054
    fn test_filter_empty_sequence() {test_filter_empty_sequence151,5424
    fn test_select_boundary() {test_select_boundary158,5663
    fn test_subseq_basic() {test_subseq_basic169,6181
    fn test_reduce_sum_basic_ch19() {test_reduce_sum_basic_ch19176,6403
    fn test_scan_sum_basic_ch19() {test_scan_sum_basic_ch19190,6982
    fn test_flatten_ch19() {test_flatten_ch19201,7381

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test13ArraySeqStEph18.rs,1040
pub mod TestArraySeqStEph {TestArraySeqStEph3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,352
        fn fib(n: N) -> N {fib12,387
    fn test_map_increment() {test_map_increment37,1067
    fn test_subseq() {test_subseq44,1307
    fn test_append() {test_append55,1797
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,2068
    fn test_filter_even() {test_filter_even76,2735
    fn test_flatten() {test_flatten94,3412
    fn test_update_sequence() {test_update_sequence108,4133
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins120,4648
    fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins133,5406
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan149,6158
    fn test_iterate_sum_basic() {test_iterate_sum_basic168,7162
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum176,7434
    fn test_collect_groups_by_key() {test_collect_groups_by_key188,7895

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test10ArraySeqStPer18.rs,937
pub mod TestArraySeqStPer {TestArraySeqStPer1,0
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,333
        fn fib(n: N) -> N {fib12,368
    fn test_map_increment() {test_map_increment37,1037
    fn test_subseq() {test_subseq44,1281
    fn test_append() {test_append55,1627
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1902
    fn test_filter_even() {test_filter_even76,2543
    fn test_flatten() {test_flatten95,3315
    fn test_update_sequence() {test_update_sequence110,4145
    fn test_inject_and_ninject() {test_inject_and_ninject120,4660
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan145,6031
    fn test_iterate_sum_basic() {test_iterate_sum_basic164,7005
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum172,7277
    fn test_collect_groups_by_key() {test_collect_groups_by_key184,7738

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test28ArraySeqMtPer.rs,573
pub mod Test28ArraySeqMtPer {Test28ArraySeqMtPer3,55
    fn test_inject_basic() {test_inject_basic13,365
    fn test_inject_conflicting_updates() {test_inject_conflicting_updates29,972
    fn test_inject_out_of_bounds() {test_inject_out_of_bounds45,1638
    fn test_inject_empty_changes() {test_inject_empty_changes57,2148
    fn test_inject_empty_values() {test_inject_empty_values69,2566
    fn test_atomic_write_migrated_from_st_test() {test_atomic_write_migrated_from_st_test83,3204
    fn test_inject_string_values() {test_inject_string_values108,4400

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test41ArraySeqMtEph.rs,156
fn test_arrayseq_mteph_basic_ops() {test_arrayseq_mteph_basic_ops5,57
fn test_arrayseq_mteph_append_and_map() {test_arrayseq_mteph_append_and_map22,518

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test26ArraySeqMtPer.rs,910
pub mod Test26ArraySeqMtPer {Test26ArraySeqMtPer3,93
    fn test_new_and_set() {test_new_and_set9,286
    fn test_length_and_nth_basic() {test_length_and_nth_basic23,733
    fn test_empty() {test_empty31,943
    fn test_sequence_basic() {test_sequence_basic38,1124
    fn test_singleton() {test_singleton51,1648
    fn test_from_vec() {test_from_vec59,1866
    fn test_subseq_copy() {test_subseq_copy67,2061
    fn test_subseq_view() {test_subseq_view77,2348
    fn test_iterators() {test_iterators87,2617
    fn test_set_out_of_bounds() {test_set_out_of_bounds100,2986
    fn test_macro_literals() {test_macro_literals107,3165
    fn test_equality_and_debug() {test_equality_and_debug127,3768
    fn test_display_format() {test_display_format143,4208
    fn test_string_sequences() {test_string_sequences152,4476
    fn test_boolean_sequences() {test_boolean_sequences160,4694

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test11ArraySeqStPer.rs,346
pub mod TestArraySeqPer {TestArraySeqPer1,0
    fn test_map_and_select_and_append() {test_map_and_select_and_append11,307
    fn test_deflate_and_filter() {test_deflate_and_filter24,856
    fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten37,1410
    fn test_inject_and_parallel() {test_inject_and_parallel56,2415

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test27ArraySeqMtPer18.rs,585
pub mod Test27ArraySeqMtPer {Test27ArraySeqMtPer3,55
    fn test_tabulate_basic() {test_tabulate_basic10,280
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci19,565
        fn fib(n: N) -> N {fib20,600
    fn test_tabulate_empty() {test_tabulate_empty51,1424
    fn test_tabulate_single() {test_tabulate_single58,1632
    fn test_tabulate_string() {test_tabulate_string65,1834
    fn test_tabulate_boolean() {test_tabulate_boolean80,2347
    fn test_tabulate_squares() {test_tabulate_squares95,2851
    fn test_tabulate_large() {test_tabulate_large110,3261

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test09ArraySeqStPer.rs,2133
pub mod TestArraySeqStPer {TestArraySeqStPer3,93
    fn test_new_and_set() {test_new_and_set9,287
    fn test_length_and_nth_basic() {test_length_and_nth_basic23,747
    fn test_empty() {test_empty31,960
    fn test_sequence_basic() {test_sequence_basic38,1150
    fn test_singleton() {test_singleton51,1861
    fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton59,2069
    fn test_from_vec() {test_from_vec74,2548
    fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers89,3223
    fn test_sequence_equality_strings() {test_sequence_equality_strings114,4165
    fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference139,5201
        struct PartialComparable {PartialComparable141,5292
            value: i32,value142,5327
        impl std::fmt::Display for PartialComparable {PartialComparable145,5413
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt146,5468
        struct TotalComparable {TotalComparable158,6112
            value: N,value159,6145
        impl std::fmt::Display for TotalComparable {TotalComparable162,6178
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt163,6231
    fn test_ordering_numbers_basic() {test_ordering_numbers_basic178,6896
    fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal187,7143
    fn test_ordering_strings_basic() {test_ordering_strings_basic193,7271
    fn test_strings_equal_is_equal() {test_strings_equal_is_equal202,7516
    fn test_nth_on_empty_panics() {test_nth_on_empty_panics209,7665
    fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics216,7818
    fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap223,7959
    fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes229,8128
    fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic238,8371
    fn test_new_set_persistent() {test_new_set_persistent247,8730
    fn test_iterator_collects_in_order() {test_iterator_collects_in_order256,9005

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap23/TestPrimTreeSeqSt.rs,494
pub mod TestPrimTreeSeqSt {TestPrimTreeSeqSt1,0
    fn expose_zero_returns_zero() {expose_zero_returns_zero6,133
    fn expose_one_returns_one() {expose_one_returns_one12,316
    fn expose_two_splits_sequence() {expose_two_splits_sequence21,593
    fn join_zero_creates_empty_sequence() {join_zero_creates_empty_sequence36,1155
    fn join_two_concatenates_sequences() {join_two_concatenates_sequences42,1339
    fn expose_then_join_roundtrip() {expose_then_join_roundtrip51,1741

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test51BSTSetMtEph.rs,11026
trait TestSet: Sized {TestSet4,49
    fn empty() -> Self;empty5,72
    fn insert(&mut self, value: i32);insert6,96
    fn delete(&mut self, value: &i32);delete7,134
    fn size(&self) -> usize;size8,173
    fn is_empty(&self) -> B;is_empty9,202
    fn contains(&self, value: &i32) -> B;contains10,231
    fn minimum(&self) -> Option<i32>;minimum11,273
    fn maximum(&self) -> Option<i32>;maximum12,311
    fn union(&self, other: &Self) -> Self;union13,349
    fn intersection(&self, other: &Self) -> Self;intersection14,392
    fn difference(&self, other: &Self) -> Self;difference15,442
    fn split(&self, pivot: &i32) -> (Self, B, Self);split16,490
    fn join_pair(left: Self, right: Self) -> Self;join_pair17,543
    fn join_m(left: Self, pivot: i32, right: Self) -> Self;join_m18,594
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self;filter19,654
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32;reduce20,722
    fn iter_seq(&self) -> ArraySeqStPerS<i32>;iter_seq21,796
impl TestSet for apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32> {BSTSetPlainMt24,846
    fn empty() -> Self { Self::empty() }empty25,937
    fn insert(&mut self, value: i32) { self.insert(value); }insert27,979
    fn delete(&mut self, value: &i32) { self.delete(value); }delete29,1041
    fn size(&self) -> usize { self.size() }size31,1104
    fn is_empty(&self) -> B { self.is_empty() }is_empty33,1149
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains35,1198
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum37,1265
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum39,1322
    fn union(&self, other: &Self) -> Self { self.union(other) }union41,1379
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection43,1444
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference45,1523
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split47,1598
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair49,1673
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m51,1757
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter53,1854
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce55,1949
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq57,2049
impl TestSet for apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32> {BSTSetAVLMt60,2123
    fn empty() -> Self { Self::empty() }empty61,2208
    fn insert(&mut self, value: i32) { self.insert(value); }insert63,2250
    fn delete(&mut self, value: &i32) { self.delete(value); }delete65,2312
    fn size(&self) -> usize { self.size() }size67,2375
    fn is_empty(&self) -> B { self.is_empty() }is_empty69,2420
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains71,2469
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum73,2536
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum75,2593
    fn union(&self, other: &Self) -> Self { self.union(other) }union77,2650
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection79,2715
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference81,2794
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split83,2869
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair85,2944
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m87,3028
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter89,3125
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce91,3220
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq93,3320
impl TestSet for apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32> {BSTSetRBMt96,3394
    fn empty() -> Self { Self::empty() }empty97,3476
    fn insert(&mut self, value: i32) { self.insert(value); }insert99,3518
    fn delete(&mut self, value: &i32) { self.delete(value); }delete101,3580
    fn size(&self) -> usize { self.size() }size103,3643
    fn is_empty(&self) -> B { self.is_empty() }is_empty105,3688
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains107,3737
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum109,3804
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum111,3861
    fn union(&self, other: &Self) -> Self { self.union(other) }union113,3918
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection115,3983
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference117,4062
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split119,4137
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair121,4212
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m123,4296
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter125,4393
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce127,4488
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq129,4588
impl TestSet for apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32> {BSTSetBBAlphaMt132,4662
    fn empty() -> Self { Self::empty() }empty133,4759
    fn insert(&mut self, value: i32) { self.insert(value); }insert135,4801
    fn delete(&mut self, value: &i32) { self.delete(value); }delete137,4863
    fn size(&self) -> usize { self.size() }size139,4926
    fn is_empty(&self) -> B { self.is_empty() }is_empty141,4971
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains143,5020
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum145,5087
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum147,5144
    fn union(&self, other: &Self) -> Self { self.union(other) }union149,5201
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection151,5266
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference153,5345
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split155,5420
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair157,5495
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m159,5579
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter161,5676
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce163,5771
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq165,5871
impl TestSet for apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32> {BSTSetTreapMt168,5945
    fn empty() -> Self { Self::empty() }empty169,6028
    fn insert(&mut self, value: i32) { self.insert(value); }insert171,6070
    fn delete(&mut self, value: &i32) { self.delete(value); }delete173,6132
    fn size(&self) -> usize { self.size() }size175,6195
    fn is_empty(&self) -> B { self.is_empty() }is_empty177,6240
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains179,6289
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum181,6356
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum183,6413
    fn union(&self, other: &Self) -> Self { self.union(other) }union185,6470
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection187,6535
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference189,6614
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split191,6689
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair193,6764
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m195,6848
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter197,6945
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce199,7040
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq201,7140
impl TestSet for apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32> {BSTSetSplayMt204,7214
    fn empty() -> Self { Self::empty() }empty205,7305
    fn insert(&mut self, value: i32) { self.insert(value); }insert207,7347
    fn delete(&mut self, value: &i32) { self.delete(value); }delete209,7409
    fn size(&self) -> usize { self.size() }size211,7472
    fn is_empty(&self) -> B { self.is_empty() }is_empty213,7517
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains215,7566
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum217,7633
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum219,7690
    fn union(&self, other: &Self) -> Self { self.union(other) }union221,7747
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection223,7812
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference225,7891
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split227,7966
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair229,8041
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m231,8125
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter233,8222
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce235,8317
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq237,8417
fn exercise_set<S: TestSet>() {exercise_set240,8491
fn test_plain_bst_set_ops() {test_plain_bst_set_ops300,10249
fn test_avl_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTStest_avl_bst_set_ops305,10385
fn test_rb_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRtest_rb_bst_set_ops308,10509
fn test_bbalpha_bst_set_ops() {test_bbalpha_bst_set_ops311,10629
fn test_treap_bst_set_ops() { exercise_set::<apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSettest_treap_bst_set_ops316,10773
fn test_splay_bst_set_ops() {test_splay_bst_set_ops319,10897

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test18AVLTreeSeqStEph.rs,117
pub mod TestAVLTreeSeqEph {TestAVLTreeSeqEph1,0
    fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic7,250

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test47BSTBBAlphaStEph.rs,162
fn bbalpha_insert_find_balances() {bbalpha_insert_find_balances5,83
fn bbalpha_duplicate_insert_is_idempotent() {bbalpha_duplicate_insert_is_idempotent25,703

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test19AVLTreeSeqStEph18.rs,281
pub mod TestAVLTreeSeqStEph {TestAVLTreeSeqStEph3,79
    fn test_tabulate_inorder() {test_tabulate_inorder13,463
    fn test_map_increment() {test_map_increment19,661
    fn test_append_union() {test_append_union27,1008
    fn test_filter_even() {test_filter_even41,1504

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test46BSTRBStEph.rs,146
fn rb_insert_find_and_bounds() {rb_insert_find_and_bounds5,73
fn rb_duplicate_insert_is_idempotent() {rb_duplicate_insert_is_idempotent25,691

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test15AVLTreeSeqStPer.rs,215
pub mod TestAVLTreeSeqPer {TestAVLTreeSeqPer1,0
    fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate8,237
    fn test_iterator_inorder_values() {test_iterator_inorder_values17,567

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test45BSTAVLStEph.rs,150
fn avl_insert_find_and_bounds() {avl_insert_find_and_bounds5,75
fn avl_duplicate_insert_is_idempotent() {avl_duplicate_insert_is_idempotent27,782

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test49BSTSplayStEph.rs,144
fn splay_basic_behaviour() {splay_basic_behaviour5,79
fn splay_duplicate_insert_is_idempotent() {splay_duplicate_insert_is_idempotent24,651

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test20AVLTreeSeqStEph.rs,248
pub mod TestAVLTreeSeqStEph {TestAVLTreeSeqStEph3,80
    fn test_tabulate_and_map() {test_tabulate_and_map13,449
    fn test_select_and_append() {test_select_and_append21,789
    fn test_deflate_and_filter() {test_deflate_and_filter47,1777

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test50BSTMtEph.rs,309
fn mt_plain_basic_ops() {mt_plain_basic_ops10,335
fn mt_avl_basic_ops() {mt_avl_basic_ops23,676
fn mt_rb_basic_ops() {mt_rb_basic_ops34,943
fn mt_bbalpha_basic_ops() {mt_bbalpha_basic_ops44,1144
fn mt_treap_basic_ops() {mt_treap_basic_ops54,1357
fn mt_splay_basic_ops() {mt_splay_basic_ops64,1566

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test16AVLTreeSeqStPer18.rs,280
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer3,49
    fn test_tabulate_inorder() {test_tabulate_inorder13,417
    fn test_map_increment() {test_map_increment19,648
    fn test_append_union() {test_append_union26,996
    fn test_filter_even() {test_filter_even34,1456

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaMtEph.rs,783
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,101
fn make_range_tree(start: i32, end: i32) -> ParamBST<i32> {make_range_tree13,254
fn para_basic_insert_find() {para_basic_insert_find22,430
fn para_split_and_join_pair() {para_split_and_join_pair32,747
fn para_union_and_delete() {para_union_and_delete44,1163
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip58,1588
fn para_intersect_and_difference() {para_intersect_and_difference80,2254
fn para_filter_and_reduce() {para_filter_and_reduce92,2606
fn para_union_large_balanced() {para_union_large_balanced106,2972
fn para_intersect_and_difference_large() {para_intersect_and_difference_large117,3272
fn para_filter_and_reduce_edge_cases() {para_filter_and_reduce_edge_cases133,3875

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test17AVLTreeSeqStPer19.rs,371
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer1,0
    fn test_tabulate_and_map_ch19() {test_tabulate_and_map_ch199,297
    fn test_select_and_append_ch19() {test_select_and_append_ch1917,661
    fn test_deflate_and_filter_ch19() {test_deflate_and_filter_ch1938,1529
    fn test_iter_inorder_after_pipeline_ch19() {test_iter_inorder_after_pipeline_ch1953,2212

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaStEph.rs,323
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,101
fn para_basic_insert_find() {para_basic_insert_find14,262
fn para_split_and_join_pair() {para_split_and_join_pair24,579
fn para_union_and_delete() {para_union_and_delete36,995
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip50,1420

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36Mt.rs,131
fn gen_data(n: usize) -> ArraySeqMtEphS<i32> {gen_data7,206
fn bench_quicksort_mt(c: &mut Criterion) {bench_quicksort_mt17,541

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph.rs,86
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch199,311

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch188,265

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch198,265

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer18.rs,90
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch187,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map9,311

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer.rs,90
fn bench_tabulate_map_mtper_ch19(c: &mut Criterion) {bench_tabulate_map_mtper_ch197,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchDirGraphStEph.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build8,270

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabUnDirGraphStEph.rs,980
fn bench_labelled_undir_graph_creation(c: &mut Criterion) {bench_labelled_undir_graph_creation7,269
fn bench_labelled_undir_graph_add_vertex(c: &mut Criterion) {bench_labelled_undir_graph_add_vertex34,1213
fn bench_labelled_undir_graph_add_labeled_edge(c: &mut Criterion) {bench_labelled_undir_graph_add_labeled_edge52,1757
fn bench_labelled_undir_graph_has_edge(c: &mut Criterion) {bench_labelled_undir_graph_has_edge70,2371
fn bench_labelled_undir_graph_get_edge_label(c: &mut Criterion) {bench_labelled_undir_graph_get_edge_label94,3135
fn bench_labelled_undir_graph_neighbors(c: &mut Criterion) {bench_labelled_undir_graph_neighbors120,4020
fn bench_labelled_undir_graph_edges(c: &mut Criterion) {bench_labelled_undir_graph_edges150,5098
fn bench_labelled_undir_graph_macro(c: &mut Criterion) {bench_labelled_undir_graph_macro168,5646
fn bench_labelled_undir_graph_edge_normalization(c: &mut Criterion) {bench_labelled_undir_graph_edge_normalization190,6250

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabDirGraphStEph.rs,932
fn bench_labelled_dir_graph_creation(c: &mut Criterion) {bench_labelled_dir_graph_creation7,263
fn bench_labelled_dir_graph_add_vertex(c: &mut Criterion) {bench_labelled_dir_graph_add_vertex34,1195
fn bench_labelled_dir_graph_add_labeled_arc(c: &mut Criterion) {bench_labelled_dir_graph_add_labeled_arc52,1733
fn bench_labelled_dir_graph_has_arc(c: &mut Criterion) {bench_labelled_dir_graph_has_arc70,2337
fn bench_labelled_dir_graph_get_arc_label(c: &mut Criterion) {bench_labelled_dir_graph_get_arc_label94,3089
fn bench_labelled_dir_graph_out_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_out_neighbors120,3962
fn bench_labelled_dir_graph_in_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_in_neighbors147,4890
fn bench_labelled_dir_graph_arcs(c: &mut Criterion) {bench_labelled_dir_graph_arcs174,5814
fn bench_labelled_dir_graph_macro(c: &mut Criterion) {bench_labelled_dir_graph_macro192,6350

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchUnDirGraphStEph.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build8,274

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPer.rs,80
fn bench_sll_persistent_ops(c: &mut Criterion) {bench_sll_persistent_ops9,263

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTTreapStEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree7,215
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap15,412

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTTreapMtEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree7,215
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap15,410

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap03/BenchInsertionSortSt.rs,161
fn build_vec(len: usize) -> Vec<i32> { (0..len as i32).rev().collect() }build_vec6,207
fn bench_insertion_sort(c: &mut Criterion) {bench_insertion_sort8,281

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36St.rs,131
fn gen_data(n: usize) -> ArraySeqStEphS<i32> {gen_data7,206
fn bench_quicksort_st(c: &mut Criterion) {bench_quicksort_st17,538

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter26Mt.rs,165
fn gen_sequence(n: usize) -> ArraySeqMtPerS<usize> { ArrayMtPerS::new(n, 0) }gen_sequence7,220
fn bench_chapter26_mt(c: &mut Criterion) {bench_chapter26_mt9,299

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchMappingStEph.rs,70
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build9,323

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchRelationStEph.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range8,271

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchFibonacciMt.rs,68
fn bench_fibonacci_mt(c: &mut Criterion) {bench_fibonacci_mt6,192

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch197,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchMathSeq.rs,72
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics9,239

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer.rs,66
fn bench_ll_per_ch18(c: &mut Criterion) {bench_ll_per_ch187,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch188,277

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqMtPerChap18.rs,90
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch187,245

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStEphChap18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map9,317

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer19.rs,66
fn bench_ll_per_ch19(c: &mut Criterion) {bench_ll_per_ch198,291

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph.rs,66
fn bench_ll_eph_ch18(c: &mut Criterion) {bench_ll_eph_ch187,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTParaTreapMtEph.rs,129
fn build_tree(len: usize) -> ParamTreap<i32> {build_tree7,202
fn bench_para_treap(c: &mut Criterion) {bench_para_treap15,362

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTAVLStEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree7,215
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl15,408

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSplayMtEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree7,223
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay15,418

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTBBAlphaMtEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree7,231
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha15,430

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTAVLMtEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree7,215
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl15,406

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph.rs,58
fn bench_avl_eph(c: &mut Criterion) {bench_avl_eph7,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTRBMtEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree7,211
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb15,400

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSetMtEph.rs,5094
trait BenchSet: Sized {BenchSet15,756
    fn empty() -> Self;empty16,780
    fn insert_value(&mut self, value: i32);insert_value17,804
    fn union_with(&self, other: &Self) -> Self;union_with18,848
    fn difference_with(&self, other: &Self) -> Self;difference_with19,896
    fn filter_divisible_by(&self, divisor: i32) -> Self;filter_divisible_by20,949
    fn reduce_sum(&self) -> i32;reduce_sum21,1006
fn build_pair<S: BenchSet>(len: usize) -> (S, S) {build_pair24,1042
fn build_single<S: BenchSet>(len: usize) -> S {build_single37,1321
fn bench_set_variants<S: BenchSet>(c: &mut Criterion, label: &str) {bench_set_variants45,1482
impl BenchSet for PlainSet<i32> {PlainSet88,2945
    fn empty() -> Self { BSTSetPlainMtEphLit![] }empty89,2979
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value91,3030
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with93,3098
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with95,3168
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by97,3248
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum99,3352
impl BenchSet for AVLSet<i32> {AVLSet102,3432
    fn empty() -> Self { BSTSetAVLMtEphLit![] }empty103,3464
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value105,3513
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with107,3581
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with109,3651
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by111,3731
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum113,3835
impl BenchSet for RBSet<i32> {RBSet116,3915
    fn empty() -> Self { BSTSetRBMtEphLit![] }empty117,3946
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value119,3994
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with121,4062
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with123,4132
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by125,4212
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum127,4316
impl BenchSet for BBAlphaSet<i32> {BBAlphaSet130,4396
    fn empty() -> Self { BSTSetBBAlphaMtEphLit![] }empty131,4432
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value133,4485
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with135,4553
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with137,4623
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by139,4703
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum141,4807
impl BenchSet for TreapSet<i32> {TreapSet144,4887
    fn empty() -> Self { BSTSetTreapMtEphLit![] }empty145,4921
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value147,4972
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with149,5040
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with151,5110
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by153,5190
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum155,5294
impl BenchSet for SplaySet<i32> {SplaySet158,5374
    fn empty() -> Self { BSTSetSplayMtEphLit![] }empty159,5408
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value161,5459
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with163,5527
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with165,5597
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by167,5677
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum169,5781
fn bench_plain_set(c: &mut Criterion) { bench_set_variants::<PlainSet<i32>>(c, "BSTSetPlainMtEphbench_plain_set172,5861
fn bench_avl_set(c: &mut Criterion) { bench_set_variants::<AVLSet<i32>>(c, "BSTSetAVLMtEph"); }bench_avl_set174,5964
fn bench_rb_set(c: &mut Criterion) { bench_set_variants::<RBSet<i32>>(c, "BSTSetRBMtEph"); }bench_rb_set176,6061
fn bench_bbalpha_set(c: &mut Criterion) { bench_set_variants::<BBAlphaSet<i32>>(c, "BSTSetBBAlphbench_bbalpha_set178,6155
fn bench_treap_set(c: &mut Criterion) { bench_set_variants::<TreapSet<i32>>(c, "BSTSetTreapMtEphbench_treap_set180,6264
fn bench_splay_set(c: &mut Criterion) { bench_set_variants::<SplaySet<i32>>(c, "BSTSetSplayMtEphbench_splay_set182,6367

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTParaStEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree7,213
fn bench_para_bst(c: &mut Criterion) {bench_para_bst15,400

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTRBStEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree7,211
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb15,402

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph19.rs,68
fn bench_avl_eph_ch19(c: &mut Criterion) {bench_avl_eph_ch197,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTParaMtEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree7,198
fn bench_para_bst(c: &mut Criterion) {bench_para_bst15,354

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer.rs,80
fn bench_build_and_contains(c: &mut Criterion) {bench_build_and_contains9,272

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTPlainMtEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree7,218
fn bench_bsteph(c: &mut Criterion) {bench_bsteph17,454

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTBBAlphaStEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree7,231
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha15,432

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer19.rs,95
fn bench_build_and_read_persistent(c: &mut Criterion) {bench_build_and_read_persistent10,345

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer18.rs,68
fn bench_avl_per_ch18(c: &mut Criterion) {bench_avl_per_ch187,234

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTPlainStEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree7,218
fn bench_bsteph(c: &mut Criterion) {bench_bsteph17,456

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSplayStEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree7,223
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay15,420

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph18.rs,68
fn bench_avl_eph_ch18(c: &mut Criterion) {bench_avl_eph_ch187,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEph.rs,56
fn bench_ll_eph(c: &mut Criterion) {bench_ll_eph7,225

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap03/BenchInsertionSortSt.rs,161
fn build_vec(len: usize) -> Vec<i32> { (0..len as i32).rev().collect() }build_vec6,207
fn bench_insertion_sort(c: &mut Criterion) {bench_insertion_sort8,281

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchMappingStEph.rs,70
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build9,323

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchRelationStEph.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range8,271

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchDirGraphStEph.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build8,270

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabUnDirGraphStEph.rs,980
fn bench_labelled_undir_graph_creation(c: &mut Criterion) {bench_labelled_undir_graph_creation7,269
fn bench_labelled_undir_graph_add_vertex(c: &mut Criterion) {bench_labelled_undir_graph_add_vertex34,1213
fn bench_labelled_undir_graph_add_labeled_edge(c: &mut Criterion) {bench_labelled_undir_graph_add_labeled_edge52,1757
fn bench_labelled_undir_graph_has_edge(c: &mut Criterion) {bench_labelled_undir_graph_has_edge70,2371
fn bench_labelled_undir_graph_get_edge_label(c: &mut Criterion) {bench_labelled_undir_graph_get_edge_label94,3135
fn bench_labelled_undir_graph_neighbors(c: &mut Criterion) {bench_labelled_undir_graph_neighbors120,4020
fn bench_labelled_undir_graph_edges(c: &mut Criterion) {bench_labelled_undir_graph_edges150,5098
fn bench_labelled_undir_graph_macro(c: &mut Criterion) {bench_labelled_undir_graph_macro168,5646
fn bench_labelled_undir_graph_edge_normalization(c: &mut Criterion) {bench_labelled_undir_graph_edge_normalization190,6250

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabDirGraphStEph.rs,932
fn bench_labelled_dir_graph_creation(c: &mut Criterion) {bench_labelled_dir_graph_creation7,263
fn bench_labelled_dir_graph_add_vertex(c: &mut Criterion) {bench_labelled_dir_graph_add_vertex34,1195
fn bench_labelled_dir_graph_add_labeled_arc(c: &mut Criterion) {bench_labelled_dir_graph_add_labeled_arc52,1733
fn bench_labelled_dir_graph_has_arc(c: &mut Criterion) {bench_labelled_dir_graph_has_arc70,2337
fn bench_labelled_dir_graph_get_arc_label(c: &mut Criterion) {bench_labelled_dir_graph_get_arc_label94,3089
fn bench_labelled_dir_graph_out_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_out_neighbors120,3962
fn bench_labelled_dir_graph_in_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_in_neighbors147,4890
fn bench_labelled_dir_graph_arcs(c: &mut Criterion) {bench_labelled_dir_graph_arcs174,5814
fn bench_labelled_dir_graph_macro(c: &mut Criterion) {bench_labelled_dir_graph_macro192,6350

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchUnDirGraphStEph.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build8,274

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch197,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchMathSeq.rs,72
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics9,239

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer.rs,66
fn bench_ll_per_ch18(c: &mut Criterion) {bench_ll_per_ch187,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch188,277

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqMtPerChap18.rs,90
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch187,245

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStEphChap18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map9,317

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer19.rs,66
fn bench_ll_per_ch19(c: &mut Criterion) {bench_ll_per_ch198,291

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph.rs,66
fn bench_ll_eph_ch18(c: &mut Criterion) {bench_ll_eph_ch187,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph.rs,86
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch199,311

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch188,265

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch198,265

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer18.rs,90
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch187,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map9,311

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer.rs,90
fn bench_tabulate_map_mtper_ch19(c: &mut Criterion) {bench_tabulate_map_mtper_ch197,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTAVLStEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree7,215
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl15,408

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSplayMtEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree7,223
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay15,418

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTBBAlphaMtEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree7,231
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha15,430

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTAVLMtEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree7,215
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl15,406

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph.rs,58
fn bench_avl_eph(c: &mut Criterion) {bench_avl_eph7,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTRBMtEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree7,211
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb15,400

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSetMtEph.rs,5094
trait BenchSet: Sized {BenchSet15,756
    fn empty() -> Self;empty16,780
    fn insert_value(&mut self, value: i32);insert_value17,804
    fn union_with(&self, other: &Self) -> Self;union_with18,848
    fn difference_with(&self, other: &Self) -> Self;difference_with19,896
    fn filter_divisible_by(&self, divisor: i32) -> Self;filter_divisible_by20,949
    fn reduce_sum(&self) -> i32;reduce_sum21,1006
fn build_pair<S: BenchSet>(len: usize) -> (S, S) {build_pair24,1042
fn build_single<S: BenchSet>(len: usize) -> S {build_single37,1321
fn bench_set_variants<S: BenchSet>(c: &mut Criterion, label: &str) {bench_set_variants45,1482
impl BenchSet for PlainSet<i32> {PlainSet88,2945
    fn empty() -> Self { BSTSetPlainMtEphLit![] }empty89,2979
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value91,3030
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with93,3098
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with95,3168
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by97,3248
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum99,3352
impl BenchSet for AVLSet<i32> {AVLSet102,3432
    fn empty() -> Self { BSTSetAVLMtEphLit![] }empty103,3464
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value105,3513
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with107,3581
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with109,3651
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by111,3731
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum113,3835
impl BenchSet for RBSet<i32> {RBSet116,3915
    fn empty() -> Self { BSTSetRBMtEphLit![] }empty117,3946
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value119,3994
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with121,4062
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with123,4132
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by125,4212
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum127,4316
impl BenchSet for BBAlphaSet<i32> {BBAlphaSet130,4396
    fn empty() -> Self { BSTSetBBAlphaMtEphLit![] }empty131,4432
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value133,4485
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with135,4553
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with137,4623
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by139,4703
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum141,4807
impl BenchSet for TreapSet<i32> {TreapSet144,4887
    fn empty() -> Self { BSTSetTreapMtEphLit![] }empty145,4921
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value147,4972
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with149,5040
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with151,5110
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by153,5190
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum155,5294
impl BenchSet for SplaySet<i32> {SplaySet158,5374
    fn empty() -> Self { BSTSetSplayMtEphLit![] }empty159,5408
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value161,5459
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with163,5527
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with165,5597
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by167,5677
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum169,5781
fn bench_plain_set(c: &mut Criterion) { bench_set_variants::<PlainSet<i32>>(c, "BSTSetPlainMtEphbench_plain_set172,5861
fn bench_avl_set(c: &mut Criterion) { bench_set_variants::<AVLSet<i32>>(c, "BSTSetAVLMtEph"); }bench_avl_set174,5964
fn bench_rb_set(c: &mut Criterion) { bench_set_variants::<RBSet<i32>>(c, "BSTSetRBMtEph"); }bench_rb_set176,6061
fn bench_bbalpha_set(c: &mut Criterion) { bench_set_variants::<BBAlphaSet<i32>>(c, "BSTSetBBAlphbench_bbalpha_set178,6155
fn bench_treap_set(c: &mut Criterion) { bench_set_variants::<TreapSet<i32>>(c, "BSTSetTreapMtEphbench_treap_set180,6264
fn bench_splay_set(c: &mut Criterion) { bench_set_variants::<SplaySet<i32>>(c, "BSTSetSplayMtEphbench_splay_set182,6367

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTParaStEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree7,213
fn bench_para_bst(c: &mut Criterion) {bench_para_bst15,400

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTRBStEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree7,211
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb15,402

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph19.rs,68
fn bench_avl_eph_ch19(c: &mut Criterion) {bench_avl_eph_ch197,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTParaMtEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree7,198
fn bench_para_bst(c: &mut Criterion) {bench_para_bst15,354

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer.rs,80
fn bench_build_and_contains(c: &mut Criterion) {bench_build_and_contains9,272

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTPlainMtEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree7,218
fn bench_bsteph(c: &mut Criterion) {bench_bsteph17,454

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTBBAlphaStEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree7,231
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha15,432

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer19.rs,95
fn bench_build_and_read_persistent(c: &mut Criterion) {bench_build_and_read_persistent10,345

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer18.rs,68
fn bench_avl_per_ch18(c: &mut Criterion) {bench_avl_per_ch187,234

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTPlainStEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree7,218
fn bench_bsteph(c: &mut Criterion) {bench_bsteph17,456

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSplayStEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree7,223
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay15,420

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph18.rs,68
fn bench_avl_eph_ch18(c: &mut Criterion) {bench_avl_eph_ch187,233
