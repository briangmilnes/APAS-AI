
/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap38/BSTParaStEph.rs,3106
pub mod BSTParaStEph {BSTParaStEph4,162
    pub enum Exposed<T: StT + Ord> {Exposed12,361
        Leaf,Leaf14,417
        Node(ParamBST<T>, T, ParamBST<T>),Node15,431
    struct NodeInner<T: StT + Ord> {NodeInner19,509
        key: T,key20,546
        size: N,size21,562
        left: ParamBST<T>,left22,579
        right: ParamBST<T>,right23,606
    pub struct ParamBST<T: StT + Ord> {ParamBST27,669
        root: Rc<RefCell<Option<Box<NodeInner<T>>>>>,root28,709
    pub trait ParamBSTTrait<T: StT + Ord>: Sized {ParamBSTTrait31,770
        fn new() -> Self;new32,821
        fn expose(&self) -> Exposed<T>;expose33,847
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid34,887
        fn size(&self) -> N;size35,937
        fn is_empty(&self) -> B;is_empty36,966
        fn insert(&self, key: T);insert37,999
        fn delete(&self, key: &T);delete38,1033
        fn find(&self, key: &T) -> Option<T>;find39,1068
        fn split(&self, key: &T) -> (Self, B, Self);split40,1114
        fn join_pair(&self, other: Self) -> Self;join_pair41,1167
        fn union(&self, other: &Self) -> Self;union42,1217
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order43,1264
    impl<T: StT + Ord> ParamBST<T> {ParamBST46,1320
        fn expose_internal(&self) -> Exposed<T> {expose_internal47,1357
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid55,1649
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner67,2101
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m86,3086
        fn min_key(tree: &Self) -> Option<T> {min_key88,3202
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner98,3546
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner109,4016
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order121,4523
    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {ParamBST133,4925
        fn new() -> Self {new134,4983
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose140,5109
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid142,5176
        fn size(&self) -> N { self.root.borrow().as_ref().map_or(0, |node| node.size) }size144,5258
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty146,5347
        fn insert(&self, key: T) {insert148,5434
        fn delete(&self, key: &T) {delete155,5724
        fn find(&self, key: &T) -> Option<T> {find162,6016
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split173,6513
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair175,6603
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union177,6704
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order179,6790
    macro_rules! ParamBSTLit {ParamBSTLit187,7034
    fn _ParamBSTLit_type_checks() {_ParamBSTLit_type_checks200,7557

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap38/BSTParaMtEph.rs,4171
pub mod BSTParaMtEph {BSTParaMtEph4,161
    pub enum Exposed<T: StTInMtT + Ord> {Exposed11,329
        Leaf,Leaf12,371
        Node(ParamBST<T>, T, ParamBST<T>),Node13,385
    struct NodeInner<T: StTInMtT + Ord> {NodeInner18,477
        key: T,key19,519
        size: N,size20,535
        left: ParamBST<T>,left21,552
        right: ParamBST<T>,right22,579
    pub struct ParamBST<T: StTInMtT + Ord> {ParamBST26,642
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root27,687
    pub trait ParamBSTTrait<T: StTInMtT + Ord + 'static>: Sized {ParamBSTTrait30,748
        fn new() -> Self;new33,905
        fn expose(&self) -> Exposed<T>;expose36,1022
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid39,1153
        fn size(&self) -> N;size42,1294
        fn is_empty(&self) -> B;is_empty45,1414
        fn insert(&self, key: T);insert48,1558
        fn delete(&self, key: &T);delete51,1703
        fn find(&self, key: &T) -> Option<T>;find54,1849
        fn split(&self, key: &T) -> (Self, B, Self);split57,2006
        fn join_pair(&self, other: Self) -> Self;join_pair60,2218
        fn union(&self, other: &Self) -> Self;union63,2393
        fn intersect(&self, other: &Self) -> Self;intersect66,2565
        fn difference(&self, other: &Self) -> Self;difference69,2741
        fn filter<F>(&self, predicate: F) -> Selffilter72,2898
        fn reduce<F>(&self, op: F, base: T) -> Treduce77,3122
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order82,3338
    impl<T: StTInMtT + Ord + 'static> ParamBST<T> {ParamBST85,3394
        fn expose_internal(&self) -> Exposed<T> {expose_internal88,3537
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid98,3927
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner112,4490
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m133,5566
        fn min_key(tree: &Self) -> Option<T> {min_key137,5793
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner149,6308
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner162,6903
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner178,7620
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner198,8583
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner219,9559
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel243,10641
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner253,11003
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel277,12082
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order287,12421
    impl<T: StTInMtT + Ord + 'static> ParamBSTTrait<T> for ParamBST<T> {ParamBST299,12823
        fn new() -> Self {new302,12987
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose310,13204
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid314,13362
        fn size(&self) -> N {size318,13535
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty325,13773
        fn insert(&self, key: T) {insert329,13971
        fn delete(&self, key: &T) {delete339,14414
        fn find(&self, key: &T) -> Option<T> {find349,14859
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split362,15467
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair366,15716
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union370,15942
        fn intersect(&self, other: &Self) -> Self { ParamBST::intersect_inner(self, other) }intersect374,16153
        fn difference(&self, other: &Self) -> Self { ParamBST::difference_inner(self, other) }difference378,16372
        fn filter<F>(&self, predicate: F) -> Selffilter382,16573
        fn reduce<F>(&self, op: F, base: T) -> Treduce391,16873
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order400,17164

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEphSlice.rs,5815
pub mod ArraySeqMtEphSlice {ArraySeqMtEphSlice9,487
    struct Inner<T: StT + Send + Sync> {Inner17,676
        data: Mutex<Box<[T]>>,data18,717
    impl<T: StT + Send + Sync> Inner<T> {Inner21,755
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }new22,797
        fn len(&self) -> N {len24,874
    pub struct ArraySeqMtEphSliceS<T: StT + Send + Sync> {ArraySeqMtEphSliceS31,1062
        inner: Arc<Inner<T>>,inner32,1121
        range: Range<N>,range33,1151
    pub trait ArraySeqMtEphSliceTrait<T: StT + Send + Sync> {ArraySeqMtEphSliceTrait37,1247
        fn new(length: N, init_value: T) -> Self;new38,1309
        fn length(&self) -> N;length39,1359
        fn nth_cloned(&self, index: N) -> T;nth_cloned40,1390
        fn empty() -> Self;empty41,1435
        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;update42,1463
        fn singleton(item: T) -> Self;singleton43,1547
        fn isEmpty(&self) -> B;isEmpty44,1586
        fn isSingleton(&self) -> B;isSingleton45,1618
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy46,1654
        fn slice(&self, start: N, length: N) -> Self;slice47,1714
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> Self;tabulate48,1768
        fn map<U: StT + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(amap49,1839
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &Self, pred: F) -> Self whefilter50,2001
        fn append(a: &Self, b: &Self) -> Self;append51,2119
        fn append_select(a: &Self, b: &Self) -> Self;append_select52,2166
        fn flatten(sequences: &[ArraySeqMtEphSliceS<T>]) -> Self;flatten53,2220
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> reduce54,2286
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &Self, f: &F, id: T) -> (ArraySeqMtEphSlicescan55,2409
        fn iterate<A: StT + Send, F: Fn(&A, &T) -> A + Send + Sync>(a: &Self, f: &F, seed: A) ->iterate56,2515
        fn inject(a: &Self, updates: &[(N, T)]) -> Self;inject57,2615
        fn ninject(a: &Self, updates: &[(N, T)]) -> Self;ninject58,2672
    impl<T: StT + Send + Sync> ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS61,2737
        pub fn from_box(data: Box<[T]>) -> Self {from_box63,2854
        pub fn from_vec(data: Vec<T>) -> Self { Self::from_box(data.into_boxed_slice()) }from_vec72,3156
        pub fn to_vec(&self) -> Vec<T> {to_vec75,3328
        pub         fn with_exclusive<F, R>(&self, f: F) -> Rwith_exclusive81,3592
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set92,3972
        fn len(&self) -> N { self.range.end - self.range.start }len96,4106
        fn clamp_subrange(&self, start: N, length: N) -> Range<N> {clamp_subrange98,4172
    impl<T: StT + Send + Sync> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS107,4532
        fn new(length: N, init_value: T) -> Self {new108,4619
        fn length(&self) -> N { self.len() }length113,4784
        fn nth_cloned(&self, index: N) -> T {nth_cloned115,4830
        fn empty() -> Self {empty121,5023
        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {update126,5260
        fn singleton(item: T) -> Self {singleton139,5660
        fn isEmpty(&self) -> B { if self.len() == 0 { B::True } else { B::False } }isEmpty152,6100
        fn isSingleton(&self) -> B { if self.len() == 1 { B::True } else { B::False } }isSingleton154,6185
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy156,6274
        fn slice(&self, start: N, length: N) -> Self {slice163,6593
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> Self {tabulate171,6841
        fn map<U: StT + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(amap179,7111
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &Self, pred: F) -> Self whefilter205,8205
        fn append(a: &Self, b: &Self) -> Self {append240,9597
        fn append_select(a: &Self, b: &Self) -> Self {append_select246,9842
        fn flatten(sequences: &[ArraySeqMtEphSliceS<T>]) -> Self {flatten258,10307
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> reduce280,11094
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &Self, f: &F, id: T) -> (ArraySeqMtEphSlicescan311,12252
        fn iterate<A: StT + Send, F: Fn(&A, &T) -> A + Send + Sync>(a: &Self, f: &F, seed: A) ->iterate335,13320
        fn inject(a: &Self, updates: &[(N, T)]) -> Self {inject345,13691
        fn ninject(a: &Self, updates: &[(N, T)]) -> Self {ninject356,14103
    impl<T: StT + Send + Sync> Clone for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS368,14536
        fn clone(&self) -> Self {clone369,14602
    impl<T: StT + Send + Sync> PartialEq for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS377,14792
        fn eq(&self, other: &Self) -> bool {eq378,14862
    impl<T: StT + Send + Sync> Eq for ArraySeqMtEphSliceS<T> {}ArraySeqMtEphSliceS391,15280
    impl<T: StT + Send + Sync> Debug for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS393,15345
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt394,15411
    impl<T: StT + Send + Sync> Display for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS402,15678
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt403,15746
    fn repeat_vec<T: StT + Send + Sync>(length: N, init: T) -> Vec<T> {repeat_vec418,16213
    macro_rules! ArraySeqMtEphSliceSLit {ArraySeqMtEphSliceSLit427,16452
    fn _ArraySeqMtEphSliceSLit_type_checks() {_ArraySeqMtEphSliceSLit_type_checks434,16915

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStEph.rs,3858
pub mod ArraySeqStEph {ArraySeqStEph4,143
    pub type ArraySeqStEphS<T> = ArraySeqStEphSChap18<T>;ArraySeqStEphS9,396
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait11,455
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>;new12,498
        fn empty() -> ArraySeqStEphS<T>;empty13,561
        fn singleton(item: T) -> ArraySeqStEphS<T>;singleton14,602
        fn length(&self) -> N;length15,654
        fn nth(&self, index: N) -> &T;nth16,685
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T>;subseq_copy17,724
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStEphS<T>;tabulate20,884
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U>;map22,1034
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T>;select24,1166
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append26,1307
        fn append_select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append_select28,1446
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStEphS<T>;deflate30,1580
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T>;filter32,1738
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, x: A) -> A;iterate33,1827
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T;reduce34,1916
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, scan35,1997
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten36,2097
        fn isEmpty(a: &ArraySeqStEphS<T>) -> bool;isEmpty37,2177
        fn isSingleton(a: &ArraySeqStEphS<T>) -> bool;isSingleton38,2228
        fn update(a: &ArraySeqStEphS<T>, index: N, item: T) -> ArraySeqStEphS<T>;update39,2283
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS42,2372
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T> {new43,2435
        fn empty() -> ArraySeqStEphS<T> {empty48,2655
        fn singleton(item: T) -> ArraySeqStEphS<T> {singleton53,2894
        fn length(&self) -> N { ArraySeqStEphTraitChap18::length(self) }length58,3116
        fn nth(&self, index: N) -> &T { ArraySeqStEphTraitChap18::nth(self, index) }nth60,3190
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T> {subseq_copy62,3276
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStEphS<T> {tabulate67,3527
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U> {map79,4009
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T> {select84,4282
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append98,4722
        fn append_select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append_select115,5511
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStEphS<T> {deflate123,5918
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T> {filter132,6303
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, x: A) -> A {iterate138,6702
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T {reduce147,7011
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, scan163,7746
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten178,8434
        fn isEmpty(a: &ArraySeqStEphS<T>) -> bool {isEmpty183,8676
        fn isSingleton(a: &ArraySeqStEphS<T>) -> bool {isSingleton188,8818
        fn update(a: &ArraySeqStEphS<T>, index: N, item: T) -> ArraySeqStEphS<T> {update193,8968

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/#ArraySeqMtEph.rs#,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtPer.rs,4832
pub mod ArraySeqMtPer {ArraySeqMtPer4,224
    pub trait ArraySeqMtPerTrait<T: StTInMtT> {ArraySeqMtPerTrait10,427
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;new12,506
        fn empty() -> ArraySeqMtPerS<T>;empty13,569
        fn singleton(item: T) -> ArraySeqMtPerS<T>;singleton14,610
        fn length(&self) -> N;length15,662
        fn nth(&self, index: N) -> &T;nth16,693
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;subseq_copy17,732
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T>;tabulate19,806
        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(a: &ArraySmap20,890
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append21,1044
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, pred: Ffilter22,1130
        fn update_single(a: &ArraySeqMtPerS<T>, index: N, item: T) -> ArraySeqMtPerS<T>;update_single23,1267
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject24,1356
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, iterate25,1458
        fn iteratePrefixes<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>,iteratePrefixes26,1566
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: reduce27,1703
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (Arrayscan28,1832
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;flatten29,1946
        fn collect(collect30,2027
        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMinject36,2224
        fn atomicWrite(atomicWrite37,2330
        fn isEmpty(a: &ArraySeqMtPerS<T>) -> bool;isEmpty42,2516
        fn isSingleton(a: &ArraySeqMtPerS<T>) -> bool;isSingleton43,2567
        fn append_select(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append_select44,2622
        fn select<'a>(a: &'a ArraySeqMtPerS<T>, b: &'a ArraySeqMtPerS<T>, i: N) -> Option<&'a T>select45,2715
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtPerS<T>;deflate46,2813
    impl<T: StTInMtT> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {ArraySeqMtPerS49,2905
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> {new50,2973
        fn empty() -> ArraySeqMtPerS<T> {empty55,3193
        fn singleton(item: T) -> ArraySeqMtPerS<T> {singleton60,3432
        fn length(&self) -> N { ArraySeqMtPerTraitChap18::length(self) }length65,3654
        fn nth(&self, index: N) -> &T { ArraySeqMtPerTraitChap18::nth(self, index) }nth67,3728
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> {subseq_copy69,3814
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T> {tabulate74,3997
        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(a: &ArraySmap84,4443
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append105,5653
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, pred: Ffilter111,6057
        fn update_single(a: &ArraySeqMtPerS<T>, index: N, item: T) -> ArraySeqMtPerS<T> {update_single147,7474
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject155,7863
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, iterate160,8136
        fn iteratePrefixes<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>,iteratePrefixes169,8464
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: reduce181,9035
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (Arrayscan203,10182
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {flatten216,10778
        fn collect(collect221,11022
        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMinject228,11269
        fn atomicWrite(atomicWrite232,11475
        fn isEmpty(a: &ArraySeqMtPerS<T>) -> bool {isEmpty240,11774
        fn isSingleton(a: &ArraySeqMtPerS<T>) -> bool {isSingleton245,11916
        fn append_select(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append_select250,12066
        fn select<'a>(a: &'a ArraySeqMtPerS<T>, b: &'a ArraySeqMtPerS<T>, i: N) -> Option<&'a T>select258,12481
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtPerS<T> {deflate272,12903

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEph.rs,4033
pub mod ArraySeqMtEph {ArraySeqMtEph4,159
    pub type ArraySeqMtEphS<T> = ArraySeqMtEphSChap18<T>;ArraySeqMtEphS11,407
    pub trait ArraySeqMtEphTrait<T: StTInMtT> {ArraySeqMtEphTrait13,466
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>;new14,514
        fn empty() -> ArraySeqMtEphS<T>;empty15,577
        fn singleton(item: T) -> ArraySeqMtEphS<T>;singleton16,618
        fn length(&self) -> N;length17,670
        fn nth_cloned(&self, index: N) -> T;nth_cloned18,701
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;subseq_copy19,746
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;tabulate21,820
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &ArraySmap22,904
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T>;select23,1058
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append24,1146
        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append_select25,1232
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtEphS<T>;deflate26,1325
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, pred: Ffilter27,1410
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, iterate28,1547
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: reduce29,1655
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (Arrayscan30,1784
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;flatten31,1898
        fn isEmpty(a: &ArraySeqMtEphS<T>) -> bool;isEmpty32,1978
        fn isSingleton(a: &ArraySeqMtEphS<T>) -> bool;isSingleton33,2029
        fn update(a: &ArraySeqMtEphS<T>, index: N, item: T) -> ArraySeqMtEphS<T>;update34,2084
    impl<T: StTInMtT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS37,2173
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T> {new38,2241
        fn empty() -> ArraySeqMtEphS<T> {empty43,2461
        fn singleton(item: T) -> ArraySeqMtEphS<T> {singleton48,2700
        fn length(&self) -> N { ArraySeqMtEphTraitChap18::length(self) }length53,2922
        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphTraitChap18::nth_cloned(self, index) nth_cloned55,2996
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {subseq_copy57,3095
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {tabulate62,3325
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &ArraySmap74,3821
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T> {select85,4443
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append99,4881
        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append_select116,5666
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtEphS<T> {deflate124,6073
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, pred: Ffilter133,6384
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, iterate190,8701
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: reduce200,9070
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (Arrayscan221,10148
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {flatten238,10928
        fn isEmpty(a: &ArraySeqMtEphS<T>) -> bool {isEmpty243,11170
        fn isSingleton(a: &ArraySeqMtEphS<T>) -> bool {isSingleton248,11312
        fn update(a: &ArraySeqMtEphS<T>, index: N, item: T) -> ArraySeqMtEphS<T> {update253,11462

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStPer.rs,4328
pub mod ArraySeqStPer {ArraySeqStPer4,138
    pub type ArraySeqStPerS<T> = ArraySeqStPerSChap18<T>;ArraySeqStPerS9,391
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait11,450
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>;new12,493
        fn empty() -> ArraySeqStPerS<T>;empty13,556
        fn singleton(item: T) -> ArraySeqStPerS<T>;singleton14,597
        fn length(&self) -> N;length15,649
        fn nth(&self, index: N) -> &T;nth16,680
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T>;subseq_copy17,719
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStPerS<T>;tabulate20,879
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U>;map22,1029
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>select24,1161
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append26,1312
        fn append_select(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append_select28,1451
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStPerS<T>;deflate30,1585
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T>;filter32,1743
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, x: A) -> A;iterate33,1832
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T;reduce34,1921
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, scan35,2002
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;flatten36,2102
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject37,2182
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject38,2283
        fn isEmpty(a: &ArraySeqStPerS<T>) -> bool;isEmpty39,2385
        fn isSingleton(a: &ArraySeqStPerS<T>) -> bool;isSingleton40,2436
        fn update(a: &ArraySeqStPerS<T>, index: N, item: T) -> ArraySeqStPerS<T>;update41,2491
    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {ArraySeqStPerS44,2580
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> {new45,2643
        fn empty() -> ArraySeqStPerS<T> { empty50,2863
        fn singleton(item: T) -> ArraySeqStPerS<T> {singleton55,3103
        fn length(&self) -> N { ArraySeqStPerTraitChap18::length(self) }length60,3325
        fn nth(&self, index: N) -> &T { ArraySeqStPerTraitChap18::nth(self, index) }nth62,3399
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T> {subseq_copy64,3485
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStPerS<T> {tabulate69,3741
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U> {map79,4173
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>select84,4446
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {append98,4868
        fn append_select(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {append_select104,5272
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStPerS<T> {deflate112,5687
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {filter121,6064
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, x: A) -> A {iterate127,6463
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T {reduce136,6772
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, scan152,7507
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {flatten165,8089
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject170,8331
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject175,8601
        fn isEmpty(a: &ArraySeqStPerS<T>) -> bool {isEmpty180,8874
        fn isSingleton(a: &ArraySeqStPerS<T>) -> bool {isSingleton185,9016
        fn update(a: &ArraySeqStPerS<T>, index: N, item: T) -> ArraySeqStPerS<T> {update190,9166

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphFloat.rs,1258
pub mod WeightedUnDirGraphMtEphFloat {WeightedUnDirGraphMtEphFloat4,199
    pub type WeightedUnDirGraphMtEphFloat<V> = LabUnDirGraphMtEph<V, OrderedF64>;WeightedUnDirGraphMtEphFloat13,555
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphFloat<V> {WeightedUnDirGraphMtEphFloat16,744
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges18,866
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge33,1429
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight38,1630
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges43,1837
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted52,2198
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight65,2755
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree73,3042
    macro_rules! WeightedUnDirGraphMtEphFloatLit {WeightedUnDirGraphMtEphFloatLit77,3150
    pub fn __weighted_undir_graph_mt_float_macro_typecheck_exercise() {__weighted_undir_graph_mt_float_macro_typecheck_exercise89,3775

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphInt.rs,1316
pub mod WeightedUnDirGraphStEphInt {WeightedUnDirGraphStEphInt4,193
    pub type WeightedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;WeightedUnDirGraphStEphInt13,524
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphInt<V> {WeightedUnDirGraphStEphInt16,680
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges18,794
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(vadd_weighted_edge33,1343
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, vget_edge_weight36,1510
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges39,1683
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted48,2024
        pub fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum()total_weight61,2561
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree64,2727
        pub fn is_connected(&self) -> bool {is_connected67,2894
    macro_rules! WeightedUnDirGraphStEphIntLit {WeightedUnDirGraphStEphIntLit96,3906
    pub fn __weighted_undir_graph_st_int_macro_typecheck_exercise() {__weighted_undir_graph_st_int_macro_typecheck_exercise108,4502

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphStEph.rs,2679
pub mod UnDirGraphStEph {UnDirGraphStEph4,172
    pub struct UnDirGraphStEph<V: StT + Hash> {UnDirGraphStEph13,402
        V: Set<V>,V14,450
        E: Set<Edge<V>>,E15,469
    pub trait UnDirGraphStEphTrait<V: StT + Hash> {UnDirGraphStEphTrait18,501
        fn empty() -> UnDirGraphStEph<V>;empty21,645
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V>;FromSets24,795
        fn vertices(&self) -> &Set<V>;vertices27,958
        fn edges(&self) -> &Set<Edge<V>>;edges30,1089
        fn sizeV(&self) -> N;sizeV33,1223
        fn sizeE(&self) -> N;sizeE36,1345
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor39,1467
        fn NG(&self, v: &V) -> Set<V>;NG42,1610
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices45,1767
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident48,1917
        fn Degree(&self, v: &V) -> N;Degree51,2066
    impl<V: StT + Hash> UnDirGraphStEphTrait<V> for UnDirGraphStEph<V> {UnDirGraphStEph54,2111
        fn empty() -> UnDirGraphStEph<V> {empty55,2184
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E }FromSets61,2341
        fn vertices(&self) -> &Set<V> { &self.V }vertices62,2440
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges63,2490
        fn sizeV(&self) -> N { self.V.size() }sizeV64,2543
        fn sizeE(&self) -> N { self.E.size() }sizeE65,2590
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor67,2638
        fn NG(&self, v: &V) -> Set<V> {NG77,2962
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices89,3324
        fn Incident(&self, e: &Edge<V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } elseIncident98,3596
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree100,3709
    impl<V: StT + Hash> Debug for UnDirGraphStEph<V> {UnDirGraphStEph103,3775
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt104,3830
    impl<V: StT + Hash> Display for UnDirGraphStEph<V> {UnDirGraphStEph112,4050
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} E={:?}", self.V, self.Efmt113,4107
    impl<V: StT + Hash> PartialEq for UnDirGraphStEph<V> {UnDirGraphStEph116,4214
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq117,4273
    impl<V: StT + Hash> Eq for UnDirGraphStEph<V> {}UnDirGraphStEph119,4365
    macro_rules! UnDirGraphStEphLit {UnDirGraphStEphLit122,4439
    fn _UnDirGraphStEphLit_type_checks() {_UnDirGraphStEphLit_type_checks140,5564
    pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise146,5827

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphInt.rs,1541
pub mod WeightedDirGraphStEphInt {WeightedDirGraphStEphInt4,191
    pub type WeightedDirGraphStEphInt<V> = LabDirGraphStEph<V, i32>;WeightedDirGraphStEphInt13,514
    impl<V: StT + Hash> WeightedDirGraphStEphInt<V> {WeightedDirGraphStEphInt16,664
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges18,770
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(add_weighted_edge33,1309
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(fromget_edge_weight36,1479
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges39,1657
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted48,2006
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted59,2419
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() total_weight70,2829
        pub fn edges_above_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_above_weight73,2985
        pub fn edges_below_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_below_weight84,3430
    macro_rules! WeightedDirGraphStEphIntLit {WeightedDirGraphStEphIntLit96,3847
    fn _WeightedDirGraphStEphIntLit_type_checks() {_WeightedDirGraphStEphIntLit_type_checks108,4433
    pub fn __weighted_dir_graph_st_int_macro_typecheck_exercise() {__weighted_dir_graph_st_int_macro_typecheck_exercise114,4725

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphFloat.rs,1530
pub mod WeightedUnDirGraphStEphFloat {WeightedUnDirGraphStEphFloat30,1202
    pub type WeightedUnDirGraphStEphFloat<V> = LabUnDirGraphStEph<V, OrderedF64>;WeightedUnDirGraphStEphFloat39,1542
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphFloat<V> {WeightedUnDirGraphStEphFloat42,1714
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges44,1830
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge59,2393
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight64,2594
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges69,2801
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted78,3156
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight91,3707
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree99,3994
        pub fn is_connected(&self) -> bool {is_connected102,4161
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge130,5187
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge138,5485
    macro_rules! WeightedUnDirGraphStEphFloatLit {WeightedUnDirGraphStEphFloatLit147,5769
    pub fn __weighted_undir_graph_st_float_macro_typecheck_exercise() {__weighted_undir_graph_st_float_macro_typecheck_exercise159,6394

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphInt.rs,1255
pub mod WeightedUnDirGraphMtEphInt {WeightedUnDirGraphMtEphInt4,192
    pub type WeightedUnDirGraphMtEphInt<V> = LabUnDirGraphMtEph<V, i32>;WeightedUnDirGraphMtEphInt13,539
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphInt<V> {WeightedUnDirGraphMtEphInt16,712
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges18,832
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(vadd_weighted_edge33,1381
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, vget_edge_weight36,1548
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges39,1721
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted48,2068
        pub fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum()total_weight61,2611
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree64,2777
    macro_rules! WeightedUnDirGraphMtEphIntLit {WeightedUnDirGraphMtEphIntLit68,2885
    pub fn __weighted_undir_graph_mt_int_macro_typecheck_exercise() {__weighted_undir_graph_mt_int_macro_typecheck_exercise80,3481

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphMtEph.rs,2687
pub mod LabUnDirGraphMtEph {LabUnDirGraphMtEph4,211
    pub struct LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph13,444
        vertices: Set<V>,vertices18,563
        labeled_edges: Set<LabEdge<V, L>>,labeled_edges19,589
    pub trait LabUnDirGraphMtEphTrait<V, L>LabUnDirGraphMtEphTrait22,639
        fn empty() -> Self;empty27,762
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges28,790
        fn vertices(&self) -> &Set<V>;vertices29,895
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;labeled_edges30,934
        fn edges(&self) -> Set<Edge<V>>;edges31,990
        fn add_vertex(&mut self, v: V);add_vertex32,1031
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);add_labeled_edge33,1071
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;get_edge_label34,1135
        fn has_edge(&self, v1: &V, v2: &V) -> bool;has_edge35,1199
        fn neighbors(&self, v: &V) -> Set<V>;neighbors36,1251
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;normalize_edge37,1297
    impl<V, L> LabUnDirGraphMtEphTrait<V, L> for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph40,1362
        fn empty() -> Self {empty45,1515
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges52,1687
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices59,1908
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> { &self.labeled_edges }labeled_edges61,1966
        fn edges(&self) -> Set<Edge<V>> {edges63,2046
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex71,2323
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge73,2392
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label84,2784
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge95,3221
        fn neighbors(&self, v: &V) -> Set<V> {neighbors105,3566
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge117,4005
    impl<V, L> Display for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph125,4425
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt130,4556
    impl<V, L> Debug for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph135,4718
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt140,4847
    macro_rules! LabUnDirGraphMtEphLit {LabUnDirGraphMtEphLit150,5119
    fn _LabUnDirGraphMtEphLit_type_checks() {_LabUnDirGraphMtEphLit_type_checks173,6325
    pub fn __lab_undir_graph_mt_macro_typecheck_exercise() {__lab_undir_graph_mt_macro_typecheck_exercise179,6604

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphFloat.rs,1253
pub mod WeightedDirGraphMtEphFloat {WeightedDirGraphMtEphFloat4,197
    pub type WeightedDirGraphMtEphFloat<V> = LabDirGraphMtEph<V, OrderedF64>;WeightedDirGraphMtEphFloat13,545
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphFloat<V> {WeightedDirGraphMtEphFloat16,728
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges18,842
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge33,1395
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight38,1599
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges43,1811
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted52,2180
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted63,2610
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight74,3037
    macro_rules! WeightedDirGraphMtEphFloatLit {WeightedDirGraphMtEphFloatLit83,3283
    pub fn __weighted_dir_graph_mt_float_macro_typecheck_exercise() {__weighted_dir_graph_mt_float_macro_typecheck_exercise95,3896

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphMtEph.rs,2767
pub mod UnDirGraphMtEph {UnDirGraphMtEph4,197
    pub struct UnDirGraphMtEph<V: StT + MtT + Hash> {UnDirGraphMtEph12,372
        V: Set<V>,V13,426
        E: Set<Edge<V>>,E14,445
    pub trait UnDirGraphMtEphTrait<V: StT + MtT + Hash> {UnDirGraphMtEphTrait17,477
        fn empty() -> UnDirGraphMtEph<V>;empty20,627
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V>;FromSets23,777
        fn vertices(&self) -> &Set<V>;vertices26,940
        fn edges(&self) -> &Set<Edge<V>>;edges29,1071
        fn sizeV(&self) -> N;sizeV32,1205
        fn sizeE(&self) -> N;sizeE35,1327
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1449
        fn NG(&self, v: &V) -> Set<V>;NG41,1592
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1749
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident47,1899
        fn Degree(&self, v: &V) -> N;Degree50,2048
    impl<V: StT + MtT + Hash> UnDirGraphMtEphTrait<V> for UnDirGraphMtEph<V> {UnDirGraphMtEph53,2093
        fn empty() -> UnDirGraphMtEph<V> {empty54,2172
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V> { UnDirGraphMtEph { V, E }FromSets60,2329
        fn vertices(&self) -> &Set<V> { &self.V }vertices61,2428
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges62,2478
        fn sizeV(&self) -> N { self.V.size() }sizeV63,2531
        fn sizeE(&self) -> N { self.E.size() }sizeE64,2578
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor66,2626
        fn NG(&self, v: &V) -> Set<V> {NG77,2978
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices89,3346
        fn Incident(&self, e: &Edge<V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } elseIncident98,3618
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree100,3731
    impl<V: StT + MtT + Hash> std::fmt::Debug for UnDirGraphMtEph<V> {UnDirGraphMtEph103,3797
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt104,3868
    impl<V: StT + MtT + Hash> std::fmt::Display for UnDirGraphMtEph<V> {UnDirGraphMtEph112,4108
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} E={fmt113,4181
    impl<V: StT + MtT + Hash> PartialEq for UnDirGraphMtEph<V> {UnDirGraphMtEph116,4308
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq117,4373
    impl<V: StT + MtT + Hash> Eq for UnDirGraphMtEph<V> {}UnDirGraphMtEph119,4465
    macro_rules! UnDirGraphMtEphLit {UnDirGraphMtEphLit122,4545
    fn _UnDirGraphMtEphLit_type_checks() {_UnDirGraphMtEphLit_type_checks140,5670
    pub fn __undirgraph_mt_macro_typecheck_exercise() {__undirgraph_mt_macro_typecheck_exercise146,5922

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphStEph.rs,2619
pub mod LabDirGraphStEph {LabDirGraphStEph4,183
    pub struct LabDirGraphStEph<V, L>LabDirGraphStEph13,414
        vertices: Set<V>,vertices18,514
        labeled_arcs: Set<LabEdge<V, L>>,labeled_arcs19,540
    pub trait LabDirGraphStEphTrait<V, L>LabDirGraphStEphTrait22,589
        fn empty() -> Self;empty27,693
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs28,721
        fn vertices(&self) -> &Set<V>;vertices29,824
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;labeled_arcs30,863
        fn arcs(&self) -> Set<Edge<V>>;arcs31,918
        fn add_vertex(&mut self, v: V);add_vertex32,958
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);add_labeled_arc33,998
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;get_arc_label34,1063
        fn has_arc(&self, from: &V, to: &V) -> bool;has_arc35,1128
        fn out_neighbors(&self, v: &V) -> Set<V>;out_neighbors36,1181
        fn in_neighbors(&self, v: &V) -> Set<V>;in_neighbors37,1231
    impl<V, L> LabDirGraphStEphTrait<V, L> for LabDirGraphStEph<V, L>LabDirGraphStEph40,1287
        fn empty() -> Self {empty45,1419
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs52,1588
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices56,1759
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }labeled_arcs58,1817
        fn arcs(&self) -> Set<Edge<V>> {arcs60,1895
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex68,2158
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc70,2227
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label76,2462
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc85,2763
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors94,3037
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors104,3359
    impl<V, L> Display for LabDirGraphStEph<V, L>LabDirGraphStEph115,3686
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt120,3823
    impl<V, L> Debug for LabDirGraphStEph<V, L>LabDirGraphStEph125,3982
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt130,4117
    macro_rules! LabDirGraphStEphLit {LabDirGraphStEphLit140,4385
    fn _LabDirGraphStEphLit_type_checks() {_LabDirGraphStEphLit_type_checks152,5177
    pub fn __lab_dir_graph_macro_typecheck_exercise() {__lab_dir_graph_macro_typecheck_exercise158,5448

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphStEph.rs,3507
pub mod DirGraphStEph {DirGraphStEph4,169
    pub struct DirGraphStEph<V: StT + Hash> {DirGraphStEph13,397
        V: Set<V>,V14,443
        A: Set<Edge<V>>,A15,462
    pub trait DirGraphStEphTrait<V: StT + Hash> {DirGraphStEphTrait18,494
        fn empty() -> DirGraphStEph<V>;empty21,636
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V>;FromSets24,784
        fn vertices(&self) -> &Set<V>;vertices27,945
        fn arcs(&self) -> &Set<Edge<V>>;arcs30,1076
        fn sizeV(&self) -> N;sizeV33,1209
        fn sizeA(&self) -> N;sizeA36,1331
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor39,1453
        fn NG(&self, v: &V) -> Set<V>;NG42,1596
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices45,1753
        fn NPlus(&self, v: &V) -> Set<V>;NPlus48,1907
        fn NMinus(&self, v: &V) -> Set<V>;NMinus51,2045
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices54,2206
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices57,2385
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident60,2539
        fn Degree(&self, v: &V) -> N;Degree63,2691
        fn InDegree(&self, v: &V) -> N;InDegree66,2825
        fn OutDegree(&self, v: &V) -> N;OutDegree69,2961
    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {DirGraphStEph72,3009
        fn empty() -> DirGraphStEph<V> {empty73,3078
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }FromSets79,3231
        fn vertices(&self) -> &Set<V> { &self.V }vertices80,3326
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs81,3376
        fn sizeV(&self) -> N { self.V.size() }sizeV82,3428
        fn sizeA(&self) -> N { self.A.size() }sizeA83,3475
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor85,3523
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG94,3788
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices96,3845
        fn NPlus(&self, v: &V) -> Set<V> {NPlus105,4117
        fn NMinus(&self, v: &V) -> Set<V> {NMinus115,4399
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices125,4682
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices134,4964
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } eIncident143,5250
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree145,5366
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree146,5428
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree147,5493
    impl<V: StT + Hash> Debug for DirGraphStEph<V> {DirGraphStEph150,5565
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt151,5618
    impl<V: StT + Hash> Display for DirGraphStEph<V> {DirGraphStEph159,5836
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.Afmt160,5891
    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {DirGraphStEph163,5998
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq164,6055
    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}DirGraphStEph166,6147
    macro_rules! DirGraphStEphLit {DirGraphStEphLit169,6219
    fn _DirGraphStEphLit_type_checks() {_DirGraphStEphLit_type_checks186,7312
    pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise192,7567

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphMtEph.rs,3595
pub mod DirGraphMtEph {DirGraphMtEph4,194
    pub struct DirGraphMtEph<V: StT + MtT + Hash> {DirGraphMtEph12,367
        V: Set<V>,V13,419
        A: Set<Edge<V>>,A14,438
    pub trait DirGraphMtEphTrait<V: StT + MtT + Hash> {DirGraphMtEphTrait17,470
        fn empty() -> DirGraphMtEph<V>;empty20,618
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V>;FromSets23,766
        fn vertices(&self) -> &Set<V>;vertices26,927
        fn arcs(&self) -> &Set<Edge<V>>;arcs29,1058
        fn sizeV(&self) -> N;sizeV32,1191
        fn sizeA(&self) -> N;sizeA35,1313
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1435
        fn NG(&self, v: &V) -> Set<V>;NG41,1578
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1735
        fn NPlus(&self, v: &V) -> Set<V>;NPlus47,1889
        fn NMinus(&self, v: &V) -> Set<V>;NMinus50,2027
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices53,2188
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices56,2367
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident59,2521
        fn Degree(&self, v: &V) -> N;Degree62,2673
        fn InDegree(&self, v: &V) -> N;InDegree65,2807
        fn OutDegree(&self, v: &V) -> N;OutDegree68,2943
    impl<V: StT + MtT + Hash> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {DirGraphMtEph71,2991
        fn empty() -> DirGraphMtEph<V> {empty72,3066
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V> { DirGraphMtEph { V, A } }FromSets78,3219
        fn vertices(&self) -> &Set<V> { &self.V }vertices79,3314
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs80,3364
        fn sizeV(&self) -> N { self.V.size() }sizeV81,3416
        fn sizeA(&self) -> N { self.A.size() }sizeA82,3463
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor84,3511
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG93,3782
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices95,3839
        fn NPlus(&self, v: &V) -> Set<V> {NPlus104,4111
        fn NMinus(&self, v: &V) -> Set<V> {NMinus114,4396
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices124,4682
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices133,4964
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } eIncident142,5250
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree144,5366
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree145,5428
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree146,5493
    impl<V: StT + MtT + Hash> std::fmt::Debug for DirGraphMtEph<V> {DirGraphMtEph149,5565
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt150,5634
    impl<V: StT + MtT + Hash> std::fmt::Display for DirGraphMtEph<V> {DirGraphMtEph158,5872
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} A={fmt159,5943
    impl<V: StT + MtT + Hash> PartialEq for DirGraphMtEph<V> {DirGraphMtEph162,6070
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq163,6133
    impl<V: StT + MtT + Hash> Eq for DirGraphMtEph<V> {}DirGraphMtEph165,6225
    macro_rules! DirGraphMtEphLit {DirGraphMtEphLit168,6303
    fn _DirGraphMtEphLit_type_checks() {_DirGraphMtEphLit_type_checks185,7396
    pub fn __dirgraph_mt_macro_typecheck_exercise() {__dirgraph_mt_macro_typecheck_exercise191,7640

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphFloat.rs,1907
pub mod WeightedDirGraphStEphFloat {WeightedDirGraphStEphFloat30,1167
    pub type WeightedDirGraphStEphFloat<V> = LabDirGraphStEph<V, OrderedF64>;WeightedDirGraphStEphFloat39,1499
    impl<V: StT + Hash> WeightedDirGraphStEphFloat<V> {WeightedDirGraphStEphFloat42,1665
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges44,1773
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge59,2326
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight64,2530
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges69,2742
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted78,3105
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted89,3532
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight100,3956
        pub fn edges_above_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_above_weight108,4233
        pub fn edges_below_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_below_weight119,4706
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge130,5165
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge138,5462
        pub fn scale_weights(&mut self, factor: OrderedFloat<f64>) {scale_weights146,5761
    macro_rules! WeightedDirGraphStEphFloatLit {WeightedDirGraphStEphFloatLit164,6384
    fn _WeightedDirGraphStEphFloatLit_type_checks() {_WeightedDirGraphStEphFloatLit_type_checks176,6997
    pub fn __weighted_dir_graph_st_float_macro_typecheck_exercise() {__weighted_dir_graph_st_float_macro_typecheck_exercise182,7299

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphInt.rs,1232
pub mod WeightedDirGraphMtEphInt {WeightedDirGraphMtEphInt4,190
    pub type WeightedDirGraphMtEphInt<V> = LabDirGraphMtEph<V, i32>;WeightedDirGraphMtEphInt13,529
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphInt<V> {WeightedDirGraphMtEphInt16,696
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges18,808
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(add_weighted_edge33,1347
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(fromget_edge_weight36,1517
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges39,1695
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted48,2050
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted59,2466
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() total_weight70,2879
    macro_rules! WeightedDirGraphMtEphIntLit {WeightedDirGraphMtEphIntLit74,3004
    pub fn __weighted_dir_graph_mt_int_macro_typecheck_exercise() {__weighted_dir_graph_mt_int_macro_typecheck_exercise86,3590

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphStEph.rs,2680
pub mod LabUnDirGraphStEph {LabUnDirGraphStEph4,186
    pub struct LabUnDirGraphStEph<V, L>LabUnDirGraphStEph13,419
        vertices: Set<V>,vertices18,527
        labeled_edges: Set<LabEdge<V, L>>,labeled_edges19,553
    pub trait LabUnDirGraphStEphTrait<V, L>LabUnDirGraphStEphTrait22,603
        fn empty() -> Self;empty27,715
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges28,743
        fn vertices(&self) -> &Set<V>;vertices29,848
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;labeled_edges30,887
        fn edges(&self) -> Set<Edge<V>>;edges31,943
        fn add_vertex(&mut self, v: V);add_vertex32,984
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);add_labeled_edge33,1024
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;get_edge_label34,1088
        fn has_edge(&self, v1: &V, v2: &V) -> bool;has_edge35,1152
        fn neighbors(&self, v: &V) -> Set<V>;neighbors36,1204
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;normalize_edge37,1250
    impl<V, L> LabUnDirGraphStEphTrait<V, L> for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph40,1315
        fn empty() -> Self {empty45,1457
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges52,1629
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices59,1850
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> { &self.labeled_edges }labeled_edges61,1908
        fn edges(&self) -> Set<Edge<V>> {edges63,1988
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex71,2259
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge73,2328
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label84,2714
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge95,3151
        fn neighbors(&self, v: &V) -> Set<V> {neighbors106,3560
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge118,3993
    impl<V, L> Display for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph126,4413
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt131,4533
    impl<V, L> Debug for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph136,4695
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt141,4813
    macro_rules! LabUnDirGraphStEphLit {LabUnDirGraphStEphLit151,5085
    fn _LabUnDirGraphStEphLit_type_checks() {_LabUnDirGraphStEphLit_type_checks174,6291
    pub fn __lab_undir_graph_macro_typecheck_exercise() {__lab_undir_graph_macro_typecheck_exercise180,6570

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphMtEph.rs,2627
pub mod LabDirGraphMtEph {LabDirGraphMtEph4,208
    pub struct LabDirGraphMtEph<V, L>LabDirGraphMtEph13,439
        vertices: Set<V>,vertices18,550
        labeled_arcs: Set<LabEdge<V, L>>,labeled_arcs19,576
    pub trait LabDirGraphMtEphTrait<V, L>LabDirGraphMtEphTrait22,625
        fn empty() -> Self;empty27,740
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs28,768
        fn vertices(&self) -> &Set<V>;vertices29,871
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;labeled_arcs30,910
        fn arcs(&self) -> Set<Edge<V>>;arcs31,965
        fn add_vertex(&mut self, v: V);add_vertex32,1005
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);add_labeled_arc33,1045
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;get_arc_label34,1110
        fn has_arc(&self, from: &V, to: &V) -> bool;has_arc35,1175
        fn out_neighbors(&self, v: &V) -> Set<V>;out_neighbors36,1228
        fn in_neighbors(&self, v: &V) -> Set<V>;in_neighbors37,1278
    impl<V, L> LabDirGraphMtEphTrait<V, L> for LabDirGraphMtEph<V, L>LabDirGraphMtEph40,1334
        fn empty() -> Self {empty45,1477
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs52,1646
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices56,1817
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }labeled_arcs58,1875
        fn arcs(&self) -> Set<Edge<V>> {arcs60,1953
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex68,2222
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc70,2291
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label76,2532
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc85,2833
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors94,3107
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors104,3432
    impl<V, L> Display for LabDirGraphMtEph<V, L>LabDirGraphMtEph115,3762
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt120,3885
    impl<V, L> Debug for LabDirGraphMtEph<V, L>LabDirGraphMtEph125,4044
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt130,4165
    macro_rules! LabDirGraphMtEphLit {LabDirGraphMtEphLit140,4433
    fn _LabDirGraphMtEphLit_type_checks() {_LabDirGraphMtEphLit_type_checks152,5225
    pub fn __lab_dir_graph_mt_macro_typecheck_exercise() {__lab_dir_graph_mt_macro_typecheck_exercise158,5496

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap11/FibonacciMt.rs,579
pub mod FibonacciMt {FibonacciMt5,223
    pub struct FibonacciMt;FibonacciMt8,290
    pub trait FibonacciMtTrait {FibonacciMtTrait10,319
        fn fib(n: N) -> N;fib11,352
    impl FibonacciMt {FibonacciMt14,386
        pub fn fib(n: N) -> N {fib15,409
    impl FibonacciMtTrait for FibonacciMt {FibonacciMt25,681
        fn fib(n: N) -> N {fib26,725
    mod tests {tests32,819
        fn fib_base_cases() {fib_base_cases37,940
        fn fib_small_values() {fib_small_values43,1093
        fn trait_and_inherent_agree() {trait_and_inherent_agree50,1297

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap03/InsertionSortSt.rs,312
pub mod InsertionSortSt {InsertionSortSt4,143
    pub trait InsertionSortStTrait<T: Ord + Clone> {InsertionSortStTrait6,170
        fn insSort(&self, slice: &mut [T]);insSort9,322
    impl<T: Ord + Clone> InsertionSortStTrait<T> for T {T12,373
        fn insSort(&self, slice: &mut [T]) {insSort13,430

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTTreapMtEph.rs,4435
pub mod BSTTreapMtEph {BSTTreapMtEph4,192
    type Link<T> = Option<Box<Node<T>>>;Link11,366
    struct Node<T: StTInMtT + Ord> {Node14,436
        key: T,key15,473
        priority: u64,priority16,489
        size: N,size17,512
        left: Link<T>,left18,529
        right: Link<T>,right19,552
    impl<T: StTInMtT + Ord> Node<T> {Node22,583
        fn new(key: T, priority: u64) -> Self {new23,621
    pub struct BSTTreapMtEph<T: StTInMtT + Ord> {BSTTreapMtEph35,876
        root: Arc<RwLock<Link<T>>>,root36,926
    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;BSTreeTreap39,969
    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTTreapMtEphTrait41,1018
        fn new() -> Self;new42,1079
        fn insert(&self, value: T);insert43,1105
        fn find(&self, target: &T) -> Option<T>;find44,1141
        fn contains(&self, target: &T) -> B;contains45,1190
        fn size(&self) -> N;size46,1235
        fn is_empty(&self) -> B;is_empty47,1264
        fn height(&self) -> N;height48,1297
        fn minimum(&self) -> Option<T>;minimum49,1328
        fn maximum(&self) -> Option<T>;maximum50,1368
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order51,1408
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order52,1457
    impl<T: StTInMtT + Ord> Default for BSTTreapMtEph<T> {BSTTreapMtEph55,1514
        fn default() -> Self { Self::new() }default56,1573
    impl<T: StTInMtT + Ord> BSTTreapMtEph<T> {BSTTreapMtEph59,1625
        pub fn new() -> Self {new60,1672
        pub fn size(&self) -> N {size66,1807
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty71,1940
        pub fn height(&self) -> N {height73,2031
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec74,2067
        pub fn insert(&self, value: T) {insert85,2417
        pub fn find(&self, target: &T) -> Option<T> {find91,2619
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains96,2789
        pub fn minimum(&self) -> Option<T> {minimum98,2903
        pub fn maximum(&self) -> Option<T> {maximum103,3055
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order108,3207
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order115,3492
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link122,3779
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate124,3862
        fn rotate_left(link: &mut Link<T>) {rotate_left126,3981
        fn rotate_right(link: &mut Link<T>) {rotate_right140,4436
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link154,4892
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link179,5912
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link194,6429
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link204,6750
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect214,7073
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect222,7360
    impl<T: StTInMtT + Ord> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {BSTTreapMtEph231,7656
        fn new() -> Self { BSTTreapMtEph::new() }new232,7729
        fn insert(&self, value: T) { BSTTreapMtEph::insert(self, value) }insert234,7780
        fn find(&self, target: &T) -> Option<T> { BSTTreapMtEph::find(self, target) }find236,7855
        fn contains(&self, target: &T) -> B { BSTTreapMtEph::contains(self, target) }contains238,7942
        fn size(&self) -> N { BSTTreapMtEph::size(self) }size240,8029
        fn is_empty(&self) -> B { BSTTreapMtEph::is_empty(self) }is_empty242,8088
        fn height(&self) -> N { BSTTreapMtEph::height(self) }height244,8155
        fn minimum(&self) -> Option<T> { BSTTreapMtEph::minimum(self) }minimum246,8218
        fn maximum(&self) -> Option<T> { BSTTreapMtEph::maximum(self) }maximum248,8291
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::in_order(self) }in_order250,8364
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::pre_order(self) }pre_order252,8447
    macro_rules! BSTTreapMtEphLit {BSTTreapMtEphLit256,8558
    fn _BSTTreapMtEphLit_type_checks() {_BSTTreapMtEphLit_type_checks268,9087

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTSetTreapMtEph.rs,5647
pub mod BSTSetTreapMtEph {BSTSetTreapMtEph4,167
    pub struct BSTSetTreapMtEph<T: StTInMtT + Ord> {BSTSetTreapMtEph12,404
        tree: BSTTreapMtEph<T>,tree13,457
    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;BSTSetTreapMt16,496
    pub trait BSTSetTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetTreapMtEphTrait18,550
        fn empty() -> Self;empty19,614
        fn singleton(value: T) -> Self;singleton20,642
        fn size(&self) -> N;size21,682
        fn is_empty(&self) -> B;is_empty22,711
        fn find(&self, value: &T) -> Option<T>;find23,744
        fn contains(&self, value: &T) -> B;contains24,792
        fn minimum(&self) -> Option<T>;minimum25,836
        fn maximum(&self) -> Option<T>;maximum26,876
        fn insert(&mut self, value: T);insert27,916
        fn delete(&mut self, target: &T);delete28,956
        fn union(&self, other: &Self) -> Self;union29,998
        fn intersection(&self, other: &Self) -> Self;intersection30,1045
        fn difference(&self, other: &Self) -> Self;difference31,1099
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1151
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1206
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1261
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1323
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1393
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1461
        fn as_tree(&self) -> &BSTTreapMtEph<T>;as_tree38,1515
    impl<T: StTInMtT + Ord> BSTSetTreapMtEph<T> {BSTSetTreapMtEph41,1570
        pub fn empty() -> Self {empty42,1620
        pub fn singleton(value: T) -> Self {singleton48,1741
        pub fn size(&self) -> N { self.tree.size() }size54,1900
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1954
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2016
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2094
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2172
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2240
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2308
        pub fn delete(&mut self, target: &T) {delete68,2381
        pub fn union(&self, other: &Self) -> Self {union76,2670
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2969
        pub fn difference(&self, other: &Self) -> Self {difference101,3547
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4124
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4815
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5128
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5484
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5893
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6157
        pub fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree180,6240
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6307
        fn rebuild_from_vec(values: Vec<T>) -> BSTTreapMtEph<T> {rebuild_from_vec184,6398
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6621
    impl<T: StTInMtT + Ord> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {BSTSetTreapMtEph204,6906
        fn empty() -> Self { Self::empty() }empty205,6985
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7031
        fn size(&self) -> N { self.tree.size() }size209,7098
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7148
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7206
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7280
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7354
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7418
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7482
        fn delete(&mut self, target: &T) { BSTSetTreapMtEph::delete(self, target) }delete223,7551
        fn union(&self, other: &Self) -> Self { BSTSetTreapMtEph::union(self, other) }union225,7636
        fn intersection(&self, other: &Self) -> Self { BSTSetTreapMtEph::intersection(self, otheintersection227,7724
        fn difference(&self, other: &Self) -> Self { BSTSetTreapMtEph::difference(self, other) }difference229,7826
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetTreapMtEph::split(self, pivot) }split231,7924
        fn join_pair(left: Self, right: Self) -> Self { BSTSetTreapMtEph::join_pair(left, right)join_pair233,8020
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetTreapMtEph::join_m(left, pijoin_m235,8120
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetTreapMtEph::filter(filter237,8231
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetTreapMtEph::reduce(sereduce239,8347
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8460
        fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree243,8539
    macro_rules! BSTSetTreapMtEphLit {BSTSetTreapMtEphLit247,8628
    fn _BSTSetTreapMtEphLit_type_checks() {_BSTSetTreapMtEphLit_type_checks259,9201

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTParaTreapMtEph.rs,4882
pub mod BSTParaTreapMtEph {BSTParaTreapMtEph4,188
    pub enum Exposed<T: StTInMtT + Ord> {Exposed13,421
        Leaf,Leaf14,463
        Node(ParamTreap<T>, T, ParamTreap<T>),Node15,477
    struct NodeInner<T: StTInMtT + Ord> {NodeInner19,552
        key: T,key20,594
        priority: i64,priority21,610
        size: N,size22,633
        left: ParamTreap<T>,left23,650
        right: ParamTreap<T>,right24,679
    pub struct ParamTreap<T: StTInMtT + Ord> {ParamTreap28,737
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root29,784
    fn priority_for<T: StTInMtT + Ord>(key: &T) -> i64 {priority_for32,845
    fn tree_priority<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> i64 {tree_priority40,1138
    fn tree_size<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> N {tree_size45,1325
    fn make_node<T: StTInMtT + Ord>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreamake_node50,1495
    impl<T: StTInMtT + Ord + 'static> ParamTreap<T> {ParamTreap63,1913
        fn expose_internal(&self) -> Exposed<T> {expose_internal66,2058
        pub fn expose_with_priority(&self) -> Option<(ParamTreap<T>, T, i64, ParamTreap<T>)> {expose_with_priority76,2448
        fn join_with_priority(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) join_with_priority85,2922
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid108,4099
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner120,4575
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner141,5706
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner155,6460
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner171,7204
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner191,8134
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner211,9047
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel235,10160
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner245,10524
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel269,11597
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order279,11938
    pub trait ParamTreapTrait<T: StTInMtT + Ord + 'static>: Sized {ParamTreapTrait291,12344
        fn new() -> Self;new294,12503
        fn expose(&self) -> Exposed<T>;expose297,12620
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid300,12751
        fn size(&self) -> N;size303,12892
        fn is_empty(&self) -> B;is_empty306,13012
        fn insert(&self, key: T);insert309,13156
        fn delete(&self, key: &T);delete312,13301
        fn find(&self, key: &T) -> Option<T>;find315,13447
        fn split(&self, key: &T) -> (Self, B, Self);split318,13604
        fn join_pair(&self, other: Self) -> Self;join_pair321,13816
        fn union(&self, other: &Self) -> Self;union324,13991
        fn intersect(&self, other: &Self) -> Self;intersect327,14163
        fn difference(&self, other: &Self) -> Self;difference330,14339
        fn filter<F>(&self, predicate: F) -> Selffilter333,14496
        fn reduce<F>(&self, op: F, base: T) -> Treduce338,14720
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order343,14936
    impl<T: StTInMtT + Ord + 'static> ParamTreapTrait<T> for ParamTreap<T> {ParamTreap346,14992
        fn new() -> Self {new349,15160
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose357,15379
        fn join_mid(exposed: Exposed<T>) -> Self { ParamTreap::join_mid(exposed) }join_mid361,15537
        fn size(&self) -> N { tree_size(self) }size365,15712
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty369,15852
        fn insert(&self, key: T) {insert373,16050
        fn delete(&self, key: &T) {delete384,16566
        fn find(&self, key: &T) -> Option<T> {find394,17015
        fn split(&self, key: &T) -> (Self, B, Self) { ParamTreap::split_inner(self, key) }split407,17627
        fn join_pair(&self, other: Self) -> Self { ParamTreap::join_pair_inner(self.clone(), othjoin_pair411,17878
        fn union(&self, other: &Self) -> Self { ParamTreap::union_inner(self, other) }union415,18106
        fn intersect(&self, other: &Self) -> Self { ParamTreap::intersect_inner(self, other) }intersect419,18319
        fn difference(&self, other: &Self) -> Self { ParamTreap::difference_inner(self, other) }difference423,18540
        fn filter<F>(&self, predicate: F) -> Selffilter427,18743
        fn reduce<F>(&self, op: F, base: T) -> Treduce436,19045
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order445,19338
    macro_rules! ParamTreapLit {ParamTreapLit453,19584
    fn _ParamTreapLit_type_checks() {_ParamTreapLit_type_checks465,20130

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTTreapStEph.rs,4545
pub mod BSTTreapStEph {BSTTreapStEph4,164
    type Link<T> = Option<Box<Node<T>>>;Link10,359
    struct Node<T: StT + Ord> {Node14,443
        key: T,key15,475
        priority: u64,priority16,491
        size: N,size17,514
        left: Link<T>,left18,531
        right: Link<T>,right19,554
    impl<T: StT + Ord> Node<T> {Node22,585
        fn new(key: T, priority: u64) -> Self {new23,618
    pub struct BSTTreapStEph<T: StT + Ord> {BSTTreapStEph35,873
        root: Link<T>,root36,918
    pub type BSTreeTreap<T> = BSTTreapStEph<T>;BSTreeTreap39,948
    pub trait BSTTreapStEphTrait<T: StT + Ord> {BSTTreapStEphTrait41,997
        fn new() -> Self;new42,1046
        fn size(&self) -> N;size43,1072
        fn is_empty(&self) -> B;is_empty44,1101
        fn height(&self) -> N;height45,1134
        fn insert(&mut self, value: T);insert46,1165
        fn find(&self, target: &T) -> Option<&T>;find47,1205
        fn contains(&self, target: &T) -> B;contains48,1255
        fn minimum(&self) -> Option<&T>;minimum49,1300
        fn maximum(&self) -> Option<&T>;maximum50,1341
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order51,1382
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order52,1431
    impl<T: StT + Ord> Default for BSTreeTreap<T> {BSTreeTreap55,1488
        fn default() -> Self { Self::new() }default56,1540
    impl<T: StT + Ord> BSTTreapStEph<T> {BSTTreapStEph59,1592
        pub fn new() -> Self { BSTTreapStEph { root: None } }new60,1634
        pub fn size(&self) -> N { Self::size_link(&self.root) }size62,1697
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty64,1762
        pub fn height(&self) -> N {height66,1853
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec67,1889
        pub fn insert(&mut self, value: T) {insert76,2185
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find81,2334
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains83,2428
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum85,2542
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum87,2618
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order89,2694
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order95,2918
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link101,3144
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate103,3227
        fn rotate_left(link: &mut Link<T>) {rotate_left105,3346
        fn rotate_right(link: &mut Link<T>) {rotate_right119,3801
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link133,4257
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link158,5277
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link173,5794
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link183,6115
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect193,6438
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect201,6725
    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {BSTTreapStEph210,7021
        fn new() -> Self { BSTTreapStEph::new() }new211,7089
        fn size(&self) -> N { BSTTreapStEph::size(self) }size213,7140
        fn is_empty(&self) -> B { BSTTreapStEph::is_empty(self) }is_empty215,7199
        fn height(&self) -> N { BSTTreapStEph::height(self) }height217,7266
        fn insert(&mut self, value: T) { BSTTreapStEph::insert(self, value) }insert219,7329
        fn find(&self, target: &T) -> Option<&T> { BSTTreapStEph::find(self, target) }find221,7408
        fn contains(&self, target: &T) -> B { BSTTreapStEph::contains(self, target) }contains223,7496
        fn minimum(&self) -> Option<&T> { BSTTreapStEph::minimum(self) }minimum225,7583
        fn maximum(&self) -> Option<&T> { BSTTreapStEph::maximum(self) }maximum227,7657
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTTreapStEph::in_order(self) }in_order229,7731
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTTreapStEph::pre_order(self) }pre_order231,7814
    macro_rules! BSTTreapStEphLit {BSTTreapStEphLit235,7925
    fn _BSTTreapStEphLit_type_checks() {_BSTTreapStEphLit_type_checks247,8458

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,6138
pub mod Types {Types5,137
    pub type N = usize;N11,273
    pub enum B {B15,416
        True,True16,433
        False,False17,447
    impl std::fmt::Display for B {B24,680
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt25,715
    impl std::ops::Not for B {B33,946
        type Output = B;Output34,977
        fn not(self) -> B {not35,1002
    pub trait StT: Eq + Clone + Display + Debug + Sized {}StT45,1278
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}T46,1337
    pub trait StTInMtT: StT + Send + Sync {}StTInMtT49,1501
    impl<T> StTInMtT for T where T: StT + Send + Sync {}T50,1546
    pub trait MtT: Sized + Send + Sync {MtT54,1740
        type Inner: StT;Inner55,1781
        fn clone_mt(&self) -> Self;clone_mt56,1806
        fn new_mt(inner: Self::Inner) -> Self;new_mt57,1842
    impl<T: StT + Send> MtT for std::sync::Mutex<T> {Mutex60,1896
        type Inner = T;Inner61,1950
        fn clone_mt(&self) -> Self {clone_mt62,1974
        fn new_mt(inner: Self::Inner) -> Self { std::sync::Mutex::new(inner) }new_mt66,2116
    impl<A: StT + Send + Sync, B: StT + Send + Sync> MtT for Pair<A, B> {Pair69,2202
        type Inner = Pair<A, B>;Inner70,2276
        fn clone_mt(&self) -> Self { self.clone() }clone_mt71,2309
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt72,2361
    impl MtT for usize {usize76,2502
        type Inner = usize;Inner77,2527
        fn clone_mt(&self) -> Self { *self }clone_mt78,2555
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt79,2600
    impl MtT for isize {isize82,2663
        type Inner = isize;Inner83,2688
        fn clone_mt(&self) -> Self { *self }clone_mt84,2716
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt85,2761
    impl MtT for i32 {i3288,2824
        type Inner = i32;Inner89,2847
        fn clone_mt(&self) -> Self { *self }clone_mt90,2873
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt91,2918
    impl MtT for u32 {u3294,2981
        type Inner = u32;Inner95,3004
        fn clone_mt(&self) -> Self { *self }clone_mt96,3030
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt97,3075
    impl MtT for i64 {i64100,3138
        type Inner = i64;Inner101,3161
        fn clone_mt(&self) -> Self { *self }clone_mt102,3187
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt103,3232
    impl MtT for u64 {u64106,3295
        type Inner = u64;Inner107,3318
        fn clone_mt(&self) -> Self { *self }clone_mt108,3344
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt109,3389
    impl MtT for bool {bool112,3452
        type Inner = bool;Inner113,3476
        fn clone_mt(&self) -> Self { *self }clone_mt114,3503
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt115,3548
    impl MtT for char {char118,3611
        type Inner = char;Inner119,3635
        fn clone_mt(&self) -> Self { *self }clone_mt120,3662
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt121,3707
    impl MtT for String {String125,3824
        type Inner = String;Inner126,3850
        fn clone_mt(&self) -> Self { self.clone() }clone_mt127,3879
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt128,3931
    impl<'a> MtT for &'a str {str132,4029
        type Inner = &'a str;Inner133,4060
        fn clone_mt(&self) -> Self { *self }clone_mt134,4090
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt135,4135
    impl MtT for B {B139,4240
        type Inner = B;Inner140,4261
        fn clone_mt(&self) -> Self { *self }clone_mt141,4285
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt142,4330
    pub struct Edge<V: StT>(pub V, pub V);Edge147,4532
    impl<V: StT> std::fmt::Display for Edge<V> {Edge149,4576
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})fmt150,4625
    impl<V: StT> From<(V, V)> for Edge<V> {Edge153,4749
        fn from(t: (V, V)) -> Self { Edge(t.0, t.1) }from154,4793
    impl<V: StT> From<Edge<V>> for (V, V) {V157,4854
        fn from(e: Edge<V>) -> (V, V) { (e.0, e.1) }from158,4898
    pub struct LabEdge<V: StT, L: StT + Hash>(pub V, pub V, pub L);LabEdge163,5071
    impl<V: StT, L: StT + Hash> std::fmt::Display for LabEdge<V, L> {LabEdge165,5140
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt166,5210
    impl<V: StT, L: StT + Hash> From<(V, V, L)> for LabEdge<V, L> {LabEdge171,5366
        fn from(t: (V, V, L)) -> Self { LabEdge(t.0, t.1, t.2) }from172,5434
    impl<V: StT, L: StT + Hash> From<LabEdge<V, L>> for (V, V, L) {L175,5506
        fn from(e: LabEdge<V, L>) -> (V, V, L) { (e.0, e.1, e.2) }from176,5574
    pub type OrderedF32 = OrderedFloat<f32>;OrderedF32183,5801
    pub type OrderedF64 = OrderedFloat<f64>;OrderedF64184,5846
    pub struct Pair<A, B>(pub A, pub B);Pair188,6046
    impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {Pair190,6088
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})fmt191,6176
    impl<A, B> From<(A, B)> for Pair<A, B> {Pair194,6300
        fn from(t: (A, B)) -> Self { Pair(t.0, t.1) }from195,6345
    impl<A, B> From<Pair<A, B>> for (A, B) {B198,6406
        fn from(p: Pair<A, B>) -> Self { (p.0, p.1) }from199,6451
    macro_rules! ParaPair {ParaPair203,6532
    fn _ParaPair_type_checks() {_ParaPair_type_checks214,7006
    pub fn ArraySeqSetEq<T: PartialEq>(a_len: N, a_nth: fn(N) -> T, b_len: N, b_nth: fn(N) -> T)ArraySeqSetEq223,7426
    macro_rules! EdgeLit {EdgeLit262,8474
    macro_rules! PairLit {PairLit269,8618
    macro_rules! EdgeList {EdgeList276,8762
    macro_rules! PairList {PairList286,8979
    fn _EdgeLit_type_checks() {_EdgeLit_type_checks296,9200
    fn _PairLit_type_checks() {_PairLit_type_checks302,9387
    fn _EdgeList_type_checks() {_EdgeList_type_checks308,9579
    fn _PairList_type_checks() {_PairList_type_checks314,9788

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_5.rs,1374
pub mod Exercise12_5 {Exercise12_54,178
    struct Node<T: StTInMtT> {Node11,352
        value: ManuallyDrop<T>,value12,383
        next: *mut Node<T>,next13,415
    pub struct ConcurrentStackMt<T: StTInMtT> {ConcurrentStackMt18,532
        head: AtomicPtr<Node<T>>,head19,580
    pub trait ConcurrentStackMtTrait<T: StTInMtT> {ConcurrentStackMtTrait22,621
        fn new() -> Self;new23,673
        fn push(&self, value: T);push24,699
        fn pop(&self) -> Option<T>;pop25,733
        fn is_empty(&self) -> bool;is_empty26,769
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt29,812
        fn raw_pop(&self) -> Option<*mut Node<T>> {raw_pop30,857
    impl<T: StTInMtT> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {ConcurrentStackMt48,1406
        fn new() -> Self {new49,1481
        fn push(&self, value: T) {push53,1586
        fn pop(&self) -> Option<T> {pop70,2223
        fn is_empty(&self) -> bool {is_empty77,2462
    impl<T: StTInMtT> Default for ConcurrentStackMt<T> {ConcurrentStackMt82,2572
        fn default() -> Self { Self::new() }default83,2629
    impl<T: StTInMtT> Drop for ConcurrentStackMt<T> {ConcurrentStackMt86,2681
        fn drop(&mut self) {drop87,2735
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt100,3139
        pub fn drain(&self) -> Vec<T> {drain102,3259

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_2.rs,478
pub mod Exercise12_2 {Exercise12_24,175
    pub trait FetchAddCasTrait {FetchAddCasTrait7,251
        fn fetch_add_cas(&self, delta: usize) -> usize;fetch_add_cas8,284
    impl FetchAddCasTrait for AtomicUsize {AtomicUsize11,347
        fn fetch_add_cas(&self, delta: usize) -> usize {fetch_add_cas12,391
    pub fn fetch_add_cas(target: &AtomicUsize, delta: usize) -> usize {fetch_add_cas24,845
    pub fn efficiency_note() -> &'static str {efficiency_note28,960

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_1.rs,1019
pub mod Exercise12_1 {Exercise12_14,164
    pub struct SpinLock {SpinLock13,369
        ticket: AtomicUsize,ticket14,395
        turn: AtomicUsize,turn15,424
    pub trait SpinLockTrait {SpinLockTrait18,458
        fn new() -> Self;new19,488
        fn lock(&self);lock20,514
        fn unlock(&self);unlock21,538
    impl SpinLock {SpinLock24,571
        pub fn new() -> Self {new25,591
        pub fn lock(&self) {lock29,713
        pub fn unlock(&self) {unlock36,936
        pub fn with_lock<T>(&self, action: impl FnOnce() -> T) -> T {with_lock40,1033
    impl SpinLockTrait for SpinLock {SpinLock48,1226
        fn new() -> Self { SpinLock::new() }new49,1264
        fn lock(&self) { SpinLock::lock(self) }lock51,1310
        fn unlock(&self) { SpinLock::unlock(self) }unlock53,1359
    impl Default for SpinLock {SpinLock56,1418
        fn default() -> Self { SpinLock::new() }default57,1450
    pub fn parallel_increment(iterations: N) -> usize {parallel_increment60,1506

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortMtSlice.rs,1389
pub mod Chapter36MtSlice {Chapter36MtSlice4,190
    pub trait Chapter36MtSliceTrait<T: StT + Ord + Send> {Chapter36MtSliceTrait14,398
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first15,457
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median316,510
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random17,565
        fn quick_sort_mt_first(&self);quick_sort_mt_first19,620
        fn quick_sort_mt_median3(&self);quick_sort_mt_median320,659
        fn quick_sort_mt_random(&self);quick_sort_mt_random21,700
    impl<T: StT + Ord + Send + Sync> Chapter36MtSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS24,747
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first25,838
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median327,916
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random41,1387
        fn quick_sort_mt_first(&self) {quick_sort_mt_first47,1563
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort52,1723
        fn quick_sort_mt_median3(&self) {quick_sort_mt_median389,3086
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort94,3248
        fn quick_sort_mt_random(&self) {quick_sort_mt_random144,5176
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort149,5337

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortSt.rs,1497
pub mod Chapter36St {Chapter36St4,187
    pub trait Chapter36StTrait<T: StT + Ord> {Chapter36StTrait10,325
        fn pivot_st_first(&self, lo: N, hi: N) -> T;pivot_st_first11,372
        fn pivot_st_median3(&self, lo: N, hi: N) -> T;pivot_st_median312,425
        fn pivot_st_random(&self, lo: N, hi: N) -> T;pivot_st_random13,480
        fn quick_sort_st_first(&mut self);quick_sort_st_first15,535
        fn quick_sort_st_median3(&mut self);quick_sort_st_median316,578
        fn quick_sort_st_random(&mut self);quick_sort_st_random17,623
    impl<T: StT + Ord> Chapter36StTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS20,674
        fn pivot_st_first(&self, lo: N, _hi: N) -> T { self.nth(lo).clone() }pivot_st_first21,741
        fn pivot_st_median3(&self, lo: N, hi: N) -> T {pivot_st_median322,819
        fn pivot_st_random(&self, lo: N, hi: N) -> T {pivot_st_random35,1292
        fn quick_sort_st_first(&mut self) {quick_sort_st_first41,1469
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort42,1513
        fn quick_sort_st_median3(&mut self) {quick_sort_st_median376,2696
            fn median3<T: StT + Ord>(a: &ArraySeqStEphS<T>, lo: N, hi: N) -> T {median377,2742
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort90,3279
        fn quick_sort_st_random(&mut self) {quick_sort_st_random124,4463
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort125,4508

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortMt.rs,1407
pub mod Chapter36Mt {Chapter36Mt4,186
    pub trait Chapter36MtTrait<T: StT + Ord + Send> {Chapter36MtTrait26,1277
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first27,1331
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median328,1384
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random29,1439
        fn quick_sort_mt_first(&mut self);quick_sort_mt_first31,1494
        fn quick_sort_mt_median3(&mut self);quick_sort_mt_median332,1537
        fn quick_sort_mt_random(&mut self);quick_sort_mt_random33,1582
    impl<T: StT + Ord + Send> Chapter36MtTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS36,1633
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first37,1707
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median338,1784
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random51,2254
        fn quick_sort_mt_first(&mut self) {quick_sort_mt_first57,2430
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort62,2554
        fn quick_sort_mt_median3(&mut self) {quick_sort_mt_median3100,3900
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort105,4026
        fn quick_sort_mt_random(&mut self) {quick_sort_mt_random155,5839
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort160,5964

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main8,328

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/MappingStEph.rs,2414
pub mod MappingStEph {MappingStEph4,164
    pub struct Mapping<A, B> {Mapping15,482
        rel: Relation<A, B>,rel16,513
    pub trait MappingStEphTrait<X: StT + Hash, Y: StT + Hash> {MappingStEphTrait19,549
        fn empty() -> Mapping<X, Y>;empty22,705
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y>;FromVec26,839
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation30,993
        fn size(&self) -> N;size34,1148
        fn domain(&self) -> Set<X>;domain38,1274
        fn range(&self) -> Set<Y>;range42,1407
        fn mem(&self, a: &X, b: &Y) -> B;mem46,1535
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>>;iter48,1578
    impl<A: Eq + Hash, B: Eq + Hash> Mapping<A, B> {Mapping51,1661
        fn unique_pairs_from_iter<I: IntoIterator<Item = Pair<A, B>>>(iter: I) -> Set<Pair<A, B>unique_pairs_from_iter52,1714
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Mapping<A, B> {Mapping62,2092
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq63,2161
    impl<A: StT + Hash, B: StT + Hash> Eq for Mapping<A, B> {}Mapping65,2236
    impl<A: StT + Hash, B: StT + Hash> Debug for Mapping<A, B> {Mapping67,2300
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.rel, f) }fmt68,2365
    impl<A: StT + Hash, B: StT + Hash> Display for Mapping<A, B> {Mapping70,2455
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.rel, f) }fmt71,2522
    impl<X: StT + Hash, Y: StT + Hash> MappingStEphTrait<X, Y> for Mapping<X, Y> {Mapping74,2615
        fn empty() -> Mapping<X, Y> {empty75,2698
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y> {FromVec81,2859
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation88,3104
        fn size(&self) -> N { self.rel.size() }size95,3370
        fn domain(&self) -> Set<X> { self.rel.domain() }domain97,3419
        fn range(&self) -> Set<Y> { self.rel.range() }range99,3477
        fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem101,3533
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.rel.iter() }iter103,3598
    macro_rules! MappingLit {MappingLit107,3720
    fn _MappingLit_type_checks() {_MappingLit_type_checks118,4285
    pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise124,4516

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/SetStEph.rs,3623
pub mod SetStEph {SetStEph4,161
    pub struct Set<T> {Set13,342
        data: HashSet<T>,data14,366
    pub trait SetStEphTrait<T: StT + Hash> {SetStEphTrait17,399
        fn empty() -> Set<T>;empty20,536
        fn singleton(x: T) -> Set<T>;singleton23,658
        fn size(&self) -> N;size26,788
        fn mem(&self, x: &T) -> B;mem29,909
        fn union(&self, other: &Set<T>) -> Set<T>;union32,1052
        fn intersection(&self, other: &Set<T>) -> Set<T>;intersection35,1211
        fn partition(&self, parts: &Set<Set<T>>) -> B;partition38,1391
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>;CartesianProduct42,1557
        fn insert(&mut self, x: T) -> &mut Self;insert46,1736
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;iter50,1878
        fn FromVec(v: Vec<T>) -> Set<T>;FromVec53,2041
    impl<T: Eq + Hash> PartialEq for Set<T> {Set56,2089
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq57,2135
    impl<T: Eq + Hash> Eq for Set<T> {}Set60,2213
    impl<T: Eq + Hash> std::fmt::Debug for Set<T>Set62,2254
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt66,2348
    impl<T: Eq + Hash> std::fmt::Display for Set<T>Set71,2503
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt75,2601
    impl<T: Eq + Hash> Hash for Set<T> {Set91,3107
        fn hash<H: Hasher>(&self, state: &mut H) {hash92,3148
    impl<T: Eq + Hash> Set<T> {Set108,3712
        pub fn empty() -> Set<T> { Set { data: HashSet::new() } }empty109,3744
        pub fn singleton(x: T) -> Set<T> {singleton111,3811
        pub fn size(&self) -> N { self.data.len() }size117,3977
        pub fn mem(&self, x: &T) -> B { if self.data.contains(x) { B::True } else { B::False } }mem119,4030
        pub fn union(&self, other: &Set<T>) -> Set<T>union121,4128
        pub fn intersection(&self, other: &Set<T>) -> Set<T>intersection132,4402
        pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition143,4759
        pub fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct161,5314
        pub fn insert(&mut self, x: T) -> &mut Self {insert175,5760
        pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter180,5883
        pub fn FromVec(v: Vec<T>) -> Set<T> {FromVec182,5975
    impl<T: StT + Hash> SetStEphTrait<T> for Set<T> {Set191,6199
        fn empty() -> Set<T> { Set { data: HashSet::new() } }empty192,6253
        fn singleton(x: T) -> Set<T> {singleton194,6316
        fn size(&self) -> N { self.data.len() }size200,6478
        fn mem(&self, x: &T) -> B { if self.data.contains(x) { B::True } else { B::False } }mem202,6527
        fn union(&self, other: &Set<T>) -> Set<T>union204,6621
        fn intersection(&self, other: &Set<T>) -> Set<T>intersection215,6891
        fn partition(&self, parts: &Set<Set<T>>) -> B {partition226,7244
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct244,7795
        fn insert(&mut self, x: T) -> &mut Self {insert258,8237
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter263,8356
        fn FromVec(v: Vec<T>) -> Set<T> {FromVec265,8444
    macro_rules! SetLit {SetLit275,8684
    fn _SetLit_type_checks() {_SetLit_type_checks287,9026
    pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise293,9222
        let _s0: Set<&'static str> = SetLit![];str294,9268

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/RelationStEph.rs,2235
pub mod RelationStEph {RelationStEph4,155
    pub struct Relation<A, B> {Relation15,426
        pairs: Set<Pair<A, B>>,pairs16,458
    pub trait RelationStEphTrait<X: StT + Hash, Y: StT + Hash> {RelationStEphTrait19,497
        fn empty() -> Relation<X, Y>;empty22,654
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y>;FromSet26,797
        fn size(&self) -> N;size30,952
        fn domain(&self) -> Set<X>domain34,1078
        fn range(&self) -> Set<Y>range40,1246
        fn mem(&self, a: &X, b: &Y) -> Bmem46,1409
        fn iter(&self) -> Iter<'_, Pair<X, Y>>;iter51,1509
    impl<A: StT + Hash, B: StT + Hash> Relation<A, B> {Relation54,1564
        pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B> { Relation { pairs: Set::FromVec(v)FromVec55,1620
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Relation<A, B> {Relation58,1728
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq59,1798
    impl<A: StT + Hash, B: StT + Hash> Eq for Relation<A, B> {}Relation62,1878
    impl<A: StT + Hash, B: StT + Hash> Debug for Relation<A, B> {Relation64,1943
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }fmt65,2009
    impl<A: StT + Hash, B: StT + Hash> Display for Relation<A, B> {Relation68,2112
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) fmt69,2180
    impl<X: StT + Hash, Y: StT + Hash> RelationStEphTrait<X, Y> for Relation<X, Y> {Relation72,2285
        fn empty() -> Relation<X, Y> { Relation { pairs: SetLit![] } }empty73,2370
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y> { Relation { pairs } }FromSet75,2442
        fn size(&self) -> N { self.pairs.size() }size77,2527
        fn domain(&self) -> Set<X>domain79,2578
        fn range(&self) -> Set<Y>range90,2845
        fn mem(&self, a: &X, b: &Y) -> Bmem101,3111
        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }iter113,3387
    macro_rules! RelationLit {RelationLit117,3483
    fn _RelationLit_type_checks() {_RelationLit_type_checks133,4360
    pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise139,4595

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap23/PrimTreeSeqSt.rs,987
pub mod PrimTreeSeqSt {PrimTreeSeqSt10,568
    pub enum PrimTreeSeqStTree<T: StT> {PrimTreeSeqStTree15,747
        Zero,Zero16,788
        One(T),One17,802
        Two(PrimTreeSeqStS<T>, PrimTreeSeqStS<T>),Two18,818
    pub struct PrimTreeSeqStS<T: StT> {PrimTreeSeqStS23,997
        data: Vec<T>,data24,1037
    impl<T: StT> PrimTreeSeqStS<T> {PrimTreeSeqStS27,1066
        pub fn empty() -> Self { Self { data: Vec::new() } }empty29,1142
        pub fn singleton(value: T) -> Self { Self { data: vec![value] } }singleton32,1263
        pub fn from_vec(vec: Vec<T>) -> Self { Self { data: vec } }from_vec35,1425
        pub fn into_vec(self) -> Vec<T> { self.data }into_vec38,1554
        pub fn as_slice(&self) -> &[T] { &self.data }as_slice41,1672
        pub fn length(&self) -> N { self.data.len() }length44,1787
        pub fn expose(&self) -> PrimTreeSeqStTree<T> {expose47,1919
        pub fn join(tree: PrimTreeSeqStTree<T>) -> Self {join61,2543

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap23/BBTEph.rs,2142
pub mod BBTEph {BBTEph4,148
    pub enum BBTree<T: StT> {BBTree10,358
        Leaf,Leaf11,388
        Node(Box<BBNode<T>>),Node12,402
    pub struct BBNode<T: StT> {BBNode16,482
        pub(crate) left: BBTree<T>,left17,514
        pub(crate) value: T,value18,550
        pub(crate) right: BBTree<T>,right19,579
    impl<T: StT> BBNode<T> {BBNode22,623
        fn new(left: BBTree<T>, value: T, right: BBTree<T>) -> Self { BBNode { left, value, righnew23,652
    pub trait BBTEphTrait<T: StT> {BBTEphTrait26,761
        fn leaf() -> Self;leaf27,797
        fn node(left: Self, value: T, right: Self) -> Self;node28,824
        fn is_leaf(&self) -> B;is_leaf29,884
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order30,916
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order31,965
        fn height(&self) -> N;height32,1015
        fn size(&self) -> N;size33,1046
    impl<T: StT> BBTree<T> {BBTree36,1082
        pub fn leaf() -> Self { BBTree::Leaf }leaf37,1111
        pub fn node(left: BBTree<T>, value: T, right: BBTree<T>) -> Self {node39,1159
        pub fn is_leaf(&self) -> B {is_leaf43,1313
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order50,1490
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order63,2158
        pub fn height(&self) -> N {height76,2827
        pub fn size(&self) -> N {size87,3162
    impl<T: StT> BBTEphTrait<T> for BBTree<T> {BBTree95,3371
        fn leaf() -> Self { BBTree::leaf() }leaf96,3419
        fn node(left: Self, value: T, right: Self) -> Self { BBTree::node(left, value, right) }node98,3465
        fn is_leaf(&self) -> B { BBTree::is_leaf(self) }is_leaf100,3562
        fn in_order(&self) -> ArraySeqStPerS<T> { BBTree::in_order(self) }in_order102,3620
        fn pre_order(&self) -> ArraySeqStPerS<T> { BBTree::pre_order(self) }pre_order104,3696
        fn height(&self) -> N { BBTree::height(self) }height106,3774
        fn size(&self) -> N { BBTree::size(self) }size108,3830
    macro_rules! BBNodeLit {BBNodeLit112,3908
    fn _BBNodeLit_type_checks() {_BBNodeLit_type_checks119,4134

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeq.rs,6339
pub mod ArraySeq {ArraySeq4,150
    pub struct ArraySeqS<T> {ArraySeqS12,332
        data: Box<[T]>,data13,362
    pub trait ArraySeq<T> {ArraySeq18,469
        fn new(length: N, init_value: T) -> ArraySeqS<T>new21,646
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str>;set27,843
        fn length(&self) -> N;length31,1046
        fn nth(&self, index: N) -> &T;nth35,1212
        fn empty() -> ArraySeqS<T>;empty39,1363
        fn singleton(item: T) -> ArraySeqS<T>;singleton43,1535
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqS<T>;tabulate47,1719
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> ArraySeqS<U>;map51,1902
        fn subseq(a: &ArraySeqS<T>, start: N, length: N) -> ArraySeqS<T>subseq55,2141
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> ArraySeqS<T>;append61,2369
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqS<T>, pred: &F) -> ArraySeqS<T>;filter65,2559
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> ArraySeqS<T>;flatten69,2771
        fn update(a: &ArraySeqS<T>, update: Pair<N, T>) -> ArraySeqS<T>;update73,2978
        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T>;inject77,3200
        fn isEmpty(&self) -> B;isEmpty81,3412
        fn isSingleton(&self) -> B;isSingleton85,3573
        fn collect<K: Clone + Eq, V: Clone>(collect89,3775
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqS<T>, f: &F, seed: A) -> A;iterate95,4019
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> Treduce99,4245
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (ArraySeqS<T>, T)scan105,4488
    impl<T: Clone> ArraySeqS<T> {ArraySeqS110,4620
        fn new(length: N, init_value: T) -> ArraySeqS<T>new111,4654
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {set118,4870
        fn length(&self) -> N { self.data.len() }length127,5156
        fn nth(&self, index: N) -> &T { &self.data[index] }nth129,5207
        fn empty() -> ArraySeqS<T> { ArraySeqS::from_vec(Vec::new()) }empty131,5268
        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::from_vec(vec![item]) }singleton133,5340
        fn isEmpty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }isEmpty135,5423
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton137,5513
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq142,5834
        pub fn subseq_copy(&self, start: N, length: N) -> ArraySeqS<T>subseq_copy151,6444
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqS<T> {update165,7126
        pub fn from_vec(elts: Vec<T>) -> ArraySeqS<T> {from_vec176,7656
        pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter182,7808
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }iter_mut184,7884
    impl<T: Clone> ArraySeq<T> for ArraySeqS<T> {ArraySeqS187,7981
        fn new(length: N, init_value: T) -> ArraySeqS<T>new188,8031
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {set193,8156
        fn length(&self) -> N { ArraySeqS::length(self) }length197,8303
        fn nth(&self, index: N) -> &T { ArraySeqS::nth(self, index) }nth199,8362
        fn empty() -> ArraySeqS<T> { ArraySeqS::empty() }empty201,8433
        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::singleton(item) }singleton203,8492
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqS<T> {tabulate205,8570
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> ArraySeqS<U> {map213,8839
        fn subseq(a: &ArraySeqS<T>, start: N, length: N) -> ArraySeqS<T>subseq222,9156
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> ArraySeqS<T>append227,9291
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqS<T>, pred: &F) -> ArraySeqS<T>filter243,9829
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> ArraySeqS<T>flatten255,10210
        fn update(a: &ArraySeqS<T>, Pair(index, item): Pair<N, T>) -> ArraySeqS<T>update267,10591
        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T>inject276,10915
        fn isEmpty(&self) -> B { ArraySeqS::isEmpty(self) }isEmpty289,11441
        fn isSingleton(&self) -> B { ArraySeqS::isSingleton(self) }isSingleton291,11502
        fn collect<K: Clone + Eq, V: Clone>(collect293,11571
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqS<T>, f: &F, seed: A) -> A {iterate316,12484
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> Treduce324,12718
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (ArraySeqS<T>, T)scan333,12952
    impl<T: PartialEq> PartialEq for ArraySeqS<T> {ArraySeqS346,13374
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq347,13426
    impl<T: Eq> Eq for ArraySeqS<T> {}ArraySeqS350,13504
    impl<T: Debug> Debug for ArraySeqS<T> {ArraySeqS352,13544
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt353,13588
    impl<T: Display> Display for ArraySeqS<T> {ArraySeqS356,13707
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt357,13755
    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {ArraySeqS369,14085
        type Item = &'a T;Item370,14137
        type IntoIter = std::slice::Iter<'a, T>;IntoIter371,14164
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }into_iter373,14214
    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {ArraySeqS376,14287
        type Item = &'a mut T;Item377,14343
        type IntoIter = std::slice::IterMut<'a, T>;IntoIter378,14374
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }into_iter380,14427
    impl<T> IntoIterator for ArraySeqS<T> {ArraySeqS383,14504
        type Item = T;Item384,14548
        type IntoIter = std::vec::IntoIter<T>;IntoIter385,14571
        fn into_iter(self) -> Self::IntoIter { Vec::from(self.data).into_iter() }into_iter387,14619
    macro_rules! ArraySeqS {ArraySeqS391,14728
    fn _arrayseqs_macro_type_checks() {_arrayseqs_macro_type_checks404,15164

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStPer.rs,5383
pub mod LinkedListStPer {LinkedListStPer4,140
    pub struct NodeP<T: StT> {NodeP10,263
        pub value: T,value11,294
        pub next: Option<Box<NodeP<T>>>,next12,316
    pub struct LinkedListStPerS<T: StT> {LinkedListStPerS16,392
        head: Option<Box<NodeP<T>>>,head17,434
        len: N,len18,471
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait21,494
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>new22,539
        fn empty() -> LinkedListStPerS<T>;empty25,639
        fn singleton(item: T) -> LinkedListStPerS<T>;singleton26,682
        fn length(&self) -> N;length27,736
        fn nth(&self, index: N) -> &T;nth28,767
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T>;subseq_copy29,806
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> LinkedListStPerS<T>;tabulate30,881
        fn map<U: StT, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> LinkedListStPerS<U>;map31,953
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append32,1048
        fn select(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>, index: N) -> Option<T>;select33,1140
        fn filter<F: Fn(&T) -> B>(a: &LinkedListStPerS<T>, pred: &F) -> LinkedListStPerS<T>;filter34,1232
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;update35,1325
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject36,1413
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject37,1520
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> A;iterate38,1628
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> iteratePrefixes39,1719
        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T;reduce40,1841
        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<scan42,1925
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten44,2030
        fn collect<A: StT, Bv: StT>(collect46,2118
    impl<T: StT> LinkedListStPerS<T> {LinkedListStPerS52,2305
        pub fn empty() -> Self { LinkedListStPerS { head: None, len: 0 } }empty53,2344
        pub fn new(length: N, init_value: T) -> Selfnew55,2420
        pub fn singleton(item: T) -> Self { LinkedListStPerS::from_vec(vec![item]) }singleton62,2595
        pub fn from_vec(elts: Vec<T>) -> Self {from_vec64,2681
        pub fn length(&self) -> N { self.len }length74,3031
        pub fn nth(&self, index: N) -> &T {nth76,3079
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy82,3254
        fn node_at(&self, index: N) -> Option<&NodeP<T>> {node_at112,4336
    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {LinkedListStPerS129,4811
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt130,4872
    impl<T: StT> PartialEq for LinkedListStPerS<T> {LinkedListStPerS147,5405
        fn eq(&self, other: &Self) -> bool {eq148,5458
    impl<T: StT> Eq for LinkedListStPerS<T> {}LinkedListStPerS165,5970
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS167,6018
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>new168,6085
        fn empty() -> LinkedListStPerS<T> { LinkedListStPerS::empty() }empty175,6260
        fn singleton(item: T) -> LinkedListStPerS<T> { LinkedListStPerS::singleton(item) }singleton176,6332
        fn length(&self) -> N { LinkedListStPerS::length(self) }length177,6423
        fn nth(&self, index: N) -> &T { LinkedListStPerS::nth(self, index) }nth178,6488
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T> {subseq_copy179,6565
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> LinkedListStPerS<T> {tabulate183,6715
        fn map<U: StT, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> LinkedListStPerS<U> {map191,6983
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append199,7299
        fn select(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>, index: N) -> Option<T> {select210,7728
        fn filter<F: Fn(&T) -> B>(a: &LinkedListStPerS<T>, pred: &F) -> LinkedListStPerS<T> {filter224,8255
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T>update235,8649
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject248,9138
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject261,9732
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> A {iterate272,10204
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> iteratePrefixes280,10444
        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T {reduce290,10878
        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<scan306,11502
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten321,12200
        fn collect<A: StT, Bv: StT>(collect332,12604

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStEph.rs,6265
pub mod ArraySeqStEph {ArraySeqStEph4,164
    pub struct ArraySeqStEphS<T: StT> {ArraySeqStEphS11,346
        data: Box<[T]>,data12,386
    pub type ArrayStEph<T> = ArraySeqStEphS<T>;ArrayStEph15,417
    impl<T: StT> ArraySeqStEphS<T> {ArraySeqStEphS17,466
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }from_vec18,503
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) new20,593
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }empty22,692
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton24,755
        pub fn length(&self) -> N { self.data.len() }length26,829
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth28,884
        pub fn iter(&self) -> std::slice::Iter<'_, T> {iter31,998
        pub fn subseq(&self, start: N, length: N) -> Self {subseq35,1094
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set42,1387
        pub fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {update51,1669
        pub fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut Self {inject56,1819
    impl<T: StT> PartialEq for ArraySeqStEphS<T> {ArraySeqStEphS68,2253
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }eq69,2304
    impl<T: StT> Eq for ArraySeqStEphS<T> {}ArraySeqStEphS72,2390
    impl<T: StT> Debug for ArraySeqStEphS<T> {ArraySeqStEphS74,2436
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt75,2483
    impl<'a, T: StT> IntoIterator for &'a ArraySeqStEphS<T> {ArraySeqStEphS78,2602
        type Item = &'a T;Item79,2664
        type IntoIter = std::slice::Iter<'a, T>;IntoIter80,2691
        fn into_iter(self) -> Self::IntoIter {into_iter82,2749
    impl<T: StT> IntoIterator for ArraySeqStEphS<T> {ArraySeqStEphS87,2842
        type Item = T;Item88,2896
        type IntoIter = std::vec::IntoIter<T>;IntoIter89,2919
        fn into_iter(self) -> Self::IntoIter {into_iter91,2975
    impl<T: StT> Display for ArraySeqStEphS<T> {ArraySeqStEphS96,3084
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt97,3133
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait109,3463
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>;new110,3506
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str>;set111,3569
        fn length(&self) -> N;length112,3663
        fn nth(&self, index: N) -> &T;nth113,3694
        fn empty() -> ArraySeqStEphS<T>;empty114,3733
        fn singleton(item: T) -> ArraySeqStEphS<T>;singleton115,3774
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStEphS<T>;tabulate116,3826
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U>;map117,3901
        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T>;subseq118,3992
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append119,4076
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T>;filter120,4162
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten121,4251
        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T>;update122,4331
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T>;inject123,4407
        fn isEmpty(&self) -> B;isEmpty124,4501
        fn isSingleton(&self) -> B;isSingleton125,4533
        fn collect<K: StT, V: StT>(collect126,4569
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A;iterate130,4744
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T;reduce131,4831
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, scan132,4912
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS135,5019
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::new(length, initnew136,5082
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str> {set138,5189
        fn length(&self) -> N { ArraySeqStEphS::length(self) }length142,5346
        fn nth(&self, index: N) -> &T { ArraySeqStEphS::nth(self, index) }nth144,5410
        fn empty() -> ArraySeqStEphS<T> { ArraySeqStEphS::empty() }empty146,5486
        fn singleton(item: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::singleton(item) }singleton148,5555
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStEphS<T> {tabulate150,5643
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U> {map158,5922
        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T> { a.subseq(stsubseq166,6232
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append168,6344
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T> {filter180,6796
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten191,7184
        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T> { ArraySeqStEphS::updupdate202,7577
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T> {inject204,7694
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty208,7850
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton210,7938
        fn collect<K: StT, V: StT>(collect212,8030
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A {iterate234,8956
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T {reduce242,9195
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, scan250,9426
    macro_rules! ArraySeqStEphS {ArraySeqStEphS262,9855
    fn _arrayseqstephs_macro_type_checks() {_arrayseqstephs_macro_type_checks269,10265

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtPer.rs,6377
pub mod ArraySeqMtPer {ArraySeqMtPer4,174
    pub struct ArraySeqMtPerS<T: StTInMtT> {ArraySeqMtPerS13,409
        data: Box<[T]>,data14,454
    impl<T: StTInMtT> ArraySeqMtPerS<T> {ArraySeqMtPerS17,485
        pub fn empty() -> Self {empty18,527
        pub fn new(length: N, init_value: T) -> Self {new24,667
        pub fn singleton(item: T) -> Self { ArraySeqMtPerS::from_vec(vec![item]) }singleton32,939
        pub fn from_vec(values: Vec<T>) -> Self {from_vec34,1023
        pub fn length(&self) -> N { self.data.len() }length40,1176
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth42,1231
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy44,1296
        pub fn is_empty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }is_empty52,1621
        pub fn is_singleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } is_singleton54,1716
        pub fn iter(&self) -> std::slice::Iter<'_, T> {iter57,1864
    impl<T: StTInMtT> Clone for ArraySeqMtPerS<T> {ArraySeqMtPerS62,1966
        fn clone(&self) -> Self {clone63,2018
    impl<T: StTInMtT> PartialEq for ArraySeqMtPerS<T> {ArraySeqMtPerS69,2184
        fn eq(&self, other: &Self) -> bool {eq70,2240
    impl<T: StTInMtT + Eq> Eq for ArraySeqMtPerS<T> {}ArraySeqMtPerS83,2575
    impl<'a, T: StTInMtT> IntoIterator for &'a ArraySeqMtPerS<T> {ArraySeqMtPerS85,2631
        type Item = &'a T;Item86,2698
        type IntoIter = std::slice::Iter<'a, T>;IntoIter87,2725
        fn into_iter(self) -> Self::IntoIter {into_iter89,2783
    impl<T: StTInMtT> IntoIterator for ArraySeqMtPerS<T> {ArraySeqMtPerS94,2876
        type Item = T;Item95,2935
        type IntoIter = std::vec::IntoIter<T>;IntoIter96,2958
        fn into_iter(self) -> Self::IntoIter {into_iter98,3014
    impl<T: StTInMtT> std::fmt::Display for ArraySeqMtPerS<T> {ArraySeqMtPerS103,3123
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt104,3187
    pub trait ArraySeqMtPerTrait<T: StTInMtT> {ArraySeqMtPerTrait114,3512
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;new115,3560
        fn empty() -> ArraySeqMtPerS<T>;empty116,3623
        fn singleton(item: T) -> ArraySeqMtPerS<T>;singleton117,3664
        fn length(&self) -> N;length118,3716
        fn nth(&self, index: N) -> &T;nth119,3747
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;subseq_copy120,3786
        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str>;set121,3859
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T>;tabulate122,3944
        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(a: &ArraySmap123,4028
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append124,4182
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtPerS<T>, pred: &F) -> ArraySeqMtPefilter125,4268
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T>;update126,4371
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject127,4455
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, iterate128,4557
        fn iteratePrefixes<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>,iteratePrefixes129,4665
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: reduce130,4802
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (Arrayscan131,4931
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;flatten132,5045
        fn collect(collect133,5126
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerSinject137,5281
        fn isEmpty(&self) -> B;isEmpty138,5382
        fn isSingleton(&self) -> B;isSingleton139,5414
    impl<T: StTInMtT> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {ArraySeqMtPerS142,5457
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::new(length, initnew143,5525
        fn empty() -> ArraySeqMtPerS<T> { ArraySeqMtPerS::empty() }empty144,5631
        fn singleton(item: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::singleton(item) }singleton145,5699
        fn length(&self) -> N { ArraySeqMtPerS::length(self) }length146,5786
        fn nth(&self, index: N) -> &T { ArraySeqMtPerS::nth(self, index) }nth147,5849
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::subseqsubseq_copy148,5924
        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str> {set150,6050
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T> {tabulate159,6417
        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(a: &ArraySmap167,6695
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append188,7621
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtPerS<T>, pred: &F) -> ArraySeqMtPefilter199,8042
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T> {update210,8447
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject220,8829
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, iterate231,9262
        fn iteratePrefixes<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>,iteratePrefixes239,9519
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: reduce249,9952
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (Arrayscan285,11556
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {flatten295,11975
        fn collect(collect306,12383
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerSinject333,13518
        fn isEmpty(&self) -> B { ArraySeqMtPerS::is_empty(self) }isEmpty345,14034
        fn isSingleton(&self) -> B { ArraySeqMtPerS::is_singleton(self) }isSingleton347,14101

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtEph.rs,5744
pub mod ArraySeqMtEph {ArraySeqMtEph4,159
    pub struct ArraySeqMtEphS<T: StT> {ArraySeqMtEphS13,417
        data: Mutex<Box<[T]>>,data14,457
    impl<T: StT> ArraySeqMtEphS<T> {ArraySeqMtEphS17,495
        pub fn empty() -> Self {empty18,532
        pub fn new(length: N, init_value: T) -> Selfnew24,684
        pub fn singleton(item: T) -> Self { ArraySeqMtEphS::from_vec(vec![item]) }singleton31,857
        pub fn from_vec(values: Vec<T>) -> Self {from_vec33,941
        pub fn length(&self) -> N {length39,1106
        pub fn nth_cloned(&self, index: N) -> T {nth_cloned44,1228
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set49,1373
        pub fn iter_cloned(&self) -> Vec<T> {iter_cloned62,1815
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy67,1967
        pub fn to_vec(&self) -> Vec<T> {to_vec76,2335
    impl<T: StT> Clone for ArraySeqMtEphS<T> {ArraySeqMtEphS82,2488
        fn clone(&self) -> Self { ArraySeqMtEphS::from_vec(self.to_vec()) }clone83,2535
    impl<T: StT> PartialEq for ArraySeqMtEphS<T> {ArraySeqMtEphS86,2618
        fn eq(&self, other: &Self) -> bool { self.to_vec() == other.to_vec() }eq87,2669
    impl<T: StT> Eq for ArraySeqMtEphS<T> {}ArraySeqMtEphS90,2755
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait92,2801
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>;new93,2844
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str>;set94,2907
        fn length(&self) -> N;length95,3001
        fn nth_cloned(&self, index: N) -> T;nth_cloned96,3032
        fn empty() -> ArraySeqMtEphS<T>;empty97,3077
        fn singleton(item: T) -> ArraySeqMtEphS<T>;singleton98,3118
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;tabulate99,3170
        fn map<U: StT + Send + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &Arramap100,3254
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;subseq_copy101,3417
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append102,3490
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtEphS<T>, pred: &F) -> ArraySeqMtEpfilter103,3576
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;update104,3679
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject105,3768
        fn isEmpty(&self) -> B;isEmpty106,3869
        fn isSingleton(&self) -> B;isSingleton107,3901
        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;flatten108,3937
        fn collect(collect109,4018
        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A)iterate113,4173
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: reduce114,4276
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (Arrayscan115,4412
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject116,4526
    impl<T: StT> std::fmt::Display for ArraySeqMtEphS<T> {ArraySeqMtEphS119,4635
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt120,4694
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS131,5066
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T> {new132,5129
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str> {set136,5256
        fn length(&self) -> N { ArraySeqMtEphS::length(self) }length140,5413
        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphS::nth_cloned(self, index) }nth_cloned142,5477
        fn empty() -> ArraySeqMtEphS<T> { ArraySeqMtEphS::empty() }empty144,5566
        fn singleton(item: T) -> ArraySeqMtEphS<T> { ArraySeqMtEphS::singleton(item) }singleton146,5635
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {tabulate148,5723
        fn map<U: StT + Send + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &Arramap156,6001
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {subseq_copy186,7323
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append190,7469
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtEphS<T>, pred: &F) -> ArraySeqMtEpfilter203,7922
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update215,8347
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject220,8508
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty232,8949
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton234,9037
        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {flatten236,9129
        fn collect(collect247,9531
        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A)iterate277,10787
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: reduce286,11080
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (Arrayscan322,12690
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject333,13150
    macro_rules! ArraySeqMtEphSLit {ArraySeqMtEphSLit344,13503
    fn _ArraySeqMtEphSLit_type_checks() {_ArraySeqMtEphSLit_type_checks351,13916

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStEph.rs,5241
pub mod LinkedListStEph {LinkedListStEph4,152
    pub struct NodeE<T: StT> {NodeE10,275
        pub value: T,value11,306
        pub next: Option<Box<NodeE<T>>>,next12,328
    pub struct LinkedListStEphS<T: StT> {LinkedListStEphS16,404
        head: Option<Box<NodeE<T>>>,head17,446
        len: N,len18,483
    impl<T: StT> LinkedListStEphS<T> {LinkedListStEphS21,506
        pub fn empty() -> Self { LinkedListStEphS { head: None, len: 0 } }empty22,545
        pub fn new(length: N, init_value: T) -> Selfnew24,621
        pub fn singleton(item: T) -> Self { LinkedListStEphS::from_vec(vec![item]) }singleton31,796
        pub fn from_vec(mut elts: Vec<T>) -> Self {from_vec33,882
        pub fn length(&self) -> N { self.len }length42,1209
        pub fn nth(&self, index: N) -> &T {nth44,1257
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set50,1432
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy60,1761
        fn node_at(&self, index: N) -> Option<&NodeE<T>> {node_at90,2843
        fn node_at_mut(&mut self, index: N) -> Option<&mut NodeE<T>> {node_at_mut106,3312
    impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {LinkedListStEphS123,3807
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt124,3868
    impl<T: StT> PartialEq for LinkedListStEphS<T> {LinkedListStEphS141,4401
        fn eq(&self, other: &Self) -> bool {eq142,4454
    impl<T: StT> Eq for LinkedListStEphS<T> {}LinkedListStEphS159,4966
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait161,5014
        fn new(length: N, init_value: T) -> Selfnew162,5059
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set166,5145
        fn length(&self) -> N;length168,5227
        fn nth(&self, index: N) -> &T;nth170,5259
        fn empty() -> Self;empty172,5299
        fn singleton(item: T) -> Self;singleton174,5328
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self;tabulate178,5460
        fn map<U: StT, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U>;map181,5613
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy182,5693
        fn append(a: &Self, b: &Self) -> Self;append185,5861
        fn filter<F: Fn(&T) -> B>(a: &Self, pred: &F) -> Self;filter188,6004
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> Self;deflate190,6137
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten191,6195
        fn update(a: &mut Self, item_at: Pair<N, T>) -> &mut Self;update194,6390
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;inject197,6577
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;ninject200,6774
        fn collect<A: StT, Bv: StT>(collect201,6852
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> A;iterate205,7032
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> (LinkedListStEpiteratePrefixes206,7108
        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T;reduce207,7215
        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (LinkedListStEphS<T>, T);scan208,7283
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS211,7379
        fn new(length: N, init_value: T) -> Selfnew212,7446
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set219,7606
        fn length(&self) -> N { LinkedListStEphS::length(self) }length223,7752
        fn nth(&self, index: N) -> &T { LinkedListStEphS::nth(self, index) }nth225,7818
        fn empty() -> Self { LinkedListStEphS::empty() }empty227,7896
        fn singleton(item: T) -> Self { LinkedListStEphS::singleton(item) }singleton229,7954
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self {tabulate231,8031
        fn map<U: StT, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U> {map239,8284
        fn subseq_copy(&self, start: N, length: N) -> Self { LinkedListStEphS::subseq_copy(self,subseq_copy247,8585
        fn append(a: &Self, b: &Self) -> Self {append249,8700
        fn filter<F: Fn(&T) -> B>(a: &Self, pred: &F) -> Self {filter260,9084
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> Self {deflate271,9448
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten280,9761
        fn update(a: &mut Self, Pair(index, item): Pair<N, T>) -> &mut Self {update291,10165
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {inject296,10308
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {ninject308,10732
        fn collect<A: StT, Bv: StT>(collect317,11036
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> A {iterate337,11885
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> (LinkedListStEpiteratePrefixes345,12110
        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T {reduce355,12529
        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (LinkedListStEphS<T>, T) {scan371,13138

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStPer.rs,5644
pub mod ArraySeqStPer {ArraySeqStPer4,171
    pub struct ArraySeqStPerS<T: StT> {ArraySeqStPerS12,439
        data: Box<[T]>,data13,479
    pub type ArrayStPer<T> = ArraySeqStPerS<T>;ArrayStPer16,510
    impl<T: StT> ArraySeqStPerS<T> {ArraySeqStPerS18,559
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }from_vec19,596
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) new20,685
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }empty21,783
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton22,845
        pub fn length(&self) -> N { self.data.len() }length23,918
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth24,972
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy25,1036
        pub fn iter(&self) -> std::slice::Iter<'_, T> {iter34,1422
    impl<T: StT> PartialEq for ArraySeqStPerS<T> {ArraySeqStPerS39,1524
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }eq40,1575
    impl<T: StT> Eq for ArraySeqStPerS<T> {}ArraySeqStPerS43,1661
    impl<T: StT> Debug for ArraySeqStPerS<T> {ArraySeqStPerS45,1707
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt46,1754
    impl<'a, T: StT> IntoIterator for &'a ArraySeqStPerS<T> {ArraySeqStPerS49,1873
        type Item = &'a T;Item50,1935
        type IntoIter = std::slice::Iter<'a, T>;IntoIter51,1962
        fn into_iter(self) -> Self::IntoIter {into_iter53,2020
    impl<T: StT> IntoIterator for ArraySeqStPerS<T> {ArraySeqStPerS58,2113
        type Item = T;Item59,2167
        type IntoIter = std::vec::IntoIter<T>;IntoIter60,2190
        fn into_iter(self) -> Self::IntoIter {into_iter62,2246
    impl<T: StT> Display for ArraySeqStPerS<T> {ArraySeqStPerS67,2355
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt68,2404
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait80,2734
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>;new81,2777
        fn length(&self) -> N;length82,2840
        fn nth(&self, index: N) -> &T;nth83,2871
        fn empty() -> ArraySeqStPerS<T>;empty84,2910
        fn singleton(item: T) -> ArraySeqStPerS<T>;singleton85,2951
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStPerS<T>;tabulate86,3003
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U>;map87,3078
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T>;subseq_copy88,3169
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append89,3258
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T>;filter90,3344
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;flatten91,3433
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject92,3513
        fn isEmpty(&self) -> B;isEmpty93,3614
        fn isSingleton(&self) -> B;isSingleton94,3646
        fn collect<K: StT, V: StT>(collect95,3682
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A;iterate99,3853
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T;reduce100,3940
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, scan101,4021
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject102,4121
    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {ArraySeqStPerS105,4230
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::new(length, initnew106,4293
        fn length(&self) -> N { ArraySeqStPerS::length(self) }length107,4399
        fn nth(&self, index: N) -> &T { ArraySeqStPerS::nth(self, index) }nth108,4462
        fn empty() -> ArraySeqStPerS<T> { ArraySeqStPerS::empty() }empty109,4537
        fn singleton(item: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::singleton(item) }singleton110,4605
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStPerS<T> {tabulate112,4693
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U> {map120,4972
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T> {subseq_copy128,5282
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {append132,5424
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {filter143,5845
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {flatten154,6236
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject165,6641
        fn isEmpty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }isEmpty179,7280
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton181,7370
        fn collect<K: StT, V: StT>(collect183,7464
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A {iterate210,8615
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T {reduce218,8854
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, scan233,9402
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject244,9840

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,3892
pub mod Types;Types6,240
pub mod Chap03 {Chap038,256
    pub mod InsertionSortSt;InsertionSortSt9,273
pub mod Chap05 {Chap0512,305
    pub mod MappingStEph;MappingStEph13,322
    pub mod RelationStEph;RelationStEph14,348
    pub mod SetStEph;SetStEph15,375
pub mod Chap06 {Chap0618,400
    pub mod DirGraphMtEph;DirGraphMtEph19,417
    pub mod DirGraphStEph;DirGraphStEph20,444
    pub mod LabDirGraphMtEph;LabDirGraphMtEph21,471
    pub mod LabDirGraphStEph;LabDirGraphStEph22,501
    pub mod LabUnDirGraphMtEph;LabUnDirGraphMtEph23,531
    pub mod LabUnDirGraphStEph;LabUnDirGraphStEph24,563
    pub mod UnDirGraphMtEph;UnDirGraphMtEph25,595
    pub mod UnDirGraphStEph;UnDirGraphStEph26,624
    pub mod WeightedDirGraphMtEphFloat;WeightedDirGraphMtEphFloat27,653
    pub mod WeightedDirGraphMtEphInt;WeightedDirGraphMtEphInt28,693
    pub mod WeightedDirGraphStEphFloat;WeightedDirGraphStEphFloat29,731
    pub mod WeightedDirGraphStEphInt;WeightedDirGraphStEphInt30,771
    pub mod WeightedUnDirGraphMtEphFloat;WeightedUnDirGraphMtEphFloat31,809
    pub mod WeightedUnDirGraphMtEphInt;WeightedUnDirGraphMtEphInt32,851
    pub mod WeightedUnDirGraphStEphFloat;WeightedUnDirGraphStEphFloat33,891
    pub mod WeightedUnDirGraphStEphInt;WeightedUnDirGraphStEphInt34,933
pub mod Chap11 {Chap1137,976
    pub mod FibonacciMt;FibonacciMt38,993
pub mod Chap12 {Chap1241,1021
    pub mod Exercise12_1;Exercise12_142,1038
    pub mod Exercise12_2;Exercise12_243,1064
    pub mod Exercise12_5;Exercise12_544,1090
pub mod Chap17 {Chap1747,1119
    pub mod MathSeq;MathSeq48,1136
pub mod Chap18 {Chap1851,1160
    pub mod ArraySeq;ArraySeq52,1177
    pub mod ArraySeqMtEph;ArraySeqMtEph53,1199
    pub mod ArraySeqMtPer;ArraySeqMtPer54,1226
    pub mod ArraySeqStEph;ArraySeqStEph55,1253
    pub mod ArraySeqStPer;ArraySeqStPer56,1280
    pub mod LinkedListStEph;LinkedListStEph58,1308
    pub mod LinkedListStPer;LinkedListStPer59,1337
pub mod Chap19 {Chap1962,1369
    pub mod ArraySeqMtEph;ArraySeqMtEph63,1386
    pub mod ArraySeqMtEphSlice;ArraySeqMtEphSlice64,1413
    pub mod ArraySeqMtPer;ArraySeqMtPer65,1445
    pub mod ArraySeqStEph;ArraySeqStEph66,1472
    pub mod ArraySeqStPer;ArraySeqStPer67,1499
pub mod Chap23 {Chap2370,1529
    pub mod BBTEph;BBTEph71,1546
    pub mod PrimTreeSeqSt;PrimTreeSeqSt72,1566
pub mod Chap36 {Chap3675,1596
    pub mod QuickSortMt;QuickSortMt76,1613
    pub mod QuickSortMtSlice;QuickSortMtSlice77,1638
    pub mod QuickSortSt;QuickSortSt78,1668
pub mod Chap37 {Chap3781,1696
    pub mod AVLTreeSeq;AVLTreeSeq82,1713
    pub mod AVLTreeSeqStEph;AVLTreeSeqStEph83,1737
    pub mod AVLTreeSeqStPer;AVLTreeSeqStPer84,1766
    pub mod BSTAVLMtEph;BSTAVLMtEph85,1795
    pub mod BSTAVLStEph;BSTAVLStEph86,1820
    pub mod BSTBBAlphaMtEph;BSTBBAlphaMtEph87,1845
    pub mod BSTBBAlphaStEph;BSTBBAlphaStEph88,1874
    pub mod BSTPlainMtEph;BSTPlainMtEph89,1903
    pub mod BSTPlainStEph;BSTPlainStEph90,1930
    pub mod BSTRBMtEph;BSTRBMtEph91,1957
    pub mod BSTRBStEph;BSTRBStEph92,1981
    pub mod BSTSetAVLMtEph;BSTSetAVLMtEph93,2005
    pub mod BSTSetBBAlphaMtEph;BSTSetBBAlphaMtEph94,2033
    pub mod BSTSetPlainMtEph;BSTSetPlainMtEph95,2065
    pub mod BSTSetRBMtEph;BSTSetRBMtEph96,2095
    pub mod BSTSetSplayMtEph;BSTSetSplayMtEph97,2122
    pub mod BSTSplayMtEph;BSTSplayMtEph98,2152
    pub mod BSTSplayStEph;BSTSplayStEph99,2179
pub mod Chap38 {Chap38102,2209
    pub mod BSTParaMtEph;BSTParaMtEph103,2226
    pub mod BSTParaStEph;BSTParaStEph104,2252
pub mod Chap39 {Chap39107,2281
    pub mod BSTParaTreapMtEph;BSTParaTreapMtEph108,2298
    pub mod BSTSetTreapMtEph;BSTSetTreapMtEph109,2329
    pub mod BSTTreapMtEph;BSTTreapMtEph110,2359
    pub mod BSTTreapStEph;BSTTreapStEph111,2386

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetAVLMtEph.rs,5593
pub mod BSTSetAVLMtEph {BSTSetAVLMtEph4,165
    pub struct BSTSetAVLMtEph<T: StTInMtT + Ord> {BSTSetAVLMtEph12,406
        tree: BSTAVLMtEph<T>,tree13,457
    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;BSTSetAVLMt16,494
    pub trait BSTSetAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetAVLMtEphTrait18,544
        fn empty() -> Self;empty19,606
        fn singleton(value: T) -> Self;singleton20,634
        fn size(&self) -> N;size21,674
        fn is_empty(&self) -> B;is_empty22,703
        fn find(&self, value: &T) -> Option<T>;find23,736
        fn contains(&self, value: &T) -> B;contains24,784
        fn minimum(&self) -> Option<T>;minimum25,828
        fn maximum(&self) -> Option<T>;maximum26,868
        fn insert(&mut self, value: T);insert27,908
        fn delete(&mut self, target: &T);delete28,948
        fn union(&self, other: &Self) -> Self;union29,990
        fn intersection(&self, other: &Self) -> Self;intersection30,1037
        fn difference(&self, other: &Self) -> Self;difference31,1091
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1143
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1198
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1253
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1315
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1385
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1453
        fn as_tree(&self) -> &BSTAVLMtEph<T>;as_tree38,1507
    impl<T: StTInMtT + Ord> BSTSetAVLMtEph<T> {BSTSetAVLMtEph41,1560
        pub fn empty() -> Self {empty42,1608
        pub fn singleton(value: T) -> Self {singleton48,1727
        pub fn size(&self) -> N { self.tree.size() }size54,1884
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1938
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2000
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2078
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2156
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2224
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2292
        pub fn delete(&mut self, target: &T) {delete68,2365
        pub fn union(&self, other: &Self) -> Self {union76,2654
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2953
        pub fn difference(&self, other: &Self) -> Self {difference101,3531
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4108
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4799
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5112
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5468
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5877
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6141
        pub fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree180,6224
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6289
        fn rebuild_from_vec(values: Vec<T>) -> BSTAVLMtEph<T> {rebuild_from_vec184,6380
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6599
    impl<T: StTInMtT + Ord> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {BSTSetAVLMtEph204,6882
        fn empty() -> Self { Self::empty() }empty205,6957
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7003
        fn size(&self) -> N { self.tree.size() }size209,7070
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7120
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7178
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7252
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7326
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7390
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7454
        fn delete(&mut self, target: &T) { BSTSetAVLMtEph::delete(self, target) }delete223,7523
        fn union(&self, other: &Self) -> Self { BSTSetAVLMtEph::union(self, other) }union225,7606
        fn intersection(&self, other: &Self) -> Self { BSTSetAVLMtEph::intersection(self, other)intersection227,7692
        fn difference(&self, other: &Self) -> Self { BSTSetAVLMtEph::difference(self, other) }difference229,7792
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetAVLMtEph::split(self, pivot) }split231,7888
        fn join_pair(left: Self, right: Self) -> Self { BSTSetAVLMtEph::join_pair(left, right) }join_pair233,7982
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetAVLMtEph::join_m(left, pivojoin_m235,8080
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetAVLMtEph::filter(sefilter237,8189
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetAVLMtEph::reduce(selfreduce239,8303
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8414
        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree243,8493
    macro_rules! BSTSetAVLMtEphLit {BSTSetAVLMtEphLit247,8580
    fn _BSTSetAVLMtEphLit_type_checks() {_BSTSetAVLMtEphLit_type_checks259,9127

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStPer.rs,4240
pub mod AVLTreeSeqStPer {AVLTreeSeqStPer4,177
    type Link<T> = Option<Rc<Node<T>>>;Link11,339
    struct Node<T: StT> {Node13,380
        value: T,value14,406
        height: N,height15,424
        size: N,size16,443
        left: Link<T>,left17,460
        right: Link<T>,right18,483
    fn height<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.height) }height21,514
    fn size<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.size) }size22,593
    fn mk<T: StT>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {mk24,669
    fn rotate_right<T: StT>(y: Rc<Node<T>>) -> Rc<Node<T>> {rotate_right37,1014
    fn rotate_left<T: StT>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left44,1321
    fn rebalance<T: StT>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance51,1627
    fn nth_ref<'a, T: StT>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref73,2480
    fn set_rec<T: StT>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> {set_rec88,2929
    fn inorder_collect<T: StT>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect106,3701
    fn build_balanced_from_slice<T: StT>(a: &[T]) -> Link<T> {build_balanced_from_slice114,3941
        fn rec<T: StT>(a: &[T]) -> Link<T> {rec115,4004
    pub struct AVLTreeSeqStPerS<T: StT> {AVLTreeSeqStPerS127,4322
        root: Link<T>,root128,4364
    pub trait AVLTreeSeqStPerTrait<T: StT> {AVLTreeSeqStPerTrait131,4394
        fn empty() -> Self;empty133,4480
        fn new() -> Self;new135,4549
        fn length(&self) -> N;length137,4616
        fn nth(&self, index: N) -> &T;nth139,4696
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set141,4845
        fn singleton(item: T) -> Self;singleton145,4996
        fn isEmpty(&self) -> B;isEmpty147,5076
        fn isSingleton(&self) -> B;isSingleton149,5149
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy151,5242
        fn from_vec(values: Vec<T>) -> Self;from_vec153,5365
        fn values_in_order(&self) -> Vec<T>;values_in_order155,5454
    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS158,5506
        fn empty() -> Self { AVLTreeSeqStPerS { root: None } }empty159,5573
        fn new() -> Self { Self::empty() }new160,5636
        fn length(&self) -> N { size(&self.root) }length161,5679
        fn nth(&self, index: N) -> &T { nth_ref(&self.root, index) }nth162,5730
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set163,5799
        fn singleton(item: T) -> Self {singleton168,5988
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty173,6133
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton174,6220
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy175,6311
        fn from_vec(values: Vec<T>) -> Self {from_vec189,6819
        fn values_in_order(&self) -> Vec<T> {values_in_order194,6982
    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS201,7173
        fn eq(&self, other: &Self) -> bool {eq202,7226
    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}AVLTreeSeqStPerS214,7552
    impl<T: StT> std::fmt::Debug for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS216,7600
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt217,7659
    impl<T: StT> AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS223,7851
        pub fn to_arrayseq(&self) -> ArraySeqStPerS<T> {to_arrayseq224,7890
        pub fn iter<'a>(&'a self) -> AVLTreeSeqStPerIter<'a, T> {iter229,8042
    pub struct AVLTreeSeqStPerIter<'a, T: StT> {AVLTreeSeqStPerIter237,8255
        stack: Vec<&'a Node<T>>,stack238,8304
        current: Option<&'a Node<T>>,current239,8337
    impl<'a, T: StT> AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter242,8382
        fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {push_left243,8432
    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter251,8642
        type Item = &'a T;Item252,8705
        fn next(&mut self) -> Option<Self::Item> {next253,8732
macro_rules! AVLTreeSeqStPerLit {AVLTreeSeqStPerLit267,9122

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetPlainMtEph.rs,5280
pub mod BSTSetPlainMtEph {BSTSetPlainMtEph4,167
    pub struct BSTSetPlainMtEph<T: StTInMtT + Ord> {BSTSetPlainMtEph12,416
        tree: BSTPlainMtEph<T>,tree13,469
    pub type BSTSetPlainMt<T> = BSTSetPlainMtEph<T>;BSTSetPlainMt16,508
    pub trait BSTSetPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetPlainMtEphTrait18,562
        fn empty() -> Self;empty19,626
        fn singleton(value: T) -> Self;singleton20,654
        fn size(&self) -> N;size21,694
        fn is_empty(&self) -> B;is_empty22,723
        fn find(&self, value: &T) -> Option<T>;find23,756
        fn contains(&self, value: &T) -> B;contains24,804
        fn minimum(&self) -> Option<T>;minimum25,848
        fn maximum(&self) -> Option<T>;maximum26,888
        fn insert(&mut self, value: T);insert27,928
        fn delete(&mut self, target: &T);delete28,968
        fn union(&self, other: &Self) -> Self;union29,1010
        fn intersection(&self, other: &Self) -> Self;intersection30,1057
        fn difference(&self, other: &Self) -> Self;difference31,1111
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1163
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1218
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1273
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1335
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1405
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1473
        fn as_tree(&self) -> &BSTPlainMtEph<T>;as_tree38,1527
    impl<T: StTInMtT + Ord> BSTSetPlainMtEph<T> {BSTSetPlainMtEph41,1582
        pub fn empty() -> Self {empty42,1632
        pub fn singleton(value: T) -> Self {singleton48,1753
        pub fn size(&self) -> N { self.tree.size() }size54,1912
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1966
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2028
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2106
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2184
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2252
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2320
        pub fn delete(&mut self, target: &T) {delete68,2393
        pub fn union(&self, other: &Self) -> Self {union76,2682
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2981
        pub fn difference(&self, other: &Self) -> Self {difference101,3559
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4136
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4827
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5140
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5496
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5905
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6169
        pub fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree180,6252
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6319
        fn rebuild_from_vec(values: Vec<T>) -> BSTPlainMtEph<T> {rebuild_from_vec184,6410
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6633
    impl<T: StTInMtT + Ord> BSTSetPlainMtEphTrait<T> for BSTSetPlainMtEph<T> {BSTSetPlainMtEph204,6918
        fn empty() -> Self { Self::empty() }empty205,6997
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7043
        fn size(&self) -> N { self.tree.size() }size209,7110
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7160
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7218
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7292
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7366
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7430
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7494
        fn delete(&mut self, target: &T) {delete223,7563
        fn union(&self, other: &Self) -> Self {union231,7848
        fn intersection(&self, other: &Self) -> Self {intersection239,8143
        fn difference(&self, other: &Self) -> Self {difference256,8717
        fn split(&self, pivot: &T) -> (Self, B, Self) {split273,9290
        fn join_pair(left: Self, right: Self) -> Self {join_pair293,9977
        fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m301,10286
        fn filter<F>(&self, predicate: F) -> Selffilter310,10638
        fn reduce<F>(&self, op: F, base: T) -> Treduce317,10799
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order324,10957
        fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree326,11036
    macro_rules! BSTSetPlainMtEphLit {BSTSetPlainMtEphLit330,11125
    fn _BSTSetPlainMtEphLit_type_checks() {_BSTSetPlainMtEphLit_type_checks342,11698

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBStEph.rs,4735
pub mod BSTRBStEph {BSTRBStEph4,185
    enum Color {Color11,414
        Red,Red12,431
        Black,Black13,444
    type Link<T> = Option<Box<Node<T>>>;Link16,466
    struct Node<T: StT + Ord> {Node19,536
        key: T,key20,568
        color: Color,color21,584
        size: N,size22,606
        left: Link<T>,left23,623
        right: Link<T>,right24,646
    impl<T: StT + Ord> Node<T> {Node27,677
        fn new(key: T) -> Self {new28,710
    pub struct BSTRBStEph<T: StT + Ord> {BSTRBStEph40,959
        root: Link<T>,root41,1001
    pub type BSTreeRB<T> = BSTRBStEph<T>;BSTreeRB44,1031
    pub trait BSTRBStEphTrait<T: StT + Ord> {BSTRBStEphTrait46,1074
        fn new() -> Self;new47,1120
        fn size(&self) -> N;size48,1146
        fn is_empty(&self) -> B;is_empty49,1175
        fn height(&self) -> N;height50,1208
        fn insert(&mut self, value: T);insert51,1239
        fn find(&self, target: &T) -> Option<&T>;find52,1279
        fn contains(&self, target: &T) -> B;contains53,1329
        fn minimum(&self) -> Option<&T>;minimum54,1374
        fn maximum(&self) -> Option<&T>;maximum55,1415
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order56,1456
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order57,1505
    impl<T: StT + Ord> Default for BSTRBStEph<T> {BSTRBStEph60,1562
        fn default() -> Self { Self::new() }default61,1613
    impl<T: StT + Ord> BSTRBStEph<T> {BSTRBStEph64,1665
        pub fn new() -> Self { BSTRBStEph { root: None } }new65,1704
        pub fn size(&self) -> N { Self::size_link(&self.root) }size67,1764
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty69,1829
        pub fn height(&self) -> N {height71,1920
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec72,1956
        pub fn insert(&mut self, value: T) {insert81,2252
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find88,2472
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains90,2566
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum92,2680
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum94,2756
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order96,2832
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order102,3056
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red108,3282
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link110,3384
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate112,3467
        fn rotate_left(link: &mut Link<T>) {rotate_left114,3586
        fn rotate_right(link: &mut Link<T>) {rotate_right131,4182
        fn flip_colors(link: &mut Link<T>) {flip_colors148,4780
        fn fix_up(link: &mut Link<T>) {fix_up169,5585
        fn insert_link(link: &mut Link<T>, value: T) {insert_link186,6272
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link202,6814
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link217,7331
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link227,7652
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect237,7975
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect245,8262
    impl<T: StT + Ord> BSTRBStEphTrait<T> for BSTRBStEph<T> {BSTRBStEph254,8558
        fn new() -> Self { BSTRBStEph::new() }new255,8620
        fn size(&self) -> N { BSTRBStEph::size(self) }size257,8668
        fn is_empty(&self) -> B { BSTRBStEph::is_empty(self) }is_empty259,8724
        fn height(&self) -> N { BSTRBStEph::height(self) }height261,8788
        fn insert(&mut self, value: T) { BSTRBStEph::insert(self, value) }insert263,8848
        fn find(&self, target: &T) -> Option<&T> { BSTRBStEph::find(self, target) }find265,8924
        fn contains(&self, target: &T) -> B { BSTRBStEph::contains(self, target) }contains267,9009
        fn minimum(&self) -> Option<&T> { BSTRBStEph::minimum(self) }minimum269,9093
        fn maximum(&self) -> Option<&T> { BSTRBStEph::maximum(self) }maximum271,9164
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTRBStEph::in_order(self) }in_order273,9235
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTRBStEph::pre_order(self) }pre_order275,9315
    macro_rules! BSTRBStEphLit {BSTRBStEphLit279,9423
    fn _BSTRBStEphLit_type_checks() {_BSTRBStEphLit_type_checks291,9917

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeq.rs,5339
pub mod AVLTreeSeq {AVLTreeSeq10,628
    type Link<T> = Option<Box<AVLTreeNode<T>>>;Link16,778
    pub struct AVLTreeNode<T: Copy + Debug> {AVLTreeNode18,827
        pub value: T,value19,873
        pub height: N,height20,895
        pub left_size: N,left_size21,918
        pub right_size: N,right_size22,944
        pub left: Link<T>,left23,971
        pub right: Link<T>,right24,998
        pub index: N,index25,1026
    impl<T: Copy + Debug> AVLTreeNode<T> {AVLTreeNode28,1055
        fn new(value: T, index: N) -> Self {new29,1098
    pub struct AVLTreeS<T: Copy + Debug> {AVLTreeS42,1391
        pub root: Link<T>,root43,1434
        pub next_key: N,next_key44,1461
    pub trait AVLTreeSeq<T: Copy + Debug> {AVLTreeSeq47,1493
        fn empty() -> AVLTreeS<T>;empty50,1616
        fn new() -> AVLTreeS<T>;new54,1739
        fn length(&self) -> N;length58,1854
        fn nth(&self, index: N) -> &T;nth62,2028
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str>;set66,2177
        fn singleton(item: T) -> AVLTreeS<T>;singleton70,2352
        fn isEmpty(&self) -> B;isEmpty74,2465
        fn isSingleton(&self) -> B;isSingleton75,2497
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>subseq_copy79,2664
    impl<T: Copy + Debug> AVLTreeS<T> {AVLTreeS84,2778
        pub fn new_root() -> Self {new_root85,2818
        pub fn new() -> Self { Self::new_root() }new91,2958
        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeS<T> {update93,3009
        pub fn from_vec(values: Vec<T>) -> AVLTreeS<T>from_vec98,3191
        pub fn to_arrayseq(&self) -> ArraySeqS<T>to_arrayseq111,3603
        pub fn iter<'a>(&'a self) -> AVLTreeSeqIter<'a, T> { AVLTreeSeqIter::new(&self.root) }iter130,4214
        pub fn push_back(&mut self, value: T) {push_back132,4310
        pub fn contains_value(&self, target: &T) -> Bcontains_value139,4618
        pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value151,4890
        pub fn delete_value(&mut self, target: &T) -> booldelete_value153,4967
        pub fn is_tree_empty(&self) -> bool { self.length() == 0 }is_tree_empty181,5911
        pub fn values_in_order(&self) -> Vec<T>values_in_order183,5979
    impl<T: Copy + Debug> AVLTreeSeq<T> for AVLTreeS<T> {AVLTreeS193,6215
        fn empty() -> AVLTreeS<T> { AVLTreeS::new_root() }empty195,6315
        fn new() -> AVLTreeS<T> { AVLTreeS::new_root() }new198,6417
        fn length(&self) -> N { size_link(&self.root) }length201,6517
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth204,6624
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str> {set207,6745
        fn singleton(item: T) -> AVLTreeS<T> {singleton213,6960
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty220,7198
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton222,7327
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>subseq_copy225,7481
    impl<T: Eq + Copy + Debug> PartialEq for AVLTreeS<T> {AVLTreeS243,8032
        fn eq(&self, other: &Self) -> bool {eq244,8091
    impl<T: Eq + Copy + Debug> Eq for AVLTreeS<T> {}AVLTreeS257,8418
    impl<T: Debug + Copy> std::fmt::Debug for AVLTreeS<T> {AVLTreeS259,8472
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt260,8532
    impl<T: std::fmt::Display + Copy + Debug> std::fmt::Display for AVLTreeS<T> {AVLTreeS266,8740
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt267,8822
    pub struct AVLTreeSeqIter<'a, T: Copy + Debug> {AVLTreeSeqIter283,9234
        stack: Vec<&'a AVLTreeNode<T>>,stack284,9287
        current: Option<&'a AVLTreeNode<T>>,current285,9327
    impl<'a, T: Copy + Debug> AVLTreeSeqIter<'a, T> {AVLTreeSeqIter288,9379
        fn new(root: &'a Link<T>) -> Self {new289,9433
        fn push_left(&mut self, link: &'a Link<T>) {push_left298,9658
    impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIter<'a, T> {AVLTreeSeqIter307,9910
        type Item = &'a T;Item308,9977
        fn next(&mut self) -> Option<Self::Item> {next309,10004
    fn h<T: Copy + Debug>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h319,10263
    fn size_link<T: Copy + Debug>(n: &Link<T>) -> N {size_link320,10346
    fn update_meta<T: Copy + Debug>(n: &mut Box<AVLTreeNode<T>>) {update_meta328,10520
    fn rotate_right<T: Copy + Debug>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right336,10774
    fn rotate_left<T: Copy + Debug>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left349,11148
    fn rebalance<T: Copy + Debug>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance362,11520
    pub(crate) fn insert_at_link<T: Copy + Debug>(node: Link<T>, index: N, value: T, next_key: &insert_at_link386,12420
    fn nth_link<'a, T: Copy + Debug>(node: &'a Link<T>, index: N) -> &'a T {nth_link406,13225
    fn set_link<T: Copy + Debug>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static set_link418,13614
    fn push_inorder<T: Copy + Debug>(link: &Link<T>, out: &mut Vec<T>)push_inorder435,14213

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetSplayMtEph.rs,5648
pub mod BSTSetSplayMtEph {BSTSetSplayMtEph4,167
    pub struct BSTSetSplayMtEph<T: StTInMtT + Ord> {BSTSetSplayMtEph12,416
        tree: BSTSplayMtEph<T>,tree13,469
    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;BSTSetSplayMt16,508
    pub trait BSTSetSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetSplayMtEphTrait18,562
        fn empty() -> Self;empty19,626
        fn singleton(value: T) -> Self;singleton20,654
        fn size(&self) -> N;size21,694
        fn is_empty(&self) -> B;is_empty22,723
        fn find(&self, value: &T) -> Option<T>;find23,756
        fn contains(&self, value: &T) -> B;contains24,804
        fn minimum(&self) -> Option<T>;minimum25,848
        fn maximum(&self) -> Option<T>;maximum26,888
        fn insert(&mut self, value: T);insert27,928
        fn delete(&mut self, target: &T);delete28,968
        fn union(&self, other: &Self) -> Self;union29,1010
        fn intersection(&self, other: &Self) -> Self;intersection30,1057
        fn difference(&self, other: &Self) -> Self;difference31,1111
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1163
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1218
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1273
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1335
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1405
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1473
        fn as_tree(&self) -> &BSTSplayMtEph<T>;as_tree38,1527
    impl<T: StTInMtT + Ord> BSTSetSplayMtEph<T> {BSTSetSplayMtEph41,1582
        pub fn empty() -> Self {empty42,1632
        pub fn singleton(value: T) -> Self {singleton48,1753
        pub fn size(&self) -> N { self.tree.size() }size54,1912
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1966
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2028
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2106
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2184
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2252
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2320
        pub fn delete(&mut self, target: &T) {delete68,2393
        pub fn union(&self, other: &Self) -> Self {union76,2682
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2981
        pub fn difference(&self, other: &Self) -> Self {difference101,3559
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4136
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4827
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5140
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5496
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5905
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6169
        pub fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree180,6252
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6319
        fn rebuild_from_vec(values: Vec<T>) -> BSTSplayMtEph<T> {rebuild_from_vec184,6410
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6633
    impl<T: StTInMtT + Ord> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {BSTSetSplayMtEph204,6918
        fn empty() -> Self { Self::empty() }empty205,6997
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7043
        fn size(&self) -> N { self.tree.size() }size209,7110
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7160
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7218
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7292
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7366
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7430
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7494
        fn delete(&mut self, target: &T) { BSTSetSplayMtEph::delete(self, target) }delete223,7563
        fn union(&self, other: &Self) -> Self { BSTSetSplayMtEph::union(self, other) }union225,7648
        fn intersection(&self, other: &Self) -> Self { BSTSetSplayMtEph::intersection(self, otheintersection227,7736
        fn difference(&self, other: &Self) -> Self { BSTSetSplayMtEph::difference(self, other) }difference229,7838
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetSplayMtEph::split(self, pivot) }split231,7936
        fn join_pair(left: Self, right: Self) -> Self { BSTSetSplayMtEph::join_pair(left, right)join_pair233,8032
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetSplayMtEph::join_m(left, pijoin_m235,8132
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetSplayMtEph::filter(filter237,8243
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetSplayMtEph::reduce(sereduce239,8359
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8472
        fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree243,8551
    macro_rules! BSTSetSplayMtEphLit {BSTSetSplayMtEphLit247,8640
    fn _BSTSetSplayMtEphLit_type_checks() {_BSTSetSplayMtEphLit_type_checks259,9213

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaMtEph.rs,4633
pub mod BSTBBAlphaMtEph {BSTBBAlphaMtEph4,200
    type Link<T> = Option<Box<Node<T>>>;Link13,436
    struct Node<T: StTInMtT + Ord> {Node17,520
        key: T,key18,557
        size: N,size19,573
        left: Link<T>,left20,590
        right: Link<T>,right21,613
    impl<T: StTInMtT + Ord> Node<T> {Node24,644
        fn new(key: T) -> Self {new25,682
    pub struct BSTBBAlphaMtEph<T: StTInMtT + Ord> {BSTBBAlphaMtEph36,896
        root: Arc<RwLock<Link<T>>>,root37,948
    pub type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;BSTreeBBAlpha40,991
    pub trait BSTBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTBBAlphaMtEphTrait42,1044
        fn new() -> Self;new43,1107
        fn insert(&self, value: T);insert44,1133
        fn find(&self, target: &T) -> Option<T>;find45,1169
        fn contains(&self, target: &T) -> B;contains46,1218
        fn size(&self) -> N;size47,1263
        fn is_empty(&self) -> B;is_empty48,1292
        fn height(&self) -> N;height49,1325
        fn minimum(&self) -> Option<T>;minimum50,1356
        fn maximum(&self) -> Option<T>;maximum51,1396
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order52,1436
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order53,1485
    impl<T: StTInMtT + Ord> Default for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph56,1542
        fn default() -> Self { Self::new() }default57,1603
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph60,1655
        pub fn new() -> Self {new61,1704
        pub fn size(&self) -> N {size67,1841
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty72,1974
        pub fn height(&self) -> N {height74,2065
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec75,2101
        pub fn insert(&self, value: T) {insert86,2451
        pub fn find(&self, target: &T) -> Option<T> {find95,2782
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains100,2952
        pub fn minimum(&self) -> Option<T> {minimum102,3066
        pub fn maximum(&self) -> Option<T> {maximum107,3218
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order112,3370
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order119,3655
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link126,3942
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate128,4025
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link130,4144
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild152,4890
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed159,5172
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values169,5582
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced177,5863
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link189,6303
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link204,6820
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link214,7141
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect224,7464
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect232,7751
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph241,8047
        fn new() -> Self { BSTBBAlphaMtEph::new() }new242,8124
        fn insert(&self, value: T) { BSTBBAlphaMtEph::insert(self, value) }insert244,8177
        fn find(&self, target: &T) -> Option<T> { BSTBBAlphaMtEph::find(self, target) }find246,8254
        fn contains(&self, target: &T) -> B { BSTBBAlphaMtEph::contains(self, target) }contains248,8343
        fn size(&self) -> N { BSTBBAlphaMtEph::size(self) }size250,8432
        fn is_empty(&self) -> B { BSTBBAlphaMtEph::is_empty(self) }is_empty252,8493
        fn height(&self) -> N { BSTBBAlphaMtEph::height(self) }height254,8562
        fn minimum(&self) -> Option<T> { BSTBBAlphaMtEph::minimum(self) }minimum256,8627
        fn maximum(&self) -> Option<T> { BSTBBAlphaMtEph::maximum(self) }maximum258,8702
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaMtEph::in_order(self) }in_order260,8777
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaMtEph::pre_order(self) }pre_order262,8862
    macro_rules! BSTBBAlphaMtEphLit {BSTBBAlphaMtEphLit266,8975
    fn _BSTBBAlphaMtEphLit_type_checks() {_BSTBBAlphaMtEphLit_type_checks278,9530

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLStEph.rs,4554
pub mod BSTAVLStEph {BSTAVLStEph4,189
    type Link<T> = Option<Box<Node<T>>>;Link9,356
    struct Node<T: StT + Ord> {Node13,440
        key: T,key14,472
        height: i32,height15,488
        size: N,size16,509
        left: Link<T>,left17,526
        right: Link<T>,right18,549
    impl<T: StT + Ord> Node<T> {Node21,580
        fn new(key: T) -> Self {new22,613
    pub struct BSTAVLStEph<T: StT + Ord> {BSTAVLStEph34,854
        root: Link<T>,root35,897
    pub type BSTreeAVL<T> = BSTAVLStEph<T>;BSTreeAVL38,927
    pub trait BSTAVLStEphTrait<T: StT + Ord> {BSTAVLStEphTrait40,972
        fn new() -> Self;new41,1019
        fn size(&self) -> N;size42,1045
        fn is_empty(&self) -> B;is_empty43,1074
        fn height(&self) -> N;height44,1107
        fn insert(&mut self, value: T);insert45,1138
        fn find(&self, target: &T) -> Option<&T>;find46,1178
        fn contains(&self, target: &T) -> B;contains47,1228
        fn minimum(&self) -> Option<&T>;minimum48,1273
        fn maximum(&self) -> Option<&T>;maximum49,1314
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order50,1355
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order51,1404
    impl<T: StT + Ord> Default for BSTAVLStEph<T> {BSTAVLStEph54,1461
        fn default() -> Self { Self::new() }default55,1513
    impl<T: StT + Ord> BSTAVLStEph<T> {BSTAVLStEph58,1565
        pub fn new() -> Self { BSTAVLStEph { root: None } }new59,1605
        pub fn size(&self) -> N { Self::size_link(&self.root) }size61,1666
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty63,1731
        pub fn height(&self) -> N { Self::height_link(&self.root) as N }height65,1822
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert67,1896
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find69,1986
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains71,2080
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum73,2194
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum75,2270
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order77,2346
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order83,2570
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link89,2796
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link91,2885
        fn update(node: &mut Node<T>) {update93,2968
        fn rotate_right(link: &mut Link<T>) {rotate_right98,3204
        fn rotate_left(link: &mut Link<T>) {rotate_left112,3660
        fn rebalance(link: &mut Link<T>) {rebalance126,4115
        fn insert_link(link: &mut Link<T>, value: T) {insert_link151,5161
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link170,5796
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link185,6313
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link195,6634
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect205,6957
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect213,7244
    impl<T: StT + Ord> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {BSTAVLStEph222,7540
        fn new() -> Self { BSTAVLStEph::new() }new223,7604
        fn size(&self) -> N { BSTAVLStEph::size(self) }size225,7653
        fn is_empty(&self) -> B { BSTAVLStEph::is_empty(self) }is_empty227,7710
        fn height(&self) -> N { BSTAVLStEph::height(self) }height229,7775
        fn insert(&mut self, value: T) { BSTAVLStEph::insert(self, value) }insert231,7836
        fn find(&self, target: &T) -> Option<&T> { BSTAVLStEph::find(self, target) }find233,7913
        fn contains(&self, target: &T) -> B { BSTAVLStEph::contains(self, target) }contains235,7999
        fn minimum(&self) -> Option<&T> { BSTAVLStEph::minimum(self) }minimum237,8084
        fn maximum(&self) -> Option<&T> { BSTAVLStEph::maximum(self) }maximum239,8156
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::in_order(self) }in_order241,8228
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::pre_order(self) }pre_order243,8309
    macro_rules! BSTAVLStEphLit {BSTAVLStEphLit247,8418
    fn _BSTAVLStEphLit_type_checks() {_BSTAVLStEphLit_type_checks259,8925

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainMtEph.rs,3812
pub mod BSTPlainMtEph {BSTPlainMtEph4,182
    type Link<T> = Arc<RwLock<Option<Node<T>>>>;Link10,330
    struct Node<T: StTInMtT + Ord> {Node14,422
        key: T,key15,459
        height: i32,height16,475
        size: N,size17,496
        left: Link<T>,left18,513
        right: Link<T>,right19,536
    impl<T: StTInMtT + Ord> Node<T> {Node22,567
        fn new(key: T) -> Self {new23,605
        fn update(&mut self) {update33,858
    pub struct BSTPlainMtEph<T: StTInMtT + Ord> {BSTPlainMtEph43,1183
        root: Link<T>,root44,1233
    pub type BSTree<T> = BSTPlainMtEph<T>;BSTree47,1263
    pub trait BSTPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTPlainMtEphTrait49,1307
        fn new() -> Self;new50,1368
        fn insert(&self, value: T);insert51,1394
        fn find(&self, target: &T) -> Option<T>;find52,1430
        fn contains(&self, target: &T) -> B;contains53,1479
        fn size(&self) -> N;size54,1524
        fn is_empty(&self) -> B;is_empty55,1553
        fn height(&self) -> N;height56,1586
        fn minimum(&self) -> Option<T>;minimum57,1617
        fn maximum(&self) -> Option<T>;maximum58,1657
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order59,1697
    impl<T: StTInMtT + Ord> BSTPlainMtEph<T> {BSTPlainMtEph62,1753
        pub fn new() -> Self {new63,1800
        pub fn insert(&self, value: T) {insert69,1926
            fn descend<T: StTInMtT + Ord>(link: &Link<T>, value: T) -> bool {descend70,1967
        pub fn find(&self, target: &T) -> Option<T> {find104,3159
            fn find_rec<T: StTInMtT + Ord>(link: &Link<T>, target: &T) -> Option<T> {find_rec105,3213
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains124,3952
        pub fn size(&self) -> N {size125,4065
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty130,4216
        pub fn height(&self) -> N {height132,4307
        pub fn minimum(&self) -> Option<T> {minimum137,4467
            fn leftmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {leftmost138,4512
        pub fn maximum(&self) -> Option<T> {maximum159,5222
            fn rightmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {rightmost160,5267
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order181,5984
            fn traverse<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {traverse182,6038
    fn height_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> i32 { link.as_ref().map_or(0, |n|height_of201,6706
    fn size_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> N { link.as_ref().map_or(0, |n| n.ssize_of203,6816
    impl<T: StTInMtT + Ord> BSTPlainMtEphTrait<T> for BSTPlainMtEph<T> {BSTPlainMtEph205,6920
        fn new() -> Self { BSTPlainMtEph::new() }new206,6993
        fn insert(&self, value: T) { BSTPlainMtEph::insert(self, value) }insert207,7043
        fn find(&self, target: &T) -> Option<T> { BSTPlainMtEph::find(self, target) }find208,7117
        fn contains(&self, target: &T) -> B { BSTPlainMtEph::contains(self, target) }contains209,7203
        fn size(&self) -> N { BSTPlainMtEph::size(self) }size210,7289
        fn is_empty(&self) -> B { BSTPlainMtEph::is_empty(self) }is_empty211,7347
        fn height(&self) -> N { BSTPlainMtEph::height(self) }height212,7413
        fn minimum(&self) -> Option<T> { BSTPlainMtEph::minimum(self) }minimum213,7475
        fn maximum(&self) -> Option<T> { BSTPlainMtEph::maximum(self) }maximum214,7547
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTPlainMtEph::in_order(self) }in_order215,7619
    macro_rules! BSTPlainMtEphLit {BSTPlainMtEphLit219,7728
    fn _BSTPlainMtEphLit_type_checks() {_BSTPlainMtEphLit_type_checks234,8292

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaStEph.rs,4746
pub mod BSTBBAlphaStEph {BSTBBAlphaStEph4,172
    type Link<T> = Option<Box<Node<T>>>;Link11,373
    struct Node<T: StT + Ord> {Node14,443
        key: T,key15,475
        size: N,size16,491
        left: Link<T>,left17,508
        right: Link<T>,right18,531
    impl<T: StT + Ord> Node<T> {Node21,562
        fn new(key: T) -> Self {new22,595
    pub struct BSTBBAlphaStEph<T: StT + Ord> {BSTBBAlphaStEph33,809
        root: Link<T>,root34,856
    pub type BSTreeBBAlpha<T> = BSTBBAlphaStEph<T>;BSTreeBBAlpha37,886
    pub trait BSTBBAlphaStEphTrait<T: StT + Ord> {BSTBBAlphaStEphTrait39,939
        fn new() -> Self;new40,990
        fn size(&self) -> N;size41,1016
        fn is_empty(&self) -> B;is_empty42,1045
        fn height(&self) -> N;height43,1078
        fn insert(&mut self, value: T);insert44,1109
        fn find(&self, target: &T) -> Option<&T>;find45,1149
        fn contains(&self, target: &T) -> B;contains46,1199
        fn minimum(&self) -> Option<&T>;minimum47,1244
        fn maximum(&self) -> Option<&T>;maximum48,1285
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order49,1326
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order50,1375
    impl<T: StT + Ord> Default for BSTBBAlphaStEph<T> {BSTBBAlphaStEph53,1432
        fn default() -> Self { Self::new() }default54,1488
    impl<T: StT + Ord> BSTBBAlphaStEph<T> {BSTBBAlphaStEph57,1540
        pub fn new() -> Self { BSTBBAlphaStEph { root: None } }new58,1584
        pub fn size(&self) -> N { Self::size_link(&self.root) }size60,1649
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty62,1714
        pub fn height(&self) -> N {height64,1805
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec65,1841
        pub fn insert(&mut self, value: T) {insert74,2137
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find82,2425
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains84,2519
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum86,2633
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum88,2709
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order90,2785
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order96,3009
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link102,3235
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate104,3318
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link106,3437
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild128,4183
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed135,4465
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values145,4875
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced153,5156
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link165,5596
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link180,6113
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link190,6434
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect200,6757
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect208,7044
    impl<T: StT + Ord> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {BSTBBAlphaStEph217,7340
        fn new() -> Self { BSTBBAlphaStEph::new() }new218,7412
        fn size(&self) -> N { BSTBBAlphaStEph::size(self) }size220,7465
        fn is_empty(&self) -> B { BSTBBAlphaStEph::is_empty(self) }is_empty222,7526
        fn height(&self) -> N { BSTBBAlphaStEph::height(self) }height224,7595
        fn insert(&mut self, value: T) { BSTBBAlphaStEph::insert(self, value) }insert226,7660
        fn find(&self, target: &T) -> Option<&T> { BSTBBAlphaStEph::find(self, target) }find228,7741
        fn contains(&self, target: &T) -> B { BSTBBAlphaStEph::contains(self, target) }contains230,7831
        fn minimum(&self) -> Option<&T> { BSTBBAlphaStEph::minimum(self) }minimum232,7920
        fn maximum(&self) -> Option<&T> { BSTBBAlphaStEph::maximum(self) }maximum234,7996
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaStEph::in_order(self) }in_order236,8072
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaStEph::pre_order(self) }pre_order238,8157
    macro_rules! BSTBBAlphaStEphLit {BSTBBAlphaStEphLit242,8270
    fn _BSTBBAlphaStEphLit_type_checks() {_BSTBBAlphaStEphLit_type_checks254,8829

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayMtEph.rs,4234
pub mod BSTSplayMtEph {BSTSplayMtEph4,191
    type Link<T> = Option<Box<Node<T>>>;Link11,395
    struct Node<T: StTInMtT + Ord> {Node14,465
        key: T,key15,502
        size: N,size16,518
        left: Link<T>,left17,535
        right: Link<T>,right18,558
    impl<T: StTInMtT + Ord> Node<T> {Node21,589
        fn new(key: T) -> Self {new22,627
    pub struct BSTSplayMtEph<T: StTInMtT + Ord> {BSTSplayMtEph33,841
        root: Arc<RwLock<Link<T>>>,root34,891
    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;BSTreeSplay37,934
    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSplayMtEphTrait39,983
        fn new() -> Self;new40,1044
        fn insert(&self, value: T);insert41,1070
        fn find(&self, target: &T) -> Option<T>;find42,1106
        fn contains(&self, target: &T) -> B;contains43,1155
        fn size(&self) -> N;size44,1200
        fn is_empty(&self) -> B;is_empty45,1229
        fn height(&self) -> N;height46,1262
        fn minimum(&self) -> Option<T>;minimum47,1293
        fn maximum(&self) -> Option<T>;maximum48,1333
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order49,1373
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order50,1422
    impl<T: StTInMtT + Ord> Default for BSTSplayMtEph<T> {BSTSplayMtEph53,1479
        fn default() -> Self { Self::new() }default54,1538
    impl<T: StTInMtT + Ord> BSTSplayMtEph<T> {BSTSplayMtEph57,1590
        pub fn new() -> Self {new58,1637
        pub fn size(&self) -> N {size64,1772
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty69,1905
        pub fn height(&self) -> N {height71,1996
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec72,2032
        pub fn insert(&self, value: T) {insert83,2382
        pub fn find(&self, target: &T) -> Option<T> {find88,2541
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains93,2711
        pub fn minimum(&self) -> Option<T> {minimum95,2825
        pub fn maximum(&self) -> Option<T> {maximum100,2977
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order105,3129
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order112,3414
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link119,3701
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate121,3784
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link123,3903
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link145,4649
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link160,5166
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link170,5487
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect180,5810
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect188,6097
    impl<T: StTInMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {BSTSplayMtEph197,6393
        fn new() -> Self { BSTSplayMtEph::new() }new198,6466
        fn insert(&self, value: T) { BSTSplayMtEph::insert(self, value) }insert200,6517
        fn find(&self, target: &T) -> Option<T> { BSTSplayMtEph::find(self, target) }find202,6592
        fn contains(&self, target: &T) -> B { BSTSplayMtEph::contains(self, target) }contains204,6679
        fn size(&self) -> N { BSTSplayMtEph::size(self) }size206,6766
        fn is_empty(&self) -> B { BSTSplayMtEph::is_empty(self) }is_empty208,6825
        fn height(&self) -> N { BSTSplayMtEph::height(self) }height210,6892
        fn minimum(&self) -> Option<T> { BSTSplayMtEph::minimum(self) }minimum212,6955
        fn maximum(&self) -> Option<T> { BSTSplayMtEph::maximum(self) }maximum214,7028
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTSplayMtEph::in_order(self) }in_order216,7101
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTSplayMtEph::pre_order(self) }pre_order218,7184
    macro_rules! BSTSplayMtEphLit {BSTSplayMtEphLit222,7295
    fn _BSTSplayMtEphLit_type_checks() {_BSTSplayMtEphLit_type_checks234,7824

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetRBMtEph.rs,5565
pub mod BSTSetRBMtEph {BSTSetRBMtEph4,171
    pub struct BSTSetRBMtEph<T: StTInMtT + Ord> {BSTSetRBMtEph12,408
        tree: BSTRBMtEph<T>,tree13,458
    pub type BSTSetRBMt<T> = BSTSetRBMtEph<T>;BSTSetRBMt16,494
    pub trait BSTSetRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetRBMtEphTrait18,542
        fn empty() -> Self;empty19,603
        fn singleton(value: T) -> Self;singleton20,631
        fn size(&self) -> N;size21,671
        fn is_empty(&self) -> B;is_empty22,700
        fn find(&self, value: &T) -> Option<T>;find23,733
        fn contains(&self, value: &T) -> B;contains24,781
        fn minimum(&self) -> Option<T>;minimum25,825
        fn maximum(&self) -> Option<T>;maximum26,865
        fn insert(&mut self, value: T);insert27,905
        fn delete(&mut self, target: &T);delete28,945
        fn union(&self, other: &Self) -> Self;union29,987
        fn intersection(&self, other: &Self) -> Self;intersection30,1034
        fn difference(&self, other: &Self) -> Self;difference31,1088
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1140
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1195
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1250
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1312
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1382
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1450
        fn as_tree(&self) -> &BSTRBMtEph<T>;as_tree38,1504
    impl<T: StTInMtT + Ord> BSTSetRBMtEph<T> {BSTSetRBMtEph41,1556
        pub fn empty() -> Self {empty42,1603
        pub fn singleton(value: T) -> Self {singleton48,1721
        pub fn size(&self) -> N { self.tree.size() }size54,1877
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1931
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,1993
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2071
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2149
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2217
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2285
        pub fn delete(&mut self, target: &T) {delete68,2358
        pub fn union(&self, other: &Self) -> Self {union76,2647
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2946
        pub fn difference(&self, other: &Self) -> Self {difference101,3524
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4101
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4792
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5105
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5461
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5870
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6134
        pub fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree180,6217
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6281
        fn rebuild_from_vec(values: Vec<T>) -> BSTRBMtEph<T> {rebuild_from_vec184,6372
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6589
    impl<T: StTInMtT + Ord> BSTSetRBMtEphTrait<T> for BSTSetRBMtEph<T> {BSTSetRBMtEph204,6871
        fn empty() -> Self { Self::empty() }empty205,6944
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,6990
        fn size(&self) -> N { self.tree.size() }size209,7057
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7107
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7165
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7239
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7313
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7377
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7441
        fn delete(&mut self, target: &T) { BSTSetRBMtEph::delete(self, target) }delete223,7510
        fn union(&self, other: &Self) -> Self { BSTSetRBMtEph::union(self, other) }union225,7592
        fn intersection(&self, other: &Self) -> Self { BSTSetRBMtEph::intersection(self, other) intersection227,7677
        fn difference(&self, other: &Self) -> Self { BSTSetRBMtEph::difference(self, other) }difference229,7776
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetRBMtEph::split(self, pivot) }split231,7871
        fn join_pair(left: Self, right: Self) -> Self { BSTSetRBMtEph::join_pair(left, right) }join_pair233,7964
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetRBMtEph::join_m(left, pivotjoin_m235,8061
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetRBMtEph::filter(selfilter237,8169
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetRBMtEph::reduce(self,reduce239,8282
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8392
        fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree243,8471
    macro_rules! BSTSetRBMtEphLit {BSTSetRBMtEphLit247,8557
    fn _BSTSetRBMtEphLit_type_checks() {_BSTSetRBMtEphLit_type_checks259,9091

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStEph.rs,4730
pub mod AVLTreeSeqStEph {AVLTreeSeqStEph4,153
    type Link<T> = Option<Box<AVLTreeNode<T>>>;Link10,294
    pub struct AVLTreeNode<T: StT> {AVLTreeNode12,343
        pub value: T,value13,380
        pub height: N,height14,402
        pub left_size: N,left_size15,425
        pub right_size: N,right_size16,451
        pub left: Link<T>,left17,478
        pub right: Link<T>,right18,505
        pub index: N,index19,533
    impl<T: StT> AVLTreeNode<T> {AVLTreeNode22,562
        fn new(value: T, index: N) -> Self {new23,596
    pub struct AVLTreeSeqStEphS<T: StT> {AVLTreeSeqStEphS36,889
        pub root: Link<T>,root37,931
        pub next_key: N,next_key38,958
    pub trait AVLTreeSeqStEphTrait<T: StT> {AVLTreeSeqStEphTrait41,990
        fn empty() -> Self;empty43,1077
        fn new() -> Self;new45,1147
        fn length(&self) -> N;length47,1215
        fn nth(&self, index: N) -> &T;nth49,1296
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set51,1385
        fn singleton(item: T) -> Self;singleton53,1508
        fn isEmpty(&self) -> B;isEmpty55,1589
        fn isSingleton(&self) -> B;isSingleton57,1663
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy59,1761
    impl<T: StT> AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS62,1828
        pub fn new_root() -> Self {new_root63,1867
        pub fn new() -> Self { Self::new_root() }new69,2015
        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqStEphS<T> {update70,2065
        pub fn from_vec(values: Vec<T>) -> AVLTreeSeqStEphS<T> {from_vec74,2221
        pub fn to_arrayseq(&self) -> ArraySeqStEphS<T> {to_arrayseq83,2604
        pub fn iter<'a>(&'a self) -> AVLTreeSeqIterStEph<'a, T> { AVLTreeSeqIterStEph::new(&selfiter99,3192
        pub fn push_back(&mut self, value: T) {push_back100,3297
        pub fn contains_value(&self, target: &T) -> B {contains_value105,3511
        pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value113,3734
        pub fn delete_value(&mut self, target: &T) -> bool {delete_value114,3810
    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS139,4628
        fn empty() -> Self { AVLTreeSeqStEphS::new_root() }empty140,4695
        fn new() -> Self { AVLTreeSeqStEphS::new_root() }new142,4756
        fn length(&self) -> N { size_link(&self.root) }length144,4815
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth146,4872
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set148,4943
        fn singleton(item: T) -> Self {singleton153,5109
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty159,5306
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton161,5394
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy163,5486
    pub struct AVLTreeSeqIterStEph<'a, T: StT> {AVLTreeSeqIterStEph178,5975
        stack: Vec<&'a AVLTreeNode<T>>,stack179,6024
        current: Option<&'a AVLTreeNode<T>>,current180,6064
    impl<'a, T: StT> AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph183,6116
        fn new(root: &'a Link<T>) -> Self {new184,6166
        fn push_left(&mut self, link: &'a Link<T>) {push_left192,6395
    impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph201,6647
        type Item = &'a T;Item202,6710
        fn next(&mut self) -> Option<Self::Item> {next203,6737
    fn h<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h211,6961
    fn size_link<T: StT>(n: &Link<T>) -> N {size_link213,7036
    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>) {update_meta221,7201
    fn rotate_right<T: StT>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right229,7446
    fn rotate_left<T: StT>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left240,7809
    fn rebalance<T: StT>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance251,8170
    pub(crate) fn insert_at_link<T: StT>(node: Link<T>, index: N, value: T, next_key: &mut N) ->insert_at_link272,8935
    fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> &'a T {nth_link292,9731
    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str> {set_link304,10111
    macro_rules! AVLTreeSeqStEphLit {AVLTreeSeqStEphLit322,10721
    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS336,11309
        fn eq(&self, other: &Self) -> bool {eq337,11362
    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}AVLTreeSeqStEphS350,11689

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBMtEph.rs,4637
pub mod BSTRBMtEph {BSTRBMtEph4,194
    enum Color {Color13,458
        Red,Red14,475
        Black,Black15,488
    type Link<T> = Option<Box<Node<T>>>;Link18,510
    struct Node<T: StTInMtT + Ord> {Node22,594
        key: T,key23,631
        color: Color,color24,647
        size: N,size25,669
        left: Link<T>,left26,686
        right: Link<T>,right27,709
    impl<T: StTInMtT + Ord> Node<T> {Node30,740
        fn new(key: T) -> Self {new31,778
    pub struct BSTRBMtEph<T: StTInMtT + Ord> {BSTRBMtEph43,1027
        root: Arc<RwLock<Link<T>>>,root44,1074
    pub type BSTreeRB<T> = BSTRBMtEph<T>;BSTreeRB47,1117
    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTRBMtEphTrait49,1160
        fn new() -> Self;new50,1218
        fn insert(&self, value: T);insert51,1244
        fn find(&self, target: &T) -> Option<T>;find52,1280
        fn contains(&self, target: &T) -> B;contains53,1329
        fn size(&self) -> N;size54,1374
        fn is_empty(&self) -> B;is_empty55,1403
        fn height(&self) -> N;height56,1436
        fn minimum(&self) -> Option<T>;minimum57,1467
        fn maximum(&self) -> Option<T>;maximum58,1507
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order59,1547
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order60,1596
    impl<T: StTInMtT + Ord> Default for BSTRBMtEph<T> {BSTRBMtEph63,1653
        fn default() -> Self { Self::new() }default64,1709
    impl<T: StTInMtT + Ord> BSTRBMtEph<T> {BSTRBMtEph67,1761
        pub fn new() -> Self {new68,1805
        pub fn size(&self) -> N {size74,1937
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty79,2070
        pub fn height(&self) -> N {height81,2161
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec82,2197
        pub fn insert(&self, value: T) {insert93,2547
        pub fn find(&self, target: &T) -> Option<T> {find101,2812
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains106,2982
        pub fn minimum(&self) -> Option<T> {minimum108,3096
        pub fn maximum(&self) -> Option<T> {maximum113,3248
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order118,3400
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order125,3685
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red132,3972
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link134,4074
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate136,4157
        fn rotate_left(link: &mut Link<T>) {rotate_left138,4276
        fn rotate_right(link: &mut Link<T>) {rotate_right157,4936
        fn flip_colors(link: &mut Link<T>) {flip_colors176,5600
        fn fix_up(link: &mut Link<T>) {fix_up197,6405
        fn insert_link(link: &mut Link<T>, value: T) {insert_link233,7559
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link249,8101
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link264,8618
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link274,8939
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect284,9262
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect292,9549
    impl<T: StTInMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {BSTRBMtEph301,9845
        fn new() -> Self { BSTRBMtEph::new() }new302,9912
        fn insert(&self, value: T) { BSTRBMtEph::insert(self, value) }insert304,9960
        fn find(&self, target: &T) -> Option<T> { BSTRBMtEph::find(self, target) }find306,10032
        fn contains(&self, target: &T) -> B { BSTRBMtEph::contains(self, target) }contains308,10116
        fn size(&self) -> N { BSTRBMtEph::size(self) }size310,10200
        fn is_empty(&self) -> B { BSTRBMtEph::is_empty(self) }is_empty312,10256
        fn height(&self) -> N { BSTRBMtEph::height(self) }height314,10320
        fn minimum(&self) -> Option<T> { BSTRBMtEph::minimum(self) }minimum316,10380
        fn maximum(&self) -> Option<T> { BSTRBMtEph::maximum(self) }maximum318,10450
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTRBMtEph::in_order(self) }in_order320,10520
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTRBMtEph::pre_order(self) }pre_order322,10600
    macro_rules! BSTRBMtEphLit {BSTRBMtEphLit326,10708
    fn _BSTRBMtEphLit_type_checks() {_BSTRBMtEphLit_type_checks338,11198

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayStEph.rs,4387
pub mod BSTSplayStEph {BSTSplayStEph4,176
    type Link<T> = Option<Box<Node<T>>>;Link9,345
    struct Node<T: StT + Ord> {Node13,429
        key: T,key14,461
        size: N,size15,477
        left: Link<T>,left16,494
        right: Link<T>,right17,517
    impl<T: StT + Ord> Node<T> {Node20,548
        fn new(key: T) -> Self {new21,581
    pub struct BSTSplayStEph<T: StT + Ord> {BSTSplayStEph32,795
        root: Link<T>,root33,840
    pub type BSTreeSplay<T> = BSTSplayStEph<T>;BSTreeSplay36,870
    pub trait BSTSplayStEphTrait<T: StT + Ord> {BSTSplayStEphTrait38,919
        fn new() -> Self;new39,968
        fn size(&self) -> N;size40,994
        fn is_empty(&self) -> B;is_empty41,1023
        fn height(&self) -> N;height42,1056
        fn insert(&mut self, value: T);insert43,1087
        fn find(&self, target: &T) -> Option<&T>;find44,1127
        fn contains(&self, target: &T) -> B;contains45,1177
        fn minimum(&self) -> Option<&T>;minimum46,1222
        fn maximum(&self) -> Option<&T>;maximum47,1263
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order48,1304
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order49,1353
    impl<T: StT + Ord> Default for BSTSplayStEph<T> {BSTSplayStEph52,1410
        fn default() -> Self { Self::new() }default53,1464
    impl<T: StT + Ord> BSTSplayStEph<T> {BSTSplayStEph56,1516
        pub fn new() -> Self { BSTSplayStEph { root: None } }new57,1558
        pub fn size(&self) -> N { Self::size_link(&self.root) }size59,1621
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty61,1686
        pub fn height(&self) -> N {height63,1777
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec64,1813
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert73,2109
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find75,2199
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains77,2293
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum79,2407
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum81,2483
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order83,2559
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order89,2783
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link95,3009
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate97,3092
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link99,3211
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link121,3957
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link136,4474
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link146,4795
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect156,5118
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect164,5405
    impl<T: StT + Ord> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {BSTSplayStEph173,5701
        fn new() -> Self { BSTSplayStEph::new() }new174,5769
        fn size(&self) -> N { BSTSplayStEph::size(self) }size176,5820
        fn is_empty(&self) -> B { BSTSplayStEph::is_empty(self) }is_empty178,5879
        fn height(&self) -> N { BSTSplayStEph::height(self) }height180,5946
        fn insert(&mut self, value: T) { BSTSplayStEph::insert(self, value) }insert182,6009
        fn find(&self, target: &T) -> Option<&T> { BSTSplayStEph::find(self, target) }find184,6088
        fn contains(&self, target: &T) -> B { BSTSplayStEph::contains(self, target) }contains186,6176
        fn minimum(&self) -> Option<&T> { BSTSplayStEph::minimum(self) }minimum188,6263
        fn maximum(&self) -> Option<&T> { BSTSplayStEph::maximum(self) }maximum190,6337
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTSplayStEph::in_order(self) }in_order192,6411
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTSplayStEph::pre_order(self) }pre_order194,6494
    macro_rules! BSTSplayStEphLit {BSTSplayStEphLit198,6605
    fn _BSTSplayStEphLit_type_checks() {_BSTSplayStEphLit_type_checks210,7138

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLMtEph.rs,4360
pub mod BSTAVLMtEph {BSTAVLMtEph4,188
    type Link<T> = Option<Box<Node<T>>>;Link11,390
    struct Node<T: StTInMtT + Ord> {Node14,460
        key: T,key15,497
        height: i32,height16,513
        size: N,size17,534
        left: Link<T>,left18,551
        right: Link<T>,right19,574
    impl<T: StTInMtT + Ord> Node<T> {Node22,605
        fn new(key: T) -> Self {new23,643
    pub struct BSTAVLMtEph<T: StTInMtT + Ord> {BSTAVLMtEph35,884
        root: Arc<RwLock<Link<T>>>,root36,932
    pub type BSTreeAVL<T> = BSTAVLMtEph<T>;BSTreeAVL39,975
    pub trait BSTAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTAVLMtEphTrait41,1020
        fn new() -> Self;new42,1079
        fn insert(&self, value: T);insert43,1105
        fn find(&self, target: &T) -> Option<T>;find44,1141
        fn contains(&self, target: &T) -> B;contains45,1190
        fn size(&self) -> N;size46,1235
        fn is_empty(&self) -> B;is_empty47,1264
        fn height(&self) -> N;height48,1297
        fn minimum(&self) -> Option<T>;minimum49,1328
        fn maximum(&self) -> Option<T>;maximum50,1368
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order51,1408
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order52,1457
    impl<T: StTInMtT + Ord> Default for BSTAVLMtEph<T> {BSTAVLMtEph55,1514
        fn default() -> Self { Self::new() }default56,1571
    impl<T: StTInMtT + Ord> BSTAVLMtEph<T> {BSTAVLMtEph59,1623
        pub fn new() -> Self {new60,1668
        pub fn size(&self) -> N {size66,1801
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty71,1934
        pub fn height(&self) -> N {height73,2025
        pub fn insert(&self, value: T) {insert78,2167
        pub fn find(&self, target: &T) -> Option<T> {find83,2326
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains88,2496
        pub fn minimum(&self) -> Option<T> {minimum90,2610
        pub fn maximum(&self) -> Option<T> {maximum95,2762
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order100,2914
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order107,3199
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link114,3486
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link116,3575
        fn update(node: &mut Node<T>) {update118,3658
        fn rotate_right(link: &mut Link<T>) {rotate_right123,3894
        fn rotate_left(link: &mut Link<T>) {rotate_left137,4350
        fn rebalance(link: &mut Link<T>) {rebalance151,4805
        fn insert_link(link: &mut Link<T>, value: T) {insert_link176,5851
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link195,6486
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link210,7003
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link220,7325
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect230,7649
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect238,7936
    impl<T: StTInMtT + Ord> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {BSTAVLMtEph247,8232
        fn new() -> Self { BSTAVLMtEph::new() }new248,8301
        fn insert(&self, value: T) { BSTAVLMtEph::insert(self, value) }insert250,8350
        fn find(&self, target: &T) -> Option<T> { BSTAVLMtEph::find(self, target) }find252,8423
        fn contains(&self, target: &T) -> B { BSTAVLMtEph::contains(self, target) }contains254,8508
        fn size(&self) -> N { BSTAVLMtEph::size(self) }size256,8593
        fn is_empty(&self) -> B { BSTAVLMtEph::is_empty(self) }is_empty258,8650
        fn height(&self) -> N { BSTAVLMtEph::height(self) }height260,8715
        fn minimum(&self) -> Option<T> { BSTAVLMtEph::minimum(self) }minimum262,8776
        fn maximum(&self) -> Option<T> { BSTAVLMtEph::maximum(self) }maximum264,8847
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTAVLMtEph::in_order(self) }in_order266,8918
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTAVLMtEph::pre_order(self) }pre_order268,8999
    macro_rules! BSTAVLMtEphLit {BSTAVLMtEphLit272,9108
    fn _BSTAVLMtEphLit_type_checks() {_BSTAVLMtEphLit_type_checks284,9611

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainStEph.rs,3573
pub mod BSTPlainStEph {BSTPlainStEph4,156
    pub struct BSTPlainStEph<T: StT + Ord> {BSTPlainStEph10,344
        root: BBTree<T>,root11,389
    pub type BSTree<T> = BSTPlainStEph<T>;BSTree14,421
    pub trait BSTPlainStEphTrait<T: StT + Ord> {BSTPlainStEphTrait16,465
        fn new() -> Self;new17,514
        fn size(&self) -> N;size18,540
        fn is_empty(&self) -> B;is_empty19,569
        fn height(&self) -> N;height20,602
        fn insert(&mut self, value: T);insert21,633
        fn find(&self, target: &T) -> Option<&T>;find22,673
        fn contains(&self, target: &T) -> B;contains23,723
        fn minimum(&self) -> Option<&T>;minimum24,768
        fn maximum(&self) -> Option<&T>;maximum25,809
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order26,850
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order27,899
    impl<T: StT + Ord> BSTPlainStEph<T> {BSTPlainStEph30,956
        pub fn new() -> Self { BSTPlainStEph { root: BBTree::leaf() } }new31,998
        pub fn size(&self) -> N { self.root.size() }size33,1071
        pub fn is_empty(&self) -> B { self.root.is_leaf() }is_empty35,1125
        pub fn height(&self) -> N { self.root.height() }height37,1186
        pub fn insert(&mut self, value: T) { insert_node(&mut self.root, value); }insert39,1244
        pub fn find(&self, target: &T) -> Option<&T> { find_node(&self.root, target) }find41,1328
        pub fn contains(&self, target: &T) -> B { contains_node(&self.root, target) }contains43,1416
        pub fn minimum(&self) -> Option<&T> { min_node(&self.root) }minimum45,1503
        pub fn maximum(&self) -> Option<&T> { max_node(&self.root) }maximum47,1573
        pub fn in_order(&self) -> ArraySeqStPerS<T> { self.root.in_order() }in_order49,1643
        pub fn pre_order(&self) -> ArraySeqStPerS<T> { self.root.pre_order() }pre_order51,1721
    impl<T: StT + Ord> BSTPlainStEphTrait<T> for BSTPlainStEph<T> {BSTPlainStEph54,1807
        fn new() -> Self { BSTPlainStEph::new() }new55,1875
        fn size(&self) -> N { BSTPlainStEph::size(self) }size57,1926
        fn is_empty(&self) -> B { BSTPlainStEph::is_empty(self) }is_empty59,1985
        fn height(&self) -> N { BSTPlainStEph::height(self) }height61,2052
        fn insert(&mut self, value: T) { BSTPlainStEph::insert(self, value) }insert63,2115
        fn find(&self, target: &T) -> Option<&T> { BSTPlainStEph::find(self, target) }find65,2194
        fn contains(&self, target: &T) -> B { BSTPlainStEph::contains(self, target) }contains67,2282
        fn minimum(&self) -> Option<&T> { BSTPlainStEph::minimum(self) }minimum69,2369
        fn maximum(&self) -> Option<&T> { BSTPlainStEph::maximum(self) }maximum71,2443
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTPlainStEph::in_order(self) }in_order73,2517
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTPlainStEph::pre_order(self) }pre_order75,2600
    fn insert_node<T: StT + Ord>(node: &mut BBTree<T>, value: T) {insert_node78,2691
    fn contains_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> B {contains_node93,3194
    fn find_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> Option<&'a T> {find_node108,3680
    fn min_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {min_node123,4173
    fn max_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {max_node133,4493
    macro_rules! BSTPlainStEphLit {BSTPlainStEphLit144,4835
    fn _BSTPlainStEphLit_type_checks() {_BSTPlainStEphLit_type_checks159,5407

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetBBAlphaMtEph.rs,5700
pub mod BSTSetBBAlphaMtEph {BSTSetBBAlphaMtEph4,170
    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord> {BSTSetBBAlphaMtEph12,427
        tree: BSTBBAlphaMtEph<T>,tree13,482
    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;BSTSetBBAlphaMt16,523
    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetBBAlphaMtEphTrait18,581
        fn empty() -> Self;empty19,647
        fn singleton(value: T) -> Self;singleton20,675
        fn size(&self) -> N;size21,715
        fn is_empty(&self) -> B;is_empty22,744
        fn find(&self, value: &T) -> Option<T>;find23,777
        fn contains(&self, value: &T) -> B;contains24,825
        fn minimum(&self) -> Option<T>;minimum25,869
        fn maximum(&self) -> Option<T>;maximum26,909
        fn insert(&mut self, value: T);insert27,949
        fn delete(&mut self, target: &T);delete28,989
        fn union(&self, other: &Self) -> Self;union29,1031
        fn intersection(&self, other: &Self) -> Self;intersection30,1078
        fn difference(&self, other: &Self) -> Self;difference31,1132
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1184
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1239
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1294
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1356
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1426
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1494
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T>;as_tree38,1548
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph41,1605
        pub fn empty() -> Self {empty42,1657
        pub fn singleton(value: T) -> Self {singleton48,1780
        pub fn size(&self) -> N { self.tree.size() }size54,1941
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1995
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2057
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2135
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2213
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2281
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2349
        pub fn delete(&mut self, target: &T) {delete68,2422
        pub fn union(&self, other: &Self) -> Self {union76,2711
        pub fn intersection(&self, other: &Self) -> Self {intersection84,3010
        pub fn difference(&self, other: &Self) -> Self {difference101,3588
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4165
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4856
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5169
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5525
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5934
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6198
        pub fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree180,6281
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6350
        fn rebuild_from_vec(values: Vec<T>) -> BSTBBAlphaMtEph<T> {rebuild_from_vec184,6441
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6668
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph204,6955
        fn empty() -> Self { Self::empty() }empty205,7038
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7084
        fn size(&self) -> N { self.tree.size() }size209,7151
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7201
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7259
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7333
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7407
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7471
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7535
        fn delete(&mut self, target: &T) { BSTSetBBAlphaMtEph::delete(self, target) }delete223,7604
        fn union(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::union(self, other) }union225,7691
        fn intersection(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::intersection(self, otintersection227,7781
        fn difference(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::difference(self, other)difference229,7885
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetBBAlphaMtEph::split(self, pivot) }split231,7985
        fn join_pair(left: Self, right: Self) -> Self { BSTSetBBAlphaMtEph::join_pair(left, righjoin_pair233,8083
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetBBAlphaMtEph::join_m(left, join_m235,8185
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetBBAlphaMtEph::filtefilter237,8298
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetBBAlphaMtEph::reduce(reduce239,8416
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8531
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree243,8610
    macro_rules! BSTSetBBAlphaMtEphLit {BSTSetBBAlphaMtEphLit247,8701
    fn _BSTSetBBAlphaMtEphLit_type_checks() {_BSTSetBBAlphaMtEphLit_type_checks259,9300

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap03/InsertionSortSt.rs,312
pub mod InsertionSortSt {InsertionSortSt4,143
    pub trait InsertionSortStTrait<T: Ord + Clone> {InsertionSortStTrait6,170
        fn insSort(&self, slice: &mut [T]);insSort9,322
    impl<T: Ord + Clone> InsertionSortStTrait<T> for T {T12,373
        fn insSort(&self, slice: &mut [T]) {insSort13,430

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/MappingStEph.rs,2414
pub mod MappingStEph {MappingStEph4,164
    pub struct Mapping<A, B> {Mapping15,482
        rel: Relation<A, B>,rel16,513
    pub trait MappingStEphTrait<X: StT + Hash, Y: StT + Hash> {MappingStEphTrait19,549
        fn empty() -> Mapping<X, Y>;empty22,705
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y>;FromVec26,839
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation30,993
        fn size(&self) -> N;size34,1148
        fn domain(&self) -> Set<X>;domain38,1274
        fn range(&self) -> Set<Y>;range42,1407
        fn mem(&self, a: &X, b: &Y) -> B;mem46,1535
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>>;iter48,1578
    impl<A: Eq + Hash, B: Eq + Hash> Mapping<A, B> {Mapping51,1661
        fn unique_pairs_from_iter<I: IntoIterator<Item = Pair<A, B>>>(iter: I) -> Set<Pair<A, B>unique_pairs_from_iter52,1714
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Mapping<A, B> {Mapping62,2092
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq63,2161
    impl<A: StT + Hash, B: StT + Hash> Eq for Mapping<A, B> {}Mapping65,2236
    impl<A: StT + Hash, B: StT + Hash> Debug for Mapping<A, B> {Mapping67,2300
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.rel, f) }fmt68,2365
    impl<A: StT + Hash, B: StT + Hash> Display for Mapping<A, B> {Mapping70,2455
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.rel, f) }fmt71,2522
    impl<X: StT + Hash, Y: StT + Hash> MappingStEphTrait<X, Y> for Mapping<X, Y> {Mapping74,2615
        fn empty() -> Mapping<X, Y> {empty75,2698
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y> {FromVec81,2859
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation88,3104
        fn size(&self) -> N { self.rel.size() }size95,3370
        fn domain(&self) -> Set<X> { self.rel.domain() }domain97,3419
        fn range(&self) -> Set<Y> { self.rel.range() }range99,3477
        fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem101,3533
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.rel.iter() }iter103,3598
    macro_rules! MappingLit {MappingLit107,3720
    fn _MappingLit_type_checks() {_MappingLit_type_checks118,4285
    pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise124,4516

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/SetStEph.rs,3623
pub mod SetStEph {SetStEph4,161
    pub struct Set<T> {Set13,342
        data: HashSet<T>,data14,366
    pub trait SetStEphTrait<T: StT + Hash> {SetStEphTrait17,399
        fn empty() -> Set<T>;empty20,536
        fn singleton(x: T) -> Set<T>;singleton23,658
        fn size(&self) -> N;size26,788
        fn mem(&self, x: &T) -> B;mem29,909
        fn union(&self, other: &Set<T>) -> Set<T>;union32,1052
        fn intersection(&self, other: &Set<T>) -> Set<T>;intersection35,1211
        fn partition(&self, parts: &Set<Set<T>>) -> B;partition38,1391
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>;CartesianProduct42,1557
        fn insert(&mut self, x: T) -> &mut Self;insert46,1736
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;iter50,1878
        fn FromVec(v: Vec<T>) -> Set<T>;FromVec53,2041
    impl<T: Eq + Hash> PartialEq for Set<T> {Set56,2089
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq57,2135
    impl<T: Eq + Hash> Eq for Set<T> {}Set60,2213
    impl<T: Eq + Hash> std::fmt::Debug for Set<T>Set62,2254
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt66,2348
    impl<T: Eq + Hash> std::fmt::Display for Set<T>Set71,2503
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt75,2601
    impl<T: Eq + Hash> Hash for Set<T> {Set91,3107
        fn hash<H: Hasher>(&self, state: &mut H) {hash92,3148
    impl<T: Eq + Hash> Set<T> {Set108,3712
        pub fn empty() -> Set<T> { Set { data: HashSet::new() } }empty109,3744
        pub fn singleton(x: T) -> Set<T> {singleton111,3811
        pub fn size(&self) -> N { self.data.len() }size117,3977
        pub fn mem(&self, x: &T) -> B { if self.data.contains(x) { B::True } else { B::False } }mem119,4030
        pub fn union(&self, other: &Set<T>) -> Set<T>union121,4128
        pub fn intersection(&self, other: &Set<T>) -> Set<T>intersection132,4402
        pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition143,4759
        pub fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct161,5314
        pub fn insert(&mut self, x: T) -> &mut Self {insert175,5760
        pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter180,5883
        pub fn FromVec(v: Vec<T>) -> Set<T> {FromVec182,5975
    impl<T: StT + Hash> SetStEphTrait<T> for Set<T> {Set191,6199
        fn empty() -> Set<T> { Set { data: HashSet::new() } }empty192,6253
        fn singleton(x: T) -> Set<T> {singleton194,6316
        fn size(&self) -> N { self.data.len() }size200,6478
        fn mem(&self, x: &T) -> B { if self.data.contains(x) { B::True } else { B::False } }mem202,6527
        fn union(&self, other: &Set<T>) -> Set<T>union204,6621
        fn intersection(&self, other: &Set<T>) -> Set<T>intersection215,6891
        fn partition(&self, parts: &Set<Set<T>>) -> B {partition226,7244
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct244,7795
        fn insert(&mut self, x: T) -> &mut Self {insert258,8237
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter263,8356
        fn FromVec(v: Vec<T>) -> Set<T> {FromVec265,8444
    macro_rules! SetLit {SetLit275,8684
    fn _SetLit_type_checks() {_SetLit_type_checks287,9026
    pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise293,9222
        let _s0: Set<&'static str> = SetLit![];str294,9268

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/RelationStEph.rs,2235
pub mod RelationStEph {RelationStEph4,155
    pub struct Relation<A, B> {Relation15,426
        pairs: Set<Pair<A, B>>,pairs16,458
    pub trait RelationStEphTrait<X: StT + Hash, Y: StT + Hash> {RelationStEphTrait19,497
        fn empty() -> Relation<X, Y>;empty22,654
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y>;FromSet26,797
        fn size(&self) -> N;size30,952
        fn domain(&self) -> Set<X>domain34,1078
        fn range(&self) -> Set<Y>range40,1246
        fn mem(&self, a: &X, b: &Y) -> Bmem46,1409
        fn iter(&self) -> Iter<'_, Pair<X, Y>>;iter51,1509
    impl<A: StT + Hash, B: StT + Hash> Relation<A, B> {Relation54,1564
        pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B> { Relation { pairs: Set::FromVec(v)FromVec55,1620
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Relation<A, B> {Relation58,1728
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq59,1798
    impl<A: StT + Hash, B: StT + Hash> Eq for Relation<A, B> {}Relation62,1878
    impl<A: StT + Hash, B: StT + Hash> Debug for Relation<A, B> {Relation64,1943
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }fmt65,2009
    impl<A: StT + Hash, B: StT + Hash> Display for Relation<A, B> {Relation68,2112
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) fmt69,2180
    impl<X: StT + Hash, Y: StT + Hash> RelationStEphTrait<X, Y> for Relation<X, Y> {Relation72,2285
        fn empty() -> Relation<X, Y> { Relation { pairs: SetLit![] } }empty73,2370
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y> { Relation { pairs } }FromSet75,2442
        fn size(&self) -> N { self.pairs.size() }size77,2527
        fn domain(&self) -> Set<X>domain79,2578
        fn range(&self) -> Set<Y>range90,2845
        fn mem(&self, a: &X, b: &Y) -> Bmem101,3111
        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }iter113,3387
    macro_rules! RelationLit {RelationLit117,3483
    fn _RelationLit_type_checks() {_RelationLit_type_checks133,4360
    pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise139,4595

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphFloat.rs,1258
pub mod WeightedUnDirGraphMtEphFloat {WeightedUnDirGraphMtEphFloat4,199
    pub type WeightedUnDirGraphMtEphFloat<V> = LabUnDirGraphMtEph<V, OrderedF64>;WeightedUnDirGraphMtEphFloat13,555
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphFloat<V> {WeightedUnDirGraphMtEphFloat16,744
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges18,866
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge33,1429
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight38,1630
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges43,1837
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted52,2198
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight65,2755
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree73,3042
    macro_rules! WeightedUnDirGraphMtEphFloatLit {WeightedUnDirGraphMtEphFloatLit77,3150
    pub fn __weighted_undir_graph_mt_float_macro_typecheck_exercise() {__weighted_undir_graph_mt_float_macro_typecheck_exercise89,3775

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphInt.rs,1316
pub mod WeightedUnDirGraphStEphInt {WeightedUnDirGraphStEphInt4,193
    pub type WeightedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;WeightedUnDirGraphStEphInt13,524
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphInt<V> {WeightedUnDirGraphStEphInt16,680
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges18,794
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(vadd_weighted_edge33,1343
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, vget_edge_weight36,1510
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges39,1683
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted48,2024
        pub fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum()total_weight61,2561
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree64,2727
        pub fn is_connected(&self) -> bool {is_connected67,2894
    macro_rules! WeightedUnDirGraphStEphIntLit {WeightedUnDirGraphStEphIntLit96,3906
    pub fn __weighted_undir_graph_st_int_macro_typecheck_exercise() {__weighted_undir_graph_st_int_macro_typecheck_exercise108,4502

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphStEph.rs,2679
pub mod UnDirGraphStEph {UnDirGraphStEph4,172
    pub struct UnDirGraphStEph<V: StT + Hash> {UnDirGraphStEph13,402
        V: Set<V>,V14,450
        E: Set<Edge<V>>,E15,469
    pub trait UnDirGraphStEphTrait<V: StT + Hash> {UnDirGraphStEphTrait18,501
        fn empty() -> UnDirGraphStEph<V>;empty21,645
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V>;FromSets24,795
        fn vertices(&self) -> &Set<V>;vertices27,958
        fn edges(&self) -> &Set<Edge<V>>;edges30,1089
        fn sizeV(&self) -> N;sizeV33,1223
        fn sizeE(&self) -> N;sizeE36,1345
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor39,1467
        fn NG(&self, v: &V) -> Set<V>;NG42,1610
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices45,1767
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident48,1917
        fn Degree(&self, v: &V) -> N;Degree51,2066
    impl<V: StT + Hash> UnDirGraphStEphTrait<V> for UnDirGraphStEph<V> {UnDirGraphStEph54,2111
        fn empty() -> UnDirGraphStEph<V> {empty55,2184
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E }FromSets61,2341
        fn vertices(&self) -> &Set<V> { &self.V }vertices62,2440
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges63,2490
        fn sizeV(&self) -> N { self.V.size() }sizeV64,2543
        fn sizeE(&self) -> N { self.E.size() }sizeE65,2590
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor67,2638
        fn NG(&self, v: &V) -> Set<V> {NG77,2962
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices89,3324
        fn Incident(&self, e: &Edge<V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } elseIncident98,3596
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree100,3709
    impl<V: StT + Hash> Debug for UnDirGraphStEph<V> {UnDirGraphStEph103,3775
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt104,3830
    impl<V: StT + Hash> Display for UnDirGraphStEph<V> {UnDirGraphStEph112,4050
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} E={:?}", self.V, self.Efmt113,4107
    impl<V: StT + Hash> PartialEq for UnDirGraphStEph<V> {UnDirGraphStEph116,4214
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq117,4273
    impl<V: StT + Hash> Eq for UnDirGraphStEph<V> {}UnDirGraphStEph119,4365
    macro_rules! UnDirGraphStEphLit {UnDirGraphStEphLit122,4439
    fn _UnDirGraphStEphLit_type_checks() {_UnDirGraphStEphLit_type_checks140,5564
    pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise146,5827

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphInt.rs,1541
pub mod WeightedDirGraphStEphInt {WeightedDirGraphStEphInt4,191
    pub type WeightedDirGraphStEphInt<V> = LabDirGraphStEph<V, i32>;WeightedDirGraphStEphInt13,514
    impl<V: StT + Hash> WeightedDirGraphStEphInt<V> {WeightedDirGraphStEphInt16,664
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges18,770
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(add_weighted_edge33,1309
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(fromget_edge_weight36,1479
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges39,1657
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted48,2006
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted59,2419
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() total_weight70,2829
        pub fn edges_above_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_above_weight73,2985
        pub fn edges_below_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_below_weight84,3430
    macro_rules! WeightedDirGraphStEphIntLit {WeightedDirGraphStEphIntLit96,3847
    fn _WeightedDirGraphStEphIntLit_type_checks() {_WeightedDirGraphStEphIntLit_type_checks108,4433
    pub fn __weighted_dir_graph_st_int_macro_typecheck_exercise() {__weighted_dir_graph_st_int_macro_typecheck_exercise114,4725

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphFloat.rs,1530
pub mod WeightedUnDirGraphStEphFloat {WeightedUnDirGraphStEphFloat30,1202
    pub type WeightedUnDirGraphStEphFloat<V> = LabUnDirGraphStEph<V, OrderedF64>;WeightedUnDirGraphStEphFloat39,1542
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphFloat<V> {WeightedUnDirGraphStEphFloat42,1714
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges44,1830
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge59,2393
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight64,2594
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges69,2801
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted78,3156
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight91,3707
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree99,3994
        pub fn is_connected(&self) -> bool {is_connected102,4161
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge130,5187
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge138,5485
    macro_rules! WeightedUnDirGraphStEphFloatLit {WeightedUnDirGraphStEphFloatLit147,5769
    pub fn __weighted_undir_graph_st_float_macro_typecheck_exercise() {__weighted_undir_graph_st_float_macro_typecheck_exercise159,6394

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphInt.rs,1255
pub mod WeightedUnDirGraphMtEphInt {WeightedUnDirGraphMtEphInt4,192
    pub type WeightedUnDirGraphMtEphInt<V> = LabUnDirGraphMtEph<V, i32>;WeightedUnDirGraphMtEphInt13,539
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphInt<V> {WeightedUnDirGraphMtEphInt16,712
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges18,832
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(vadd_weighted_edge33,1381
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, vget_edge_weight36,1548
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges39,1721
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted48,2068
        pub fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum()total_weight61,2611
        pub fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }vertex_degree64,2777
    macro_rules! WeightedUnDirGraphMtEphIntLit {WeightedUnDirGraphMtEphIntLit68,2885
    pub fn __weighted_undir_graph_mt_int_macro_typecheck_exercise() {__weighted_undir_graph_mt_int_macro_typecheck_exercise80,3481

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphMtEph.rs,2687
pub mod LabUnDirGraphMtEph {LabUnDirGraphMtEph4,211
    pub struct LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph13,444
        vertices: Set<V>,vertices18,563
        labeled_edges: Set<LabEdge<V, L>>,labeled_edges19,589
    pub trait LabUnDirGraphMtEphTrait<V, L>LabUnDirGraphMtEphTrait22,639
        fn empty() -> Self;empty27,762
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges28,790
        fn vertices(&self) -> &Set<V>;vertices29,895
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;labeled_edges30,934
        fn edges(&self) -> Set<Edge<V>>;edges31,990
        fn add_vertex(&mut self, v: V);add_vertex32,1031
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);add_labeled_edge33,1071
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;get_edge_label34,1135
        fn has_edge(&self, v1: &V, v2: &V) -> bool;has_edge35,1199
        fn neighbors(&self, v: &V) -> Set<V>;neighbors36,1251
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;normalize_edge37,1297
    impl<V, L> LabUnDirGraphMtEphTrait<V, L> for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph40,1362
        fn empty() -> Self {empty45,1515
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges52,1687
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices59,1908
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> { &self.labeled_edges }labeled_edges61,1966
        fn edges(&self) -> Set<Edge<V>> {edges63,2046
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex71,2323
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge73,2392
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label84,2784
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge95,3221
        fn neighbors(&self, v: &V) -> Set<V> {neighbors105,3566
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge117,4005
    impl<V, L> Display for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph125,4425
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt130,4556
    impl<V, L> Debug for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph135,4718
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt140,4847
    macro_rules! LabUnDirGraphMtEphLit {LabUnDirGraphMtEphLit150,5119
    fn _LabUnDirGraphMtEphLit_type_checks() {_LabUnDirGraphMtEphLit_type_checks173,6325
    pub fn __lab_undir_graph_mt_macro_typecheck_exercise() {__lab_undir_graph_mt_macro_typecheck_exercise179,6604

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphFloat.rs,1253
pub mod WeightedDirGraphMtEphFloat {WeightedDirGraphMtEphFloat4,197
    pub type WeightedDirGraphMtEphFloat<V> = LabDirGraphMtEph<V, OrderedF64>;WeightedDirGraphMtEphFloat13,545
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphFloat<V> {WeightedDirGraphMtEphFloat16,728
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges18,842
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge33,1395
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight38,1599
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges43,1811
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted52,2180
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted63,2610
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight74,3037
    macro_rules! WeightedDirGraphMtEphFloatLit {WeightedDirGraphMtEphFloatLit83,3283
    pub fn __weighted_dir_graph_mt_float_macro_typecheck_exercise() {__weighted_dir_graph_mt_float_macro_typecheck_exercise95,3896

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphMtEph.rs,2767
pub mod UnDirGraphMtEph {UnDirGraphMtEph4,197
    pub struct UnDirGraphMtEph<V: StT + MtT + Hash> {UnDirGraphMtEph12,372
        V: Set<V>,V13,426
        E: Set<Edge<V>>,E14,445
    pub trait UnDirGraphMtEphTrait<V: StT + MtT + Hash> {UnDirGraphMtEphTrait17,477
        fn empty() -> UnDirGraphMtEph<V>;empty20,627
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V>;FromSets23,777
        fn vertices(&self) -> &Set<V>;vertices26,940
        fn edges(&self) -> &Set<Edge<V>>;edges29,1071
        fn sizeV(&self) -> N;sizeV32,1205
        fn sizeE(&self) -> N;sizeE35,1327
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1449
        fn NG(&self, v: &V) -> Set<V>;NG41,1592
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1749
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident47,1899
        fn Degree(&self, v: &V) -> N;Degree50,2048
    impl<V: StT + MtT + Hash> UnDirGraphMtEphTrait<V> for UnDirGraphMtEph<V> {UnDirGraphMtEph53,2093
        fn empty() -> UnDirGraphMtEph<V> {empty54,2172
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V> { UnDirGraphMtEph { V, E }FromSets60,2329
        fn vertices(&self) -> &Set<V> { &self.V }vertices61,2428
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges62,2478
        fn sizeV(&self) -> N { self.V.size() }sizeV63,2531
        fn sizeE(&self) -> N { self.E.size() }sizeE64,2578
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor66,2626
        fn NG(&self, v: &V) -> Set<V> {NG77,2978
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices89,3346
        fn Incident(&self, e: &Edge<V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } elseIncident98,3618
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree100,3731
    impl<V: StT + MtT + Hash> std::fmt::Debug for UnDirGraphMtEph<V> {UnDirGraphMtEph103,3797
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt104,3868
    impl<V: StT + MtT + Hash> std::fmt::Display for UnDirGraphMtEph<V> {UnDirGraphMtEph112,4108
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} E={fmt113,4181
    impl<V: StT + MtT + Hash> PartialEq for UnDirGraphMtEph<V> {UnDirGraphMtEph116,4308
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq117,4373
    impl<V: StT + MtT + Hash> Eq for UnDirGraphMtEph<V> {}UnDirGraphMtEph119,4465
    macro_rules! UnDirGraphMtEphLit {UnDirGraphMtEphLit122,4545
    fn _UnDirGraphMtEphLit_type_checks() {_UnDirGraphMtEphLit_type_checks140,5670
    pub fn __undirgraph_mt_macro_typecheck_exercise() {__undirgraph_mt_macro_typecheck_exercise146,5922

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphStEph.rs,2619
pub mod LabDirGraphStEph {LabDirGraphStEph4,183
    pub struct LabDirGraphStEph<V, L>LabDirGraphStEph13,414
        vertices: Set<V>,vertices18,514
        labeled_arcs: Set<LabEdge<V, L>>,labeled_arcs19,540
    pub trait LabDirGraphStEphTrait<V, L>LabDirGraphStEphTrait22,589
        fn empty() -> Self;empty27,693
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs28,721
        fn vertices(&self) -> &Set<V>;vertices29,824
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;labeled_arcs30,863
        fn arcs(&self) -> Set<Edge<V>>;arcs31,918
        fn add_vertex(&mut self, v: V);add_vertex32,958
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);add_labeled_arc33,998
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;get_arc_label34,1063
        fn has_arc(&self, from: &V, to: &V) -> bool;has_arc35,1128
        fn out_neighbors(&self, v: &V) -> Set<V>;out_neighbors36,1181
        fn in_neighbors(&self, v: &V) -> Set<V>;in_neighbors37,1231
    impl<V, L> LabDirGraphStEphTrait<V, L> for LabDirGraphStEph<V, L>LabDirGraphStEph40,1287
        fn empty() -> Self {empty45,1419
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs52,1588
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices56,1759
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }labeled_arcs58,1817
        fn arcs(&self) -> Set<Edge<V>> {arcs60,1895
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex68,2158
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc70,2227
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label76,2462
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc85,2763
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors94,3037
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors104,3359
    impl<V, L> Display for LabDirGraphStEph<V, L>LabDirGraphStEph115,3686
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt120,3823
    impl<V, L> Debug for LabDirGraphStEph<V, L>LabDirGraphStEph125,3982
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt130,4117
    macro_rules! LabDirGraphStEphLit {LabDirGraphStEphLit140,4385
    fn _LabDirGraphStEphLit_type_checks() {_LabDirGraphStEphLit_type_checks152,5177
    pub fn __lab_dir_graph_macro_typecheck_exercise() {__lab_dir_graph_macro_typecheck_exercise158,5448

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphStEph.rs,3507
pub mod DirGraphStEph {DirGraphStEph4,169
    pub struct DirGraphStEph<V: StT + Hash> {DirGraphStEph13,397
        V: Set<V>,V14,443
        A: Set<Edge<V>>,A15,462
    pub trait DirGraphStEphTrait<V: StT + Hash> {DirGraphStEphTrait18,494
        fn empty() -> DirGraphStEph<V>;empty21,636
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V>;FromSets24,784
        fn vertices(&self) -> &Set<V>;vertices27,945
        fn arcs(&self) -> &Set<Edge<V>>;arcs30,1076
        fn sizeV(&self) -> N;sizeV33,1209
        fn sizeA(&self) -> N;sizeA36,1331
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor39,1453
        fn NG(&self, v: &V) -> Set<V>;NG42,1596
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices45,1753
        fn NPlus(&self, v: &V) -> Set<V>;NPlus48,1907
        fn NMinus(&self, v: &V) -> Set<V>;NMinus51,2045
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices54,2206
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices57,2385
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident60,2539
        fn Degree(&self, v: &V) -> N;Degree63,2691
        fn InDegree(&self, v: &V) -> N;InDegree66,2825
        fn OutDegree(&self, v: &V) -> N;OutDegree69,2961
    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {DirGraphStEph72,3009
        fn empty() -> DirGraphStEph<V> {empty73,3078
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }FromSets79,3231
        fn vertices(&self) -> &Set<V> { &self.V }vertices80,3326
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs81,3376
        fn sizeV(&self) -> N { self.V.size() }sizeV82,3428
        fn sizeA(&self) -> N { self.A.size() }sizeA83,3475
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor85,3523
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG94,3788
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices96,3845
        fn NPlus(&self, v: &V) -> Set<V> {NPlus105,4117
        fn NMinus(&self, v: &V) -> Set<V> {NMinus115,4399
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices125,4682
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices134,4964
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } eIncident143,5250
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree145,5366
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree146,5428
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree147,5493
    impl<V: StT + Hash> Debug for DirGraphStEph<V> {DirGraphStEph150,5565
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt151,5618
    impl<V: StT + Hash> Display for DirGraphStEph<V> {DirGraphStEph159,5836
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.Afmt160,5891
    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {DirGraphStEph163,5998
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq164,6055
    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}DirGraphStEph166,6147
    macro_rules! DirGraphStEphLit {DirGraphStEphLit169,6219
    fn _DirGraphStEphLit_type_checks() {_DirGraphStEphLit_type_checks186,7312
    pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise192,7567

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphMtEph.rs,3595
pub mod DirGraphMtEph {DirGraphMtEph4,194
    pub struct DirGraphMtEph<V: StT + MtT + Hash> {DirGraphMtEph12,367
        V: Set<V>,V13,419
        A: Set<Edge<V>>,A14,438
    pub trait DirGraphMtEphTrait<V: StT + MtT + Hash> {DirGraphMtEphTrait17,470
        fn empty() -> DirGraphMtEph<V>;empty20,618
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V>;FromSets23,766
        fn vertices(&self) -> &Set<V>;vertices26,927
        fn arcs(&self) -> &Set<Edge<V>>;arcs29,1058
        fn sizeV(&self) -> N;sizeV32,1191
        fn sizeA(&self) -> N;sizeA35,1313
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1435
        fn NG(&self, v: &V) -> Set<V>;NG41,1578
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1735
        fn NPlus(&self, v: &V) -> Set<V>;NPlus47,1889
        fn NMinus(&self, v: &V) -> Set<V>;NMinus50,2027
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices53,2188
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices56,2367
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident59,2521
        fn Degree(&self, v: &V) -> N;Degree62,2673
        fn InDegree(&self, v: &V) -> N;InDegree65,2807
        fn OutDegree(&self, v: &V) -> N;OutDegree68,2943
    impl<V: StT + MtT + Hash> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {DirGraphMtEph71,2991
        fn empty() -> DirGraphMtEph<V> {empty72,3066
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V> { DirGraphMtEph { V, A } }FromSets78,3219
        fn vertices(&self) -> &Set<V> { &self.V }vertices79,3314
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs80,3364
        fn sizeV(&self) -> N { self.V.size() }sizeV81,3416
        fn sizeA(&self) -> N { self.A.size() }sizeA82,3463
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor84,3511
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG93,3782
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices95,3839
        fn NPlus(&self, v: &V) -> Set<V> {NPlus104,4111
        fn NMinus(&self, v: &V) -> Set<V> {NMinus114,4396
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices124,4682
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices133,4964
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } eIncident142,5250
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree144,5366
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree145,5428
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree146,5493
    impl<V: StT + MtT + Hash> std::fmt::Debug for DirGraphMtEph<V> {DirGraphMtEph149,5565
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt150,5634
    impl<V: StT + MtT + Hash> std::fmt::Display for DirGraphMtEph<V> {DirGraphMtEph158,5872
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} A={fmt159,5943
    impl<V: StT + MtT + Hash> PartialEq for DirGraphMtEph<V> {DirGraphMtEph162,6070
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq163,6133
    impl<V: StT + MtT + Hash> Eq for DirGraphMtEph<V> {}DirGraphMtEph165,6225
    macro_rules! DirGraphMtEphLit {DirGraphMtEphLit168,6303
    fn _DirGraphMtEphLit_type_checks() {_DirGraphMtEphLit_type_checks185,7396
    pub fn __dirgraph_mt_macro_typecheck_exercise() {__dirgraph_mt_macro_typecheck_exercise191,7640

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphFloat.rs,1907
pub mod WeightedDirGraphStEphFloat {WeightedDirGraphStEphFloat30,1167
    pub type WeightedDirGraphStEphFloat<V> = LabDirGraphStEph<V, OrderedF64>;WeightedDirGraphStEphFloat39,1499
    impl<V: StT + Hash> WeightedDirGraphStEphFloat<V> {WeightedDirGraphStEphFloat42,1665
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges44,1773
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge59,2326
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight64,2530
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges69,2742
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted78,3105
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted89,3532
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight100,3956
        pub fn edges_above_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_above_weight108,4233
        pub fn edges_below_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_below_weight119,4706
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge130,5165
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge138,5462
        pub fn scale_weights(&mut self, factor: OrderedFloat<f64>) {scale_weights146,5761
    macro_rules! WeightedDirGraphStEphFloatLit {WeightedDirGraphStEphFloatLit164,6384
    fn _WeightedDirGraphStEphFloatLit_type_checks() {_WeightedDirGraphStEphFloatLit_type_checks176,6997
    pub fn __weighted_dir_graph_st_float_macro_typecheck_exercise() {__weighted_dir_graph_st_float_macro_typecheck_exercise182,7299

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphInt.rs,1232
pub mod WeightedDirGraphMtEphInt {WeightedDirGraphMtEphInt4,190
    pub type WeightedDirGraphMtEphInt<V> = LabDirGraphMtEph<V, i32>;WeightedDirGraphMtEphInt13,529
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphInt<V> {WeightedDirGraphMtEphInt16,696
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges18,808
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(add_weighted_edge33,1347
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(fromget_edge_weight36,1517
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges39,1695
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted48,2050
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted59,2466
        pub fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() total_weight70,2879
    macro_rules! WeightedDirGraphMtEphIntLit {WeightedDirGraphMtEphIntLit74,3004
    pub fn __weighted_dir_graph_mt_int_macro_typecheck_exercise() {__weighted_dir_graph_mt_int_macro_typecheck_exercise86,3590

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphStEph.rs,2680
pub mod LabUnDirGraphStEph {LabUnDirGraphStEph4,186
    pub struct LabUnDirGraphStEph<V, L>LabUnDirGraphStEph13,419
        vertices: Set<V>,vertices18,527
        labeled_edges: Set<LabEdge<V, L>>,labeled_edges19,553
    pub trait LabUnDirGraphStEphTrait<V, L>LabUnDirGraphStEphTrait22,603
        fn empty() -> Self;empty27,715
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges28,743
        fn vertices(&self) -> &Set<V>;vertices29,848
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;labeled_edges30,887
        fn edges(&self) -> Set<Edge<V>>;edges31,943
        fn add_vertex(&mut self, v: V);add_vertex32,984
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);add_labeled_edge33,1024
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;get_edge_label34,1088
        fn has_edge(&self, v1: &V, v2: &V) -> bool;has_edge35,1152
        fn neighbors(&self, v: &V) -> Set<V>;neighbors36,1204
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;normalize_edge37,1250
    impl<V, L> LabUnDirGraphStEphTrait<V, L> for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph40,1315
        fn empty() -> Self {empty45,1457
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges52,1629
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices59,1850
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> { &self.labeled_edges }labeled_edges61,1908
        fn edges(&self) -> Set<Edge<V>> {edges63,1988
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex71,2259
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge73,2328
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label84,2714
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge95,3151
        fn neighbors(&self, v: &V) -> Set<V> {neighbors106,3560
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge118,3993
    impl<V, L> Display for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph126,4413
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt131,4533
    impl<V, L> Debug for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph136,4695
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt141,4813
    macro_rules! LabUnDirGraphStEphLit {LabUnDirGraphStEphLit151,5085
    fn _LabUnDirGraphStEphLit_type_checks() {_LabUnDirGraphStEphLit_type_checks174,6291
    pub fn __lab_undir_graph_macro_typecheck_exercise() {__lab_undir_graph_macro_typecheck_exercise180,6570

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphMtEph.rs,2627
pub mod LabDirGraphMtEph {LabDirGraphMtEph4,208
    pub struct LabDirGraphMtEph<V, L>LabDirGraphMtEph13,439
        vertices: Set<V>,vertices18,550
        labeled_arcs: Set<LabEdge<V, L>>,labeled_arcs19,576
    pub trait LabDirGraphMtEphTrait<V, L>LabDirGraphMtEphTrait22,625
        fn empty() -> Self;empty27,740
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs28,768
        fn vertices(&self) -> &Set<V>;vertices29,871
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;labeled_arcs30,910
        fn arcs(&self) -> Set<Edge<V>>;arcs31,965
        fn add_vertex(&mut self, v: V);add_vertex32,1005
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);add_labeled_arc33,1045
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;get_arc_label34,1110
        fn has_arc(&self, from: &V, to: &V) -> bool;has_arc35,1175
        fn out_neighbors(&self, v: &V) -> Set<V>;out_neighbors36,1228
        fn in_neighbors(&self, v: &V) -> Set<V>;in_neighbors37,1278
    impl<V, L> LabDirGraphMtEphTrait<V, L> for LabDirGraphMtEph<V, L>LabDirGraphMtEph40,1334
        fn empty() -> Self {empty45,1477
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs52,1646
        fn vertices(&self) -> &Set<V> { &self.vertices }vertices56,1817
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }labeled_arcs58,1875
        fn arcs(&self) -> Set<Edge<V>> {arcs60,1953
        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }add_vertex68,2222
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc70,2291
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label76,2532
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc85,2833
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors94,3107
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors104,3432
    impl<V, L> Display for LabDirGraphMtEph<V, L>LabDirGraphMtEph115,3762
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt120,3885
    impl<V, L> Debug for LabDirGraphMtEph<V, L>LabDirGraphMtEph125,4044
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt130,4165
    macro_rules! LabDirGraphMtEphLit {LabDirGraphMtEphLit140,4433
    fn _LabDirGraphMtEphLit_type_checks() {_LabDirGraphMtEphLit_type_checks152,5225
    pub fn __lab_dir_graph_mt_macro_typecheck_exercise() {__lab_dir_graph_mt_macro_typecheck_exercise158,5496

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap11/FibonacciMt.rs,579
pub mod FibonacciMt {FibonacciMt5,223
    pub struct FibonacciMt;FibonacciMt8,290
    pub trait FibonacciMtTrait {FibonacciMtTrait10,319
        fn fib(n: N) -> N;fib11,352
    impl FibonacciMt {FibonacciMt14,386
        pub fn fib(n: N) -> N {fib15,409
    impl FibonacciMtTrait for FibonacciMt {FibonacciMt25,681
        fn fib(n: N) -> N {fib26,725
    mod tests {tests32,819
        fn fib_base_cases() {fib_base_cases37,940
        fn fib_small_values() {fib_small_values43,1093
        fn trait_and_inherent_agree() {trait_and_inherent_agree50,1297

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_5.rs,1374
pub mod Exercise12_5 {Exercise12_54,178
    struct Node<T: StTInMtT> {Node11,352
        value: ManuallyDrop<T>,value12,383
        next: *mut Node<T>,next13,415
    pub struct ConcurrentStackMt<T: StTInMtT> {ConcurrentStackMt18,532
        head: AtomicPtr<Node<T>>,head19,580
    pub trait ConcurrentStackMtTrait<T: StTInMtT> {ConcurrentStackMtTrait22,621
        fn new() -> Self;new23,673
        fn push(&self, value: T);push24,699
        fn pop(&self) -> Option<T>;pop25,733
        fn is_empty(&self) -> bool;is_empty26,769
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt29,812
        fn raw_pop(&self) -> Option<*mut Node<T>> {raw_pop30,857
    impl<T: StTInMtT> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {ConcurrentStackMt48,1406
        fn new() -> Self {new49,1481
        fn push(&self, value: T) {push53,1586
        fn pop(&self) -> Option<T> {pop70,2223
        fn is_empty(&self) -> bool {is_empty77,2462
    impl<T: StTInMtT> Default for ConcurrentStackMt<T> {ConcurrentStackMt82,2572
        fn default() -> Self { Self::new() }default83,2629
    impl<T: StTInMtT> Drop for ConcurrentStackMt<T> {ConcurrentStackMt86,2681
        fn drop(&mut self) {drop87,2735
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt100,3139
        pub fn drain(&self) -> Vec<T> {drain102,3259

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_2.rs,478
pub mod Exercise12_2 {Exercise12_24,175
    pub trait FetchAddCasTrait {FetchAddCasTrait7,251
        fn fetch_add_cas(&self, delta: usize) -> usize;fetch_add_cas8,284
    impl FetchAddCasTrait for AtomicUsize {AtomicUsize11,347
        fn fetch_add_cas(&self, delta: usize) -> usize {fetch_add_cas12,391
    pub fn fetch_add_cas(target: &AtomicUsize, delta: usize) -> usize {fetch_add_cas24,845
    pub fn efficiency_note() -> &'static str {efficiency_note28,960

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_1.rs,1019
pub mod Exercise12_1 {Exercise12_14,164
    pub struct SpinLock {SpinLock13,369
        ticket: AtomicUsize,ticket14,395
        turn: AtomicUsize,turn15,424
    pub trait SpinLockTrait {SpinLockTrait18,458
        fn new() -> Self;new19,488
        fn lock(&self);lock20,514
        fn unlock(&self);unlock21,538
    impl SpinLock {SpinLock24,571
        pub fn new() -> Self {new25,591
        pub fn lock(&self) {lock29,713
        pub fn unlock(&self) {unlock36,936
        pub fn with_lock<T>(&self, action: impl FnOnce() -> T) -> T {with_lock40,1033
    impl SpinLockTrait for SpinLock {SpinLock48,1226
        fn new() -> Self { SpinLock::new() }new49,1264
        fn lock(&self) { SpinLock::lock(self) }lock51,1310
        fn unlock(&self) { SpinLock::unlock(self) }unlock53,1359
    impl Default for SpinLock {SpinLock56,1418
        fn default() -> Self { SpinLock::new() }default57,1450
    pub fn parallel_increment(iterations: N) -> usize {parallel_increment60,1506

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeq.rs,6339
pub mod ArraySeq {ArraySeq4,150
    pub struct ArraySeqS<T> {ArraySeqS12,332
        data: Box<[T]>,data13,362
    pub trait ArraySeq<T> {ArraySeq18,469
        fn new(length: N, init_value: T) -> ArraySeqS<T>new21,646
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str>;set27,843
        fn length(&self) -> N;length31,1046
        fn nth(&self, index: N) -> &T;nth35,1212
        fn empty() -> ArraySeqS<T>;empty39,1363
        fn singleton(item: T) -> ArraySeqS<T>;singleton43,1535
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqS<T>;tabulate47,1719
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> ArraySeqS<U>;map51,1902
        fn subseq(a: &ArraySeqS<T>, start: N, length: N) -> ArraySeqS<T>subseq55,2141
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> ArraySeqS<T>;append61,2369
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqS<T>, pred: &F) -> ArraySeqS<T>;filter65,2559
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> ArraySeqS<T>;flatten69,2771
        fn update(a: &ArraySeqS<T>, update: Pair<N, T>) -> ArraySeqS<T>;update73,2978
        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T>;inject77,3200
        fn isEmpty(&self) -> B;isEmpty81,3412
        fn isSingleton(&self) -> B;isSingleton85,3573
        fn collect<K: Clone + Eq, V: Clone>(collect89,3775
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqS<T>, f: &F, seed: A) -> A;iterate95,4019
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> Treduce99,4245
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (ArraySeqS<T>, T)scan105,4488
    impl<T: Clone> ArraySeqS<T> {ArraySeqS110,4620
        fn new(length: N, init_value: T) -> ArraySeqS<T>new111,4654
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {set118,4870
        fn length(&self) -> N { self.data.len() }length127,5156
        fn nth(&self, index: N) -> &T { &self.data[index] }nth129,5207
        fn empty() -> ArraySeqS<T> { ArraySeqS::from_vec(Vec::new()) }empty131,5268
        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::from_vec(vec![item]) }singleton133,5340
        fn isEmpty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }isEmpty135,5423
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton137,5513
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq142,5834
        pub fn subseq_copy(&self, start: N, length: N) -> ArraySeqS<T>subseq_copy151,6444
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqS<T> {update165,7126
        pub fn from_vec(elts: Vec<T>) -> ArraySeqS<T> {from_vec176,7656
        pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter182,7808
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }iter_mut184,7884
    impl<T: Clone> ArraySeq<T> for ArraySeqS<T> {ArraySeqS187,7981
        fn new(length: N, init_value: T) -> ArraySeqS<T>new188,8031
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {set193,8156
        fn length(&self) -> N { ArraySeqS::length(self) }length197,8303
        fn nth(&self, index: N) -> &T { ArraySeqS::nth(self, index) }nth199,8362
        fn empty() -> ArraySeqS<T> { ArraySeqS::empty() }empty201,8433
        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::singleton(item) }singleton203,8492
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqS<T> {tabulate205,8570
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> ArraySeqS<U> {map213,8839
        fn subseq(a: &ArraySeqS<T>, start: N, length: N) -> ArraySeqS<T>subseq222,9156
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> ArraySeqS<T>append227,9291
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqS<T>, pred: &F) -> ArraySeqS<T>filter243,9829
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> ArraySeqS<T>flatten255,10210
        fn update(a: &ArraySeqS<T>, Pair(index, item): Pair<N, T>) -> ArraySeqS<T>update267,10591
        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T>inject276,10915
        fn isEmpty(&self) -> B { ArraySeqS::isEmpty(self) }isEmpty289,11441
        fn isSingleton(&self) -> B { ArraySeqS::isSingleton(self) }isSingleton291,11502
        fn collect<K: Clone + Eq, V: Clone>(collect293,11571
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqS<T>, f: &F, seed: A) -> A {iterate316,12484
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> Treduce324,12718
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (ArraySeqS<T>, T)scan333,12952
    impl<T: PartialEq> PartialEq for ArraySeqS<T> {ArraySeqS346,13374
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq347,13426
    impl<T: Eq> Eq for ArraySeqS<T> {}ArraySeqS350,13504
    impl<T: Debug> Debug for ArraySeqS<T> {ArraySeqS352,13544
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt353,13588
    impl<T: Display> Display for ArraySeqS<T> {ArraySeqS356,13707
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt357,13755
    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {ArraySeqS369,14085
        type Item = &'a T;Item370,14137
        type IntoIter = std::slice::Iter<'a, T>;IntoIter371,14164
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }into_iter373,14214
    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {ArraySeqS376,14287
        type Item = &'a mut T;Item377,14343
        type IntoIter = std::slice::IterMut<'a, T>;IntoIter378,14374
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }into_iter380,14427
    impl<T> IntoIterator for ArraySeqS<T> {ArraySeqS383,14504
        type Item = T;Item384,14548
        type IntoIter = std::vec::IntoIter<T>;IntoIter385,14571
        fn into_iter(self) -> Self::IntoIter { Vec::from(self.data).into_iter() }into_iter387,14619
    macro_rules! ArraySeqS {ArraySeqS391,14728
    fn _arrayseqs_macro_type_checks() {_arrayseqs_macro_type_checks404,15164

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStPer.rs,5383
pub mod LinkedListStPer {LinkedListStPer4,140
    pub struct NodeP<T: StT> {NodeP10,263
        pub value: T,value11,294
        pub next: Option<Box<NodeP<T>>>,next12,316
    pub struct LinkedListStPerS<T: StT> {LinkedListStPerS16,392
        head: Option<Box<NodeP<T>>>,head17,434
        len: N,len18,471
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait21,494
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>new22,539
        fn empty() -> LinkedListStPerS<T>;empty25,639
        fn singleton(item: T) -> LinkedListStPerS<T>;singleton26,682
        fn length(&self) -> N;length27,736
        fn nth(&self, index: N) -> &T;nth28,767
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T>;subseq_copy29,806
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> LinkedListStPerS<T>;tabulate30,881
        fn map<U: StT, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> LinkedListStPerS<U>;map31,953
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append32,1048
        fn select(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>, index: N) -> Option<T>;select33,1140
        fn filter<F: Fn(&T) -> B>(a: &LinkedListStPerS<T>, pred: &F) -> LinkedListStPerS<T>;filter34,1232
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;update35,1325
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject36,1413
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject37,1520
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> A;iterate38,1628
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> iteratePrefixes39,1719
        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T;reduce40,1841
        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<scan42,1925
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten44,2030
        fn collect<A: StT, Bv: StT>(collect46,2118
    impl<T: StT> LinkedListStPerS<T> {LinkedListStPerS52,2305
        pub fn empty() -> Self { LinkedListStPerS { head: None, len: 0 } }empty53,2344
        pub fn new(length: N, init_value: T) -> Selfnew55,2420
        pub fn singleton(item: T) -> Self { LinkedListStPerS::from_vec(vec![item]) }singleton62,2595
        pub fn from_vec(elts: Vec<T>) -> Self {from_vec64,2681
        pub fn length(&self) -> N { self.len }length74,3031
        pub fn nth(&self, index: N) -> &T {nth76,3079
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy82,3254
        fn node_at(&self, index: N) -> Option<&NodeP<T>> {node_at112,4336
    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {LinkedListStPerS129,4811
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt130,4872
    impl<T: StT> PartialEq for LinkedListStPerS<T> {LinkedListStPerS147,5405
        fn eq(&self, other: &Self) -> bool {eq148,5458
    impl<T: StT> Eq for LinkedListStPerS<T> {}LinkedListStPerS165,5970
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS167,6018
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>new168,6085
        fn empty() -> LinkedListStPerS<T> { LinkedListStPerS::empty() }empty175,6260
        fn singleton(item: T) -> LinkedListStPerS<T> { LinkedListStPerS::singleton(item) }singleton176,6332
        fn length(&self) -> N { LinkedListStPerS::length(self) }length177,6423
        fn nth(&self, index: N) -> &T { LinkedListStPerS::nth(self, index) }nth178,6488
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T> {subseq_copy179,6565
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> LinkedListStPerS<T> {tabulate183,6715
        fn map<U: StT, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> LinkedListStPerS<U> {map191,6983
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append199,7299
        fn select(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>, index: N) -> Option<T> {select210,7728
        fn filter<F: Fn(&T) -> B>(a: &LinkedListStPerS<T>, pred: &F) -> LinkedListStPerS<T> {filter224,8255
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T>update235,8649
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject248,9138
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject261,9732
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> A {iterate272,10204
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> iteratePrefixes280,10444
        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T {reduce290,10878
        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<scan306,11502
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten321,12200
        fn collect<A: StT, Bv: StT>(collect332,12604

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStEph.rs,6265
pub mod ArraySeqStEph {ArraySeqStEph4,164
    pub struct ArraySeqStEphS<T: StT> {ArraySeqStEphS11,346
        data: Box<[T]>,data12,386
    pub type ArrayStEph<T> = ArraySeqStEphS<T>;ArrayStEph15,417
    impl<T: StT> ArraySeqStEphS<T> {ArraySeqStEphS17,466
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }from_vec18,503
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) new20,593
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }empty22,692
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton24,755
        pub fn length(&self) -> N { self.data.len() }length26,829
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth28,884
        pub fn iter(&self) -> std::slice::Iter<'_, T> {iter31,998
        pub fn subseq(&self, start: N, length: N) -> Self {subseq35,1094
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set42,1387
        pub fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {update51,1669
        pub fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut Self {inject56,1819
    impl<T: StT> PartialEq for ArraySeqStEphS<T> {ArraySeqStEphS68,2253
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }eq69,2304
    impl<T: StT> Eq for ArraySeqStEphS<T> {}ArraySeqStEphS72,2390
    impl<T: StT> Debug for ArraySeqStEphS<T> {ArraySeqStEphS74,2436
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt75,2483
    impl<'a, T: StT> IntoIterator for &'a ArraySeqStEphS<T> {ArraySeqStEphS78,2602
        type Item = &'a T;Item79,2664
        type IntoIter = std::slice::Iter<'a, T>;IntoIter80,2691
        fn into_iter(self) -> Self::IntoIter {into_iter82,2749
    impl<T: StT> IntoIterator for ArraySeqStEphS<T> {ArraySeqStEphS87,2842
        type Item = T;Item88,2896
        type IntoIter = std::vec::IntoIter<T>;IntoIter89,2919
        fn into_iter(self) -> Self::IntoIter {into_iter91,2975
    impl<T: StT> Display for ArraySeqStEphS<T> {ArraySeqStEphS96,3084
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt97,3133
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait109,3463
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>;new110,3506
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str>;set111,3569
        fn length(&self) -> N;length112,3663
        fn nth(&self, index: N) -> &T;nth113,3694
        fn empty() -> ArraySeqStEphS<T>;empty114,3733
        fn singleton(item: T) -> ArraySeqStEphS<T>;singleton115,3774
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStEphS<T>;tabulate116,3826
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U>;map117,3901
        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T>;subseq118,3992
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append119,4076
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T>;filter120,4162
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten121,4251
        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T>;update122,4331
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T>;inject123,4407
        fn isEmpty(&self) -> B;isEmpty124,4501
        fn isSingleton(&self) -> B;isSingleton125,4533
        fn collect<K: StT, V: StT>(collect126,4569
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A;iterate130,4744
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T;reduce131,4831
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, scan132,4912
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS135,5019
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::new(length, initnew136,5082
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str> {set138,5189
        fn length(&self) -> N { ArraySeqStEphS::length(self) }length142,5346
        fn nth(&self, index: N) -> &T { ArraySeqStEphS::nth(self, index) }nth144,5410
        fn empty() -> ArraySeqStEphS<T> { ArraySeqStEphS::empty() }empty146,5486
        fn singleton(item: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::singleton(item) }singleton148,5555
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStEphS<T> {tabulate150,5643
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U> {map158,5922
        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T> { a.subseq(stsubseq166,6232
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append168,6344
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T> {filter180,6796
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten191,7184
        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T> { ArraySeqStEphS::updupdate202,7577
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T> {inject204,7694
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty208,7850
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton210,7938
        fn collect<K: StT, V: StT>(collect212,8030
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A {iterate234,8956
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T {reduce242,9195
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, scan250,9426
    macro_rules! ArraySeqStEphS {ArraySeqStEphS262,9855
    fn _arrayseqstephs_macro_type_checks() {_arrayseqstephs_macro_type_checks269,10265

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtPer.rs,6377
pub mod ArraySeqMtPer {ArraySeqMtPer4,174
    pub struct ArraySeqMtPerS<T: StTInMtT> {ArraySeqMtPerS13,409
        data: Box<[T]>,data14,454
    impl<T: StTInMtT> ArraySeqMtPerS<T> {ArraySeqMtPerS17,485
        pub fn empty() -> Self {empty18,527
        pub fn new(length: N, init_value: T) -> Self {new24,667
        pub fn singleton(item: T) -> Self { ArraySeqMtPerS::from_vec(vec![item]) }singleton32,939
        pub fn from_vec(values: Vec<T>) -> Self {from_vec34,1023
        pub fn length(&self) -> N { self.data.len() }length40,1176
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth42,1231
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy44,1296
        pub fn is_empty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }is_empty52,1621
        pub fn is_singleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } is_singleton54,1716
        pub fn iter(&self) -> std::slice::Iter<'_, T> {iter57,1864
    impl<T: StTInMtT> Clone for ArraySeqMtPerS<T> {ArraySeqMtPerS62,1966
        fn clone(&self) -> Self {clone63,2018
    impl<T: StTInMtT> PartialEq for ArraySeqMtPerS<T> {ArraySeqMtPerS69,2184
        fn eq(&self, other: &Self) -> bool {eq70,2240
    impl<T: StTInMtT + Eq> Eq for ArraySeqMtPerS<T> {}ArraySeqMtPerS83,2575
    impl<'a, T: StTInMtT> IntoIterator for &'a ArraySeqMtPerS<T> {ArraySeqMtPerS85,2631
        type Item = &'a T;Item86,2698
        type IntoIter = std::slice::Iter<'a, T>;IntoIter87,2725
        fn into_iter(self) -> Self::IntoIter {into_iter89,2783
    impl<T: StTInMtT> IntoIterator for ArraySeqMtPerS<T> {ArraySeqMtPerS94,2876
        type Item = T;Item95,2935
        type IntoIter = std::vec::IntoIter<T>;IntoIter96,2958
        fn into_iter(self) -> Self::IntoIter {into_iter98,3014
    impl<T: StTInMtT> std::fmt::Display for ArraySeqMtPerS<T> {ArraySeqMtPerS103,3123
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt104,3187
    pub trait ArraySeqMtPerTrait<T: StTInMtT> {ArraySeqMtPerTrait114,3512
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;new115,3560
        fn empty() -> ArraySeqMtPerS<T>;empty116,3623
        fn singleton(item: T) -> ArraySeqMtPerS<T>;singleton117,3664
        fn length(&self) -> N;length118,3716
        fn nth(&self, index: N) -> &T;nth119,3747
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;subseq_copy120,3786
        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str>;set121,3859
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T>;tabulate122,3944
        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(a: &ArraySmap123,4028
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append124,4182
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtPerS<T>, pred: &F) -> ArraySeqMtPefilter125,4268
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T>;update126,4371
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject127,4455
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, iterate128,4557
        fn iteratePrefixes<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>,iteratePrefixes129,4665
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: reduce130,4802
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (Arrayscan131,4931
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;flatten132,5045
        fn collect(collect133,5126
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerSinject137,5281
        fn isEmpty(&self) -> B;isEmpty138,5382
        fn isSingleton(&self) -> B;isSingleton139,5414
    impl<T: StTInMtT> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {ArraySeqMtPerS142,5457
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::new(length, initnew143,5525
        fn empty() -> ArraySeqMtPerS<T> { ArraySeqMtPerS::empty() }empty144,5631
        fn singleton(item: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::singleton(item) }singleton145,5699
        fn length(&self) -> N { ArraySeqMtPerS::length(self) }length146,5786
        fn nth(&self, index: N) -> &T { ArraySeqMtPerS::nth(self, index) }nth147,5849
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::subseqsubseq_copy148,5924
        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str> {set150,6050
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T> {tabulate159,6417
        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(a: &ArraySmap167,6695
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append188,7621
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtPerS<T>, pred: &F) -> ArraySeqMtPefilter199,8042
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T> {update210,8447
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject220,8829
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, iterate231,9262
        fn iteratePrefixes<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>,iteratePrefixes239,9519
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: reduce249,9952
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (Arrayscan285,11556
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {flatten295,11975
        fn collect(collect306,12383
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerSinject333,13518
        fn isEmpty(&self) -> B { ArraySeqMtPerS::is_empty(self) }isEmpty345,14034
        fn isSingleton(&self) -> B { ArraySeqMtPerS::is_singleton(self) }isSingleton347,14101

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtEph.rs,5744
pub mod ArraySeqMtEph {ArraySeqMtEph4,159
    pub struct ArraySeqMtEphS<T: StT> {ArraySeqMtEphS13,417
        data: Mutex<Box<[T]>>,data14,457
    impl<T: StT> ArraySeqMtEphS<T> {ArraySeqMtEphS17,495
        pub fn empty() -> Self {empty18,532
        pub fn new(length: N, init_value: T) -> Selfnew24,684
        pub fn singleton(item: T) -> Self { ArraySeqMtEphS::from_vec(vec![item]) }singleton31,857
        pub fn from_vec(values: Vec<T>) -> Self {from_vec33,941
        pub fn length(&self) -> N {length39,1106
        pub fn nth_cloned(&self, index: N) -> T {nth_cloned44,1228
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set49,1373
        pub fn iter_cloned(&self) -> Vec<T> {iter_cloned62,1815
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy67,1967
        pub fn to_vec(&self) -> Vec<T> {to_vec76,2335
    impl<T: StT> Clone for ArraySeqMtEphS<T> {ArraySeqMtEphS82,2488
        fn clone(&self) -> Self { ArraySeqMtEphS::from_vec(self.to_vec()) }clone83,2535
    impl<T: StT> PartialEq for ArraySeqMtEphS<T> {ArraySeqMtEphS86,2618
        fn eq(&self, other: &Self) -> bool { self.to_vec() == other.to_vec() }eq87,2669
    impl<T: StT> Eq for ArraySeqMtEphS<T> {}ArraySeqMtEphS90,2755
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait92,2801
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>;new93,2844
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str>;set94,2907
        fn length(&self) -> N;length95,3001
        fn nth_cloned(&self, index: N) -> T;nth_cloned96,3032
        fn empty() -> ArraySeqMtEphS<T>;empty97,3077
        fn singleton(item: T) -> ArraySeqMtEphS<T>;singleton98,3118
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;tabulate99,3170
        fn map<U: StT + Send + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &Arramap100,3254
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;subseq_copy101,3417
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append102,3490
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtEphS<T>, pred: &F) -> ArraySeqMtEpfilter103,3576
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;update104,3679
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject105,3768
        fn isEmpty(&self) -> B;isEmpty106,3869
        fn isSingleton(&self) -> B;isSingleton107,3901
        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;flatten108,3937
        fn collect(collect109,4018
        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A)iterate113,4173
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: reduce114,4276
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (Arrayscan115,4412
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject116,4526
    impl<T: StT> std::fmt::Display for ArraySeqMtEphS<T> {ArraySeqMtEphS119,4635
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt120,4694
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS131,5066
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T> {new132,5129
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str> {set136,5256
        fn length(&self) -> N { ArraySeqMtEphS::length(self) }length140,5413
        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphS::nth_cloned(self, index) }nth_cloned142,5477
        fn empty() -> ArraySeqMtEphS<T> { ArraySeqMtEphS::empty() }empty144,5566
        fn singleton(item: T) -> ArraySeqMtEphS<T> { ArraySeqMtEphS::singleton(item) }singleton146,5635
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {tabulate148,5723
        fn map<U: StT + Send + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &Arramap156,6001
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {subseq_copy186,7323
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append190,7469
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtEphS<T>, pred: &F) -> ArraySeqMtEpfilter203,7922
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update215,8347
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject220,8508
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty232,8949
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton234,9037
        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {flatten236,9129
        fn collect(collect247,9531
        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A)iterate277,10787
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: reduce286,11080
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (Arrayscan322,12690
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject333,13150
    macro_rules! ArraySeqMtEphSLit {ArraySeqMtEphSLit344,13503
    fn _ArraySeqMtEphSLit_type_checks() {_ArraySeqMtEphSLit_type_checks351,13916

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStEph.rs,5241
pub mod LinkedListStEph {LinkedListStEph4,152
    pub struct NodeE<T: StT> {NodeE10,275
        pub value: T,value11,306
        pub next: Option<Box<NodeE<T>>>,next12,328
    pub struct LinkedListStEphS<T: StT> {LinkedListStEphS16,404
        head: Option<Box<NodeE<T>>>,head17,446
        len: N,len18,483
    impl<T: StT> LinkedListStEphS<T> {LinkedListStEphS21,506
        pub fn empty() -> Self { LinkedListStEphS { head: None, len: 0 } }empty22,545
        pub fn new(length: N, init_value: T) -> Selfnew24,621
        pub fn singleton(item: T) -> Self { LinkedListStEphS::from_vec(vec![item]) }singleton31,796
        pub fn from_vec(mut elts: Vec<T>) -> Self {from_vec33,882
        pub fn length(&self) -> N { self.len }length42,1209
        pub fn nth(&self, index: N) -> &T {nth44,1257
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set50,1432
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy60,1761
        fn node_at(&self, index: N) -> Option<&NodeE<T>> {node_at90,2843
        fn node_at_mut(&mut self, index: N) -> Option<&mut NodeE<T>> {node_at_mut106,3312
    impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {LinkedListStEphS123,3807
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt124,3868
    impl<T: StT> PartialEq for LinkedListStEphS<T> {LinkedListStEphS141,4401
        fn eq(&self, other: &Self) -> bool {eq142,4454
    impl<T: StT> Eq for LinkedListStEphS<T> {}LinkedListStEphS159,4966
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait161,5014
        fn new(length: N, init_value: T) -> Selfnew162,5059
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set166,5145
        fn length(&self) -> N;length168,5227
        fn nth(&self, index: N) -> &T;nth170,5259
        fn empty() -> Self;empty172,5299
        fn singleton(item: T) -> Self;singleton174,5328
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self;tabulate178,5460
        fn map<U: StT, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U>;map181,5613
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy182,5693
        fn append(a: &Self, b: &Self) -> Self;append185,5861
        fn filter<F: Fn(&T) -> B>(a: &Self, pred: &F) -> Self;filter188,6004
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> Self;deflate190,6137
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten191,6195
        fn update(a: &mut Self, item_at: Pair<N, T>) -> &mut Self;update194,6390
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;inject197,6577
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;ninject200,6774
        fn collect<A: StT, Bv: StT>(collect201,6852
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> A;iterate205,7032
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> (LinkedListStEpiteratePrefixes206,7108
        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T;reduce207,7215
        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (LinkedListStEphS<T>, T);scan208,7283
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS211,7379
        fn new(length: N, init_value: T) -> Selfnew212,7446
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set219,7606
        fn length(&self) -> N { LinkedListStEphS::length(self) }length223,7752
        fn nth(&self, index: N) -> &T { LinkedListStEphS::nth(self, index) }nth225,7818
        fn empty() -> Self { LinkedListStEphS::empty() }empty227,7896
        fn singleton(item: T) -> Self { LinkedListStEphS::singleton(item) }singleton229,7954
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self {tabulate231,8031
        fn map<U: StT, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U> {map239,8284
        fn subseq_copy(&self, start: N, length: N) -> Self { LinkedListStEphS::subseq_copy(self,subseq_copy247,8585
        fn append(a: &Self, b: &Self) -> Self {append249,8700
        fn filter<F: Fn(&T) -> B>(a: &Self, pred: &F) -> Self {filter260,9084
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> Self {deflate271,9448
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten280,9761
        fn update(a: &mut Self, Pair(index, item): Pair<N, T>) -> &mut Self {update291,10165
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {inject296,10308
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {ninject308,10732
        fn collect<A: StT, Bv: StT>(collect317,11036
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> A {iterate337,11885
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> (LinkedListStEpiteratePrefixes345,12110
        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T {reduce355,12529
        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (LinkedListStEphS<T>, T) {scan371,13138

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStPer.rs,5644
pub mod ArraySeqStPer {ArraySeqStPer4,171
    pub struct ArraySeqStPerS<T: StT> {ArraySeqStPerS12,439
        data: Box<[T]>,data13,479
    pub type ArrayStPer<T> = ArraySeqStPerS<T>;ArrayStPer16,510
    impl<T: StT> ArraySeqStPerS<T> {ArraySeqStPerS18,559
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }from_vec19,596
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) new20,685
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }empty21,783
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton22,845
        pub fn length(&self) -> N { self.data.len() }length23,918
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth24,972
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy25,1036
        pub fn iter(&self) -> std::slice::Iter<'_, T> {iter34,1422
    impl<T: StT> PartialEq for ArraySeqStPerS<T> {ArraySeqStPerS39,1524
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }eq40,1575
    impl<T: StT> Eq for ArraySeqStPerS<T> {}ArraySeqStPerS43,1661
    impl<T: StT> Debug for ArraySeqStPerS<T> {ArraySeqStPerS45,1707
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.itefmt46,1754
    impl<'a, T: StT> IntoIterator for &'a ArraySeqStPerS<T> {ArraySeqStPerS49,1873
        type Item = &'a T;Item50,1935
        type IntoIter = std::slice::Iter<'a, T>;IntoIter51,1962
        fn into_iter(self) -> Self::IntoIter {into_iter53,2020
    impl<T: StT> IntoIterator for ArraySeqStPerS<T> {ArraySeqStPerS58,2113
        type Item = T;Item59,2167
        type IntoIter = std::vec::IntoIter<T>;IntoIter60,2190
        fn into_iter(self) -> Self::IntoIter {into_iter62,2246
    impl<T: StT> Display for ArraySeqStPerS<T> {ArraySeqStPerS67,2355
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {fmt68,2404
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait80,2734
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>;new81,2777
        fn length(&self) -> N;length82,2840
        fn nth(&self, index: N) -> &T;nth83,2871
        fn empty() -> ArraySeqStPerS<T>;empty84,2910
        fn singleton(item: T) -> ArraySeqStPerS<T>;singleton85,2951
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStPerS<T>;tabulate86,3003
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U>;map87,3078
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T>;subseq_copy88,3169
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append89,3258
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T>;filter90,3344
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;flatten91,3433
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject92,3513
        fn isEmpty(&self) -> B;isEmpty93,3614
        fn isSingleton(&self) -> B;isSingleton94,3646
        fn collect<K: StT, V: StT>(collect95,3682
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A;iterate99,3853
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T;reduce100,3940
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, scan101,4021
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject102,4121
    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {ArraySeqStPerS105,4230
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::new(length, initnew106,4293
        fn length(&self) -> N { ArraySeqStPerS::length(self) }length107,4399
        fn nth(&self, index: N) -> &T { ArraySeqStPerS::nth(self, index) }nth108,4462
        fn empty() -> ArraySeqStPerS<T> { ArraySeqStPerS::empty() }empty109,4537
        fn singleton(item: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::singleton(item) }singleton110,4605
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStPerS<T> {tabulate112,4693
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U> {map120,4972
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T> {subseq_copy128,5282
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {append132,5424
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {filter143,5845
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {flatten154,6236
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject165,6641
        fn isEmpty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }isEmpty179,7280
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }isSingleton181,7370
        fn collect<K: StT, V: StT>(collect183,7464
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A {iterate210,8615
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T {reduce218,8854
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, scan233,9402
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject244,9840

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEphSlice.rs,5815
pub mod ArraySeqMtEphSlice {ArraySeqMtEphSlice9,487
    struct Inner<T: StT + Send + Sync> {Inner17,676
        data: Mutex<Box<[T]>>,data18,717
    impl<T: StT + Send + Sync> Inner<T> {Inner21,755
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }new22,797
        fn len(&self) -> N {len24,874
    pub struct ArraySeqMtEphSliceS<T: StT + Send + Sync> {ArraySeqMtEphSliceS31,1062
        inner: Arc<Inner<T>>,inner32,1121
        range: Range<N>,range33,1151
    pub trait ArraySeqMtEphSliceTrait<T: StT + Send + Sync> {ArraySeqMtEphSliceTrait37,1247
        fn new(length: N, init_value: T) -> Self;new38,1309
        fn length(&self) -> N;length39,1359
        fn nth_cloned(&self, index: N) -> T;nth_cloned40,1390
        fn empty() -> Self;empty41,1435
        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;update42,1463
        fn singleton(item: T) -> Self;singleton43,1547
        fn isEmpty(&self) -> B;isEmpty44,1586
        fn isSingleton(&self) -> B;isSingleton45,1618
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy46,1654
        fn slice(&self, start: N, length: N) -> Self;slice47,1714
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> Self;tabulate48,1768
        fn map<U: StT + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(amap49,1839
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &Self, pred: F) -> Self whefilter50,2001
        fn append(a: &Self, b: &Self) -> Self;append51,2119
        fn append_select(a: &Self, b: &Self) -> Self;append_select52,2166
        fn flatten(sequences: &[ArraySeqMtEphSliceS<T>]) -> Self;flatten53,2220
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> reduce54,2286
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &Self, f: &F, id: T) -> (ArraySeqMtEphSlicescan55,2409
        fn iterate<A: StT + Send, F: Fn(&A, &T) -> A + Send + Sync>(a: &Self, f: &F, seed: A) ->iterate56,2515
        fn inject(a: &Self, updates: &[(N, T)]) -> Self;inject57,2615
        fn ninject(a: &Self, updates: &[(N, T)]) -> Self;ninject58,2672
    impl<T: StT + Send + Sync> ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS61,2737
        pub fn from_box(data: Box<[T]>) -> Self {from_box63,2854
        pub fn from_vec(data: Vec<T>) -> Self { Self::from_box(data.into_boxed_slice()) }from_vec72,3156
        pub fn to_vec(&self) -> Vec<T> {to_vec75,3328
        pub         fn with_exclusive<F, R>(&self, f: F) -> Rwith_exclusive81,3592
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set92,3972
        fn len(&self) -> N { self.range.end - self.range.start }len96,4106
        fn clamp_subrange(&self, start: N, length: N) -> Range<N> {clamp_subrange98,4172
    impl<T: StT + Send + Sync> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS107,4532
        fn new(length: N, init_value: T) -> Self {new108,4619
        fn length(&self) -> N { self.len() }length113,4784
        fn nth_cloned(&self, index: N) -> T {nth_cloned115,4830
        fn empty() -> Self {empty121,5023
        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {update126,5260
        fn singleton(item: T) -> Self {singleton139,5660
        fn isEmpty(&self) -> B { if self.len() == 0 { B::True } else { B::False } }isEmpty152,6100
        fn isSingleton(&self) -> B { if self.len() == 1 { B::True } else { B::False } }isSingleton154,6185
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy156,6274
        fn slice(&self, start: N, length: N) -> Self {slice163,6593
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> Self {tabulate171,6841
        fn map<U: StT + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(amap179,7111
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &Self, pred: F) -> Self whefilter205,8205
        fn append(a: &Self, b: &Self) -> Self {append240,9597
        fn append_select(a: &Self, b: &Self) -> Self {append_select246,9842
        fn flatten(sequences: &[ArraySeqMtEphSliceS<T>]) -> Self {flatten258,10307
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> reduce280,11094
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &Self, f: &F, id: T) -> (ArraySeqMtEphSlicescan311,12252
        fn iterate<A: StT + Send, F: Fn(&A, &T) -> A + Send + Sync>(a: &Self, f: &F, seed: A) ->iterate335,13320
        fn inject(a: &Self, updates: &[(N, T)]) -> Self {inject345,13691
        fn ninject(a: &Self, updates: &[(N, T)]) -> Self {ninject356,14103
    impl<T: StT + Send + Sync> Clone for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS368,14536
        fn clone(&self) -> Self {clone369,14602
    impl<T: StT + Send + Sync> PartialEq for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS377,14792
        fn eq(&self, other: &Self) -> bool {eq378,14862
    impl<T: StT + Send + Sync> Eq for ArraySeqMtEphSliceS<T> {}ArraySeqMtEphSliceS391,15280
    impl<T: StT + Send + Sync> Debug for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS393,15345
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt394,15411
    impl<T: StT + Send + Sync> Display for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS402,15678
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt403,15746
    fn repeat_vec<T: StT + Send + Sync>(length: N, init: T) -> Vec<T> {repeat_vec418,16213
    macro_rules! ArraySeqMtEphSliceSLit {ArraySeqMtEphSliceSLit427,16452
    fn _ArraySeqMtEphSliceSLit_type_checks() {_ArraySeqMtEphSliceSLit_type_checks434,16915

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStEph.rs,3858
pub mod ArraySeqStEph {ArraySeqStEph4,143
    pub type ArraySeqStEphS<T> = ArraySeqStEphSChap18<T>;ArraySeqStEphS9,396
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait11,455
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>;new12,498
        fn empty() -> ArraySeqStEphS<T>;empty13,561
        fn singleton(item: T) -> ArraySeqStEphS<T>;singleton14,602
        fn length(&self) -> N;length15,654
        fn nth(&self, index: N) -> &T;nth16,685
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T>;subseq_copy17,724
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStEphS<T>;tabulate20,884
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U>;map22,1034
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T>;select24,1166
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append26,1307
        fn append_select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append_select28,1446
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStEphS<T>;deflate30,1580
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T>;filter32,1738
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, x: A) -> A;iterate33,1827
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T;reduce34,1916
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, scan35,1997
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten36,2097
        fn isEmpty(a: &ArraySeqStEphS<T>) -> bool;isEmpty37,2177
        fn isSingleton(a: &ArraySeqStEphS<T>) -> bool;isSingleton38,2228
        fn update(a: &ArraySeqStEphS<T>, index: N, item: T) -> ArraySeqStEphS<T>;update39,2283
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS42,2372
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T> {new43,2435
        fn empty() -> ArraySeqStEphS<T> {empty48,2655
        fn singleton(item: T) -> ArraySeqStEphS<T> {singleton53,2894
        fn length(&self) -> N { ArraySeqStEphTraitChap18::length(self) }length58,3116
        fn nth(&self, index: N) -> &T { ArraySeqStEphTraitChap18::nth(self, index) }nth60,3190
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T> {subseq_copy62,3276
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStEphS<T> {tabulate67,3527
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U> {map79,4009
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T> {select84,4282
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append98,4722
        fn append_select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append_select115,5511
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStEphS<T> {deflate123,5918
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T> {filter132,6303
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, x: A) -> A {iterate138,6702
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T {reduce147,7011
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, scan163,7746
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten178,8434
        fn isEmpty(a: &ArraySeqStEphS<T>) -> bool {isEmpty183,8676
        fn isSingleton(a: &ArraySeqStEphS<T>) -> bool {isSingleton188,8818
        fn update(a: &ArraySeqStEphS<T>, index: N, item: T) -> ArraySeqStEphS<T> {update193,8968

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/#ArraySeqMtEph.rs#,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtPer.rs,4832
pub mod ArraySeqMtPer {ArraySeqMtPer4,224
    pub trait ArraySeqMtPerTrait<T: StTInMtT> {ArraySeqMtPerTrait10,427
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;new12,506
        fn empty() -> ArraySeqMtPerS<T>;empty13,569
        fn singleton(item: T) -> ArraySeqMtPerS<T>;singleton14,610
        fn length(&self) -> N;length15,662
        fn nth(&self, index: N) -> &T;nth16,693
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;subseq_copy17,732
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T>;tabulate19,806
        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(a: &ArraySmap20,890
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append21,1044
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, pred: Ffilter22,1130
        fn update_single(a: &ArraySeqMtPerS<T>, index: N, item: T) -> ArraySeqMtPerS<T>;update_single23,1267
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject24,1356
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, iterate25,1458
        fn iteratePrefixes<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>,iteratePrefixes26,1566
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: reduce27,1703
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (Arrayscan28,1832
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;flatten29,1946
        fn collect(collect30,2027
        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMinject36,2224
        fn atomicWrite(atomicWrite37,2330
        fn isEmpty(a: &ArraySeqMtPerS<T>) -> bool;isEmpty42,2516
        fn isSingleton(a: &ArraySeqMtPerS<T>) -> bool;isSingleton43,2567
        fn append_select(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;append_select44,2622
        fn select<'a>(a: &'a ArraySeqMtPerS<T>, b: &'a ArraySeqMtPerS<T>, i: N) -> Option<&'a T>select45,2715
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtPerS<T>;deflate46,2813
    impl<T: StTInMtT> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {ArraySeqMtPerS49,2905
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> {new50,2973
        fn empty() -> ArraySeqMtPerS<T> {empty55,3193
        fn singleton(item: T) -> ArraySeqMtPerS<T> {singleton60,3432
        fn length(&self) -> N { ArraySeqMtPerTraitChap18::length(self) }length65,3654
        fn nth(&self, index: N) -> &T { ArraySeqMtPerTraitChap18::nth(self, index) }nth67,3728
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> {subseq_copy69,3814
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T> {tabulate74,3997
        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(a: &ArraySmap84,4443
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append105,5653
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, pred: Ffilter111,6057
        fn update_single(a: &ArraySeqMtPerS<T>, index: N, item: T) -> ArraySeqMtPerS<T> {update_single147,7474
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerninject155,7863
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, iterate160,8136
        fn iteratePrefixes<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>,iteratePrefixes169,8464
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: reduce181,9035
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (Arrayscan203,10182
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {flatten216,10778
        fn collect(collect221,11022
        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMinject228,11269
        fn atomicWrite(atomicWrite232,11475
        fn isEmpty(a: &ArraySeqMtPerS<T>) -> bool {isEmpty240,11774
        fn isSingleton(a: &ArraySeqMtPerS<T>) -> bool {isSingleton245,11916
        fn append_select(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {append_select250,12066
        fn select<'a>(a: &'a ArraySeqMtPerS<T>, b: &'a ArraySeqMtPerS<T>, i: N) -> Option<&'a T>select258,12481
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtPerS<T> {deflate272,12903

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEph.rs,4033
pub mod ArraySeqMtEph {ArraySeqMtEph4,159
    pub type ArraySeqMtEphS<T> = ArraySeqMtEphSChap18<T>;ArraySeqMtEphS11,407
    pub trait ArraySeqMtEphTrait<T: StTInMtT> {ArraySeqMtEphTrait13,466
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>;new14,514
        fn empty() -> ArraySeqMtEphS<T>;empty15,577
        fn singleton(item: T) -> ArraySeqMtEphS<T>;singleton16,618
        fn length(&self) -> N;length17,670
        fn nth_cloned(&self, index: N) -> T;nth_cloned18,701
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;subseq_copy19,746
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;tabulate21,820
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &ArraySmap22,904
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T>;select23,1058
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append24,1146
        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append_select25,1232
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtEphS<T>;deflate26,1325
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, pred: Ffilter27,1410
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, iterate28,1547
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: reduce29,1655
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (Arrayscan30,1784
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;flatten31,1898
        fn isEmpty(a: &ArraySeqMtEphS<T>) -> bool;isEmpty32,1978
        fn isSingleton(a: &ArraySeqMtEphS<T>) -> bool;isSingleton33,2029
        fn update(a: &ArraySeqMtEphS<T>, index: N, item: T) -> ArraySeqMtEphS<T>;update34,2084
    impl<T: StTInMtT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS37,2173
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T> {new38,2241
        fn empty() -> ArraySeqMtEphS<T> {empty43,2461
        fn singleton(item: T) -> ArraySeqMtEphS<T> {singleton48,2700
        fn length(&self) -> N { ArraySeqMtEphTraitChap18::length(self) }length53,2922
        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphTraitChap18::nth_cloned(self, index) nth_cloned55,2996
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {subseq_copy57,3095
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {tabulate62,3325
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &ArraySmap74,3821
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T> {select85,4443
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append99,4881
        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append_select116,5666
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtEphS<T> {deflate124,6073
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, pred: Ffilter133,6384
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, iterate190,8701
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: reduce200,9070
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (Arrayscan221,10148
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {flatten238,10928
        fn isEmpty(a: &ArraySeqMtEphS<T>) -> bool {isEmpty243,11170
        fn isSingleton(a: &ArraySeqMtEphS<T>) -> bool {isSingleton248,11312
        fn update(a: &ArraySeqMtEphS<T>, index: N, item: T) -> ArraySeqMtEphS<T> {update253,11462

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStPer.rs,4328
pub mod ArraySeqStPer {ArraySeqStPer4,138
    pub type ArraySeqStPerS<T> = ArraySeqStPerSChap18<T>;ArraySeqStPerS9,391
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait11,450
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>;new12,493
        fn empty() -> ArraySeqStPerS<T>;empty13,556
        fn singleton(item: T) -> ArraySeqStPerS<T>;singleton14,597
        fn length(&self) -> N;length15,649
        fn nth(&self, index: N) -> &T;nth16,680
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T>;subseq_copy17,719
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStPerS<T>;tabulate20,879
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U>;map22,1029
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>select24,1161
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append26,1312
        fn append_select(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;append_select28,1451
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStPerS<T>;deflate30,1585
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T>;filter32,1743
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, x: A) -> A;iterate33,1832
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T;reduce34,1921
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, scan35,2002
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;flatten36,2102
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject37,2182
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject38,2283
        fn isEmpty(a: &ArraySeqStPerS<T>) -> bool;isEmpty39,2385
        fn isSingleton(a: &ArraySeqStPerS<T>) -> bool;isSingleton40,2436
        fn update(a: &ArraySeqStPerS<T>, index: N, item: T) -> ArraySeqStPerS<T>;update41,2491
    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {ArraySeqStPerS44,2580
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> {new45,2643
        fn empty() -> ArraySeqStPerS<T> { empty50,2863
        fn singleton(item: T) -> ArraySeqStPerS<T> {singleton55,3103
        fn length(&self) -> N { ArraySeqStPerTraitChap18::length(self) }length60,3325
        fn nth(&self, index: N) -> &T { ArraySeqStPerTraitChap18::nth(self, index) }nth62,3399
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T> {subseq_copy64,3485
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStPerS<T> {tabulate69,3741
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U> {map79,4173
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>select84,4446
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {append98,4868
        fn append_select(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {append_select104,5272
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStPerS<T> {deflate112,5687
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {filter121,6064
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, x: A) -> A {iterate127,6463
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T {reduce136,6772
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, scan152,7507
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {flatten165,8089
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerSinject170,8331
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerninject175,8601
        fn isEmpty(a: &ArraySeqStPerS<T>) -> bool {isEmpty180,8874
        fn isSingleton(a: &ArraySeqStPerS<T>) -> bool {isSingleton185,9016
        fn update(a: &ArraySeqStPerS<T>, index: N, item: T) -> ArraySeqStPerS<T> {update190,9166

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap23/PrimTreeSeqSt.rs,987
pub mod PrimTreeSeqSt {PrimTreeSeqSt10,568
    pub enum PrimTreeSeqStTree<T: StT> {PrimTreeSeqStTree15,747
        Zero,Zero16,788
        One(T),One17,802
        Two(PrimTreeSeqStS<T>, PrimTreeSeqStS<T>),Two18,818
    pub struct PrimTreeSeqStS<T: StT> {PrimTreeSeqStS23,997
        data: Vec<T>,data24,1037
    impl<T: StT> PrimTreeSeqStS<T> {PrimTreeSeqStS27,1066
        pub fn empty() -> Self { Self { data: Vec::new() } }empty29,1142
        pub fn singleton(value: T) -> Self { Self { data: vec![value] } }singleton32,1263
        pub fn from_vec(vec: Vec<T>) -> Self { Self { data: vec } }from_vec35,1425
        pub fn into_vec(self) -> Vec<T> { self.data }into_vec38,1554
        pub fn as_slice(&self) -> &[T] { &self.data }as_slice41,1672
        pub fn length(&self) -> N { self.data.len() }length44,1787
        pub fn expose(&self) -> PrimTreeSeqStTree<T> {expose47,1919
        pub fn join(tree: PrimTreeSeqStTree<T>) -> Self {join61,2543

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap23/BBTEph.rs,2142
pub mod BBTEph {BBTEph4,148
    pub enum BBTree<T: StT> {BBTree10,358
        Leaf,Leaf11,388
        Node(Box<BBNode<T>>),Node12,402
    pub struct BBNode<T: StT> {BBNode16,482
        pub(crate) left: BBTree<T>,left17,514
        pub(crate) value: T,value18,550
        pub(crate) right: BBTree<T>,right19,579
    impl<T: StT> BBNode<T> {BBNode22,623
        fn new(left: BBTree<T>, value: T, right: BBTree<T>) -> Self { BBNode { left, value, righnew23,652
    pub trait BBTEphTrait<T: StT> {BBTEphTrait26,761
        fn leaf() -> Self;leaf27,797
        fn node(left: Self, value: T, right: Self) -> Self;node28,824
        fn is_leaf(&self) -> B;is_leaf29,884
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order30,916
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order31,965
        fn height(&self) -> N;height32,1015
        fn size(&self) -> N;size33,1046
    impl<T: StT> BBTree<T> {BBTree36,1082
        pub fn leaf() -> Self { BBTree::Leaf }leaf37,1111
        pub fn node(left: BBTree<T>, value: T, right: BBTree<T>) -> Self {node39,1159
        pub fn is_leaf(&self) -> B {is_leaf43,1313
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order50,1490
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order63,2158
        pub fn height(&self) -> N {height76,2827
        pub fn size(&self) -> N {size87,3162
    impl<T: StT> BBTEphTrait<T> for BBTree<T> {BBTree95,3371
        fn leaf() -> Self { BBTree::leaf() }leaf96,3419
        fn node(left: Self, value: T, right: Self) -> Self { BBTree::node(left, value, right) }node98,3465
        fn is_leaf(&self) -> B { BBTree::is_leaf(self) }is_leaf100,3562
        fn in_order(&self) -> ArraySeqStPerS<T> { BBTree::in_order(self) }in_order102,3620
        fn pre_order(&self) -> ArraySeqStPerS<T> { BBTree::pre_order(self) }pre_order104,3696
        fn height(&self) -> N { BBTree::height(self) }height106,3774
        fn size(&self) -> N { BBTree::size(self) }size108,3830
    macro_rules! BBNodeLit {BBNodeLit112,3908
    fn _BBNodeLit_type_checks() {_BBNodeLit_type_checks119,4134

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortMtSlice.rs,1389
pub mod Chapter36MtSlice {Chapter36MtSlice4,190
    pub trait Chapter36MtSliceTrait<T: StT + Ord + Send> {Chapter36MtSliceTrait14,398
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first15,457
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median316,510
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random17,565
        fn quick_sort_mt_first(&self);quick_sort_mt_first19,620
        fn quick_sort_mt_median3(&self);quick_sort_mt_median320,659
        fn quick_sort_mt_random(&self);quick_sort_mt_random21,700
    impl<T: StT + Ord + Send + Sync> Chapter36MtSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS24,747
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first25,838
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median327,916
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random41,1387
        fn quick_sort_mt_first(&self) {quick_sort_mt_first47,1563
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort52,1723
        fn quick_sort_mt_median3(&self) {quick_sort_mt_median389,3086
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort94,3248
        fn quick_sort_mt_random(&self) {quick_sort_mt_random144,5176
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort149,5337

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortSt.rs,1497
pub mod Chapter36St {Chapter36St4,187
    pub trait Chapter36StTrait<T: StT + Ord> {Chapter36StTrait10,325
        fn pivot_st_first(&self, lo: N, hi: N) -> T;pivot_st_first11,372
        fn pivot_st_median3(&self, lo: N, hi: N) -> T;pivot_st_median312,425
        fn pivot_st_random(&self, lo: N, hi: N) -> T;pivot_st_random13,480
        fn quick_sort_st_first(&mut self);quick_sort_st_first15,535
        fn quick_sort_st_median3(&mut self);quick_sort_st_median316,578
        fn quick_sort_st_random(&mut self);quick_sort_st_random17,623
    impl<T: StT + Ord> Chapter36StTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS20,674
        fn pivot_st_first(&self, lo: N, _hi: N) -> T { self.nth(lo).clone() }pivot_st_first21,741
        fn pivot_st_median3(&self, lo: N, hi: N) -> T {pivot_st_median322,819
        fn pivot_st_random(&self, lo: N, hi: N) -> T {pivot_st_random35,1292
        fn quick_sort_st_first(&mut self) {quick_sort_st_first41,1469
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort42,1513
        fn quick_sort_st_median3(&mut self) {quick_sort_st_median376,2696
            fn median3<T: StT + Ord>(a: &ArraySeqStEphS<T>, lo: N, hi: N) -> T {median377,2742
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort90,3279
        fn quick_sort_st_random(&mut self) {quick_sort_st_random124,4463
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort125,4508

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap36/QuickSortMt.rs,1407
pub mod Chapter36Mt {Chapter36Mt4,186
    pub trait Chapter36MtTrait<T: StT + Ord + Send> {Chapter36MtTrait26,1277
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first27,1331
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median328,1384
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random29,1439
        fn quick_sort_mt_first(&mut self);quick_sort_mt_first31,1494
        fn quick_sort_mt_median3(&mut self);quick_sort_mt_median332,1537
        fn quick_sort_mt_random(&mut self);quick_sort_mt_random33,1582
    impl<T: StT + Ord + Send> Chapter36MtTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS36,1633
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first37,1707
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median338,1784
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random51,2254
        fn quick_sort_mt_first(&mut self) {quick_sort_mt_first57,2430
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort62,2554
        fn quick_sort_mt_median3(&mut self) {quick_sort_mt_median3100,3900
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort105,4026
        fn quick_sort_mt_random(&mut self) {quick_sort_mt_random155,5839
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort160,5964

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetAVLMtEph.rs,5593
pub mod BSTSetAVLMtEph {BSTSetAVLMtEph4,165
    pub struct BSTSetAVLMtEph<T: StTInMtT + Ord> {BSTSetAVLMtEph12,406
        tree: BSTAVLMtEph<T>,tree13,457
    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;BSTSetAVLMt16,494
    pub trait BSTSetAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetAVLMtEphTrait18,544
        fn empty() -> Self;empty19,606
        fn singleton(value: T) -> Self;singleton20,634
        fn size(&self) -> N;size21,674
        fn is_empty(&self) -> B;is_empty22,703
        fn find(&self, value: &T) -> Option<T>;find23,736
        fn contains(&self, value: &T) -> B;contains24,784
        fn minimum(&self) -> Option<T>;minimum25,828
        fn maximum(&self) -> Option<T>;maximum26,868
        fn insert(&mut self, value: T);insert27,908
        fn delete(&mut self, target: &T);delete28,948
        fn union(&self, other: &Self) -> Self;union29,990
        fn intersection(&self, other: &Self) -> Self;intersection30,1037
        fn difference(&self, other: &Self) -> Self;difference31,1091
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1143
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1198
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1253
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1315
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1385
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1453
        fn as_tree(&self) -> &BSTAVLMtEph<T>;as_tree38,1507
    impl<T: StTInMtT + Ord> BSTSetAVLMtEph<T> {BSTSetAVLMtEph41,1560
        pub fn empty() -> Self {empty42,1608
        pub fn singleton(value: T) -> Self {singleton48,1727
        pub fn size(&self) -> N { self.tree.size() }size54,1884
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1938
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2000
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2078
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2156
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2224
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2292
        pub fn delete(&mut self, target: &T) {delete68,2365
        pub fn union(&self, other: &Self) -> Self {union76,2654
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2953
        pub fn difference(&self, other: &Self) -> Self {difference101,3531
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4108
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4799
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5112
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5468
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5877
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6141
        pub fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree180,6224
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6289
        fn rebuild_from_vec(values: Vec<T>) -> BSTAVLMtEph<T> {rebuild_from_vec184,6380
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6599
    impl<T: StTInMtT + Ord> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {BSTSetAVLMtEph204,6882
        fn empty() -> Self { Self::empty() }empty205,6957
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7003
        fn size(&self) -> N { self.tree.size() }size209,7070
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7120
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7178
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7252
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7326
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7390
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7454
        fn delete(&mut self, target: &T) { BSTSetAVLMtEph::delete(self, target) }delete223,7523
        fn union(&self, other: &Self) -> Self { BSTSetAVLMtEph::union(self, other) }union225,7606
        fn intersection(&self, other: &Self) -> Self { BSTSetAVLMtEph::intersection(self, other)intersection227,7692
        fn difference(&self, other: &Self) -> Self { BSTSetAVLMtEph::difference(self, other) }difference229,7792
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetAVLMtEph::split(self, pivot) }split231,7888
        fn join_pair(left: Self, right: Self) -> Self { BSTSetAVLMtEph::join_pair(left, right) }join_pair233,7982
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetAVLMtEph::join_m(left, pivojoin_m235,8080
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetAVLMtEph::filter(sefilter237,8189
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetAVLMtEph::reduce(selfreduce239,8303
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8414
        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree243,8493
    macro_rules! BSTSetAVLMtEphLit {BSTSetAVLMtEphLit247,8580
    fn _BSTSetAVLMtEphLit_type_checks() {_BSTSetAVLMtEphLit_type_checks259,9127

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStPer.rs,4240
pub mod AVLTreeSeqStPer {AVLTreeSeqStPer4,177
    type Link<T> = Option<Rc<Node<T>>>;Link11,339
    struct Node<T: StT> {Node13,380
        value: T,value14,406
        height: N,height15,424
        size: N,size16,443
        left: Link<T>,left17,460
        right: Link<T>,right18,483
    fn height<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.height) }height21,514
    fn size<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.size) }size22,593
    fn mk<T: StT>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {mk24,669
    fn rotate_right<T: StT>(y: Rc<Node<T>>) -> Rc<Node<T>> {rotate_right37,1014
    fn rotate_left<T: StT>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left44,1321
    fn rebalance<T: StT>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance51,1627
    fn nth_ref<'a, T: StT>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref73,2480
    fn set_rec<T: StT>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> {set_rec88,2929
    fn inorder_collect<T: StT>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect106,3701
    fn build_balanced_from_slice<T: StT>(a: &[T]) -> Link<T> {build_balanced_from_slice114,3941
        fn rec<T: StT>(a: &[T]) -> Link<T> {rec115,4004
    pub struct AVLTreeSeqStPerS<T: StT> {AVLTreeSeqStPerS127,4322
        root: Link<T>,root128,4364
    pub trait AVLTreeSeqStPerTrait<T: StT> {AVLTreeSeqStPerTrait131,4394
        fn empty() -> Self;empty133,4480
        fn new() -> Self;new135,4549
        fn length(&self) -> N;length137,4616
        fn nth(&self, index: N) -> &T;nth139,4696
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set141,4845
        fn singleton(item: T) -> Self;singleton145,4996
        fn isEmpty(&self) -> B;isEmpty147,5076
        fn isSingleton(&self) -> B;isSingleton149,5149
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy151,5242
        fn from_vec(values: Vec<T>) -> Self;from_vec153,5365
        fn values_in_order(&self) -> Vec<T>;values_in_order155,5454
    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS158,5506
        fn empty() -> Self { AVLTreeSeqStPerS { root: None } }empty159,5573
        fn new() -> Self { Self::empty() }new160,5636
        fn length(&self) -> N { size(&self.root) }length161,5679
        fn nth(&self, index: N) -> &T { nth_ref(&self.root, index) }nth162,5730
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set163,5799
        fn singleton(item: T) -> Self {singleton168,5988
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty173,6133
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton174,6220
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy175,6311
        fn from_vec(values: Vec<T>) -> Self {from_vec189,6819
        fn values_in_order(&self) -> Vec<T> {values_in_order194,6982
    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS201,7173
        fn eq(&self, other: &Self) -> bool {eq202,7226
    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}AVLTreeSeqStPerS214,7552
    impl<T: StT> std::fmt::Debug for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS216,7600
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt217,7659
    impl<T: StT> AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS223,7851
        pub fn to_arrayseq(&self) -> ArraySeqStPerS<T> {to_arrayseq224,7890
        pub fn iter<'a>(&'a self) -> AVLTreeSeqStPerIter<'a, T> {iter229,8042
    pub struct AVLTreeSeqStPerIter<'a, T: StT> {AVLTreeSeqStPerIter237,8255
        stack: Vec<&'a Node<T>>,stack238,8304
        current: Option<&'a Node<T>>,current239,8337
    impl<'a, T: StT> AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter242,8382
        fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {push_left243,8432
    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter251,8642
        type Item = &'a T;Item252,8705
        fn next(&mut self) -> Option<Self::Item> {next253,8732
macro_rules! AVLTreeSeqStPerLit {AVLTreeSeqStPerLit267,9122

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetPlainMtEph.rs,5280
pub mod BSTSetPlainMtEph {BSTSetPlainMtEph4,167
    pub struct BSTSetPlainMtEph<T: StTInMtT + Ord> {BSTSetPlainMtEph12,416
        tree: BSTPlainMtEph<T>,tree13,469
    pub type BSTSetPlainMt<T> = BSTSetPlainMtEph<T>;BSTSetPlainMt16,508
    pub trait BSTSetPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetPlainMtEphTrait18,562
        fn empty() -> Self;empty19,626
        fn singleton(value: T) -> Self;singleton20,654
        fn size(&self) -> N;size21,694
        fn is_empty(&self) -> B;is_empty22,723
        fn find(&self, value: &T) -> Option<T>;find23,756
        fn contains(&self, value: &T) -> B;contains24,804
        fn minimum(&self) -> Option<T>;minimum25,848
        fn maximum(&self) -> Option<T>;maximum26,888
        fn insert(&mut self, value: T);insert27,928
        fn delete(&mut self, target: &T);delete28,968
        fn union(&self, other: &Self) -> Self;union29,1010
        fn intersection(&self, other: &Self) -> Self;intersection30,1057
        fn difference(&self, other: &Self) -> Self;difference31,1111
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1163
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1218
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1273
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1335
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1405
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1473
        fn as_tree(&self) -> &BSTPlainMtEph<T>;as_tree38,1527
    impl<T: StTInMtT + Ord> BSTSetPlainMtEph<T> {BSTSetPlainMtEph41,1582
        pub fn empty() -> Self {empty42,1632
        pub fn singleton(value: T) -> Self {singleton48,1753
        pub fn size(&self) -> N { self.tree.size() }size54,1912
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1966
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2028
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2106
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2184
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2252
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2320
        pub fn delete(&mut self, target: &T) {delete68,2393
        pub fn union(&self, other: &Self) -> Self {union76,2682
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2981
        pub fn difference(&self, other: &Self) -> Self {difference101,3559
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4136
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4827
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5140
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5496
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5905
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6169
        pub fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree180,6252
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6319
        fn rebuild_from_vec(values: Vec<T>) -> BSTPlainMtEph<T> {rebuild_from_vec184,6410
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6633
    impl<T: StTInMtT + Ord> BSTSetPlainMtEphTrait<T> for BSTSetPlainMtEph<T> {BSTSetPlainMtEph204,6918
        fn empty() -> Self { Self::empty() }empty205,6997
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7043
        fn size(&self) -> N { self.tree.size() }size209,7110
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7160
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7218
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7292
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7366
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7430
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7494
        fn delete(&mut self, target: &T) {delete223,7563
        fn union(&self, other: &Self) -> Self {union231,7848
        fn intersection(&self, other: &Self) -> Self {intersection239,8143
        fn difference(&self, other: &Self) -> Self {difference256,8717
        fn split(&self, pivot: &T) -> (Self, B, Self) {split273,9290
        fn join_pair(left: Self, right: Self) -> Self {join_pair293,9977
        fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m301,10286
        fn filter<F>(&self, predicate: F) -> Selffilter310,10638
        fn reduce<F>(&self, op: F, base: T) -> Treduce317,10799
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order324,10957
        fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree326,11036
    macro_rules! BSTSetPlainMtEphLit {BSTSetPlainMtEphLit330,11125
    fn _BSTSetPlainMtEphLit_type_checks() {_BSTSetPlainMtEphLit_type_checks342,11698

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBStEph.rs,4735
pub mod BSTRBStEph {BSTRBStEph4,185
    enum Color {Color11,414
        Red,Red12,431
        Black,Black13,444
    type Link<T> = Option<Box<Node<T>>>;Link16,466
    struct Node<T: StT + Ord> {Node19,536
        key: T,key20,568
        color: Color,color21,584
        size: N,size22,606
        left: Link<T>,left23,623
        right: Link<T>,right24,646
    impl<T: StT + Ord> Node<T> {Node27,677
        fn new(key: T) -> Self {new28,710
    pub struct BSTRBStEph<T: StT + Ord> {BSTRBStEph40,959
        root: Link<T>,root41,1001
    pub type BSTreeRB<T> = BSTRBStEph<T>;BSTreeRB44,1031
    pub trait BSTRBStEphTrait<T: StT + Ord> {BSTRBStEphTrait46,1074
        fn new() -> Self;new47,1120
        fn size(&self) -> N;size48,1146
        fn is_empty(&self) -> B;is_empty49,1175
        fn height(&self) -> N;height50,1208
        fn insert(&mut self, value: T);insert51,1239
        fn find(&self, target: &T) -> Option<&T>;find52,1279
        fn contains(&self, target: &T) -> B;contains53,1329
        fn minimum(&self) -> Option<&T>;minimum54,1374
        fn maximum(&self) -> Option<&T>;maximum55,1415
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order56,1456
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order57,1505
    impl<T: StT + Ord> Default for BSTRBStEph<T> {BSTRBStEph60,1562
        fn default() -> Self { Self::new() }default61,1613
    impl<T: StT + Ord> BSTRBStEph<T> {BSTRBStEph64,1665
        pub fn new() -> Self { BSTRBStEph { root: None } }new65,1704
        pub fn size(&self) -> N { Self::size_link(&self.root) }size67,1764
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty69,1829
        pub fn height(&self) -> N {height71,1920
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec72,1956
        pub fn insert(&mut self, value: T) {insert81,2252
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find88,2472
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains90,2566
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum92,2680
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum94,2756
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order96,2832
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order102,3056
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red108,3282
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link110,3384
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate112,3467
        fn rotate_left(link: &mut Link<T>) {rotate_left114,3586
        fn rotate_right(link: &mut Link<T>) {rotate_right131,4182
        fn flip_colors(link: &mut Link<T>) {flip_colors148,4780
        fn fix_up(link: &mut Link<T>) {fix_up169,5585
        fn insert_link(link: &mut Link<T>, value: T) {insert_link186,6272
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link202,6814
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link217,7331
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link227,7652
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect237,7975
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect245,8262
    impl<T: StT + Ord> BSTRBStEphTrait<T> for BSTRBStEph<T> {BSTRBStEph254,8558
        fn new() -> Self { BSTRBStEph::new() }new255,8620
        fn size(&self) -> N { BSTRBStEph::size(self) }size257,8668
        fn is_empty(&self) -> B { BSTRBStEph::is_empty(self) }is_empty259,8724
        fn height(&self) -> N { BSTRBStEph::height(self) }height261,8788
        fn insert(&mut self, value: T) { BSTRBStEph::insert(self, value) }insert263,8848
        fn find(&self, target: &T) -> Option<&T> { BSTRBStEph::find(self, target) }find265,8924
        fn contains(&self, target: &T) -> B { BSTRBStEph::contains(self, target) }contains267,9009
        fn minimum(&self) -> Option<&T> { BSTRBStEph::minimum(self) }minimum269,9093
        fn maximum(&self) -> Option<&T> { BSTRBStEph::maximum(self) }maximum271,9164
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTRBStEph::in_order(self) }in_order273,9235
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTRBStEph::pre_order(self) }pre_order275,9315
    macro_rules! BSTRBStEphLit {BSTRBStEphLit279,9423
    fn _BSTRBStEphLit_type_checks() {_BSTRBStEphLit_type_checks291,9917

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeq.rs,5339
pub mod AVLTreeSeq {AVLTreeSeq10,628
    type Link<T> = Option<Box<AVLTreeNode<T>>>;Link16,778
    pub struct AVLTreeNode<T: Copy + Debug> {AVLTreeNode18,827
        pub value: T,value19,873
        pub height: N,height20,895
        pub left_size: N,left_size21,918
        pub right_size: N,right_size22,944
        pub left: Link<T>,left23,971
        pub right: Link<T>,right24,998
        pub index: N,index25,1026
    impl<T: Copy + Debug> AVLTreeNode<T> {AVLTreeNode28,1055
        fn new(value: T, index: N) -> Self {new29,1098
    pub struct AVLTreeS<T: Copy + Debug> {AVLTreeS42,1391
        pub root: Link<T>,root43,1434
        pub next_key: N,next_key44,1461
    pub trait AVLTreeSeq<T: Copy + Debug> {AVLTreeSeq47,1493
        fn empty() -> AVLTreeS<T>;empty50,1616
        fn new() -> AVLTreeS<T>;new54,1739
        fn length(&self) -> N;length58,1854
        fn nth(&self, index: N) -> &T;nth62,2028
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str>;set66,2177
        fn singleton(item: T) -> AVLTreeS<T>;singleton70,2352
        fn isEmpty(&self) -> B;isEmpty74,2465
        fn isSingleton(&self) -> B;isSingleton75,2497
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>subseq_copy79,2664
    impl<T: Copy + Debug> AVLTreeS<T> {AVLTreeS84,2778
        pub fn new_root() -> Self {new_root85,2818
        pub fn new() -> Self { Self::new_root() }new91,2958
        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeS<T> {update93,3009
        pub fn from_vec(values: Vec<T>) -> AVLTreeS<T>from_vec98,3191
        pub fn to_arrayseq(&self) -> ArraySeqS<T>to_arrayseq111,3603
        pub fn iter<'a>(&'a self) -> AVLTreeSeqIter<'a, T> { AVLTreeSeqIter::new(&self.root) }iter130,4214
        pub fn push_back(&mut self, value: T) {push_back132,4310
        pub fn contains_value(&self, target: &T) -> Bcontains_value139,4618
        pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value151,4890
        pub fn delete_value(&mut self, target: &T) -> booldelete_value153,4967
        pub fn is_tree_empty(&self) -> bool { self.length() == 0 }is_tree_empty181,5911
        pub fn values_in_order(&self) -> Vec<T>values_in_order183,5979
    impl<T: Copy + Debug> AVLTreeSeq<T> for AVLTreeS<T> {AVLTreeS193,6215
        fn empty() -> AVLTreeS<T> { AVLTreeS::new_root() }empty195,6315
        fn new() -> AVLTreeS<T> { AVLTreeS::new_root() }new198,6417
        fn length(&self) -> N { size_link(&self.root) }length201,6517
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth204,6624
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str> {set207,6745
        fn singleton(item: T) -> AVLTreeS<T> {singleton213,6960
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty220,7198
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton222,7327
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>subseq_copy225,7481
    impl<T: Eq + Copy + Debug> PartialEq for AVLTreeS<T> {AVLTreeS243,8032
        fn eq(&self, other: &Self) -> bool {eq244,8091
    impl<T: Eq + Copy + Debug> Eq for AVLTreeS<T> {}AVLTreeS257,8418
    impl<T: Debug + Copy> std::fmt::Debug for AVLTreeS<T> {AVLTreeS259,8472
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt260,8532
    impl<T: std::fmt::Display + Copy + Debug> std::fmt::Display for AVLTreeS<T> {AVLTreeS266,8740
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt267,8822
    pub struct AVLTreeSeqIter<'a, T: Copy + Debug> {AVLTreeSeqIter283,9234
        stack: Vec<&'a AVLTreeNode<T>>,stack284,9287
        current: Option<&'a AVLTreeNode<T>>,current285,9327
    impl<'a, T: Copy + Debug> AVLTreeSeqIter<'a, T> {AVLTreeSeqIter288,9379
        fn new(root: &'a Link<T>) -> Self {new289,9433
        fn push_left(&mut self, link: &'a Link<T>) {push_left298,9658
    impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIter<'a, T> {AVLTreeSeqIter307,9910
        type Item = &'a T;Item308,9977
        fn next(&mut self) -> Option<Self::Item> {next309,10004
    fn h<T: Copy + Debug>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h319,10263
    fn size_link<T: Copy + Debug>(n: &Link<T>) -> N {size_link320,10346
    fn update_meta<T: Copy + Debug>(n: &mut Box<AVLTreeNode<T>>) {update_meta328,10520
    fn rotate_right<T: Copy + Debug>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right336,10774
    fn rotate_left<T: Copy + Debug>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left349,11148
    fn rebalance<T: Copy + Debug>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance362,11520
    pub(crate) fn insert_at_link<T: Copy + Debug>(node: Link<T>, index: N, value: T, next_key: &insert_at_link386,12420
    fn nth_link<'a, T: Copy + Debug>(node: &'a Link<T>, index: N) -> &'a T {nth_link406,13225
    fn set_link<T: Copy + Debug>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static set_link418,13614
    fn push_inorder<T: Copy + Debug>(link: &Link<T>, out: &mut Vec<T>)push_inorder435,14213

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetSplayMtEph.rs,5648
pub mod BSTSetSplayMtEph {BSTSetSplayMtEph4,167
    pub struct BSTSetSplayMtEph<T: StTInMtT + Ord> {BSTSetSplayMtEph12,416
        tree: BSTSplayMtEph<T>,tree13,469
    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;BSTSetSplayMt16,508
    pub trait BSTSetSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetSplayMtEphTrait18,562
        fn empty() -> Self;empty19,626
        fn singleton(value: T) -> Self;singleton20,654
        fn size(&self) -> N;size21,694
        fn is_empty(&self) -> B;is_empty22,723
        fn find(&self, value: &T) -> Option<T>;find23,756
        fn contains(&self, value: &T) -> B;contains24,804
        fn minimum(&self) -> Option<T>;minimum25,848
        fn maximum(&self) -> Option<T>;maximum26,888
        fn insert(&mut self, value: T);insert27,928
        fn delete(&mut self, target: &T);delete28,968
        fn union(&self, other: &Self) -> Self;union29,1010
        fn intersection(&self, other: &Self) -> Self;intersection30,1057
        fn difference(&self, other: &Self) -> Self;difference31,1111
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1163
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1218
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1273
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1335
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1405
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1473
        fn as_tree(&self) -> &BSTSplayMtEph<T>;as_tree38,1527
    impl<T: StTInMtT + Ord> BSTSetSplayMtEph<T> {BSTSetSplayMtEph41,1582
        pub fn empty() -> Self {empty42,1632
        pub fn singleton(value: T) -> Self {singleton48,1753
        pub fn size(&self) -> N { self.tree.size() }size54,1912
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1966
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2028
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2106
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2184
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2252
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2320
        pub fn delete(&mut self, target: &T) {delete68,2393
        pub fn union(&self, other: &Self) -> Self {union76,2682
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2981
        pub fn difference(&self, other: &Self) -> Self {difference101,3559
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4136
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4827
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5140
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5496
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5905
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6169
        pub fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree180,6252
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6319
        fn rebuild_from_vec(values: Vec<T>) -> BSTSplayMtEph<T> {rebuild_from_vec184,6410
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6633
    impl<T: StTInMtT + Ord> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {BSTSetSplayMtEph204,6918
        fn empty() -> Self { Self::empty() }empty205,6997
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7043
        fn size(&self) -> N { self.tree.size() }size209,7110
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7160
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7218
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7292
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7366
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7430
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7494
        fn delete(&mut self, target: &T) { BSTSetSplayMtEph::delete(self, target) }delete223,7563
        fn union(&self, other: &Self) -> Self { BSTSetSplayMtEph::union(self, other) }union225,7648
        fn intersection(&self, other: &Self) -> Self { BSTSetSplayMtEph::intersection(self, otheintersection227,7736
        fn difference(&self, other: &Self) -> Self { BSTSetSplayMtEph::difference(self, other) }difference229,7838
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetSplayMtEph::split(self, pivot) }split231,7936
        fn join_pair(left: Self, right: Self) -> Self { BSTSetSplayMtEph::join_pair(left, right)join_pair233,8032
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetSplayMtEph::join_m(left, pijoin_m235,8132
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetSplayMtEph::filter(filter237,8243
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetSplayMtEph::reduce(sereduce239,8359
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8472
        fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree243,8551
    macro_rules! BSTSetSplayMtEphLit {BSTSetSplayMtEphLit247,8640
    fn _BSTSetSplayMtEphLit_type_checks() {_BSTSetSplayMtEphLit_type_checks259,9213

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaMtEph.rs,4633
pub mod BSTBBAlphaMtEph {BSTBBAlphaMtEph4,200
    type Link<T> = Option<Box<Node<T>>>;Link13,436
    struct Node<T: StTInMtT + Ord> {Node17,520
        key: T,key18,557
        size: N,size19,573
        left: Link<T>,left20,590
        right: Link<T>,right21,613
    impl<T: StTInMtT + Ord> Node<T> {Node24,644
        fn new(key: T) -> Self {new25,682
    pub struct BSTBBAlphaMtEph<T: StTInMtT + Ord> {BSTBBAlphaMtEph36,896
        root: Arc<RwLock<Link<T>>>,root37,948
    pub type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;BSTreeBBAlpha40,991
    pub trait BSTBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTBBAlphaMtEphTrait42,1044
        fn new() -> Self;new43,1107
        fn insert(&self, value: T);insert44,1133
        fn find(&self, target: &T) -> Option<T>;find45,1169
        fn contains(&self, target: &T) -> B;contains46,1218
        fn size(&self) -> N;size47,1263
        fn is_empty(&self) -> B;is_empty48,1292
        fn height(&self) -> N;height49,1325
        fn minimum(&self) -> Option<T>;minimum50,1356
        fn maximum(&self) -> Option<T>;maximum51,1396
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order52,1436
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order53,1485
    impl<T: StTInMtT + Ord> Default for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph56,1542
        fn default() -> Self { Self::new() }default57,1603
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph60,1655
        pub fn new() -> Self {new61,1704
        pub fn size(&self) -> N {size67,1841
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty72,1974
        pub fn height(&self) -> N {height74,2065
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec75,2101
        pub fn insert(&self, value: T) {insert86,2451
        pub fn find(&self, target: &T) -> Option<T> {find95,2782
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains100,2952
        pub fn minimum(&self) -> Option<T> {minimum102,3066
        pub fn maximum(&self) -> Option<T> {maximum107,3218
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order112,3370
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order119,3655
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link126,3942
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate128,4025
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link130,4144
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild152,4890
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed159,5172
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values169,5582
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced177,5863
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link189,6303
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link204,6820
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link214,7141
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect224,7464
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect232,7751
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph241,8047
        fn new() -> Self { BSTBBAlphaMtEph::new() }new242,8124
        fn insert(&self, value: T) { BSTBBAlphaMtEph::insert(self, value) }insert244,8177
        fn find(&self, target: &T) -> Option<T> { BSTBBAlphaMtEph::find(self, target) }find246,8254
        fn contains(&self, target: &T) -> B { BSTBBAlphaMtEph::contains(self, target) }contains248,8343
        fn size(&self) -> N { BSTBBAlphaMtEph::size(self) }size250,8432
        fn is_empty(&self) -> B { BSTBBAlphaMtEph::is_empty(self) }is_empty252,8493
        fn height(&self) -> N { BSTBBAlphaMtEph::height(self) }height254,8562
        fn minimum(&self) -> Option<T> { BSTBBAlphaMtEph::minimum(self) }minimum256,8627
        fn maximum(&self) -> Option<T> { BSTBBAlphaMtEph::maximum(self) }maximum258,8702
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaMtEph::in_order(self) }in_order260,8777
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaMtEph::pre_order(self) }pre_order262,8862
    macro_rules! BSTBBAlphaMtEphLit {BSTBBAlphaMtEphLit266,8975
    fn _BSTBBAlphaMtEphLit_type_checks() {_BSTBBAlphaMtEphLit_type_checks278,9530

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLStEph.rs,4554
pub mod BSTAVLStEph {BSTAVLStEph4,189
    type Link<T> = Option<Box<Node<T>>>;Link9,356
    struct Node<T: StT + Ord> {Node13,440
        key: T,key14,472
        height: i32,height15,488
        size: N,size16,509
        left: Link<T>,left17,526
        right: Link<T>,right18,549
    impl<T: StT + Ord> Node<T> {Node21,580
        fn new(key: T) -> Self {new22,613
    pub struct BSTAVLStEph<T: StT + Ord> {BSTAVLStEph34,854
        root: Link<T>,root35,897
    pub type BSTreeAVL<T> = BSTAVLStEph<T>;BSTreeAVL38,927
    pub trait BSTAVLStEphTrait<T: StT + Ord> {BSTAVLStEphTrait40,972
        fn new() -> Self;new41,1019
        fn size(&self) -> N;size42,1045
        fn is_empty(&self) -> B;is_empty43,1074
        fn height(&self) -> N;height44,1107
        fn insert(&mut self, value: T);insert45,1138
        fn find(&self, target: &T) -> Option<&T>;find46,1178
        fn contains(&self, target: &T) -> B;contains47,1228
        fn minimum(&self) -> Option<&T>;minimum48,1273
        fn maximum(&self) -> Option<&T>;maximum49,1314
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order50,1355
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order51,1404
    impl<T: StT + Ord> Default for BSTAVLStEph<T> {BSTAVLStEph54,1461
        fn default() -> Self { Self::new() }default55,1513
    impl<T: StT + Ord> BSTAVLStEph<T> {BSTAVLStEph58,1565
        pub fn new() -> Self { BSTAVLStEph { root: None } }new59,1605
        pub fn size(&self) -> N { Self::size_link(&self.root) }size61,1666
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty63,1731
        pub fn height(&self) -> N { Self::height_link(&self.root) as N }height65,1822
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert67,1896
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find69,1986
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains71,2080
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum73,2194
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum75,2270
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order77,2346
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order83,2570
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link89,2796
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link91,2885
        fn update(node: &mut Node<T>) {update93,2968
        fn rotate_right(link: &mut Link<T>) {rotate_right98,3204
        fn rotate_left(link: &mut Link<T>) {rotate_left112,3660
        fn rebalance(link: &mut Link<T>) {rebalance126,4115
        fn insert_link(link: &mut Link<T>, value: T) {insert_link151,5161
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link170,5796
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link185,6313
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link195,6634
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect205,6957
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect213,7244
    impl<T: StT + Ord> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {BSTAVLStEph222,7540
        fn new() -> Self { BSTAVLStEph::new() }new223,7604
        fn size(&self) -> N { BSTAVLStEph::size(self) }size225,7653
        fn is_empty(&self) -> B { BSTAVLStEph::is_empty(self) }is_empty227,7710
        fn height(&self) -> N { BSTAVLStEph::height(self) }height229,7775
        fn insert(&mut self, value: T) { BSTAVLStEph::insert(self, value) }insert231,7836
        fn find(&self, target: &T) -> Option<&T> { BSTAVLStEph::find(self, target) }find233,7913
        fn contains(&self, target: &T) -> B { BSTAVLStEph::contains(self, target) }contains235,7999
        fn minimum(&self) -> Option<&T> { BSTAVLStEph::minimum(self) }minimum237,8084
        fn maximum(&self) -> Option<&T> { BSTAVLStEph::maximum(self) }maximum239,8156
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::in_order(self) }in_order241,8228
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::pre_order(self) }pre_order243,8309
    macro_rules! BSTAVLStEphLit {BSTAVLStEphLit247,8418
    fn _BSTAVLStEphLit_type_checks() {_BSTAVLStEphLit_type_checks259,8925

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainMtEph.rs,3812
pub mod BSTPlainMtEph {BSTPlainMtEph4,182
    type Link<T> = Arc<RwLock<Option<Node<T>>>>;Link10,330
    struct Node<T: StTInMtT + Ord> {Node14,422
        key: T,key15,459
        height: i32,height16,475
        size: N,size17,496
        left: Link<T>,left18,513
        right: Link<T>,right19,536
    impl<T: StTInMtT + Ord> Node<T> {Node22,567
        fn new(key: T) -> Self {new23,605
        fn update(&mut self) {update33,858
    pub struct BSTPlainMtEph<T: StTInMtT + Ord> {BSTPlainMtEph43,1183
        root: Link<T>,root44,1233
    pub type BSTree<T> = BSTPlainMtEph<T>;BSTree47,1263
    pub trait BSTPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTPlainMtEphTrait49,1307
        fn new() -> Self;new50,1368
        fn insert(&self, value: T);insert51,1394
        fn find(&self, target: &T) -> Option<T>;find52,1430
        fn contains(&self, target: &T) -> B;contains53,1479
        fn size(&self) -> N;size54,1524
        fn is_empty(&self) -> B;is_empty55,1553
        fn height(&self) -> N;height56,1586
        fn minimum(&self) -> Option<T>;minimum57,1617
        fn maximum(&self) -> Option<T>;maximum58,1657
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order59,1697
    impl<T: StTInMtT + Ord> BSTPlainMtEph<T> {BSTPlainMtEph62,1753
        pub fn new() -> Self {new63,1800
        pub fn insert(&self, value: T) {insert69,1926
            fn descend<T: StTInMtT + Ord>(link: &Link<T>, value: T) -> bool {descend70,1967
        pub fn find(&self, target: &T) -> Option<T> {find104,3159
            fn find_rec<T: StTInMtT + Ord>(link: &Link<T>, target: &T) -> Option<T> {find_rec105,3213
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains124,3952
        pub fn size(&self) -> N {size125,4065
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty130,4216
        pub fn height(&self) -> N {height132,4307
        pub fn minimum(&self) -> Option<T> {minimum137,4467
            fn leftmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {leftmost138,4512
        pub fn maximum(&self) -> Option<T> {maximum159,5222
            fn rightmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {rightmost160,5267
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order181,5984
            fn traverse<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {traverse182,6038
    fn height_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> i32 { link.as_ref().map_or(0, |n|height_of201,6706
    fn size_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> N { link.as_ref().map_or(0, |n| n.ssize_of203,6816
    impl<T: StTInMtT + Ord> BSTPlainMtEphTrait<T> for BSTPlainMtEph<T> {BSTPlainMtEph205,6920
        fn new() -> Self { BSTPlainMtEph::new() }new206,6993
        fn insert(&self, value: T) { BSTPlainMtEph::insert(self, value) }insert207,7043
        fn find(&self, target: &T) -> Option<T> { BSTPlainMtEph::find(self, target) }find208,7117
        fn contains(&self, target: &T) -> B { BSTPlainMtEph::contains(self, target) }contains209,7203
        fn size(&self) -> N { BSTPlainMtEph::size(self) }size210,7289
        fn is_empty(&self) -> B { BSTPlainMtEph::is_empty(self) }is_empty211,7347
        fn height(&self) -> N { BSTPlainMtEph::height(self) }height212,7413
        fn minimum(&self) -> Option<T> { BSTPlainMtEph::minimum(self) }minimum213,7475
        fn maximum(&self) -> Option<T> { BSTPlainMtEph::maximum(self) }maximum214,7547
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTPlainMtEph::in_order(self) }in_order215,7619
    macro_rules! BSTPlainMtEphLit {BSTPlainMtEphLit219,7728
    fn _BSTPlainMtEphLit_type_checks() {_BSTPlainMtEphLit_type_checks234,8292

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaStEph.rs,4746
pub mod BSTBBAlphaStEph {BSTBBAlphaStEph4,172
    type Link<T> = Option<Box<Node<T>>>;Link11,373
    struct Node<T: StT + Ord> {Node14,443
        key: T,key15,475
        size: N,size16,491
        left: Link<T>,left17,508
        right: Link<T>,right18,531
    impl<T: StT + Ord> Node<T> {Node21,562
        fn new(key: T) -> Self {new22,595
    pub struct BSTBBAlphaStEph<T: StT + Ord> {BSTBBAlphaStEph33,809
        root: Link<T>,root34,856
    pub type BSTreeBBAlpha<T> = BSTBBAlphaStEph<T>;BSTreeBBAlpha37,886
    pub trait BSTBBAlphaStEphTrait<T: StT + Ord> {BSTBBAlphaStEphTrait39,939
        fn new() -> Self;new40,990
        fn size(&self) -> N;size41,1016
        fn is_empty(&self) -> B;is_empty42,1045
        fn height(&self) -> N;height43,1078
        fn insert(&mut self, value: T);insert44,1109
        fn find(&self, target: &T) -> Option<&T>;find45,1149
        fn contains(&self, target: &T) -> B;contains46,1199
        fn minimum(&self) -> Option<&T>;minimum47,1244
        fn maximum(&self) -> Option<&T>;maximum48,1285
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order49,1326
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order50,1375
    impl<T: StT + Ord> Default for BSTBBAlphaStEph<T> {BSTBBAlphaStEph53,1432
        fn default() -> Self { Self::new() }default54,1488
    impl<T: StT + Ord> BSTBBAlphaStEph<T> {BSTBBAlphaStEph57,1540
        pub fn new() -> Self { BSTBBAlphaStEph { root: None } }new58,1584
        pub fn size(&self) -> N { Self::size_link(&self.root) }size60,1649
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty62,1714
        pub fn height(&self) -> N {height64,1805
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec65,1841
        pub fn insert(&mut self, value: T) {insert74,2137
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find82,2425
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains84,2519
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum86,2633
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum88,2709
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order90,2785
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order96,3009
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link102,3235
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate104,3318
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link106,3437
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild128,4183
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed135,4465
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values145,4875
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced153,5156
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link165,5596
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link180,6113
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link190,6434
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect200,6757
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect208,7044
    impl<T: StT + Ord> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {BSTBBAlphaStEph217,7340
        fn new() -> Self { BSTBBAlphaStEph::new() }new218,7412
        fn size(&self) -> N { BSTBBAlphaStEph::size(self) }size220,7465
        fn is_empty(&self) -> B { BSTBBAlphaStEph::is_empty(self) }is_empty222,7526
        fn height(&self) -> N { BSTBBAlphaStEph::height(self) }height224,7595
        fn insert(&mut self, value: T) { BSTBBAlphaStEph::insert(self, value) }insert226,7660
        fn find(&self, target: &T) -> Option<&T> { BSTBBAlphaStEph::find(self, target) }find228,7741
        fn contains(&self, target: &T) -> B { BSTBBAlphaStEph::contains(self, target) }contains230,7831
        fn minimum(&self) -> Option<&T> { BSTBBAlphaStEph::minimum(self) }minimum232,7920
        fn maximum(&self) -> Option<&T> { BSTBBAlphaStEph::maximum(self) }maximum234,7996
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaStEph::in_order(self) }in_order236,8072
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTBBAlphaStEph::pre_order(self) }pre_order238,8157
    macro_rules! BSTBBAlphaStEphLit {BSTBBAlphaStEphLit242,8270
    fn _BSTBBAlphaStEphLit_type_checks() {_BSTBBAlphaStEphLit_type_checks254,8829

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayMtEph.rs,4234
pub mod BSTSplayMtEph {BSTSplayMtEph4,191
    type Link<T> = Option<Box<Node<T>>>;Link11,395
    struct Node<T: StTInMtT + Ord> {Node14,465
        key: T,key15,502
        size: N,size16,518
        left: Link<T>,left17,535
        right: Link<T>,right18,558
    impl<T: StTInMtT + Ord> Node<T> {Node21,589
        fn new(key: T) -> Self {new22,627
    pub struct BSTSplayMtEph<T: StTInMtT + Ord> {BSTSplayMtEph33,841
        root: Arc<RwLock<Link<T>>>,root34,891
    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;BSTreeSplay37,934
    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSplayMtEphTrait39,983
        fn new() -> Self;new40,1044
        fn insert(&self, value: T);insert41,1070
        fn find(&self, target: &T) -> Option<T>;find42,1106
        fn contains(&self, target: &T) -> B;contains43,1155
        fn size(&self) -> N;size44,1200
        fn is_empty(&self) -> B;is_empty45,1229
        fn height(&self) -> N;height46,1262
        fn minimum(&self) -> Option<T>;minimum47,1293
        fn maximum(&self) -> Option<T>;maximum48,1333
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order49,1373
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order50,1422
    impl<T: StTInMtT + Ord> Default for BSTSplayMtEph<T> {BSTSplayMtEph53,1479
        fn default() -> Self { Self::new() }default54,1538
    impl<T: StTInMtT + Ord> BSTSplayMtEph<T> {BSTSplayMtEph57,1590
        pub fn new() -> Self {new58,1637
        pub fn size(&self) -> N {size64,1772
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty69,1905
        pub fn height(&self) -> N {height71,1996
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec72,2032
        pub fn insert(&self, value: T) {insert83,2382
        pub fn find(&self, target: &T) -> Option<T> {find88,2541
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains93,2711
        pub fn minimum(&self) -> Option<T> {minimum95,2825
        pub fn maximum(&self) -> Option<T> {maximum100,2977
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order105,3129
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order112,3414
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link119,3701
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate121,3784
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link123,3903
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link145,4649
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link160,5166
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link170,5487
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect180,5810
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect188,6097
    impl<T: StTInMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {BSTSplayMtEph197,6393
        fn new() -> Self { BSTSplayMtEph::new() }new198,6466
        fn insert(&self, value: T) { BSTSplayMtEph::insert(self, value) }insert200,6517
        fn find(&self, target: &T) -> Option<T> { BSTSplayMtEph::find(self, target) }find202,6592
        fn contains(&self, target: &T) -> B { BSTSplayMtEph::contains(self, target) }contains204,6679
        fn size(&self) -> N { BSTSplayMtEph::size(self) }size206,6766
        fn is_empty(&self) -> B { BSTSplayMtEph::is_empty(self) }is_empty208,6825
        fn height(&self) -> N { BSTSplayMtEph::height(self) }height210,6892
        fn minimum(&self) -> Option<T> { BSTSplayMtEph::minimum(self) }minimum212,6955
        fn maximum(&self) -> Option<T> { BSTSplayMtEph::maximum(self) }maximum214,7028
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTSplayMtEph::in_order(self) }in_order216,7101
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTSplayMtEph::pre_order(self) }pre_order218,7184
    macro_rules! BSTSplayMtEphLit {BSTSplayMtEphLit222,7295
    fn _BSTSplayMtEphLit_type_checks() {_BSTSplayMtEphLit_type_checks234,7824

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetRBMtEph.rs,5565
pub mod BSTSetRBMtEph {BSTSetRBMtEph4,171
    pub struct BSTSetRBMtEph<T: StTInMtT + Ord> {BSTSetRBMtEph12,408
        tree: BSTRBMtEph<T>,tree13,458
    pub type BSTSetRBMt<T> = BSTSetRBMtEph<T>;BSTSetRBMt16,494
    pub trait BSTSetRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetRBMtEphTrait18,542
        fn empty() -> Self;empty19,603
        fn singleton(value: T) -> Self;singleton20,631
        fn size(&self) -> N;size21,671
        fn is_empty(&self) -> B;is_empty22,700
        fn find(&self, value: &T) -> Option<T>;find23,733
        fn contains(&self, value: &T) -> B;contains24,781
        fn minimum(&self) -> Option<T>;minimum25,825
        fn maximum(&self) -> Option<T>;maximum26,865
        fn insert(&mut self, value: T);insert27,905
        fn delete(&mut self, target: &T);delete28,945
        fn union(&self, other: &Self) -> Self;union29,987
        fn intersection(&self, other: &Self) -> Self;intersection30,1034
        fn difference(&self, other: &Self) -> Self;difference31,1088
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1140
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1195
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1250
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1312
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1382
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1450
        fn as_tree(&self) -> &BSTRBMtEph<T>;as_tree38,1504
    impl<T: StTInMtT + Ord> BSTSetRBMtEph<T> {BSTSetRBMtEph41,1556
        pub fn empty() -> Self {empty42,1603
        pub fn singleton(value: T) -> Self {singleton48,1721
        pub fn size(&self) -> N { self.tree.size() }size54,1877
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1931
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,1993
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2071
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2149
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2217
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2285
        pub fn delete(&mut self, target: &T) {delete68,2358
        pub fn union(&self, other: &Self) -> Self {union76,2647
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2946
        pub fn difference(&self, other: &Self) -> Self {difference101,3524
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4101
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4792
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5105
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5461
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5870
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6134
        pub fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree180,6217
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6281
        fn rebuild_from_vec(values: Vec<T>) -> BSTRBMtEph<T> {rebuild_from_vec184,6372
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6589
    impl<T: StTInMtT + Ord> BSTSetRBMtEphTrait<T> for BSTSetRBMtEph<T> {BSTSetRBMtEph204,6871
        fn empty() -> Self { Self::empty() }empty205,6944
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,6990
        fn size(&self) -> N { self.tree.size() }size209,7057
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7107
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7165
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7239
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7313
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7377
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7441
        fn delete(&mut self, target: &T) { BSTSetRBMtEph::delete(self, target) }delete223,7510
        fn union(&self, other: &Self) -> Self { BSTSetRBMtEph::union(self, other) }union225,7592
        fn intersection(&self, other: &Self) -> Self { BSTSetRBMtEph::intersection(self, other) intersection227,7677
        fn difference(&self, other: &Self) -> Self { BSTSetRBMtEph::difference(self, other) }difference229,7776
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetRBMtEph::split(self, pivot) }split231,7871
        fn join_pair(left: Self, right: Self) -> Self { BSTSetRBMtEph::join_pair(left, right) }join_pair233,7964
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetRBMtEph::join_m(left, pivotjoin_m235,8061
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetRBMtEph::filter(selfilter237,8169
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetRBMtEph::reduce(self,reduce239,8282
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8392
        fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree243,8471
    macro_rules! BSTSetRBMtEphLit {BSTSetRBMtEphLit247,8557
    fn _BSTSetRBMtEphLit_type_checks() {_BSTSetRBMtEphLit_type_checks259,9091

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStEph.rs,4730
pub mod AVLTreeSeqStEph {AVLTreeSeqStEph4,153
    type Link<T> = Option<Box<AVLTreeNode<T>>>;Link10,294
    pub struct AVLTreeNode<T: StT> {AVLTreeNode12,343
        pub value: T,value13,380
        pub height: N,height14,402
        pub left_size: N,left_size15,425
        pub right_size: N,right_size16,451
        pub left: Link<T>,left17,478
        pub right: Link<T>,right18,505
        pub index: N,index19,533
    impl<T: StT> AVLTreeNode<T> {AVLTreeNode22,562
        fn new(value: T, index: N) -> Self {new23,596
    pub struct AVLTreeSeqStEphS<T: StT> {AVLTreeSeqStEphS36,889
        pub root: Link<T>,root37,931
        pub next_key: N,next_key38,958
    pub trait AVLTreeSeqStEphTrait<T: StT> {AVLTreeSeqStEphTrait41,990
        fn empty() -> Self;empty43,1077
        fn new() -> Self;new45,1147
        fn length(&self) -> N;length47,1215
        fn nth(&self, index: N) -> &T;nth49,1296
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set51,1385
        fn singleton(item: T) -> Self;singleton53,1508
        fn isEmpty(&self) -> B;isEmpty55,1589
        fn isSingleton(&self) -> B;isSingleton57,1663
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy59,1761
    impl<T: StT> AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS62,1828
        pub fn new_root() -> Self {new_root63,1867
        pub fn new() -> Self { Self::new_root() }new69,2015
        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqStEphS<T> {update70,2065
        pub fn from_vec(values: Vec<T>) -> AVLTreeSeqStEphS<T> {from_vec74,2221
        pub fn to_arrayseq(&self) -> ArraySeqStEphS<T> {to_arrayseq83,2604
        pub fn iter<'a>(&'a self) -> AVLTreeSeqIterStEph<'a, T> { AVLTreeSeqIterStEph::new(&selfiter99,3192
        pub fn push_back(&mut self, value: T) {push_back100,3297
        pub fn contains_value(&self, target: &T) -> B {contains_value105,3511
        pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value113,3734
        pub fn delete_value(&mut self, target: &T) -> bool {delete_value114,3810
    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS139,4628
        fn empty() -> Self { AVLTreeSeqStEphS::new_root() }empty140,4695
        fn new() -> Self { AVLTreeSeqStEphS::new_root() }new142,4756
        fn length(&self) -> N { size_link(&self.root) }length144,4815
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth146,4872
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set148,4943
        fn singleton(item: T) -> Self {singleton153,5109
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }isEmpty159,5306
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }isSingleton161,5394
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy163,5486
    pub struct AVLTreeSeqIterStEph<'a, T: StT> {AVLTreeSeqIterStEph178,5975
        stack: Vec<&'a AVLTreeNode<T>>,stack179,6024
        current: Option<&'a AVLTreeNode<T>>,current180,6064
    impl<'a, T: StT> AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph183,6116
        fn new(root: &'a Link<T>) -> Self {new184,6166
        fn push_left(&mut self, link: &'a Link<T>) {push_left192,6395
    impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph201,6647
        type Item = &'a T;Item202,6710
        fn next(&mut self) -> Option<Self::Item> {next203,6737
    fn h<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h211,6961
    fn size_link<T: StT>(n: &Link<T>) -> N {size_link213,7036
    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>) {update_meta221,7201
    fn rotate_right<T: StT>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right229,7446
    fn rotate_left<T: StT>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left240,7809
    fn rebalance<T: StT>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance251,8170
    pub(crate) fn insert_at_link<T: StT>(node: Link<T>, index: N, value: T, next_key: &mut N) ->insert_at_link272,8935
    fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> &'a T {nth_link292,9731
    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str> {set_link304,10111
    macro_rules! AVLTreeSeqStEphLit {AVLTreeSeqStEphLit322,10721
    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS336,11309
        fn eq(&self, other: &Self) -> bool {eq337,11362
    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}AVLTreeSeqStEphS350,11689

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBMtEph.rs,4637
pub mod BSTRBMtEph {BSTRBMtEph4,194
    enum Color {Color13,458
        Red,Red14,475
        Black,Black15,488
    type Link<T> = Option<Box<Node<T>>>;Link18,510
    struct Node<T: StTInMtT + Ord> {Node22,594
        key: T,key23,631
        color: Color,color24,647
        size: N,size25,669
        left: Link<T>,left26,686
        right: Link<T>,right27,709
    impl<T: StTInMtT + Ord> Node<T> {Node30,740
        fn new(key: T) -> Self {new31,778
    pub struct BSTRBMtEph<T: StTInMtT + Ord> {BSTRBMtEph43,1027
        root: Arc<RwLock<Link<T>>>,root44,1074
    pub type BSTreeRB<T> = BSTRBMtEph<T>;BSTreeRB47,1117
    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTRBMtEphTrait49,1160
        fn new() -> Self;new50,1218
        fn insert(&self, value: T);insert51,1244
        fn find(&self, target: &T) -> Option<T>;find52,1280
        fn contains(&self, target: &T) -> B;contains53,1329
        fn size(&self) -> N;size54,1374
        fn is_empty(&self) -> B;is_empty55,1403
        fn height(&self) -> N;height56,1436
        fn minimum(&self) -> Option<T>;minimum57,1467
        fn maximum(&self) -> Option<T>;maximum58,1507
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order59,1547
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order60,1596
    impl<T: StTInMtT + Ord> Default for BSTRBMtEph<T> {BSTRBMtEph63,1653
        fn default() -> Self { Self::new() }default64,1709
    impl<T: StTInMtT + Ord> BSTRBMtEph<T> {BSTRBMtEph67,1761
        pub fn new() -> Self {new68,1805
        pub fn size(&self) -> N {size74,1937
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty79,2070
        pub fn height(&self) -> N {height81,2161
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec82,2197
        pub fn insert(&self, value: T) {insert93,2547
        pub fn find(&self, target: &T) -> Option<T> {find101,2812
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains106,2982
        pub fn minimum(&self) -> Option<T> {minimum108,3096
        pub fn maximum(&self) -> Option<T> {maximum113,3248
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order118,3400
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order125,3685
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red132,3972
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link134,4074
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate136,4157
        fn rotate_left(link: &mut Link<T>) {rotate_left138,4276
        fn rotate_right(link: &mut Link<T>) {rotate_right157,4936
        fn flip_colors(link: &mut Link<T>) {flip_colors176,5600
        fn fix_up(link: &mut Link<T>) {fix_up197,6405
        fn insert_link(link: &mut Link<T>, value: T) {insert_link233,7559
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link249,8101
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link264,8618
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link274,8939
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect284,9262
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect292,9549
    impl<T: StTInMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {BSTRBMtEph301,9845
        fn new() -> Self { BSTRBMtEph::new() }new302,9912
        fn insert(&self, value: T) { BSTRBMtEph::insert(self, value) }insert304,9960
        fn find(&self, target: &T) -> Option<T> { BSTRBMtEph::find(self, target) }find306,10032
        fn contains(&self, target: &T) -> B { BSTRBMtEph::contains(self, target) }contains308,10116
        fn size(&self) -> N { BSTRBMtEph::size(self) }size310,10200
        fn is_empty(&self) -> B { BSTRBMtEph::is_empty(self) }is_empty312,10256
        fn height(&self) -> N { BSTRBMtEph::height(self) }height314,10320
        fn minimum(&self) -> Option<T> { BSTRBMtEph::minimum(self) }minimum316,10380
        fn maximum(&self) -> Option<T> { BSTRBMtEph::maximum(self) }maximum318,10450
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTRBMtEph::in_order(self) }in_order320,10520
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTRBMtEph::pre_order(self) }pre_order322,10600
    macro_rules! BSTRBMtEphLit {BSTRBMtEphLit326,10708
    fn _BSTRBMtEphLit_type_checks() {_BSTRBMtEphLit_type_checks338,11198

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayStEph.rs,4387
pub mod BSTSplayStEph {BSTSplayStEph4,176
    type Link<T> = Option<Box<Node<T>>>;Link9,345
    struct Node<T: StT + Ord> {Node13,429
        key: T,key14,461
        size: N,size15,477
        left: Link<T>,left16,494
        right: Link<T>,right17,517
    impl<T: StT + Ord> Node<T> {Node20,548
        fn new(key: T) -> Self {new21,581
    pub struct BSTSplayStEph<T: StT + Ord> {BSTSplayStEph32,795
        root: Link<T>,root33,840
    pub type BSTreeSplay<T> = BSTSplayStEph<T>;BSTreeSplay36,870
    pub trait BSTSplayStEphTrait<T: StT + Ord> {BSTSplayStEphTrait38,919
        fn new() -> Self;new39,968
        fn size(&self) -> N;size40,994
        fn is_empty(&self) -> B;is_empty41,1023
        fn height(&self) -> N;height42,1056
        fn insert(&mut self, value: T);insert43,1087
        fn find(&self, target: &T) -> Option<&T>;find44,1127
        fn contains(&self, target: &T) -> B;contains45,1177
        fn minimum(&self) -> Option<&T>;minimum46,1222
        fn maximum(&self) -> Option<&T>;maximum47,1263
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order48,1304
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order49,1353
    impl<T: StT + Ord> Default for BSTSplayStEph<T> {BSTSplayStEph52,1410
        fn default() -> Self { Self::new() }default53,1464
    impl<T: StT + Ord> BSTSplayStEph<T> {BSTSplayStEph56,1516
        pub fn new() -> Self { BSTSplayStEph { root: None } }new57,1558
        pub fn size(&self) -> N { Self::size_link(&self.root) }size59,1621
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty61,1686
        pub fn height(&self) -> N {height63,1777
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec64,1813
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert73,2109
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find75,2199
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains77,2293
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum79,2407
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum81,2483
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order83,2559
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order89,2783
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link95,3009
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate97,3092
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link99,3211
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link121,3957
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link136,4474
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link146,4795
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect156,5118
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect164,5405
    impl<T: StT + Ord> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {BSTSplayStEph173,5701
        fn new() -> Self { BSTSplayStEph::new() }new174,5769
        fn size(&self) -> N { BSTSplayStEph::size(self) }size176,5820
        fn is_empty(&self) -> B { BSTSplayStEph::is_empty(self) }is_empty178,5879
        fn height(&self) -> N { BSTSplayStEph::height(self) }height180,5946
        fn insert(&mut self, value: T) { BSTSplayStEph::insert(self, value) }insert182,6009
        fn find(&self, target: &T) -> Option<&T> { BSTSplayStEph::find(self, target) }find184,6088
        fn contains(&self, target: &T) -> B { BSTSplayStEph::contains(self, target) }contains186,6176
        fn minimum(&self) -> Option<&T> { BSTSplayStEph::minimum(self) }minimum188,6263
        fn maximum(&self) -> Option<&T> { BSTSplayStEph::maximum(self) }maximum190,6337
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTSplayStEph::in_order(self) }in_order192,6411
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTSplayStEph::pre_order(self) }pre_order194,6494
    macro_rules! BSTSplayStEphLit {BSTSplayStEphLit198,6605
    fn _BSTSplayStEphLit_type_checks() {_BSTSplayStEphLit_type_checks210,7138

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLMtEph.rs,4360
pub mod BSTAVLMtEph {BSTAVLMtEph4,188
    type Link<T> = Option<Box<Node<T>>>;Link11,390
    struct Node<T: StTInMtT + Ord> {Node14,460
        key: T,key15,497
        height: i32,height16,513
        size: N,size17,534
        left: Link<T>,left18,551
        right: Link<T>,right19,574
    impl<T: StTInMtT + Ord> Node<T> {Node22,605
        fn new(key: T) -> Self {new23,643
    pub struct BSTAVLMtEph<T: StTInMtT + Ord> {BSTAVLMtEph35,884
        root: Arc<RwLock<Link<T>>>,root36,932
    pub type BSTreeAVL<T> = BSTAVLMtEph<T>;BSTreeAVL39,975
    pub trait BSTAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTAVLMtEphTrait41,1020
        fn new() -> Self;new42,1079
        fn insert(&self, value: T);insert43,1105
        fn find(&self, target: &T) -> Option<T>;find44,1141
        fn contains(&self, target: &T) -> B;contains45,1190
        fn size(&self) -> N;size46,1235
        fn is_empty(&self) -> B;is_empty47,1264
        fn height(&self) -> N;height48,1297
        fn minimum(&self) -> Option<T>;minimum49,1328
        fn maximum(&self) -> Option<T>;maximum50,1368
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order51,1408
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order52,1457
    impl<T: StTInMtT + Ord> Default for BSTAVLMtEph<T> {BSTAVLMtEph55,1514
        fn default() -> Self { Self::new() }default56,1571
    impl<T: StTInMtT + Ord> BSTAVLMtEph<T> {BSTAVLMtEph59,1623
        pub fn new() -> Self {new60,1668
        pub fn size(&self) -> N {size66,1801
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty71,1934
        pub fn height(&self) -> N {height73,2025
        pub fn insert(&self, value: T) {insert78,2167
        pub fn find(&self, target: &T) -> Option<T> {find83,2326
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains88,2496
        pub fn minimum(&self) -> Option<T> {minimum90,2610
        pub fn maximum(&self) -> Option<T> {maximum95,2762
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order100,2914
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order107,3199
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link114,3486
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link116,3575
        fn update(node: &mut Node<T>) {update118,3658
        fn rotate_right(link: &mut Link<T>) {rotate_right123,3894
        fn rotate_left(link: &mut Link<T>) {rotate_left137,4350
        fn rebalance(link: &mut Link<T>) {rebalance151,4805
        fn insert_link(link: &mut Link<T>, value: T) {insert_link176,5851
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link195,6486
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link210,7003
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link220,7325
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect230,7649
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect238,7936
    impl<T: StTInMtT + Ord> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {BSTAVLMtEph247,8232
        fn new() -> Self { BSTAVLMtEph::new() }new248,8301
        fn insert(&self, value: T) { BSTAVLMtEph::insert(self, value) }insert250,8350
        fn find(&self, target: &T) -> Option<T> { BSTAVLMtEph::find(self, target) }find252,8423
        fn contains(&self, target: &T) -> B { BSTAVLMtEph::contains(self, target) }contains254,8508
        fn size(&self) -> N { BSTAVLMtEph::size(self) }size256,8593
        fn is_empty(&self) -> B { BSTAVLMtEph::is_empty(self) }is_empty258,8650
        fn height(&self) -> N { BSTAVLMtEph::height(self) }height260,8715
        fn minimum(&self) -> Option<T> { BSTAVLMtEph::minimum(self) }minimum262,8776
        fn maximum(&self) -> Option<T> { BSTAVLMtEph::maximum(self) }maximum264,8847
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTAVLMtEph::in_order(self) }in_order266,8918
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTAVLMtEph::pre_order(self) }pre_order268,8999
    macro_rules! BSTAVLMtEphLit {BSTAVLMtEphLit272,9108
    fn _BSTAVLMtEphLit_type_checks() {_BSTAVLMtEphLit_type_checks284,9611

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainStEph.rs,3573
pub mod BSTPlainStEph {BSTPlainStEph4,156
    pub struct BSTPlainStEph<T: StT + Ord> {BSTPlainStEph10,344
        root: BBTree<T>,root11,389
    pub type BSTree<T> = BSTPlainStEph<T>;BSTree14,421
    pub trait BSTPlainStEphTrait<T: StT + Ord> {BSTPlainStEphTrait16,465
        fn new() -> Self;new17,514
        fn size(&self) -> N;size18,540
        fn is_empty(&self) -> B;is_empty19,569
        fn height(&self) -> N;height20,602
        fn insert(&mut self, value: T);insert21,633
        fn find(&self, target: &T) -> Option<&T>;find22,673
        fn contains(&self, target: &T) -> B;contains23,723
        fn minimum(&self) -> Option<&T>;minimum24,768
        fn maximum(&self) -> Option<&T>;maximum25,809
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order26,850
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order27,899
    impl<T: StT + Ord> BSTPlainStEph<T> {BSTPlainStEph30,956
        pub fn new() -> Self { BSTPlainStEph { root: BBTree::leaf() } }new31,998
        pub fn size(&self) -> N { self.root.size() }size33,1071
        pub fn is_empty(&self) -> B { self.root.is_leaf() }is_empty35,1125
        pub fn height(&self) -> N { self.root.height() }height37,1186
        pub fn insert(&mut self, value: T) { insert_node(&mut self.root, value); }insert39,1244
        pub fn find(&self, target: &T) -> Option<&T> { find_node(&self.root, target) }find41,1328
        pub fn contains(&self, target: &T) -> B { contains_node(&self.root, target) }contains43,1416
        pub fn minimum(&self) -> Option<&T> { min_node(&self.root) }minimum45,1503
        pub fn maximum(&self) -> Option<&T> { max_node(&self.root) }maximum47,1573
        pub fn in_order(&self) -> ArraySeqStPerS<T> { self.root.in_order() }in_order49,1643
        pub fn pre_order(&self) -> ArraySeqStPerS<T> { self.root.pre_order() }pre_order51,1721
    impl<T: StT + Ord> BSTPlainStEphTrait<T> for BSTPlainStEph<T> {BSTPlainStEph54,1807
        fn new() -> Self { BSTPlainStEph::new() }new55,1875
        fn size(&self) -> N { BSTPlainStEph::size(self) }size57,1926
        fn is_empty(&self) -> B { BSTPlainStEph::is_empty(self) }is_empty59,1985
        fn height(&self) -> N { BSTPlainStEph::height(self) }height61,2052
        fn insert(&mut self, value: T) { BSTPlainStEph::insert(self, value) }insert63,2115
        fn find(&self, target: &T) -> Option<&T> { BSTPlainStEph::find(self, target) }find65,2194
        fn contains(&self, target: &T) -> B { BSTPlainStEph::contains(self, target) }contains67,2282
        fn minimum(&self) -> Option<&T> { BSTPlainStEph::minimum(self) }minimum69,2369
        fn maximum(&self) -> Option<&T> { BSTPlainStEph::maximum(self) }maximum71,2443
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTPlainStEph::in_order(self) }in_order73,2517
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTPlainStEph::pre_order(self) }pre_order75,2600
    fn insert_node<T: StT + Ord>(node: &mut BBTree<T>, value: T) {insert_node78,2691
    fn contains_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> B {contains_node93,3194
    fn find_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> Option<&'a T> {find_node108,3680
    fn min_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {min_node123,4173
    fn max_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {max_node133,4493
    macro_rules! BSTPlainStEphLit {BSTPlainStEphLit144,4835
    fn _BSTPlainStEphLit_type_checks() {_BSTPlainStEphLit_type_checks159,5407

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetBBAlphaMtEph.rs,5700
pub mod BSTSetBBAlphaMtEph {BSTSetBBAlphaMtEph4,170
    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord> {BSTSetBBAlphaMtEph12,427
        tree: BSTBBAlphaMtEph<T>,tree13,482
    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;BSTSetBBAlphaMt16,523
    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetBBAlphaMtEphTrait18,581
        fn empty() -> Self;empty19,647
        fn singleton(value: T) -> Self;singleton20,675
        fn size(&self) -> N;size21,715
        fn is_empty(&self) -> B;is_empty22,744
        fn find(&self, value: &T) -> Option<T>;find23,777
        fn contains(&self, value: &T) -> B;contains24,825
        fn minimum(&self) -> Option<T>;minimum25,869
        fn maximum(&self) -> Option<T>;maximum26,909
        fn insert(&mut self, value: T);insert27,949
        fn delete(&mut self, target: &T);delete28,989
        fn union(&self, other: &Self) -> Self;union29,1031
        fn intersection(&self, other: &Self) -> Self;intersection30,1078
        fn difference(&self, other: &Self) -> Self;difference31,1132
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1184
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1239
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1294
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1356
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1426
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1494
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T>;as_tree38,1548
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph41,1605
        pub fn empty() -> Self {empty42,1657
        pub fn singleton(value: T) -> Self {singleton48,1780
        pub fn size(&self) -> N { self.tree.size() }size54,1941
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1995
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2057
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2135
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2213
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2281
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2349
        pub fn delete(&mut self, target: &T) {delete68,2422
        pub fn union(&self, other: &Self) -> Self {union76,2711
        pub fn intersection(&self, other: &Self) -> Self {intersection84,3010
        pub fn difference(&self, other: &Self) -> Self {difference101,3588
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4165
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4856
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5169
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5525
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5934
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6198
        pub fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree180,6281
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6350
        fn rebuild_from_vec(values: Vec<T>) -> BSTBBAlphaMtEph<T> {rebuild_from_vec184,6441
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6668
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph204,6955
        fn empty() -> Self { Self::empty() }empty205,7038
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7084
        fn size(&self) -> N { self.tree.size() }size209,7151
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7201
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7259
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7333
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7407
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7471
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7535
        fn delete(&mut self, target: &T) { BSTSetBBAlphaMtEph::delete(self, target) }delete223,7604
        fn union(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::union(self, other) }union225,7691
        fn intersection(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::intersection(self, otintersection227,7781
        fn difference(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::difference(self, other)difference229,7885
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetBBAlphaMtEph::split(self, pivot) }split231,7985
        fn join_pair(left: Self, right: Self) -> Self { BSTSetBBAlphaMtEph::join_pair(left, righjoin_pair233,8083
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetBBAlphaMtEph::join_m(left, join_m235,8185
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetBBAlphaMtEph::filtefilter237,8298
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetBBAlphaMtEph::reduce(reduce239,8416
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8531
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree243,8610
    macro_rules! BSTSetBBAlphaMtEphLit {BSTSetBBAlphaMtEphLit247,8701
    fn _BSTSetBBAlphaMtEphLit_type_checks() {_BSTSetBBAlphaMtEphLit_type_checks259,9300

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap38/BSTParaStEph.rs,3106
pub mod BSTParaStEph {BSTParaStEph4,162
    pub enum Exposed<T: StT + Ord> {Exposed12,361
        Leaf,Leaf14,417
        Node(ParamBST<T>, T, ParamBST<T>),Node15,431
    struct NodeInner<T: StT + Ord> {NodeInner19,509
        key: T,key20,546
        size: N,size21,562
        left: ParamBST<T>,left22,579
        right: ParamBST<T>,right23,606
    pub struct ParamBST<T: StT + Ord> {ParamBST27,669
        root: Rc<RefCell<Option<Box<NodeInner<T>>>>>,root28,709
    pub trait ParamBSTTrait<T: StT + Ord>: Sized {ParamBSTTrait31,770
        fn new() -> Self;new32,821
        fn expose(&self) -> Exposed<T>;expose33,847
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid34,887
        fn size(&self) -> N;size35,937
        fn is_empty(&self) -> B;is_empty36,966
        fn insert(&self, key: T);insert37,999
        fn delete(&self, key: &T);delete38,1033
        fn find(&self, key: &T) -> Option<T>;find39,1068
        fn split(&self, key: &T) -> (Self, B, Self);split40,1114
        fn join_pair(&self, other: Self) -> Self;join_pair41,1167
        fn union(&self, other: &Self) -> Self;union42,1217
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order43,1264
    impl<T: StT + Ord> ParamBST<T> {ParamBST46,1320
        fn expose_internal(&self) -> Exposed<T> {expose_internal47,1357
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid55,1649
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner67,2101
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m86,3086
        fn min_key(tree: &Self) -> Option<T> {min_key88,3202
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner98,3546
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner109,4016
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order121,4523
    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {ParamBST133,4925
        fn new() -> Self {new134,4983
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose140,5109
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid142,5176
        fn size(&self) -> N { self.root.borrow().as_ref().map_or(0, |node| node.size) }size144,5258
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty146,5347
        fn insert(&self, key: T) {insert148,5434
        fn delete(&self, key: &T) {delete155,5724
        fn find(&self, key: &T) -> Option<T> {find162,6016
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split173,6513
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair175,6603
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union177,6704
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order179,6790
    macro_rules! ParamBSTLit {ParamBSTLit187,7034
    fn _ParamBSTLit_type_checks() {_ParamBSTLit_type_checks200,7557

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap38/BSTParaMtEph.rs,4171
pub mod BSTParaMtEph {BSTParaMtEph4,161
    pub enum Exposed<T: StTInMtT + Ord> {Exposed11,329
        Leaf,Leaf12,371
        Node(ParamBST<T>, T, ParamBST<T>),Node13,385
    struct NodeInner<T: StTInMtT + Ord> {NodeInner18,477
        key: T,key19,519
        size: N,size20,535
        left: ParamBST<T>,left21,552
        right: ParamBST<T>,right22,579
    pub struct ParamBST<T: StTInMtT + Ord> {ParamBST26,642
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root27,687
    pub trait ParamBSTTrait<T: StTInMtT + Ord + 'static>: Sized {ParamBSTTrait30,748
        fn new() -> Self;new33,905
        fn expose(&self) -> Exposed<T>;expose36,1022
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid39,1153
        fn size(&self) -> N;size42,1294
        fn is_empty(&self) -> B;is_empty45,1414
        fn insert(&self, key: T);insert48,1558
        fn delete(&self, key: &T);delete51,1703
        fn find(&self, key: &T) -> Option<T>;find54,1849
        fn split(&self, key: &T) -> (Self, B, Self);split57,2006
        fn join_pair(&self, other: Self) -> Self;join_pair60,2218
        fn union(&self, other: &Self) -> Self;union63,2393
        fn intersect(&self, other: &Self) -> Self;intersect66,2565
        fn difference(&self, other: &Self) -> Self;difference69,2741
        fn filter<F>(&self, predicate: F) -> Selffilter72,2898
        fn reduce<F>(&self, op: F, base: T) -> Treduce77,3122
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order82,3338
    impl<T: StTInMtT + Ord + 'static> ParamBST<T> {ParamBST85,3394
        fn expose_internal(&self) -> Exposed<T> {expose_internal88,3537
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid98,3927
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner112,4490
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m133,5566
        fn min_key(tree: &Self) -> Option<T> {min_key137,5793
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner149,6308
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner162,6903
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner178,7620
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner198,8583
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner219,9559
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel243,10641
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner253,11003
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel277,12082
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order287,12421
    impl<T: StTInMtT + Ord + 'static> ParamBSTTrait<T> for ParamBST<T> {ParamBST299,12823
        fn new() -> Self {new302,12987
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose310,13204
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid314,13362
        fn size(&self) -> N {size318,13535
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty325,13773
        fn insert(&self, key: T) {insert329,13971
        fn delete(&self, key: &T) {delete339,14414
        fn find(&self, key: &T) -> Option<T> {find349,14859
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split362,15467
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair366,15716
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union370,15942
        fn intersect(&self, other: &Self) -> Self { ParamBST::intersect_inner(self, other) }intersect374,16153
        fn difference(&self, other: &Self) -> Self { ParamBST::difference_inner(self, other) }difference378,16372
        fn filter<F>(&self, predicate: F) -> Selffilter382,16573
        fn reduce<F>(&self, op: F, base: T) -> Treduce391,16873
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order400,17164

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTTreapMtEph.rs,4435
pub mod BSTTreapMtEph {BSTTreapMtEph4,192
    type Link<T> = Option<Box<Node<T>>>;Link11,366
    struct Node<T: StTInMtT + Ord> {Node14,436
        key: T,key15,473
        priority: u64,priority16,489
        size: N,size17,512
        left: Link<T>,left18,529
        right: Link<T>,right19,552
    impl<T: StTInMtT + Ord> Node<T> {Node22,583
        fn new(key: T, priority: u64) -> Self {new23,621
    pub struct BSTTreapMtEph<T: StTInMtT + Ord> {BSTTreapMtEph35,876
        root: Arc<RwLock<Link<T>>>,root36,926
    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;BSTreeTreap39,969
    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTTreapMtEphTrait41,1018
        fn new() -> Self;new42,1079
        fn insert(&self, value: T);insert43,1105
        fn find(&self, target: &T) -> Option<T>;find44,1141
        fn contains(&self, target: &T) -> B;contains45,1190
        fn size(&self) -> N;size46,1235
        fn is_empty(&self) -> B;is_empty47,1264
        fn height(&self) -> N;height48,1297
        fn minimum(&self) -> Option<T>;minimum49,1328
        fn maximum(&self) -> Option<T>;maximum50,1368
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order51,1408
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order52,1457
    impl<T: StTInMtT + Ord> Default for BSTTreapMtEph<T> {BSTTreapMtEph55,1514
        fn default() -> Self { Self::new() }default56,1573
    impl<T: StTInMtT + Ord> BSTTreapMtEph<T> {BSTTreapMtEph59,1625
        pub fn new() -> Self {new60,1672
        pub fn size(&self) -> N {size66,1807
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty71,1940
        pub fn height(&self) -> N {height73,2031
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec74,2067
        pub fn insert(&self, value: T) {insert85,2417
        pub fn find(&self, target: &T) -> Option<T> {find91,2619
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains96,2789
        pub fn minimum(&self) -> Option<T> {minimum98,2903
        pub fn maximum(&self) -> Option<T> {maximum103,3055
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order108,3207
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order115,3492
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link122,3779
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate124,3862
        fn rotate_left(link: &mut Link<T>) {rotate_left126,3981
        fn rotate_right(link: &mut Link<T>) {rotate_right140,4436
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link154,4892
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link179,5912
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link194,6429
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link204,6750
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect214,7073
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect222,7360
    impl<T: StTInMtT + Ord> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {BSTTreapMtEph231,7656
        fn new() -> Self { BSTTreapMtEph::new() }new232,7729
        fn insert(&self, value: T) { BSTTreapMtEph::insert(self, value) }insert234,7780
        fn find(&self, target: &T) -> Option<T> { BSTTreapMtEph::find(self, target) }find236,7855
        fn contains(&self, target: &T) -> B { BSTTreapMtEph::contains(self, target) }contains238,7942
        fn size(&self) -> N { BSTTreapMtEph::size(self) }size240,8029
        fn is_empty(&self) -> B { BSTTreapMtEph::is_empty(self) }is_empty242,8088
        fn height(&self) -> N { BSTTreapMtEph::height(self) }height244,8155
        fn minimum(&self) -> Option<T> { BSTTreapMtEph::minimum(self) }minimum246,8218
        fn maximum(&self) -> Option<T> { BSTTreapMtEph::maximum(self) }maximum248,8291
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::in_order(self) }in_order250,8364
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::pre_order(self) }pre_order252,8447
    macro_rules! BSTTreapMtEphLit {BSTTreapMtEphLit256,8558
    fn _BSTTreapMtEphLit_type_checks() {_BSTTreapMtEphLit_type_checks268,9087

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTSetTreapMtEph.rs,5647
pub mod BSTSetTreapMtEph {BSTSetTreapMtEph4,167
    pub struct BSTSetTreapMtEph<T: StTInMtT + Ord> {BSTSetTreapMtEph12,404
        tree: BSTTreapMtEph<T>,tree13,457
    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;BSTSetTreapMt16,496
    pub trait BSTSetTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetTreapMtEphTrait18,550
        fn empty() -> Self;empty19,614
        fn singleton(value: T) -> Self;singleton20,642
        fn size(&self) -> N;size21,682
        fn is_empty(&self) -> B;is_empty22,711
        fn find(&self, value: &T) -> Option<T>;find23,744
        fn contains(&self, value: &T) -> B;contains24,792
        fn minimum(&self) -> Option<T>;minimum25,836
        fn maximum(&self) -> Option<T>;maximum26,876
        fn insert(&mut self, value: T);insert27,916
        fn delete(&mut self, target: &T);delete28,956
        fn union(&self, other: &Self) -> Self;union29,998
        fn intersection(&self, other: &Self) -> Self;intersection30,1045
        fn difference(&self, other: &Self) -> Self;difference31,1099
        fn split(&self, pivot: &T) -> (Self, B, Self);split32,1151
        fn join_pair(left: Self, right: Self) -> Self;join_pair33,1206
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m34,1261
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter35,1323
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce36,1393
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;iter_in_order37,1461
        fn as_tree(&self) -> &BSTTreapMtEph<T>;as_tree38,1515
    impl<T: StTInMtT + Ord> BSTSetTreapMtEph<T> {BSTSetTreapMtEph41,1570
        pub fn empty() -> Self {empty42,1620
        pub fn singleton(value: T) -> Self {singleton48,1741
        pub fn size(&self) -> N { self.tree.size() }size54,1900
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty56,1954
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find58,2016
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains60,2094
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum62,2172
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum64,2240
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert66,2308
        pub fn delete(&mut self, target: &T) {delete68,2381
        pub fn union(&self, other: &Self) -> Self {union76,2670
        pub fn intersection(&self, other: &Self) -> Self {intersection84,2969
        pub fn difference(&self, other: &Self) -> Self {difference101,3547
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split118,4124
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair138,4815
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m146,5128
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter155,5484
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce168,5893
        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order178,6157
        pub fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree180,6240
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec182,6307
        fn rebuild_from_vec(values: Vec<T>) -> BSTTreapMtEph<T> {rebuild_from_vec184,6398
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter192,6621
    impl<T: StTInMtT + Ord> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {BSTSetTreapMtEph204,6906
        fn empty() -> Self { Self::empty() }empty205,6985
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton207,7031
        fn size(&self) -> N { self.tree.size() }size209,7098
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty211,7148
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find213,7206
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains215,7280
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum217,7354
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum219,7418
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert221,7482
        fn delete(&mut self, target: &T) { BSTSetTreapMtEph::delete(self, target) }delete223,7551
        fn union(&self, other: &Self) -> Self { BSTSetTreapMtEph::union(self, other) }union225,7636
        fn intersection(&self, other: &Self) -> Self { BSTSetTreapMtEph::intersection(self, otheintersection227,7724
        fn difference(&self, other: &Self) -> Self { BSTSetTreapMtEph::difference(self, other) }difference229,7826
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetTreapMtEph::split(self, pivot) }split231,7924
        fn join_pair(left: Self, right: Self) -> Self { BSTSetTreapMtEph::join_pair(left, right)join_pair233,8020
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetTreapMtEph::join_m(left, pijoin_m235,8120
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetTreapMtEph::filter(filter237,8231
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetTreapMtEph::reduce(sereduce239,8347
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }iter_in_order241,8460
        fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree243,8539
    macro_rules! BSTSetTreapMtEphLit {BSTSetTreapMtEphLit247,8628
    fn _BSTSetTreapMtEphLit_type_checks() {_BSTSetTreapMtEphLit_type_checks259,9201

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTParaTreapMtEph.rs,4882
pub mod BSTParaTreapMtEph {BSTParaTreapMtEph4,188
    pub enum Exposed<T: StTInMtT + Ord> {Exposed13,421
        Leaf,Leaf14,463
        Node(ParamTreap<T>, T, ParamTreap<T>),Node15,477
    struct NodeInner<T: StTInMtT + Ord> {NodeInner19,552
        key: T,key20,594
        priority: i64,priority21,610
        size: N,size22,633
        left: ParamTreap<T>,left23,650
        right: ParamTreap<T>,right24,679
    pub struct ParamTreap<T: StTInMtT + Ord> {ParamTreap28,737
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root29,784
    fn priority_for<T: StTInMtT + Ord>(key: &T) -> i64 {priority_for32,845
    fn tree_priority<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> i64 {tree_priority40,1138
    fn tree_size<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> N {tree_size45,1325
    fn make_node<T: StTInMtT + Ord>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreamake_node50,1495
    impl<T: StTInMtT + Ord + 'static> ParamTreap<T> {ParamTreap63,1913
        fn expose_internal(&self) -> Exposed<T> {expose_internal66,2058
        pub fn expose_with_priority(&self) -> Option<(ParamTreap<T>, T, i64, ParamTreap<T>)> {expose_with_priority76,2448
        fn join_with_priority(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) join_with_priority85,2922
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid108,4099
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner120,4575
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner141,5706
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner155,6460
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner171,7204
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner191,8134
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner211,9047
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel235,10160
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner245,10524
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel269,11597
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order279,11938
    pub trait ParamTreapTrait<T: StTInMtT + Ord + 'static>: Sized {ParamTreapTrait291,12344
        fn new() -> Self;new294,12503
        fn expose(&self) -> Exposed<T>;expose297,12620
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid300,12751
        fn size(&self) -> N;size303,12892
        fn is_empty(&self) -> B;is_empty306,13012
        fn insert(&self, key: T);insert309,13156
        fn delete(&self, key: &T);delete312,13301
        fn find(&self, key: &T) -> Option<T>;find315,13447
        fn split(&self, key: &T) -> (Self, B, Self);split318,13604
        fn join_pair(&self, other: Self) -> Self;join_pair321,13816
        fn union(&self, other: &Self) -> Self;union324,13991
        fn intersect(&self, other: &Self) -> Self;intersect327,14163
        fn difference(&self, other: &Self) -> Self;difference330,14339
        fn filter<F>(&self, predicate: F) -> Selffilter333,14496
        fn reduce<F>(&self, op: F, base: T) -> Treduce338,14720
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order343,14936
    impl<T: StTInMtT + Ord + 'static> ParamTreapTrait<T> for ParamTreap<T> {ParamTreap346,14992
        fn new() -> Self {new349,15160
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose357,15379
        fn join_mid(exposed: Exposed<T>) -> Self { ParamTreap::join_mid(exposed) }join_mid361,15537
        fn size(&self) -> N { tree_size(self) }size365,15712
        fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty369,15852
        fn insert(&self, key: T) {insert373,16050
        fn delete(&self, key: &T) {delete384,16566
        fn find(&self, key: &T) -> Option<T> {find394,17015
        fn split(&self, key: &T) -> (Self, B, Self) { ParamTreap::split_inner(self, key) }split407,17627
        fn join_pair(&self, other: Self) -> Self { ParamTreap::join_pair_inner(self.clone(), othjoin_pair411,17878
        fn union(&self, other: &Self) -> Self { ParamTreap::union_inner(self, other) }union415,18106
        fn intersect(&self, other: &Self) -> Self { ParamTreap::intersect_inner(self, other) }intersect419,18319
        fn difference(&self, other: &Self) -> Self { ParamTreap::difference_inner(self, other) }difference423,18540
        fn filter<F>(&self, predicate: F) -> Selffilter427,18743
        fn reduce<F>(&self, op: F, base: T) -> Treduce436,19045
        fn in_order(&self) -> ArraySeqStPerS<T> {in_order445,19338
    macro_rules! ParamTreapLit {ParamTreapLit453,19584
    fn _ParamTreapLit_type_checks() {_ParamTreapLit_type_checks465,20130

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap39/BSTTreapStEph.rs,4545
pub mod BSTTreapStEph {BSTTreapStEph4,164
    type Link<T> = Option<Box<Node<T>>>;Link10,359
    struct Node<T: StT + Ord> {Node14,443
        key: T,key15,475
        priority: u64,priority16,491
        size: N,size17,514
        left: Link<T>,left18,531
        right: Link<T>,right19,554
    impl<T: StT + Ord> Node<T> {Node22,585
        fn new(key: T, priority: u64) -> Self {new23,618
    pub struct BSTTreapStEph<T: StT + Ord> {BSTTreapStEph35,873
        root: Link<T>,root36,918
    pub type BSTreeTreap<T> = BSTTreapStEph<T>;BSTreeTreap39,948
    pub trait BSTTreapStEphTrait<T: StT + Ord> {BSTTreapStEphTrait41,997
        fn new() -> Self;new42,1046
        fn size(&self) -> N;size43,1072
        fn is_empty(&self) -> B;is_empty44,1101
        fn height(&self) -> N;height45,1134
        fn insert(&mut self, value: T);insert46,1165
        fn find(&self, target: &T) -> Option<&T>;find47,1205
        fn contains(&self, target: &T) -> B;contains48,1255
        fn minimum(&self) -> Option<&T>;minimum49,1300
        fn maximum(&self) -> Option<&T>;maximum50,1341
        fn in_order(&self) -> ArraySeqStPerS<T>;in_order51,1382
        fn pre_order(&self) -> ArraySeqStPerS<T>;pre_order52,1431
    impl<T: StT + Ord> Default for BSTreeTreap<T> {BSTreeTreap55,1488
        fn default() -> Self { Self::new() }default56,1540
    impl<T: StT + Ord> BSTTreapStEph<T> {BSTTreapStEph59,1592
        pub fn new() -> Self { BSTTreapStEph { root: None } }new60,1634
        pub fn size(&self) -> N { Self::size_link(&self.root) }size62,1697
        pub fn is_empty(&self) -> B { if self.size() == 0 { B::True } else { B::False } }is_empty64,1762
        pub fn height(&self) -> N {height66,1853
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec67,1889
        pub fn insert(&mut self, value: T) {insert76,2185
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find81,2334
        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { B::True } elscontains83,2428
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum85,2542
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum87,2618
        pub fn in_order(&self) -> ArraySeqStPerS<T> {in_order89,2694
        pub fn pre_order(&self) -> ArraySeqStPerS<T> {pre_order95,2918
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link101,3144
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate103,3227
        fn rotate_left(link: &mut Link<T>) {rotate_left105,3346
        fn rotate_right(link: &mut Link<T>) {rotate_right119,3801
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link133,4257
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link158,5277
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link173,5794
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link183,6115
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect193,6438
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect201,6725
    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {BSTTreapStEph210,7021
        fn new() -> Self { BSTTreapStEph::new() }new211,7089
        fn size(&self) -> N { BSTTreapStEph::size(self) }size213,7140
        fn is_empty(&self) -> B { BSTTreapStEph::is_empty(self) }is_empty215,7199
        fn height(&self) -> N { BSTTreapStEph::height(self) }height217,7266
        fn insert(&mut self, value: T) { BSTTreapStEph::insert(self, value) }insert219,7329
        fn find(&self, target: &T) -> Option<&T> { BSTTreapStEph::find(self, target) }find221,7408
        fn contains(&self, target: &T) -> B { BSTTreapStEph::contains(self, target) }contains223,7496
        fn minimum(&self) -> Option<&T> { BSTTreapStEph::minimum(self) }minimum225,7583
        fn maximum(&self) -> Option<&T> { BSTTreapStEph::maximum(self) }maximum227,7657
        fn in_order(&self) -> ArraySeqStPerS<T> { BSTTreapStEph::in_order(self) }in_order229,7731
        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTTreapStEph::pre_order(self) }pre_order231,7814
    macro_rules! BSTTreapStEphLit {BSTTreapStEphLit235,7925
    fn _BSTTreapStEphLit_type_checks() {_BSTTreapStEphLit_type_checks247,8458

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test42BBTEph.rs,231
pub mod Test42BBTEph {Test42BBTEph3,93
fn inorder_and_preorder_traversals_match_definitions() {inorder_and_preorder_traversals_match_definitions10,319
fn bst_insert_and_search_behavior() {bst_insert_and_search_behavior25,819

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test01Types.rs,721
pub mod TestTypes {TestTypes3,93
pub mod Test01Types {Test01Types6,148
    fn test_boolean_eq_neq_and_ordering() {test_boolean_eq_neq_and_ordering10,213
    fn test_ordering_on_n_naturals() {test_ordering_on_n_naturals20,470
    fn test_cmp_on_b_returns_expected_ordering_variants() {test_cmp_on_b_returns_expected_ordering_variants29,717
    fn test_btree_set_orders_b_true_before_false() {test_btree_set_orders_b_true_before_false37,1041
    fn test_n_aliases_usize_and_cmp_examples() {test_n_aliases_usize_and_cmp_examples49,1490
    fn test_debug_format_for_b_variants() {test_debug_format_for_b_variants64,1943
    fn test_display_format_for_b_variants() {test_display_format_for_b_variants70,2116

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test12ArraySeqStEph.rs,283
pub mod TestArraySeqEph {TestArraySeqEph3,93
    fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic8,224
    fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter17,502
    fn test_iterators_collect() {test_iterators_collect28,1032

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test14ArraySeqStEph.rs,1250
pub mod TestArraySeqStEph {TestArraySeqStEph3,93
    fn test_empty() {test_empty8,226
    fn test_singleton() {test_singleton14,362
    fn test_map() {test_map20,532
    fn test_append() {test_append27,790
    fn test_append2() {test_append235,1121
    fn test_filter_even_numbers() {test_filter_even_numbers44,1454
    fn test_filter_none() {test_filter_none51,1779
    fn test_update_in_bounds() {test_update_in_bounds59,2101
    fn test_update_out_of_bounds() {test_update_out_of_bounds66,2350
    fn test_isEmpty() {test_isEmpty73,2596
    fn test_isSingleton() {test_isSingleton83,2964
    fn test_iterate_sum() {test_iterate_sum93,3348
    fn test_iterate_concat() {test_iterate_concat100,3580
    fn test_map_empty() {test_map_empty114,3999
    fn test_append_with_empty() {test_append_with_empty121,4223
    fn test_append2_equivalence() {test_append2_equivalence131,4700
    fn test_filter_empty_sequence() {test_filter_empty_sequence140,5071
    fn test_select_boundary() {test_select_boundary147,5312
    fn test_reduce_sum_basic_ch19() {test_reduce_sum_basic_ch19160,5777
    fn test_scan_sum_basic_ch19() {test_scan_sum_basic_ch19174,6359
    fn test_flatten_ch19() {test_flatten_ch19185,6759

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test13ArraySeqStEph18.rs,925
pub mod TestArraySeqStEph {TestArraySeqStEph3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci8,226
        fn fib(n: N) -> N {fib9,261
    fn test_map_increment() {test_map_increment37,1001
    fn test_append() {test_append45,1267
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append53,1574
    fn test_filter_even() {test_filter_even66,2301
    fn test_flatten() {test_flatten92,3160
    fn test_update_sequence() {test_update_sequence106,3989
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins118,4552
    fn test_inject_conflicts_last_wins_2() {test_inject_conflicts_last_wins_2137,5390
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan155,6170
    fn test_iterate_sum_basic() {test_iterate_sum_basic174,7198
    fn test_collect_groups_by_key() {test_collect_groups_by_key183,7484

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test10ArraySeqStPer18.rs,936
pub mod TestArraySeqStPer {TestArraySeqStPer3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci8,226
        fn fib(n: N) -> N {fib9,261
    fn test_map_increment() {test_map_increment37,990
    fn test_subseq() {test_subseq44,1259
    fn test_append() {test_append55,1641
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1952
    fn test_filter_even() {test_filter_even76,2653
    fn test_flatten() {test_flatten103,3607
    fn test_update_sequence() {test_update_sequence117,4533
    fn test_inject_and_ninject() {test_inject_and_ninject127,5162
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan162,6755
    fn test_iterate_sum_basic() {test_iterate_sum_basic181,7753
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum189,8038
    fn test_collect_groups_by_key() {test_collect_groups_by_key201,8501

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test28ArraySeqMtPer.rs,474
pub mod Test28ArraySeqMtPer {Test28ArraySeqMtPer4,147
    fn test_inject_basic() {test_inject_basic12,335
    fn test_inject_conflicting_updates() {test_inject_conflicting_updates28,975
    fn test_inject_out_of_bounds() {test_inject_out_of_bounds44,1674
    fn test_inject_empty_changes() {test_inject_empty_changes56,2217
    fn test_inject_empty_values() {test_inject_empty_values68,2671
    fn test_inject_string_values() {test_inject_string_values81,3255

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test41ArraySeqMtEph.rs,212
pub mod Test41ArraySeqMtEph {Test41ArraySeqMtEph3,93
fn test_arrayseq_mteph_basic_ops() {test_arrayseq_mteph_basic_ops9,244
fn test_arrayseq_mteph_append_and_map() {test_arrayseq_mteph_append_and_map26,717

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test26ArraySeqMtPer.rs,910
pub mod Test26ArraySeqMtPer {Test26ArraySeqMtPer3,93
    fn test_new_and_set() {test_new_and_set8,228
    fn test_length_and_nth_basic() {test_length_and_nth_basic22,699
    fn test_empty() {test_empty30,924
    fn test_sequence_basic() {test_sequence_basic37,1123
    fn test_singleton() {test_singleton53,1729
    fn test_from_vec() {test_from_vec61,1953
    fn test_subseq_copy() {test_subseq_copy69,2163
    fn test_subseq_view() {test_subseq_view79,2465
    fn test_iterators() {test_iterators89,2772
    fn test_set_out_of_bounds() {test_set_out_of_bounds102,3263
    fn test_macro_literals() {test_macro_literals109,3460
    fn test_equality_and_debug() {test_equality_and_debug129,4126
    fn test_display_format() {test_display_format145,4611
    fn test_string_sequences() {test_string_sequences154,4894
    fn test_boolean_sequences() {test_boolean_sequences162,5127

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test11ArraySeqStPer.rs,323
pub mod TestArraySeqPer {TestArraySeqPer3,93
    fn test_map_and_select_and_append() {test_map_and_select_and_append10,251
    fn test_filter() {test_filter23,813
    fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten31,1159
    fn test_inject_and_parallel() {test_inject_and_parallel50,2192

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test27ArraySeqMtPer18.rs,586
pub mod Test27ArraySeqMtPer {Test27ArraySeqMtPer4,147
    fn test_tabulate_basic() {test_tabulate_basic9,282
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci23,754
        fn fib(n: N) -> N {fib24,789
    fn test_tabulate_empty() {test_tabulate_empty56,1818
    fn test_tabulate_single() {test_tabulate_single63,2033
    fn test_tabulate_string() {test_tabulate_string70,2242
    fn test_tabulate_boolean() {test_tabulate_boolean86,3012
    fn test_tabulate_squares() {test_tabulate_squares101,3680
    fn test_tabulate_large() {test_tabulate_large116,4259

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test09ArraySeqStPer.rs,2134
pub mod TestArraySeqStPer {TestArraySeqStPer2,92
    fn test_new_and_set() {test_new_and_set8,226
    fn test_length_and_nth_basic() {test_length_and_nth_basic24,822
    fn test_empty() {test_empty32,1047
    fn test_sequence_basic() {test_sequence_basic39,1237
    fn test_singleton() {test_singleton55,2105
    fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton63,2313
    fn test_from_vec() {test_from_vec78,2804
    fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers93,3515
    fn test_sequence_equality_strings() {test_sequence_equality_strings118,4577
    fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference143,5733
        struct PartialComparable {PartialComparable145,5824
            value: i32,value146,5859
        impl std::fmt::Display for PartialComparable {PartialComparable149,5945
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt150,6000
        struct TotalComparable {TotalComparable162,6692
            value: N,value163,6725
        impl std::fmt::Display for TotalComparable {TotalComparable166,6758
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt167,6811
    fn test_ordering_numbers_basic() {test_ordering_numbers_basic182,7524
    fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal191,7771
    fn test_ordering_strings_basic() {test_ordering_strings_basic197,7899
    fn test_strings_equal_is_equal() {test_strings_equal_is_equal206,8144
    fn test_nth_on_empty_panics() {test_nth_on_empty_panics213,8293
    fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics220,8446
    fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap227,8599
    fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes234,8832
    fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic242,9092
    fn test_new_set_persistent() {test_new_set_persistent251,9475
    fn test_iterator_collects_in_order() {test_iterator_collects_in_order261,9814

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test24DirGraphStEph.rs,131
pub mod TestDirGraphStEph {TestDirGraphStEph2,92
    fn test_digraph_vertices_and_arcs() {test_digraph_vertices_and_arcs10,388

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test26LabDirGraphStEph.rs,1040
pub mod TestLabDirGraphStEph {TestLabDirGraphStEph3,93
    fn test_labelled_dir_graph_empty() {test_labelled_dir_graph_empty12,436
    fn test_labelled_dir_graph_add_vertex() {test_labelled_dir_graph_add_vertex20,727
    fn test_labelled_dir_graph_add_labeled_arc() {test_labelled_dir_graph_add_labeled_arc32,1111
    fn test_labelled_dir_graph_neighbors() {test_labelled_dir_graph_neighbors49,1721
    fn test_labelled_dir_graph_arcs() {test_labelled_dir_graph_arcs73,2575
    fn test_labelled_dir_graph_macro_empty() {test_labelled_dir_graph_macro_empty85,2964
    fn test_labelled_dir_graph_macro_with_data() {test_labelled_dir_graph_macro_with_data92,3191
    fn test_labelled_dir_graph_different_label_types() {test_labelled_dir_graph_different_label_types107,3675
    fn test_labelled_dir_graph_display() {test_labelled_dir_graph_display124,4161
    fn test_labelled_dir_graph_debug() {test_labelled_dir_graph_debug136,4515
    fn test_labelled_dir_graph_self_loop() {test_labelled_dir_graph_self_loop148,4874

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test27LabUnDirGraphStEph.rs,1332
pub mod TestLabUnDirGraphStEph {TestLabUnDirGraphStEph3,93
    fn test_labelled_undir_graph_empty() {test_labelled_undir_graph_empty12,444
    fn test_labelled_undir_graph_add_vertex() {test_labelled_undir_graph_add_vertex20,744
    fn test_labelled_undir_graph_add_labeled_edge() {test_labelled_undir_graph_add_labeled_edge32,1135
    fn test_labelled_undir_graph_neighbors() {test_labelled_undir_graph_neighbors53,2053
    fn test_labelled_undir_graph_edges() {test_labelled_undir_graph_edges76,2871
    fn test_labelled_undir_graph_macro_empty() {test_labelled_undir_graph_macro_empty90,3388
    fn test_labelled_undir_graph_macro_with_data() {test_labelled_undir_graph_macro_with_data97,3622
    fn test_labelled_undir_graph_edge_normalization() {test_labelled_undir_graph_edge_normalization114,4217
    fn test_labelled_undir_graph_different_label_types() {test_labelled_undir_graph_different_label_types128,4734
    fn test_labelled_undir_graph_display() {test_labelled_undir_graph_display147,5381
    fn test_labelled_undir_graph_debug() {test_labelled_undir_graph_debug159,5741
    fn test_labelled_undir_graph_self_loop() {test_labelled_undir_graph_self_loop171,6107
    fn test_labelled_undir_graph_multiple_edges_same_vertices() {test_labelled_undir_graph_multiple_edges_same_vertices184,6534

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test25UnDirGraphStEph.rs,141
pub mod TestUnDirGraphStEph {TestUnDirGraphStEph2,92
    fn test_undigraph_vertices_and_edges() {test_undigraph_vertices_and_edges10,394

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap03/TestInsertionSortSt.rs,784
pub mod TestInsertionSortSt {TestInsertionSortSt3,93
fn sort_and_assert(mut data: Vec<i32>, expected: &[i32]) {sort_and_assert6,186
fn insertion_sort_handles_empty() {insertion_sort_handles_empty12,341
fn insertion_sort_single_element() { sort_and_assert(vec![42], &[42]); }insertion_sort_single_element19,512
fn insertion_sort_already_sorted() { sort_and_assert(vec![1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]); }insertion_sort_already_sorted22,594
fn insertion_sort_reverse_order() { sort_and_assert(vec![5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]); }insertion_sort_reverse_order25,698
fn insertion_sort_with_duplicates() { sort_and_assert(vec![3, 1, 2, 3, 1], &[1, 1, 2, 3, 3]); }insertion_sort_with_duplicates28,801
fn insertion_sort_random_slice() {insertion_sort_random_slice31,906

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test29Algorithm_21_1.rs,489
pub mod Test29Algorithm_21_1 {Test29Algorithm_21_14,176
fn points2d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, N>> {points2d_tab_flat11,463
fn test_points2d_n3_example() {test_points2d_n3_example26,1043
fn test_points2d_n1_empty() {test_points2d_n1_empty40,1347
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values46,1453
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order54,1652
fn test_points2d_debug_shape() {test_points2d_debug_shape75,2155

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test38Problem21_1.rs,482
pub mod Test38Problem21_1 {Test38Problem21_14,167
fn points2d(n: N) -> ArraySeqStPerS<Pair<N, N>> {points2d10,401
fn test_points2d_n3_example() {test_points2d_n3_example25,742
fn test_points2d_n1_empty() {test_points2d_n1_empty40,1058
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values46,1155
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order54,1337
fn test_points2d_debug_display_shapes() {test_points2d_debug_display_shapes75,1783

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test30Algorithm_21_2.rs,588
pub mod Test30Algorithm_21_2 {Test30Algorithm_21_24,172
fn points3d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {points3d_tab_flat11,516
fn test_points3d_tab_flat_n0_empty() {test_points3d_tab_flat_n0_empty40,1894
fn test_points3d_tab_flat_n1_single() {test_points3d_tab_flat_n1_single46,2009
fn test_points3d_tab_flat_n2_values_and_order() {test_points3d_tab_flat_n2_values_and_order53,2191
fn test_points3d_tab_flat_iterator_order() {test_points3d_tab_flat_iterator_order70,2632
fn test_points3d_tab_flat_debug_shape() {test_points3d_tab_flat_debug_shape89,3126

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test34Exercise_21_8_and_Algorithm_21_5.rs,443
pub mod Test34Exercise_21_8_and_Algorithm_21_5 {Test34Exercise_21_8_and_Algorithm_21_53,93
fn is_divisible(n: N, i: N) -> B {is_divisible9,314
fn is_prime(n: N) -> B {is_prime20,585
fn primes_bf(n: N) -> ArraySeqStPerS<N> {primes_bf38,1214
fn test_is_prime_small_values() {test_is_prime_small_values48,1580
fn test_primes_bf_small() {test_primes_bf_small58,1857
fn test_primes_bf_debug_shape() {test_primes_bf_debug_shape65,2009

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Exercise_21_2.txt,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test33Exercise_21_7.rs,440
pub mod Test33Exercise_21_7 {Test33Exercise_21_74,190
fn is_even(x: &N) -> B {is_even8,313
fn is_vowel(c: &char) -> B {is_vowel15,413
fn pair_even_with_vowels(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<char>) -> ArraySeqStPerS<Pairpair_even_with_vowels24,702
fn test_pair_even_with_vowels_basic() {test_pair_even_with_vowels_basic43,1579
fn test_pair_even_with_vowels_debug_shape() {test_pair_even_with_vowels_debug_shape52,1920

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test37Problem_21_4.rs,1292
pub mod Test37Problem_21_4 {Test37Problem_21_44,137
fn cartesian_loops(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<&'static str>) -> ArraySeqStPerS<Pacartesian_loops10,372
    let mut v: Vec<Pair<N, &'static str>> = Vec::with_capacity(a.length() * b.length());str11,491
fn cartesian_tab_flat(cartesian_tab_flat22,874
    let nested: ArraySeqStPerS<ArraySeqStPerS<Pair<N, &'static str>>> =str26,1007
        <ArraySeqStPerS<ArraySeqStPerS<Pair<N, &'static str>>> as ArraySeqStPerTrait<str27,1079
            ArraySeqStPerS<Pair<N, &'static str>>,str28,1165
                <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static ststr31,1257
                <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static ststr31,1257
    <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::flattestr38,1522
    <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::flattestr38,1522
fn test_cartesian_loops_basic() {test_cartesian_loops_basic42,1640
fn test_cartesian_tab_flat_basic() {test_cartesian_tab_flat_basic58,2064
fn test_cartesian_iterator_order() {test_cartesian_iterator_order74,2494
fn test_cartesian_debug_shape() {test_cartesian_debug_shape83,2828

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test32Exercise_21_5_and_21_6.rs,463
pub mod Test32Exercise_21_5_and_21_6 {Test32Exercise_21_5_and_21_64,169
fn all_contiguous_subseqs<T: StT>(a: &ArraySeqStPerS<T>) -> ArraySeqStPerS<ArraySeqStPerS<T>> {all_contiguous_subseqs11,461
fn test_all_contiguous_subseqs_n0() {test_all_contiguous_subseqs_n030,1229
fn test_all_contiguous_subseqs_n3_values() {test_all_contiguous_subseqs_n3_values37,1418
fn test_all_contiguous_subseqs_debug_shape() {test_all_contiguous_subseqs_debug_shape46,1769

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test36Problem_21_3.rs,546
pub mod Test36Problem_21_3 {Test36Problem_21_34,171
fn points3d_loops(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {points3d_loops10,457
fn test_points3d_loops_n0_empty() {test_points3d_loops_n0_empty27,881
fn test_points3d_loops_n1_single() {test_points3d_loops_n1_single33,990
fn test_points3d_loops_n2_values_and_order() {test_points3d_loops_n2_values_and_order40,1166
fn test_points3d_loops_iterator_order() {test_points3d_loops_iterator_order57,1601
fn test_points3d_loops_debug_shape() {test_points3d_loops_debug_shape76,2089

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test31Algorithm_21_6.rs,322
pub mod Test31Algorithm_21_6 {Test31Algorithm_21_64,157
fn prime_sieve(n: N) -> ArraySeqStPerS<N> {prime_sieve11,493
fn test_prime_sieve_small() {test_prime_sieve_small59,2472
fn test_prime_sieve_n2_empty() {test_prime_sieve_n2_empty65,2605
fn test_prime_sieve_debug_shape() {test_prime_sieve_debug_shape71,2708

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap39/Test53BSTParaTreapMtEph.rs,675
pub mod Test53BSTParaTreapMtEph {Test53BSTParaTreapMtEph3,93
fn make_tree(values: &[i32]) -> ParamTreap<i32> {make_tree8,286
fn make_range_tree(start: i32, end: i32) -> ParamTreap<i32> {make_range_tree16,443
fn treap_basic_insert_find() {treap_basic_insert_find25,623
fn treap_split_join_pair() {treap_split_join_pair35,953
fn treap_union_intersect_difference() {treap_union_intersect_difference47,1396
fn treap_filter_reduce() {treap_filter_reduce65,2157
fn treap_join_mid_roundtrip() {treap_join_mid_roundtrip78,2558
fn treap_invariants_priority_heap() {treap_invariants_priority_heap97,3152
    fn check_heap(tree: &ParamTreap<i32>) {check_heap98,3190

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap39/Test48BSTTreapStEph.rs,222
pub mod Test48BSTTreapStEph {Test48BSTTreapStEph3,93
fn treap_insert_find_stays_balanced() {treap_insert_find_stays_balanced8,210
fn treap_duplicate_insert_is_idempotent() {treap_duplicate_insert_is_idempotent28,837

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test35Exercsise_21_9.rs,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap12/TestExercise12_5.rs,366
pub mod TestExercise12_5 {TestExercise12_53,93
fn push_pop_lifo_single_thread() {push_pop_lifo_single_thread10,315
fn pop_on_empty_returns_none() {pop_on_empty_returns_none24,653
fn multi_thread_push_collects_all_items() {multi_thread_push_collects_all_items30,801
fn multi_thread_pop_consumes_all_elements() {multi_thread_pop_consumes_all_elements58,1620

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap12/TestExercise12_2.rs,289
pub mod TestExercise12_2 {TestExercise12_23,93
fn fetch_add_cas_returns_previous_value() {fetch_add_cas_returns_previous_value11,315
fn trait_impl_matches_free_function() {trait_impl_matches_free_function18,505
fn fetch_add_cas_is_thread_safe() {fetch_add_cas_is_thread_safe28,791

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap12/TestExercise12_1.rs,398
pub mod TestExercise12_1 {TestExercise12_13,93
fn spin_lock_excludes_parallel_threads() {spin_lock_excludes_parallel_threads12,324
fn spin_lock_with_lock_helper_executes_body() {spin_lock_with_lock_helper_executes_body40,1142
fn parallel_increment_counts_all_iterations() {parallel_increment_counts_all_iterations48,1375
fn spin_lock_is_non_reentrant() {spin_lock_is_non_reentrant53,1484

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap36/Test39Chapter36St.rs,665
pub mod Test39Chapter36St {Test39Chapter36St3,93
trait ToVec<T: StT> {ToVec9,303
    fn to_vec(&self) -> Vec<T>;to_vec10,325
impl<T: StT> ToVec<T> for ArraySeqStEphS<T> {ArraySeqStEphS12,359
    fn to_vec(&self) -> Vec<T> { (0..self.length()).map(|i| self.nth(i).clone()).collect() }to_vec13,405
fn quick_sort_variants_produce_sorted_output() {quick_sort_variants_produce_sorted_output17,509
fn quick_sort_handles_edge_cases() {quick_sort_handles_edge_cases33,1022
fn pivot_strategies_match_expectations() {pivot_strategies_match_expectations56,1811
fn quick_sort_small_inputs_use_shared_pivots() {quick_sort_small_inputs_use_shared_pivots81,2622

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap36/Test44Chapter36MtSlice.rs,847
pub mod Test44Chapter36MtSlice {Test44Chapter36MtSlice3,93
fn to_vec<T: StT + Send + Sync>(a: &ArraySeqMtEphSliceS<T>) -> Vec<T> { a.to_vec() }to_vec11,398
fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }is_sorted13,484
fn mk_seq(data: &[i32]) -> ArraySeqMtEphSliceS<i32> { ArraySeqMtEphSliceS::from_vec(data.to_vec(mk_seq15,578
fn quick_sort_slice_variants_produce_sorted_output() {quick_sort_slice_variants_produce_sorted_output18,688
fn quick_sort_slice_edge_cases() {quick_sort_slice_edge_cases36,1375
fn quick_sort_slice_large_inputs() {quick_sort_slice_large_inputs59,2383
fn slice_pivot_strategies_match_expectations() {slice_pivot_strategies_match_expectations72,3000
fn quick_sort_slice_small_inputs_use_shared_pivots() {quick_sort_slice_small_inputs_use_shared_pivots92,3625

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap36/Test40Chapter36Mt.rs,708
pub mod Test40Chapter36Mt {Test40Chapter36Mt3,93
fn to_vec<T: StT>(a: &ArraySeqMtEphS<T>) -> Vec<T> { (0..a.length()).map(|i| a.nth_cloned(i)).coto_vec10,310
fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }is_sorted12,417
fn quick_sort_mt_variants_produce_sorted_output() {quick_sort_mt_variants_produce_sorted_output15,519
fn quick_sort_mt_edge_cases() {quick_sort_mt_edge_cases33,1194
fn pivot_mt_strategies_match_expectations() {pivot_mt_strategies_match_expectations56,2238
fn quick_sort_mt_large_inputs() {quick_sort_mt_large_inputs77,2981
fn quick_sort_mt_small_inputs_use_shared_pivots() {quick_sort_mt_small_inputs_use_shared_pivots90,3565

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test23MappingStEph.rs,563
pub mod Test23MappingStEphChap5_5 {Test23MappingStEphChap5_54,147
    fn test_empty_mapping() {test_empty_mapping13,457
    fn test_from_vec_basic() {test_from_vec_basic21,671
    fn test_from_vec_duplicate_keys() {test_from_vec_duplicate_keys33,1171
    fn test_from_relation() {test_from_relation44,1722
    fn test_domain_and_range() {test_domain_and_range58,2494
    fn test_iter() {test_iter77,3202
    fn test_mem_comprehensive() {test_mem_comprehensive90,3679
    fn test_empty_mapping_operations() {test_empty_mapping_operations109,4341

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test22RelationStEph.rs,153
pub mod TestRelationStEphChap5_2 {TestRelationStEphChap5_23,93
    fn test_relation_domain_range_and_mem() {test_relation_domain_range_and_mem12,362

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test21SetStEph.rs,578
pub mod TestSetStEphChap5_1 {TestSetStEphChap5_13,93
    fn macro_typecheck_exercise() {macro_typecheck_exercise10,281
        let _empty: Set<&'static str> = SetLit![];str11,317
    fn test_cartesian_product_example_5_1() {test_cartesian_product_example_5_117,467
    fn test_partition_example_5_2_true() {test_partition_example_5_2_true37,1037
    fn test_partition_example_5_2_false_due_to_overlap() {test_partition_example_5_2_false_due_to_overlap46,1334
    fn test_partition_false_due_to_missing_element() {test_partition_false_due_to_missing_element55,1679

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap23/TestPrimTreeSeqSt.rs,495
pub mod TestPrimTreeSeqSt {TestPrimTreeSeqSt2,92
    fn expose_zero_returns_zero() {expose_zero_returns_zero7,225
    fn expose_one_returns_one() {expose_one_returns_one13,408
    fn expose_two_splits_sequence() {expose_two_splits_sequence22,685
    fn join_zero_creates_empty_sequence() {join_zero_creates_empty_sequence37,1247
    fn join_two_concatenates_sequences() {join_two_concatenates_sequences43,1431
    fn expose_then_join_roundtrip() {expose_then_join_roundtrip52,1833

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test08LinkedListStEph19.rs,615
pub mod TestLinkedListStEph {TestLinkedListStEph2,92
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list6,219
    fn test_eph_set_and_nth() {test_eph_set_and_nth14,484
    fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug21,679
    fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1931,1064
    fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1937,1246
    fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1949,1772

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test01ArraySeqMacro.rs,726
pub mod TestArraySeqMacro {TestArraySeqMacro3,93
fn arrayseqs_empty_macro() {arrayseqs_empty_macro10,282
fn arrayseqs_literal_macro_keeps_order() {arrayseqs_literal_macro_keeps_order17,509
fn arrayseqs_repeat_macro_clones_element() {arrayseqs_repeat_macro_clones_element25,807
fn arrayseq_tabulate_and_map_work() {arrayseq_tabulate_and_map_work35,1162
fn arrayseq_subseq_append_filter_flatten() {arrayseq_subseq_append_filter_flatten46,1654
fn arrayseq_update_and_inject_preserve_original() {arrayseq_update_and_inject_preserve_original72,2888
fn arrayseq_collect_iterate_reduce_scan() {arrayseq_collect_iterate_reduce_scan87,3606
    fn pair_cmp(lhs: &&str, rhs: &&str) -> O { lhs.cmp(rhs) }pair_cmp89,3720

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test12ArraySeqStEphMacro.rs,282
pub mod TestArraySeqStEphMacro {TestArraySeqStEphMacro3,93
fn arrayseq_steph_basic_macros() {arrayseq_steph_basic_macros10,344
fn arrayseq_steph_full_pipeline() {arrayseq_steph_full_pipeline19,702
    fn pair_cmp(lhs: &&str, rhs: &&str) -> O { lhs.cmp(rhs) }pair_cmp46,2262

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test10ArraySeqStPerChap18.rs,946
pub mod TestArraySeqStPerChap {TestArraySeqStPerChap3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci10,325
        fn fib(n: N) -> N {fib11,360
    fn test_map_increment() {test_map_increment36,1054
    fn test_subseq() {test_subseq43,1323
    fn test_append() {test_append54,1705
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append62,2016
    fn test_filter_even() {test_filter_even75,2717
    fn test_flatten() {test_flatten90,3452
    fn test_update_sequence() {test_update_sequence104,4378
    fn test_inject_and_ninject() {test_inject_and_ninject114,5007
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan139,6484
    fn test_iterate_sum_basic() {test_iterate_sum_basic158,7482
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum166,7767
    fn test_collect_groups_by_key() {test_collect_groups_by_key178,8230

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test13ArraySeqStEphChap18.rs,1049
pub mod TestArraySeqStEphChap {TestArraySeqStEphChap3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci9,288
        fn fib(n: N) -> N {fib10,323
    fn test_map_increment() {test_map_increment35,1028
    fn test_subseq() {test_subseq42,1293
    fn test_append() {test_append53,1804
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append61,2111
    fn test_filter_even() {test_filter_even74,2838
    fn test_flatten() {test_flatten88,3478
    fn test_update_sequence() {test_update_sequence102,4307
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins114,4870
    fn test_inject_conflicts_last_wins_2() {test_inject_conflicts_last_wins_2127,5702
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan143,6522
    fn test_iterate_sum_basic() {test_iterate_sum_basic162,7550
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum170,7835
    fn test_collect_groups_by_key() {test_collect_groups_by_key183,8310

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test28ArraySeqMtEphChap18.rs,1095
pub mod Test28ArraySeqMtEphChap {Test28ArraySeqMtEphChap4,163
    fn identity(i: N) -> N { i }identity9,334
    fn double(i: N) -> N { i * 2 }double10,367
    fn add_one(i: N) -> N { i + 1 }add_one11,402
    fn add_ten(i: N) -> N { i + 10 }add_ten12,438
    fn multiply_by_two(x: &N) -> N { x * 2 }multiply_by_two13,475
    fn add_nums(x: &N, y: &N) -> N { x + y }add_nums14,520
    fn is_even_bool(x: &N) -> B { if x % 2 == 0 { B::True } else { B::False } }is_even_bool15,565
    fn test_new_and_set() {test_new_and_set18,658
    fn test_tabulate_basic() {test_tabulate_basic32,1068
    fn test_map_parallel() {test_map_parallel42,1347
    fn test_reduce_parallel() {test_reduce_parallel53,1790
    fn test_filter() {test_filter62,2161
    fn test_append() {test_append73,2553
    fn test_flatten() {test_flatten87,3177
    fn test_scan() {test_scan102,3889
    fn test_iterate() {test_iterate116,4536
    fn test_update() {test_update125,4928
    fn test_inject() {test_inject135,5337
    fn test_empty_and_singleton() {test_empty_and_singleton152,6081

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test05LinkedListStPer19.rs,383
pub mod TestLinkedListPer {TestLinkedListPer2,92
    fn test_select() {test_select7,229
    fn test_append_variants() {test_append_variants20,753
    fn test_map() {test_map29,1074
    fn test_iterate_and_reduce() {test_iterate_and_reduce36,1325
    fn test_scan() {test_scan45,1695
    fn test_flatten() {test_flatten53,2007
    fn test_inject() {test_inject60,2338

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test02MathSeq.rs,1347
pub mod TestMathSeq {TestMathSeq3,93
    fn test_length_and_nth_basic() {test_length_and_nth_basic10,272
    fn test_add_last_and_delete_last() {test_add_last_and_delete_last18,492
    fn test_new_empty_singleton_and_predicates() {test_new_empty_singleton_and_predicates31,941
    fn test_set_in_bounds_and_out_of_bounds() {test_set_in_bounds_and_out_of_bounds50,1539
    fn test_subseq_view_bounds() {test_subseq_view_bounds61,1898
    fn test_subseq_copy_bounds() {test_subseq_copy_bounds74,2367
    fn test_domain() {test_domain84,2663
    fn test_range_deduplicates_preserving_order() {test_range_deduplicates_preserving_order90,2804
    fn test_debug_format_for_mathseq() {test_debug_format_for_mathseq97,3032
    fn test_display_format_for_mathseq() {test_display_format_for_mathseq103,3197
    fn test_multiset_range_counts_first_occurrence_order() {test_multiset_range_counts_first_occurrence_order109,3362
    fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics117,3612
    fn test_range_empty_returns_empty() {test_range_empty_returns_empty123,3742
    fn test_multiset_range_empty_returns_empty() {test_multiset_range_empty_returns_empty130,3915
    fn test_domain_empty_is_empty() {test_domain_empty_is_empty137,4111
    fn test_iter_collect_and_sum() {test_iter_collect_and_sum144,4273

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test03LinkedListStPer.rs,539
pub mod TestLinkedListPer {TestLinkedListPer3,93
    fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates9,292
    fn test_new_and_nth_set() {test_new_and_nth_set18,681
    fn test_subseq_copy() {test_subseq_copy30,1116
    fn test_from_vec_and_debug_format() {test_from_vec_and_debug_format39,1367
    fn test_iter_inorder_collect() {test_iter_inorder_collect46,1575
    fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics54,1829
    fn test_display_impl() {test_display_impl60,1971

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test04LinkedListStPer.rs,569
pub mod TestLinkedListStPer {TestLinkedListStPer2,92
    fn test_tabulate() {test_tabulate7,231
    fn test_map() {test_map14,435
    fn test_filter() {test_filter22,720
    fn test_append() {test_append35,1132
    fn test_update() {test_update43,1473
    fn test_inject() {test_inject50,1746
    fn test_ninject() {test_ninject58,2097
    fn test_iterate() {test_iterate66,2452
    fn test_reduce() {test_reduce73,2691
    fn test_scan() {test_scan80,2924
    fn test_flatten() {test_flatten88,3248
    fn test_collect() {test_collect99,3672

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test27ArraySeqMtPerChap18.rs,1134
pub mod Test27ArraySeqMtPerChap {Test27ArraySeqMtPerChap4,153
    fn identity(i: N) -> N { i }identity10,382
    fn double(i: N) -> N { i * 2 }double11,415
    fn square(i: N) -> N { i * i }square12,450
    fn add_100(i: N) -> N { i + 100 }add_10013,485
    fn const_42(_i: N) -> N { 42 }const_4214,523
    fn format_item(i: N) -> String { format!("item_{}", i) }format_item15,558
    fn is_even_bool(i: N) -> B { if i % 2 == 0 { B::True } else { B::False } }is_even_bool16,619
    fn assert_set_eq<T: PartialEq + std::fmt::Debug>(actual: &[T], expected: &[T]) {assert_set_eq19,750
    fn test_tabulate_basic() {test_tabulate_basic30,1159
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci41,1558
        fn fib(n: N) -> N {fib42,1593
    fn test_tabulate_empty() {test_tabulate_empty69,2396
    fn test_tabulate_single() {test_tabulate_single76,2608
    fn test_tabulate_string() {test_tabulate_string83,2819
    fn test_tabulate_boolean() {test_tabulate_boolean95,3390
    fn test_tabulate_squares() {test_tabulate_squares106,3840
    fn test_tabulate_large() {test_tabulate_large117,4232

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test06LinkedListStEph.rs,547
pub mod TestLinkedListEph {TestLinkedListEph4,149
    fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates9,286
    fn test_new_and_nth_set() {test_new_and_nth_set18,675
    fn test_subseq_copy() {test_subseq_copy27,926
    fn test_linkedlisteph_basic() {test_linkedlisteph_basic36,1177
    fn test_debug_format_for_eph() {test_debug_format_for_eph45,1433
    fn test_display_format_for_eph() {test_display_format_for_eph51,1601
    fn test_iter_inorder_collect_eph() {test_iter_inorder_collect_eph57,1769

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test07LinkedListStEph.rs,928
pub mod TestLinkedListStEph {TestLinkedListStEph3,93
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list7,220
    fn test_construct_eph_from_vec() {test_construct_eph_from_vec17,593
    fn test_eph_is_empty_and_singleton() {test_eph_is_empty_and_singleton23,766
    fn test_eph_set_and_subseq_copy() {test_eph_set_and_subseq_copy31,1117
    fn test_iter_inorder_collect_eph_ch18() {test_iter_inorder_collect_eph_ch1840,1396
    fn test_tabulate_and_map_ch18() {test_tabulate_and_map_ch1846,1578
    fn test_append_ch18() {test_append_ch1853,1894
    fn test_filter_ch18() {test_filter_ch1861,2238
    fn test_update_ch18() {test_update_ch1878,2722
    fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1885,3002
    fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1895,3521
    fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch18107,4104

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test11ArraySeqStPerMacro.rs,415
pub mod TestArraySeqStPerMacro {TestArraySeqStPerMacro3,93
fn arrayseq_stper_macro_empty() {arrayseq_stper_macro_empty11,440
fn arrayseq_stper_macro_literal() {arrayseq_stper_macro_literal18,661
fn arrayseq_stper_macro_repeat() {arrayseq_stper_macro_repeat25,884
fn arrayseq_stper_operations() {arrayseq_stper_operations34,1244
    fn str_cmp(lhs: &&str, rhs: &&str) -> O { lhs.cmp(rhs) }str_cmp62,2964

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test51BSTSetMtEph.rs,11099
pub mod Test51BSTSetMtEph {Test51BSTSetMtEph3,93
trait TestSet: Sized {TestSet8,278
    fn empty() -> Self;empty9,301
    fn insert(&mut self, value: i32);insert10,325
    fn delete(&mut self, value: &i32);delete11,363
    fn size(&self) -> usize;size12,402
    fn is_empty(&self) -> B;is_empty13,431
    fn contains(&self, value: &i32) -> B;contains14,460
    fn minimum(&self) -> Option<i32>;minimum15,502
    fn maximum(&self) -> Option<i32>;maximum16,540
    fn union(&self, other: &Self) -> Self;union17,578
    fn intersection(&self, other: &Self) -> Self;intersection18,621
    fn difference(&self, other: &Self) -> Self;difference19,671
    fn split(&self, pivot: &i32) -> (Self, B, Self);split20,719
    fn join_pair(left: Self, right: Self) -> Self;join_pair21,772
    fn join_m(left: Self, pivot: i32, right: Self) -> Self;join_m22,823
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self;filter23,883
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32;reduce24,951
    fn iter_seq(&self) -> ArraySeqStPerS<i32>;iter_seq25,1025
impl TestSet for apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32> {BSTSetPlainMt28,1075
    fn empty() -> Self { Self::empty() }empty29,1166
    fn insert(&mut self, value: i32) { self.insert(value); }insert31,1208
    fn delete(&mut self, value: &i32) { self.delete(value); }delete33,1270
    fn size(&self) -> usize { self.size() }size35,1333
    fn is_empty(&self) -> B { self.is_empty() }is_empty37,1378
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains39,1427
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum41,1494
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum43,1551
    fn union(&self, other: &Self) -> Self { self.union(other) }union45,1608
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection47,1673
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference49,1752
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split51,1827
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair53,1902
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m55,1986
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter57,2083
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce59,2178
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq61,2278
impl TestSet for apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32> {BSTSetAVLMt64,2352
    fn empty() -> Self { Self::empty() }empty65,2437
    fn insert(&mut self, value: i32) { self.insert(value); }insert67,2479
    fn delete(&mut self, value: &i32) { self.delete(value); }delete69,2541
    fn size(&self) -> usize { self.size() }size71,2604
    fn is_empty(&self) -> B { self.is_empty() }is_empty73,2649
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains75,2698
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum77,2765
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum79,2822
    fn union(&self, other: &Self) -> Self { self.union(other) }union81,2879
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection83,2944
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference85,3023
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split87,3098
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair89,3173
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m91,3257
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter93,3354
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce95,3449
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq97,3549
impl TestSet for apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32> {BSTSetRBMt100,3623
    fn empty() -> Self { Self::empty() }empty101,3705
    fn insert(&mut self, value: i32) { self.insert(value); }insert103,3747
    fn delete(&mut self, value: &i32) { self.delete(value); }delete105,3809
    fn size(&self) -> usize { self.size() }size107,3872
    fn is_empty(&self) -> B { self.is_empty() }is_empty109,3917
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains111,3966
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum113,4033
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum115,4090
    fn union(&self, other: &Self) -> Self { self.union(other) }union117,4147
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection119,4212
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference121,4291
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split123,4366
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair125,4441
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m127,4525
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter129,4622
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce131,4717
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq133,4817
impl TestSet for apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32> {BSTSetBBAlphaMt136,4891
    fn empty() -> Self { Self::empty() }empty137,4988
    fn insert(&mut self, value: i32) { self.insert(value); }insert139,5030
    fn delete(&mut self, value: &i32) { self.delete(value); }delete141,5092
    fn size(&self) -> usize { self.size() }size143,5155
    fn is_empty(&self) -> B { self.is_empty() }is_empty145,5200
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains147,5249
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum149,5316
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum151,5373
    fn union(&self, other: &Self) -> Self { self.union(other) }union153,5430
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection155,5495
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference157,5574
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split159,5649
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair161,5724
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m163,5808
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter165,5905
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce167,6000
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq169,6100
impl TestSet for apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32> {BSTSetTreapMt172,6174
    fn empty() -> Self { Self::empty() }empty173,6265
    fn insert(&mut self, value: i32) { self.insert(value); }insert175,6307
    fn delete(&mut self, value: &i32) { self.delete(value); }delete177,6369
    fn size(&self) -> usize { self.size() }size179,6432
    fn is_empty(&self) -> B { self.is_empty() }is_empty181,6477
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains183,6526
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum185,6593
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum187,6650
    fn union(&self, other: &Self) -> Self { self.union(other) }union189,6707
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection191,6772
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference193,6851
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split195,6926
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair197,7001
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m199,7085
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter201,7182
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce203,7277
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq205,7377
impl TestSet for apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32> {BSTSetSplayMt208,7451
    fn empty() -> Self { Self::empty() }empty209,7542
    fn insert(&mut self, value: i32) { self.insert(value); }insert211,7584
    fn delete(&mut self, value: &i32) { self.delete(value); }delete213,7646
    fn size(&self) -> usize { self.size() }size215,7709
    fn is_empty(&self) -> B { self.is_empty() }is_empty217,7754
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains219,7803
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum221,7870
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum223,7927
    fn union(&self, other: &Self) -> Self { self.union(other) }union225,7984
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection227,8049
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference229,8128
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split231,8203
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair233,8278
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m235,8362
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter237,8459
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce239,8554
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq241,8654
fn exercise_set<S: TestSet>() {exercise_set244,8728
fn test_plain_bst_set_ops() {test_plain_bst_set_ops304,10498
fn test_avl_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTStest_avl_bst_set_ops309,10634
fn test_rb_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRtest_rb_bst_set_ops312,10758
fn test_bbalpha_bst_set_ops() {test_bbalpha_bst_set_ops315,10878
fn test_treap_bst_set_ops() { exercise_set::<apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEphtest_treap_bst_set_ops320,11022
fn test_splay_bst_set_ops() {test_splay_bst_set_ops323,11154

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test18AVLTreeSeqStEph.rs,118
pub mod TestAVLTreeSeqEph {TestAVLTreeSeqEph2,92
    fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic8,342

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test47BSTBBAlphaStEph.rs,218
pub mod TestBSTBBAlphaStEph {TestBSTBBAlphaStEph3,93
fn bbalpha_insert_find_balances() {bbalpha_insert_find_balances8,214
fn bbalpha_duplicate_insert_is_idempotent() {bbalpha_duplicate_insert_is_idempotent28,834

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test19AVLTreeSeqStEph18.rs,281
pub mod TestAVLTreeSeqStEph {TestAVLTreeSeqStEph4,171
    fn test_tabulate_inorder() {test_tabulate_inorder11,405
    fn test_map_increment() {test_map_increment17,614
    fn test_append_union() {test_append_union25,972
    fn test_filter_even() {test_filter_even39,1478

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test46BSTRBStEph.rs,196
pub mod Test46BSTRBStEph {Test46BSTRBStEph3,93
fn rb_insert_find_and_bounds() {rb_insert_find_and_bounds9,235
fn rb_duplicate_insert_is_idempotent() {rb_duplicate_insert_is_idempotent29,853

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test15AVLTreeSeqStPer.rs,216
pub mod TestAVLTreeSeqPer {TestAVLTreeSeqPer2,92
    fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate8,266
    fn test_iterator_inorder_values() {test_iterator_inorder_values17,595

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test45BSTAVLStEph.rs,202
pub mod Test45BSTAVLStEph {Test45BSTAVLStEph3,93
fn avl_insert_find_and_bounds() {avl_insert_find_and_bounds9,238
fn avl_duplicate_insert_is_idempotent() {avl_duplicate_insert_is_idempotent31,945

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test49BSTSplayStEph.rs,200
pub mod Test49BSTSplayStEph {Test49BSTSplayStEph3,93
fn splay_basic_behaviour() {splay_basic_behaviour9,244
fn splay_duplicate_insert_is_idempotent() {splay_duplicate_insert_is_idempotent28,816

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test20AVLTreeSeqStEph.rs,249
pub mod TestAVLTreeSeqStEph {TestAVLTreeSeqStEph4,172
    fn test_tabulate_and_map() {test_tabulate_and_map11,406
    fn test_select_and_append() {test_select_and_append19,757
    fn test_deflate_and_filter() {test_deflate_and_filter49,1827

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test50BSTMtEph.rs,355
pub mod Test50BSTMtEph {Test50BSTMtEph3,93
fn mt_plain_basic_ops() {mt_plain_basic_ops14,523
fn mt_avl_basic_ops() {mt_avl_basic_ops27,864
fn mt_rb_basic_ops() {mt_rb_basic_ops38,1131
fn mt_bbalpha_basic_ops() {mt_bbalpha_basic_ops48,1332
fn mt_treap_basic_ops() {mt_treap_basic_ops58,1545
fn mt_splay_basic_ops() {mt_splay_basic_ops68,1756

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test16AVLTreeSeqStPer18.rs,282
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer4,141
    fn test_tabulate_inorder() {test_tabulate_inorder10,338
    fn test_map_increment() {test_map_increment17,626
    fn test_append_union() {test_append_union26,1133
    fn test_filter_even() {test_filter_even42,1902

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaMtEph.rs,836
pub mod Test52BSTParaMtEph {Test52BSTParaMtEph3,93
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree8,271
fn make_range_tree(start: i32, end: i32) -> ParamBST<i32> {make_range_tree16,424
fn para_basic_insert_find() {para_basic_insert_find25,600
fn para_split_and_join_pair() {para_split_and_join_pair35,929
fn para_union_and_delete() {para_union_and_delete47,1381
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip64,1852
fn para_intersect_and_difference() {para_intersect_and_difference86,2518
fn para_filter_and_reduce() {para_filter_and_reduce98,2894
fn para_union_large_balanced() {para_union_large_balanced112,3272
fn para_intersect_and_difference_large() {para_intersect_and_difference_large123,3572
fn para_filter_and_reduce_edge_cases() {para_filter_and_reduce_edge_cases139,4175

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test17AVLTreeSeqStPer19.rs,55
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer3,93

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaStEph.rs,377
pub mod Test52BSTParaStEph {Test52BSTParaStEph3,93
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree8,271
fn para_basic_insert_find() {para_basic_insert_find17,432
fn para_split_and_join_pair() {para_split_and_join_pair27,761
fn para_union_and_delete() {para_union_and_delete39,1213
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip56,1684

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap03/TestInsertionSortSt.rs,784
pub mod TestInsertionSortSt {TestInsertionSortSt3,93
fn sort_and_assert(mut data: Vec<i32>, expected: &[i32]) {sort_and_assert6,186
fn insertion_sort_handles_empty() {insertion_sort_handles_empty12,341
fn insertion_sort_single_element() { sort_and_assert(vec![42], &[42]); }insertion_sort_single_element19,512
fn insertion_sort_already_sorted() { sort_and_assert(vec![1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]); }insertion_sort_already_sorted22,594
fn insertion_sort_reverse_order() { sort_and_assert(vec![5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]); }insertion_sort_reverse_order25,698
fn insertion_sort_with_duplicates() { sort_and_assert(vec![3, 1, 2, 3, 1], &[1, 1, 2, 3, 3]); }insertion_sort_with_duplicates28,801
fn insertion_sort_random_slice() {insertion_sort_random_slice31,906

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test23MappingStEph.rs,563
pub mod Test23MappingStEphChap5_5 {Test23MappingStEphChap5_54,147
    fn test_empty_mapping() {test_empty_mapping13,457
    fn test_from_vec_basic() {test_from_vec_basic21,671
    fn test_from_vec_duplicate_keys() {test_from_vec_duplicate_keys33,1171
    fn test_from_relation() {test_from_relation44,1722
    fn test_domain_and_range() {test_domain_and_range58,2494
    fn test_iter() {test_iter77,3202
    fn test_mem_comprehensive() {test_mem_comprehensive90,3679
    fn test_empty_mapping_operations() {test_empty_mapping_operations109,4341

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test22RelationStEph.rs,153
pub mod TestRelationStEphChap5_2 {TestRelationStEphChap5_23,93
    fn test_relation_domain_range_and_mem() {test_relation_domain_range_and_mem12,362

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test21SetStEph.rs,578
pub mod TestSetStEphChap5_1 {TestSetStEphChap5_13,93
    fn macro_typecheck_exercise() {macro_typecheck_exercise10,281
        let _empty: Set<&'static str> = SetLit![];str11,317
    fn test_cartesian_product_example_5_1() {test_cartesian_product_example_5_117,467
    fn test_partition_example_5_2_true() {test_partition_example_5_2_true37,1037
    fn test_partition_example_5_2_false_due_to_overlap() {test_partition_example_5_2_false_due_to_overlap46,1334
    fn test_partition_false_due_to_missing_element() {test_partition_false_due_to_missing_element55,1679

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test24DirGraphStEph.rs,131
pub mod TestDirGraphStEph {TestDirGraphStEph2,92
    fn test_digraph_vertices_and_arcs() {test_digraph_vertices_and_arcs10,388

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test26LabDirGraphStEph.rs,1040
pub mod TestLabDirGraphStEph {TestLabDirGraphStEph3,93
    fn test_labelled_dir_graph_empty() {test_labelled_dir_graph_empty12,436
    fn test_labelled_dir_graph_add_vertex() {test_labelled_dir_graph_add_vertex20,727
    fn test_labelled_dir_graph_add_labeled_arc() {test_labelled_dir_graph_add_labeled_arc32,1111
    fn test_labelled_dir_graph_neighbors() {test_labelled_dir_graph_neighbors49,1721
    fn test_labelled_dir_graph_arcs() {test_labelled_dir_graph_arcs73,2575
    fn test_labelled_dir_graph_macro_empty() {test_labelled_dir_graph_macro_empty85,2964
    fn test_labelled_dir_graph_macro_with_data() {test_labelled_dir_graph_macro_with_data92,3191
    fn test_labelled_dir_graph_different_label_types() {test_labelled_dir_graph_different_label_types107,3675
    fn test_labelled_dir_graph_display() {test_labelled_dir_graph_display124,4161
    fn test_labelled_dir_graph_debug() {test_labelled_dir_graph_debug136,4515
    fn test_labelled_dir_graph_self_loop() {test_labelled_dir_graph_self_loop148,4874

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test27LabUnDirGraphStEph.rs,1332
pub mod TestLabUnDirGraphStEph {TestLabUnDirGraphStEph3,93
    fn test_labelled_undir_graph_empty() {test_labelled_undir_graph_empty12,444
    fn test_labelled_undir_graph_add_vertex() {test_labelled_undir_graph_add_vertex20,744
    fn test_labelled_undir_graph_add_labeled_edge() {test_labelled_undir_graph_add_labeled_edge32,1135
    fn test_labelled_undir_graph_neighbors() {test_labelled_undir_graph_neighbors53,2053
    fn test_labelled_undir_graph_edges() {test_labelled_undir_graph_edges76,2871
    fn test_labelled_undir_graph_macro_empty() {test_labelled_undir_graph_macro_empty90,3388
    fn test_labelled_undir_graph_macro_with_data() {test_labelled_undir_graph_macro_with_data97,3622
    fn test_labelled_undir_graph_edge_normalization() {test_labelled_undir_graph_edge_normalization114,4217
    fn test_labelled_undir_graph_different_label_types() {test_labelled_undir_graph_different_label_types128,4734
    fn test_labelled_undir_graph_display() {test_labelled_undir_graph_display147,5381
    fn test_labelled_undir_graph_debug() {test_labelled_undir_graph_debug159,5741
    fn test_labelled_undir_graph_self_loop() {test_labelled_undir_graph_self_loop171,6107
    fn test_labelled_undir_graph_multiple_edges_same_vertices() {test_labelled_undir_graph_multiple_edges_same_vertices184,6534

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test25UnDirGraphStEph.rs,141
pub mod TestUnDirGraphStEph {TestUnDirGraphStEph2,92
    fn test_undigraph_vertices_and_edges() {test_undigraph_vertices_and_edges10,394

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap12/TestExercise12_5.rs,366
pub mod TestExercise12_5 {TestExercise12_53,93
fn push_pop_lifo_single_thread() {push_pop_lifo_single_thread10,315
fn pop_on_empty_returns_none() {pop_on_empty_returns_none24,653
fn multi_thread_push_collects_all_items() {multi_thread_push_collects_all_items30,801
fn multi_thread_pop_consumes_all_elements() {multi_thread_pop_consumes_all_elements58,1620

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap12/TestExercise12_2.rs,289
pub mod TestExercise12_2 {TestExercise12_23,93
fn fetch_add_cas_returns_previous_value() {fetch_add_cas_returns_previous_value11,315
fn trait_impl_matches_free_function() {trait_impl_matches_free_function18,505
fn fetch_add_cas_is_thread_safe() {fetch_add_cas_is_thread_safe28,791

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap12/TestExercise12_1.rs,398
pub mod TestExercise12_1 {TestExercise12_13,93
fn spin_lock_excludes_parallel_threads() {spin_lock_excludes_parallel_threads12,324
fn spin_lock_with_lock_helper_executes_body() {spin_lock_with_lock_helper_executes_body40,1142
fn parallel_increment_counts_all_iterations() {parallel_increment_counts_all_iterations48,1375
fn spin_lock_is_non_reentrant() {spin_lock_is_non_reentrant53,1484

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test08LinkedListStEph19.rs,615
pub mod TestLinkedListStEph {TestLinkedListStEph2,92
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list6,219
    fn test_eph_set_and_nth() {test_eph_set_and_nth14,484
    fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug21,679
    fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1931,1064
    fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1937,1246
    fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1949,1772

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test01ArraySeqMacro.rs,726
pub mod TestArraySeqMacro {TestArraySeqMacro3,93
fn arrayseqs_empty_macro() {arrayseqs_empty_macro10,282
fn arrayseqs_literal_macro_keeps_order() {arrayseqs_literal_macro_keeps_order17,509
fn arrayseqs_repeat_macro_clones_element() {arrayseqs_repeat_macro_clones_element25,807
fn arrayseq_tabulate_and_map_work() {arrayseq_tabulate_and_map_work35,1162
fn arrayseq_subseq_append_filter_flatten() {arrayseq_subseq_append_filter_flatten46,1654
fn arrayseq_update_and_inject_preserve_original() {arrayseq_update_and_inject_preserve_original72,2888
fn arrayseq_collect_iterate_reduce_scan() {arrayseq_collect_iterate_reduce_scan87,3606
    fn pair_cmp(lhs: &&str, rhs: &&str) -> O { lhs.cmp(rhs) }pair_cmp89,3720

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test12ArraySeqStEphMacro.rs,282
pub mod TestArraySeqStEphMacro {TestArraySeqStEphMacro3,93
fn arrayseq_steph_basic_macros() {arrayseq_steph_basic_macros10,344
fn arrayseq_steph_full_pipeline() {arrayseq_steph_full_pipeline19,702
    fn pair_cmp(lhs: &&str, rhs: &&str) -> O { lhs.cmp(rhs) }pair_cmp46,2262

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test10ArraySeqStPerChap18.rs,946
pub mod TestArraySeqStPerChap {TestArraySeqStPerChap3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci10,325
        fn fib(n: N) -> N {fib11,360
    fn test_map_increment() {test_map_increment36,1054
    fn test_subseq() {test_subseq43,1323
    fn test_append() {test_append54,1705
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append62,2016
    fn test_filter_even() {test_filter_even75,2717
    fn test_flatten() {test_flatten90,3452
    fn test_update_sequence() {test_update_sequence104,4378
    fn test_inject_and_ninject() {test_inject_and_ninject114,5007
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan139,6484
    fn test_iterate_sum_basic() {test_iterate_sum_basic158,7482
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum166,7767
    fn test_collect_groups_by_key() {test_collect_groups_by_key178,8230

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test13ArraySeqStEphChap18.rs,1049
pub mod TestArraySeqStEphChap {TestArraySeqStEphChap3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci9,288
        fn fib(n: N) -> N {fib10,323
    fn test_map_increment() {test_map_increment35,1028
    fn test_subseq() {test_subseq42,1293
    fn test_append() {test_append53,1804
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append61,2111
    fn test_filter_even() {test_filter_even74,2838
    fn test_flatten() {test_flatten88,3478
    fn test_update_sequence() {test_update_sequence102,4307
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins114,4870
    fn test_inject_conflicts_last_wins_2() {test_inject_conflicts_last_wins_2127,5702
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan143,6522
    fn test_iterate_sum_basic() {test_iterate_sum_basic162,7550
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum170,7835
    fn test_collect_groups_by_key() {test_collect_groups_by_key183,8310

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test28ArraySeqMtEphChap18.rs,1095
pub mod Test28ArraySeqMtEphChap {Test28ArraySeqMtEphChap4,163
    fn identity(i: N) -> N { i }identity9,334
    fn double(i: N) -> N { i * 2 }double10,367
    fn add_one(i: N) -> N { i + 1 }add_one11,402
    fn add_ten(i: N) -> N { i + 10 }add_ten12,438
    fn multiply_by_two(x: &N) -> N { x * 2 }multiply_by_two13,475
    fn add_nums(x: &N, y: &N) -> N { x + y }add_nums14,520
    fn is_even_bool(x: &N) -> B { if x % 2 == 0 { B::True } else { B::False } }is_even_bool15,565
    fn test_new_and_set() {test_new_and_set18,658
    fn test_tabulate_basic() {test_tabulate_basic32,1068
    fn test_map_parallel() {test_map_parallel42,1347
    fn test_reduce_parallel() {test_reduce_parallel53,1790
    fn test_filter() {test_filter62,2161
    fn test_append() {test_append73,2553
    fn test_flatten() {test_flatten87,3177
    fn test_scan() {test_scan102,3889
    fn test_iterate() {test_iterate116,4536
    fn test_update() {test_update125,4928
    fn test_inject() {test_inject135,5337
    fn test_empty_and_singleton() {test_empty_and_singleton152,6081

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test05LinkedListStPer19.rs,383
pub mod TestLinkedListPer {TestLinkedListPer2,92
    fn test_select() {test_select7,229
    fn test_append_variants() {test_append_variants20,753
    fn test_map() {test_map29,1074
    fn test_iterate_and_reduce() {test_iterate_and_reduce36,1325
    fn test_scan() {test_scan45,1695
    fn test_flatten() {test_flatten53,2007
    fn test_inject() {test_inject60,2338

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test02MathSeq.rs,1347
pub mod TestMathSeq {TestMathSeq3,93
    fn test_length_and_nth_basic() {test_length_and_nth_basic10,272
    fn test_add_last_and_delete_last() {test_add_last_and_delete_last18,492
    fn test_new_empty_singleton_and_predicates() {test_new_empty_singleton_and_predicates31,941
    fn test_set_in_bounds_and_out_of_bounds() {test_set_in_bounds_and_out_of_bounds50,1539
    fn test_subseq_view_bounds() {test_subseq_view_bounds61,1898
    fn test_subseq_copy_bounds() {test_subseq_copy_bounds74,2367
    fn test_domain() {test_domain84,2663
    fn test_range_deduplicates_preserving_order() {test_range_deduplicates_preserving_order90,2804
    fn test_debug_format_for_mathseq() {test_debug_format_for_mathseq97,3032
    fn test_display_format_for_mathseq() {test_display_format_for_mathseq103,3197
    fn test_multiset_range_counts_first_occurrence_order() {test_multiset_range_counts_first_occurrence_order109,3362
    fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics117,3612
    fn test_range_empty_returns_empty() {test_range_empty_returns_empty123,3742
    fn test_multiset_range_empty_returns_empty() {test_multiset_range_empty_returns_empty130,3915
    fn test_domain_empty_is_empty() {test_domain_empty_is_empty137,4111
    fn test_iter_collect_and_sum() {test_iter_collect_and_sum144,4273

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test03LinkedListStPer.rs,539
pub mod TestLinkedListPer {TestLinkedListPer3,93
    fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates9,292
    fn test_new_and_nth_set() {test_new_and_nth_set18,681
    fn test_subseq_copy() {test_subseq_copy30,1116
    fn test_from_vec_and_debug_format() {test_from_vec_and_debug_format39,1367
    fn test_iter_inorder_collect() {test_iter_inorder_collect46,1575
    fn test_nth_out_of_bounds_panics() {test_nth_out_of_bounds_panics54,1829
    fn test_display_impl() {test_display_impl60,1971

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test04LinkedListStPer.rs,569
pub mod TestLinkedListStPer {TestLinkedListStPer2,92
    fn test_tabulate() {test_tabulate7,231
    fn test_map() {test_map14,435
    fn test_filter() {test_filter22,720
    fn test_append() {test_append35,1132
    fn test_update() {test_update43,1473
    fn test_inject() {test_inject50,1746
    fn test_ninject() {test_ninject58,2097
    fn test_iterate() {test_iterate66,2452
    fn test_reduce() {test_reduce73,2691
    fn test_scan() {test_scan80,2924
    fn test_flatten() {test_flatten88,3248
    fn test_collect() {test_collect99,3672

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test27ArraySeqMtPerChap18.rs,1134
pub mod Test27ArraySeqMtPerChap {Test27ArraySeqMtPerChap4,153
    fn identity(i: N) -> N { i }identity10,382
    fn double(i: N) -> N { i * 2 }double11,415
    fn square(i: N) -> N { i * i }square12,450
    fn add_100(i: N) -> N { i + 100 }add_10013,485
    fn const_42(_i: N) -> N { 42 }const_4214,523
    fn format_item(i: N) -> String { format!("item_{}", i) }format_item15,558
    fn is_even_bool(i: N) -> B { if i % 2 == 0 { B::True } else { B::False } }is_even_bool16,619
    fn assert_set_eq<T: PartialEq + std::fmt::Debug>(actual: &[T], expected: &[T]) {assert_set_eq19,750
    fn test_tabulate_basic() {test_tabulate_basic30,1159
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci41,1558
        fn fib(n: N) -> N {fib42,1593
    fn test_tabulate_empty() {test_tabulate_empty69,2396
    fn test_tabulate_single() {test_tabulate_single76,2608
    fn test_tabulate_string() {test_tabulate_string83,2819
    fn test_tabulate_boolean() {test_tabulate_boolean95,3390
    fn test_tabulate_squares() {test_tabulate_squares106,3840
    fn test_tabulate_large() {test_tabulate_large117,4232

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test06LinkedListStEph.rs,547
pub mod TestLinkedListEph {TestLinkedListEph4,149
    fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates9,286
    fn test_new_and_nth_set() {test_new_and_nth_set18,675
    fn test_subseq_copy() {test_subseq_copy27,926
    fn test_linkedlisteph_basic() {test_linkedlisteph_basic36,1177
    fn test_debug_format_for_eph() {test_debug_format_for_eph45,1433
    fn test_display_format_for_eph() {test_display_format_for_eph51,1601
    fn test_iter_inorder_collect_eph() {test_iter_inorder_collect_eph57,1769

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test07LinkedListStEph.rs,928
pub mod TestLinkedListStEph {TestLinkedListStEph3,93
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list7,220
    fn test_construct_eph_from_vec() {test_construct_eph_from_vec17,593
    fn test_eph_is_empty_and_singleton() {test_eph_is_empty_and_singleton23,766
    fn test_eph_set_and_subseq_copy() {test_eph_set_and_subseq_copy31,1117
    fn test_iter_inorder_collect_eph_ch18() {test_iter_inorder_collect_eph_ch1840,1396
    fn test_tabulate_and_map_ch18() {test_tabulate_and_map_ch1846,1578
    fn test_append_ch18() {test_append_ch1853,1894
    fn test_filter_ch18() {test_filter_ch1861,2238
    fn test_update_ch18() {test_update_ch1878,2722
    fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1885,3002
    fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1895,3521
    fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch18107,4104

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test11ArraySeqStPerMacro.rs,415
pub mod TestArraySeqStPerMacro {TestArraySeqStPerMacro3,93
fn arrayseq_stper_macro_empty() {arrayseq_stper_macro_empty11,440
fn arrayseq_stper_macro_literal() {arrayseq_stper_macro_literal18,661
fn arrayseq_stper_macro_repeat() {arrayseq_stper_macro_repeat25,884
fn arrayseq_stper_operations() {arrayseq_stper_operations34,1244
    fn str_cmp(lhs: &&str, rhs: &&str) -> O { lhs.cmp(rhs) }str_cmp62,2964

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test12ArraySeqStEph.rs,283
pub mod TestArraySeqEph {TestArraySeqEph3,93
    fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic8,224
    fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter17,502
    fn test_iterators_collect() {test_iterators_collect28,1032

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test14ArraySeqStEph.rs,1250
pub mod TestArraySeqStEph {TestArraySeqStEph3,93
    fn test_empty() {test_empty8,226
    fn test_singleton() {test_singleton14,362
    fn test_map() {test_map20,532
    fn test_append() {test_append27,790
    fn test_append2() {test_append235,1121
    fn test_filter_even_numbers() {test_filter_even_numbers44,1454
    fn test_filter_none() {test_filter_none51,1779
    fn test_update_in_bounds() {test_update_in_bounds59,2101
    fn test_update_out_of_bounds() {test_update_out_of_bounds66,2350
    fn test_isEmpty() {test_isEmpty73,2596
    fn test_isSingleton() {test_isSingleton83,2964
    fn test_iterate_sum() {test_iterate_sum93,3348
    fn test_iterate_concat() {test_iterate_concat100,3580
    fn test_map_empty() {test_map_empty114,3999
    fn test_append_with_empty() {test_append_with_empty121,4223
    fn test_append2_equivalence() {test_append2_equivalence131,4700
    fn test_filter_empty_sequence() {test_filter_empty_sequence140,5071
    fn test_select_boundary() {test_select_boundary147,5312
    fn test_reduce_sum_basic_ch19() {test_reduce_sum_basic_ch19160,5777
    fn test_scan_sum_basic_ch19() {test_scan_sum_basic_ch19174,6359
    fn test_flatten_ch19() {test_flatten_ch19185,6759

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test13ArraySeqStEph18.rs,925
pub mod TestArraySeqStEph {TestArraySeqStEph3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci8,226
        fn fib(n: N) -> N {fib9,261
    fn test_map_increment() {test_map_increment37,1001
    fn test_append() {test_append45,1267
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append53,1574
    fn test_filter_even() {test_filter_even66,2301
    fn test_flatten() {test_flatten92,3160
    fn test_update_sequence() {test_update_sequence106,3989
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins118,4552
    fn test_inject_conflicts_last_wins_2() {test_inject_conflicts_last_wins_2137,5390
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan155,6170
    fn test_iterate_sum_basic() {test_iterate_sum_basic174,7198
    fn test_collect_groups_by_key() {test_collect_groups_by_key183,7484

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test10ArraySeqStPer18.rs,936
pub mod TestArraySeqStPer {TestArraySeqStPer3,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci8,226
        fn fib(n: N) -> N {fib9,261
    fn test_map_increment() {test_map_increment37,990
    fn test_subseq() {test_subseq44,1259
    fn test_append() {test_append55,1641
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1952
    fn test_filter_even() {test_filter_even76,2653
    fn test_flatten() {test_flatten103,3607
    fn test_update_sequence() {test_update_sequence117,4533
    fn test_inject_and_ninject() {test_inject_and_ninject127,5162
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan162,6755
    fn test_iterate_sum_basic() {test_iterate_sum_basic181,7753
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum189,8038
    fn test_collect_groups_by_key() {test_collect_groups_by_key201,8501

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test28ArraySeqMtPer.rs,474
pub mod Test28ArraySeqMtPer {Test28ArraySeqMtPer4,147
    fn test_inject_basic() {test_inject_basic12,335
    fn test_inject_conflicting_updates() {test_inject_conflicting_updates28,975
    fn test_inject_out_of_bounds() {test_inject_out_of_bounds44,1674
    fn test_inject_empty_changes() {test_inject_empty_changes56,2217
    fn test_inject_empty_values() {test_inject_empty_values68,2671
    fn test_inject_string_values() {test_inject_string_values81,3255

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test41ArraySeqMtEph.rs,212
pub mod Test41ArraySeqMtEph {Test41ArraySeqMtEph3,93
fn test_arrayseq_mteph_basic_ops() {test_arrayseq_mteph_basic_ops9,244
fn test_arrayseq_mteph_append_and_map() {test_arrayseq_mteph_append_and_map26,717

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test26ArraySeqMtPer.rs,910
pub mod Test26ArraySeqMtPer {Test26ArraySeqMtPer3,93
    fn test_new_and_set() {test_new_and_set8,228
    fn test_length_and_nth_basic() {test_length_and_nth_basic22,699
    fn test_empty() {test_empty30,924
    fn test_sequence_basic() {test_sequence_basic37,1123
    fn test_singleton() {test_singleton53,1729
    fn test_from_vec() {test_from_vec61,1953
    fn test_subseq_copy() {test_subseq_copy69,2163
    fn test_subseq_view() {test_subseq_view79,2465
    fn test_iterators() {test_iterators89,2772
    fn test_set_out_of_bounds() {test_set_out_of_bounds102,3263
    fn test_macro_literals() {test_macro_literals109,3460
    fn test_equality_and_debug() {test_equality_and_debug129,4126
    fn test_display_format() {test_display_format145,4611
    fn test_string_sequences() {test_string_sequences154,4894
    fn test_boolean_sequences() {test_boolean_sequences162,5127

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test11ArraySeqStPer.rs,323
pub mod TestArraySeqPer {TestArraySeqPer3,93
    fn test_map_and_select_and_append() {test_map_and_select_and_append10,251
    fn test_filter() {test_filter23,813
    fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten31,1159
    fn test_inject_and_parallel() {test_inject_and_parallel50,2192

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test27ArraySeqMtPer18.rs,586
pub mod Test27ArraySeqMtPer {Test27ArraySeqMtPer4,147
    fn test_tabulate_basic() {test_tabulate_basic9,282
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci23,754
        fn fib(n: N) -> N {fib24,789
    fn test_tabulate_empty() {test_tabulate_empty56,1818
    fn test_tabulate_single() {test_tabulate_single63,2033
    fn test_tabulate_string() {test_tabulate_string70,2242
    fn test_tabulate_boolean() {test_tabulate_boolean86,3012
    fn test_tabulate_squares() {test_tabulate_squares101,3680
    fn test_tabulate_large() {test_tabulate_large116,4259

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test09ArraySeqStPer.rs,2134
pub mod TestArraySeqStPer {TestArraySeqStPer2,92
    fn test_new_and_set() {test_new_and_set8,226
    fn test_length_and_nth_basic() {test_length_and_nth_basic24,822
    fn test_empty() {test_empty32,1047
    fn test_sequence_basic() {test_sequence_basic39,1237
    fn test_singleton() {test_singleton55,2105
    fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton63,2313
    fn test_from_vec() {test_from_vec78,2804
    fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers93,3515
    fn test_sequence_equality_strings() {test_sequence_equality_strings118,4577
    fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference143,5733
        struct PartialComparable {PartialComparable145,5824
            value: i32,value146,5859
        impl std::fmt::Display for PartialComparable {PartialComparable149,5945
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt150,6000
        struct TotalComparable {TotalComparable162,6692
            value: N,value163,6725
        impl std::fmt::Display for TotalComparable {TotalComparable166,6758
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt167,6811
    fn test_ordering_numbers_basic() {test_ordering_numbers_basic182,7524
    fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal191,7771
    fn test_ordering_strings_basic() {test_ordering_strings_basic197,7899
    fn test_strings_equal_is_equal() {test_strings_equal_is_equal206,8144
    fn test_nth_on_empty_panics() {test_nth_on_empty_panics213,8293
    fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics220,8446
    fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap227,8599
    fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes234,8832
    fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic242,9092
    fn test_new_set_persistent() {test_new_set_persistent251,9475
    fn test_iterator_collects_in_order() {test_iterator_collects_in_order261,9814

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test29Algorithm_21_1.rs,489
pub mod Test29Algorithm_21_1 {Test29Algorithm_21_14,176
fn points2d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, N>> {points2d_tab_flat11,463
fn test_points2d_n3_example() {test_points2d_n3_example26,1043
fn test_points2d_n1_empty() {test_points2d_n1_empty40,1347
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values46,1453
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order54,1652
fn test_points2d_debug_shape() {test_points2d_debug_shape75,2155

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test38Problem21_1.rs,482
pub mod Test38Problem21_1 {Test38Problem21_14,167
fn points2d(n: N) -> ArraySeqStPerS<Pair<N, N>> {points2d10,401
fn test_points2d_n3_example() {test_points2d_n3_example25,742
fn test_points2d_n1_empty() {test_points2d_n1_empty40,1058
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values46,1155
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order54,1337
fn test_points2d_debug_display_shapes() {test_points2d_debug_display_shapes75,1783

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test30Algorithm_21_2.rs,588
pub mod Test30Algorithm_21_2 {Test30Algorithm_21_24,172
fn points3d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {points3d_tab_flat11,516
fn test_points3d_tab_flat_n0_empty() {test_points3d_tab_flat_n0_empty40,1894
fn test_points3d_tab_flat_n1_single() {test_points3d_tab_flat_n1_single46,2009
fn test_points3d_tab_flat_n2_values_and_order() {test_points3d_tab_flat_n2_values_and_order53,2191
fn test_points3d_tab_flat_iterator_order() {test_points3d_tab_flat_iterator_order70,2632
fn test_points3d_tab_flat_debug_shape() {test_points3d_tab_flat_debug_shape89,3126

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test34Exercise_21_8_and_Algorithm_21_5.rs,443
pub mod Test34Exercise_21_8_and_Algorithm_21_5 {Test34Exercise_21_8_and_Algorithm_21_53,93
fn is_divisible(n: N, i: N) -> B {is_divisible9,314
fn is_prime(n: N) -> B {is_prime20,585
fn primes_bf(n: N) -> ArraySeqStPerS<N> {primes_bf38,1214
fn test_is_prime_small_values() {test_is_prime_small_values48,1580
fn test_primes_bf_small() {test_primes_bf_small58,1857
fn test_primes_bf_debug_shape() {test_primes_bf_debug_shape65,2009

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Exercise_21_2.txt,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test33Exercise_21_7.rs,440
pub mod Test33Exercise_21_7 {Test33Exercise_21_74,190
fn is_even(x: &N) -> B {is_even8,313
fn is_vowel(c: &char) -> B {is_vowel15,413
fn pair_even_with_vowels(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<char>) -> ArraySeqStPerS<Pairpair_even_with_vowels24,702
fn test_pair_even_with_vowels_basic() {test_pair_even_with_vowels_basic43,1579
fn test_pair_even_with_vowels_debug_shape() {test_pair_even_with_vowels_debug_shape52,1920

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test37Problem_21_4.rs,1292
pub mod Test37Problem_21_4 {Test37Problem_21_44,137
fn cartesian_loops(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<&'static str>) -> ArraySeqStPerS<Pacartesian_loops10,372
    let mut v: Vec<Pair<N, &'static str>> = Vec::with_capacity(a.length() * b.length());str11,491
fn cartesian_tab_flat(cartesian_tab_flat22,874
    let nested: ArraySeqStPerS<ArraySeqStPerS<Pair<N, &'static str>>> =str26,1007
        <ArraySeqStPerS<ArraySeqStPerS<Pair<N, &'static str>>> as ArraySeqStPerTrait<str27,1079
            ArraySeqStPerS<Pair<N, &'static str>>,str28,1165
                <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static ststr31,1257
                <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static ststr31,1257
    <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::flattestr38,1522
    <ArraySeqStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::flattestr38,1522
fn test_cartesian_loops_basic() {test_cartesian_loops_basic42,1640
fn test_cartesian_tab_flat_basic() {test_cartesian_tab_flat_basic58,2064
fn test_cartesian_iterator_order() {test_cartesian_iterator_order74,2494
fn test_cartesian_debug_shape() {test_cartesian_debug_shape83,2828

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test32Exercise_21_5_and_21_6.rs,463
pub mod Test32Exercise_21_5_and_21_6 {Test32Exercise_21_5_and_21_64,169
fn all_contiguous_subseqs<T: StT>(a: &ArraySeqStPerS<T>) -> ArraySeqStPerS<ArraySeqStPerS<T>> {all_contiguous_subseqs11,461
fn test_all_contiguous_subseqs_n0() {test_all_contiguous_subseqs_n030,1229
fn test_all_contiguous_subseqs_n3_values() {test_all_contiguous_subseqs_n3_values37,1418
fn test_all_contiguous_subseqs_debug_shape() {test_all_contiguous_subseqs_debug_shape46,1769

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test36Problem_21_3.rs,546
pub mod Test36Problem_21_3 {Test36Problem_21_34,171
fn points3d_loops(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {points3d_loops10,457
fn test_points3d_loops_n0_empty() {test_points3d_loops_n0_empty27,881
fn test_points3d_loops_n1_single() {test_points3d_loops_n1_single33,990
fn test_points3d_loops_n2_values_and_order() {test_points3d_loops_n2_values_and_order40,1166
fn test_points3d_loops_iterator_order() {test_points3d_loops_iterator_order57,1601
fn test_points3d_loops_debug_shape() {test_points3d_loops_debug_shape76,2089

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap21/Test31Algorithm_21_6.rs,322
pub mod Test31Algorithm_21_6 {Test31Algorithm_21_64,157
fn prime_sieve(n: N) -> ArraySeqStPerS<N> {prime_sieve11,493
fn test_prime_sieve_small() {test_prime_sieve_small59,2472
fn test_prime_sieve_n2_empty() {test_prime_sieve_n2_empty65,2605
fn test_prime_sieve_debug_shape() {test_prime_sieve_debug_shape71,2708

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap23/TestPrimTreeSeqSt.rs,495
pub mod TestPrimTreeSeqSt {TestPrimTreeSeqSt2,92
    fn expose_zero_returns_zero() {expose_zero_returns_zero7,225
    fn expose_one_returns_one() {expose_one_returns_one13,408
    fn expose_two_splits_sequence() {expose_two_splits_sequence22,685
    fn join_zero_creates_empty_sequence() {join_zero_creates_empty_sequence37,1247
    fn join_two_concatenates_sequences() {join_two_concatenates_sequences43,1431
    fn expose_then_join_roundtrip() {expose_then_join_roundtrip52,1833

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap36/Test39Chapter36St.rs,665
pub mod Test39Chapter36St {Test39Chapter36St3,93
trait ToVec<T: StT> {ToVec9,303
    fn to_vec(&self) -> Vec<T>;to_vec10,325
impl<T: StT> ToVec<T> for ArraySeqStEphS<T> {ArraySeqStEphS12,359
    fn to_vec(&self) -> Vec<T> { (0..self.length()).map(|i| self.nth(i).clone()).collect() }to_vec13,405
fn quick_sort_variants_produce_sorted_output() {quick_sort_variants_produce_sorted_output17,509
fn quick_sort_handles_edge_cases() {quick_sort_handles_edge_cases33,1022
fn pivot_strategies_match_expectations() {pivot_strategies_match_expectations56,1811
fn quick_sort_small_inputs_use_shared_pivots() {quick_sort_small_inputs_use_shared_pivots81,2622

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap36/Test44Chapter36MtSlice.rs,847
pub mod Test44Chapter36MtSlice {Test44Chapter36MtSlice3,93
fn to_vec<T: StT + Send + Sync>(a: &ArraySeqMtEphSliceS<T>) -> Vec<T> { a.to_vec() }to_vec11,398
fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }is_sorted13,484
fn mk_seq(data: &[i32]) -> ArraySeqMtEphSliceS<i32> { ArraySeqMtEphSliceS::from_vec(data.to_vec(mk_seq15,578
fn quick_sort_slice_variants_produce_sorted_output() {quick_sort_slice_variants_produce_sorted_output18,688
fn quick_sort_slice_edge_cases() {quick_sort_slice_edge_cases36,1375
fn quick_sort_slice_large_inputs() {quick_sort_slice_large_inputs59,2383
fn slice_pivot_strategies_match_expectations() {slice_pivot_strategies_match_expectations72,3000
fn quick_sort_slice_small_inputs_use_shared_pivots() {quick_sort_slice_small_inputs_use_shared_pivots92,3625

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap36/Test40Chapter36Mt.rs,708
pub mod Test40Chapter36Mt {Test40Chapter36Mt3,93
fn to_vec<T: StT>(a: &ArraySeqMtEphS<T>) -> Vec<T> { (0..a.length()).map(|i| a.nth_cloned(i)).coto_vec10,310
fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }is_sorted12,417
fn quick_sort_mt_variants_produce_sorted_output() {quick_sort_mt_variants_produce_sorted_output15,519
fn quick_sort_mt_edge_cases() {quick_sort_mt_edge_cases33,1194
fn pivot_mt_strategies_match_expectations() {pivot_mt_strategies_match_expectations56,2238
fn quick_sort_mt_large_inputs() {quick_sort_mt_large_inputs77,2981
fn quick_sort_mt_small_inputs_use_shared_pivots() {quick_sort_mt_small_inputs_use_shared_pivots90,3565

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test51BSTSetMtEph.rs,11099
pub mod Test51BSTSetMtEph {Test51BSTSetMtEph3,93
trait TestSet: Sized {TestSet8,278
    fn empty() -> Self;empty9,301
    fn insert(&mut self, value: i32);insert10,325
    fn delete(&mut self, value: &i32);delete11,363
    fn size(&self) -> usize;size12,402
    fn is_empty(&self) -> B;is_empty13,431
    fn contains(&self, value: &i32) -> B;contains14,460
    fn minimum(&self) -> Option<i32>;minimum15,502
    fn maximum(&self) -> Option<i32>;maximum16,540
    fn union(&self, other: &Self) -> Self;union17,578
    fn intersection(&self, other: &Self) -> Self;intersection18,621
    fn difference(&self, other: &Self) -> Self;difference19,671
    fn split(&self, pivot: &i32) -> (Self, B, Self);split20,719
    fn join_pair(left: Self, right: Self) -> Self;join_pair21,772
    fn join_m(left: Self, pivot: i32, right: Self) -> Self;join_m22,823
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self;filter23,883
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32;reduce24,951
    fn iter_seq(&self) -> ArraySeqStPerS<i32>;iter_seq25,1025
impl TestSet for apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32> {BSTSetPlainMt28,1075
    fn empty() -> Self { Self::empty() }empty29,1166
    fn insert(&mut self, value: i32) { self.insert(value); }insert31,1208
    fn delete(&mut self, value: &i32) { self.delete(value); }delete33,1270
    fn size(&self) -> usize { self.size() }size35,1333
    fn is_empty(&self) -> B { self.is_empty() }is_empty37,1378
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains39,1427
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum41,1494
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum43,1551
    fn union(&self, other: &Self) -> Self { self.union(other) }union45,1608
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection47,1673
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference49,1752
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split51,1827
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair53,1902
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m55,1986
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter57,2083
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce59,2178
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq61,2278
impl TestSet for apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32> {BSTSetAVLMt64,2352
    fn empty() -> Self { Self::empty() }empty65,2437
    fn insert(&mut self, value: i32) { self.insert(value); }insert67,2479
    fn delete(&mut self, value: &i32) { self.delete(value); }delete69,2541
    fn size(&self) -> usize { self.size() }size71,2604
    fn is_empty(&self) -> B { self.is_empty() }is_empty73,2649
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains75,2698
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum77,2765
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum79,2822
    fn union(&self, other: &Self) -> Self { self.union(other) }union81,2879
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection83,2944
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference85,3023
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split87,3098
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair89,3173
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m91,3257
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter93,3354
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce95,3449
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq97,3549
impl TestSet for apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32> {BSTSetRBMt100,3623
    fn empty() -> Self { Self::empty() }empty101,3705
    fn insert(&mut self, value: i32) { self.insert(value); }insert103,3747
    fn delete(&mut self, value: &i32) { self.delete(value); }delete105,3809
    fn size(&self) -> usize { self.size() }size107,3872
    fn is_empty(&self) -> B { self.is_empty() }is_empty109,3917
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains111,3966
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum113,4033
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum115,4090
    fn union(&self, other: &Self) -> Self { self.union(other) }union117,4147
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection119,4212
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference121,4291
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split123,4366
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair125,4441
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m127,4525
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter129,4622
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce131,4717
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq133,4817
impl TestSet for apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32> {BSTSetBBAlphaMt136,4891
    fn empty() -> Self { Self::empty() }empty137,4988
    fn insert(&mut self, value: i32) { self.insert(value); }insert139,5030
    fn delete(&mut self, value: &i32) { self.delete(value); }delete141,5092
    fn size(&self) -> usize { self.size() }size143,5155
    fn is_empty(&self) -> B { self.is_empty() }is_empty145,5200
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains147,5249
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum149,5316
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum151,5373
    fn union(&self, other: &Self) -> Self { self.union(other) }union153,5430
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection155,5495
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference157,5574
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split159,5649
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair161,5724
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m163,5808
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter165,5905
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce167,6000
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq169,6100
impl TestSet for apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32> {BSTSetTreapMt172,6174
    fn empty() -> Self { Self::empty() }empty173,6265
    fn insert(&mut self, value: i32) { self.insert(value); }insert175,6307
    fn delete(&mut self, value: &i32) { self.delete(value); }delete177,6369
    fn size(&self) -> usize { self.size() }size179,6432
    fn is_empty(&self) -> B { self.is_empty() }is_empty181,6477
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains183,6526
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum185,6593
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum187,6650
    fn union(&self, other: &Self) -> Self { self.union(other) }union189,6707
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection191,6772
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference193,6851
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split195,6926
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair197,7001
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m199,7085
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter201,7182
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce203,7277
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq205,7377
impl TestSet for apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32> {BSTSetSplayMt208,7451
    fn empty() -> Self { Self::empty() }empty209,7542
    fn insert(&mut self, value: i32) { self.insert(value); }insert211,7584
    fn delete(&mut self, value: &i32) { self.delete(value); }delete213,7646
    fn size(&self) -> usize { self.size() }size215,7709
    fn is_empty(&self) -> B { self.is_empty() }is_empty217,7754
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains219,7803
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum221,7870
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum223,7927
    fn union(&self, other: &Self) -> Self { self.union(other) }union225,7984
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection227,8049
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference229,8128
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split231,8203
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair233,8278
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m235,8362
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter237,8459
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce239,8554
    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }iter_seq241,8654
fn exercise_set<S: TestSet>() {exercise_set244,8728
fn test_plain_bst_set_ops() {test_plain_bst_set_ops304,10498
fn test_avl_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTStest_avl_bst_set_ops309,10634
fn test_rb_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRtest_rb_bst_set_ops312,10758
fn test_bbalpha_bst_set_ops() {test_bbalpha_bst_set_ops315,10878
fn test_treap_bst_set_ops() { exercise_set::<apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEphtest_treap_bst_set_ops320,11022
fn test_splay_bst_set_ops() {test_splay_bst_set_ops323,11154

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test18AVLTreeSeqStEph.rs,118
pub mod TestAVLTreeSeqEph {TestAVLTreeSeqEph2,92
    fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic8,342

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test47BSTBBAlphaStEph.rs,218
pub mod TestBSTBBAlphaStEph {TestBSTBBAlphaStEph3,93
fn bbalpha_insert_find_balances() {bbalpha_insert_find_balances8,214
fn bbalpha_duplicate_insert_is_idempotent() {bbalpha_duplicate_insert_is_idempotent28,834

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test19AVLTreeSeqStEph18.rs,281
pub mod TestAVLTreeSeqStEph {TestAVLTreeSeqStEph4,171
    fn test_tabulate_inorder() {test_tabulate_inorder11,405
    fn test_map_increment() {test_map_increment17,614
    fn test_append_union() {test_append_union25,972
    fn test_filter_even() {test_filter_even39,1478

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test46BSTRBStEph.rs,196
pub mod Test46BSTRBStEph {Test46BSTRBStEph3,93
fn rb_insert_find_and_bounds() {rb_insert_find_and_bounds9,235
fn rb_duplicate_insert_is_idempotent() {rb_duplicate_insert_is_idempotent29,853

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test15AVLTreeSeqStPer.rs,216
pub mod TestAVLTreeSeqPer {TestAVLTreeSeqPer2,92
    fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate8,266
    fn test_iterator_inorder_values() {test_iterator_inorder_values17,595

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test45BSTAVLStEph.rs,202
pub mod Test45BSTAVLStEph {Test45BSTAVLStEph3,93
fn avl_insert_find_and_bounds() {avl_insert_find_and_bounds9,238
fn avl_duplicate_insert_is_idempotent() {avl_duplicate_insert_is_idempotent31,945

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test49BSTSplayStEph.rs,200
pub mod Test49BSTSplayStEph {Test49BSTSplayStEph3,93
fn splay_basic_behaviour() {splay_basic_behaviour9,244
fn splay_duplicate_insert_is_idempotent() {splay_duplicate_insert_is_idempotent28,816

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test20AVLTreeSeqStEph.rs,249
pub mod TestAVLTreeSeqStEph {TestAVLTreeSeqStEph4,172
    fn test_tabulate_and_map() {test_tabulate_and_map11,406
    fn test_select_and_append() {test_select_and_append19,757
    fn test_deflate_and_filter() {test_deflate_and_filter49,1827

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test50BSTMtEph.rs,355
pub mod Test50BSTMtEph {Test50BSTMtEph3,93
fn mt_plain_basic_ops() {mt_plain_basic_ops14,523
fn mt_avl_basic_ops() {mt_avl_basic_ops27,864
fn mt_rb_basic_ops() {mt_rb_basic_ops38,1131
fn mt_bbalpha_basic_ops() {mt_bbalpha_basic_ops48,1332
fn mt_treap_basic_ops() {mt_treap_basic_ops58,1545
fn mt_splay_basic_ops() {mt_splay_basic_ops68,1756

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test16AVLTreeSeqStPer18.rs,282
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer4,141
    fn test_tabulate_inorder() {test_tabulate_inorder10,338
    fn test_map_increment() {test_map_increment17,626
    fn test_append_union() {test_append_union26,1133
    fn test_filter_even() {test_filter_even42,1902

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaMtEph.rs,836
pub mod Test52BSTParaMtEph {Test52BSTParaMtEph3,93
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree8,271
fn make_range_tree(start: i32, end: i32) -> ParamBST<i32> {make_range_tree16,424
fn para_basic_insert_find() {para_basic_insert_find25,600
fn para_split_and_join_pair() {para_split_and_join_pair35,929
fn para_union_and_delete() {para_union_and_delete47,1381
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip64,1852
fn para_intersect_and_difference() {para_intersect_and_difference86,2518
fn para_filter_and_reduce() {para_filter_and_reduce98,2894
fn para_union_large_balanced() {para_union_large_balanced112,3272
fn para_intersect_and_difference_large() {para_intersect_and_difference_large123,3572
fn para_filter_and_reduce_edge_cases() {para_filter_and_reduce_edge_cases139,4175

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test17AVLTreeSeqStPer19.rs,55
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer3,93

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaStEph.rs,377
pub mod Test52BSTParaStEph {Test52BSTParaStEph3,93
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree8,271
fn para_basic_insert_find() {para_basic_insert_find17,432
fn para_split_and_join_pair() {para_split_and_join_pair27,761
fn para_union_and_delete() {para_union_and_delete39,1213
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip56,1684

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap39/Test53BSTParaTreapMtEph.rs,675
pub mod Test53BSTParaTreapMtEph {Test53BSTParaTreapMtEph3,93
fn make_tree(values: &[i32]) -> ParamTreap<i32> {make_tree8,286
fn make_range_tree(start: i32, end: i32) -> ParamTreap<i32> {make_range_tree16,443
fn treap_basic_insert_find() {treap_basic_insert_find25,623
fn treap_split_join_pair() {treap_split_join_pair35,953
fn treap_union_intersect_difference() {treap_union_intersect_difference47,1396
fn treap_filter_reduce() {treap_filter_reduce65,2157
fn treap_join_mid_roundtrip() {treap_join_mid_roundtrip78,2558
fn treap_invariants_priority_heap() {treap_invariants_priority_heap97,3152
    fn check_heap(tree: &ParamTreap<i32>) {check_heap98,3190

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap39/Test48BSTTreapStEph.rs,222
pub mod Test48BSTTreapStEph {Test48BSTTreapStEph3,93
fn treap_insert_find_stays_balanced() {treap_insert_find_stays_balanced8,210
fn treap_duplicate_insert_is_idempotent() {treap_duplicate_insert_is_idempotent28,837

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap26/BenchChapter26Mt.rs,169
fn gen_sequence(n: usize) -> ArraySeqMtPerS<usize> { ArraySeqMtPerS::new(n, 0) }gen_sequence8,320
fn bench_chapter26_mt(c: &mut Criterion) {bench_chapter26_mt10,402

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph.rs,87
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch1910,419

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch189,373

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch199,373

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer18.rs,90
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch188,341

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph18.rs,69
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map10,419

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer.rs,90
fn bench_tabulate_map_mtper_ch19(c: &mut Criterion) {bench_tabulate_map_mtper_ch198,341

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchDirGraphStEph.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build9,362

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabUnDirGraphStEph.rs,980
fn bench_labelled_undir_graph_creation(c: &mut Criterion) {bench_labelled_undir_graph_creation8,361
fn bench_labelled_undir_graph_add_vertex(c: &mut Criterion) {bench_labelled_undir_graph_add_vertex35,1305
fn bench_labelled_undir_graph_add_labeled_edge(c: &mut Criterion) {bench_labelled_undir_graph_add_labeled_edge53,1849
fn bench_labelled_undir_graph_has_edge(c: &mut Criterion) {bench_labelled_undir_graph_has_edge71,2463
fn bench_labelled_undir_graph_get_edge_label(c: &mut Criterion) {bench_labelled_undir_graph_get_edge_label95,3227
fn bench_labelled_undir_graph_neighbors(c: &mut Criterion) {bench_labelled_undir_graph_neighbors121,4112
fn bench_labelled_undir_graph_edges(c: &mut Criterion) {bench_labelled_undir_graph_edges151,5190
fn bench_labelled_undir_graph_macro(c: &mut Criterion) {bench_labelled_undir_graph_macro169,5738
fn bench_labelled_undir_graph_edge_normalization(c: &mut Criterion) {bench_labelled_undir_graph_edge_normalization191,6342

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabDirGraphStEph.rs,932
fn bench_labelled_dir_graph_creation(c: &mut Criterion) {bench_labelled_dir_graph_creation8,355
fn bench_labelled_dir_graph_add_vertex(c: &mut Criterion) {bench_labelled_dir_graph_add_vertex35,1287
fn bench_labelled_dir_graph_add_labeled_arc(c: &mut Criterion) {bench_labelled_dir_graph_add_labeled_arc53,1825
fn bench_labelled_dir_graph_has_arc(c: &mut Criterion) {bench_labelled_dir_graph_has_arc71,2429
fn bench_labelled_dir_graph_get_arc_label(c: &mut Criterion) {bench_labelled_dir_graph_get_arc_label95,3181
fn bench_labelled_dir_graph_out_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_out_neighbors121,4054
fn bench_labelled_dir_graph_in_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_in_neighbors148,4982
fn bench_labelled_dir_graph_arcs(c: &mut Criterion) {bench_labelled_dir_graph_arcs175,5906
fn bench_labelled_dir_graph_macro(c: &mut Criterion) {bench_labelled_dir_graph_macro193,6442

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchUnDirGraphStEph.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build9,366

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap11/BenchFibonacciMt.rs,68
fn bench_fibonacci_mt(c: &mut Criterion) {bench_fibonacci_mt7,284

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap03/BenchInsertionSortSt.rs,161
fn build_vec(len: usize) -> Vec<i32> { (0..len as i32).rev().collect() }build_vec7,299
fn bench_insertion_sort(c: &mut Criterion) {bench_insertion_sort9,373

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap39/BenchBSTTreapStEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree8,307
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap16,504

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap39/BenchBSTTreapMtEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree8,307
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap16,502

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap39/BenchBSTParaTreapMtEph.rs,129
fn build_tree(len: usize) -> ParamTreap<i32> {build_tree8,294
fn bench_para_treap(c: &mut Criterion) {bench_para_treap16,454

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap36/BenchChapter36Mt.rs,131
fn gen_data(n: usize) -> ArraySeqMtEphS<i32> {gen_data8,298
fn bench_quicksort_mt(c: &mut Criterion) {bench_quicksort_mt18,633

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap36/BenchChapter36St.rs,131
fn gen_data(n: usize) -> ArraySeqStEphS<i32> {gen_data8,298
fn bench_quicksort_st(c: &mut Criterion) {bench_quicksort_st18,630

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchMappingStEph.rs,71
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build10,415

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchRelationStEph.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range9,363

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch198,318

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchMathSeq.rs,73
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics10,339

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer.rs,81
fn bench_sll_persistent_ops(c: &mut Criterion) {bench_sll_persistent_ops10,355

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch189,377

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqMtPerChap18.rs,183
fn identity(i: N) -> N { i }identity9,380
fn add_one(x: &N) -> N { x + 1 }add_one10,409
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch1812,443

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStEphChap18.rs,69
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map10,425

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqMtEphChap18.rs,344
fn bench_tabulate_map_mteph_ch18(c: &mut Criterion) {bench_tabulate_map_mteph_ch187,287
fn bench_reduce_parallel_mteph_ch18(c: &mut Criterion) {bench_reduce_parallel_mteph_ch1826,1077
fn bench_filter_mteph_ch18(c: &mut Criterion) {bench_filter_mteph_ch1845,1835
fn bench_scan_mteph_ch18(c: &mut Criterion) {bench_scan_mteph_ch1862,2519

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer19.rs,66
fn bench_ll_per_ch19(c: &mut Criterion) {bench_ll_per_ch199,383

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph.rs,56
fn bench_ll_eph(c: &mut Criterion) {bench_ll_eph8,317

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTAVLStEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree8,307
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl16,500

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSplayMtEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree8,315
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay16,510

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTBBAlphaMtEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree8,323
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha16,522

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTAVLMtEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree8,307
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl16,498

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph.rs,58
fn bench_avl_eph(c: &mut Criterion) {bench_avl_eph8,325

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTRBMtEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree8,303
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb16,492

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSetMtEph.rs,5096
trait BenchSet: Sized {BenchSet16,848
    fn empty() -> Self;empty17,872
    fn insert_value(&mut self, value: i32);insert_value18,896
    fn union_with(&self, other: &Self) -> Self;union_with19,940
    fn difference_with(&self, other: &Self) -> Self;difference_with20,988
    fn filter_divisible_by(&self, divisor: i32) -> Self;filter_divisible_by21,1041
    fn reduce_sum(&self) -> i32;reduce_sum22,1098
fn build_pair<S: BenchSet>(len: usize) -> (S, S) {build_pair25,1134
fn build_single<S: BenchSet>(len: usize) -> S {build_single38,1413
fn bench_set_variants<S: BenchSet>(c: &mut Criterion, label: &str) {bench_set_variants46,1574
impl BenchSet for PlainSet<i32> {PlainSet89,3037
    fn empty() -> Self { BSTSetPlainMtEphLit![] }empty90,3071
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value92,3122
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with94,3190
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with96,3260
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by98,3340
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum100,3444
impl BenchSet for AVLSet<i32> {AVLSet103,3524
    fn empty() -> Self { BSTSetAVLMtEphLit![] }empty104,3556
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value106,3605
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with108,3673
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with110,3743
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by112,3823
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum114,3927
impl BenchSet for RBSet<i32> {RBSet117,4007
    fn empty() -> Self { BSTSetRBMtEphLit![] }empty118,4038
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value120,4086
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with122,4154
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with124,4224
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by126,4304
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum128,4408
impl BenchSet for BBAlphaSet<i32> {BBAlphaSet131,4488
    fn empty() -> Self { BSTSetBBAlphaMtEphLit![] }empty132,4524
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value134,4577
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with136,4645
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with138,4715
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by140,4795
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum142,4899
impl BenchSet for TreapSet<i32> {TreapSet145,4979
    fn empty() -> Self { BSTSetTreapMtEphLit![] }empty146,5013
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value148,5064
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with150,5132
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with152,5202
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by154,5282
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum156,5386
impl BenchSet for SplaySet<i32> {SplaySet159,5466
    fn empty() -> Self { BSTSetSplayMtEphLit![] }empty160,5500
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value162,5551
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with164,5619
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with166,5689
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by168,5769
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum170,5873
fn bench_plain_set(c: &mut Criterion) { bench_set_variants::<PlainSet<i32>>(c, "BSTSetPlainMtEphbench_plain_set173,5953
fn bench_avl_set(c: &mut Criterion) { bench_set_variants::<AVLSet<i32>>(c, "BSTSetAVLMtEph"); }bench_avl_set175,6056
fn bench_rb_set(c: &mut Criterion) { bench_set_variants::<RBSet<i32>>(c, "BSTSetRBMtEph"); }bench_rb_set177,6153
fn bench_bbalpha_set(c: &mut Criterion) { bench_set_variants::<BBAlphaSet<i32>>(c, "BSTSetBBAlphbench_bbalpha_set179,6247
fn bench_treap_set(c: &mut Criterion) { bench_set_variants::<TreapSet<i32>>(c, "BSTSetTreapMtEphbench_treap_set181,6356
fn bench_splay_set(c: &mut Criterion) { bench_set_variants::<SplaySet<i32>>(c, "BSTSetSplayMtEphbench_splay_set183,6459

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTParaStEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree8,305
fn bench_para_bst(c: &mut Criterion) {bench_para_bst16,492

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTRBStEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree8,303
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb16,494

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph19.rs,68
fn bench_avl_eph_ch19(c: &mut Criterion) {bench_avl_eph_ch198,325

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTParaMtEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree8,290
fn bench_para_bst(c: &mut Criterion) {bench_para_bst16,446

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer.rs,81
fn bench_build_and_contains(c: &mut Criterion) {bench_build_and_contains10,364

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTPlainMtEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree8,310
fn bench_bsteph(c: &mut Criterion) {bench_bsteph18,546

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTBBAlphaStEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree8,323
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha16,524

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer19.rs,95
fn bench_build_and_read_persistent(c: &mut Criterion) {bench_build_and_read_persistent11,437

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer18.rs,68
fn bench_avl_per_ch18(c: &mut Criterion) {bench_avl_per_ch188,326

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTPlainStEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree8,310
fn bench_bsteph(c: &mut Criterion) {bench_bsteph18,548

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSplayStEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree8,315
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay16,512

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph18.rs,68
fn bench_avl_eph_ch18(c: &mut Criterion) {bench_avl_eph_ch188,325

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap03/BenchInsertionSortSt.rs,161
fn build_vec(len: usize) -> Vec<i32> { (0..len as i32).rev().collect() }build_vec7,299
fn bench_insertion_sort(c: &mut Criterion) {bench_insertion_sort9,373

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchMappingStEph.rs,71
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build10,415

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchRelationStEph.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range9,363

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchDirGraphStEph.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build9,362

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabUnDirGraphStEph.rs,980
fn bench_labelled_undir_graph_creation(c: &mut Criterion) {bench_labelled_undir_graph_creation8,361
fn bench_labelled_undir_graph_add_vertex(c: &mut Criterion) {bench_labelled_undir_graph_add_vertex35,1305
fn bench_labelled_undir_graph_add_labeled_edge(c: &mut Criterion) {bench_labelled_undir_graph_add_labeled_edge53,1849
fn bench_labelled_undir_graph_has_edge(c: &mut Criterion) {bench_labelled_undir_graph_has_edge71,2463
fn bench_labelled_undir_graph_get_edge_label(c: &mut Criterion) {bench_labelled_undir_graph_get_edge_label95,3227
fn bench_labelled_undir_graph_neighbors(c: &mut Criterion) {bench_labelled_undir_graph_neighbors121,4112
fn bench_labelled_undir_graph_edges(c: &mut Criterion) {bench_labelled_undir_graph_edges151,5190
fn bench_labelled_undir_graph_macro(c: &mut Criterion) {bench_labelled_undir_graph_macro169,5738
fn bench_labelled_undir_graph_edge_normalization(c: &mut Criterion) {bench_labelled_undir_graph_edge_normalization191,6342

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabDirGraphStEph.rs,932
fn bench_labelled_dir_graph_creation(c: &mut Criterion) {bench_labelled_dir_graph_creation8,355
fn bench_labelled_dir_graph_add_vertex(c: &mut Criterion) {bench_labelled_dir_graph_add_vertex35,1287
fn bench_labelled_dir_graph_add_labeled_arc(c: &mut Criterion) {bench_labelled_dir_graph_add_labeled_arc53,1825
fn bench_labelled_dir_graph_has_arc(c: &mut Criterion) {bench_labelled_dir_graph_has_arc71,2429
fn bench_labelled_dir_graph_get_arc_label(c: &mut Criterion) {bench_labelled_dir_graph_get_arc_label95,3181
fn bench_labelled_dir_graph_out_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_out_neighbors121,4054
fn bench_labelled_dir_graph_in_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_in_neighbors148,4982
fn bench_labelled_dir_graph_arcs(c: &mut Criterion) {bench_labelled_dir_graph_arcs175,5906
fn bench_labelled_dir_graph_macro(c: &mut Criterion) {bench_labelled_dir_graph_macro193,6442

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchUnDirGraphStEph.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build9,366

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap11/BenchFibonacciMt.rs,68
fn bench_fibonacci_mt(c: &mut Criterion) {bench_fibonacci_mt7,284

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch198,318

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchMathSeq.rs,73
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics10,339

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer.rs,81
fn bench_sll_persistent_ops(c: &mut Criterion) {bench_sll_persistent_ops10,355

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch189,377

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqMtPerChap18.rs,183
fn identity(i: N) -> N { i }identity9,380
fn add_one(x: &N) -> N { x + 1 }add_one10,409
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch1812,443

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStEphChap18.rs,69
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map10,425

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqMtEphChap18.rs,344
fn bench_tabulate_map_mteph_ch18(c: &mut Criterion) {bench_tabulate_map_mteph_ch187,287
fn bench_reduce_parallel_mteph_ch18(c: &mut Criterion) {bench_reduce_parallel_mteph_ch1826,1077
fn bench_filter_mteph_ch18(c: &mut Criterion) {bench_filter_mteph_ch1845,1835
fn bench_scan_mteph_ch18(c: &mut Criterion) {bench_scan_mteph_ch1862,2519

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer19.rs,66
fn bench_ll_per_ch19(c: &mut Criterion) {bench_ll_per_ch199,383

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph.rs,56
fn bench_ll_eph(c: &mut Criterion) {bench_ll_eph8,317

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph.rs,87
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch1910,419

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch189,373

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch199,373

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer18.rs,90
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch188,341

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph18.rs,69
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map10,419

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer.rs,90
fn bench_tabulate_map_mtper_ch19(c: &mut Criterion) {bench_tabulate_map_mtper_ch198,341

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap26/BenchChapter26Mt.rs,169
fn gen_sequence(n: usize) -> ArraySeqMtPerS<usize> { ArraySeqMtPerS::new(n, 0) }gen_sequence8,320
fn bench_chapter26_mt(c: &mut Criterion) {bench_chapter26_mt10,402

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap36/BenchChapter36Mt.rs,131
fn gen_data(n: usize) -> ArraySeqMtEphS<i32> {gen_data8,298
fn bench_quicksort_mt(c: &mut Criterion) {bench_quicksort_mt18,633

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap36/BenchChapter36St.rs,131
fn gen_data(n: usize) -> ArraySeqStEphS<i32> {gen_data8,298
fn bench_quicksort_st(c: &mut Criterion) {bench_quicksort_st18,630

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTAVLStEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree8,307
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl16,500

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSplayMtEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree8,315
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay16,510

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTBBAlphaMtEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree8,323
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha16,522

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTAVLMtEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree8,307
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl16,498

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph.rs,58
fn bench_avl_eph(c: &mut Criterion) {bench_avl_eph8,325

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTRBMtEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree8,303
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb16,492

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSetMtEph.rs,5096
trait BenchSet: Sized {BenchSet16,848
    fn empty() -> Self;empty17,872
    fn insert_value(&mut self, value: i32);insert_value18,896
    fn union_with(&self, other: &Self) -> Self;union_with19,940
    fn difference_with(&self, other: &Self) -> Self;difference_with20,988
    fn filter_divisible_by(&self, divisor: i32) -> Self;filter_divisible_by21,1041
    fn reduce_sum(&self) -> i32;reduce_sum22,1098
fn build_pair<S: BenchSet>(len: usize) -> (S, S) {build_pair25,1134
fn build_single<S: BenchSet>(len: usize) -> S {build_single38,1413
fn bench_set_variants<S: BenchSet>(c: &mut Criterion, label: &str) {bench_set_variants46,1574
impl BenchSet for PlainSet<i32> {PlainSet89,3037
    fn empty() -> Self { BSTSetPlainMtEphLit![] }empty90,3071
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value92,3122
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with94,3190
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with96,3260
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by98,3340
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum100,3444
impl BenchSet for AVLSet<i32> {AVLSet103,3524
    fn empty() -> Self { BSTSetAVLMtEphLit![] }empty104,3556
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value106,3605
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with108,3673
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with110,3743
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by112,3823
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum114,3927
impl BenchSet for RBSet<i32> {RBSet117,4007
    fn empty() -> Self { BSTSetRBMtEphLit![] }empty118,4038
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value120,4086
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with122,4154
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with124,4224
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by126,4304
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum128,4408
impl BenchSet for BBAlphaSet<i32> {BBAlphaSet131,4488
    fn empty() -> Self { BSTSetBBAlphaMtEphLit![] }empty132,4524
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value134,4577
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with136,4645
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with138,4715
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by140,4795
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum142,4899
impl BenchSet for TreapSet<i32> {TreapSet145,4979
    fn empty() -> Self { BSTSetTreapMtEphLit![] }empty146,5013
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value148,5064
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with150,5132
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with152,5202
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by154,5282
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum156,5386
impl BenchSet for SplaySet<i32> {SplaySet159,5466
    fn empty() -> Self { BSTSetSplayMtEphLit![] }empty160,5500
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value162,5551
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with164,5619
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with166,5689
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by168,5769
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum170,5873
fn bench_plain_set(c: &mut Criterion) { bench_set_variants::<PlainSet<i32>>(c, "BSTSetPlainMtEphbench_plain_set173,5953
fn bench_avl_set(c: &mut Criterion) { bench_set_variants::<AVLSet<i32>>(c, "BSTSetAVLMtEph"); }bench_avl_set175,6056
fn bench_rb_set(c: &mut Criterion) { bench_set_variants::<RBSet<i32>>(c, "BSTSetRBMtEph"); }bench_rb_set177,6153
fn bench_bbalpha_set(c: &mut Criterion) { bench_set_variants::<BBAlphaSet<i32>>(c, "BSTSetBBAlphbench_bbalpha_set179,6247
fn bench_treap_set(c: &mut Criterion) { bench_set_variants::<TreapSet<i32>>(c, "BSTSetTreapMtEphbench_treap_set181,6356
fn bench_splay_set(c: &mut Criterion) { bench_set_variants::<SplaySet<i32>>(c, "BSTSetSplayMtEphbench_splay_set183,6459

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTParaStEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree8,305
fn bench_para_bst(c: &mut Criterion) {bench_para_bst16,492

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTRBStEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree8,303
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb16,494

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph19.rs,68
fn bench_avl_eph_ch19(c: &mut Criterion) {bench_avl_eph_ch198,325

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTParaMtEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree8,290
fn bench_para_bst(c: &mut Criterion) {bench_para_bst16,446

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer.rs,81
fn bench_build_and_contains(c: &mut Criterion) {bench_build_and_contains10,364

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTPlainMtEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree8,310
fn bench_bsteph(c: &mut Criterion) {bench_bsteph18,546

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTBBAlphaStEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree8,323
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha16,524

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer19.rs,95
fn bench_build_and_read_persistent(c: &mut Criterion) {bench_build_and_read_persistent11,437

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStPer18.rs,68
fn bench_avl_per_ch18(c: &mut Criterion) {bench_avl_per_ch188,326

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTPlainStEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree8,310
fn bench_bsteph(c: &mut Criterion) {bench_bsteph18,548

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSplayStEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree8,315
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay16,512

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchAVLTreeSeqStEph18.rs,68
fn bench_avl_eph_ch18(c: &mut Criterion) {bench_avl_eph_ch188,325

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap39/BenchBSTTreapStEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree8,307
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap16,504

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap39/BenchBSTTreapMtEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree8,307
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap16,502

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap39/BenchBSTParaTreapMtEph.rs,129
fn build_tree(len: usize) -> ParamTreap<i32> {build_tree8,294
fn bench_para_treap(c: &mut Criterion) {bench_para_treap16,454
