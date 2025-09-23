
/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36MtSlice.rs,1381
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/LinkedListStPer.rs,2686
pub mod LinkedListStPer19 {LinkedListStPer193,48
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait8,221
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate9,266
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map10,336
        fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T>select11,429
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append12,527
        fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append213,619
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T>;deflate14,712
        fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter15,783
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,871
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,960
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan18,1042
        fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten19,1145
        fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>) -> Linkeinject20,1231
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS23,1350
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate24,1417
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map27,1575
        fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T>select30,1751
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append44,2399
        fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append248,2578
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T> {deflate52,2758
        fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter60,3070
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate64,3245
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce68,3425
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan72,3598
        fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten76,3790
        fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>) -> Linkeinject80,3961

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEphSlice.rs,3453
pub mod ArraySeqMtEphSlice {ArraySeqMtEphSlice8,395
    struct Inner<T: StT> {Inner15,563
        data: Mutex<Box<[T]>>,data16,590
    impl<T: StT> Inner<T> {Inner19,628
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }new20,656
        fn len(&self) -> N {len22,733
    pub struct ArraySeqMtEphSliceS<T: StT> {ArraySeqMtEphSliceS29,921
        inner: Arc<Inner<T>>,inner30,966
        range: Range<N>,range31,996
    pub trait ArraySeqMtEphSliceTrait<T: StT> {ArraySeqMtEphSliceTrait35,1092
        fn new(length: N, init_value: T) -> Self;new36,1140
        fn length(&self) -> N;length37,1190
        fn nth_cloned(&self, index: N) -> T;nth_cloned38,1221
        fn empty() -> Self;empty39,1266
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set40,1294
        fn singleton(item: T) -> Self;singleton41,1375
        fn isEmpty(&self) -> B;isEmpty42,1414
        fn isSingleton(&self) -> B;isSingleton43,1446
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy44,1482
        fn slice(&self, start: N, length: N) -> Self;slice45,1542
    impl<T: StT> ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS48,1603
        pub fn from_box(data: Box<[T]>) -> Self {from_box50,1706
        pub fn from_vec(data: Vec<T>) -> Self { Self::from_box(data.into_boxed_slice()) }from_vec59,2008
        pub fn to_vec(&self) -> Vec<T> {to_vec62,2180
        pub fn with_exclusive<F, R>(&self, f: F) -> Rwith_exclusive68,2444
        fn len(&self) -> N { self.range.end - self.range.start }len78,2750
        fn clamp_subrange(&self, start: N, length: N) -> Range<N> {clamp_subrange80,2816
    impl<T: StT> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS89,3176
        fn new(length: N, init_value: T) -> Self {new90,3249
        fn length(&self) -> N { self.len() }length95,3414
        fn nth_cloned(&self, index: N) -> T {nth_cloned97,3460
        fn empty() -> Self { ArraySeqMtEphSliceS::from_vec(Vec::new()) }empty103,3653
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set105,3727
        fn singleton(item: T) -> Self { ArraySeqMtEphSliceS::from_vec(vec![item]) }singleton117,4123
        fn isEmpty(&self) -> B {isEmpty119,4208
        fn isSingleton(&self) -> B {isSingleton127,4369
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy135,4534
        fn slice(&self, start: N, length: N) -> Self {slice142,4853
    impl<T: StT> Clone for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS151,5107
        fn clone(&self) -> Self {clone152,5159
    impl<T: StT> PartialEq for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS160,5349
        fn eq(&self, other: &Self) -> bool {eq161,5405
    impl<T: StT> Eq for ArraySeqMtEphSliceS<T> {}ArraySeqMtEphSliceS174,5823
    impl<T: StT> Debug for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS176,5874
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt177,5926
    impl<T: StT> Display for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS185,6193
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt186,6247
    fn repeat_vec<T: StT>(length: N, init: T) -> Vec<T> {repeat_vec201,6714
    macro_rules! ArraySeqMtEphSliceSLit {ArraySeqMtEphSliceSLit210,6939
    fn _ArraySeqMtEphSliceSLit_type_checks() {_ArraySeqMtEphSliceSLit_type_checks217,7402

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStEph.rs,2377
pub mod ArraySeqStEph {ArraySeqStEph3,51
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait8,192
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate11,327
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map14,491
        fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T>;select17,672
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append20,874
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append223,1068
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T>;deflate26,1247
        fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter29,1412
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1496
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce31,1583
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan32,1663
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten33,1762
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS36,1849
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate37,1912
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map40,2064
        fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T> {select43,2232
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append55,2636
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append258,2804
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T> {deflate61,2973
        fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter68,3274
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate71,3440
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce74,3613
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan77,3779
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten80,3962

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtPer.rs,3749
pub mod ArraySeqMtPer {ArraySeqMtPer3,129
    pub trait ArraySeqMtPerTrait<T: MtT> {ArraySeqMtPerTrait11,333
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;tabulate15,589
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;map18,826
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;append21,1021
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;filter24,1304
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;update27,1485
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;ninject30,1687
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate32,1857
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes34,1986
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce36,2179
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan38,2303
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten40,2466
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect42,2617
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;inject45,2775
        fn atomicWrite(atomicWrite47,2913
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arrayinject_parallel253,3156
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins54,3263
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arraninject_parallel260,3531
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins61,3639
    impl<T: MtT + StT> ArraySeqMtPerTrait<T> for ArrayMtPerS<T> {ArrayMtPerS68,3852
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate69,3918
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map73,4065
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append77,4225
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter81,4382
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T> {update85,4546
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject89,4710
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate93,4890
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes97,5058
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce101,5260
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan105,5421
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten109,5596
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect120,5998
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> inject154,7335
        fn atomicWrite(atomicWrite158,7523
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arrayinject_parallel2166,7816
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins171,8090
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arraninject_parallel2192,8860
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins197,9137

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEph.rs,2378
pub mod ArraySeqMtEph {ArraySeqMtEph3,67
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait8,208
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate9,251
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map10,319
        fn select<'a>(a: &'a ArraySeqMtEphS<T>, b: &'a ArraySeqMtEphS<T>, i: N) -> Option<T>;select11,408
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append12,502
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append213,588
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T>;deflate14,675
        fn filter(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter15,744
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,828
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,915
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtEphS<T>, Tscan18,995
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;flatten19,1094
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS22,1181
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate23,1244
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map26,1396
        fn select<'a>(a: &'a ArraySeqMtEphS<T>, b: &'a ArraySeqMtEphS<T>, i: N) -> Option<T> {select29,1564
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append41,1966
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append244,2134
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T> {deflate47,2303
        fn filter(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter54,2562
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, mut x: A) -> A {iterate57,2728
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> T {reduce63,2940
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> (ArraySeqMtEphS<Tscan69,3148
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {flatten82,3650

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/LinkedListStEph.rs,2686
pub mod LinkedListStEph19 {LinkedListStEph193,60
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait8,233
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;tabulate9,278
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map10,348
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>select11,441
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append12,539
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append213,631
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T>;deflate14,724
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T>;filter15,795
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,883
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,972
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan18,1054
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten19,1157
        fn inject(values: &LinkedListStEphS<T>, changes: &LinkedListStEphS<Pair<N, T>>) -> Linkeinject20,1243
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS23,1362
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate24,1429
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map27,1587
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>select30,1763
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append44,2411
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append247,2589
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T> {deflate50,2768
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter57,3079
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate60,3253
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce63,3432
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan66,3604
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten69,3795
        fn inject(values: &LinkedListStEphS<T>, changes: &LinkedListStEphS<Pair<N, T>>) -> Linkeinject72,3965

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStPer.rs,2694
pub mod ArraySeqStPer {ArraySeqStPer3,46
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait8,187
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,316
        fn map<U: StT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map12,461
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T>;select14,585
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append16,730
        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append218,860
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T>;deflate20,979
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1132
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate26,1390
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce28,1521
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan30,1645
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten32,1802
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject34,1932
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject36,2083
    impl<T: StT> ArraySeqStPerTrait<T> for ArrayStPerS<T> {ArrayStPerS39,2183
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate40,2243
        fn map<U: StT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map44,2390
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T> {select47,2577
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append60,2958
        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append267,3227
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T> {deflate74,3497
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter78,3719
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate87,4130
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce99,4621
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan114,5278
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten157,6998
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject161,7147
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject165,7325

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphFloat.rs,1231
pub mod WeightedUnDirGraphMtEphFloat {WeightedUnDirGraphMtEphFloat3,107
    pub type WeightedUnDirGraphMtEphFloat<V> = LabUnDirGraphMtEph<V, OrderedF64>;WeightedUnDirGraphMtEphFloat12,463
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphFloat<V> {WeightedUnDirGraphMtEphFloat15,652
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges17,774
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge31,1342
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight36,1543
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges41,1750
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted50,2111
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight63,2668
        pub fn vertex_degree(&self, v: &V) -> usize {vertex_degree68,2904
    macro_rules! WeightedUnDirGraphMtEphFloatLit {WeightedUnDirGraphMtEphFloatLit74,3032
    pub fn __weighted_undir_graph_mt_float_macro_typecheck_exercise() {__weighted_undir_graph_mt_float_macro_typecheck_exercise86,3657

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphInt.rs,1187
pub mod WeightedUnDirGraphStEphInt {WeightedUnDirGraphStEphInt3,101
    pub type WeightedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;WeightedUnDirGraphStEphInt12,432
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphInt<V> {WeightedUnDirGraphStEphInt15,588
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,702
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) {add_weighted_edge31,1256
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> {get_edge_weight36,1443
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges41,1636
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted50,1977
        pub fn total_weight(&self) -> i32 {total_weight63,2514
        pub fn vertex_degree(&self, v: &V) -> usize {vertex_degree68,2700
        pub fn is_connected(&self) -> bool {is_connected73,2887
    macro_rules! WeightedUnDirGraphStEphIntLit {WeightedUnDirGraphStEphIntLit102,3951
    pub fn __weighted_undir_graph_st_int_macro_typecheck_exercise() {__weighted_undir_graph_st_int_macro_typecheck_exercise114,4547

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphStEph.rs,2635
pub mod UnDirGraphStEph {UnDirGraphStEph3,80
    pub struct UnDirGraphStEph<V: StT + Hash> {UnDirGraphStEph12,314
        V: Set<V>,V13,362
        E: Set<Edge<V>>,E14,381
    pub trait UnDirGraphStEphTrait<V: StT + Hash> {UnDirGraphStEphTrait17,413
        fn empty() -> UnDirGraphStEph<V>;empty20,557
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V>;FromSets23,707
        fn vertices(&self) -> &Set<V>;vertices26,870
        fn edges(&self) -> &Set<Edge<V>>;edges29,1001
        fn sizeV(&self) -> N;sizeV32,1135
        fn sizeE(&self) -> N;sizeE35,1257
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1379
        fn NG(&self, v: &V) -> Set<V>;NG41,1522
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1679
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident47,1829
        fn Degree(&self, v: &V) -> N;Degree50,1978
    impl<V: StT + Hash> UnDirGraphStEphTrait<V> for UnDirGraphStEph<V> {UnDirGraphStEph53,2023
        fn empty() -> UnDirGraphStEph<V> {empty54,2096
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E }FromSets60,2253
        fn vertices(&self) -> &Set<V> { &self.V }vertices61,2352
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges62,2402
        fn sizeV(&self) -> N { self.V.size() }sizeV63,2455
        fn sizeE(&self) -> N { self.E.size() }sizeE64,2502
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor66,2550
        fn NG(&self, v: &V) -> Set<V> {NG76,2874
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices88,3236
        fn Incident(&self, e: &Edge<V>, v: &V) -> B {Incident97,3508
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree105,3697
    impl<V: StT + Hash> Debug for UnDirGraphStEph<V> {UnDirGraphStEph108,3763
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt109,3818
    impl<V: StT + Hash> Display for UnDirGraphStEph<V> {UnDirGraphStEph117,4038
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} E={:?}", self.V, self.Efmt118,4095
    impl<V: StT + Hash> PartialEq for UnDirGraphStEph<V> {UnDirGraphStEph121,4202
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq122,4261
    impl<V: StT + Hash> Eq for UnDirGraphStEph<V> {}UnDirGraphStEph124,4353
    macro_rules! UnDirGraphStEphLit {UnDirGraphStEphLit127,4427
    fn _UnDirGraphStEphLit_type_checks() {_UnDirGraphStEphLit_type_checks145,5552
    pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise151,5815

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphInt.rs,1442
pub mod WeightedDirGraphStEphInt {WeightedDirGraphStEphInt3,99
    pub type WeightedDirGraphStEphInt<V> = LabDirGraphStEph<V, i32>;WeightedDirGraphStEphInt12,422
    impl<V: StT + Hash> WeightedDirGraphStEphInt<V> {WeightedDirGraphStEphInt15,572
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,678
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) {add_weighted_edge31,1222
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> {get_edge_weight36,1412
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges41,1610
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted50,1959
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted61,2372
        pub fn total_weight(&self) -> i32 {total_weight72,2782
        pub fn edges_above_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_above_weight77,2958
        pub fn edges_below_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_below_weight88,3403
    macro_rules! WeightedDirGraphStEphIntLit {WeightedDirGraphStEphIntLit100,3820
    fn _WeightedDirGraphStEphIntLit_type_checks() {_WeightedDirGraphStEphIntLit_type_checks112,4406
    pub fn __weighted_dir_graph_st_int_macro_typecheck_exercise() {__weighted_dir_graph_st_int_macro_typecheck_exercise118,4698

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphFloat.rs,1502
pub mod WeightedUnDirGraphStEphFloat {WeightedUnDirGraphStEphFloat29,1111
    pub type WeightedUnDirGraphStEphFloat<V> = LabUnDirGraphStEph<V, OrderedF64>;WeightedUnDirGraphStEphFloat38,1451
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphFloat<V> {WeightedUnDirGraphStEphFloat41,1623
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges43,1739
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge57,2307
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight62,2508
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges67,2715
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted76,3070
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight89,3621
        pub fn vertex_degree(&self, v: &V) -> usize {vertex_degree94,3857
        pub fn is_connected(&self) -> bool {is_connected99,4044
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge127,5122
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge134,5403
    macro_rules! WeightedUnDirGraphStEphFloatLit {WeightedUnDirGraphStEphFloatLit142,5670
    pub fn __weighted_undir_graph_st_float_macro_typecheck_exercise() {__weighted_undir_graph_st_float_macro_typecheck_exercise154,6295

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphInt.rs,1125
pub mod WeightedUnDirGraphMtEphInt {WeightedUnDirGraphMtEphInt3,100
    pub type WeightedUnDirGraphMtEphInt<V> = LabUnDirGraphMtEph<V, i32>;WeightedUnDirGraphMtEphInt12,447
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphInt<V> {WeightedUnDirGraphMtEphInt15,620
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,740
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) {add_weighted_edge31,1294
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> {get_edge_weight36,1481
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges41,1674
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted50,2021
        pub fn total_weight(&self) -> i32 {total_weight63,2564
        pub fn vertex_degree(&self, v: &V) -> usize {vertex_degree68,2750
    macro_rules! WeightedUnDirGraphMtEphIntLit {WeightedUnDirGraphMtEphIntLit74,2878
    pub fn __weighted_undir_graph_mt_int_macro_typecheck_exercise() {__weighted_undir_graph_mt_int_macro_typecheck_exercise86,3474

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphMtEph.rs,2619
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
        fn vertices(&self) -> &Set<V> {vertices55,1771
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> {labeled_edges59,1849
        fn edges(&self) -> Set<Edge<V>> {edges63,1949
        fn add_vertex(&mut self, v: V) {add_vertex71,2226
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge75,2315
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label86,2707
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge97,3147
        fn neighbors(&self, v: &V) -> Set<V> {neighbors107,3495
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge119,3934
    impl<V, L> Display for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph127,4354
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt132,4485
    impl<V, L> Debug for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph137,4647
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt142,4776
    macro_rules! LabUnDirGraphMtEphLit {LabUnDirGraphMtEphLit149,5006
    fn _LabUnDirGraphMtEphLit_type_checks() {_LabUnDirGraphMtEphLit_type_checks172,6212
    pub fn __lab_undir_graph_mt_macro_typecheck_exercise() {__lab_undir_graph_mt_macro_typecheck_exercise178,6491

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphFloat.rs,1253
pub mod WeightedDirGraphMtEphFloat {WeightedDirGraphMtEphFloat3,105
    pub type WeightedDirGraphMtEphFloat<V> = LabDirGraphMtEph<V, OrderedF64>;WeightedDirGraphMtEphFloat12,453
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphFloat<V> {WeightedDirGraphMtEphFloat15,636
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges17,750
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge31,1308
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight36,1512
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges41,1724
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted50,2093
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted61,2523
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight72,2950
    macro_rules! WeightedDirGraphMtEphFloatLit {WeightedDirGraphMtEphFloatLit78,3145
    pub fn __weighted_dir_graph_mt_float_macro_typecheck_exercise() {__weighted_dir_graph_mt_float_macro_typecheck_exercise90,3758

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphMtEph.rs,2723
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
        fn NG(&self, v: &V) -> Set<V> {NG75,2870
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices87,3238
        fn Incident(&self, e: &Edge<V>, v: &V) -> B {Incident96,3510
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree104,3699
    impl<V: StT + MtT + Hash> std::fmt::Debug for UnDirGraphMtEph<V> {UnDirGraphMtEph107,3765
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt108,3836
    impl<V: StT + MtT + Hash> std::fmt::Display for UnDirGraphMtEph<V> {UnDirGraphMtEph116,4076
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} E={fmt117,4149
    impl<V: StT + MtT + Hash> PartialEq for UnDirGraphMtEph<V> {UnDirGraphMtEph120,4276
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq121,4341
    impl<V: StT + MtT + Hash> Eq for UnDirGraphMtEph<V> {}UnDirGraphMtEph123,4433
    macro_rules! UnDirGraphMtEphLit {UnDirGraphMtEphLit126,4513
    fn _UnDirGraphMtEphLit_type_checks() {_UnDirGraphMtEphLit_type_checks144,5638
    pub fn __undirgraph_mt_macro_typecheck_exercise() {__undirgraph_mt_macro_typecheck_exercise150,5890

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphStEph.rs,2552
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
        fn vertices(&self) -> &Set<V> {vertices55,1667
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> {labeled_arcs59,1745
        fn arcs(&self) -> Set<Edge<V>> {arcs63,1843
        fn add_vertex(&mut self, v: V) {add_vertex71,2106
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc75,2195
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label81,2430
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc90,2731
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors99,3005
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors109,3327
    impl<V, L> Display for LabDirGraphStEph<V, L>LabDirGraphStEph120,3654
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt125,3791
    impl<V, L> Debug for LabDirGraphStEph<V, L>LabDirGraphStEph130,3950
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt135,4085
    macro_rules! LabDirGraphStEphLit {LabDirGraphStEphLit142,4311
    fn _LabDirGraphStEphLit_type_checks() {_LabDirGraphStEphLit_type_checks154,5103
    pub fn __lab_dir_graph_macro_typecheck_exercise() {__lab_dir_graph_macro_typecheck_exercise160,5374

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphStEph.rs,3465
pub mod DirGraphStEph {DirGraphStEph3,77
    pub struct DirGraphStEph<V: StT + Hash> {DirGraphStEph12,309
        V: Set<V>,V13,355
        A: Set<Edge<V>>,A14,374
    pub trait DirGraphStEphTrait<V: StT + Hash> {DirGraphStEphTrait17,406
        fn empty() -> DirGraphStEph<V>;empty20,548
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V>;FromSets23,696
        fn vertices(&self) -> &Set<V>;vertices26,857
        fn arcs(&self) -> &Set<Edge<V>>;arcs29,988
        fn sizeV(&self) -> N;sizeV32,1121
        fn sizeA(&self) -> N;sizeA35,1243
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1365
        fn NG(&self, v: &V) -> Set<V>;NG41,1508
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1665
        fn NPlus(&self, v: &V) -> Set<V>;NPlus47,1819
        fn NMinus(&self, v: &V) -> Set<V>;NMinus50,1957
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices53,2118
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices56,2297
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident59,2451
        fn Degree(&self, v: &V) -> N;Degree62,2603
        fn InDegree(&self, v: &V) -> N;InDegree65,2737
        fn OutDegree(&self, v: &V) -> N;OutDegree68,2873
    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {DirGraphStEph71,2921
        fn empty() -> DirGraphStEph<V> {empty72,2990
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }FromSets78,3143
        fn vertices(&self) -> &Set<V> { &self.V }vertices79,3238
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs80,3288
        fn sizeV(&self) -> N { self.V.size() }sizeV81,3340
        fn sizeA(&self) -> N { self.A.size() }sizeA82,3387
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor84,3435
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG94,3712
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices96,3769
        fn NPlus(&self, v: &V) -> Set<V> {NPlus105,4041
        fn NMinus(&self, v: &V) -> Set<V> {NMinus115,4323
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices125,4606
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices134,4888
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B {Incident143,5174
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree151,5366
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree152,5428
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree153,5493
    impl<V: StT + Hash> Debug for DirGraphStEph<V> {DirGraphStEph156,5565
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt157,5618
    impl<V: StT + Hash> Display for DirGraphStEph<V> {DirGraphStEph165,5836
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.Afmt166,5891
    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {DirGraphStEph169,5998
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq170,6055
    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}DirGraphStEph172,6147
    macro_rules! DirGraphStEphLit {DirGraphStEphLit175,6219
    fn _DirGraphStEphLit_type_checks() {_DirGraphStEphLit_type_checks192,7312
    pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise198,7567

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphMtEph.rs,3554
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
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG93,3702
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices95,3759
        fn NPlus(&self, v: &V) -> Set<V> {NPlus104,4031
        fn NMinus(&self, v: &V) -> Set<V> {NMinus114,4316
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices124,4602
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices133,4884
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B {Incident142,5170
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree150,5362
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree151,5424
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree152,5489
    impl<V: StT + MtT + Hash> std::fmt::Debug for DirGraphMtEph<V> {DirGraphMtEph155,5561
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt156,5630
    impl<V: StT + MtT + Hash> std::fmt::Display for DirGraphMtEph<V> {DirGraphMtEph164,5868
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} A={fmt165,5939
    impl<V: StT + MtT + Hash> PartialEq for DirGraphMtEph<V> {DirGraphMtEph168,6066
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq169,6129
    impl<V: StT + MtT + Hash> Eq for DirGraphMtEph<V> {}DirGraphMtEph171,6221
    macro_rules! DirGraphMtEphLit {DirGraphMtEphLit174,6299
    fn _DirGraphMtEphLit_type_checks() {_DirGraphMtEphLit_type_checks191,7392
    pub fn __dirgraph_mt_macro_typecheck_exercise() {__dirgraph_mt_macro_typecheck_exercise197,7636

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphFloat.rs,1906
pub mod WeightedDirGraphStEphFloat {WeightedDirGraphStEphFloat29,1076
    pub type WeightedDirGraphStEphFloat<V> = LabDirGraphStEph<V, OrderedF64>;WeightedDirGraphStEphFloat38,1408
    impl<V: StT + Hash> WeightedDirGraphStEphFloat<V> {WeightedDirGraphStEphFloat41,1574
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges43,1682
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge57,2240
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight62,2444
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges67,2656
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted76,3019
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted87,3446
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight98,3870
        pub fn edges_above_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_above_weight103,4096
        pub fn edges_below_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_below_weight114,4569
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge125,5028
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge132,5308
        pub fn scale_weights(&mut self, factor: OrderedFloat<f64>) {scale_weights139,5590
    macro_rules! WeightedDirGraphStEphFloatLit {WeightedDirGraphStEphFloatLit157,6237
    fn _WeightedDirGraphStEphFloatLit_type_checks() {_WeightedDirGraphStEphFloatLit_type_checks169,6850
    pub fn __weighted_dir_graph_st_float_macro_typecheck_exercise() {__weighted_dir_graph_st_float_macro_typecheck_exercise175,7152

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphInt.rs,1132
pub mod WeightedDirGraphMtEphInt {WeightedDirGraphMtEphInt3,98
    pub type WeightedDirGraphMtEphInt<V> = LabDirGraphMtEph<V, i32>;WeightedDirGraphMtEphInt12,437
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphInt<V> {WeightedDirGraphMtEphInt15,604
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,716
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) {add_weighted_edge31,1260
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> {get_edge_weight36,1450
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges41,1648
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted50,2003
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted61,2419
        pub fn total_weight(&self) -> i32 {total_weight72,2832
    macro_rules! WeightedDirGraphMtEphIntLit {WeightedDirGraphMtEphIntLit78,2977
    pub fn __weighted_dir_graph_mt_int_macro_typecheck_exercise() {__weighted_dir_graph_mt_int_macro_typecheck_exercise90,3563

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphStEph.rs,2611
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
        fn vertices(&self) -> &Set<V> {vertices55,1713
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> {labeled_edges59,1791
        fn edges(&self) -> Set<Edge<V>> {edges63,1891
        fn add_vertex(&mut self, v: V) {add_vertex71,2162
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge75,2251
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label86,2637
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge97,3078
        fn neighbors(&self, v: &V) -> Set<V> {neighbors108,3491
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge120,3924
    impl<V, L> Display for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph128,4344
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt133,4464
    impl<V, L> Debug for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph138,4626
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt143,4744
    macro_rules! LabUnDirGraphStEphLit {LabUnDirGraphStEphLit150,4974
    fn _LabUnDirGraphStEphLit_type_checks() {_LabUnDirGraphStEphLit_type_checks173,6180
    pub fn __lab_undir_graph_macro_typecheck_exercise() {__lab_undir_graph_macro_typecheck_exercise179,6459

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphMtEph.rs,2560
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
        fn vertices(&self) -> &Set<V> {vertices55,1725
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> {labeled_arcs59,1803
        fn arcs(&self) -> Set<Edge<V>> {arcs63,1901
        fn add_vertex(&mut self, v: V) {add_vertex71,2170
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc75,2259
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label81,2500
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc90,2801
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors99,3075
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors109,3400
    impl<V, L> Display for LabDirGraphMtEph<V, L>LabDirGraphMtEph120,3730
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt125,3853
    impl<V, L> Debug for LabDirGraphMtEph<V, L>LabDirGraphMtEph130,4012
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt135,4133
    macro_rules! LabDirGraphMtEphLit {LabDirGraphMtEphLit142,4359
    fn _LabDirGraphMtEphLit_type_checks() {_LabDirGraphMtEphLit_type_checks154,5151
    pub fn __lab_dir_graph_mt_macro_typecheck_exercise() {__lab_dir_graph_mt_macro_typecheck_exercise160,5422

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36Mt.rs,1405
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap11/FibonacciMt.rs,578
pub mod FibonacciMt {FibonacciMt4,131
    pub struct FibonacciMt;FibonacciMt7,198
    pub trait FibonacciMtTrait {FibonacciMtTrait9,227
        fn fib(n: N) -> N;fib10,260
    impl FibonacciMt {FibonacciMt13,294
        pub fn fib(n: N) -> N {fib14,317
    impl FibonacciMtTrait for FibonacciMt {FibonacciMt24,589
        fn fib(n: N) -> N {fib25,633
    mod tests {tests31,727
        fn fib_base_cases() {fib_base_cases36,848
        fn fib_small_values() {fib_small_values42,1001
        fn trait_and_inherent_agree() {trait_and_inherent_agree49,1205

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStPer.rs,3138
pub mod LinkedListStPer {LinkedListStPer3,64
    pub struct NodeP<T: StT> {NodeP7,144
        value: T,value8,175
        next: Option<Box<NodeP<T>>>,next9,193
    pub struct LinkedListStPerS<T: StT> {LinkedListStPerS13,258
        head: Option<Box<NodeP<T>>>,head14,300
        len: N,len15,337
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait18,360
        fn empty() -> Self;empty21,497
        fn new(length: N, init_value: T) -> Self;new24,627
        fn length(&self) -> N;length27,769
        fn nth(&self, index: N) -> &T;nth30,908
        fn isEmpty(&self) -> B;isEmpty33,1039
        fn isSingleton(&self) -> B;isSingleton36,1163
        fn singleton(item: T) -> Self;singleton39,1291
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set42,1475
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy47,1729
    impl<T: StT> LinkedListStPerS<T> {LinkedListStPerS50,1796
        fn push_front_node(&mut self, node: Box<NodeP<T>>) {push_front_node51,1835
        pub fn from_vec(v: Vec<T>) -> Self {from_vec58,2036
        pub fn iter<'a>(&'a self) -> LinkedListStPerIter<'a, T> {iter66,2301
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS73,2478
        fn empty() -> Self { LinkedListStPerS { head: None, len: 0 } }empty74,2545
        fn new(length: N, init_value: T) -> Self {new75,2616
        fn length(&self) -> N { self.len }length94,3297
        fn nth(&self, index: N) -> &T {nth95,3340
        fn isEmpty(&self) -> B {isEmpty107,3707
        fn isSingleton(&self) -> B {isSingleton114,3865
        fn singleton(item: T) -> Self {singleton121,4027
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set130,4276
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy155,5275
    impl<T: StT> std::fmt::Debug for LinkedListStPerS<T> {LinkedListStPerS190,6416
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt191,6475
    pub struct LinkedListStPerIter<'a, T: StT> {LinkedListStPerIter202,6841
        cursor: Option<&'a NodeP<T>>,cursor203,6890
    impl<'a, T: StT> Iterator for LinkedListStPerIter<'a, T> {LinkedListStPerIter206,6935
        type Item = &'a T;Item207,6998
        fn next(&mut self) -> Option<Self::Item> {next208,7025
    impl<T: StT> PartialEq for LinkedListStPerS<T> {LinkedListStPerS218,7272
        fn eq(&self, other: &Self) -> bool {eq219,7325
    impl<T: StT> Eq for LinkedListStPerS<T> {}LinkedListStPerS236,7814
    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {LinkedListStPerS238,7862
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt239,7923
    macro_rules! NodePLit {NodePLit257,8451
    macro_rules! LinkedListStPerIterLit {LinkedListStPerIterLit267,8709
    fn _LinkedListStPer_struct_macro_checks() {_LinkedListStPer_struct_macro_checks276,8956
    macro_rules! LinkedListStPerSLit {LinkedListStPerSLit287,9320
    fn _LinkedListStPerSLit_type_checks() {_LinkedListStPerSLit_type_checks296,9816

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap03/InsertionSortSt.rs.saved,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap03/InsertionSortSt.rs,390
pub mod InsertionSortSt {InsertionSortSt3,51
    pub trait InsertionSortStTrait {InsertionSortStTrait5,78
        fn insSort<T: Ord + Clone>(&self, slice: &mut [T]);insSort8,214
    pub struct InsertionSortSt;InsertionSortSt12,317
    impl InsertionSortStTrait for InsertionSortSt {InsertionSortSt14,350
        fn insSort<T: Ord + Clone>(&self, slice: &mut [T]) {insSort15,402

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPerChap18.base.rs,3025
pub mod ArraySeqStPerChap {ArraySeqStPerChap3,47
    pub trait ArraySeqStPerChap18Trait<T: StT> {ArraySeqStPerChap18Trait7,156
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,387
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map14,625
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append18,829
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1113
        fn update(a: &ArrayStPerS<T>, item_at: Pair<N, T>) -> ArrayStPerS<T>;update26,1295
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject30,1534
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject34,1751
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate37,1922
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes40,2052
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce43,2254
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan46,2379
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten49,2543
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect52,2695
    impl<T: StT> ArraySeqStPerChap18Trait<T> for ArrayStPerS<T> {ArrayStPerS55,2818
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate56,2884
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map60,3063
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append72,3559
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter86,4083
        fn update(a: &ArrayStPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayStPerS<T> {update95,4418
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject101,4651
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject113,5215
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate123,5657
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes130,5889
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce139,6292
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec140,6370
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan155,6870
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec156,6964
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten177,7724
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect187,8092

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,6136
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
    pub trait StT: Eq + Clone + Display + Debug + Sized {}StT44,1182
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}T45,1241
    pub trait StTInMtT: StT + Send + Sync {}StTInMtT48,1405
    impl<T> StTInMtT for T where T: StT + Send + Sync {}T49,1450
    pub trait MtT: Sized + Send + Sync {MtT53,1644
        type Inner: StT;Inner54,1685
        fn clone_mt(&self) -> Self;clone_mt55,1710
        fn new_mt(inner: Self::Inner) -> Self;new_mt56,1746
    impl<T: StT + Send> MtT for std::sync::Mutex<T> {Mutex59,1800
        type Inner = T;Inner60,1854
        fn clone_mt(&self) -> Self {clone_mt61,1878
        fn new_mt(inner: Self::Inner) -> Self { std::sync::Mutex::new(inner) }new_mt65,2020
    impl<A: StT + Send + Sync, B: StT + Send + Sync> MtT for Pair<A, B> {Pair68,2106
        type Inner = Pair<A, B>;Inner69,2180
        fn clone_mt(&self) -> Self { self.clone() }clone_mt70,2213
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt71,2265
    impl MtT for usize {usize75,2406
        type Inner = usize;Inner76,2431
        fn clone_mt(&self) -> Self { *self }clone_mt77,2459
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt78,2504
    impl MtT for isize {isize81,2567
        type Inner = isize;Inner82,2592
        fn clone_mt(&self) -> Self { *self }clone_mt83,2620
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt84,2665
    impl MtT for i32 {i3287,2728
        type Inner = i32;Inner88,2751
        fn clone_mt(&self) -> Self { *self }clone_mt89,2777
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt90,2822
    impl MtT for u32 {u3293,2885
        type Inner = u32;Inner94,2908
        fn clone_mt(&self) -> Self { *self }clone_mt95,2934
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt96,2979
    impl MtT for i64 {i6499,3042
        type Inner = i64;Inner100,3065
        fn clone_mt(&self) -> Self { *self }clone_mt101,3091
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt102,3136
    impl MtT for u64 {u64105,3199
        type Inner = u64;Inner106,3222
        fn clone_mt(&self) -> Self { *self }clone_mt107,3248
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt108,3293
    impl MtT for bool {bool111,3356
        type Inner = bool;Inner112,3380
        fn clone_mt(&self) -> Self { *self }clone_mt113,3407
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt114,3452
    impl MtT for char {char117,3515
        type Inner = char;Inner118,3539
        fn clone_mt(&self) -> Self { *self }clone_mt119,3566
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt120,3611
    impl MtT for String {String124,3728
        type Inner = String;Inner125,3754
        fn clone_mt(&self) -> Self { self.clone() }clone_mt126,3783
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt127,3835
    impl<'a> MtT for &'a str {str131,3933
        type Inner = &'a str;Inner132,3964
        fn clone_mt(&self) -> Self { *self }clone_mt133,3994
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt134,4039
    impl MtT for B {B138,4144
        type Inner = B;Inner139,4165
        fn clone_mt(&self) -> Self { *self }clone_mt140,4189
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt141,4234
    pub struct Edge<V: StT>(pub V, pub V);Edge146,4436
    impl<V: StT> std::fmt::Display for Edge<V> {Edge148,4480
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})fmt149,4529
    impl<V: StT> From<(V, V)> for Edge<V> {Edge152,4653
        fn from(t: (V, V)) -> Self { Edge(t.0, t.1) }from153,4697
    impl<V: StT> From<Edge<V>> for (V, V) {V156,4758
        fn from(e: Edge<V>) -> (V, V) { (e.0, e.1) }from157,4802
    pub struct LabEdge<V: StT, L: StT + Hash>(pub V, pub V, pub L);LabEdge162,4975
    impl<V: StT, L: StT + Hash> std::fmt::Display for LabEdge<V, L> {LabEdge164,5044
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { fmt165,5114
    impl<V: StT, L: StT + Hash> From<(V, V, L)> for LabEdge<V, L> {LabEdge170,5272
        fn from(t: (V, V, L)) -> Self { LabEdge(t.0, t.1, t.2) }from171,5340
    impl<V: StT, L: StT + Hash> From<LabEdge<V, L>> for (V, V, L) {L174,5412
        fn from(e: LabEdge<V, L>) -> (V, V, L) { (e.0, e.1, e.2) }from175,5480
    pub type OrderedF32 = OrderedFloat<f32>;OrderedF32182,5707
    pub type OrderedF64 = OrderedFloat<f64>;OrderedF64183,5752
    pub struct Pair<A, B>(pub A, pub B);Pair187,5952
    impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {Pair189,5994
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})fmt190,6082
    impl<A, B> From<(A, B)> for Pair<A, B> {Pair193,6206
        fn from(t: (A, B)) -> Self { Pair(t.0, t.1) }from194,6251
    impl<A, B> From<Pair<A, B>> for (A, B) {B197,6312
        fn from(p: Pair<A, B>) -> Self { (p.0, p.1) }from198,6357
    macro_rules! ParaPair {ParaPair202,6438
    fn _ParaPair_type_checks() {_ParaPair_type_checks213,6912
    pub fn ArraySeqSetEq<T: PartialEq>(a_len: N, a_nth: impl Fn(N) -> T, b_len: N, b_nth: impl FArraySeqSetEq222,7332
    macro_rules! EdgeLit {EdgeLit261,8390
    macro_rules! PairLit {PairLit268,8534
    macro_rules! EdgeList {EdgeList275,8678
    macro_rules! PairList {PairList285,8895
    fn _EdgeLit_type_checks() {_EdgeLit_type_checks295,9116
    fn _PairLit_type_checks() {_PairLit_type_checks301,9303
    fn _EdgeList_type_checks() {_EdgeList_type_checks307,9495
    fn _PairList_type_checks() {_PairList_type_checks313,9704

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_5.rs,1372
pub mod Exercise12_5 {Exercise12_53,86
    struct Node<T: StTInMtT> {Node10,260
        value: ManuallyDrop<T>,value11,291
        next: *mut Node<T>,next12,323
    pub struct ConcurrentStackMt<T: StTInMtT> {ConcurrentStackMt16,419
        head: AtomicPtr<Node<T>>,head17,467
    pub trait ConcurrentStackMtTrait<T: StTInMtT> {ConcurrentStackMtTrait20,508
        fn new() -> Self;new21,560
        fn push(&self, value: T);push22,586
        fn pop(&self) -> Option<T>;pop23,620
        fn is_empty(&self) -> bool;is_empty24,656
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt27,699
        fn raw_pop(&self) -> Option<*mut Node<T>> {raw_pop28,744
    impl<T: StTInMtT> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {ConcurrentStackMt46,1293
        fn new() -> Self {new47,1368
        fn push(&self, value: T) {push51,1473
        fn pop(&self) -> Option<T> {pop68,2110
        fn is_empty(&self) -> bool {is_empty75,2349
    impl<T: StTInMtT> Default for ConcurrentStackMt<T> {ConcurrentStackMt80,2459
        fn default() -> Self { Self::new() }default81,2516
    impl<T: StTInMtT> Drop for ConcurrentStackMt<T> {ConcurrentStackMt84,2568
        fn drop(&mut self) {drop85,2622
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt98,3026
        pub fn drain(&self) -> Vec<T> {drain100,3146

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
    pub struct SpinLock {SpinLock12,277
        ticket: AtomicUsize,ticket13,303
        turn: AtomicUsize,turn14,332
    pub trait SpinLockTrait {SpinLockTrait17,366
        fn new() -> Self;new18,396
        fn lock(&self);lock19,422
        fn unlock(&self);unlock20,446
    impl SpinLock {SpinLock23,479
        pub fn new() -> Self {new24,499
        pub fn lock(&self) {lock28,621
        pub fn unlock(&self) {unlock35,844
        pub fn with_lock<T>(&self, action: impl FnOnce() -> T) -> T {with_lock39,941
    impl SpinLockTrait for SpinLock {SpinLock47,1134
        fn new() -> Self { SpinLock::new() }new48,1172
        fn lock(&self) { SpinLock::lock(self) }lock50,1218
        fn unlock(&self) { SpinLock::unlock(self) }unlock52,1267
    impl Default for SpinLock {SpinLock55,1326
        fn default() -> Self { SpinLock::new() }default56,1358
    pub fn parallel_increment(iterations: N) -> usize {parallel_increment59,1414

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTTreapMtEph.rs,4319
pub mod BSTTreapMtEph {BSTTreapMtEph3,100
    type Link<T> = Option<Box<Node<T>>>;Link11,330
    struct Node<T: StTInMtT + Ord> {Node14,393
        key: T,key15,430
        priority: u64,priority16,446
        size: N,size17,469
        left: Link<T>,left18,486
        right: Link<T>,right19,509
    impl<T: StTInMtT + Ord> Node<T> {Node22,540
        fn new(key: T, priority: u64) -> Self {new23,578
    pub struct BSTTreapMtEph<T: StTInMtT + Ord> {BSTTreapMtEph35,826
        root: Arc<RwLock<Link<T>>>,root36,876
    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;BSTreeTreap39,919
    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTTreapMtEphTrait41,968
        fn new() -> Self;new42,1029
        fn insert(&self, value: T);insert43,1055
        fn find(&self, target: &T) -> Option<T>;find44,1091
        fn contains(&self, target: &T) -> B;contains45,1140
        fn size(&self) -> N;size46,1185
        fn is_empty(&self) -> B;is_empty47,1214
        fn height(&self) -> N;height48,1247
        fn minimum(&self) -> Option<T>;minimum49,1278
        fn maximum(&self) -> Option<T>;maximum50,1318
        fn in_order(&self) -> ArrayStPerS<T>;in_order51,1358
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order52,1404
    impl<T: StTInMtT + Ord> Default for BSTTreapMtEph<T> {BSTTreapMtEph55,1458
        fn default() -> Self { Self::new() }default56,1517
    impl<T: StTInMtT + Ord> BSTTreapMtEph<T> {BSTTreapMtEph59,1569
        pub fn new() -> Self {new60,1616
        pub fn size(&self) -> N {size66,1751
        pub fn is_empty(&self) -> B {is_empty71,1884
        pub fn height(&self) -> N {height79,2051
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec80,2087
        pub fn insert(&self, value: T) {insert91,2437
        pub fn find(&self, target: &T) -> Option<T> {find97,2639
        pub fn contains(&self, target: &T) -> B {contains102,2809
        pub fn minimum(&self) -> Option<T> {minimum110,2999
        pub fn maximum(&self) -> Option<T> {maximum115,3151
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order120,3303
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order127,3582
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link134,3863
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate136,3946
        fn rotate_left(link: &mut Link<T>) {rotate_left138,4065
        fn rotate_right(link: &mut Link<T>) {rotate_right152,4520
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link166,4976
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link191,5996
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link206,6513
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link216,6834
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect226,7157
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect234,7444
    impl<T: StTInMtT + Ord> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {BSTTreapMtEph243,7740
        fn new() -> Self { BSTTreapMtEph::new() }new244,7813
        fn insert(&self, value: T) { BSTTreapMtEph::insert(self, value) }insert246,7864
        fn find(&self, target: &T) -> Option<T> { BSTTreapMtEph::find(self, target) }find248,7939
        fn contains(&self, target: &T) -> B { BSTTreapMtEph::contains(self, target) }contains250,8026
        fn size(&self) -> N { BSTTreapMtEph::size(self) }size252,8113
        fn is_empty(&self) -> B { BSTTreapMtEph::is_empty(self) }is_empty254,8172
        fn height(&self) -> N { BSTTreapMtEph::height(self) }height256,8239
        fn minimum(&self) -> Option<T> { BSTTreapMtEph::minimum(self) }minimum258,8302
        fn maximum(&self) -> Option<T> { BSTTreapMtEph::maximum(self) }maximum260,8375
        fn in_order(&self) -> ArrayStPerS<T> { BSTTreapMtEph::in_order(self) }in_order262,8448
        fn pre_order(&self) -> ArrayStPerS<T> { BSTTreapMtEph::pre_order(self) }pre_order264,8528
    macro_rules! BSTTreapMtEphLit {BSTTreapMtEphLit268,8636
    fn _BSTTreapMtEphLit_type_checks() {_BSTTreapMtEphLit_type_checks280,9133

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main7,236

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/MappingStEph.rs,2266
pub mod MappingStEph {MappingStEph3,72
    pub struct Mapping<A, B> {Mapping14,394
        rel: Relation<A, B>,rel15,425
    pub trait MappingStEphTrait<MappingStEphTrait18,461
        fn empty() -> Mapping<X, Y>;empty25,644
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y>;FromVec29,778
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation33,932
        fn size(&self) -> N;size37,1087
        fn domain(&self) -> Set<X>;domain41,1213
        fn range(&self) -> Set<Y>;range45,1346
        fn mem(&self, a: &X, b: &Y) -> B;mem49,1474
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>>;iter51,1517
    impl<A: Eq + Hash, B: Eq + Hash> Mapping<A, B> {Mapping54,1600
        fn unique_pairs_from_iter<I: IntoIterator<Item = Pair<A, B>>>(iter: I) -> Set<Pair<A, B>unique_pairs_from_iter55,1653
    impl<A: StT + Hash, B: StT + Hash>Mapping65,2031
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq68,2112
    impl<A: StT + Hash, B: StT + Hash> EqMapping70,2187
    impl<A: StT + Hash, B: StT + Hash> Debug for Mapping<A, B> {Mapping75,2268
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.rel, f) }fmt76,2333
    impl<A: StT + Hash, B: StT + Hash> Display for Mapping<A, B> {Mapping78,2423
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.rel, f) }fmt79,2490
        impl<Mapping82,2583
        fn empty() -> Mapping<X, Y> {empty87,2709
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y> {FromVec93,2870
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation100,3115
        fn size(&self) -> N { self.rel.size() }size107,3381
        fn domain(&self) -> Set<X> { self.rel.domain() }domain109,3430
        fn range(&self) -> Set<Y> { self.rel.range() }range111,3488
        fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem113,3544
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.rel.iter() }iter115,3609
    macro_rules! MappingLit {MappingLit119,3731
    fn _MappingLit_type_checks() {_MappingLit_type_checks130,4296
    pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise136,4527

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/SetStEph.rs,3507
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
        pub fn mem(&self, x: &T) -> B {mem118,3938
        pub fn union(&self, other: &Set<T>) -> Set<T>union126,4112
        pub fn intersection(&self, other: &Set<T>) -> Set<T>intersection137,4386
        pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition148,4743
        pub fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct166,5298
        pub fn insert(&mut self, x: T) -> &mut Self {insert180,5744
        pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter185,5867
        pub fn FromVec(v: Vec<T>) -> Set<T> {FromVec187,5959
    impl<T: StT + Hash> SetStEphTrait<T> for Set<T> {Set196,6183
        fn empty() -> Set<T> { Set { data: HashSet::new() } }empty197,6237
        fn singleton(x: T) -> Set<T> {singleton199,6300
        fn size(&self) -> N { self.data.len() }size205,6462
        fn mem(&self, x: &T) -> B {mem207,6511
        fn union(&self, other: &Set<T>) -> Set<T>union215,6681
        fn intersection(&self, other: &Set<T>) -> Set<T>intersection226,6951
        fn partition(&self, parts: &Set<Set<T>>) -> B {partition237,7304
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct255,7855
        fn insert(&mut self, x: T) -> &mut Self {insert269,8297
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter274,8416
        fn FromVec(v: Vec<T>) -> Set<T> {FromVec276,8504
    macro_rules! SetLit {SetLit286,8744
    fn _SetLit_type_checks() {_SetLit_type_checks298,9086
    pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise304,9282
        let _s0: Set<&'static str> = SetLit![];str305,9328

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/RelationStEph.rs,2199
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
        pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B> {FromVec54,1528
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Relation<A, B> {Relation59,1656
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq60,1726
    impl<A: StT + Hash, B: StT + Hash> Eq for Relation<A, B> {}Relation63,1806
    impl<A: StT + Hash, B: StT + Hash> Debug for Relation<A, B> {Relation65,1871
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }fmt66,1937
    impl<A: StT + Hash, B: StT + Hash> Display for Relation<A, B> {Relation69,2040
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) fmt70,2108
    impl<X: StT + Hash, Y: StT + Hash> RelationStEphTrait<X, Y> for Relation<X, Y> {Relation73,2213
        fn empty() -> Relation<X, Y> { Relation { pairs: SetLit![] } }empty74,2298
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y> { Relation { pairs } }FromSet76,2370
        fn size(&self) -> N { self.pairs.size() }size78,2455
        fn domain(&self) -> Set<X>domain80,2506
        fn range(&self) -> Set<Y>range91,2773
        fn mem(&self, a: &X, b: &Y) -> Bmem102,3039
        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }iter114,3315
    macro_rules! RelationLit {RelationLit118,3411
    fn _RelationLit_type_checks() {_RelationLit_type_checks134,4288
    pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise140,4523

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36St.rs,1495
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTSetTreapMtEph.rs,5634
pub mod BSTSetTreapMtEph {BSTSetTreapMtEph3,75
    pub struct BSTSetTreapMtEph<T: StTInMtT + Ord> {BSTSetTreapMtEph10,288
        tree: BSTTreapMtEph<T>,tree11,341
    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;BSTSetTreapMt14,380
    pub trait BSTSetTreapMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetTreapMtEphTrait16,434
        fn empty() -> Self;empty17,498
        fn singleton(value: T) -> Self;singleton18,526
        fn size(&self) -> N;size19,566
        fn is_empty(&self) -> B;is_empty20,595
        fn find(&self, value: &T) -> Option<T>;find21,628
        fn contains(&self, value: &T) -> B;contains22,676
        fn minimum(&self) -> Option<T>;minimum23,720
        fn maximum(&self) -> Option<T>;maximum24,760
        fn insert(&mut self, value: T);insert25,800
        fn delete(&mut self, target: &T);delete26,840
        fn union(&self, other: &Self) -> Self;union27,882
        fn intersection(&self, other: &Self) -> Self;intersection28,929
        fn difference(&self, other: &Self) -> Self;difference29,983
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1035
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1090
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1145
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1207
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1277
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1345
        fn as_tree(&self) -> &BSTTreapMtEph<T>;as_tree36,1396
    impl<T: StTInMtT + Ord> BSTSetTreapMtEph<T> {BSTSetTreapMtEph39,1451
        pub fn empty() -> Self {empty40,1501
        pub fn singleton(value: T) -> Self {singleton46,1622
        pub fn size(&self) -> N { self.tree.size() }size52,1781
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1835
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1897
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1975
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2053
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2121
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2189
        pub fn delete(&mut self, target: &T) {delete66,2262
        pub fn union(&self, other: &Self) -> Self {union74,2551
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2850
        pub fn difference(&self, other: &Self) -> Self {difference99,3428
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4005
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4697
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5010
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5366
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5775
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6039
        pub fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree178,6119
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6186
        fn rebuild_from_vec(values: Vec<T>) -> BSTTreapMtEph<T> {rebuild_from_vec182,6277
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6500
    impl<T: StTInMtT + Ord> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {BSTSetTreapMtEph202,6785
        fn empty() -> Self { Self::empty() }empty203,6864
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6910
        fn size(&self) -> N { self.tree.size() }size207,6977
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7027
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7085
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7159
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7233
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7297
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7361
        fn delete(&mut self, target: &T) { BSTSetTreapMtEph::delete(self, target) }delete221,7430
        fn union(&self, other: &Self) -> Self { BSTSetTreapMtEph::union(self, other) }union223,7515
        fn intersection(&self, other: &Self) -> Self { BSTSetTreapMtEph::intersection(self, otheintersection225,7603
        fn difference(&self, other: &Self) -> Self { BSTSetTreapMtEph::difference(self, other) }difference227,7705
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetTreapMtEph::split(self, pivot) }split229,7803
        fn join_pair(left: Self, right: Self) -> Self { BSTSetTreapMtEph::join_pair(left, right)join_pair231,7899
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetTreapMtEph::join_m(left, pijoin_m233,7999
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetTreapMtEph::filter(filter235,8110
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetTreapMtEph::reduce(sereduce237,8226
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8339
        fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree241,8415
    macro_rules! BSTSetTreapMtEphLit {BSTSetTreapMtEphLit245,8504
    fn _BSTSetTreapMtEphLit_type_checks() {_BSTSetTreapMtEphLit_type_checks257,9045

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStEphChap18.rs,2834
pub mod ArraySeqStEphChap {ArraySeqStEphChap3,51
    pub trait ArraySeqStEphChap18Trait<T: StT> {ArraySeqStEphChap18Trait7,168
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate10,309
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map13,473
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append16,670
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter19,852
        fn update(a: &mut ArraySeqStEphS<T>, item_at: (N, T)) -> &mut ArraySeqStEphS<T>;update22,1031
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject25,1240
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject28,1461
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate29,1563
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes30,1650
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce31,1766
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan32,1846
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten33,1945
        fn collect<A: StT, Bv: StT>(collect34,2026
    impl<T: StT> ArraySeqStEphChap18Trait<T> for ArraySeqStEphS<T> {ArraySeqStEphS40,2212
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate41,2281
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map48,2527
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append61,3066
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter78,3774
        fn update(a: &mut ArraySeqStEphS<T>, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update90,4199
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject93,4341
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject101,4667
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,4817
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes111,5052
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce124,5616
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan131,5845
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten145,6403
        fn collect<A: StT, Bv: StT>(collect173,7494

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/MathSeq.rs,4286
pub mod MathSeq {MathSeq8,306
    pub struct MathSeqS<T: StT> {MathSeqS17,588
        data: Vec<T>,data18,622
    impl<T: StT> PartialEq for MathSeqS<T> {MathSeqS21,651
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq22,696
    impl<T: StT> Eq for MathSeqS<T> {}MathSeqS25,774
    impl<T: StT> std::fmt::Debug for MathSeqS<T> {MathSeqS27,814
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt28,865
    impl<T: StT> std::fmt::Display for MathSeqS<T> {MathSeqS33,1021
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt34,1074
    impl<T: StT> MathSeqS<T> {MathSeqS49,1487
        pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter52,1610
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }iter_mut55,1777
        pub fn empty() -> Self { Self { data: Vec::new() } }empty59,1960
        pub fn singleton(item: T) -> Self { Self { data: vec![item] } }singleton62,2113
        pub fn from_vec(data: Vec<T>) -> Self { Self { data } }from_vec65,2287
        pub fn with_len(length: N, init_value: T) -> Self {with_len68,2453
    impl<'a, T: StT> IntoIterator for &'a MathSeqS<T> {MathSeqS75,2611
        type Item = &'a T;Item76,2667
        type IntoIter = std::slice::Iter<'a, T>;IntoIter77,2694
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }into_iter78,2743
    impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {MathSeqS81,2816
        type Item = &'a mut T;Item82,2876
        type IntoIter = std::slice::IterMut<'a, T>;IntoIter83,2907
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }into_iter84,2959
    impl<T: StT> IntoIterator for MathSeqS<T> {MathSeqS87,3036
        type Item = T;Item88,3084
        type IntoIter = std::vec::IntoIter<T>;IntoIter89,3107
        fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }into_iter90,3154
    pub trait MathSeqTrait<T: StT + Hash> {MathSeqTrait94,3268
        fn new(length: N, init_value: T) -> Self;new97,3414
        fn empty() -> Self;empty101,3557
        fn singleton(item: T) -> Self;singleton105,3678
        fn length(&self) -> N;length109,3810
        fn nth(&self, index: N) -> &T;nth113,3934
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str>;set117,4066
        fn add_last(&mut self, value: T) -> &mut Self;add_last121,4353
        fn delete_last(&mut self) -> Option<T>;delete_last125,4501
        fn subseq(&self, start: N, length: N) -> &[T];subseq129,4642
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy133,4800
        fn isEmpty(&self) -> B;isEmpty137,4953
        fn isSingleton(&self) -> B;isSingleton141,5078
        fn domain(&self) -> Vec<N>;domain145,5211
        fn range(&self) -> Vec<T>;range149,5344
        fn multiset_range(&self) -> Vec<(N, T)>;multiset_range153,5476
    impl<T: StT + Hash> MathSeqTrait<T> for MathSeqS<T> {MathSeqS156,5532
        fn new(length: N, init_value: T) -> Self {new159,5692
        fn empty() -> Self { MathSeqS { data: Vec::new() } }empty167,5931
        fn singleton(item: T) -> Self { MathSeqS { data: vec![item] } }singleton171,6085
        fn length(&self) -> N { self.data.len() }length175,6250
        fn nth(&self, index: N) -> &T { &self.data[index] }nth179,6393
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str> {set183,6546
        fn add_last(&mut self, value: T) -> &mut Self {add_last194,7030
        fn delete_last(&mut self) -> Option<T> { self.data.pop() }delete_last201,7241
        fn subseq(&self, start: N, length: N) -> &[T] {subseq205,7401
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy214,7727
        fn isEmpty(&self) -> B {isEmpty228,8196
        fn isSingleton(&self) -> B {isSingleton238,8454
        fn domain(&self) -> Vec<N> { (0..self.data.len()).collect() }domain248,8720
        fn range(&self) -> Vec<T> {range252,8887
        fn multiset_range(&self) -> Vec<(N, T)> {multiset_range265,9354
    macro_rules! MathSeqSLit {MathSeqSLit284,10038
    fn _MathSeqSLit_type_checks() {_MathSeqSLit_type_checks297,10432

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStPer.rs,2877
pub mod LinkedListStPer {LinkedListStPer3,48
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait7,136
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate10,349
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map14,592
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append18,798
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter22,1097
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;update26,1289
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject29,1455
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject32,1622
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate35,1808
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes38,1943
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce41,2147
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan44,2277
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten47,2451
        fn collect<A: StT, Bv: StT>(collect50,2618
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS56,2810
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate57,2877
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map65,3128
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append72,3575
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter84,4217
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T>update94,4687
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject102,5215
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject120,6182
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate137,7118
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes144,7455
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce153,8014
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan168,8835
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten185,9739
        fn collect<A: StT, Bv: StT>(collect197,10431

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtEphChap18.rs,1659
pub mod ArraySeqMtEphChap {ArraySeqMtEphChap3,67
    pub trait ArraySeqMtEphChap18Trait<T: StT> {ArraySeqMtEphChap18Trait7,184
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate8,233
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map9,301
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append10,390
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter11,476
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;update12,563
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject13,652
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject14,753
    impl<T: StT> ArraySeqMtEphChap18Trait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS17,862
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate18,931
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map25,1177
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append38,1688
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter55,2356
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update67,2781
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject71,2941
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject79,3266

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStEph.rs,2806
pub mod ArraySeqStEph{ArraySeqStEph3,51
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait7,139
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate10,274
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map13,438
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append16,635
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter19,817
        fn update(a: &mut ArraySeqStEphS<T>, item_at: (N, T)) -> &mut ArraySeqStEphS<T>;update22,996
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject25,1205
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject28,1426
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate29,1528
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes30,1615
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce31,1731
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan32,1811
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten33,1910
        fn collect<A: StT, Bv: StT>(collect34,1991
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS40,2177
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate41,2240
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map48,2486
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append61,3028
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter78,3742
        fn update(a: &mut ArraySeqStEphS<T>, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update90,4167
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject93,4309
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject101,4635
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,4785
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes111,5020
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce124,5587
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan131,5816
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten145,6377
        fn collect<A: StT, Bv: StT>(collect173,7463

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStPerChap18.rs,3025
pub mod ArraySeqStPerChap {ArraySeqStPerChap3,46
    pub trait ArraySeqStPerChap18Trait<T: StT> {ArraySeqStPerChap18Trait7,163
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,394
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map14,632
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append18,836
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1120
        fn update(a: &ArrayStPerS<T>, item_at: Pair<N, T>) -> ArrayStPerS<T>;update26,1302
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject30,1541
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject34,1758
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate37,1929
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes40,2059
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce43,2261
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan46,2386
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten49,2550
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect52,2702
    impl<T: StT> ArraySeqStPerChap18Trait<T> for ArrayStPerS<T> {ArrayStPerS55,2825
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate56,2891
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map60,3070
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append72,3571
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter86,4100
        fn update(a: &ArrayStPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayStPerS<T> {update95,4435
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject101,4668
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject113,5232
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate123,5674
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes130,5906
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce139,6309
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec140,6387
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan155,6897
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec156,6991
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten177,7761
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect187,8129

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtPerChap18.rs,2829
pub mod ArraySeqMtPerChap {ArraySeqMtPerChap3,82
    pub trait ArraySeqMtPerChap18Trait<T: MtT> {ArraySeqMtPerChap18Trait7,199
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;tabulate10,430
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;map13,667
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;append16,862
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;filter19,1145
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;update22,1326
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;inject25,1564
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;ninject28,1780
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1950
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes32,2079
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce34,2272
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan36,2396
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten38,2559
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect40,2710
    impl<T: MtT> ArraySeqMtPerChap18Trait<T> for ArrayMtPerS<T> {ArrayMtPerS43,2833
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate44,2899
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map49,3079
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append53,3273
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter66,3678
        fn update(a: &ArrayMtPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayMtPerS<T> {update76,4035
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {inject80,4196
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject93,4845
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,5280
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes112,5513
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce123,5952
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan134,6287
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten144,6671
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect155,7073

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtPer.rs,2802
pub mod ArraySeqMtPer{ArraySeqMtPer3,82
    pub trait ArraySeqMtPerTrait<T: MtT> {ArraySeqMtPerTrait7,170
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;tabulate10,395
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;map13,632
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;append16,827
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;filter19,1110
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;update22,1291
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;inject25,1529
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;ninject28,1745
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1915
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes32,2044
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce34,2237
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan36,2361
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten38,2524
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect40,2675
    impl<T: MtT> ArraySeqMtPerTrait<T> for ArrayMtPerS<T> {ArrayMtPerS43,2798
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate44,2858
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map49,3038
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append53,3226
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter66,3631
        fn update(a: &ArrayMtPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayMtPerS<T> {update76,3988
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {inject80,4149
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject93,4798
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,5233
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes112,5466
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce123,5905
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan134,6240
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten144,6624
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect155,7026

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtEph.rs,1632
pub mod ArraySeqMtEph{ArraySeqMtEph3,67
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait7,155
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate8,198
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map9,266
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append10,355
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter11,441
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;update12,528
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject13,617
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject14,718
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS17,827
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate18,890
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map25,1136
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append38,1650
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter55,2321
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update67,2746
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject71,2906
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject79,3231

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStEph.rs,2883
pub mod LinkedListStEph {LinkedListStEph3,60
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait9,184
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;tabulate12,321
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map15,487
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append18,688
        fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T>;filter21,876
        fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T>;update24,1075
        fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListinject27,1292
        fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedLisninject30,1519
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate31,1627
        fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes32,1716
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce33,1836
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan34,1918
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten35,2021
        fn collect<A: StT, Bv: StT>(collect36,2108
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS42,2300
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate43,2367
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map50,2617
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append57,2966
        fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter69,3559
        fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T> update79,3980
        fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListinject82,4169
        fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedLisninject100,5045
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate114,5755
        fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes121,6043
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce131,6518
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan146,7244
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten165,8044
        fn collect<A: StT, Bv: StT>(collect177,8736

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStPer.rs,2998
pub mod ArraySeqStPer{ArraySeqStPer3,46
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait7,134
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,359
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map14,597
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append18,801
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1085
        fn update(a: &ArrayStPerS<T>, item_at: Pair<N, T>) -> ArrayStPerS<T>;update26,1267
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject30,1506
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject34,1723
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate37,1894
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes40,2024
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce43,2226
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan46,2351
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten49,2515
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect52,2667
    impl<T: StT> ArraySeqStPerTrait<T> for ArrayStPerS<T> {ArrayStPerS55,2790
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate56,2850
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map60,3029
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append72,3530
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter86,4062
        fn update(a: &ArrayStPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayStPerS<T> {update95,4397
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject101,4630
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject113,5194
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate123,5636
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes130,5868
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce139,6271
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec140,6349
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan155,6865
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec156,6959
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten177,7735
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect187,8103

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTParaTreapMtEph.rs,4821
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
    fn tree_priority<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> i64 {tree_priority39,1055
    fn tree_size<T: StTInMtT + Ord>(tree: &ParamTreap<T>) -> N {tree_size44,1242
    fn make_node<T: StTInMtT + Ord>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreamake_node49,1412
    impl<T: StTInMtT + Ord + 'static> ParamTreap<T> {ParamTreap56,1737
        fn expose_internal(&self) -> Exposed<T> {expose_internal59,1882
        pub fn expose_with_priority(&self) -> Option<(ParamTreap<T>, T, i64, ParamTreap<T>)> {expose_with_priority69,2268
        fn join_with_priority(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) join_with_priority76,2708
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid99,3885
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner111,4357
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner132,5478
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner146,6228
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner162,6958
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner182,7874
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner202,8773
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel226,9872
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner236,10236
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel260,11305
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order270,11646
    pub trait ParamTreapTrait<T: StTInMtT + Ord + 'static>: Sized {ParamTreapTrait282,12048
        fn new() -> Self;new285,12207
        fn expose(&self) -> Exposed<T>;expose288,12324
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid291,12455
        fn size(&self) -> N;size294,12596
        fn is_empty(&self) -> B;is_empty297,12716
        fn insert(&self, key: T);insert300,12860
        fn delete(&self, key: &T);delete303,13005
        fn find(&self, key: &T) -> Option<T>;find306,13151
        fn split(&self, key: &T) -> (Self, B, Self);split309,13308
        fn join_pair(&self, other: Self) -> Self;join_pair312,13520
        fn union(&self, other: &Self) -> Self;union315,13695
        fn intersect(&self, other: &Self) -> Self;intersect318,13867
        fn difference(&self, other: &Self) -> Self;difference321,14043
        fn filter<F>(&self, predicate: F) -> Selffilter324,14200
        fn reduce<F>(&self, op: F, base: T) -> Treduce329,14424
        fn in_order(&self) -> ArrayStPerS<T>;in_order334,14640
    impl<T: StTInMtT + Ord + 'static> ParamTreapTrait<T> for ParamTreap<T> {ParamTreap337,14693
        fn new() -> Self {new340,14861
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose348,15080
        fn join_mid(exposed: Exposed<T>) -> Self { ParamTreap::join_mid(exposed) }join_mid352,15238
        fn size(&self) -> N { tree_size(self) }size356,15413
        fn is_empty(&self) -> B {is_empty360,15553
        fn insert(&self, key: T) {insert370,15827
        fn delete(&self, key: &T) {delete381,16343
        fn find(&self, key: &T) -> Option<T> {find391,16792
        fn split(&self, key: &T) -> (Self, B, Self) { ParamTreap::split_inner(self, key) }split404,17394
        fn join_pair(&self, other: Self) -> Self { ParamTreap::join_pair_inner(self.clone(), othjoin_pair408,17645
        fn union(&self, other: &Self) -> Self { ParamTreap::union_inner(self, other) }union412,17873
        fn intersect(&self, other: &Self) -> Self { ParamTreap::intersect_inner(self, other) }intersect416,18086
        fn difference(&self, other: &Self) -> Self { ParamTreap::difference_inner(self, other) }difference420,18307
        fn filter<F>(&self, predicate: F) -> Selffilter424,18510
        fn reduce<F>(&self, op: F, base: T) -> Treduce433,18812
        fn in_order(&self) -> ArrayStPerS<T> {in_order442,19105
    macro_rules! ParamTreapLit {ParamTreapLit450,19345
    fn _ParamTreapLit_type_checks() {_ParamTreapLit_type_checks462,19859

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,6290
pub mod Types;Types9,256
pub mod Chap03 {Chap0312,304
    pub mod InsertionSortSt;InsertionSortSt13,321
pub mod Chap3 {Chap317,385
    pub mod InsertionSortSt {InsertionSortSt20,449
        pub mod InsertionSortSt {InsertionSortSt23,548
pub mod Chap05 {Chap0529,673
    pub mod SetStEph;SetStEph30,690
    pub mod RelationStEph;RelationStEph31,712
    pub mod MappingStEph;MappingStEph32,739
pub mod Chap18 {Chap1835,768
    pub mod MathSeq;MathSeq36,785
    pub mod LinkedListStEph;LinkedListStEph37,806
    pub mod LinkedListStPer;LinkedListStPer38,835
pub mod MathSeq {MathSeq46,1182
    pub mod MathSeq {MathSeq49,1249
pub mod Chap37 {Chap3754,1332
    pub mod AVLTreeSeqStEph;AVLTreeSeqStEph55,1349
    pub mod AVLTreeSeqStPer;AVLTreeSeqStPer56,1378
    pub mod BSTAVLMtEph;BSTAVLMtEph57,1407
    pub mod BSTAVLStEph;BSTAVLStEph58,1432
    pub mod BSTBBAlphaMtEph;BSTBBAlphaMtEph59,1457
    pub mod BSTBBAlphaStEph;BSTBBAlphaStEph60,1486
    pub mod BSTParaMtEph;BSTParaMtEph61,1515
    pub mod BSTParaStEph;BSTParaStEph62,1541
    pub mod BSTPlainMtEph;BSTPlainMtEph63,1567
    pub mod BSTPlainStEph;BSTPlainStEph64,1594
    pub mod BSTRBMtEph;BSTRBMtEph65,1621
    pub mod BSTRBStEph;BSTRBStEph66,1645
    pub mod BSTSetAVLMtEph;BSTSetAVLMtEph67,1669
    pub mod BSTSetBBAlphaMtEph;BSTSetBBAlphaMtEph68,1697
    pub mod BSTSetPlainMtEph;BSTSetPlainMtEph69,1729
    pub mod BSTSetRBMtEph;BSTSetRBMtEph70,1759
    pub mod BSTSetSplayMtEph;BSTSetSplayMtEph71,1786
    pub mod BSTSplayMtEph;BSTSplayMtEph72,1816
    pub mod BSTSplayStEph;BSTSplayStEph73,1843
pub mod Chap19 {Chap1976,1873
    pub mod ArraySeqStPer;ArraySeqStPer77,1890
    pub mod ArraySeqStEph;ArraySeqStEph78,1917
    pub mod ArraySeqMtPer;ArraySeqMtPer79,1944
    pub mod ArraySeqMtEph;ArraySeqMtEph80,1971
    pub mod ArraySeqMtEphSlice;ArraySeqMtEphSlice81,1998
pub mod Chap5 {Chap5102,2479
    pub mod SetStEph {SetStEph107,2664
        pub mod SetStEph {SetStEph110,2742
    pub mod RelationStEph {RelationStEph115,2844
        pub mod RelationStEph {RelationStEph118,2937
    pub mod MappingStEph {MappingStEph123,3054
        pub mod MappingStEph {MappingStEph126,3144
pub mod Chap06 {Chap06132,3260
    pub mod DirGraphStEph;DirGraphStEph133,3277
    pub mod UnDirGraphStEph;UnDirGraphStEph134,3304
    pub mod LabDirGraphStEph;LabDirGraphStEph135,3333
    pub mod LabUnDirGraphStEph;LabUnDirGraphStEph136,3363
    pub mod DirGraphMtEph;DirGraphMtEph137,3395
    pub mod UnDirGraphMtEph;UnDirGraphMtEph138,3422
    pub mod LabDirGraphMtEph;LabDirGraphMtEph139,3451
    pub mod LabUnDirGraphMtEph;LabUnDirGraphMtEph140,3481
    pub mod WeightedDirGraphStEphInt;WeightedDirGraphStEphInt141,3513
    pub mod WeightedDirGraphStEphFloat;WeightedDirGraphStEphFloat142,3551
    pub mod WeightedDirGraphMtEphInt;WeightedDirGraphMtEphInt143,3591
    pub mod WeightedDirGraphMtEphFloat;WeightedDirGraphMtEphFloat144,3629
    pub mod WeightedUnDirGraphStEphInt;WeightedUnDirGraphStEphInt145,3669
    pub mod WeightedUnDirGraphStEphFloat;WeightedUnDirGraphStEphFloat146,3709
    pub mod WeightedUnDirGraphMtEphInt;WeightedUnDirGraphMtEphInt147,3751
    pub mod WeightedUnDirGraphMtEphFloat;WeightedUnDirGraphMtEphFloat148,3791
pub mod Chap6 {Chap6167,4980
    pub mod DirGraphStEph {DirGraphStEph185,6205
        pub mod DirGraphStEph {DirGraphStEph188,6298
    pub mod UnDirGraphStEph {UnDirGraphStEph193,6415
        pub mod UnDirGraphStEph {UnDirGraphStEph196,6514
    pub mod LabDirGraphStEph {LabDirGraphStEph201,6637
        pub mod LabDirGraphStEph {LabDirGraphStEph204,6739
    pub mod LabUnDirGraphStEph {LabUnDirGraphStEph209,6865
        pub mod LabUnDirGraphStEph {LabUnDirGraphStEph212,6973
    pub mod DirGraphMtEph {DirGraphMtEph217,7105
        pub mod DirGraphMtEph {DirGraphMtEph220,7198
    pub mod UnDirGraphMtEph {UnDirGraphMtEph225,7315
        pub mod UnDirGraphMtEph {UnDirGraphMtEph228,7414
    pub mod LabDirGraphMtEph {LabDirGraphMtEph233,7537
        pub mod LabDirGraphMtEph {LabDirGraphMtEph236,7639
    pub mod LabUnDirGraphMtEph {LabUnDirGraphMtEph241,7765
        pub mod LabUnDirGraphMtEph {LabUnDirGraphMtEph244,7873
    pub mod WeightedDirGraphStEphInt {WeightedDirGraphStEphInt249,8005
        pub mod WeightedDirGraphStEphInt {WeightedDirGraphStEphInt252,8131
    pub mod WeightedDirGraphStEphFloat {WeightedDirGraphStEphFloat257,8281
        pub mod WeightedDirGraphStEphFloat {WeightedDirGraphStEphFloat260,8413
    pub mod WeightedDirGraphMtEphInt {WeightedDirGraphMtEphInt265,8569
        pub mod WeightedDirGraphMtEphInt {WeightedDirGraphMtEphInt268,8695
    pub mod WeightedDirGraphMtEphFloat {WeightedDirGraphMtEphFloat273,8845
        pub mod WeightedDirGraphMtEphFloat {WeightedDirGraphMtEphFloat276,8977
    pub mod WeightedUnDirGraphStEphInt {WeightedUnDirGraphStEphInt281,9133
        pub mod WeightedUnDirGraphStEphInt {WeightedUnDirGraphStEphInt284,9265
    pub mod WeightedUnDirGraphStEphFloat {WeightedUnDirGraphStEphFloat289,9421
        pub mod WeightedUnDirGraphStEphFloat {WeightedUnDirGraphStEphFloat292,9559
    pub mod WeightedUnDirGraphMtEphInt {WeightedUnDirGraphMtEphInt297,9721
        pub mod WeightedUnDirGraphMtEphInt {WeightedUnDirGraphMtEphInt300,9853
    pub mod WeightedUnDirGraphMtEphFloat {WeightedUnDirGraphMtEphFloat305,10009
        pub mod WeightedUnDirGraphMtEphFloat {WeightedUnDirGraphMtEphFloat308,10147
pub mod Chap11 {Chap11314,10311
    pub mod FibonacciMt;FibonacciMt315,10328
pub mod Chap12 {Chap12319,10408
    pub mod Exercise12_1;Exercise12_1320,10425
    pub mod Exercise12_2;Exercise12_2321,10451
    pub mod Exercise12_5;Exercise12_5322,10477
pub mod LinkedListStPer;LinkedListStPer328,10668
pub mod LinkedListStEph;LinkedListStEph331,10746
pub mod Chapter36St;Chapter36St358,11998
pub mod Chapter36Mt;Chapter36Mt360,12063
pub mod Chapter36MtSlice;Chapter36MtSlice362,12128
pub mod BBTEph;BBTEph366,12236
pub mod BSTTreapStEph;BSTTreapStEph373,12719
pub mod BSTSetTreapMtEph;BSTSetTreapMtEph374,12742
pub mod BSTTreapMtEph;BSTTreapMtEph383,13506
pub mod BSTParaTreapMtEph;BSTParaTreapMtEph384,13529

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEph.rs,3348
pub mod LinkedListStEph {LinkedListStEph3,90
    pub struct NodeE<T: StT> {NodeE7,170
        value: T,value8,201
        next: Option<Box<NodeE<T>>>,next9,219
    pub struct LinkedListStEphS<T: StT> {LinkedListStEphS13,284
        head: Option<Box<NodeE<T>>>,head14,326
        len: N,len15,363
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait18,386
        fn empty() -> LinkedListStEphS<T>;empty21,523
        fn new(length: N, init_value: T) -> Self;new24,668
        fn length(&self) -> N;length27,810
        fn nth(&self, index: N) -> &T;nth30,949
        fn isEmpty(&self) -> B;isEmpty33,1080
        fn isSingleton(&self) -> B;isSingleton36,1204
        fn singleton(item: T) -> Self;singleton39,1332
        fn update(&mut self, item_at: Pair<N, T>) -> &mut Self;update42,1479
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set45,1651
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy48,1876
    impl<T: StT> LinkedListStEphS<T> {LinkedListStEphS51,1943
        fn push_front_node(&mut self, node: Box<NodeE<T>>) {push_front_node54,2074
        pub fn from_vec(v: Vec<T>) -> Self {from_vec63,2371
        pub fn iter<'a>(&'a self) -> LinkedListStEphIter<'a, T> {iter73,2728
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS80,2905
        fn empty() -> Self { LinkedListStEphS { head: None, len: 0 } }empty83,3064
        fn new(length: N, init_value: T) -> Self {new86,3237
        fn length(&self) -> N { self.len }length107,4010
        fn nth(&self, index: N) -> &T {nth110,4161
        fn isEmpty(&self) -> B {isEmpty124,4620
        fn isSingleton(&self) -> B {isSingleton133,4870
        fn singleton(item: T) -> Self {singleton142,5124
        fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {update153,5481
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set168,5992
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy183,6578
    impl<T: StT> std::fmt::Debug for LinkedListStEphS<T> {LinkedListStEphS219,7760
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt220,7819
    pub struct LinkedListStEphIter<'a, T: StT> {LinkedListStEphIter231,8185
        cursor: Option<&'a NodeE<T>>,cursor232,8234
    impl<'a, T: StT> Iterator for LinkedListStEphIter<'a, T> {LinkedListStEphIter235,8279
        type Item = &'a T;Item236,8342
        fn next(&mut self) -> Option<Self::Item> {next237,8369
    impl<T: StT> PartialEq for LinkedListStEphS<T> {LinkedListStEphS247,8616
        fn eq(&self, other: &Self) -> bool {eq248,8669
    impl<T: StT> Eq for LinkedListStEphS<T> {}LinkedListStEphS265,9158
    impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {LinkedListStEphS267,9206
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt268,9267
    macro_rules! NodeELit {NodeELit286,9795
    macro_rules! LinkedListStEphIterLit {LinkedListStEphIterLit296,10053
    fn _LinkedListStEph_struct_macro_checks() {_LinkedListStEph_struct_macro_checks305,10300
    macro_rules! LinkedListStEphSLit {LinkedListStEphSLit316,10664
    fn _LinkedListStEphSLit_type_checks() {_LinkedListStEphSLit_type_checks338,11962

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetAVLMtEph.rs,5580
pub mod BSTSetAVLMtEph {BSTSetAVLMtEph3,73
    pub struct BSTSetAVLMtEph<T: StTInMtT + Ord> {BSTSetAVLMtEph10,286
        tree: BSTAVLMtEph<T>,tree11,337
    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;BSTSetAVLMt14,374
    pub trait BSTSetAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetAVLMtEphTrait16,424
        fn empty() -> Self;empty17,486
        fn singleton(value: T) -> Self;singleton18,514
        fn size(&self) -> N;size19,554
        fn is_empty(&self) -> B;is_empty20,583
        fn find(&self, value: &T) -> Option<T>;find21,616
        fn contains(&self, value: &T) -> B;contains22,664
        fn minimum(&self) -> Option<T>;minimum23,708
        fn maximum(&self) -> Option<T>;maximum24,748
        fn insert(&mut self, value: T);insert25,788
        fn delete(&mut self, target: &T);delete26,828
        fn union(&self, other: &Self) -> Self;union27,870
        fn intersection(&self, other: &Self) -> Self;intersection28,917
        fn difference(&self, other: &Self) -> Self;difference29,971
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1023
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1078
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1133
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1195
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1265
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1333
        fn as_tree(&self) -> &BSTAVLMtEph<T>;as_tree36,1384
    impl<T: StTInMtT + Ord> BSTSetAVLMtEph<T> {BSTSetAVLMtEph39,1437
        pub fn empty() -> Self {empty40,1485
        pub fn singleton(value: T) -> Self {singleton46,1604
        pub fn size(&self) -> N { self.tree.size() }size52,1761
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1815
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1877
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1955
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2033
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2101
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2169
        pub fn delete(&mut self, target: &T) {delete66,2242
        pub fn union(&self, other: &Self) -> Self {union74,2531
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2830
        pub fn difference(&self, other: &Self) -> Self {difference99,3408
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,3985
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4677
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,4990
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5346
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5755
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6019
        pub fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree178,6099
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6164
        fn rebuild_from_vec(values: Vec<T>) -> BSTAVLMtEph<T> {rebuild_from_vec182,6255
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6474
    impl<T: StTInMtT + Ord> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {BSTSetAVLMtEph202,6757
        fn empty() -> Self { Self::empty() }empty203,6832
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6878
        fn size(&self) -> N { self.tree.size() }size207,6945
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,6995
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7053
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7127
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7201
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7265
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7329
        fn delete(&mut self, target: &T) { BSTSetAVLMtEph::delete(self, target) }delete221,7398
        fn union(&self, other: &Self) -> Self { BSTSetAVLMtEph::union(self, other) }union223,7481
        fn intersection(&self, other: &Self) -> Self { BSTSetAVLMtEph::intersection(self, other)intersection225,7567
        fn difference(&self, other: &Self) -> Self { BSTSetAVLMtEph::difference(self, other) }difference227,7667
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetAVLMtEph::split(self, pivot) }split229,7763
        fn join_pair(left: Self, right: Self) -> Self { BSTSetAVLMtEph::join_pair(left, right) }join_pair231,7857
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetAVLMtEph::join_m(left, pivojoin_m233,7955
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetAVLMtEph::filter(sefilter235,8064
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetAVLMtEph::reduce(selfreduce237,8178
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8289
        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree241,8365
    macro_rules! BSTSetAVLMtEphLit {BSTSetAVLMtEphLit245,8452
    fn _BSTSetAVLMtEphLit_type_checks() {_BSTSetAVLMtEphLit_type_checks257,8999

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStPer.rs,1394
pub mod AVLTreeSeqStPer {AVLTreeSeqStPer3,49
    pub trait AVLTreeSeqStPerTrait<T: StT> {AVLTreeSeqStPerTrait10,235
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T>;tabulate11,280
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U>;map12,350
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T>;select13,443
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T>;append14,531
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T>;deflate15,623
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T>;filter16,694
    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS19,789
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T> {tabulate20,856
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U> {map23,1014
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T> {select26,1190
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T> {append40,1849
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T> {deflate43,2027
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T> {filter50,2338

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetPlainMtEph.rs,5266
pub mod BSTSetPlainMtEph {BSTSetPlainMtEph3,75
    pub struct BSTSetPlainMtEph<T: StTInMtT + Ord> {BSTSetPlainMtEph10,296
        tree: BSTPlainMtEph<T>,tree11,349
    pub type BSTSetPlainMt<T> = BSTSetPlainMtEph<T>;BSTSetPlainMt14,388
    pub trait BSTSetPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetPlainMtEphTrait16,442
        fn empty() -> Self;empty17,506
        fn singleton(value: T) -> Self;singleton18,534
        fn size(&self) -> N;size19,574
        fn is_empty(&self) -> B;is_empty20,603
        fn find(&self, value: &T) -> Option<T>;find21,636
        fn contains(&self, value: &T) -> B;contains22,684
        fn minimum(&self) -> Option<T>;minimum23,728
        fn maximum(&self) -> Option<T>;maximum24,768
        fn insert(&mut self, value: T);insert25,808
        fn delete(&mut self, target: &T);delete26,848
        fn union(&self, other: &Self) -> Self;union27,890
        fn intersection(&self, other: &Self) -> Self;intersection28,937
        fn difference(&self, other: &Self) -> Self;difference29,991
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1043
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1098
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1153
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1215
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1285
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1353
        fn as_tree(&self) -> &BSTPlainMtEph<T>;as_tree36,1404
    impl<T: StTInMtT + Ord> BSTSetPlainMtEph<T> {BSTSetPlainMtEph39,1459
        pub fn empty() -> Self {empty40,1509
        pub fn singleton(value: T) -> Self {singleton46,1630
        pub fn size(&self) -> N { self.tree.size() }size52,1789
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1843
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1905
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1983
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2061
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2129
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2197
        pub fn delete(&mut self, target: &T) {delete66,2270
        pub fn union(&self, other: &Self) -> Self {union74,2559
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2858
        pub fn difference(&self, other: &Self) -> Self {difference99,3436
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4013
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4705
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5018
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5374
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5783
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6047
        pub fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree178,6127
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6194
        fn rebuild_from_vec(values: Vec<T>) -> BSTPlainMtEph<T> {rebuild_from_vec182,6285
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6508
    impl<T: StTInMtT + Ord> BSTSetPlainMtEphTrait<T> for BSTSetPlainMtEph<T> {BSTSetPlainMtEph202,6793
        fn empty() -> Self { Self::empty() }empty203,6872
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6918
        fn size(&self) -> N { self.tree.size() }size207,6985
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7035
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7093
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7167
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7241
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7305
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7369
        fn delete(&mut self, target: &T) {delete221,7438
        fn union(&self, other: &Self) -> Self {union229,7723
        fn intersection(&self, other: &Self) -> Self {intersection237,8018
        fn difference(&self, other: &Self) -> Self {difference254,8592
        fn split(&self, pivot: &T) -> (Self, B, Self) {split271,9165
        fn join_pair(left: Self, right: Self) -> Self {join_pair291,9853
        fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m299,10162
        fn filter<F>(&self, predicate: F) -> Selffilter308,10514
        fn reduce<F>(&self, op: F, base: T) -> Treduce315,10675
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order322,10833
        fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree324,10909
    macro_rules! BSTSetPlainMtEphLit {BSTSetPlainMtEphLit328,10998
    fn _BSTSetPlainMtEphLit_type_checks() {_BSTSetPlainMtEphLit_type_checks340,11571

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBStEph.rs,4614
pub mod BSTRBStEph {BSTRBStEph3,93
    enum Color {Color9,301
        Red,Red10,318
        Black,Black11,331
    type Link<T> = Option<Box<Node<T>>>;Link14,353
    struct Node<T: StT + Ord> {Node17,416
        key: T,key18,448
        color: Color,color19,464
        size: N,size20,486
        left: Link<T>,left21,503
        right: Link<T>,right22,526
    impl<T: StT + Ord> Node<T> {Node25,557
        fn new(key: T) -> Self {new26,590
    pub struct BSTRBStEph<T: StT + Ord> {BSTRBStEph37,811
        root: Link<T>,root38,853
    pub type BSTreeRB<T> = BSTRBStEph<T>;BSTreeRB41,883
    pub trait BSTRBStEphTrait<T: StT + Ord> {BSTRBStEphTrait43,926
        fn new() -> Self;new44,972
        fn size(&self) -> N;size45,998
        fn is_empty(&self) -> B;is_empty46,1027
        fn height(&self) -> N;height47,1060
        fn insert(&mut self, value: T);insert48,1091
        fn find(&self, target: &T) -> Option<&T>;find49,1131
        fn contains(&self, target: &T) -> B;contains50,1181
        fn minimum(&self) -> Option<&T>;minimum51,1226
        fn maximum(&self) -> Option<&T>;maximum52,1267
        fn in_order(&self) -> ArrayStPerS<T>;in_order53,1308
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order54,1354
    impl<T: StT + Ord> Default for BSTRBStEph<T> {BSTRBStEph57,1408
        fn default() -> Self { Self::new() }default58,1459
    impl<T: StT + Ord> BSTRBStEph<T> {BSTRBStEph61,1511
        pub fn new() -> Self { BSTRBStEph { root: None } }new62,1550
        pub fn size(&self) -> N { Self::size_link(&self.root) }size64,1610
        pub fn is_empty(&self) -> B {is_empty66,1675
        pub fn height(&self) -> N {height74,1842
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec75,1878
        pub fn insert(&mut self, value: T) {insert84,2174
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find91,2394
        pub fn contains(&self, target: &T) -> B {contains93,2488
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum101,2678
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum103,2754
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order105,2830
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order111,3048
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red117,3268
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link119,3370
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate121,3453
        fn rotate_left(link: &mut Link<T>) {rotate_left123,3572
        fn rotate_right(link: &mut Link<T>) {rotate_right140,4168
        fn flip_colors(link: &mut Link<T>) {flip_colors157,4766
        fn fix_up(link: &mut Link<T>) {fix_up178,5571
        fn insert_link(link: &mut Link<T>, value: T) {insert_link195,6258
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link211,6800
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link226,7317
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link236,7638
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect246,7961
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect254,8248
    impl<T: StT + Ord> BSTRBStEphTrait<T> for BSTRBStEph<T> {BSTRBStEph263,8544
        fn new() -> Self { BSTRBStEph::new() }new264,8606
        fn size(&self) -> N { BSTRBStEph::size(self) }size266,8654
        fn is_empty(&self) -> B { BSTRBStEph::is_empty(self) }is_empty268,8710
        fn height(&self) -> N { BSTRBStEph::height(self) }height270,8774
        fn insert(&mut self, value: T) { BSTRBStEph::insert(self, value) }insert272,8834
        fn find(&self, target: &T) -> Option<&T> { BSTRBStEph::find(self, target) }find274,8910
        fn contains(&self, target: &T) -> B { BSTRBStEph::contains(self, target) }contains276,8995
        fn minimum(&self) -> Option<&T> { BSTRBStEph::minimum(self) }minimum278,9079
        fn maximum(&self) -> Option<&T> { BSTRBStEph::maximum(self) }maximum280,9150
        fn in_order(&self) -> ArrayStPerS<T> { BSTRBStEph::in_order(self) }in_order282,9221
        fn pre_order(&self) -> ArrayStPerS<T> { BSTRBStEph::pre_order(self) }pre_order284,9298
    macro_rules! BSTRBStEphLit {BSTRBStEphLit288,9403
    fn _BSTRBStEphLit_type_checks() {_BSTRBStEphLit_type_checks300,9897

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetSplayMtEph.rs,5634
pub mod BSTSetSplayMtEph {BSTSetSplayMtEph3,75
    pub struct BSTSetSplayMtEph<T: StTInMtT + Ord> {BSTSetSplayMtEph10,296
        tree: BSTSplayMtEph<T>,tree11,349
    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;BSTSetSplayMt14,388
    pub trait BSTSetSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetSplayMtEphTrait16,442
        fn empty() -> Self;empty17,506
        fn singleton(value: T) -> Self;singleton18,534
        fn size(&self) -> N;size19,574
        fn is_empty(&self) -> B;is_empty20,603
        fn find(&self, value: &T) -> Option<T>;find21,636
        fn contains(&self, value: &T) -> B;contains22,684
        fn minimum(&self) -> Option<T>;minimum23,728
        fn maximum(&self) -> Option<T>;maximum24,768
        fn insert(&mut self, value: T);insert25,808
        fn delete(&mut self, target: &T);delete26,848
        fn union(&self, other: &Self) -> Self;union27,890
        fn intersection(&self, other: &Self) -> Self;intersection28,937
        fn difference(&self, other: &Self) -> Self;difference29,991
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1043
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1098
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1153
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1215
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1285
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1353
        fn as_tree(&self) -> &BSTSplayMtEph<T>;as_tree36,1404
    impl<T: StTInMtT + Ord> BSTSetSplayMtEph<T> {BSTSetSplayMtEph39,1459
        pub fn empty() -> Self {empty40,1509
        pub fn singleton(value: T) -> Self {singleton46,1630
        pub fn size(&self) -> N { self.tree.size() }size52,1789
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1843
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1905
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1983
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2061
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2129
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2197
        pub fn delete(&mut self, target: &T) {delete66,2270
        pub fn union(&self, other: &Self) -> Self {union74,2559
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2858
        pub fn difference(&self, other: &Self) -> Self {difference99,3436
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4013
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4705
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5018
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5374
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5783
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6047
        pub fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree178,6127
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6194
        fn rebuild_from_vec(values: Vec<T>) -> BSTSplayMtEph<T> {rebuild_from_vec182,6285
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6508
    impl<T: StTInMtT + Ord> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {BSTSetSplayMtEph202,6793
        fn empty() -> Self { Self::empty() }empty203,6872
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6918
        fn size(&self) -> N { self.tree.size() }size207,6985
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7035
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7093
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7167
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7241
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7305
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7369
        fn delete(&mut self, target: &T) { BSTSetSplayMtEph::delete(self, target) }delete221,7438
        fn union(&self, other: &Self) -> Self { BSTSetSplayMtEph::union(self, other) }union223,7523
        fn intersection(&self, other: &Self) -> Self { BSTSetSplayMtEph::intersection(self, otheintersection225,7611
        fn difference(&self, other: &Self) -> Self { BSTSetSplayMtEph::difference(self, other) }difference227,7713
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetSplayMtEph::split(self, pivot) }split229,7811
        fn join_pair(left: Self, right: Self) -> Self { BSTSetSplayMtEph::join_pair(left, right)join_pair231,7907
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetSplayMtEph::join_m(left, pijoin_m233,8007
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetSplayMtEph::filter(filter235,8118
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetSplayMtEph::reduce(sereduce237,8234
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8347
        fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree241,8423
    macro_rules! BSTSetSplayMtEphLit {BSTSetSplayMtEphLit245,8512
    fn _BSTSetSplayMtEphLit_type_checks() {_BSTSetSplayMtEphLit_type_checks257,9085

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaMtEph.rs,4514
pub mod BSTBBAlphaMtEph {BSTBBAlphaMtEph3,108
    type Link<T> = Option<Box<Node<T>>>;Link12,344
    struct Node<T: StTInMtT + Ord> {Node15,407
        key: T,key16,444
        size: N,size17,460
        left: Link<T>,left18,477
        right: Link<T>,right19,500
    impl<T: StTInMtT + Ord> Node<T> {Node22,531
        fn new(key: T) -> Self {new23,569
    pub struct BSTBBAlphaMtEph<T: StTInMtT + Ord> {BSTBBAlphaMtEph34,776
        root: Arc<RwLock<Link<T>>>,root35,828
    pub type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;BSTreeBBAlpha38,871
    pub trait BSTBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTBBAlphaMtEphTrait40,924
        fn new() -> Self;new41,987
        fn insert(&self, value: T);insert42,1013
        fn find(&self, target: &T) -> Option<T>;find43,1049
        fn contains(&self, target: &T) -> B;contains44,1098
        fn size(&self) -> N;size45,1143
        fn is_empty(&self) -> B;is_empty46,1172
        fn height(&self) -> N;height47,1205
        fn minimum(&self) -> Option<T>;minimum48,1236
        fn maximum(&self) -> Option<T>;maximum49,1276
        fn in_order(&self) -> ArrayStPerS<T>;in_order50,1316
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order51,1362
    impl<T: StTInMtT + Ord> Default for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph54,1416
        fn default() -> Self { Self::new() }default55,1477
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph58,1529
        pub fn new() -> Self {new59,1578
        pub fn size(&self) -> N {size65,1715
        pub fn is_empty(&self) -> B {is_empty70,1848
        pub fn height(&self) -> N {height78,2015
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec79,2051
        pub fn insert(&self, value: T) {insert90,2401
        pub fn find(&self, target: &T) -> Option<T> {find99,2732
        pub fn contains(&self, target: &T) -> B {contains104,2902
        pub fn minimum(&self) -> Option<T> {minimum112,3092
        pub fn maximum(&self) -> Option<T> {maximum117,3244
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order122,3396
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order129,3675
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link136,3956
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate138,4039
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link140,4158
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild162,4904
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed169,5186
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values179,5596
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced187,5877
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link199,6317
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link214,6834
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link224,7155
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect234,7478
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect242,7765
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph251,8061
        fn new() -> Self { BSTBBAlphaMtEph::new() }new252,8138
        fn insert(&self, value: T) { BSTBBAlphaMtEph::insert(self, value) }insert254,8191
        fn find(&self, target: &T) -> Option<T> { BSTBBAlphaMtEph::find(self, target) }find256,8268
        fn contains(&self, target: &T) -> B { BSTBBAlphaMtEph::contains(self, target) }contains258,8357
        fn size(&self) -> N { BSTBBAlphaMtEph::size(self) }size260,8446
        fn is_empty(&self) -> B { BSTBBAlphaMtEph::is_empty(self) }is_empty262,8507
        fn height(&self) -> N { BSTBBAlphaMtEph::height(self) }height264,8576
        fn minimum(&self) -> Option<T> { BSTBBAlphaMtEph::minimum(self) }minimum266,8641
        fn maximum(&self) -> Option<T> { BSTBBAlphaMtEph::maximum(self) }maximum268,8716
        fn in_order(&self) -> ArrayStPerS<T> { BSTBBAlphaMtEph::in_order(self) }in_order270,8791
        fn pre_order(&self) -> ArrayStPerS<T> { BSTBBAlphaMtEph::pre_order(self) }pre_order272,8873
    macro_rules! BSTBBAlphaMtEphLit {BSTBBAlphaMtEphLit276,8983
    fn _BSTBBAlphaMtEphLit_type_checks() {_BSTBBAlphaMtEphLit_type_checks288,9538

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLStEph.rs,4434
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
    pub struct BSTAVLStEph<T: StT + Ord> {BSTAVLStEph31,713
        root: Link<T>,root32,756
    pub type BSTreeAVL<T> = BSTAVLStEph<T>;BSTreeAVL35,786
    pub trait BSTAVLStEphTrait<T: StT + Ord> {BSTAVLStEphTrait37,831
        fn new() -> Self;new38,878
        fn size(&self) -> N;size39,904
        fn is_empty(&self) -> B;is_empty40,933
        fn height(&self) -> N;height41,966
        fn insert(&mut self, value: T);insert42,997
        fn find(&self, target: &T) -> Option<&T>;find43,1037
        fn contains(&self, target: &T) -> B;contains44,1087
        fn minimum(&self) -> Option<&T>;minimum45,1132
        fn maximum(&self) -> Option<&T>;maximum46,1173
        fn in_order(&self) -> ArrayStPerS<T>;in_order47,1214
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order48,1260
    impl<T: StT + Ord> Default for BSTAVLStEph<T> {BSTAVLStEph51,1314
        fn default() -> Self { Self::new() }default52,1366
    impl<T: StT + Ord> BSTAVLStEph<T> {BSTAVLStEph55,1418
        pub fn new() -> Self { BSTAVLStEph { root: None } }new56,1458
        pub fn size(&self) -> N { Self::size_link(&self.root) }size58,1519
        pub fn is_empty(&self) -> B {is_empty60,1584
        pub fn height(&self) -> N { Self::height_link(&self.root) as N }height68,1751
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert70,1825
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find72,1915
        pub fn contains(&self, target: &T) -> B {contains74,2009
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum82,2199
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum84,2275
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order86,2351
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order92,2569
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link98,2789
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link100,2878
        fn update(node: &mut Node<T>) {update102,2961
        fn rotate_right(link: &mut Link<T>) {rotate_right107,3197
        fn rotate_left(link: &mut Link<T>) {rotate_left121,3653
        fn rebalance(link: &mut Link<T>) {rebalance135,4108
        fn insert_link(link: &mut Link<T>, value: T) {insert_link160,5154
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link179,5789
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link194,6306
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link204,6627
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect214,6950
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect222,7237
    impl<T: StT + Ord> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {BSTAVLStEph231,7533
        fn new() -> Self { BSTAVLStEph::new() }new232,7597
        fn size(&self) -> N { BSTAVLStEph::size(self) }size234,7646
        fn is_empty(&self) -> B { BSTAVLStEph::is_empty(self) }is_empty236,7703
        fn height(&self) -> N { BSTAVLStEph::height(self) }height238,7768
        fn insert(&mut self, value: T) { BSTAVLStEph::insert(self, value) }insert240,7829
        fn find(&self, target: &T) -> Option<&T> { BSTAVLStEph::find(self, target) }find242,7906
        fn contains(&self, target: &T) -> B { BSTAVLStEph::contains(self, target) }contains244,7992
        fn minimum(&self) -> Option<&T> { BSTAVLStEph::minimum(self) }minimum246,8077
        fn maximum(&self) -> Option<&T> { BSTAVLStEph::maximum(self) }maximum248,8149
        fn in_order(&self) -> ArrayStPerS<T> { BSTAVLStEph::in_order(self) }in_order250,8221
        fn pre_order(&self) -> ArrayStPerS<T> { BSTAVLStEph::pre_order(self) }pre_order252,8299
    macro_rules! BSTAVLStEphLit {BSTAVLStEphLit256,8405
    fn _BSTAVLStEphLit_type_checks() {_BSTAVLStEphLit_type_checks268,8912

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainMtEph.rs,3702
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
    pub struct BSTPlainMtEph<T: StTInMtT + Ord> {BSTPlainMtEph39,1028
        root: Link<T>,root40,1078
    pub type BSTree<T> = BSTPlainMtEph<T>;BSTree43,1108
    pub trait BSTPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTPlainMtEphTrait45,1152
        fn new() -> Self;new46,1213
        fn insert(&self, value: T);insert47,1239
        fn find(&self, target: &T) -> Option<T>;find48,1275
        fn contains(&self, target: &T) -> B;contains49,1324
        fn size(&self) -> N;size50,1369
        fn is_empty(&self) -> B;is_empty51,1398
        fn height(&self) -> N;height52,1431
        fn minimum(&self) -> Option<T>;minimum53,1462
        fn maximum(&self) -> Option<T>;maximum54,1502
        fn in_order(&self) -> ArrayStPerS<T>;in_order55,1542
    impl<T: StTInMtT + Ord> BSTPlainMtEph<T> {BSTPlainMtEph58,1595
        pub fn new() -> Self {new59,1642
        pub fn insert(&self, value: T) {insert65,1768
            fn descend<T: StTInMtT + Ord>(link: &Link<T>, value: T) -> bool {descend66,1809
        pub fn find(&self, target: &T) -> Option<T> {find100,3001
            fn find_rec<T: StTInMtT + Ord>(link: &Link<T>, target: &T) -> Option<T> {find_rec101,3055
        pub fn contains(&self, target: &T) -> B {contains120,3794
        pub fn size(&self) -> N {size127,3983
        pub fn is_empty(&self) -> B {is_empty132,4134
        pub fn height(&self) -> N {height140,4301
        pub fn minimum(&self) -> Option<T> {minimum145,4461
            fn leftmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {leftmost146,4506
        pub fn maximum(&self) -> Option<T> {maximum167,5216
            fn rightmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {rightmost168,5261
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order189,5978
            fn traverse<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {traverse190,6029
    fn height_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> i32 { link.as_ref().map_or(0, |n|height_of209,6694
    fn size_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> N { link.as_ref().map_or(0, |n| n.ssize_of211,6804
    impl<T: StTInMtT + Ord> BSTPlainMtEphTrait<T> for BSTPlainMtEph<T> {BSTPlainMtEph213,6908
        fn new() -> Self { BSTPlainMtEph::new() }new214,6981
        fn insert(&self, value: T) { BSTPlainMtEph::insert(self, value) }insert215,7031
        fn find(&self, target: &T) -> Option<T> { BSTPlainMtEph::find(self, target) }find216,7105
        fn contains(&self, target: &T) -> B { BSTPlainMtEph::contains(self, target) }contains217,7191
        fn size(&self) -> N { BSTPlainMtEph::size(self) }size218,7277
        fn is_empty(&self) -> B { BSTPlainMtEph::is_empty(self) }is_empty219,7335
        fn height(&self) -> N { BSTPlainMtEph::height(self) }height220,7401
        fn minimum(&self) -> Option<T> { BSTPlainMtEph::minimum(self) }minimum221,7463
        fn maximum(&self) -> Option<T> { BSTPlainMtEph::maximum(self) }maximum222,7535
        fn in_order(&self) -> ArrayStPerS<T> { BSTPlainMtEph::in_order(self) }in_order223,7607
    macro_rules! BSTPlainMtEphLit {BSTPlainMtEphLit227,7713
    fn _BSTPlainMtEphLit_type_checks() {_BSTPlainMtEphLit_type_checks242,8277

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaStEph.rs,4626
pub mod BSTBBAlphaStEph {BSTBBAlphaStEph3,80
    type Link<T> = Option<Box<Node<T>>>;Link10,281
    struct Node<T: StT + Ord> {Node13,344
        key: T,key14,376
        size: N,size15,392
        left: Link<T>,left16,409
        right: Link<T>,right17,432
    impl<T: StT + Ord> Node<T> {Node20,463
        fn new(key: T) -> Self {new21,496
    pub struct BSTBBAlphaStEph<T: StT + Ord> {BSTBBAlphaStEph31,682
        root: Link<T>,root32,729
    pub type BSTreeBBAlpha<T> = BSTBBAlphaStEph<T>;BSTreeBBAlpha35,759
    pub trait BSTBBAlphaStEphTrait<T: StT + Ord> {BSTBBAlphaStEphTrait37,812
        fn new() -> Self;new38,863
        fn size(&self) -> N;size39,889
        fn is_empty(&self) -> B;is_empty40,918
        fn height(&self) -> N;height41,951
        fn insert(&mut self, value: T);insert42,982
        fn find(&self, target: &T) -> Option<&T>;find43,1022
        fn contains(&self, target: &T) -> B;contains44,1072
        fn minimum(&self) -> Option<&T>;minimum45,1117
        fn maximum(&self) -> Option<&T>;maximum46,1158
        fn in_order(&self) -> ArrayStPerS<T>;in_order47,1199
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order48,1245
    impl<T: StT + Ord> Default for BSTBBAlphaStEph<T> {BSTBBAlphaStEph51,1299
        fn default() -> Self { Self::new() }default52,1355
    impl<T: StT + Ord> BSTBBAlphaStEph<T> {BSTBBAlphaStEph55,1407
        pub fn new() -> Self { BSTBBAlphaStEph { root: None } }new56,1451
        pub fn size(&self) -> N { Self::size_link(&self.root) }size58,1516
        pub fn is_empty(&self) -> B {is_empty60,1581
        pub fn height(&self) -> N {height68,1748
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec69,1784
        pub fn insert(&mut self, value: T) {insert78,2080
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find86,2368
        pub fn contains(&self, target: &T) -> B {contains88,2462
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum96,2652
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum98,2728
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order100,2804
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order106,3022
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link112,3242
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate114,3325
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link116,3444
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild138,4190
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed145,4472
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values155,4882
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced163,5163
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link175,5603
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link190,6120
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link200,6441
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect210,6764
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect218,7051
    impl<T: StT + Ord> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {BSTBBAlphaStEph227,7347
        fn new() -> Self { BSTBBAlphaStEph::new() }new228,7419
        fn size(&self) -> N { BSTBBAlphaStEph::size(self) }size230,7472
        fn is_empty(&self) -> B { BSTBBAlphaStEph::is_empty(self) }is_empty232,7533
        fn height(&self) -> N { BSTBBAlphaStEph::height(self) }height234,7602
        fn insert(&mut self, value: T) { BSTBBAlphaStEph::insert(self, value) }insert236,7667
        fn find(&self, target: &T) -> Option<&T> { BSTBBAlphaStEph::find(self, target) }find238,7748
        fn contains(&self, target: &T) -> B { BSTBBAlphaStEph::contains(self, target) }contains240,7838
        fn minimum(&self) -> Option<&T> { BSTBBAlphaStEph::minimum(self) }minimum242,7927
        fn maximum(&self) -> Option<&T> { BSTBBAlphaStEph::maximum(self) }maximum244,8003
        fn in_order(&self) -> ArrayStPerS<T> { BSTBBAlphaStEph::in_order(self) }in_order246,8079
        fn pre_order(&self) -> ArrayStPerS<T> { BSTBBAlphaStEph::pre_order(self) }pre_order248,8161
    macro_rules! BSTBBAlphaStEphLit {BSTBBAlphaStEphLit252,8271
    fn _BSTBBAlphaStEphLit_type_checks() {_BSTBBAlphaStEphLit_type_checks264,8830

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTParaStEph.rs,2986
pub mod BSTParaStEph {BSTParaStEph3,70
    pub enum Exposed<T: StT + Ord> {Exposed11,253
        Leaf,Leaf12,290
        Node(ParamBST<T>, T, ParamBST<T>),Node13,304
    struct NodeInner<T: StT + Ord> {NodeInner17,375
        key: T,key18,412
        size: N,size19,428
        left: ParamBST<T>,left20,445
        right: ParamBST<T>,right21,472
    pub struct ParamBST<T: StT + Ord> {ParamBST25,528
        root: Rc<RefCell<Option<Box<NodeInner<T>>>>>,root26,568
    pub trait ParamBSTTrait<T: StT + Ord>: Sized {ParamBSTTrait29,629
        fn new() -> Self;new30,680
        fn expose(&self) -> Exposed<T>;expose31,706
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid32,746
        fn size(&self) -> N;size33,796
        fn is_empty(&self) -> B;is_empty34,825
        fn insert(&self, key: T);insert35,858
        fn delete(&self, key: &T);delete36,892
        fn find(&self, key: &T) -> Option<T>;find37,927
        fn split(&self, key: &T) -> (Self, B, Self);split38,973
        fn join_pair(&self, other: Self) -> Self;join_pair39,1026
        fn union(&self, other: &Self) -> Self;union40,1076
        fn in_order(&self) -> ArrayStPerS<T>;in_order41,1123
    impl<T: StT + Ord> ParamBST<T> {ParamBST44,1176
        fn expose_internal(&self) -> Exposed<T> {expose_internal45,1213
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid53,1505
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner65,1957
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m84,2942
        fn min_key(tree: &Self) -> Option<T> {min_key86,3058
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner96,3402
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner107,3872
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order119,4379
    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {ParamBST131,4781
        fn new() -> Self {new132,4839
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose138,4965
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid140,5032
        fn size(&self) -> N {size142,5114
        fn is_empty(&self) -> B {is_empty146,5223
        fn insert(&self, key: T) {insert154,5386
        fn delete(&self, key: &T) {delete161,5676
        fn find(&self, key: &T) -> Option<T> {find168,5968
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split179,6465
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair181,6555
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union183,6656
        fn in_order(&self) -> ArrayStPerS<T> {in_order185,6742
    macro_rules! ParamBSTLit {ParamBSTLit193,6980
    fn _ParamBSTLit_type_checks() {_ParamBSTLit_type_checks205,7476

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayMtEph.rs,4115
pub mod BSTSplayMtEph {BSTSplayMtEph3,99
    type Link<T> = Option<Box<Node<T>>>;Link10,303
    struct Node<T: StTInMtT + Ord> {Node13,366
        key: T,key14,403
        size: N,size15,419
        left: Link<T>,left16,436
        right: Link<T>,right17,459
    impl<T: StTInMtT + Ord> Node<T> {Node20,490
        fn new(key: T) -> Self {new21,528
    pub struct BSTSplayMtEph<T: StTInMtT + Ord> {BSTSplayMtEph32,735
        root: Arc<RwLock<Link<T>>>,root33,785
    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;BSTreeSplay36,828
    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSplayMtEphTrait38,877
        fn new() -> Self;new39,938
        fn insert(&self, value: T);insert40,964
        fn find(&self, target: &T) -> Option<T>;find41,1000
        fn contains(&self, target: &T) -> B;contains42,1049
        fn size(&self) -> N;size43,1094
        fn is_empty(&self) -> B;is_empty44,1123
        fn height(&self) -> N;height45,1156
        fn minimum(&self) -> Option<T>;minimum46,1187
        fn maximum(&self) -> Option<T>;maximum47,1227
        fn in_order(&self) -> ArrayStPerS<T>;in_order48,1267
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order49,1313
    impl<T: StTInMtT + Ord> Default for BSTSplayMtEph<T> {BSTSplayMtEph52,1367
        fn default() -> Self { Self::new() }default53,1426
    impl<T: StTInMtT + Ord> BSTSplayMtEph<T> {BSTSplayMtEph56,1478
        pub fn new() -> Self {new57,1525
        pub fn size(&self) -> N {size63,1660
        pub fn is_empty(&self) -> B {is_empty68,1793
        pub fn height(&self) -> N {height76,1960
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec77,1996
        pub fn insert(&self, value: T) {insert88,2346
        pub fn find(&self, target: &T) -> Option<T> {find93,2505
        pub fn contains(&self, target: &T) -> B {contains98,2675
        pub fn minimum(&self) -> Option<T> {minimum106,2865
        pub fn maximum(&self) -> Option<T> {maximum111,3017
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order116,3169
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order123,3448
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link130,3729
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate132,3812
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link134,3931
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link156,4677
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link171,5194
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link181,5515
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect191,5838
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect199,6125
    impl<T: StTInMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {BSTSplayMtEph208,6421
        fn new() -> Self { BSTSplayMtEph::new() }new209,6494
        fn insert(&self, value: T) { BSTSplayMtEph::insert(self, value) }insert211,6545
        fn find(&self, target: &T) -> Option<T> { BSTSplayMtEph::find(self, target) }find213,6620
        fn contains(&self, target: &T) -> B { BSTSplayMtEph::contains(self, target) }contains215,6707
        fn size(&self) -> N { BSTSplayMtEph::size(self) }size217,6794
        fn is_empty(&self) -> B { BSTSplayMtEph::is_empty(self) }is_empty219,6853
        fn height(&self) -> N { BSTSplayMtEph::height(self) }height221,6920
        fn minimum(&self) -> Option<T> { BSTSplayMtEph::minimum(self) }minimum223,6983
        fn maximum(&self) -> Option<T> { BSTSplayMtEph::maximum(self) }maximum225,7056
        fn in_order(&self) -> ArrayStPerS<T> { BSTSplayMtEph::in_order(self) }in_order227,7129
        fn pre_order(&self) -> ArrayStPerS<T> { BSTSplayMtEph::pre_order(self) }pre_order229,7209
    macro_rules! BSTSplayMtEphLit {BSTSplayMtEphLit233,7317
    fn _BSTSplayMtEphLit_type_checks() {_BSTSplayMtEphLit_type_checks245,7846

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetRBMtEph.rs,5552
pub mod BSTSetRBMtEph {BSTSetRBMtEph3,79
    pub struct BSTSetRBMtEph<T: StTInMtT + Ord> {BSTSetRBMtEph10,288
        tree: BSTRBMtEph<T>,tree11,338
    pub type BSTSetRBMt<T> = BSTSetRBMtEph<T>;BSTSetRBMt14,374
    pub trait BSTSetRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetRBMtEphTrait16,422
        fn empty() -> Self;empty17,483
        fn singleton(value: T) -> Self;singleton18,511
        fn size(&self) -> N;size19,551
        fn is_empty(&self) -> B;is_empty20,580
        fn find(&self, value: &T) -> Option<T>;find21,613
        fn contains(&self, value: &T) -> B;contains22,661
        fn minimum(&self) -> Option<T>;minimum23,705
        fn maximum(&self) -> Option<T>;maximum24,745
        fn insert(&mut self, value: T);insert25,785
        fn delete(&mut self, target: &T);delete26,825
        fn union(&self, other: &Self) -> Self;union27,867
        fn intersection(&self, other: &Self) -> Self;intersection28,914
        fn difference(&self, other: &Self) -> Self;difference29,968
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1020
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1075
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1130
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1192
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1262
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1330
        fn as_tree(&self) -> &BSTRBMtEph<T>;as_tree36,1381
    impl<T: StTInMtT + Ord> BSTSetRBMtEph<T> {BSTSetRBMtEph39,1433
        pub fn empty() -> Self {empty40,1480
        pub fn singleton(value: T) -> Self {singleton46,1598
        pub fn size(&self) -> N { self.tree.size() }size52,1754
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1808
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1870
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1948
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2026
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2094
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2162
        pub fn delete(&mut self, target: &T) {delete66,2235
        pub fn union(&self, other: &Self) -> Self {union74,2524
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2823
        pub fn difference(&self, other: &Self) -> Self {difference99,3401
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,3978
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4670
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,4983
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5339
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5748
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6012
        pub fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree178,6092
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6156
        fn rebuild_from_vec(values: Vec<T>) -> BSTRBMtEph<T> {rebuild_from_vec182,6247
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6464
    impl<T: StTInMtT + Ord> BSTSetRBMtEphTrait<T> for BSTSetRBMtEph<T> {BSTSetRBMtEph202,6746
        fn empty() -> Self { Self::empty() }empty203,6819
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6865
        fn size(&self) -> N { self.tree.size() }size207,6932
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,6982
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7040
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7114
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7188
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7252
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7316
        fn delete(&mut self, target: &T) { BSTSetRBMtEph::delete(self, target) }delete221,7385
        fn union(&self, other: &Self) -> Self { BSTSetRBMtEph::union(self, other) }union223,7467
        fn intersection(&self, other: &Self) -> Self { BSTSetRBMtEph::intersection(self, other) intersection225,7552
        fn difference(&self, other: &Self) -> Self { BSTSetRBMtEph::difference(self, other) }difference227,7651
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetRBMtEph::split(self, pivot) }split229,7746
        fn join_pair(left: Self, right: Self) -> Self { BSTSetRBMtEph::join_pair(left, right) }join_pair231,7839
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetRBMtEph::join_m(left, pivotjoin_m233,7936
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetRBMtEph::filter(selfilter235,8044
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetRBMtEph::reduce(self,reduce237,8157
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8267
        fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree241,8343
    macro_rules! BSTSetRBMtEphLit {BSTSetRBMtEphLit245,8429
    fn _BSTSetRBMtEphLit_type_checks() {_BSTSetRBMtEphLit_type_checks257,8963

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStEph.rs,1394
pub mod AVLTreeSeqStEph {AVLTreeSeqStEph3,54
    pub trait AVLTreeSeqStEphTrait<T: StT> {AVLTreeSeqStEphTrait11,241
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T>;tabulate12,286
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U>;map13,356
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T>;select14,449
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T>;append15,537
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T>;deflate16,629
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T>;filter17,700
    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS20,795
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T> {tabulate21,862
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U> {map24,1020
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T> {select27,1196
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T> {append41,1855
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T> {deflate44,2033
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T> {filter51,2344

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBMtEph.rs,4517
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
    pub struct BSTRBMtEph<T: StTInMtT + Ord> {BSTRBMtEph40,886
        root: Arc<RwLock<Link<T>>>,root41,933
    pub type BSTreeRB<T> = BSTRBMtEph<T>;BSTreeRB44,976
    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTRBMtEphTrait46,1019
        fn new() -> Self;new47,1077
        fn insert(&self, value: T);insert48,1103
        fn find(&self, target: &T) -> Option<T>;find49,1139
        fn contains(&self, target: &T) -> B;contains50,1188
        fn size(&self) -> N;size51,1233
        fn is_empty(&self) -> B;is_empty52,1262
        fn height(&self) -> N;height53,1295
        fn minimum(&self) -> Option<T>;minimum54,1326
        fn maximum(&self) -> Option<T>;maximum55,1366
        fn in_order(&self) -> ArrayStPerS<T>;in_order56,1406
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order57,1452
    impl<T: StTInMtT + Ord> Default for BSTRBMtEph<T> {BSTRBMtEph60,1506
        fn default() -> Self { Self::new() }default61,1562
    impl<T: StTInMtT + Ord> BSTRBMtEph<T> {BSTRBMtEph64,1614
        pub fn new() -> Self {new65,1658
        pub fn size(&self) -> N {size71,1790
        pub fn is_empty(&self) -> B {is_empty76,1923
        pub fn height(&self) -> N {height84,2090
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec85,2126
        pub fn insert(&self, value: T) {insert96,2476
        pub fn find(&self, target: &T) -> Option<T> {find104,2741
        pub fn contains(&self, target: &T) -> B {contains109,2911
        pub fn minimum(&self) -> Option<T> {minimum117,3101
        pub fn maximum(&self) -> Option<T> {maximum122,3253
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order127,3405
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order134,3684
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red141,3965
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link143,4067
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate145,4150
        fn rotate_left(link: &mut Link<T>) {rotate_left147,4269
        fn rotate_right(link: &mut Link<T>) {rotate_right166,4929
        fn flip_colors(link: &mut Link<T>) {flip_colors185,5593
        fn fix_up(link: &mut Link<T>) {fix_up206,6398
        fn insert_link(link: &mut Link<T>, value: T) {insert_link242,7552
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link258,8094
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link273,8611
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link283,8932
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect293,9255
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect301,9542
    impl<T: StTInMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {BSTRBMtEph310,9838
        fn new() -> Self { BSTRBMtEph::new() }new311,9905
        fn insert(&self, value: T) { BSTRBMtEph::insert(self, value) }insert313,9953
        fn find(&self, target: &T) -> Option<T> { BSTRBMtEph::find(self, target) }find315,10025
        fn contains(&self, target: &T) -> B { BSTRBMtEph::contains(self, target) }contains317,10109
        fn size(&self) -> N { BSTRBMtEph::size(self) }size319,10193
        fn is_empty(&self) -> B { BSTRBMtEph::is_empty(self) }is_empty321,10249
        fn height(&self) -> N { BSTRBMtEph::height(self) }height323,10313
        fn minimum(&self) -> Option<T> { BSTRBMtEph::minimum(self) }minimum325,10373
        fn maximum(&self) -> Option<T> { BSTRBMtEph::maximum(self) }maximum327,10443
        fn in_order(&self) -> ArrayStPerS<T> { BSTRBMtEph::in_order(self) }in_order329,10513
        fn pre_order(&self) -> ArrayStPerS<T> { BSTRBMtEph::pre_order(self) }pre_order331,10590
    macro_rules! BSTRBMtEphLit {BSTRBMtEphLit335,10695
    fn _BSTRBMtEphLit_type_checks() {_BSTRBMtEphLit_type_checks347,11185

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTParaMtEph.rs,4111
pub mod BSTParaMtEph {BSTParaMtEph3,69
    pub enum Exposed<T: StTInMtT + Ord> {Exposed10,237
        Leaf,Leaf11,279
        Node(ParamBST<T>, T, ParamBST<T>),Node12,293
    struct NodeInner<T: StTInMtT + Ord> {NodeInner16,364
        key: T,key17,406
        size: N,size18,422
        left: ParamBST<T>,left19,439
        right: ParamBST<T>,right20,466
    pub struct ParamBST<T: StTInMtT + Ord> {ParamBST24,522
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root25,567
    pub trait ParamBSTTrait<T: StTInMtT + Ord + 'static>: Sized {ParamBSTTrait28,628
        fn new() -> Self;new31,785
        fn expose(&self) -> Exposed<T>;expose34,902
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid37,1033
        fn size(&self) -> N;size40,1174
        fn is_empty(&self) -> B;is_empty43,1294
        fn insert(&self, key: T);insert46,1438
        fn delete(&self, key: &T);delete49,1583
        fn find(&self, key: &T) -> Option<T>;find52,1729
        fn split(&self, key: &T) -> (Self, B, Self);split55,1886
        fn join_pair(&self, other: Self) -> Self;join_pair58,2098
        fn union(&self, other: &Self) -> Self;union61,2273
        fn intersect(&self, other: &Self) -> Self;intersect64,2445
        fn difference(&self, other: &Self) -> Self;difference67,2621
        fn filter<F>(&self, predicate: F) -> Selffilter70,2778
        fn reduce<F>(&self, op: F, base: T) -> Treduce75,3002
        fn in_order(&self) -> ArrayStPerS<T>;in_order80,3218
    impl<T: StTInMtT + Ord + 'static> ParamBST<T> {ParamBST83,3271
        fn expose_internal(&self) -> Exposed<T> {expose_internal86,3414
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid96,3804
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner110,4367
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m131,5443
        fn min_key(tree: &Self) -> Option<T> {min_key135,5670
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner147,6185
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner160,6780
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner176,7497
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner196,8450
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner217,9416
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel241,10488
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner251,10850
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel275,11919
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order285,12258
    impl<T: StTInMtT + Ord + 'static> ParamBSTTrait<T> for ParamBST<T> {ParamBST297,12660
        fn new() -> Self {new300,12824
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose308,13041
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid312,13199
        fn size(&self) -> N {size316,13372
        fn is_empty(&self) -> B {is_empty323,13610
        fn insert(&self, key: T) {insert333,13884
        fn delete(&self, key: &T) {delete343,14327
        fn find(&self, key: &T) -> Option<T> {find353,14772
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split366,15380
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair370,15629
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union374,15855
        fn intersect(&self, other: &Self) -> Self { ParamBST::intersect_inner(self, other) }intersect378,16066
        fn difference(&self, other: &Self) -> Self { ParamBST::difference_inner(self, other) }difference382,16285
        fn filter<F>(&self, predicate: F) -> Selffilter386,16486
        fn reduce<F>(&self, op: F, base: T) -> Treduce395,16786
        fn in_order(&self) -> ArrayStPerS<T> {in_order404,17077

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayStEph.rs,4268
pub mod BSTSplayStEph {BSTSplayStEph3,84
    type Link<T> = Option<Box<Node<T>>>;Link8,253
    struct Node<T: StT + Ord> {Node11,316
        key: T,key12,348
        size: N,size13,364
        left: Link<T>,left14,381
        right: Link<T>,right15,404
    impl<T: StT + Ord> Node<T> {Node18,435
        fn new(key: T) -> Self {new19,468
    pub struct BSTSplayStEph<T: StT + Ord> {BSTSplayStEph29,654
        root: Link<T>,root30,699
    pub type BSTreeSplay<T> = BSTSplayStEph<T>;BSTreeSplay33,729
    pub trait BSTSplayStEphTrait<T: StT + Ord> {BSTSplayStEphTrait35,778
        fn new() -> Self;new36,827
        fn size(&self) -> N;size37,853
        fn is_empty(&self) -> B;is_empty38,882
        fn height(&self) -> N;height39,915
        fn insert(&mut self, value: T);insert40,946
        fn find(&self, target: &T) -> Option<&T>;find41,986
        fn contains(&self, target: &T) -> B;contains42,1036
        fn minimum(&self) -> Option<&T>;minimum43,1081
        fn maximum(&self) -> Option<&T>;maximum44,1122
        fn in_order(&self) -> ArrayStPerS<T>;in_order45,1163
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order46,1209
    impl<T: StT + Ord> Default for BSTSplayStEph<T> {BSTSplayStEph49,1263
        fn default() -> Self { Self::new() }default50,1317
    impl<T: StT + Ord> BSTSplayStEph<T> {BSTSplayStEph53,1369
        pub fn new() -> Self { BSTSplayStEph { root: None } }new54,1411
        pub fn size(&self) -> N { Self::size_link(&self.root) }size56,1474
        pub fn is_empty(&self) -> B {is_empty58,1539
        pub fn height(&self) -> N {height66,1706
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec67,1742
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert76,2038
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find78,2128
        pub fn contains(&self, target: &T) -> B {contains80,2222
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum88,2412
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum90,2488
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order92,2564
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order98,2782
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link104,3002
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate106,3085
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link108,3204
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link130,3950
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link145,4467
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link155,4788
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect165,5111
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect173,5398
    impl<T: StT + Ord> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {BSTSplayStEph182,5694
        fn new() -> Self { BSTSplayStEph::new() }new183,5762
        fn size(&self) -> N { BSTSplayStEph::size(self) }size185,5813
        fn is_empty(&self) -> B { BSTSplayStEph::is_empty(self) }is_empty187,5872
        fn height(&self) -> N { BSTSplayStEph::height(self) }height189,5939
        fn insert(&mut self, value: T) { BSTSplayStEph::insert(self, value) }insert191,6002
        fn find(&self, target: &T) -> Option<&T> { BSTSplayStEph::find(self, target) }find193,6081
        fn contains(&self, target: &T) -> B { BSTSplayStEph::contains(self, target) }contains195,6169
        fn minimum(&self) -> Option<&T> { BSTSplayStEph::minimum(self) }minimum197,6256
        fn maximum(&self) -> Option<&T> { BSTSplayStEph::maximum(self) }maximum199,6330
        fn in_order(&self) -> ArrayStPerS<T> { BSTSplayStEph::in_order(self) }in_order201,6404
        fn pre_order(&self) -> ArrayStPerS<T> { BSTSplayStEph::pre_order(self) }pre_order203,6484
    macro_rules! BSTSplayStEphLit {BSTSplayStEphLit207,6592
    fn _BSTSplayStEphLit_type_checks() {_BSTSplayStEphLit_type_checks219,7125

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLMtEph.rs,4241
pub mod BSTAVLMtEph {BSTAVLMtEph3,96
    type Link<T> = Option<Box<Node<T>>>;Link10,298
    struct Node<T: StTInMtT + Ord> {Node13,361
        key: T,key14,398
        height: i32,height15,414
        size: N,size16,435
        left: Link<T>,left17,452
        right: Link<T>,right18,475
    impl<T: StTInMtT + Ord> Node<T> {Node21,506
        fn new(key: T) -> Self {new22,544
    pub struct BSTAVLMtEph<T: StTInMtT + Ord> {BSTAVLMtEph34,778
        root: Arc<RwLock<Link<T>>>,root35,826
    pub type BSTreeAVL<T> = BSTAVLMtEph<T>;BSTreeAVL38,869
    pub trait BSTAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTAVLMtEphTrait40,914
        fn new() -> Self;new41,973
        fn insert(&self, value: T);insert42,999
        fn find(&self, target: &T) -> Option<T>;find43,1035
        fn contains(&self, target: &T) -> B;contains44,1084
        fn size(&self) -> N;size45,1129
        fn is_empty(&self) -> B;is_empty46,1158
        fn height(&self) -> N;height47,1191
        fn minimum(&self) -> Option<T>;minimum48,1222
        fn maximum(&self) -> Option<T>;maximum49,1262
        fn in_order(&self) -> ArrayStPerS<T>;in_order50,1302
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order51,1348
    impl<T: StTInMtT + Ord> Default for BSTAVLMtEph<T> {BSTAVLMtEph54,1402
        fn default() -> Self { Self::new() }default55,1459
    impl<T: StTInMtT + Ord> BSTAVLMtEph<T> {BSTAVLMtEph58,1511
        pub fn new() -> Self {new59,1556
        pub fn size(&self) -> N {size65,1689
        pub fn is_empty(&self) -> B {is_empty70,1822
        pub fn height(&self) -> N {height78,1989
        pub fn insert(&self, value: T) {insert83,2131
        pub fn find(&self, target: &T) -> Option<T> {find88,2290
        pub fn contains(&self, target: &T) -> B {contains93,2460
        pub fn minimum(&self) -> Option<T> {minimum101,2650
        pub fn maximum(&self) -> Option<T> {maximum106,2802
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order111,2954
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order118,3233
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link125,3514
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link127,3603
        fn update(node: &mut Node<T>) {update129,3686
        fn rotate_right(link: &mut Link<T>) {rotate_right134,3922
        fn rotate_left(link: &mut Link<T>) {rotate_left148,4378
        fn rebalance(link: &mut Link<T>) {rebalance162,4833
        fn insert_link(link: &mut Link<T>, value: T) {insert_link187,5879
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link206,6514
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link221,7031
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link231,7353
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect241,7677
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect249,7964
    impl<T: StTInMtT + Ord> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {BSTAVLMtEph258,8260
        fn new() -> Self { BSTAVLMtEph::new() }new259,8329
        fn insert(&self, value: T) { BSTAVLMtEph::insert(self, value) }insert261,8378
        fn find(&self, target: &T) -> Option<T> { BSTAVLMtEph::find(self, target) }find263,8451
        fn contains(&self, target: &T) -> B { BSTAVLMtEph::contains(self, target) }contains265,8536
        fn size(&self) -> N { BSTAVLMtEph::size(self) }size267,8621
        fn is_empty(&self) -> B { BSTAVLMtEph::is_empty(self) }is_empty269,8678
        fn height(&self) -> N { BSTAVLMtEph::height(self) }height271,8743
        fn minimum(&self) -> Option<T> { BSTAVLMtEph::minimum(self) }minimum273,8804
        fn maximum(&self) -> Option<T> { BSTAVLMtEph::maximum(self) }maximum275,8875
        fn in_order(&self) -> ArrayStPerS<T> { BSTAVLMtEph::in_order(self) }in_order277,8946
        fn pre_order(&self) -> ArrayStPerS<T> { BSTAVLMtEph::pre_order(self) }pre_order279,9024
    macro_rules! BSTAVLMtEphLit {BSTAVLMtEphLit283,9130
    fn _BSTAVLMtEphLit_type_checks() {_BSTAVLMtEphLit_type_checks295,9633

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainStEph.rs,3550
pub mod BSTPlainStEph {BSTPlainStEph3,64
    pub struct BSTPlainStEph<T: StT + Ord> {BSTPlainStEph8,216
        root: BBTree<T>,root9,261
    pub type BSTree<T> = BSTPlainStEph<T>;BSTree12,293
    pub trait BSTPlainStEphTrait<T: StT + Ord> {BSTPlainStEphTrait14,337
        fn new() -> Self;new15,386
        fn size(&self) -> N;size16,412
        fn is_empty(&self) -> B;is_empty17,441
        fn height(&self) -> N;height18,474
        fn insert(&mut self, value: T);insert19,505
        fn find(&self, target: &T) -> Option<&T>;find20,545
        fn contains(&self, target: &T) -> B;contains21,595
        fn minimum(&self) -> Option<&T>;minimum22,640
        fn maximum(&self) -> Option<&T>;maximum23,681
        fn in_order(&self) -> ArrayStPerS<T>;in_order24,722
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order25,768
    impl<T: StT + Ord> BSTPlainStEph<T> {BSTPlainStEph28,822
        pub fn new() -> Self { BSTPlainStEph { root: BBTree::leaf() } }new29,864
        pub fn size(&self) -> N { self.root.size() }size31,937
        pub fn is_empty(&self) -> B { self.root.is_leaf() }is_empty33,991
        pub fn height(&self) -> N { self.root.height() }height35,1052
        pub fn insert(&mut self, value: T) { insert_node(&mut self.root, value); }insert37,1110
        pub fn find(&self, target: &T) -> Option<&T> { find_node(&self.root, target) }find39,1194
        pub fn contains(&self, target: &T) -> B { contains_node(&self.root, target) }contains41,1282
        pub fn minimum(&self) -> Option<&T> { min_node(&self.root) }minimum43,1369
        pub fn maximum(&self) -> Option<&T> { max_node(&self.root) }maximum45,1439
        pub fn in_order(&self) -> ArrayStPerS<T> { self.root.in_order() }in_order47,1509
        pub fn pre_order(&self) -> ArrayStPerS<T> { self.root.pre_order() }pre_order49,1584
    impl<T: StT + Ord> BSTPlainStEphTrait<T> for BSTPlainStEph<T> {BSTPlainStEph52,1667
        fn new() -> Self { BSTPlainStEph::new() }new53,1735
        fn size(&self) -> N { BSTPlainStEph::size(self) }size55,1786
        fn is_empty(&self) -> B { BSTPlainStEph::is_empty(self) }is_empty57,1845
        fn height(&self) -> N { BSTPlainStEph::height(self) }height59,1912
        fn insert(&mut self, value: T) { BSTPlainStEph::insert(self, value) }insert61,1975
        fn find(&self, target: &T) -> Option<&T> { BSTPlainStEph::find(self, target) }find63,2054
        fn contains(&self, target: &T) -> B { BSTPlainStEph::contains(self, target) }contains65,2142
        fn minimum(&self) -> Option<&T> { BSTPlainStEph::minimum(self) }minimum67,2229
        fn maximum(&self) -> Option<&T> { BSTPlainStEph::maximum(self) }maximum69,2303
        fn in_order(&self) -> ArrayStPerS<T> { BSTPlainStEph::in_order(self) }in_order71,2377
        fn pre_order(&self) -> ArrayStPerS<T> { BSTPlainStEph::pre_order(self) }pre_order73,2457
    fn insert_node<T: StT + Ord>(node: &mut BBTree<T>, value: T) {insert_node76,2545
    fn contains_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> B {contains_node91,3048
    fn find_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> Option<&'a T> {find_node106,3534
    fn min_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {min_node121,4027
    fn max_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {max_node131,4347
    macro_rules! BSTPlainStEphLit {BSTPlainStEphLit142,4689
    fn _BSTPlainStEphLit_type_checks() {_BSTPlainStEphLit_type_checks157,5261

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetBBAlphaMtEph.rs,5687
pub mod BSTSetBBAlphaMtEph {BSTSetBBAlphaMtEph3,78
    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord> {BSTSetBBAlphaMtEph10,307
        tree: BSTBBAlphaMtEph<T>,tree11,362
    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;BSTSetBBAlphaMt14,403
    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetBBAlphaMtEphTrait16,461
        fn empty() -> Self;empty17,527
        fn singleton(value: T) -> Self;singleton18,555
        fn size(&self) -> N;size19,595
        fn is_empty(&self) -> B;is_empty20,624
        fn find(&self, value: &T) -> Option<T>;find21,657
        fn contains(&self, value: &T) -> B;contains22,705
        fn minimum(&self) -> Option<T>;minimum23,749
        fn maximum(&self) -> Option<T>;maximum24,789
        fn insert(&mut self, value: T);insert25,829
        fn delete(&mut self, target: &T);delete26,869
        fn union(&self, other: &Self) -> Self;union27,911
        fn intersection(&self, other: &Self) -> Self;intersection28,958
        fn difference(&self, other: &Self) -> Self;difference29,1012
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1064
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1119
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1174
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1236
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1306
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1374
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T>;as_tree36,1425
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph39,1482
        pub fn empty() -> Self {empty40,1534
        pub fn singleton(value: T) -> Self {singleton46,1657
        pub fn size(&self) -> N { self.tree.size() }size52,1818
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1872
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1934
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,2012
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2090
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2158
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2226
        pub fn delete(&mut self, target: &T) {delete66,2299
        pub fn union(&self, other: &Self) -> Self {union74,2588
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2887
        pub fn difference(&self, other: &Self) -> Self {difference99,3465
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4042
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4734
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5047
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5403
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5812
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6076
        pub fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree178,6156
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6225
        fn rebuild_from_vec(values: Vec<T>) -> BSTBBAlphaMtEph<T> {rebuild_from_vec182,6316
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6543
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph202,6830
        fn empty() -> Self { Self::empty() }empty203,6913
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6959
        fn size(&self) -> N { self.tree.size() }size207,7026
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7076
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7134
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7208
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7282
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7346
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7410
        fn delete(&mut self, target: &T) { BSTSetBBAlphaMtEph::delete(self, target) }delete221,7479
        fn union(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::union(self, other) }union223,7566
        fn intersection(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::intersection(self, otintersection225,7656
        fn difference(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::difference(self, other)difference227,7760
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetBBAlphaMtEph::split(self, pivot) }split229,7860
        fn join_pair(left: Self, right: Self) -> Self { BSTSetBBAlphaMtEph::join_pair(left, righjoin_pair231,7958
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetBBAlphaMtEph::join_m(left, join_m233,8060
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetBBAlphaMtEph::filtefilter235,8173
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetBBAlphaMtEph::reduce(reduce237,8291
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8406
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree241,8482
    macro_rules! BSTSetBBAlphaMtEphLit {BSTSetBBAlphaMtEphLit245,8573
    fn _BSTSetBBAlphaMtEphLit_type_checks() {_BSTSetBBAlphaMtEphLit_type_checks257,9172

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTTreapStEph.rs,4427
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
    pub struct BSTTreapStEph<T: StT + Ord> {BSTTreapStEph32,732
        root: Link<T>,root33,777
    pub type BSTreeTreap<T> = BSTTreapStEph<T>;BSTreeTreap36,807
    pub trait BSTTreapStEphTrait<T: StT + Ord> {BSTTreapStEphTrait38,856
        fn new() -> Self;new39,905
        fn size(&self) -> N;size40,931
        fn is_empty(&self) -> B;is_empty41,960
        fn height(&self) -> N;height42,993
        fn insert(&mut self, value: T);insert43,1024
        fn find(&self, target: &T) -> Option<&T>;find44,1064
        fn contains(&self, target: &T) -> B;contains45,1114
        fn minimum(&self) -> Option<&T>;minimum46,1159
        fn maximum(&self) -> Option<&T>;maximum47,1200
        fn in_order(&self) -> ArrayStPerS<T>;in_order48,1241
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order49,1287
    impl<T: StT + Ord> Default for BSTTreapStEph<T> {BSTTreapStEph52,1341
        fn default() -> Self { Self::new() }default53,1395
    impl<T: StT + Ord> BSTTreapStEph<T> {BSTTreapStEph56,1447
        pub fn new() -> Self { BSTTreapStEph { root: None } }new57,1489
        pub fn size(&self) -> N { Self::size_link(&self.root) }size59,1552
        pub fn is_empty(&self) -> B {is_empty61,1617
        pub fn height(&self) -> N {height69,1784
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec70,1820
        pub fn insert(&mut self, value: T) {insert79,2116
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find84,2265
        pub fn contains(&self, target: &T) -> B {contains86,2359
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum94,2549
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum96,2625
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order98,2701
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order104,2919
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link110,3139
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate112,3222
        fn rotate_left(link: &mut Link<T>) {rotate_left114,3341
        fn rotate_right(link: &mut Link<T>) {rotate_right128,3796
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link142,4252
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link167,5272
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link182,5789
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link192,6110
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect202,6433
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect210,6720
    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {BSTTreapStEph219,7016
        fn new() -> Self { BSTTreapStEph::new() }new220,7084
        fn size(&self) -> N { BSTTreapStEph::size(self) }size222,7135
        fn is_empty(&self) -> B { BSTTreapStEph::is_empty(self) }is_empty224,7194
        fn height(&self) -> N { BSTTreapStEph::height(self) }height226,7261
        fn insert(&mut self, value: T) { BSTTreapStEph::insert(self, value) }insert228,7324
        fn find(&self, target: &T) -> Option<&T> { BSTTreapStEph::find(self, target) }find230,7403
        fn contains(&self, target: &T) -> B { BSTTreapStEph::contains(self, target) }contains232,7491
        fn minimum(&self) -> Option<&T> { BSTTreapStEph::minimum(self) }minimum234,7578
        fn maximum(&self) -> Option<&T> { BSTTreapStEph::maximum(self) }maximum236,7652
        fn in_order(&self) -> ArrayStPerS<T> { BSTTreapStEph::in_order(self) }in_order238,7726
        fn pre_order(&self) -> ArrayStPerS<T> { BSTTreapStEph::pre_order(self) }pre_order240,7806
    macro_rules! BSTTreapStEphLit {BSTTreapStEphLit244,7914
    fn _BSTTreapStEphLit_type_checks() {_BSTTreapStEphLit_type_checks256,8415

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BBTEph.rs,2121
pub mod BBTEph {BBTEph3,56
    pub enum BBTree<T: StT> {BBTree10,301
        Leaf,Leaf11,331
        Node(Box<BBNode<T>>),Node12,345
    pub struct BBNode<T: StT> {BBNode16,425
        pub(crate) left: BBTree<T>,left17,457
        pub(crate) value: T,value18,493
        pub(crate) right: BBTree<T>,right19,522
    impl<T: StT> BBNode<T> {BBNode22,566
        fn new(left: BBTree<T>, value: T, right: BBTree<T>) -> Self { BBNode { left, value, righnew23,595
    pub trait BBTEphTrait<T: StT> {BBTEphTrait26,704
        fn leaf() -> Self;leaf27,740
        fn node(left: Self, value: T, right: Self) -> Self;node28,767
        fn is_leaf(&self) -> B;is_leaf29,827
        fn in_order(&self) -> ArrayStPerS<T>;in_order30,859
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order31,905
        fn height(&self) -> N;height32,952
        fn size(&self) -> N;size33,983
    impl<T: StT> BBTree<T> {BBTree36,1019
        pub fn leaf() -> Self { BBTree::Leaf }leaf37,1048
        pub fn node(left: BBTree<T>, value: T, right: BBTree<T>) -> Self {node39,1096
        pub fn is_leaf(&self) -> B {is_leaf43,1250
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order50,1427
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order63,2084
        pub fn height(&self) -> N {height76,2742
        pub fn size(&self) -> N {size87,3077
    impl<T: StT> BBTEphTrait<T> for BBTree<T> {BBTree95,3286
        fn leaf() -> Self { BBTree::leaf() }leaf96,3334
        fn node(left: Self, value: T, right: Self) -> Self { BBTree::node(left, value, right) }node98,3380
        fn is_leaf(&self) -> B { BBTree::is_leaf(self) }is_leaf100,3477
        fn in_order(&self) -> ArrayStPerS<T> { BBTree::in_order(self) }in_order102,3535
        fn pre_order(&self) -> ArrayStPerS<T> { BBTree::pre_order(self) }pre_order104,3608
        fn height(&self) -> N { BBTree::height(self) }height106,3683
        fn size(&self) -> N { BBTree::size(self) }size108,3739
    macro_rules! BBNodeLit {BBNodeLit112,3817
    fn _BBNodeLit_type_checks() {_BBNodeLit_type_checks119,4035

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap03/InsertionSortSt.rs.saved,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap03/InsertionSortSt.rs,390
pub mod InsertionSortSt {InsertionSortSt3,51
    pub trait InsertionSortStTrait {InsertionSortStTrait5,78
        fn insSort<T: Ord + Clone>(&self, slice: &mut [T]);insSort8,214
    pub struct InsertionSortSt;InsertionSortSt12,317
    impl InsertionSortStTrait for InsertionSortSt {InsertionSortSt14,350
        fn insSort<T: Ord + Clone>(&self, slice: &mut [T]) {insSort15,402

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/MappingStEph.rs,2266
pub mod MappingStEph {MappingStEph3,72
    pub struct Mapping<A, B> {Mapping14,394
        rel: Relation<A, B>,rel15,425
    pub trait MappingStEphTrait<MappingStEphTrait18,461
        fn empty() -> Mapping<X, Y>;empty25,644
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y>;FromVec29,778
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation33,932
        fn size(&self) -> N;size37,1087
        fn domain(&self) -> Set<X>;domain41,1213
        fn range(&self) -> Set<Y>;range45,1346
        fn mem(&self, a: &X, b: &Y) -> B;mem49,1474
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>>;iter51,1517
    impl<A: Eq + Hash, B: Eq + Hash> Mapping<A, B> {Mapping54,1600
        fn unique_pairs_from_iter<I: IntoIterator<Item = Pair<A, B>>>(iter: I) -> Set<Pair<A, B>unique_pairs_from_iter55,1653
    impl<A: StT + Hash, B: StT + Hash>Mapping65,2031
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq68,2112
    impl<A: StT + Hash, B: StT + Hash> EqMapping70,2187
    impl<A: StT + Hash, B: StT + Hash> Debug for Mapping<A, B> {Mapping75,2268
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.rel, f) }fmt76,2333
    impl<A: StT + Hash, B: StT + Hash> Display for Mapping<A, B> {Mapping78,2423
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.rel, f) }fmt79,2490
        impl<Mapping82,2583
        fn empty() -> Mapping<X, Y> {empty87,2709
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y> {FromVec93,2870
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation100,3115
        fn size(&self) -> N { self.rel.size() }size107,3381
        fn domain(&self) -> Set<X> { self.rel.domain() }domain109,3430
        fn range(&self) -> Set<Y> { self.rel.range() }range111,3488
        fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem113,3544
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.rel.iter() }iter115,3609
    macro_rules! MappingLit {MappingLit119,3731
    fn _MappingLit_type_checks() {_MappingLit_type_checks130,4296
    pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise136,4527

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/SetStEph.rs,3507
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
        pub fn mem(&self, x: &T) -> B {mem118,3938
        pub fn union(&self, other: &Set<T>) -> Set<T>union126,4112
        pub fn intersection(&self, other: &Set<T>) -> Set<T>intersection137,4386
        pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition148,4743
        pub fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct166,5298
        pub fn insert(&mut self, x: T) -> &mut Self {insert180,5744
        pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter185,5867
        pub fn FromVec(v: Vec<T>) -> Set<T> {FromVec187,5959
    impl<T: StT + Hash> SetStEphTrait<T> for Set<T> {Set196,6183
        fn empty() -> Set<T> { Set { data: HashSet::new() } }empty197,6237
        fn singleton(x: T) -> Set<T> {singleton199,6300
        fn size(&self) -> N { self.data.len() }size205,6462
        fn mem(&self, x: &T) -> B {mem207,6511
        fn union(&self, other: &Set<T>) -> Set<T>union215,6681
        fn intersection(&self, other: &Set<T>) -> Set<T>intersection226,6951
        fn partition(&self, parts: &Set<Set<T>>) -> B {partition237,7304
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct255,7855
        fn insert(&mut self, x: T) -> &mut Self {insert269,8297
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter274,8416
        fn FromVec(v: Vec<T>) -> Set<T> {FromVec276,8504
    macro_rules! SetLit {SetLit286,8744
    fn _SetLit_type_checks() {_SetLit_type_checks298,9086
    pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise304,9282
        let _s0: Set<&'static str> = SetLit![];str305,9328

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap05/RelationStEph.rs,2199
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
        pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B> {FromVec54,1528
    impl<A: StT + Hash, B: StT + Hash> PartialEq for Relation<A, B> {Relation59,1656
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq60,1726
    impl<A: StT + Hash, B: StT + Hash> Eq for Relation<A, B> {}Relation63,1806
    impl<A: StT + Hash, B: StT + Hash> Debug for Relation<A, B> {Relation65,1871
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }fmt66,1937
    impl<A: StT + Hash, B: StT + Hash> Display for Relation<A, B> {Relation69,2040
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) fmt70,2108
    impl<X: StT + Hash, Y: StT + Hash> RelationStEphTrait<X, Y> for Relation<X, Y> {Relation73,2213
        fn empty() -> Relation<X, Y> { Relation { pairs: SetLit![] } }empty74,2298
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y> { Relation { pairs } }FromSet76,2370
        fn size(&self) -> N { self.pairs.size() }size78,2455
        fn domain(&self) -> Set<X>domain80,2506
        fn range(&self) -> Set<Y>range91,2773
        fn mem(&self, a: &X, b: &Y) -> Bmem102,3039
        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }iter114,3315
    macro_rules! RelationLit {RelationLit118,3411
    fn _RelationLit_type_checks() {_RelationLit_type_checks134,4288
    pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise140,4523

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphFloat.rs,1231
pub mod WeightedUnDirGraphMtEphFloat {WeightedUnDirGraphMtEphFloat3,107
    pub type WeightedUnDirGraphMtEphFloat<V> = LabUnDirGraphMtEph<V, OrderedF64>;WeightedUnDirGraphMtEphFloat12,463
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphFloat<V> {WeightedUnDirGraphMtEphFloat15,652
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges17,774
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge31,1342
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight36,1543
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges41,1750
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted50,2111
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight63,2668
        pub fn vertex_degree(&self, v: &V) -> usize {vertex_degree68,2904
    macro_rules! WeightedUnDirGraphMtEphFloatLit {WeightedUnDirGraphMtEphFloatLit74,3032
    pub fn __weighted_undir_graph_mt_float_macro_typecheck_exercise() {__weighted_undir_graph_mt_float_macro_typecheck_exercise86,3657

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphInt.rs,1187
pub mod WeightedUnDirGraphStEphInt {WeightedUnDirGraphStEphInt3,101
    pub type WeightedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;WeightedUnDirGraphStEphInt12,432
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphInt<V> {WeightedUnDirGraphStEphInt15,588
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,702
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) {add_weighted_edge31,1256
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> {get_edge_weight36,1443
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges41,1636
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted50,1977
        pub fn total_weight(&self) -> i32 {total_weight63,2514
        pub fn vertex_degree(&self, v: &V) -> usize {vertex_degree68,2700
        pub fn is_connected(&self) -> bool {is_connected73,2887
    macro_rules! WeightedUnDirGraphStEphIntLit {WeightedUnDirGraphStEphIntLit102,3951
    pub fn __weighted_undir_graph_st_int_macro_typecheck_exercise() {__weighted_undir_graph_st_int_macro_typecheck_exercise114,4547

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphStEph.rs,2635
pub mod UnDirGraphStEph {UnDirGraphStEph3,80
    pub struct UnDirGraphStEph<V: StT + Hash> {UnDirGraphStEph12,314
        V: Set<V>,V13,362
        E: Set<Edge<V>>,E14,381
    pub trait UnDirGraphStEphTrait<V: StT + Hash> {UnDirGraphStEphTrait17,413
        fn empty() -> UnDirGraphStEph<V>;empty20,557
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V>;FromSets23,707
        fn vertices(&self) -> &Set<V>;vertices26,870
        fn edges(&self) -> &Set<Edge<V>>;edges29,1001
        fn sizeV(&self) -> N;sizeV32,1135
        fn sizeE(&self) -> N;sizeE35,1257
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1379
        fn NG(&self, v: &V) -> Set<V>;NG41,1522
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1679
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident47,1829
        fn Degree(&self, v: &V) -> N;Degree50,1978
    impl<V: StT + Hash> UnDirGraphStEphTrait<V> for UnDirGraphStEph<V> {UnDirGraphStEph53,2023
        fn empty() -> UnDirGraphStEph<V> {empty54,2096
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E }FromSets60,2253
        fn vertices(&self) -> &Set<V> { &self.V }vertices61,2352
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges62,2402
        fn sizeV(&self) -> N { self.V.size() }sizeV63,2455
        fn sizeE(&self) -> N { self.E.size() }sizeE64,2502
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor66,2550
        fn NG(&self, v: &V) -> Set<V> {NG76,2874
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices88,3236
        fn Incident(&self, e: &Edge<V>, v: &V) -> B {Incident97,3508
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree105,3697
    impl<V: StT + Hash> Debug for UnDirGraphStEph<V> {UnDirGraphStEph108,3763
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt109,3818
    impl<V: StT + Hash> Display for UnDirGraphStEph<V> {UnDirGraphStEph117,4038
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} E={:?}", self.V, self.Efmt118,4095
    impl<V: StT + Hash> PartialEq for UnDirGraphStEph<V> {UnDirGraphStEph121,4202
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq122,4261
    impl<V: StT + Hash> Eq for UnDirGraphStEph<V> {}UnDirGraphStEph124,4353
    macro_rules! UnDirGraphStEphLit {UnDirGraphStEphLit127,4427
    fn _UnDirGraphStEphLit_type_checks() {_UnDirGraphStEphLit_type_checks145,5552
    pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise151,5815

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphInt.rs,1442
pub mod WeightedDirGraphStEphInt {WeightedDirGraphStEphInt3,99
    pub type WeightedDirGraphStEphInt<V> = LabDirGraphStEph<V, i32>;WeightedDirGraphStEphInt12,422
    impl<V: StT + Hash> WeightedDirGraphStEphInt<V> {WeightedDirGraphStEphInt15,572
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,678
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) {add_weighted_edge31,1222
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> {get_edge_weight36,1412
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges41,1610
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted50,1959
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted61,2372
        pub fn total_weight(&self) -> i32 {total_weight72,2782
        pub fn edges_above_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_above_weight77,2958
        pub fn edges_below_weight(&self, threshold: i32) -> Set<(V, V, i32)> {edges_below_weight88,3403
    macro_rules! WeightedDirGraphStEphIntLit {WeightedDirGraphStEphIntLit100,3820
    fn _WeightedDirGraphStEphIntLit_type_checks() {_WeightedDirGraphStEphIntLit_type_checks112,4406
    pub fn __weighted_dir_graph_st_int_macro_typecheck_exercise() {__weighted_dir_graph_st_int_macro_typecheck_exercise118,4698

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphStEphFloat.rs,1502
pub mod WeightedUnDirGraphStEphFloat {WeightedUnDirGraphStEphFloat29,1111
    pub type WeightedUnDirGraphStEphFloat<V> = LabUnDirGraphStEph<V, OrderedF64>;WeightedUnDirGraphStEphFloat38,1451
    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphFloat<V> {WeightedUnDirGraphStEphFloat41,1623
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges43,1739
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {add_weighted_edge57,2307
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> {get_edge_weight62,2508
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges67,2715
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {neighbors_weighted76,3070
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight89,3621
        pub fn vertex_degree(&self, v: &V) -> usize {vertex_degree94,3857
        pub fn is_connected(&self) -> bool {is_connected99,4044
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge127,5122
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge134,5403
    macro_rules! WeightedUnDirGraphStEphFloatLit {WeightedUnDirGraphStEphFloatLit142,5670
    pub fn __weighted_undir_graph_st_float_macro_typecheck_exercise() {__weighted_undir_graph_st_float_macro_typecheck_exercise154,6295

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedUnDirGraphMtEphInt.rs,1125
pub mod WeightedUnDirGraphMtEphInt {WeightedUnDirGraphMtEphInt3,100
    pub type WeightedUnDirGraphMtEphInt<V> = LabUnDirGraphMtEph<V, i32>;WeightedUnDirGraphMtEphInt12,447
    impl<V: StT + MtT + Hash + Ord> WeightedUnDirGraphMtEphInt<V> {WeightedUnDirGraphMtEphInt15,620
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,740
        pub fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) {add_weighted_edge31,1294
        pub fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> {get_edge_weight36,1481
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges41,1674
        pub fn neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {neighbors_weighted50,2021
        pub fn total_weight(&self) -> i32 {total_weight63,2564
        pub fn vertex_degree(&self, v: &V) -> usize {vertex_degree68,2750
    macro_rules! WeightedUnDirGraphMtEphIntLit {WeightedUnDirGraphMtEphIntLit74,2878
    pub fn __weighted_undir_graph_mt_int_macro_typecheck_exercise() {__weighted_undir_graph_mt_int_macro_typecheck_exercise86,3474

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphMtEph.rs,2619
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
        fn vertices(&self) -> &Set<V> {vertices55,1771
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> {labeled_edges59,1849
        fn edges(&self) -> Set<Edge<V>> {edges63,1949
        fn add_vertex(&mut self, v: V) {add_vertex71,2226
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge75,2315
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label86,2707
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge97,3147
        fn neighbors(&self, v: &V) -> Set<V> {neighbors107,3495
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge119,3934
    impl<V, L> Display for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph127,4354
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt132,4485
    impl<V, L> Debug for LabUnDirGraphMtEph<V, L>LabUnDirGraphMtEph137,4647
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt142,4776
    macro_rules! LabUnDirGraphMtEphLit {LabUnDirGraphMtEphLit149,5006
    fn _LabUnDirGraphMtEphLit_type_checks() {_LabUnDirGraphMtEphLit_type_checks172,6212
    pub fn __lab_undir_graph_mt_macro_typecheck_exercise() {__lab_undir_graph_mt_macro_typecheck_exercise178,6491

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphFloat.rs,1253
pub mod WeightedDirGraphMtEphFloat {WeightedDirGraphMtEphFloat3,105
    pub type WeightedDirGraphMtEphFloat<V> = LabDirGraphMtEph<V, OrderedF64>;WeightedDirGraphMtEphFloat12,453
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphFloat<V> {WeightedDirGraphMtEphFloat15,636
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges17,750
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge31,1308
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight36,1512
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges41,1724
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted50,2093
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted61,2523
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight72,2950
    macro_rules! WeightedDirGraphMtEphFloatLit {WeightedDirGraphMtEphFloatLit78,3145
    pub fn __weighted_dir_graph_mt_float_macro_typecheck_exercise() {__weighted_dir_graph_mt_float_macro_typecheck_exercise90,3758

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/UnDirGraphMtEph.rs,2723
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
        fn NG(&self, v: &V) -> Set<V> {NG75,2870
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices87,3238
        fn Incident(&self, e: &Edge<V>, v: &V) -> B {Incident96,3510
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree104,3699
    impl<V: StT + MtT + Hash> std::fmt::Debug for UnDirGraphMtEph<V> {UnDirGraphMtEph107,3765
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt108,3836
    impl<V: StT + MtT + Hash> std::fmt::Display for UnDirGraphMtEph<V> {UnDirGraphMtEph116,4076
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} E={fmt117,4149
    impl<V: StT + MtT + Hash> PartialEq for UnDirGraphMtEph<V> {UnDirGraphMtEph120,4276
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq121,4341
    impl<V: StT + MtT + Hash> Eq for UnDirGraphMtEph<V> {}UnDirGraphMtEph123,4433
    macro_rules! UnDirGraphMtEphLit {UnDirGraphMtEphLit126,4513
    fn _UnDirGraphMtEphLit_type_checks() {_UnDirGraphMtEphLit_type_checks144,5638
    pub fn __undirgraph_mt_macro_typecheck_exercise() {__undirgraph_mt_macro_typecheck_exercise150,5890

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphStEph.rs,2552
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
        fn vertices(&self) -> &Set<V> {vertices55,1667
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> {labeled_arcs59,1745
        fn arcs(&self) -> Set<Edge<V>> {arcs63,1843
        fn add_vertex(&mut self, v: V) {add_vertex71,2106
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc75,2195
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label81,2430
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc90,2731
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors99,3005
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors109,3327
    impl<V, L> Display for LabDirGraphStEph<V, L>LabDirGraphStEph120,3654
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt125,3791
    impl<V, L> Debug for LabDirGraphStEph<V, L>LabDirGraphStEph130,3950
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt135,4085
    macro_rules! LabDirGraphStEphLit {LabDirGraphStEphLit142,4311
    fn _LabDirGraphStEphLit_type_checks() {_LabDirGraphStEphLit_type_checks154,5103
    pub fn __lab_dir_graph_macro_typecheck_exercise() {__lab_dir_graph_macro_typecheck_exercise160,5374

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphStEph.rs,3465
pub mod DirGraphStEph {DirGraphStEph3,77
    pub struct DirGraphStEph<V: StT + Hash> {DirGraphStEph12,309
        V: Set<V>,V13,355
        A: Set<Edge<V>>,A14,374
    pub trait DirGraphStEphTrait<V: StT + Hash> {DirGraphStEphTrait17,406
        fn empty() -> DirGraphStEph<V>;empty20,548
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V>;FromSets23,696
        fn vertices(&self) -> &Set<V>;vertices26,857
        fn arcs(&self) -> &Set<Edge<V>>;arcs29,988
        fn sizeV(&self) -> N;sizeV32,1121
        fn sizeA(&self) -> N;sizeA35,1243
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor38,1365
        fn NG(&self, v: &V) -> Set<V>;NG41,1508
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices44,1665
        fn NPlus(&self, v: &V) -> Set<V>;NPlus47,1819
        fn NMinus(&self, v: &V) -> Set<V>;NMinus50,1957
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices53,2118
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices56,2297
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident59,2451
        fn Degree(&self, v: &V) -> N;Degree62,2603
        fn InDegree(&self, v: &V) -> N;InDegree65,2737
        fn OutDegree(&self, v: &V) -> N;OutDegree68,2873
    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {DirGraphStEph71,2921
        fn empty() -> DirGraphStEph<V> {empty72,2990
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }FromSets78,3143
        fn vertices(&self) -> &Set<V> { &self.V }vertices79,3238
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs80,3288
        fn sizeV(&self) -> N { self.V.size() }sizeV81,3340
        fn sizeA(&self) -> N { self.A.size() }sizeA82,3387
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor84,3435
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG94,3712
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices96,3769
        fn NPlus(&self, v: &V) -> Set<V> {NPlus105,4041
        fn NMinus(&self, v: &V) -> Set<V> {NMinus115,4323
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices125,4606
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices134,4888
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B {Incident143,5174
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree151,5366
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree152,5428
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree153,5493
    impl<V: StT + Hash> Debug for DirGraphStEph<V> {DirGraphStEph156,5565
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt157,5618
    impl<V: StT + Hash> Display for DirGraphStEph<V> {DirGraphStEph165,5836
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.Afmt166,5891
    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {DirGraphStEph169,5998
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq170,6055
    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}DirGraphStEph172,6147
    macro_rules! DirGraphStEphLit {DirGraphStEphLit175,6219
    fn _DirGraphStEphLit_type_checks() {_DirGraphStEphLit_type_checks192,7312
    pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise198,7567

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/DirGraphMtEph.rs,3554
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
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG93,3702
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices95,3759
        fn NPlus(&self, v: &V) -> Set<V> {NPlus104,4031
        fn NMinus(&self, v: &V) -> Set<V> {NMinus114,4316
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices124,4602
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices133,4884
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B {Incident142,5170
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree150,5362
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree151,5424
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree152,5489
    impl<V: StT + MtT + Hash> std::fmt::Debug for DirGraphMtEph<V> {DirGraphMtEph155,5561
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt156,5630
    impl<V: StT + MtT + Hash> std::fmt::Display for DirGraphMtEph<V> {DirGraphMtEph164,5868
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} A={fmt165,5939
    impl<V: StT + MtT + Hash> PartialEq for DirGraphMtEph<V> {DirGraphMtEph168,6066
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq169,6129
    impl<V: StT + MtT + Hash> Eq for DirGraphMtEph<V> {}DirGraphMtEph171,6221
    macro_rules! DirGraphMtEphLit {DirGraphMtEphLit174,6299
    fn _DirGraphMtEphLit_type_checks() {_DirGraphMtEphLit_type_checks191,7392
    pub fn __dirgraph_mt_macro_typecheck_exercise() {__dirgraph_mt_macro_typecheck_exercise197,7636

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphStEphFloat.rs,1906
pub mod WeightedDirGraphStEphFloat {WeightedDirGraphStEphFloat29,1076
    pub type WeightedDirGraphStEphFloat<V> = LabDirGraphStEph<V, OrderedF64>;WeightedDirGraphStEphFloat38,1408
    impl<V: StT + Hash> WeightedDirGraphStEphFloat<V> {WeightedDirGraphStEphFloat41,1574
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, OrderedFloat<f64>)>) -> Sfrom_weighted_edges43,1682
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {add_weighted_edge57,2240
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {get_edge_weight62,2444
        pub fn weighted_edges(&self) -> Set<(V, V, OrderedFloat<f64>)> {weighted_edges67,2656
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {out_neighbors_weighted76,3019
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, OrderedFloat<f64>)> {in_neighbors_weighted87,3446
        pub fn total_weight(&self) -> OrderedFloat<f64> {total_weight98,3870
        pub fn edges_above_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_above_weight103,4096
        pub fn edges_below_weight(&self, threshold: OrderedFloat<f64>) -> Set<(V, V, OrderedFloaedges_below_weight114,4569
        pub fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {min_weight_edge125,5028
        pub fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {max_weight_edge132,5308
        pub fn scale_weights(&mut self, factor: OrderedFloat<f64>) {scale_weights139,5590
    macro_rules! WeightedDirGraphStEphFloatLit {WeightedDirGraphStEphFloatLit157,6237
    fn _WeightedDirGraphStEphFloatLit_type_checks() {_WeightedDirGraphStEphFloatLit_type_checks169,6850
    pub fn __weighted_dir_graph_st_float_macro_typecheck_exercise() {__weighted_dir_graph_st_float_macro_typecheck_exercise175,7152

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/WeightedDirGraphMtEphInt.rs,1132
pub mod WeightedDirGraphMtEphInt {WeightedDirGraphMtEphInt3,98
    pub type WeightedDirGraphMtEphInt<V> = LabDirGraphMtEph<V, i32>;WeightedDirGraphMtEphInt12,437
    impl<V: StT + MtT + Hash> WeightedDirGraphMtEphInt<V> {WeightedDirGraphMtEphInt15,604
        pub fn from_weighted_edges(vertices: Set<V>, edges: Set<(V, V, i32)>) -> Self {from_weighted_edges17,716
        pub fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) {add_weighted_edge31,1260
        pub fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> {get_edge_weight36,1450
        pub fn weighted_edges(&self) -> Set<(V, V, i32)> {weighted_edges41,1648
        pub fn out_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {out_neighbors_weighted50,2003
        pub fn in_neighbors_weighted(&self, v: &V) -> Set<(V, i32)> {in_neighbors_weighted61,2419
        pub fn total_weight(&self) -> i32 {total_weight72,2832
    macro_rules! WeightedDirGraphMtEphIntLit {WeightedDirGraphMtEphIntLit78,2977
    pub fn __weighted_dir_graph_mt_int_macro_typecheck_exercise() {__weighted_dir_graph_mt_int_macro_typecheck_exercise90,3563

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabUnDirGraphStEph.rs,2611
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
        fn vertices(&self) -> &Set<V> {vertices55,1713
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> {labeled_edges59,1791
        fn edges(&self) -> Set<Edge<V>> {edges63,1891
        fn add_vertex(&mut self, v: V) {add_vertex71,2162
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge75,2251
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label86,2637
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge97,3078
        fn neighbors(&self, v: &V) -> Set<V> {neighbors108,3491
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge120,3924
    impl<V, L> Display for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph128,4344
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt133,4464
    impl<V, L> Debug for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph138,4626
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt143,4744
    macro_rules! LabUnDirGraphStEphLit {LabUnDirGraphStEphLit150,4974
    fn _LabUnDirGraphStEphLit_type_checks() {_LabUnDirGraphStEphLit_type_checks173,6180
    pub fn __lab_undir_graph_macro_typecheck_exercise() {__lab_undir_graph_macro_typecheck_exercise179,6459

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap06/LabDirGraphMtEph.rs,2560
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
        fn vertices(&self) -> &Set<V> {vertices55,1725
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> {labeled_arcs59,1803
        fn arcs(&self) -> Set<Edge<V>> {arcs63,1901
        fn add_vertex(&mut self, v: V) {add_vertex71,2170
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc75,2259
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label81,2500
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc90,2801
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors99,3075
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors109,3400
    impl<V, L> Display for LabDirGraphMtEph<V, L>LabDirGraphMtEph120,3730
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt125,3853
    impl<V, L> Debug for LabDirGraphMtEph<V, L>LabDirGraphMtEph130,4012
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt135,4133
    macro_rules! LabDirGraphMtEphLit {LabDirGraphMtEphLit142,4359
    fn _LabDirGraphMtEphLit_type_checks() {_LabDirGraphMtEphLit_type_checks154,5151
    pub fn __lab_dir_graph_mt_macro_typecheck_exercise() {__lab_dir_graph_mt_macro_typecheck_exercise160,5422

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap11/FibonacciMt.rs,578
pub mod FibonacciMt {FibonacciMt4,131
    pub struct FibonacciMt;FibonacciMt7,198
    pub trait FibonacciMtTrait {FibonacciMtTrait9,227
        fn fib(n: N) -> N;fib10,260
    impl FibonacciMt {FibonacciMt13,294
        pub fn fib(n: N) -> N {fib14,317
    impl FibonacciMtTrait for FibonacciMt {FibonacciMt24,589
        fn fib(n: N) -> N {fib25,633
    mod tests {tests31,727
        fn fib_base_cases() {fib_base_cases36,848
        fn fib_small_values() {fib_small_values42,1001
        fn trait_and_inherent_agree() {trait_and_inherent_agree49,1205

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap12/Exercise12_5.rs,1372
pub mod Exercise12_5 {Exercise12_53,86
    struct Node<T: StTInMtT> {Node10,260
        value: ManuallyDrop<T>,value11,291
        next: *mut Node<T>,next12,323
    pub struct ConcurrentStackMt<T: StTInMtT> {ConcurrentStackMt16,419
        head: AtomicPtr<Node<T>>,head17,467
    pub trait ConcurrentStackMtTrait<T: StTInMtT> {ConcurrentStackMtTrait20,508
        fn new() -> Self;new21,560
        fn push(&self, value: T);push22,586
        fn pop(&self) -> Option<T>;pop23,620
        fn is_empty(&self) -> bool;is_empty24,656
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt27,699
        fn raw_pop(&self) -> Option<*mut Node<T>> {raw_pop28,744
    impl<T: StTInMtT> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {ConcurrentStackMt46,1293
        fn new() -> Self {new47,1368
        fn push(&self, value: T) {push51,1473
        fn pop(&self) -> Option<T> {pop68,2110
        fn is_empty(&self) -> bool {is_empty75,2349
    impl<T: StTInMtT> Default for ConcurrentStackMt<T> {ConcurrentStackMt80,2459
        fn default() -> Self { Self::new() }default81,2516
    impl<T: StTInMtT> Drop for ConcurrentStackMt<T> {ConcurrentStackMt84,2568
        fn drop(&mut self) {drop85,2622
    impl<T: StTInMtT> ConcurrentStackMt<T> {ConcurrentStackMt98,3026
        pub fn drain(&self) -> Vec<T> {drain100,3146

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
    pub struct SpinLock {SpinLock12,277
        ticket: AtomicUsize,ticket13,303
        turn: AtomicUsize,turn14,332
    pub trait SpinLockTrait {SpinLockTrait17,366
        fn new() -> Self;new18,396
        fn lock(&self);lock19,422
        fn unlock(&self);unlock20,446
    impl SpinLock {SpinLock23,479
        pub fn new() -> Self {new24,499
        pub fn lock(&self) {lock28,621
        pub fn unlock(&self) {unlock35,844
        pub fn with_lock<T>(&self, action: impl FnOnce() -> T) -> T {with_lock39,941
    impl SpinLockTrait for SpinLock {SpinLock47,1134
        fn new() -> Self { SpinLock::new() }new48,1172
        fn lock(&self) { SpinLock::lock(self) }lock50,1218
        fn unlock(&self) { SpinLock::unlock(self) }unlock52,1267
    impl Default for SpinLock {SpinLock55,1326
        fn default() -> Self { SpinLock::new() }default56,1358
    pub fn parallel_increment(iterations: N) -> usize {parallel_increment59,1414

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStEphChap18.rs,2834
pub mod ArraySeqStEphChap {ArraySeqStEphChap3,51
    pub trait ArraySeqStEphChap18Trait<T: StT> {ArraySeqStEphChap18Trait7,168
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate10,309
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map13,473
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append16,670
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter19,852
        fn update(a: &mut ArraySeqStEphS<T>, item_at: (N, T)) -> &mut ArraySeqStEphS<T>;update22,1031
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject25,1240
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject28,1461
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate29,1563
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes30,1650
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce31,1766
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan32,1846
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten33,1945
        fn collect<A: StT, Bv: StT>(collect34,2026
    impl<T: StT> ArraySeqStEphChap18Trait<T> for ArraySeqStEphS<T> {ArraySeqStEphS40,2212
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate41,2281
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map48,2527
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append61,3066
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter78,3774
        fn update(a: &mut ArraySeqStEphS<T>, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update90,4199
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject93,4341
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject101,4667
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,4817
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes111,5052
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce124,5616
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan131,5845
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten145,6403
        fn collect<A: StT, Bv: StT>(collect173,7494

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/MathSeq.rs,4286
pub mod MathSeq {MathSeq8,306
    pub struct MathSeqS<T: StT> {MathSeqS17,588
        data: Vec<T>,data18,622
    impl<T: StT> PartialEq for MathSeqS<T> {MathSeqS21,651
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq22,696
    impl<T: StT> Eq for MathSeqS<T> {}MathSeqS25,774
    impl<T: StT> std::fmt::Debug for MathSeqS<T> {MathSeqS27,814
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt28,865
    impl<T: StT> std::fmt::Display for MathSeqS<T> {MathSeqS33,1021
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt34,1074
    impl<T: StT> MathSeqS<T> {MathSeqS49,1487
        pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }iter52,1610
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }iter_mut55,1777
        pub fn empty() -> Self { Self { data: Vec::new() } }empty59,1960
        pub fn singleton(item: T) -> Self { Self { data: vec![item] } }singleton62,2113
        pub fn from_vec(data: Vec<T>) -> Self { Self { data } }from_vec65,2287
        pub fn with_len(length: N, init_value: T) -> Self {with_len68,2453
    impl<'a, T: StT> IntoIterator for &'a MathSeqS<T> {MathSeqS75,2611
        type Item = &'a T;Item76,2667
        type IntoIter = std::slice::Iter<'a, T>;IntoIter77,2694
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }into_iter78,2743
    impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {MathSeqS81,2816
        type Item = &'a mut T;Item82,2876
        type IntoIter = std::slice::IterMut<'a, T>;IntoIter83,2907
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }into_iter84,2959
    impl<T: StT> IntoIterator for MathSeqS<T> {MathSeqS87,3036
        type Item = T;Item88,3084
        type IntoIter = std::vec::IntoIter<T>;IntoIter89,3107
        fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }into_iter90,3154
    pub trait MathSeqTrait<T: StT + Hash> {MathSeqTrait94,3268
        fn new(length: N, init_value: T) -> Self;new97,3414
        fn empty() -> Self;empty101,3557
        fn singleton(item: T) -> Self;singleton105,3678
        fn length(&self) -> N;length109,3810
        fn nth(&self, index: N) -> &T;nth113,3934
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str>;set117,4066
        fn add_last(&mut self, value: T) -> &mut Self;add_last121,4353
        fn delete_last(&mut self) -> Option<T>;delete_last125,4501
        fn subseq(&self, start: N, length: N) -> &[T];subseq129,4642
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy133,4800
        fn isEmpty(&self) -> B;isEmpty137,4953
        fn isSingleton(&self) -> B;isSingleton141,5078
        fn domain(&self) -> Vec<N>;domain145,5211
        fn range(&self) -> Vec<T>;range149,5344
        fn multiset_range(&self) -> Vec<(N, T)>;multiset_range153,5476
    impl<T: StT + Hash> MathSeqTrait<T> for MathSeqS<T> {MathSeqS156,5532
        fn new(length: N, init_value: T) -> Self {new159,5692
        fn empty() -> Self { MathSeqS { data: Vec::new() } }empty167,5931
        fn singleton(item: T) -> Self { MathSeqS { data: vec![item] } }singleton171,6085
        fn length(&self) -> N { self.data.len() }length175,6250
        fn nth(&self, index: N) -> &T { &self.data[index] }nth179,6393
        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str> {set183,6546
        fn add_last(&mut self, value: T) -> &mut Self {add_last194,7030
        fn delete_last(&mut self) -> Option<T> { self.data.pop() }delete_last201,7241
        fn subseq(&self, start: N, length: N) -> &[T] {subseq205,7401
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy214,7727
        fn isEmpty(&self) -> B {isEmpty228,8196
        fn isSingleton(&self) -> B {isSingleton238,8454
        fn domain(&self) -> Vec<N> { (0..self.data.len()).collect() }domain248,8720
        fn range(&self) -> Vec<T> {range252,8887
        fn multiset_range(&self) -> Vec<(N, T)> {multiset_range265,9354
    macro_rules! MathSeqSLit {MathSeqSLit284,10038
    fn _MathSeqSLit_type_checks() {_MathSeqSLit_type_checks297,10432

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStPer.rs,2877
pub mod LinkedListStPer {LinkedListStPer3,48
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait7,136
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate10,349
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map14,592
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append18,798
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter22,1097
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;update26,1289
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject29,1455
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject32,1622
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate35,1808
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes38,1943
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce41,2147
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan44,2277
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten47,2451
        fn collect<A: StT, Bv: StT>(collect50,2618
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS56,2810
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate57,2877
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map65,3128
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append72,3575
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter84,4217
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T>update94,4687
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject102,5215
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject120,6182
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate137,7118
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes144,7455
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce153,8014
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan168,8835
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten185,9739
        fn collect<A: StT, Bv: StT>(collect197,10431

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtEphChap18.rs,1659
pub mod ArraySeqMtEphChap {ArraySeqMtEphChap3,67
    pub trait ArraySeqMtEphChap18Trait<T: StT> {ArraySeqMtEphChap18Trait7,184
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate8,233
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map9,301
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append10,390
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter11,476
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;update12,563
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject13,652
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject14,753
    impl<T: StT> ArraySeqMtEphChap18Trait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS17,862
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate18,931
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map25,1177
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append38,1688
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter55,2356
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update67,2781
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject71,2941
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject79,3266

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStEph.rs,2806
pub mod ArraySeqStEph{ArraySeqStEph3,51
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait7,139
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate10,274
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map13,438
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append16,635
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter19,817
        fn update(a: &mut ArraySeqStEphS<T>, item_at: (N, T)) -> &mut ArraySeqStEphS<T>;update22,996
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject25,1205
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject28,1426
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate29,1528
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes30,1615
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce31,1731
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan32,1811
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten33,1910
        fn collect<A: StT, Bv: StT>(collect34,1991
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS40,2177
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate41,2240
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map48,2486
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append61,3028
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter78,3742
        fn update(a: &mut ArraySeqStEphS<T>, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update90,4167
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject93,4309
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject101,4635
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,4785
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes111,5020
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce124,5587
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan131,5816
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten145,6377
        fn collect<A: StT, Bv: StT>(collect173,7463

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStPerChap18.rs,3025
pub mod ArraySeqStPerChap {ArraySeqStPerChap3,46
    pub trait ArraySeqStPerChap18Trait<T: StT> {ArraySeqStPerChap18Trait7,163
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,394
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map14,632
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append18,836
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1120
        fn update(a: &ArrayStPerS<T>, item_at: Pair<N, T>) -> ArrayStPerS<T>;update26,1302
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject30,1541
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject34,1758
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate37,1929
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes40,2059
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce43,2261
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan46,2386
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten49,2550
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect52,2702
    impl<T: StT> ArraySeqStPerChap18Trait<T> for ArrayStPerS<T> {ArrayStPerS55,2825
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate56,2891
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map60,3070
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append72,3571
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter86,4100
        fn update(a: &ArrayStPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayStPerS<T> {update95,4435
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject101,4668
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject113,5232
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate123,5674
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes130,5906
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce139,6309
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec140,6387
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan155,6897
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec156,6991
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten177,7761
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect187,8129

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtPerChap18.rs,2829
pub mod ArraySeqMtPerChap {ArraySeqMtPerChap3,82
    pub trait ArraySeqMtPerChap18Trait<T: MtT> {ArraySeqMtPerChap18Trait7,199
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;tabulate10,430
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;map13,667
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;append16,862
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;filter19,1145
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;update22,1326
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;inject25,1564
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;ninject28,1780
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1950
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes32,2079
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce34,2272
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan36,2396
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten38,2559
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect40,2710
    impl<T: MtT> ArraySeqMtPerChap18Trait<T> for ArrayMtPerS<T> {ArrayMtPerS43,2833
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate44,2899
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map49,3079
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append53,3273
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter66,3678
        fn update(a: &ArrayMtPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayMtPerS<T> {update76,4035
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {inject80,4196
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject93,4845
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,5280
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes112,5513
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce123,5952
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan134,6287
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten144,6671
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect155,7073

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtPer.rs,2802
pub mod ArraySeqMtPer{ArraySeqMtPer3,82
    pub trait ArraySeqMtPerTrait<T: MtT> {ArraySeqMtPerTrait7,170
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;tabulate10,395
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;map13,632
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;append16,827
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;filter19,1110
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;update22,1291
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;inject25,1529
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;ninject28,1745
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1915
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes32,2044
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce34,2237
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan36,2361
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten38,2524
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect40,2675
    impl<T: MtT> ArraySeqMtPerTrait<T> for ArrayMtPerS<T> {ArrayMtPerS43,2798
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate44,2858
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map49,3038
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append53,3226
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter66,3631
        fn update(a: &ArrayMtPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayMtPerS<T> {update76,3988
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {inject80,4149
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject93,4798
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,5233
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes112,5466
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce123,5905
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan134,6240
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten144,6624
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect155,7026

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqMtEph.rs,1632
pub mod ArraySeqMtEph{ArraySeqMtEph3,67
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait7,155
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate8,198
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map9,266
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append10,355
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter11,441
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;update12,528
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject13,617
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject14,718
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS17,827
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate18,890
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map25,1136
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append38,1650
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter55,2321
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update67,2746
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject71,2906
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject79,3231

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/LinkedListStEph.rs,2883
pub mod LinkedListStEph {LinkedListStEph3,60
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait9,184
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;tabulate12,321
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map15,487
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append18,688
        fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T>;filter21,876
        fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T>;update24,1075
        fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListinject27,1292
        fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedLisninject30,1519
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate31,1627
        fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes32,1716
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce33,1836
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan34,1918
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten35,2021
        fn collect<A: StT, Bv: StT>(collect36,2108
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS42,2300
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate43,2367
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map50,2617
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append57,2966
        fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter69,3559
        fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T> update79,3980
        fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListinject82,4169
        fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedLisninject100,5045
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate114,5755
        fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes121,6043
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce131,6518
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan146,7244
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten165,8044
        fn collect<A: StT, Bv: StT>(collect177,8736

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap18/ArraySeqStPer.rs,2998
pub mod ArraySeqStPer{ArraySeqStPer3,46
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait7,134
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,359
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map14,597
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append18,801
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1085
        fn update(a: &ArrayStPerS<T>, item_at: Pair<N, T>) -> ArrayStPerS<T>;update26,1267
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject30,1506
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject34,1723
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate37,1894
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes40,2024
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce43,2226
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan46,2351
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten49,2515
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect52,2667
    impl<T: StT> ArraySeqStPerTrait<T> for ArrayStPerS<T> {ArrayStPerS55,2790
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate56,2850
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map60,3029
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append72,3530
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter86,4062
        fn update(a: &ArrayStPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayStPerS<T> {update95,4397
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject101,4630
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject113,5194
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate123,5636
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes130,5868
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce139,6271
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec140,6349
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan155,6865
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec156,6959
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten177,7735
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect187,8103

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/LinkedListStPer.rs,2686
pub mod LinkedListStPer19 {LinkedListStPer193,48
    pub trait LinkedListStPerTrait<T: StT> {LinkedListStPerTrait8,221
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate9,266
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map10,336
        fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T>select11,429
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append12,527
        fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append213,619
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T>;deflate14,712
        fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter15,783
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,871
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,960
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan18,1042
        fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten19,1145
        fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>) -> Linkeinject20,1231
    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {LinkedListStPerS23,1350
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate24,1417
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map27,1575
        fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T>select30,1751
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append44,2399
        fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append248,2578
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T> {deflate52,2758
        fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter60,3070
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate64,3245
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce68,3425
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan72,3598
        fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten76,3790
        fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>) -> Linkeinject80,3961

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEphSlice.rs,3453
pub mod ArraySeqMtEphSlice {ArraySeqMtEphSlice8,395
    struct Inner<T: StT> {Inner15,563
        data: Mutex<Box<[T]>>,data16,590
    impl<T: StT> Inner<T> {Inner19,628
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }new20,656
        fn len(&self) -> N {len22,733
    pub struct ArraySeqMtEphSliceS<T: StT> {ArraySeqMtEphSliceS29,921
        inner: Arc<Inner<T>>,inner30,966
        range: Range<N>,range31,996
    pub trait ArraySeqMtEphSliceTrait<T: StT> {ArraySeqMtEphSliceTrait35,1092
        fn new(length: N, init_value: T) -> Self;new36,1140
        fn length(&self) -> N;length37,1190
        fn nth_cloned(&self, index: N) -> T;nth_cloned38,1221
        fn empty() -> Self;empty39,1266
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set40,1294
        fn singleton(item: T) -> Self;singleton41,1375
        fn isEmpty(&self) -> B;isEmpty42,1414
        fn isSingleton(&self) -> B;isSingleton43,1446
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy44,1482
        fn slice(&self, start: N, length: N) -> Self;slice45,1542
    impl<T: StT> ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS48,1603
        pub fn from_box(data: Box<[T]>) -> Self {from_box50,1706
        pub fn from_vec(data: Vec<T>) -> Self { Self::from_box(data.into_boxed_slice()) }from_vec59,2008
        pub fn to_vec(&self) -> Vec<T> {to_vec62,2180
        pub fn with_exclusive<F, R>(&self, f: F) -> Rwith_exclusive68,2444
        fn len(&self) -> N { self.range.end - self.range.start }len78,2750
        fn clamp_subrange(&self, start: N, length: N) -> Range<N> {clamp_subrange80,2816
    impl<T: StT> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS89,3176
        fn new(length: N, init_value: T) -> Self {new90,3249
        fn length(&self) -> N { self.len() }length95,3414
        fn nth_cloned(&self, index: N) -> T {nth_cloned97,3460
        fn empty() -> Self { ArraySeqMtEphSliceS::from_vec(Vec::new()) }empty103,3653
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set105,3727
        fn singleton(item: T) -> Self { ArraySeqMtEphSliceS::from_vec(vec![item]) }singleton117,4123
        fn isEmpty(&self) -> B {isEmpty119,4208
        fn isSingleton(&self) -> B {isSingleton127,4369
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy135,4534
        fn slice(&self, start: N, length: N) -> Self {slice142,4853
    impl<T: StT> Clone for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS151,5107
        fn clone(&self) -> Self {clone152,5159
    impl<T: StT> PartialEq for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS160,5349
        fn eq(&self, other: &Self) -> bool {eq161,5405
    impl<T: StT> Eq for ArraySeqMtEphSliceS<T> {}ArraySeqMtEphSliceS174,5823
    impl<T: StT> Debug for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS176,5874
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt177,5926
    impl<T: StT> Display for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS185,6193
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {fmt186,6247
    fn repeat_vec<T: StT>(length: N, init: T) -> Vec<T> {repeat_vec201,6714
    macro_rules! ArraySeqMtEphSliceSLit {ArraySeqMtEphSliceSLit210,6939
    fn _ArraySeqMtEphSliceSLit_type_checks() {_ArraySeqMtEphSliceSLit_type_checks217,7402

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStEph.rs,2377
pub mod ArraySeqStEph {ArraySeqStEph3,51
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait8,192
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate11,327
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map14,491
        fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T>;select17,672
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append20,874
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append223,1068
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T>;deflate26,1247
        fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter29,1412
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1496
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce31,1583
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan32,1663
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten33,1762
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS36,1849
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate37,1912
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map40,2064
        fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T> {select43,2232
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append55,2636
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append258,2804
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T> {deflate61,2973
        fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter68,3274
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate71,3440
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce74,3613
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan77,3779
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten80,3962

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtPer.rs,3749
pub mod ArraySeqMtPer {ArraySeqMtPer3,129
    pub trait ArraySeqMtPerTrait<T: MtT> {ArraySeqMtPerTrait11,333
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;tabulate15,589
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;map18,826
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;append21,1021
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;filter24,1304
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;update27,1485
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;ninject30,1687
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate32,1857
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes34,1986
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce36,2179
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan38,2303
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten40,2466
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect42,2617
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;inject45,2775
        fn atomicWrite(atomicWrite47,2913
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arrayinject_parallel253,3156
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins54,3263
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arraninject_parallel260,3531
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins61,3639
    impl<T: MtT + StT> ArraySeqMtPerTrait<T> for ArrayMtPerS<T> {ArrayMtPerS68,3852
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate69,3918
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map73,4065
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append77,4225
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter81,4382
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T> {update85,4546
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject89,4710
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate93,4890
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes97,5058
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce101,5260
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan105,5421
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten109,5596
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect120,5998
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> inject154,7335
        fn atomicWrite(atomicWrite158,7523
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arrayinject_parallel2166,7816
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins171,8090
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arraninject_parallel2192,8860
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins197,9137

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqMtEph.rs,2378
pub mod ArraySeqMtEph {ArraySeqMtEph3,67
    pub trait ArraySeqMtEphTrait<T: StT> {ArraySeqMtEphTrait8,208
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;tabulate9,251
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;map10,319
        fn select<'a>(a: &'a ArraySeqMtEphS<T>, b: &'a ArraySeqMtEphS<T>, i: N) -> Option<T>;select11,408
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append12,502
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;append213,588
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T>;deflate14,675
        fn filter(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;filter15,744
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,828
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,915
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtEphS<T>, Tscan18,995
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;flatten19,1094
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS22,1181
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {tabulate23,1244
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map26,1396
        fn select<'a>(a: &'a ArraySeqMtEphS<T>, b: &'a ArraySeqMtEphS<T>, i: N) -> Option<T> {select29,1564
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append41,1966
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append244,2134
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T> {deflate47,2303
        fn filter(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter54,2562
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, mut x: A) -> A {iterate57,2728
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> T {reduce63,2940
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> (ArraySeqMtEphS<Tscan69,3148
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {flatten82,3650

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/LinkedListStEph.rs,2686
pub mod LinkedListStEph19 {LinkedListStEph193,60
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait8,233
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;tabulate9,278
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map10,348
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>select11,441
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append12,539
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append213,631
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T>;deflate14,724
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T>;filter15,795
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate16,883
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce17,972
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan18,1054
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten19,1157
        fn inject(values: &LinkedListStEphS<T>, changes: &LinkedListStEphS<Pair<N, T>>) -> Linkeinject20,1243
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS23,1362
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate24,1429
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map27,1587
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>select30,1763
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append44,2411
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append247,2589
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T> {deflate50,2768
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter57,3079
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate60,3253
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce63,3432
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan66,3604
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten69,3795
        fn inject(values: &LinkedListStEphS<T>, changes: &LinkedListStEphS<Pair<N, T>>) -> Linkeinject72,3965

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap19/ArraySeqStPer.rs,2694
pub mod ArraySeqStPer {ArraySeqStPer3,46
    pub trait ArraySeqStPerTrait<T: StT> {ArraySeqStPerTrait8,187
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,316
        fn map<U: StT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map12,461
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T>;select14,585
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append16,730
        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append218,860
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T>;deflate20,979
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1132
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate26,1390
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce28,1521
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan30,1645
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten32,1802
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject34,1932
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject36,2083
    impl<T: StT> ArraySeqStPerTrait<T> for ArrayStPerS<T> {ArrayStPerS39,2183
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate40,2243
        fn map<U: StT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map44,2390
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T> {select47,2577
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append60,2958
        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append267,3227
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T> {deflate74,3497
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter78,3719
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate87,4130
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce99,4621
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan114,5278
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten157,6998
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject161,7147
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject165,7325

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetAVLMtEph.rs,5580
pub mod BSTSetAVLMtEph {BSTSetAVLMtEph3,73
    pub struct BSTSetAVLMtEph<T: StTInMtT + Ord> {BSTSetAVLMtEph10,286
        tree: BSTAVLMtEph<T>,tree11,337
    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;BSTSetAVLMt14,374
    pub trait BSTSetAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetAVLMtEphTrait16,424
        fn empty() -> Self;empty17,486
        fn singleton(value: T) -> Self;singleton18,514
        fn size(&self) -> N;size19,554
        fn is_empty(&self) -> B;is_empty20,583
        fn find(&self, value: &T) -> Option<T>;find21,616
        fn contains(&self, value: &T) -> B;contains22,664
        fn minimum(&self) -> Option<T>;minimum23,708
        fn maximum(&self) -> Option<T>;maximum24,748
        fn insert(&mut self, value: T);insert25,788
        fn delete(&mut self, target: &T);delete26,828
        fn union(&self, other: &Self) -> Self;union27,870
        fn intersection(&self, other: &Self) -> Self;intersection28,917
        fn difference(&self, other: &Self) -> Self;difference29,971
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1023
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1078
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1133
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1195
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1265
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1333
        fn as_tree(&self) -> &BSTAVLMtEph<T>;as_tree36,1384
    impl<T: StTInMtT + Ord> BSTSetAVLMtEph<T> {BSTSetAVLMtEph39,1437
        pub fn empty() -> Self {empty40,1485
        pub fn singleton(value: T) -> Self {singleton46,1604
        pub fn size(&self) -> N { self.tree.size() }size52,1761
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1815
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1877
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1955
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2033
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2101
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2169
        pub fn delete(&mut self, target: &T) {delete66,2242
        pub fn union(&self, other: &Self) -> Self {union74,2531
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2830
        pub fn difference(&self, other: &Self) -> Self {difference99,3408
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,3985
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4677
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,4990
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5346
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5755
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6019
        pub fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree178,6099
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6164
        fn rebuild_from_vec(values: Vec<T>) -> BSTAVLMtEph<T> {rebuild_from_vec182,6255
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6474
    impl<T: StTInMtT + Ord> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {BSTSetAVLMtEph202,6757
        fn empty() -> Self { Self::empty() }empty203,6832
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6878
        fn size(&self) -> N { self.tree.size() }size207,6945
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,6995
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7053
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7127
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7201
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7265
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7329
        fn delete(&mut self, target: &T) { BSTSetAVLMtEph::delete(self, target) }delete221,7398
        fn union(&self, other: &Self) -> Self { BSTSetAVLMtEph::union(self, other) }union223,7481
        fn intersection(&self, other: &Self) -> Self { BSTSetAVLMtEph::intersection(self, other)intersection225,7567
        fn difference(&self, other: &Self) -> Self { BSTSetAVLMtEph::difference(self, other) }difference227,7667
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetAVLMtEph::split(self, pivot) }split229,7763
        fn join_pair(left: Self, right: Self) -> Self { BSTSetAVLMtEph::join_pair(left, right) }join_pair231,7857
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetAVLMtEph::join_m(left, pivojoin_m233,7955
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetAVLMtEph::filter(sefilter235,8064
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetAVLMtEph::reduce(selfreduce237,8178
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8289
        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree241,8365
    macro_rules! BSTSetAVLMtEphLit {BSTSetAVLMtEphLit245,8452
    fn _BSTSetAVLMtEphLit_type_checks() {_BSTSetAVLMtEphLit_type_checks257,8999

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStPer.rs,1394
pub mod AVLTreeSeqStPer {AVLTreeSeqStPer3,49
    pub trait AVLTreeSeqStPerTrait<T: StT> {AVLTreeSeqStPerTrait10,235
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T>;tabulate11,280
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U>;map12,350
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T>;select13,443
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T>;append14,531
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T>;deflate15,623
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T>;filter16,694
    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS19,789
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T> {tabulate20,856
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U> {map23,1014
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T> {select26,1190
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T> {append40,1849
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T> {deflate43,2027
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T> {filter50,2338

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetPlainMtEph.rs,5266
pub mod BSTSetPlainMtEph {BSTSetPlainMtEph3,75
    pub struct BSTSetPlainMtEph<T: StTInMtT + Ord> {BSTSetPlainMtEph10,296
        tree: BSTPlainMtEph<T>,tree11,349
    pub type BSTSetPlainMt<T> = BSTSetPlainMtEph<T>;BSTSetPlainMt14,388
    pub trait BSTSetPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetPlainMtEphTrait16,442
        fn empty() -> Self;empty17,506
        fn singleton(value: T) -> Self;singleton18,534
        fn size(&self) -> N;size19,574
        fn is_empty(&self) -> B;is_empty20,603
        fn find(&self, value: &T) -> Option<T>;find21,636
        fn contains(&self, value: &T) -> B;contains22,684
        fn minimum(&self) -> Option<T>;minimum23,728
        fn maximum(&self) -> Option<T>;maximum24,768
        fn insert(&mut self, value: T);insert25,808
        fn delete(&mut self, target: &T);delete26,848
        fn union(&self, other: &Self) -> Self;union27,890
        fn intersection(&self, other: &Self) -> Self;intersection28,937
        fn difference(&self, other: &Self) -> Self;difference29,991
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1043
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1098
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1153
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1215
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1285
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1353
        fn as_tree(&self) -> &BSTPlainMtEph<T>;as_tree36,1404
    impl<T: StTInMtT + Ord> BSTSetPlainMtEph<T> {BSTSetPlainMtEph39,1459
        pub fn empty() -> Self {empty40,1509
        pub fn singleton(value: T) -> Self {singleton46,1630
        pub fn size(&self) -> N { self.tree.size() }size52,1789
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1843
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1905
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1983
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2061
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2129
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2197
        pub fn delete(&mut self, target: &T) {delete66,2270
        pub fn union(&self, other: &Self) -> Self {union74,2559
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2858
        pub fn difference(&self, other: &Self) -> Self {difference99,3436
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4013
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4705
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5018
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5374
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5783
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6047
        pub fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree178,6127
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6194
        fn rebuild_from_vec(values: Vec<T>) -> BSTPlainMtEph<T> {rebuild_from_vec182,6285
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6508
    impl<T: StTInMtT + Ord> BSTSetPlainMtEphTrait<T> for BSTSetPlainMtEph<T> {BSTSetPlainMtEph202,6793
        fn empty() -> Self { Self::empty() }empty203,6872
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6918
        fn size(&self) -> N { self.tree.size() }size207,6985
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7035
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7093
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7167
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7241
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7305
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7369
        fn delete(&mut self, target: &T) {delete221,7438
        fn union(&self, other: &Self) -> Self {union229,7723
        fn intersection(&self, other: &Self) -> Self {intersection237,8018
        fn difference(&self, other: &Self) -> Self {difference254,8592
        fn split(&self, pivot: &T) -> (Self, B, Self) {split271,9165
        fn join_pair(left: Self, right: Self) -> Self {join_pair291,9853
        fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m299,10162
        fn filter<F>(&self, predicate: F) -> Selffilter308,10514
        fn reduce<F>(&self, op: F, base: T) -> Treduce315,10675
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order322,10833
        fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree324,10909
    macro_rules! BSTSetPlainMtEphLit {BSTSetPlainMtEphLit328,10998
    fn _BSTSetPlainMtEphLit_type_checks() {_BSTSetPlainMtEphLit_type_checks340,11571

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBStEph.rs,4614
pub mod BSTRBStEph {BSTRBStEph3,93
    enum Color {Color9,301
        Red,Red10,318
        Black,Black11,331
    type Link<T> = Option<Box<Node<T>>>;Link14,353
    struct Node<T: StT + Ord> {Node17,416
        key: T,key18,448
        color: Color,color19,464
        size: N,size20,486
        left: Link<T>,left21,503
        right: Link<T>,right22,526
    impl<T: StT + Ord> Node<T> {Node25,557
        fn new(key: T) -> Self {new26,590
    pub struct BSTRBStEph<T: StT + Ord> {BSTRBStEph37,811
        root: Link<T>,root38,853
    pub type BSTreeRB<T> = BSTRBStEph<T>;BSTreeRB41,883
    pub trait BSTRBStEphTrait<T: StT + Ord> {BSTRBStEphTrait43,926
        fn new() -> Self;new44,972
        fn size(&self) -> N;size45,998
        fn is_empty(&self) -> B;is_empty46,1027
        fn height(&self) -> N;height47,1060
        fn insert(&mut self, value: T);insert48,1091
        fn find(&self, target: &T) -> Option<&T>;find49,1131
        fn contains(&self, target: &T) -> B;contains50,1181
        fn minimum(&self) -> Option<&T>;minimum51,1226
        fn maximum(&self) -> Option<&T>;maximum52,1267
        fn in_order(&self) -> ArrayStPerS<T>;in_order53,1308
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order54,1354
    impl<T: StT + Ord> Default for BSTRBStEph<T> {BSTRBStEph57,1408
        fn default() -> Self { Self::new() }default58,1459
    impl<T: StT + Ord> BSTRBStEph<T> {BSTRBStEph61,1511
        pub fn new() -> Self { BSTRBStEph { root: None } }new62,1550
        pub fn size(&self) -> N { Self::size_link(&self.root) }size64,1610
        pub fn is_empty(&self) -> B {is_empty66,1675
        pub fn height(&self) -> N {height74,1842
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec75,1878
        pub fn insert(&mut self, value: T) {insert84,2174
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find91,2394
        pub fn contains(&self, target: &T) -> B {contains93,2488
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum101,2678
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum103,2754
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order105,2830
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order111,3048
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red117,3268
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link119,3370
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate121,3453
        fn rotate_left(link: &mut Link<T>) {rotate_left123,3572
        fn rotate_right(link: &mut Link<T>) {rotate_right140,4168
        fn flip_colors(link: &mut Link<T>) {flip_colors157,4766
        fn fix_up(link: &mut Link<T>) {fix_up178,5571
        fn insert_link(link: &mut Link<T>, value: T) {insert_link195,6258
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link211,6800
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link226,7317
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link236,7638
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect246,7961
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect254,8248
    impl<T: StT + Ord> BSTRBStEphTrait<T> for BSTRBStEph<T> {BSTRBStEph263,8544
        fn new() -> Self { BSTRBStEph::new() }new264,8606
        fn size(&self) -> N { BSTRBStEph::size(self) }size266,8654
        fn is_empty(&self) -> B { BSTRBStEph::is_empty(self) }is_empty268,8710
        fn height(&self) -> N { BSTRBStEph::height(self) }height270,8774
        fn insert(&mut self, value: T) { BSTRBStEph::insert(self, value) }insert272,8834
        fn find(&self, target: &T) -> Option<&T> { BSTRBStEph::find(self, target) }find274,8910
        fn contains(&self, target: &T) -> B { BSTRBStEph::contains(self, target) }contains276,8995
        fn minimum(&self) -> Option<&T> { BSTRBStEph::minimum(self) }minimum278,9079
        fn maximum(&self) -> Option<&T> { BSTRBStEph::maximum(self) }maximum280,9150
        fn in_order(&self) -> ArrayStPerS<T> { BSTRBStEph::in_order(self) }in_order282,9221
        fn pre_order(&self) -> ArrayStPerS<T> { BSTRBStEph::pre_order(self) }pre_order284,9298
    macro_rules! BSTRBStEphLit {BSTRBStEphLit288,9403
    fn _BSTRBStEphLit_type_checks() {_BSTRBStEphLit_type_checks300,9897

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetSplayMtEph.rs,5634
pub mod BSTSetSplayMtEph {BSTSetSplayMtEph3,75
    pub struct BSTSetSplayMtEph<T: StTInMtT + Ord> {BSTSetSplayMtEph10,296
        tree: BSTSplayMtEph<T>,tree11,349
    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;BSTSetSplayMt14,388
    pub trait BSTSetSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetSplayMtEphTrait16,442
        fn empty() -> Self;empty17,506
        fn singleton(value: T) -> Self;singleton18,534
        fn size(&self) -> N;size19,574
        fn is_empty(&self) -> B;is_empty20,603
        fn find(&self, value: &T) -> Option<T>;find21,636
        fn contains(&self, value: &T) -> B;contains22,684
        fn minimum(&self) -> Option<T>;minimum23,728
        fn maximum(&self) -> Option<T>;maximum24,768
        fn insert(&mut self, value: T);insert25,808
        fn delete(&mut self, target: &T);delete26,848
        fn union(&self, other: &Self) -> Self;union27,890
        fn intersection(&self, other: &Self) -> Self;intersection28,937
        fn difference(&self, other: &Self) -> Self;difference29,991
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1043
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1098
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1153
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1215
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1285
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1353
        fn as_tree(&self) -> &BSTSplayMtEph<T>;as_tree36,1404
    impl<T: StTInMtT + Ord> BSTSetSplayMtEph<T> {BSTSetSplayMtEph39,1459
        pub fn empty() -> Self {empty40,1509
        pub fn singleton(value: T) -> Self {singleton46,1630
        pub fn size(&self) -> N { self.tree.size() }size52,1789
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1843
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1905
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1983
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2061
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2129
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2197
        pub fn delete(&mut self, target: &T) {delete66,2270
        pub fn union(&self, other: &Self) -> Self {union74,2559
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2858
        pub fn difference(&self, other: &Self) -> Self {difference99,3436
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4013
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4705
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5018
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5374
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5783
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6047
        pub fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree178,6127
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6194
        fn rebuild_from_vec(values: Vec<T>) -> BSTSplayMtEph<T> {rebuild_from_vec182,6285
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6508
    impl<T: StTInMtT + Ord> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {BSTSetSplayMtEph202,6793
        fn empty() -> Self { Self::empty() }empty203,6872
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6918
        fn size(&self) -> N { self.tree.size() }size207,6985
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7035
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7093
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7167
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7241
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7305
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7369
        fn delete(&mut self, target: &T) { BSTSetSplayMtEph::delete(self, target) }delete221,7438
        fn union(&self, other: &Self) -> Self { BSTSetSplayMtEph::union(self, other) }union223,7523
        fn intersection(&self, other: &Self) -> Self { BSTSetSplayMtEph::intersection(self, otheintersection225,7611
        fn difference(&self, other: &Self) -> Self { BSTSetSplayMtEph::difference(self, other) }difference227,7713
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetSplayMtEph::split(self, pivot) }split229,7811
        fn join_pair(left: Self, right: Self) -> Self { BSTSetSplayMtEph::join_pair(left, right)join_pair231,7907
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetSplayMtEph::join_m(left, pijoin_m233,8007
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetSplayMtEph::filter(filter235,8118
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetSplayMtEph::reduce(sereduce237,8234
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8347
        fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree241,8423
    macro_rules! BSTSetSplayMtEphLit {BSTSetSplayMtEphLit245,8512
    fn _BSTSetSplayMtEphLit_type_checks() {_BSTSetSplayMtEphLit_type_checks257,9085

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaMtEph.rs,4514
pub mod BSTBBAlphaMtEph {BSTBBAlphaMtEph3,108
    type Link<T> = Option<Box<Node<T>>>;Link12,344
    struct Node<T: StTInMtT + Ord> {Node15,407
        key: T,key16,444
        size: N,size17,460
        left: Link<T>,left18,477
        right: Link<T>,right19,500
    impl<T: StTInMtT + Ord> Node<T> {Node22,531
        fn new(key: T) -> Self {new23,569
    pub struct BSTBBAlphaMtEph<T: StTInMtT + Ord> {BSTBBAlphaMtEph34,776
        root: Arc<RwLock<Link<T>>>,root35,828
    pub type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;BSTreeBBAlpha38,871
    pub trait BSTBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTBBAlphaMtEphTrait40,924
        fn new() -> Self;new41,987
        fn insert(&self, value: T);insert42,1013
        fn find(&self, target: &T) -> Option<T>;find43,1049
        fn contains(&self, target: &T) -> B;contains44,1098
        fn size(&self) -> N;size45,1143
        fn is_empty(&self) -> B;is_empty46,1172
        fn height(&self) -> N;height47,1205
        fn minimum(&self) -> Option<T>;minimum48,1236
        fn maximum(&self) -> Option<T>;maximum49,1276
        fn in_order(&self) -> ArrayStPerS<T>;in_order50,1316
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order51,1362
    impl<T: StTInMtT + Ord> Default for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph54,1416
        fn default() -> Self { Self::new() }default55,1477
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph58,1529
        pub fn new() -> Self {new59,1578
        pub fn size(&self) -> N {size65,1715
        pub fn is_empty(&self) -> B {is_empty70,1848
        pub fn height(&self) -> N {height78,2015
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec79,2051
        pub fn insert(&self, value: T) {insert90,2401
        pub fn find(&self, target: &T) -> Option<T> {find99,2732
        pub fn contains(&self, target: &T) -> B {contains104,2902
        pub fn minimum(&self) -> Option<T> {minimum112,3092
        pub fn maximum(&self) -> Option<T> {maximum117,3244
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order122,3396
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order129,3675
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link136,3956
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate138,4039
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link140,4158
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild162,4904
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed169,5186
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values179,5596
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced187,5877
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link199,6317
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link214,6834
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link224,7155
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect234,7478
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect242,7765
    impl<T: StTInMtT + Ord> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph251,8061
        fn new() -> Self { BSTBBAlphaMtEph::new() }new252,8138
        fn insert(&self, value: T) { BSTBBAlphaMtEph::insert(self, value) }insert254,8191
        fn find(&self, target: &T) -> Option<T> { BSTBBAlphaMtEph::find(self, target) }find256,8268
        fn contains(&self, target: &T) -> B { BSTBBAlphaMtEph::contains(self, target) }contains258,8357
        fn size(&self) -> N { BSTBBAlphaMtEph::size(self) }size260,8446
        fn is_empty(&self) -> B { BSTBBAlphaMtEph::is_empty(self) }is_empty262,8507
        fn height(&self) -> N { BSTBBAlphaMtEph::height(self) }height264,8576
        fn minimum(&self) -> Option<T> { BSTBBAlphaMtEph::minimum(self) }minimum266,8641
        fn maximum(&self) -> Option<T> { BSTBBAlphaMtEph::maximum(self) }maximum268,8716
        fn in_order(&self) -> ArrayStPerS<T> { BSTBBAlphaMtEph::in_order(self) }in_order270,8791
        fn pre_order(&self) -> ArrayStPerS<T> { BSTBBAlphaMtEph::pre_order(self) }pre_order272,8873
    macro_rules! BSTBBAlphaMtEphLit {BSTBBAlphaMtEphLit276,8983
    fn _BSTBBAlphaMtEphLit_type_checks() {_BSTBBAlphaMtEphLit_type_checks288,9538

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLStEph.rs,4434
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
    pub struct BSTAVLStEph<T: StT + Ord> {BSTAVLStEph31,713
        root: Link<T>,root32,756
    pub type BSTreeAVL<T> = BSTAVLStEph<T>;BSTreeAVL35,786
    pub trait BSTAVLStEphTrait<T: StT + Ord> {BSTAVLStEphTrait37,831
        fn new() -> Self;new38,878
        fn size(&self) -> N;size39,904
        fn is_empty(&self) -> B;is_empty40,933
        fn height(&self) -> N;height41,966
        fn insert(&mut self, value: T);insert42,997
        fn find(&self, target: &T) -> Option<&T>;find43,1037
        fn contains(&self, target: &T) -> B;contains44,1087
        fn minimum(&self) -> Option<&T>;minimum45,1132
        fn maximum(&self) -> Option<&T>;maximum46,1173
        fn in_order(&self) -> ArrayStPerS<T>;in_order47,1214
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order48,1260
    impl<T: StT + Ord> Default for BSTAVLStEph<T> {BSTAVLStEph51,1314
        fn default() -> Self { Self::new() }default52,1366
    impl<T: StT + Ord> BSTAVLStEph<T> {BSTAVLStEph55,1418
        pub fn new() -> Self { BSTAVLStEph { root: None } }new56,1458
        pub fn size(&self) -> N { Self::size_link(&self.root) }size58,1519
        pub fn is_empty(&self) -> B {is_empty60,1584
        pub fn height(&self) -> N { Self::height_link(&self.root) as N }height68,1751
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert70,1825
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find72,1915
        pub fn contains(&self, target: &T) -> B {contains74,2009
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum82,2199
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum84,2275
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order86,2351
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order92,2569
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link98,2789
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link100,2878
        fn update(node: &mut Node<T>) {update102,2961
        fn rotate_right(link: &mut Link<T>) {rotate_right107,3197
        fn rotate_left(link: &mut Link<T>) {rotate_left121,3653
        fn rebalance(link: &mut Link<T>) {rebalance135,4108
        fn insert_link(link: &mut Link<T>, value: T) {insert_link160,5154
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link179,5789
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link194,6306
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link204,6627
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect214,6950
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect222,7237
    impl<T: StT + Ord> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {BSTAVLStEph231,7533
        fn new() -> Self { BSTAVLStEph::new() }new232,7597
        fn size(&self) -> N { BSTAVLStEph::size(self) }size234,7646
        fn is_empty(&self) -> B { BSTAVLStEph::is_empty(self) }is_empty236,7703
        fn height(&self) -> N { BSTAVLStEph::height(self) }height238,7768
        fn insert(&mut self, value: T) { BSTAVLStEph::insert(self, value) }insert240,7829
        fn find(&self, target: &T) -> Option<&T> { BSTAVLStEph::find(self, target) }find242,7906
        fn contains(&self, target: &T) -> B { BSTAVLStEph::contains(self, target) }contains244,7992
        fn minimum(&self) -> Option<&T> { BSTAVLStEph::minimum(self) }minimum246,8077
        fn maximum(&self) -> Option<&T> { BSTAVLStEph::maximum(self) }maximum248,8149
        fn in_order(&self) -> ArrayStPerS<T> { BSTAVLStEph::in_order(self) }in_order250,8221
        fn pre_order(&self) -> ArrayStPerS<T> { BSTAVLStEph::pre_order(self) }pre_order252,8299
    macro_rules! BSTAVLStEphLit {BSTAVLStEphLit256,8405
    fn _BSTAVLStEphLit_type_checks() {_BSTAVLStEphLit_type_checks268,8912

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainMtEph.rs,3702
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
    pub struct BSTPlainMtEph<T: StTInMtT + Ord> {BSTPlainMtEph39,1028
        root: Link<T>,root40,1078
    pub type BSTree<T> = BSTPlainMtEph<T>;BSTree43,1108
    pub trait BSTPlainMtEphTrait<T: StTInMtT + Ord>: Sized {BSTPlainMtEphTrait45,1152
        fn new() -> Self;new46,1213
        fn insert(&self, value: T);insert47,1239
        fn find(&self, target: &T) -> Option<T>;find48,1275
        fn contains(&self, target: &T) -> B;contains49,1324
        fn size(&self) -> N;size50,1369
        fn is_empty(&self) -> B;is_empty51,1398
        fn height(&self) -> N;height52,1431
        fn minimum(&self) -> Option<T>;minimum53,1462
        fn maximum(&self) -> Option<T>;maximum54,1502
        fn in_order(&self) -> ArrayStPerS<T>;in_order55,1542
    impl<T: StTInMtT + Ord> BSTPlainMtEph<T> {BSTPlainMtEph58,1595
        pub fn new() -> Self {new59,1642
        pub fn insert(&self, value: T) {insert65,1768
            fn descend<T: StTInMtT + Ord>(link: &Link<T>, value: T) -> bool {descend66,1809
        pub fn find(&self, target: &T) -> Option<T> {find100,3001
            fn find_rec<T: StTInMtT + Ord>(link: &Link<T>, target: &T) -> Option<T> {find_rec101,3055
        pub fn contains(&self, target: &T) -> B {contains120,3794
        pub fn size(&self) -> N {size127,3983
        pub fn is_empty(&self) -> B {is_empty132,4134
        pub fn height(&self) -> N {height140,4301
        pub fn minimum(&self) -> Option<T> {minimum145,4461
            fn leftmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {leftmost146,4506
        pub fn maximum(&self) -> Option<T> {maximum167,5216
            fn rightmost<T: StTInMtT + Ord>(link: &Link<T>) -> Option<T> {rightmost168,5261
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order189,5978
            fn traverse<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {traverse190,6029
    fn height_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> i32 { link.as_ref().map_or(0, |n|height_of209,6694
    fn size_of<T: StTInMtT + Ord>(link: &Option<Node<T>>) -> N { link.as_ref().map_or(0, |n| n.ssize_of211,6804
    impl<T: StTInMtT + Ord> BSTPlainMtEphTrait<T> for BSTPlainMtEph<T> {BSTPlainMtEph213,6908
        fn new() -> Self { BSTPlainMtEph::new() }new214,6981
        fn insert(&self, value: T) { BSTPlainMtEph::insert(self, value) }insert215,7031
        fn find(&self, target: &T) -> Option<T> { BSTPlainMtEph::find(self, target) }find216,7105
        fn contains(&self, target: &T) -> B { BSTPlainMtEph::contains(self, target) }contains217,7191
        fn size(&self) -> N { BSTPlainMtEph::size(self) }size218,7277
        fn is_empty(&self) -> B { BSTPlainMtEph::is_empty(self) }is_empty219,7335
        fn height(&self) -> N { BSTPlainMtEph::height(self) }height220,7401
        fn minimum(&self) -> Option<T> { BSTPlainMtEph::minimum(self) }minimum221,7463
        fn maximum(&self) -> Option<T> { BSTPlainMtEph::maximum(self) }maximum222,7535
        fn in_order(&self) -> ArrayStPerS<T> { BSTPlainMtEph::in_order(self) }in_order223,7607
    macro_rules! BSTPlainMtEphLit {BSTPlainMtEphLit227,7713
    fn _BSTPlainMtEphLit_type_checks() {_BSTPlainMtEphLit_type_checks242,8277

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTBBAlphaStEph.rs,4626
pub mod BSTBBAlphaStEph {BSTBBAlphaStEph3,80
    type Link<T> = Option<Box<Node<T>>>;Link10,281
    struct Node<T: StT + Ord> {Node13,344
        key: T,key14,376
        size: N,size15,392
        left: Link<T>,left16,409
        right: Link<T>,right17,432
    impl<T: StT + Ord> Node<T> {Node20,463
        fn new(key: T) -> Self {new21,496
    pub struct BSTBBAlphaStEph<T: StT + Ord> {BSTBBAlphaStEph31,682
        root: Link<T>,root32,729
    pub type BSTreeBBAlpha<T> = BSTBBAlphaStEph<T>;BSTreeBBAlpha35,759
    pub trait BSTBBAlphaStEphTrait<T: StT + Ord> {BSTBBAlphaStEphTrait37,812
        fn new() -> Self;new38,863
        fn size(&self) -> N;size39,889
        fn is_empty(&self) -> B;is_empty40,918
        fn height(&self) -> N;height41,951
        fn insert(&mut self, value: T);insert42,982
        fn find(&self, target: &T) -> Option<&T>;find43,1022
        fn contains(&self, target: &T) -> B;contains44,1072
        fn minimum(&self) -> Option<&T>;minimum45,1117
        fn maximum(&self) -> Option<&T>;maximum46,1158
        fn in_order(&self) -> ArrayStPerS<T>;in_order47,1199
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order48,1245
    impl<T: StT + Ord> Default for BSTBBAlphaStEph<T> {BSTBBAlphaStEph51,1299
        fn default() -> Self { Self::new() }default52,1355
    impl<T: StT + Ord> BSTBBAlphaStEph<T> {BSTBBAlphaStEph55,1407
        pub fn new() -> Self { BSTBBAlphaStEph { root: None } }new56,1451
        pub fn size(&self) -> N { Self::size_link(&self.root) }size58,1516
        pub fn is_empty(&self) -> B {is_empty60,1581
        pub fn height(&self) -> N {height68,1748
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec69,1784
        pub fn insert(&mut self, value: T) {insert78,2080
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find86,2368
        pub fn contains(&self, target: &T) -> B {contains88,2462
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum96,2652
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum98,2728
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order100,2804
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order106,3022
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link112,3242
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate114,3325
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link116,3444
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild138,4190
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed145,4472
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values155,4882
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced163,5163
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link175,5603
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link190,6120
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link200,6441
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect210,6764
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect218,7051
    impl<T: StT + Ord> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {BSTBBAlphaStEph227,7347
        fn new() -> Self { BSTBBAlphaStEph::new() }new228,7419
        fn size(&self) -> N { BSTBBAlphaStEph::size(self) }size230,7472
        fn is_empty(&self) -> B { BSTBBAlphaStEph::is_empty(self) }is_empty232,7533
        fn height(&self) -> N { BSTBBAlphaStEph::height(self) }height234,7602
        fn insert(&mut self, value: T) { BSTBBAlphaStEph::insert(self, value) }insert236,7667
        fn find(&self, target: &T) -> Option<&T> { BSTBBAlphaStEph::find(self, target) }find238,7748
        fn contains(&self, target: &T) -> B { BSTBBAlphaStEph::contains(self, target) }contains240,7838
        fn minimum(&self) -> Option<&T> { BSTBBAlphaStEph::minimum(self) }minimum242,7927
        fn maximum(&self) -> Option<&T> { BSTBBAlphaStEph::maximum(self) }maximum244,8003
        fn in_order(&self) -> ArrayStPerS<T> { BSTBBAlphaStEph::in_order(self) }in_order246,8079
        fn pre_order(&self) -> ArrayStPerS<T> { BSTBBAlphaStEph::pre_order(self) }pre_order248,8161
    macro_rules! BSTBBAlphaStEphLit {BSTBBAlphaStEphLit252,8271
    fn _BSTBBAlphaStEphLit_type_checks() {_BSTBBAlphaStEphLit_type_checks264,8830

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTParaStEph.rs,2986
pub mod BSTParaStEph {BSTParaStEph3,70
    pub enum Exposed<T: StT + Ord> {Exposed11,253
        Leaf,Leaf12,290
        Node(ParamBST<T>, T, ParamBST<T>),Node13,304
    struct NodeInner<T: StT + Ord> {NodeInner17,375
        key: T,key18,412
        size: N,size19,428
        left: ParamBST<T>,left20,445
        right: ParamBST<T>,right21,472
    pub struct ParamBST<T: StT + Ord> {ParamBST25,528
        root: Rc<RefCell<Option<Box<NodeInner<T>>>>>,root26,568
    pub trait ParamBSTTrait<T: StT + Ord>: Sized {ParamBSTTrait29,629
        fn new() -> Self;new30,680
        fn expose(&self) -> Exposed<T>;expose31,706
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid32,746
        fn size(&self) -> N;size33,796
        fn is_empty(&self) -> B;is_empty34,825
        fn insert(&self, key: T);insert35,858
        fn delete(&self, key: &T);delete36,892
        fn find(&self, key: &T) -> Option<T>;find37,927
        fn split(&self, key: &T) -> (Self, B, Self);split38,973
        fn join_pair(&self, other: Self) -> Self;join_pair39,1026
        fn union(&self, other: &Self) -> Self;union40,1076
        fn in_order(&self) -> ArrayStPerS<T>;in_order41,1123
    impl<T: StT + Ord> ParamBST<T> {ParamBST44,1176
        fn expose_internal(&self) -> Exposed<T> {expose_internal45,1213
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid53,1505
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner65,1957
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m84,2942
        fn min_key(tree: &Self) -> Option<T> {min_key86,3058
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner96,3402
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner107,3872
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order119,4379
    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {ParamBST131,4781
        fn new() -> Self {new132,4839
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose138,4965
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid140,5032
        fn size(&self) -> N {size142,5114
        fn is_empty(&self) -> B {is_empty146,5223
        fn insert(&self, key: T) {insert154,5386
        fn delete(&self, key: &T) {delete161,5676
        fn find(&self, key: &T) -> Option<T> {find168,5968
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split179,6465
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair181,6555
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union183,6656
        fn in_order(&self) -> ArrayStPerS<T> {in_order185,6742
    macro_rules! ParamBSTLit {ParamBSTLit193,6980
    fn _ParamBSTLit_type_checks() {_ParamBSTLit_type_checks205,7476

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayMtEph.rs,4115
pub mod BSTSplayMtEph {BSTSplayMtEph3,99
    type Link<T> = Option<Box<Node<T>>>;Link10,303
    struct Node<T: StTInMtT + Ord> {Node13,366
        key: T,key14,403
        size: N,size15,419
        left: Link<T>,left16,436
        right: Link<T>,right17,459
    impl<T: StTInMtT + Ord> Node<T> {Node20,490
        fn new(key: T) -> Self {new21,528
    pub struct BSTSplayMtEph<T: StTInMtT + Ord> {BSTSplayMtEph32,735
        root: Arc<RwLock<Link<T>>>,root33,785
    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;BSTreeSplay36,828
    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSplayMtEphTrait38,877
        fn new() -> Self;new39,938
        fn insert(&self, value: T);insert40,964
        fn find(&self, target: &T) -> Option<T>;find41,1000
        fn contains(&self, target: &T) -> B;contains42,1049
        fn size(&self) -> N;size43,1094
        fn is_empty(&self) -> B;is_empty44,1123
        fn height(&self) -> N;height45,1156
        fn minimum(&self) -> Option<T>;minimum46,1187
        fn maximum(&self) -> Option<T>;maximum47,1227
        fn in_order(&self) -> ArrayStPerS<T>;in_order48,1267
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order49,1313
    impl<T: StTInMtT + Ord> Default for BSTSplayMtEph<T> {BSTSplayMtEph52,1367
        fn default() -> Self { Self::new() }default53,1426
    impl<T: StTInMtT + Ord> BSTSplayMtEph<T> {BSTSplayMtEph56,1478
        pub fn new() -> Self {new57,1525
        pub fn size(&self) -> N {size63,1660
        pub fn is_empty(&self) -> B {is_empty68,1793
        pub fn height(&self) -> N {height76,1960
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec77,1996
        pub fn insert(&self, value: T) {insert88,2346
        pub fn find(&self, target: &T) -> Option<T> {find93,2505
        pub fn contains(&self, target: &T) -> B {contains98,2675
        pub fn minimum(&self) -> Option<T> {minimum106,2865
        pub fn maximum(&self) -> Option<T> {maximum111,3017
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order116,3169
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order123,3448
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link130,3729
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate132,3812
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link134,3931
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link156,4677
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link171,5194
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link181,5515
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect191,5838
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect199,6125
    impl<T: StTInMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {BSTSplayMtEph208,6421
        fn new() -> Self { BSTSplayMtEph::new() }new209,6494
        fn insert(&self, value: T) { BSTSplayMtEph::insert(self, value) }insert211,6545
        fn find(&self, target: &T) -> Option<T> { BSTSplayMtEph::find(self, target) }find213,6620
        fn contains(&self, target: &T) -> B { BSTSplayMtEph::contains(self, target) }contains215,6707
        fn size(&self) -> N { BSTSplayMtEph::size(self) }size217,6794
        fn is_empty(&self) -> B { BSTSplayMtEph::is_empty(self) }is_empty219,6853
        fn height(&self) -> N { BSTSplayMtEph::height(self) }height221,6920
        fn minimum(&self) -> Option<T> { BSTSplayMtEph::minimum(self) }minimum223,6983
        fn maximum(&self) -> Option<T> { BSTSplayMtEph::maximum(self) }maximum225,7056
        fn in_order(&self) -> ArrayStPerS<T> { BSTSplayMtEph::in_order(self) }in_order227,7129
        fn pre_order(&self) -> ArrayStPerS<T> { BSTSplayMtEph::pre_order(self) }pre_order229,7209
    macro_rules! BSTSplayMtEphLit {BSTSplayMtEphLit233,7317
    fn _BSTSplayMtEphLit_type_checks() {_BSTSplayMtEphLit_type_checks245,7846

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetRBMtEph.rs,5552
pub mod BSTSetRBMtEph {BSTSetRBMtEph3,79
    pub struct BSTSetRBMtEph<T: StTInMtT + Ord> {BSTSetRBMtEph10,288
        tree: BSTRBMtEph<T>,tree11,338
    pub type BSTSetRBMt<T> = BSTSetRBMtEph<T>;BSTSetRBMt14,374
    pub trait BSTSetRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetRBMtEphTrait16,422
        fn empty() -> Self;empty17,483
        fn singleton(value: T) -> Self;singleton18,511
        fn size(&self) -> N;size19,551
        fn is_empty(&self) -> B;is_empty20,580
        fn find(&self, value: &T) -> Option<T>;find21,613
        fn contains(&self, value: &T) -> B;contains22,661
        fn minimum(&self) -> Option<T>;minimum23,705
        fn maximum(&self) -> Option<T>;maximum24,745
        fn insert(&mut self, value: T);insert25,785
        fn delete(&mut self, target: &T);delete26,825
        fn union(&self, other: &Self) -> Self;union27,867
        fn intersection(&self, other: &Self) -> Self;intersection28,914
        fn difference(&self, other: &Self) -> Self;difference29,968
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1020
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1075
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1130
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1192
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1262
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1330
        fn as_tree(&self) -> &BSTRBMtEph<T>;as_tree36,1381
    impl<T: StTInMtT + Ord> BSTSetRBMtEph<T> {BSTSetRBMtEph39,1433
        pub fn empty() -> Self {empty40,1480
        pub fn singleton(value: T) -> Self {singleton46,1598
        pub fn size(&self) -> N { self.tree.size() }size52,1754
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1808
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1870
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1948
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2026
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2094
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2162
        pub fn delete(&mut self, target: &T) {delete66,2235
        pub fn union(&self, other: &Self) -> Self {union74,2524
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2823
        pub fn difference(&self, other: &Self) -> Self {difference99,3401
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,3978
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4670
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,4983
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5339
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5748
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6012
        pub fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree178,6092
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6156
        fn rebuild_from_vec(values: Vec<T>) -> BSTRBMtEph<T> {rebuild_from_vec182,6247
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6464
    impl<T: StTInMtT + Ord> BSTSetRBMtEphTrait<T> for BSTSetRBMtEph<T> {BSTSetRBMtEph202,6746
        fn empty() -> Self { Self::empty() }empty203,6819
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6865
        fn size(&self) -> N { self.tree.size() }size207,6932
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,6982
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7040
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7114
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7188
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7252
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7316
        fn delete(&mut self, target: &T) { BSTSetRBMtEph::delete(self, target) }delete221,7385
        fn union(&self, other: &Self) -> Self { BSTSetRBMtEph::union(self, other) }union223,7467
        fn intersection(&self, other: &Self) -> Self { BSTSetRBMtEph::intersection(self, other) intersection225,7552
        fn difference(&self, other: &Self) -> Self { BSTSetRBMtEph::difference(self, other) }difference227,7651
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetRBMtEph::split(self, pivot) }split229,7746
        fn join_pair(left: Self, right: Self) -> Self { BSTSetRBMtEph::join_pair(left, right) }join_pair231,7839
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetRBMtEph::join_m(left, pivotjoin_m233,7936
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetRBMtEph::filter(selfilter235,8044
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetRBMtEph::reduce(self,reduce237,8157
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8267
        fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree241,8343
    macro_rules! BSTSetRBMtEphLit {BSTSetRBMtEphLit245,8429
    fn _BSTSetRBMtEphLit_type_checks() {_BSTSetRBMtEphLit_type_checks257,8963

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/AVLTreeSeqStEph.rs,1394
pub mod AVLTreeSeqStEph {AVLTreeSeqStEph3,54
    pub trait AVLTreeSeqStEphTrait<T: StT> {AVLTreeSeqStEphTrait11,241
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T>;tabulate12,286
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U>;map13,356
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T>;select14,449
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T>;append15,537
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T>;deflate16,629
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T>;filter17,700
    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS20,795
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T> {tabulate21,862
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U> {map24,1020
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T> {select27,1196
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T> {append41,1855
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T> {deflate44,2033
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T> {filter51,2344

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTRBMtEph.rs,4517
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
    pub struct BSTRBMtEph<T: StTInMtT + Ord> {BSTRBMtEph40,886
        root: Arc<RwLock<Link<T>>>,root41,933
    pub type BSTreeRB<T> = BSTRBMtEph<T>;BSTreeRB44,976
    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord>: Sized {BSTRBMtEphTrait46,1019
        fn new() -> Self;new47,1077
        fn insert(&self, value: T);insert48,1103
        fn find(&self, target: &T) -> Option<T>;find49,1139
        fn contains(&self, target: &T) -> B;contains50,1188
        fn size(&self) -> N;size51,1233
        fn is_empty(&self) -> B;is_empty52,1262
        fn height(&self) -> N;height53,1295
        fn minimum(&self) -> Option<T>;minimum54,1326
        fn maximum(&self) -> Option<T>;maximum55,1366
        fn in_order(&self) -> ArrayStPerS<T>;in_order56,1406
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order57,1452
    impl<T: StTInMtT + Ord> Default for BSTRBMtEph<T> {BSTRBMtEph60,1506
        fn default() -> Self { Self::new() }default61,1562
    impl<T: StTInMtT + Ord> BSTRBMtEph<T> {BSTRBMtEph64,1614
        pub fn new() -> Self {new65,1658
        pub fn size(&self) -> N {size71,1790
        pub fn is_empty(&self) -> B {is_empty76,1923
        pub fn height(&self) -> N {height84,2090
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {height_rec85,2126
        pub fn insert(&self, value: T) {insert96,2476
        pub fn find(&self, target: &T) -> Option<T> {find104,2741
        pub fn contains(&self, target: &T) -> B {contains109,2911
        pub fn minimum(&self) -> Option<T> {minimum117,3101
        pub fn maximum(&self) -> Option<T> {maximum122,3253
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order127,3405
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order134,3684
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red141,3965
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link143,4067
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate145,4150
        fn rotate_left(link: &mut Link<T>) {rotate_left147,4269
        fn rotate_right(link: &mut Link<T>) {rotate_right166,4929
        fn flip_colors(link: &mut Link<T>) {flip_colors185,5593
        fn fix_up(link: &mut Link<T>) {fix_up206,6398
        fn insert_link(link: &mut Link<T>, value: T) {insert_link242,7552
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link258,8094
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link273,8611
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link283,8932
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect293,9255
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect301,9542
    impl<T: StTInMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {BSTRBMtEph310,9838
        fn new() -> Self { BSTRBMtEph::new() }new311,9905
        fn insert(&self, value: T) { BSTRBMtEph::insert(self, value) }insert313,9953
        fn find(&self, target: &T) -> Option<T> { BSTRBMtEph::find(self, target) }find315,10025
        fn contains(&self, target: &T) -> B { BSTRBMtEph::contains(self, target) }contains317,10109
        fn size(&self) -> N { BSTRBMtEph::size(self) }size319,10193
        fn is_empty(&self) -> B { BSTRBMtEph::is_empty(self) }is_empty321,10249
        fn height(&self) -> N { BSTRBMtEph::height(self) }height323,10313
        fn minimum(&self) -> Option<T> { BSTRBMtEph::minimum(self) }minimum325,10373
        fn maximum(&self) -> Option<T> { BSTRBMtEph::maximum(self) }maximum327,10443
        fn in_order(&self) -> ArrayStPerS<T> { BSTRBMtEph::in_order(self) }in_order329,10513
        fn pre_order(&self) -> ArrayStPerS<T> { BSTRBMtEph::pre_order(self) }pre_order331,10590
    macro_rules! BSTRBMtEphLit {BSTRBMtEphLit335,10695
    fn _BSTRBMtEphLit_type_checks() {_BSTRBMtEphLit_type_checks347,11185

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTParaMtEph.rs,4111
pub mod BSTParaMtEph {BSTParaMtEph3,69
    pub enum Exposed<T: StTInMtT + Ord> {Exposed10,237
        Leaf,Leaf11,279
        Node(ParamBST<T>, T, ParamBST<T>),Node12,293
    struct NodeInner<T: StTInMtT + Ord> {NodeInner16,364
        key: T,key17,406
        size: N,size18,422
        left: ParamBST<T>,left19,439
        right: ParamBST<T>,right20,466
    pub struct ParamBST<T: StTInMtT + Ord> {ParamBST24,522
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root25,567
    pub trait ParamBSTTrait<T: StTInMtT + Ord + 'static>: Sized {ParamBSTTrait28,628
        fn new() -> Self;new31,785
        fn expose(&self) -> Exposed<T>;expose34,902
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid37,1033
        fn size(&self) -> N;size40,1174
        fn is_empty(&self) -> B;is_empty43,1294
        fn insert(&self, key: T);insert46,1438
        fn delete(&self, key: &T);delete49,1583
        fn find(&self, key: &T) -> Option<T>;find52,1729
        fn split(&self, key: &T) -> (Self, B, Self);split55,1886
        fn join_pair(&self, other: Self) -> Self;join_pair58,2098
        fn union(&self, other: &Self) -> Self;union61,2273
        fn intersect(&self, other: &Self) -> Self;intersect64,2445
        fn difference(&self, other: &Self) -> Self;difference67,2621
        fn filter<F>(&self, predicate: F) -> Selffilter70,2778
        fn reduce<F>(&self, op: F, base: T) -> Treduce75,3002
        fn in_order(&self) -> ArrayStPerS<T>;in_order80,3218
    impl<T: StTInMtT + Ord + 'static> ParamBST<T> {ParamBST83,3271
        fn expose_internal(&self) -> Exposed<T> {expose_internal86,3414
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid96,3804
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner110,4367
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m131,5443
        fn min_key(tree: &Self) -> Option<T> {min_key135,5670
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner147,6185
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner160,6780
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner176,7497
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner196,8450
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner217,9416
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel241,10488
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner251,10850
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel275,11919
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order285,12258
    impl<T: StTInMtT + Ord + 'static> ParamBSTTrait<T> for ParamBST<T> {ParamBST297,12660
        fn new() -> Self {new300,12824
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose308,13041
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid312,13199
        fn size(&self) -> N {size316,13372
        fn is_empty(&self) -> B {is_empty323,13610
        fn insert(&self, key: T) {insert333,13884
        fn delete(&self, key: &T) {delete343,14327
        fn find(&self, key: &T) -> Option<T> {find353,14772
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split366,15380
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair370,15629
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union374,15855
        fn intersect(&self, other: &Self) -> Self { ParamBST::intersect_inner(self, other) }intersect378,16066
        fn difference(&self, other: &Self) -> Self { ParamBST::difference_inner(self, other) }difference382,16285
        fn filter<F>(&self, predicate: F) -> Selffilter386,16486
        fn reduce<F>(&self, op: F, base: T) -> Treduce395,16786
        fn in_order(&self) -> ArrayStPerS<T> {in_order404,17077

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSplayStEph.rs,4268
pub mod BSTSplayStEph {BSTSplayStEph3,84
    type Link<T> = Option<Box<Node<T>>>;Link8,253
    struct Node<T: StT + Ord> {Node11,316
        key: T,key12,348
        size: N,size13,364
        left: Link<T>,left14,381
        right: Link<T>,right15,404
    impl<T: StT + Ord> Node<T> {Node18,435
        fn new(key: T) -> Self {new19,468
    pub struct BSTSplayStEph<T: StT + Ord> {BSTSplayStEph29,654
        root: Link<T>,root30,699
    pub type BSTreeSplay<T> = BSTSplayStEph<T>;BSTreeSplay33,729
    pub trait BSTSplayStEphTrait<T: StT + Ord> {BSTSplayStEphTrait35,778
        fn new() -> Self;new36,827
        fn size(&self) -> N;size37,853
        fn is_empty(&self) -> B;is_empty38,882
        fn height(&self) -> N;height39,915
        fn insert(&mut self, value: T);insert40,946
        fn find(&self, target: &T) -> Option<&T>;find41,986
        fn contains(&self, target: &T) -> B;contains42,1036
        fn minimum(&self) -> Option<&T>;minimum43,1081
        fn maximum(&self) -> Option<&T>;maximum44,1122
        fn in_order(&self) -> ArrayStPerS<T>;in_order45,1163
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order46,1209
    impl<T: StT + Ord> Default for BSTSplayStEph<T> {BSTSplayStEph49,1263
        fn default() -> Self { Self::new() }default50,1317
    impl<T: StT + Ord> BSTSplayStEph<T> {BSTSplayStEph53,1369
        pub fn new() -> Self { BSTSplayStEph { root: None } }new54,1411
        pub fn size(&self) -> N { Self::size_link(&self.root) }size56,1474
        pub fn is_empty(&self) -> B {is_empty58,1539
        pub fn height(&self) -> N {height66,1706
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec67,1742
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert76,2038
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find78,2128
        pub fn contains(&self, target: &T) -> B {contains80,2222
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum88,2412
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum90,2488
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order92,2564
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order98,2782
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link104,3002
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate106,3085
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link108,3204
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link130,3950
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link145,4467
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link155,4788
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect165,5111
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect173,5398
    impl<T: StT + Ord> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {BSTSplayStEph182,5694
        fn new() -> Self { BSTSplayStEph::new() }new183,5762
        fn size(&self) -> N { BSTSplayStEph::size(self) }size185,5813
        fn is_empty(&self) -> B { BSTSplayStEph::is_empty(self) }is_empty187,5872
        fn height(&self) -> N { BSTSplayStEph::height(self) }height189,5939
        fn insert(&mut self, value: T) { BSTSplayStEph::insert(self, value) }insert191,6002
        fn find(&self, target: &T) -> Option<&T> { BSTSplayStEph::find(self, target) }find193,6081
        fn contains(&self, target: &T) -> B { BSTSplayStEph::contains(self, target) }contains195,6169
        fn minimum(&self) -> Option<&T> { BSTSplayStEph::minimum(self) }minimum197,6256
        fn maximum(&self) -> Option<&T> { BSTSplayStEph::maximum(self) }maximum199,6330
        fn in_order(&self) -> ArrayStPerS<T> { BSTSplayStEph::in_order(self) }in_order201,6404
        fn pre_order(&self) -> ArrayStPerS<T> { BSTSplayStEph::pre_order(self) }pre_order203,6484
    macro_rules! BSTSplayStEphLit {BSTSplayStEphLit207,6592
    fn _BSTSplayStEphLit_type_checks() {_BSTSplayStEphLit_type_checks219,7125

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTAVLMtEph.rs,4241
pub mod BSTAVLMtEph {BSTAVLMtEph3,96
    type Link<T> = Option<Box<Node<T>>>;Link10,298
    struct Node<T: StTInMtT + Ord> {Node13,361
        key: T,key14,398
        height: i32,height15,414
        size: N,size16,435
        left: Link<T>,left17,452
        right: Link<T>,right18,475
    impl<T: StTInMtT + Ord> Node<T> {Node21,506
        fn new(key: T) -> Self {new22,544
    pub struct BSTAVLMtEph<T: StTInMtT + Ord> {BSTAVLMtEph34,778
        root: Arc<RwLock<Link<T>>>,root35,826
    pub type BSTreeAVL<T> = BSTAVLMtEph<T>;BSTreeAVL38,869
    pub trait BSTAVLMtEphTrait<T: StTInMtT + Ord>: Sized {BSTAVLMtEphTrait40,914
        fn new() -> Self;new41,973
        fn insert(&self, value: T);insert42,999
        fn find(&self, target: &T) -> Option<T>;find43,1035
        fn contains(&self, target: &T) -> B;contains44,1084
        fn size(&self) -> N;size45,1129
        fn is_empty(&self) -> B;is_empty46,1158
        fn height(&self) -> N;height47,1191
        fn minimum(&self) -> Option<T>;minimum48,1222
        fn maximum(&self) -> Option<T>;maximum49,1262
        fn in_order(&self) -> ArrayStPerS<T>;in_order50,1302
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order51,1348
    impl<T: StTInMtT + Ord> Default for BSTAVLMtEph<T> {BSTAVLMtEph54,1402
        fn default() -> Self { Self::new() }default55,1459
    impl<T: StTInMtT + Ord> BSTAVLMtEph<T> {BSTAVLMtEph58,1511
        pub fn new() -> Self {new59,1556
        pub fn size(&self) -> N {size65,1689
        pub fn is_empty(&self) -> B {is_empty70,1822
        pub fn height(&self) -> N {height78,1989
        pub fn insert(&self, value: T) {insert83,2131
        pub fn find(&self, target: &T) -> Option<T> {find88,2290
        pub fn contains(&self, target: &T) -> B {contains93,2460
        pub fn minimum(&self) -> Option<T> {minimum101,2650
        pub fn maximum(&self) -> Option<T> {maximum106,2802
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order111,2954
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order118,3233
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link125,3514
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link127,3603
        fn update(node: &mut Node<T>) {update129,3686
        fn rotate_right(link: &mut Link<T>) {rotate_right134,3922
        fn rotate_left(link: &mut Link<T>) {rotate_left148,4378
        fn rebalance(link: &mut Link<T>) {rebalance162,4833
        fn insert_link(link: &mut Link<T>, value: T) {insert_link187,5879
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link206,6514
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link221,7031
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link231,7353
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect241,7677
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect249,7964
    impl<T: StTInMtT + Ord> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {BSTAVLMtEph258,8260
        fn new() -> Self { BSTAVLMtEph::new() }new259,8329
        fn insert(&self, value: T) { BSTAVLMtEph::insert(self, value) }insert261,8378
        fn find(&self, target: &T) -> Option<T> { BSTAVLMtEph::find(self, target) }find263,8451
        fn contains(&self, target: &T) -> B { BSTAVLMtEph::contains(self, target) }contains265,8536
        fn size(&self) -> N { BSTAVLMtEph::size(self) }size267,8621
        fn is_empty(&self) -> B { BSTAVLMtEph::is_empty(self) }is_empty269,8678
        fn height(&self) -> N { BSTAVLMtEph::height(self) }height271,8743
        fn minimum(&self) -> Option<T> { BSTAVLMtEph::minimum(self) }minimum273,8804
        fn maximum(&self) -> Option<T> { BSTAVLMtEph::maximum(self) }maximum275,8875
        fn in_order(&self) -> ArrayStPerS<T> { BSTAVLMtEph::in_order(self) }in_order277,8946
        fn pre_order(&self) -> ArrayStPerS<T> { BSTAVLMtEph::pre_order(self) }pre_order279,9024
    macro_rules! BSTAVLMtEphLit {BSTAVLMtEphLit283,9130
    fn _BSTAVLMtEphLit_type_checks() {_BSTAVLMtEphLit_type_checks295,9633

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTPlainStEph.rs,3550
pub mod BSTPlainStEph {BSTPlainStEph3,64
    pub struct BSTPlainStEph<T: StT + Ord> {BSTPlainStEph8,216
        root: BBTree<T>,root9,261
    pub type BSTree<T> = BSTPlainStEph<T>;BSTree12,293
    pub trait BSTPlainStEphTrait<T: StT + Ord> {BSTPlainStEphTrait14,337
        fn new() -> Self;new15,386
        fn size(&self) -> N;size16,412
        fn is_empty(&self) -> B;is_empty17,441
        fn height(&self) -> N;height18,474
        fn insert(&mut self, value: T);insert19,505
        fn find(&self, target: &T) -> Option<&T>;find20,545
        fn contains(&self, target: &T) -> B;contains21,595
        fn minimum(&self) -> Option<&T>;minimum22,640
        fn maximum(&self) -> Option<&T>;maximum23,681
        fn in_order(&self) -> ArrayStPerS<T>;in_order24,722
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order25,768
    impl<T: StT + Ord> BSTPlainStEph<T> {BSTPlainStEph28,822
        pub fn new() -> Self { BSTPlainStEph { root: BBTree::leaf() } }new29,864
        pub fn size(&self) -> N { self.root.size() }size31,937
        pub fn is_empty(&self) -> B { self.root.is_leaf() }is_empty33,991
        pub fn height(&self) -> N { self.root.height() }height35,1052
        pub fn insert(&mut self, value: T) { insert_node(&mut self.root, value); }insert37,1110
        pub fn find(&self, target: &T) -> Option<&T> { find_node(&self.root, target) }find39,1194
        pub fn contains(&self, target: &T) -> B { contains_node(&self.root, target) }contains41,1282
        pub fn minimum(&self) -> Option<&T> { min_node(&self.root) }minimum43,1369
        pub fn maximum(&self) -> Option<&T> { max_node(&self.root) }maximum45,1439
        pub fn in_order(&self) -> ArrayStPerS<T> { self.root.in_order() }in_order47,1509
        pub fn pre_order(&self) -> ArrayStPerS<T> { self.root.pre_order() }pre_order49,1584
    impl<T: StT + Ord> BSTPlainStEphTrait<T> for BSTPlainStEph<T> {BSTPlainStEph52,1667
        fn new() -> Self { BSTPlainStEph::new() }new53,1735
        fn size(&self) -> N { BSTPlainStEph::size(self) }size55,1786
        fn is_empty(&self) -> B { BSTPlainStEph::is_empty(self) }is_empty57,1845
        fn height(&self) -> N { BSTPlainStEph::height(self) }height59,1912
        fn insert(&mut self, value: T) { BSTPlainStEph::insert(self, value) }insert61,1975
        fn find(&self, target: &T) -> Option<&T> { BSTPlainStEph::find(self, target) }find63,2054
        fn contains(&self, target: &T) -> B { BSTPlainStEph::contains(self, target) }contains65,2142
        fn minimum(&self) -> Option<&T> { BSTPlainStEph::minimum(self) }minimum67,2229
        fn maximum(&self) -> Option<&T> { BSTPlainStEph::maximum(self) }maximum69,2303
        fn in_order(&self) -> ArrayStPerS<T> { BSTPlainStEph::in_order(self) }in_order71,2377
        fn pre_order(&self) -> ArrayStPerS<T> { BSTPlainStEph::pre_order(self) }pre_order73,2457
    fn insert_node<T: StT + Ord>(node: &mut BBTree<T>, value: T) {insert_node76,2545
    fn contains_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> B {contains_node91,3048
    fn find_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> Option<&'a T> {find_node106,3534
    fn min_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {min_node121,4027
    fn max_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {max_node131,4347
    macro_rules! BSTPlainStEphLit {BSTPlainStEphLit142,4689
    fn _BSTPlainStEphLit_type_checks() {_BSTPlainStEphLit_type_checks157,5261

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap37/BSTSetBBAlphaMtEph.rs,5687
pub mod BSTSetBBAlphaMtEph {BSTSetBBAlphaMtEph3,78
    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord> {BSTSetBBAlphaMtEph10,307
        tree: BSTBBAlphaMtEph<T>,tree11,362
    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;BSTSetBBAlphaMt14,403
    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {BSTSetBBAlphaMtEphTrait16,461
        fn empty() -> Self;empty17,527
        fn singleton(value: T) -> Self;singleton18,555
        fn size(&self) -> N;size19,595
        fn is_empty(&self) -> B;is_empty20,624
        fn find(&self, value: &T) -> Option<T>;find21,657
        fn contains(&self, value: &T) -> B;contains22,705
        fn minimum(&self) -> Option<T>;minimum23,749
        fn maximum(&self) -> Option<T>;maximum24,789
        fn insert(&mut self, value: T);insert25,829
        fn delete(&mut self, target: &T);delete26,869
        fn union(&self, other: &Self) -> Self;union27,911
        fn intersection(&self, other: &Self) -> Self;intersection28,958
        fn difference(&self, other: &Self) -> Self;difference29,1012
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1064
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1119
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1174
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1236
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1306
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1374
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T>;as_tree36,1425
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph39,1482
        pub fn empty() -> Self {empty40,1534
        pub fn singleton(value: T) -> Self {singleton46,1657
        pub fn size(&self) -> N { self.tree.size() }size52,1818
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1872
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1934
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,2012
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2090
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2158
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2226
        pub fn delete(&mut self, target: &T) {delete66,2299
        pub fn union(&self, other: &Self) -> Self {union74,2588
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2887
        pub fn difference(&self, other: &Self) -> Self {difference99,3465
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4042
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4734
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5047
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5403
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5812
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6076
        pub fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree178,6156
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6225
        fn rebuild_from_vec(values: Vec<T>) -> BSTBBAlphaMtEph<T> {rebuild_from_vec182,6316
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6543
    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph202,6830
        fn empty() -> Self { Self::empty() }empty203,6913
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6959
        fn size(&self) -> N { self.tree.size() }size207,7026
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7076
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7134
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7208
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7282
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7346
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7410
        fn delete(&mut self, target: &T) { BSTSetBBAlphaMtEph::delete(self, target) }delete221,7479
        fn union(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::union(self, other) }union223,7566
        fn intersection(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::intersection(self, otintersection225,7656
        fn difference(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::difference(self, other)difference227,7760
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetBBAlphaMtEph::split(self, pivot) }split229,7860
        fn join_pair(left: Self, right: Self) -> Self { BSTSetBBAlphaMtEph::join_pair(left, righjoin_pair231,7958
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetBBAlphaMtEph::join_m(left, join_m233,8060
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetBBAlphaMtEph::filtefilter235,8173
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetBBAlphaMtEph::reduce(reduce237,8291
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8406
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree241,8482
    macro_rules! BSTSetBBAlphaMtEphLit {BSTSetBBAlphaMtEphLit245,8573
    fn _BSTSetBBAlphaMtEphLit_type_checks() {_BSTSetBBAlphaMtEphLit_type_checks257,9172

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36Mt.rs,1405
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36MtSlice.rs,1381
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36St.rs,1495
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test42BBTEph.rs,189
fn inorder_and_preorder_traversals_match_definitions() {inorder_and_preorder_traversals_match_definitions7,140
fn bst_insert_and_search_behavior() {bst_insert_and_search_behavior22,610

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test29Algorithm_21_1.rs,428
fn points2d_tab_flat(n: N) -> ArrayStPerS<Pair<N, N>> {points2d_tab_flat13,469
fn test_points2d_n3_example() {test_points2d_n3_example31,1067
fn test_points2d_n1_empty() {test_points2d_n1_empty45,1355
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values51,1461
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order59,1660
fn test_points2d_debug_shape() {test_points2d_debug_shape80,2163

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
    fn test_iterators_collect() {test_iterators_collect32,1109

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
    fn test_flatten() {test_flatten90,3337
    fn test_update_sequence() {test_update_sequence104,4058
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins116,4573
    fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins129,5331
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan145,6083
    fn test_iterate_sum_basic() {test_iterate_sum_basic164,7087
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum172,7359
    fn test_collect_groups_by_key() {test_collect_groups_by_key185,7832

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test10ArraySeqStPer18.rs,937
pub mod TestArraySeqStPer {TestArraySeqStPer1,0
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,330
        fn fib(n: N) -> N {fib12,365
    fn test_map_increment() {test_map_increment37,1022
    fn test_subseq() {test_subseq44,1257
    fn test_append() {test_append55,1594
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1857
    fn test_filter_even() {test_filter_even76,2468
    fn test_flatten() {test_flatten91,3144
    fn test_update_sequence() {test_update_sequence105,3902
    fn test_inject_and_ninject() {test_inject_and_ninject115,4399
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan140,5731
    fn test_iterate_sum_basic() {test_iterate_sum_basic159,6681
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum167,6947
    fn test_collect_groups_by_key() {test_collect_groups_by_key179,7402

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
    fn test_map_and_select_and_append() {test_map_and_select_and_append11,304
    fn test_deflate_and_filter() {test_deflate_and_filter24,835
    fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten40,1421
    fn test_inject_and_parallel() {test_inject_and_parallel59,2393

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
    fn test_new_and_set() {test_new_and_set9,284
    fn test_length_and_nth_basic() {test_length_and_nth_basic23,732
    fn test_empty() {test_empty31,942
    fn test_sequence_basic() {test_sequence_basic38,1126
    fn test_singleton() {test_singleton51,1816
    fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton59,2021
    fn test_from_vec() {test_from_vec74,2488
    fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers89,3139
    fn test_sequence_equality_strings() {test_sequence_equality_strings114,4051
    fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference139,5057
        struct PartialComparable {PartialComparable141,5148
            value: i32,value142,5183
        impl std::fmt::Display for PartialComparable {PartialComparable145,5269
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt146,5324
        struct TotalComparable {TotalComparable158,5956
            value: N,value159,5989
        impl std::fmt::Display for TotalComparable {TotalComparable162,6022
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt163,6075
    fn test_ordering_numbers_basic() {test_ordering_numbers_basic178,6728
    fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal187,6975
    fn test_ordering_strings_basic() {test_ordering_strings_basic193,7103
    fn test_strings_equal_is_equal() {test_strings_equal_is_equal202,7348
    fn test_nth_on_empty_panics() {test_nth_on_empty_panics209,7497
    fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics216,7644
    fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap223,7782
    fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes229,7945
    fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic238,8182
    fn test_new_set_persistent() {test_new_set_persistent247,8523
    fn test_iterator_collects_in_order() {test_iterator_collects_in_order256,8789

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test53BSTParaTreapMtEph.rs,612
fn make_tree(values: &[i32]) -> ParamTreap<i32> {make_tree5,100
fn make_range_tree(start: i32, end: i32) -> ParamTreap<i32> {make_range_tree13,257
fn treap_basic_insert_find() {treap_basic_insert_find22,437
fn treap_split_join_pair() {treap_split_join_pair32,752
fn treap_union_intersect_difference() {treap_union_intersect_difference44,1150
fn treap_filter_reduce() {treap_filter_reduce62,1911
fn treap_join_mid_roundtrip() {treap_join_mid_roundtrip75,2307
fn treap_invariants_priority_heap() {treap_invariants_priority_heap94,2901
    fn check_heap(tree: &ParamTreap<i32>) {check_heap95,2939

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test24DirGraphStEph.rs,129
pub mod TestDirGraphStEph {TestDirGraphStEph1,0
    fn test_digraph_vertices_and_arcs() {test_digraph_vertices_and_arcs8,218

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test26LabDirGraphStEph.rs,1037
pub mod TestLabDirGraphStEph {TestLabDirGraphStEph1,0
    fn test_labelled_dir_graph_empty() {test_labelled_dir_graph_empty8,250
    fn test_labelled_dir_graph_add_vertex() {test_labelled_dir_graph_add_vertex16,541
    fn test_labelled_dir_graph_add_labeled_arc() {test_labelled_dir_graph_add_labeled_arc28,933
    fn test_labelled_dir_graph_neighbors() {test_labelled_dir_graph_neighbors45,1559
    fn test_labelled_dir_graph_arcs() {test_labelled_dir_graph_arcs69,2445
    fn test_labelled_dir_graph_macro_empty() {test_labelled_dir_graph_macro_empty81,2842
    fn test_labelled_dir_graph_macro_with_data() {test_labelled_dir_graph_macro_with_data88,3069
    fn test_labelled_dir_graph_different_label_types() {test_labelled_dir_graph_different_label_types103,3561
    fn test_labelled_dir_graph_display() {test_labelled_dir_graph_display120,4055
    fn test_labelled_dir_graph_debug() {test_labelled_dir_graph_debug132,4409
    fn test_labelled_dir_graph_self_loop() {test_labelled_dir_graph_self_loop144,4768

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test27LabUnDirGraphStEph.rs,1329
pub mod TestLabUnDirGraphStEph {TestLabUnDirGraphStEph1,0
    fn test_labelled_undir_graph_empty() {test_labelled_undir_graph_empty8,258
    fn test_labelled_undir_graph_add_vertex() {test_labelled_undir_graph_add_vertex16,558
    fn test_labelled_undir_graph_add_labeled_edge() {test_labelled_undir_graph_add_labeled_edge28,957
    fn test_labelled_undir_graph_neighbors() {test_labelled_undir_graph_neighbors49,1891
    fn test_labelled_undir_graph_edges() {test_labelled_undir_graph_edges72,2733
    fn test_labelled_undir_graph_macro_empty() {test_labelled_undir_graph_macro_empty86,3258
    fn test_labelled_undir_graph_macro_with_data() {test_labelled_undir_graph_macro_with_data93,3492
    fn test_labelled_undir_graph_edge_normalization() {test_labelled_undir_graph_edge_normalization110,4095
    fn test_labelled_undir_graph_different_label_types() {test_labelled_undir_graph_different_label_types124,4620
    fn test_labelled_undir_graph_display() {test_labelled_undir_graph_display143,5275
    fn test_labelled_undir_graph_debug() {test_labelled_undir_graph_debug155,5635
    fn test_labelled_undir_graph_self_loop() {test_labelled_undir_graph_self_loop167,6001
    fn test_labelled_undir_graph_multiple_edges_same_vertices() {test_labelled_undir_graph_multiple_edges_same_vertices180,6444

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test25UnDirGraphStEph.rs,139
pub mod TestUnDirGraphStEph {TestUnDirGraphStEph1,0
    fn test_undigraph_vertices_and_edges() {test_undigraph_vertices_and_edges8,224

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/TestExercise12_5.rs,316
fn push_pop_lifo_single_thread() {push_pop_lifo_single_thread7,179
fn pop_on_empty_returns_none() {pop_on_empty_returns_none21,517
fn multi_thread_push_collects_all_items() {multi_thread_push_collects_all_items27,665
fn multi_thread_pop_consumes_all_elements() {multi_thread_pop_consumes_all_elements55,1485

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap03/TestInsertionSortSt.rs,517
fn sort_and_assert(mut data: Vec<i32>, expected: &[i32]) {sort_and_assert3,97
fn insertion_sort_handles_empty() {insertion_sort_handles_empty9,250
fn insertion_sort_single_element() {insertion_sort_single_element16,419
fn insertion_sort_already_sorted() {insertion_sort_already_sorted21,505
fn insertion_sort_reverse_order() {insertion_sort_reverse_order26,613
fn insertion_sort_with_duplicates() {insertion_sort_with_duplicates31,720
fn insertion_sort_random_slice() {insertion_sort_random_slice36,829

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test39Chapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test35Exercsise_21_9.rs,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test38Problem21_1.rs,425
fn points2d(n: N) -> ArrayStPerS<Pair<N, N>> {points2d10,322
fn test_points2d_n3_example() {test_points2d_n3_example25,642
fn test_points2d_n1_empty() {test_points2d_n1_empty33,888
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values39,985
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order47,1167
fn test_points2d_debug_display_shapes() {test_points2d_debug_display_shapes68,1613

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test39Chapter36St.rs,550
trait ToVec<T: StT> {ToVec5,106
    fn to_vec(&self) -> Vec<T>;to_vec6,128
impl<T: StT> ToVec<T> for ArraySeqStEphS<T> {ArraySeqStEphS8,162
    fn to_vec(&self) -> Vec<T> {to_vec9,208
fn quick_sort_variants_produce_sorted_output() {quick_sort_variants_produce_sorted_output15,324
fn quick_sort_handles_edge_cases() {quick_sort_handles_edge_cases30,848
fn pivot_strategies_match_expectations() {pivot_strategies_match_expectations53,1623
fn quick_sort_small_inputs_use_shared_pivots() {quick_sort_small_inputs_use_shared_pivots78,2398

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test30Algorithm_21_2.rs,527
fn points3d_tab_flat(n: N) -> ArrayStPerS<Pair<N, Pair<N, N>>> {points3d_tab_flat13,526
fn test_points3d_tab_flat_n0_empty() {test_points3d_tab_flat_n0_empty36,1706
fn test_points3d_tab_flat_n1_single() {test_points3d_tab_flat_n1_single42,1821
fn test_points3d_tab_flat_n2_values_and_order() {test_points3d_tab_flat_n2_values_and_order49,1988
fn test_points3d_tab_flat_iterator_order() {test_points3d_tab_flat_iterator_order66,2414
fn test_points3d_tab_flat_debug_shape() {test_points3d_tab_flat_debug_shape85,2908

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test34Exercise_21_8_and_Algorithm_21_5.rs,347
fn is_divisible(n: N, i: N) -> B {is_divisible8,248
fn is_prime(n: N) -> B {is_prime15,495
fn primes_bf(n: N) -> ArrayStPerS<N> {primes_bf29,1086
fn test_is_prime_small_values() {test_is_prime_small_values39,1432
fn test_primes_bf_small() {test_primes_bf_small49,1709
fn test_primes_bf_debug_shape() {test_primes_bf_debug_shape56,1861

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test33Exercise_21_7.rs,385
fn is_even(x: &N) -> B {is_even10,324
fn is_vowel(c: &char) -> B {is_vowel13,400
fn pair_even_with_vowels(a: &ArrayStPerS<N>, b: &ArrayStPerS<char>) -> ArrayStPerS<Pair<N, char>pair_even_with_vowels22,689
fn test_pair_even_with_vowels_basic() {test_pair_even_with_vowels_basic41,1532
fn test_pair_even_with_vowels_debug_shape() {test_pair_even_with_vowels_debug_shape50,1828

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test23MappingStEph.rs,562
pub mod Test23MappingStEphChap5_5 {Test23MappingStEphChap5_53,55
    fn test_empty_mapping() {test_empty_mapping12,404
    fn test_from_vec_basic() {test_from_vec_basic20,618
    fn test_from_vec_duplicate_keys() {test_from_vec_duplicate_keys32,1125
    fn test_from_relation() {test_from_relation43,1683
    fn test_domain_and_range() {test_domain_and_range57,2476
    fn test_iter() {test_iter76,3191
    fn test_mem_comprehensive() {test_mem_comprehensive89,3675
    fn test_empty_mapping_operations() {test_empty_mapping_operations108,4344

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test22RelationStEph.rs,151
pub mod TestRelationStEphChap5_2 {TestRelationStEphChap5_21,0
    fn test_relation_domain_range_and_mem() {test_relation_domain_range_and_mem9,296

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test21SetStEph.rs,575
pub mod TestSetStEphChap5_1 {TestSetStEphChap5_11,0
    fn macro_typecheck_exercise() {macro_typecheck_exercise9,220
        let _empty: Set<&'static str> = SetLit![];str10,256
    fn test_cartesian_product_example_5_1() {test_cartesian_product_example_5_116,406
    fn test_partition_example_5_2_true() {test_partition_example_5_2_true32,928
    fn test_partition_example_5_2_false_due_to_overlap() {test_partition_example_5_2_false_due_to_overlap41,1225
    fn test_partition_false_due_to_missing_element() {test_partition_false_due_to_missing_element50,1570

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/TestExercise12_2.rs,239
fn fetch_add_cas_returns_previous_value() {fetch_add_cas_returns_previous_value7,178
fn trait_impl_matches_free_function() {trait_impl_matches_free_function14,368
fn fetch_add_cas_is_thread_safe() {fetch_add_cas_is_thread_safe24,654

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test48BSTTreapStEph.rs,166
fn treap_insert_find_stays_balanced() {treap_insert_find_stays_balanced5,71
fn treap_duplicate_insert_is_idempotent() {treap_duplicate_insert_is_idempotent25,698

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test44Chapter36MtSlice.rs,672
fn to_vec<T: StT>(a: &ArraySeqMtEphSliceS<T>) -> Vec<T> {to_vec6,112
fn is_sorted<T: StT + Ord>(values: &[T]) -> bool {is_sorted10,188
fn mk_seq(data: &[i32]) -> ArraySeqMtEphSliceS<i32> {mk_seq14,286
fn quick_sort_slice_variants_produce_sorted_output() {quick_sort_slice_variants_produce_sorted_output19,400
fn quick_sort_slice_edge_cases() {quick_sort_slice_edge_cases37,889
fn quick_sort_slice_large_inputs() {quick_sort_slice_large_inputs60,1567
fn slice_pivot_strategies_match_expectations() {slice_pivot_strategies_match_expectations73,2052
fn quick_sort_slice_small_inputs_use_shared_pivots() {quick_sort_slice_small_inputs_use_shared_pivots93,2677

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test37Problem_21_4.rs,1296
fn cartesian_loops(a: &ArrayStPerS<N>, b: &ArrayStPerS<&'static str>) -> ArrayStPerS<Pair<N, &'scartesian_loops12,384
    let mut v: Vec<Pair<N, &'static str>> = Vec::with_capacity(a.length() * b.length());str13,494
fn cartesian_tab_flat(a: &ArrayStPerS<N>, b: &ArrayStPerS<&'static str>) -> ArrayStPerS<Pair<N, cartesian_tab_flat24,874
    let nested: ArrayStPerS<ArrayStPerS<Pair<N, &'static str>>> =str25,987
        <ArrayStPerS<ArrayStPerS<Pair<N, &'static str>>> as ArraySeqStPerTrait<str26,1053
            ArrayStPerS<Pair<N, &'static str>>,str27,1133
                <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>str30,1221
                <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>str30,1221
    <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::flatten(&str37,1482
    <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerTrait<Pair<N, &'static str>>>::flatten(&str37,1482
fn test_cartesian_loops_basic() {test_cartesian_loops_basic41,1597
fn test_cartesian_tab_flat_basic() {test_cartesian_tab_flat_basic57,1975
fn test_cartesian_iterator_order() {test_cartesian_iterator_order73,2359
fn test_cartesian_debug_shape() {test_cartesian_debug_shape82,2663

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test08LinkedListStEph19.rs,614
pub mod TestLinkedListStEph {TestLinkedListStEph1,0
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list8,269
    fn test_eph_set_and_nth() {test_eph_set_and_nth17,530
    fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug24,713
    fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1934,1074
    fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1940,1244
    fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1953,1841

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test10ArraySeqStPerChap18.rs,945
pub mod TestArraySeqStPerChap {TestArraySeqStPerChap1,0
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,352
        fn fib(n: N) -> N {fib12,387
    fn test_map_increment() {test_map_increment37,1044
    fn test_subseq() {test_subseq44,1285
    fn test_append() {test_append55,1622
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1891
    fn test_filter_even() {test_filter_even76,2520
    fn test_flatten() {test_flatten91,3208
    fn test_update_sequence() {test_update_sequence105,3978
    fn test_inject_and_ninject() {test_inject_and_ninject115,4487
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan140,5843
    fn test_iterate_sum_basic() {test_iterate_sum_basic159,6817
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum167,7089
    fn test_collect_groups_by_key() {test_collect_groups_by_key179,7550

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
    fn test_map() {test_map46,1679
    fn test_iterate_and_reduce() {test_iterate_and_reduce53,1905
    fn test_scan() {test_scan62,2262
    fn test_flatten() {test_flatten70,2550
    fn test_inject() {test_inject77,2833

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test04LinkedListStPer.rs,568
pub mod TestLinkedListStPer {TestLinkedListStPer1,0
    fn test_tabulate() {test_tabulate9,281
    fn test_map() {test_map16,484
    fn test_filter() {test_filter24,767
    fn test_append() {test_append33,1105
    fn test_update() {test_update41,1410
    fn test_inject() {test_inject48,1659
    fn test_ninject() {test_ninject56,1974
    fn test_iterate() {test_iterate64,2293
    fn test_reduce() {test_reduce71,2519
    fn test_scan() {test_scan78,2740
    fn test_flatten() {test_flatten86,3040
    fn test_collect() {test_collect97,3404

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
    fn test_update_ch18() {test_update_ch1872,2461
    fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1879,2692
    fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1889,3187
    fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch18101,3757

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test06LinkedListStEph.rs,547
pub mod TestLinkedListEph {TestLinkedListEph2,56
    fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates10,274
    fn test_new_and_nth_set() {test_new_and_nth_set19,569
    fn test_subseq_copy() {test_subseq_copy28,808
    fn test_linkedlisteph_basic() {test_linkedlisteph_basic37,1047
    fn test_debug_format_for_eph() {test_debug_format_for_eph46,1291
    fn test_display_format_for_eph() {test_display_format_for_eph52,1447
    fn test_iter_inorder_collect_eph() {test_iter_inorder_collect_eph58,1603

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test40Chapter36Mt.rs,569
fn to_vec<T: StT>(a: &ArraySeqMtEphS<T>) -> Vec<T> {to_vec7,129
fn is_sorted<T: StT + Ord>(values: &[T]) -> bool {is_sorted11,240
fn quick_sort_mt_variants_produce_sorted_output() {quick_sort_mt_variants_produce_sorted_output16,346
fn quick_sort_mt_edge_cases() {quick_sort_mt_edge_cases34,853
fn pivot_mt_strategies_match_expectations() {pivot_mt_strategies_match_expectations57,1577
fn quick_sort_mt_large_inputs() {quick_sort_mt_large_inputs78,2284
fn quick_sort_mt_small_inputs_use_shared_pivots() {quick_sort_mt_small_inputs_use_shared_pivots91,2764

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test51BSTSetMtEph.rs,8555
trait TestSet: Sized {TestSet4,46
    fn empty() -> Self;empty5,69
    fn insert(&mut self, value: i32);insert6,93
    fn delete(&mut self, value: &i32);delete7,131
    fn size(&self) -> usize;size8,170
    fn is_empty(&self) -> B;is_empty9,199
    fn contains(&self, value: &i32) -> B;contains10,228
    fn minimum(&self) -> Option<i32>;minimum11,270
    fn maximum(&self) -> Option<i32>;maximum12,308
    fn union(&self, other: &Self) -> Self;union13,346
    fn intersection(&self, other: &Self) -> Self;intersection14,389
    fn difference(&self, other: &Self) -> Self;difference15,439
    fn split(&self, pivot: &i32) -> (Self, B, Self);split16,487
    fn join_pair(left: Self, right: Self) -> Self;join_pair17,540
    fn join_m(left: Self, pivot: i32, right: Self) -> Self;join_m18,591
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self;filter19,651
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32;reduce20,719
    fn iter_seq(&self) -> ArrayStPerS<i32>;iter_seq21,793
impl TestSet for apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32> {BSTSetPlainMt24,840
    fn empty() -> Self {empty25,931
    fn insert(&mut self, value: i32) {insert29,985
    fn delete(&mut self, value: &i32) {delete33,1059
    fn size(&self) -> usize {size37,1134
    fn is_empty(&self) -> B {is_empty41,1191
    fn contains(&self, value: &i32) -> B {contains45,1252
    fn minimum(&self) -> Option<i32> {minimum49,1331
    fn maximum(&self) -> Option<i32> {maximum53,1400
    fn union(&self, other: &Self) -> Self {union57,1469
    fn intersection(&self, other: &Self) -> Self {intersection61,1546
    fn difference(&self, other: &Self) -> Self {difference65,1637
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split69,1724
    fn join_pair(left: Self, right: Self) -> Self {join_pair73,1811
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m77,1907
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter81,2016
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce85,2123
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq89,2235
impl TestSet for apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32> {BSTSetAVLMt94,2318
    fn empty() -> Self {empty95,2403
    fn insert(&mut self, value: i32) {insert99,2457
    fn delete(&mut self, value: &i32) {delete103,2531
    fn size(&self) -> usize {size107,2606
    fn is_empty(&self) -> B {is_empty111,2663
    fn contains(&self, value: &i32) -> B {contains115,2724
    fn minimum(&self) -> Option<i32> {minimum119,2803
    fn maximum(&self) -> Option<i32> {maximum123,2872
    fn union(&self, other: &Self) -> Self {union127,2941
    fn intersection(&self, other: &Self) -> Self {intersection131,3018
    fn difference(&self, other: &Self) -> Self {difference135,3109
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split139,3196
    fn join_pair(left: Self, right: Self) -> Self {join_pair143,3283
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m147,3379
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter151,3488
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce155,3595
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq159,3707
impl TestSet for apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32> {BSTSetRBMt164,3790
    fn empty() -> Self {empty165,3872
    fn insert(&mut self, value: i32) {insert169,3926
    fn delete(&mut self, value: &i32) {delete173,4000
    fn size(&self) -> usize {size177,4075
    fn is_empty(&self) -> B {is_empty181,4132
    fn contains(&self, value: &i32) -> B {contains185,4193
    fn minimum(&self) -> Option<i32> {minimum189,4272
    fn maximum(&self) -> Option<i32> {maximum193,4341
    fn union(&self, other: &Self) -> Self {union197,4410
    fn intersection(&self, other: &Self) -> Self {intersection201,4487
    fn difference(&self, other: &Self) -> Self {difference205,4578
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split209,4665
    fn join_pair(left: Self, right: Self) -> Self {join_pair213,4752
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m217,4848
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter221,4957
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce225,5064
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq229,5176
impl TestSet for apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32> {BSTSetBBAlphaMt234,5259
    fn empty() -> Self {empty235,5356
    fn insert(&mut self, value: i32) {insert239,5410
    fn delete(&mut self, value: &i32) {delete243,5484
    fn size(&self) -> usize {size247,5559
    fn is_empty(&self) -> B {is_empty251,5616
    fn contains(&self, value: &i32) -> B {contains255,5677
    fn minimum(&self) -> Option<i32> {minimum259,5756
    fn maximum(&self) -> Option<i32> {maximum263,5825
    fn union(&self, other: &Self) -> Self {union267,5894
    fn intersection(&self, other: &Self) -> Self {intersection271,5971
    fn difference(&self, other: &Self) -> Self {difference275,6062
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split279,6149
    fn join_pair(left: Self, right: Self) -> Self {join_pair283,6236
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m287,6332
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter291,6441
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce295,6548
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq299,6660
impl TestSet for apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32> {BSTSetTreapMt304,6743
    fn empty() -> Self {empty305,6826
    fn insert(&mut self, value: i32) {insert309,6880
    fn delete(&mut self, value: &i32) {delete313,6954
    fn size(&self) -> usize {size317,7029
    fn is_empty(&self) -> B {is_empty321,7086
    fn contains(&self, value: &i32) -> B {contains325,7147
    fn minimum(&self) -> Option<i32> {minimum329,7226
    fn maximum(&self) -> Option<i32> {maximum333,7295
    fn union(&self, other: &Self) -> Self {union337,7364
    fn intersection(&self, other: &Self) -> Self {intersection341,7441
    fn difference(&self, other: &Self) -> Self {difference345,7532
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split349,7619
    fn join_pair(left: Self, right: Self) -> Self {join_pair353,7706
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m357,7802
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter361,7911
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce365,8018
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq369,8130
impl TestSet for apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32> {BSTSetSplayMt374,8213
    fn empty() -> Self {empty375,8304
    fn insert(&mut self, value: i32) {insert379,8358
    fn delete(&mut self, value: &i32) {delete383,8432
    fn size(&self) -> usize {size387,8507
    fn is_empty(&self) -> B {is_empty391,8564
    fn contains(&self, value: &i32) -> B {contains395,8625
    fn minimum(&self) -> Option<i32> {minimum399,8704
    fn maximum(&self) -> Option<i32> {maximum403,8773
    fn union(&self, other: &Self) -> Self {union407,8842
    fn intersection(&self, other: &Self) -> Self {intersection411,8919
    fn difference(&self, other: &Self) -> Self {difference415,9010
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split419,9097
    fn join_pair(left: Self, right: Self) -> Self {join_pair423,9184
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m427,9280
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter431,9389
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce435,9496
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq439,9608
fn exercise_set<S: TestSet>() {exercise_set444,9691
fn test_plain_bst_set_ops() {test_plain_bst_set_ops504,11446
fn test_avl_bst_set_ops() {test_avl_bst_set_ops509,11582
fn test_rb_bst_set_ops() {test_rb_bst_set_ops514,11710
fn test_bbalpha_bst_set_ops() {test_bbalpha_bst_set_ops519,11834
fn test_treap_bst_set_ops() {test_treap_bst_set_ops524,11978
fn test_splay_bst_set_ops() {test_splay_bst_set_ops529,12106

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
    fn test_tabulate_inorder() {test_tabulate_inorder13,411
    fn test_map_increment() {test_map_increment19,639
    fn test_append_union() {test_append_union26,984
    fn test_filter_even() {test_filter_even34,1441

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaMtEph.rs,782
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,98
fn make_range_tree(start: i32, end: i32) -> ParamBST<i32> {make_range_tree13,251
fn para_basic_insert_find() {para_basic_insert_find22,427
fn para_split_and_join_pair() {para_split_and_join_pair32,741
fn para_union_and_delete() {para_union_and_delete44,1148
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip58,1567
fn para_intersect_and_difference() {para_intersect_and_difference80,2233
fn para_filter_and_reduce() {para_filter_and_reduce92,2579
fn para_union_large_balanced() {para_union_large_balanced106,2942
fn para_intersect_and_difference_large() {para_intersect_and_difference_large117,3242
fn para_filter_and_reduce_edge_cases() {para_filter_and_reduce_edge_cases133,3845

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test17AVLTreeSeqStPer19.rs,371
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer1,0
    fn test_tabulate_and_map_ch19() {test_tabulate_and_map_ch199,297
    fn test_select_and_append_ch19() {test_select_and_append_ch1917,661
    fn test_deflate_and_filter_ch19() {test_deflate_and_filter_ch1938,1529
    fn test_iter_inorder_after_pipeline_ch19() {test_iter_inorder_after_pipeline_ch1956,2259

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaStEph.rs,322
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,98
fn para_basic_insert_find() {para_basic_insert_find14,259
fn para_split_and_join_pair() {para_split_and_join_pair24,573
fn para_union_and_delete() {para_union_and_delete36,980
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip50,1399

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test32Exercise_21_5_and_21_6.rs,380
fn all_contiguous_subseqs<T: StT>(a: &ArrayStPerS<T>) -> ArrayStPerS<ArrayStPerS<T>> {all_contiguous_subseqs13,463
fn test_all_contiguous_subseqs_n0() {test_all_contiguous_subseqs_n032,1199
fn test_all_contiguous_subseqs_n3_values() {test_all_contiguous_subseqs_n3_values39,1370
fn test_all_contiguous_subseqs_debug_shape() {test_all_contiguous_subseqs_debug_shape48,1706

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test36Problem_21_3.rs,489
fn points3d_loops(n: N) -> ArrayStPerS<Pair<N, Pair<N, N>>> {points3d_loops10,377
fn test_points3d_loops_n0_empty() {test_points3d_loops_n0_empty27,780
fn test_points3d_loops_n1_single() {test_points3d_loops_n1_single33,889
fn test_points3d_loops_n2_values_and_order() {test_points3d_loops_n2_values_and_order40,1050
fn test_points3d_loops_iterator_order() {test_points3d_loops_iterator_order57,1470
fn test_points3d_loops_debug_shape() {test_points3d_loops_debug_shape76,1958

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/TestExercise12_1.rs,347
fn spin_lock_excludes_parallel_threads() {spin_lock_excludes_parallel_threads8,183
fn spin_lock_with_lock_helper_executes_body() {spin_lock_with_lock_helper_executes_body34,975
fn parallel_increment_counts_all_iterations() {parallel_increment_counts_all_iterations42,1208
fn spin_lock_is_non_reentrant() {spin_lock_is_non_reentrant47,1317

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test31Algorithm_21_6.rs,261
fn prime_sieve(n: N) -> ArrayStPerS<N> {prime_sieve13,505
fn test_prime_sieve_small() {test_prime_sieve_small61,2406
fn test_prime_sieve_n2_empty() {test_prime_sieve_n2_empty67,2524
fn test_prime_sieve_debug_shape() {test_prime_sieve_debug_shape73,2627

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap03/TestInsertionSortSt.rs,517
fn sort_and_assert(mut data: Vec<i32>, expected: &[i32]) {sort_and_assert3,97
fn insertion_sort_handles_empty() {insertion_sort_handles_empty9,250
fn insertion_sort_single_element() {insertion_sort_single_element16,419
fn insertion_sort_already_sorted() {insertion_sort_already_sorted21,505
fn insertion_sort_reverse_order() {insertion_sort_reverse_order26,613
fn insertion_sort_with_duplicates() {insertion_sort_with_duplicates31,720
fn insertion_sort_random_slice() {insertion_sort_random_slice36,829

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test23MappingStEph.rs,562
pub mod Test23MappingStEphChap5_5 {Test23MappingStEphChap5_53,55
    fn test_empty_mapping() {test_empty_mapping12,404
    fn test_from_vec_basic() {test_from_vec_basic20,618
    fn test_from_vec_duplicate_keys() {test_from_vec_duplicate_keys32,1125
    fn test_from_relation() {test_from_relation43,1683
    fn test_domain_and_range() {test_domain_and_range57,2476
    fn test_iter() {test_iter76,3191
    fn test_mem_comprehensive() {test_mem_comprehensive89,3675
    fn test_empty_mapping_operations() {test_empty_mapping_operations108,4344

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test22RelationStEph.rs,151
pub mod TestRelationStEphChap5_2 {TestRelationStEphChap5_21,0
    fn test_relation_domain_range_and_mem() {test_relation_domain_range_and_mem9,296

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap05/Test21SetStEph.rs,575
pub mod TestSetStEphChap5_1 {TestSetStEphChap5_11,0
    fn macro_typecheck_exercise() {macro_typecheck_exercise9,220
        let _empty: Set<&'static str> = SetLit![];str10,256
    fn test_cartesian_product_example_5_1() {test_cartesian_product_example_5_116,406
    fn test_partition_example_5_2_true() {test_partition_example_5_2_true32,928
    fn test_partition_example_5_2_false_due_to_overlap() {test_partition_example_5_2_false_due_to_overlap41,1225
    fn test_partition_false_due_to_missing_element() {test_partition_false_due_to_missing_element50,1570

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test24DirGraphStEph.rs,129
pub mod TestDirGraphStEph {TestDirGraphStEph1,0
    fn test_digraph_vertices_and_arcs() {test_digraph_vertices_and_arcs8,218

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test26LabDirGraphStEph.rs,1037
pub mod TestLabDirGraphStEph {TestLabDirGraphStEph1,0
    fn test_labelled_dir_graph_empty() {test_labelled_dir_graph_empty8,250
    fn test_labelled_dir_graph_add_vertex() {test_labelled_dir_graph_add_vertex16,541
    fn test_labelled_dir_graph_add_labeled_arc() {test_labelled_dir_graph_add_labeled_arc28,933
    fn test_labelled_dir_graph_neighbors() {test_labelled_dir_graph_neighbors45,1559
    fn test_labelled_dir_graph_arcs() {test_labelled_dir_graph_arcs69,2445
    fn test_labelled_dir_graph_macro_empty() {test_labelled_dir_graph_macro_empty81,2842
    fn test_labelled_dir_graph_macro_with_data() {test_labelled_dir_graph_macro_with_data88,3069
    fn test_labelled_dir_graph_different_label_types() {test_labelled_dir_graph_different_label_types103,3561
    fn test_labelled_dir_graph_display() {test_labelled_dir_graph_display120,4055
    fn test_labelled_dir_graph_debug() {test_labelled_dir_graph_debug132,4409
    fn test_labelled_dir_graph_self_loop() {test_labelled_dir_graph_self_loop144,4768

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test27LabUnDirGraphStEph.rs,1329
pub mod TestLabUnDirGraphStEph {TestLabUnDirGraphStEph1,0
    fn test_labelled_undir_graph_empty() {test_labelled_undir_graph_empty8,258
    fn test_labelled_undir_graph_add_vertex() {test_labelled_undir_graph_add_vertex16,558
    fn test_labelled_undir_graph_add_labeled_edge() {test_labelled_undir_graph_add_labeled_edge28,957
    fn test_labelled_undir_graph_neighbors() {test_labelled_undir_graph_neighbors49,1891
    fn test_labelled_undir_graph_edges() {test_labelled_undir_graph_edges72,2733
    fn test_labelled_undir_graph_macro_empty() {test_labelled_undir_graph_macro_empty86,3258
    fn test_labelled_undir_graph_macro_with_data() {test_labelled_undir_graph_macro_with_data93,3492
    fn test_labelled_undir_graph_edge_normalization() {test_labelled_undir_graph_edge_normalization110,4095
    fn test_labelled_undir_graph_different_label_types() {test_labelled_undir_graph_different_label_types124,4620
    fn test_labelled_undir_graph_display() {test_labelled_undir_graph_display143,5275
    fn test_labelled_undir_graph_debug() {test_labelled_undir_graph_debug155,5635
    fn test_labelled_undir_graph_self_loop() {test_labelled_undir_graph_self_loop167,6001
    fn test_labelled_undir_graph_multiple_edges_same_vertices() {test_labelled_undir_graph_multiple_edges_same_vertices180,6444

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap06/Test25UnDirGraphStEph.rs,139
pub mod TestUnDirGraphStEph {TestUnDirGraphStEph1,0
    fn test_undigraph_vertices_and_edges() {test_undigraph_vertices_and_edges8,224

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test08LinkedListStEph19.rs,614
pub mod TestLinkedListStEph {TestLinkedListStEph1,0
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list8,269
    fn test_eph_set_and_nth() {test_eph_set_and_nth17,530
    fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug24,713
    fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1934,1074
    fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1940,1244
    fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1953,1841

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test10ArraySeqStPerChap18.rs,945
pub mod TestArraySeqStPerChap {TestArraySeqStPerChap1,0
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,352
        fn fib(n: N) -> N {fib12,387
    fn test_map_increment() {test_map_increment37,1044
    fn test_subseq() {test_subseq44,1285
    fn test_append() {test_append55,1622
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1891
    fn test_filter_even() {test_filter_even76,2520
    fn test_flatten() {test_flatten91,3208
    fn test_update_sequence() {test_update_sequence105,3978
    fn test_inject_and_ninject() {test_inject_and_ninject115,4487
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan140,5843
    fn test_iterate_sum_basic() {test_iterate_sum_basic159,6817
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum167,7089
    fn test_collect_groups_by_key() {test_collect_groups_by_key179,7550

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
    fn test_map() {test_map46,1679
    fn test_iterate_and_reduce() {test_iterate_and_reduce53,1905
    fn test_scan() {test_scan62,2262
    fn test_flatten() {test_flatten70,2550
    fn test_inject() {test_inject77,2833

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap18/Test04LinkedListStPer.rs,568
pub mod TestLinkedListStPer {TestLinkedListStPer1,0
    fn test_tabulate() {test_tabulate9,281
    fn test_map() {test_map16,484
    fn test_filter() {test_filter24,767
    fn test_append() {test_append33,1105
    fn test_update() {test_update41,1410
    fn test_inject() {test_inject48,1659
    fn test_ninject() {test_ninject56,1974
    fn test_iterate() {test_iterate64,2293
    fn test_reduce() {test_reduce71,2519
    fn test_scan() {test_scan78,2740
    fn test_flatten() {test_flatten86,3040
    fn test_collect() {test_collect97,3404

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
    fn test_update_ch18() {test_update_ch1872,2461
    fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1879,2692
    fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1889,3187
    fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch18101,3757

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test12ArraySeqStEph.rs,283
pub mod TestArraySeqEph {TestArraySeqEph1,0
    fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic11,308
    fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter20,574
    fn test_iterators_collect() {test_iterators_collect32,1109

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
    fn test_flatten() {test_flatten90,3337
    fn test_update_sequence() {test_update_sequence104,4058
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins116,4573
    fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins129,5331
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan145,6083
    fn test_iterate_sum_basic() {test_iterate_sum_basic164,7087
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum172,7359
    fn test_collect_groups_by_key() {test_collect_groups_by_key185,7832

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap19/Test10ArraySeqStPer18.rs,937
pub mod TestArraySeqStPer {TestArraySeqStPer1,0
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,330
        fn fib(n: N) -> N {fib12,365
    fn test_map_increment() {test_map_increment37,1022
    fn test_subseq() {test_subseq44,1257
    fn test_append() {test_append55,1594
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1857
    fn test_filter_even() {test_filter_even76,2468
    fn test_flatten() {test_flatten91,3144
    fn test_update_sequence() {test_update_sequence105,3902
    fn test_inject_and_ninject() {test_inject_and_ninject115,4399
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan140,5731
    fn test_iterate_sum_basic() {test_iterate_sum_basic159,6681
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum167,6947
    fn test_collect_groups_by_key() {test_collect_groups_by_key179,7402

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
    fn test_map_and_select_and_append() {test_map_and_select_and_append11,304
    fn test_deflate_and_filter() {test_deflate_and_filter24,835
    fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten40,1421
    fn test_inject_and_parallel() {test_inject_and_parallel59,2393

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
    fn test_new_and_set() {test_new_and_set9,284
    fn test_length_and_nth_basic() {test_length_and_nth_basic23,732
    fn test_empty() {test_empty31,942
    fn test_sequence_basic() {test_sequence_basic38,1126
    fn test_singleton() {test_singleton51,1816
    fn test_is_empty_and_is_singleton() {test_is_empty_and_is_singleton59,2021
    fn test_from_vec() {test_from_vec74,2488
    fn test_sequence_equality_natural_numbers() {test_sequence_equality_natural_numbers89,3139
    fn test_sequence_equality_strings() {test_sequence_equality_strings114,4051
    fn test_eq_vs_partial_eq_difference() {test_eq_vs_partial_eq_difference139,5057
        struct PartialComparable {PartialComparable141,5148
            value: i32,value142,5183
        impl std::fmt::Display for PartialComparable {PartialComparable145,5269
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt146,5324
        struct TotalComparable {TotalComparable158,5956
            value: N,value159,5989
        impl std::fmt::Display for TotalComparable {TotalComparable162,6022
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt163,6075
    fn test_ordering_numbers_basic() {test_ordering_numbers_basic178,6728
    fn test_numbers_equal_is_equal() {test_numbers_equal_is_equal187,6975
    fn test_ordering_strings_basic() {test_ordering_strings_basic193,7103
    fn test_strings_equal_is_equal() {test_strings_equal_is_equal202,7348
    fn test_nth_on_empty_panics() {test_nth_on_empty_panics209,7497
    fn test_nth_upper_bound_panics() {test_nth_upper_bound_panics216,7644
    fn test_set_out_of_bounds_panics_on_unwrap() {test_set_out_of_bounds_panics_on_unwrap223,7782
    fn test_set_in_bounds_ok_and_writes() {test_set_in_bounds_ok_and_writes229,7945
    fn test_subseq_copy_trait_form_basic() {test_subseq_copy_trait_form_basic238,8182
    fn test_new_set_persistent() {test_new_set_persistent247,8523
    fn test_iterator_collects_in_order() {test_iterator_collects_in_order256,8789

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test51BSTSetMtEph.rs,8555
trait TestSet: Sized {TestSet4,46
    fn empty() -> Self;empty5,69
    fn insert(&mut self, value: i32);insert6,93
    fn delete(&mut self, value: &i32);delete7,131
    fn size(&self) -> usize;size8,170
    fn is_empty(&self) -> B;is_empty9,199
    fn contains(&self, value: &i32) -> B;contains10,228
    fn minimum(&self) -> Option<i32>;minimum11,270
    fn maximum(&self) -> Option<i32>;maximum12,308
    fn union(&self, other: &Self) -> Self;union13,346
    fn intersection(&self, other: &Self) -> Self;intersection14,389
    fn difference(&self, other: &Self) -> Self;difference15,439
    fn split(&self, pivot: &i32) -> (Self, B, Self);split16,487
    fn join_pair(left: Self, right: Self) -> Self;join_pair17,540
    fn join_m(left: Self, pivot: i32, right: Self) -> Self;join_m18,591
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self;filter19,651
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32;reduce20,719
    fn iter_seq(&self) -> ArrayStPerS<i32>;iter_seq21,793
impl TestSet for apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32> {BSTSetPlainMt24,840
    fn empty() -> Self {empty25,931
    fn insert(&mut self, value: i32) {insert29,985
    fn delete(&mut self, value: &i32) {delete33,1059
    fn size(&self) -> usize {size37,1134
    fn is_empty(&self) -> B {is_empty41,1191
    fn contains(&self, value: &i32) -> B {contains45,1252
    fn minimum(&self) -> Option<i32> {minimum49,1331
    fn maximum(&self) -> Option<i32> {maximum53,1400
    fn union(&self, other: &Self) -> Self {union57,1469
    fn intersection(&self, other: &Self) -> Self {intersection61,1546
    fn difference(&self, other: &Self) -> Self {difference65,1637
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split69,1724
    fn join_pair(left: Self, right: Self) -> Self {join_pair73,1811
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m77,1907
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter81,2016
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce85,2123
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq89,2235
impl TestSet for apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32> {BSTSetAVLMt94,2318
    fn empty() -> Self {empty95,2403
    fn insert(&mut self, value: i32) {insert99,2457
    fn delete(&mut self, value: &i32) {delete103,2531
    fn size(&self) -> usize {size107,2606
    fn is_empty(&self) -> B {is_empty111,2663
    fn contains(&self, value: &i32) -> B {contains115,2724
    fn minimum(&self) -> Option<i32> {minimum119,2803
    fn maximum(&self) -> Option<i32> {maximum123,2872
    fn union(&self, other: &Self) -> Self {union127,2941
    fn intersection(&self, other: &Self) -> Self {intersection131,3018
    fn difference(&self, other: &Self) -> Self {difference135,3109
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split139,3196
    fn join_pair(left: Self, right: Self) -> Self {join_pair143,3283
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m147,3379
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter151,3488
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce155,3595
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq159,3707
impl TestSet for apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32> {BSTSetRBMt164,3790
    fn empty() -> Self {empty165,3872
    fn insert(&mut self, value: i32) {insert169,3926
    fn delete(&mut self, value: &i32) {delete173,4000
    fn size(&self) -> usize {size177,4075
    fn is_empty(&self) -> B {is_empty181,4132
    fn contains(&self, value: &i32) -> B {contains185,4193
    fn minimum(&self) -> Option<i32> {minimum189,4272
    fn maximum(&self) -> Option<i32> {maximum193,4341
    fn union(&self, other: &Self) -> Self {union197,4410
    fn intersection(&self, other: &Self) -> Self {intersection201,4487
    fn difference(&self, other: &Self) -> Self {difference205,4578
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split209,4665
    fn join_pair(left: Self, right: Self) -> Self {join_pair213,4752
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m217,4848
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter221,4957
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce225,5064
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq229,5176
impl TestSet for apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32> {BSTSetBBAlphaMt234,5259
    fn empty() -> Self {empty235,5356
    fn insert(&mut self, value: i32) {insert239,5410
    fn delete(&mut self, value: &i32) {delete243,5484
    fn size(&self) -> usize {size247,5559
    fn is_empty(&self) -> B {is_empty251,5616
    fn contains(&self, value: &i32) -> B {contains255,5677
    fn minimum(&self) -> Option<i32> {minimum259,5756
    fn maximum(&self) -> Option<i32> {maximum263,5825
    fn union(&self, other: &Self) -> Self {union267,5894
    fn intersection(&self, other: &Self) -> Self {intersection271,5971
    fn difference(&self, other: &Self) -> Self {difference275,6062
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split279,6149
    fn join_pair(left: Self, right: Self) -> Self {join_pair283,6236
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m287,6332
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter291,6441
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce295,6548
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq299,6660
impl TestSet for apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32> {BSTSetTreapMt304,6743
    fn empty() -> Self {empty305,6826
    fn insert(&mut self, value: i32) {insert309,6880
    fn delete(&mut self, value: &i32) {delete313,6954
    fn size(&self) -> usize {size317,7029
    fn is_empty(&self) -> B {is_empty321,7086
    fn contains(&self, value: &i32) -> B {contains325,7147
    fn minimum(&self) -> Option<i32> {minimum329,7226
    fn maximum(&self) -> Option<i32> {maximum333,7295
    fn union(&self, other: &Self) -> Self {union337,7364
    fn intersection(&self, other: &Self) -> Self {intersection341,7441
    fn difference(&self, other: &Self) -> Self {difference345,7532
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split349,7619
    fn join_pair(left: Self, right: Self) -> Self {join_pair353,7706
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m357,7802
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter361,7911
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce365,8018
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq369,8130
impl TestSet for apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32> {BSTSetSplayMt374,8213
    fn empty() -> Self {empty375,8304
    fn insert(&mut self, value: i32) {insert379,8358
    fn delete(&mut self, value: &i32) {delete383,8432
    fn size(&self) -> usize {size387,8507
    fn is_empty(&self) -> B {is_empty391,8564
    fn contains(&self, value: &i32) -> B {contains395,8625
    fn minimum(&self) -> Option<i32> {minimum399,8704
    fn maximum(&self) -> Option<i32> {maximum403,8773
    fn union(&self, other: &Self) -> Self {union407,8842
    fn intersection(&self, other: &Self) -> Self {intersection411,8919
    fn difference(&self, other: &Self) -> Self {difference415,9010
    fn split(&self, pivot: &i32) -> (Self, B, Self) {split419,9097
    fn join_pair(left: Self, right: Self) -> Self {join_pair423,9184
    fn join_m(left: Self, pivot: i32, right: Self) -> Self {join_m427,9280
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self {filter431,9389
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 {reduce435,9496
    fn iter_seq(&self) -> ArrayStPerS<i32> {iter_seq439,9608
fn exercise_set<S: TestSet>() {exercise_set444,9691
fn test_plain_bst_set_ops() {test_plain_bst_set_ops504,11446
fn test_avl_bst_set_ops() {test_avl_bst_set_ops509,11582
fn test_rb_bst_set_ops() {test_rb_bst_set_ops514,11710
fn test_bbalpha_bst_set_ops() {test_bbalpha_bst_set_ops519,11834
fn test_treap_bst_set_ops() {test_treap_bst_set_ops524,11978
fn test_splay_bst_set_ops() {test_splay_bst_set_ops529,12106

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
    fn test_tabulate_inorder() {test_tabulate_inorder13,411
    fn test_map_increment() {test_map_increment19,639
    fn test_append_union() {test_append_union26,984
    fn test_filter_even() {test_filter_even34,1441

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaMtEph.rs,782
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,98
fn make_range_tree(start: i32, end: i32) -> ParamBST<i32> {make_range_tree13,251
fn para_basic_insert_find() {para_basic_insert_find22,427
fn para_split_and_join_pair() {para_split_and_join_pair32,741
fn para_union_and_delete() {para_union_and_delete44,1148
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip58,1567
fn para_intersect_and_difference() {para_intersect_and_difference80,2233
fn para_filter_and_reduce() {para_filter_and_reduce92,2579
fn para_union_large_balanced() {para_union_large_balanced106,2942
fn para_intersect_and_difference_large() {para_intersect_and_difference_large117,3242
fn para_filter_and_reduce_edge_cases() {para_filter_and_reduce_edge_cases133,3845

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test17AVLTreeSeqStPer19.rs,371
pub mod TestAVLTreeSeqStPer {TestAVLTreeSeqStPer1,0
    fn test_tabulate_and_map_ch19() {test_tabulate_and_map_ch199,297
    fn test_select_and_append_ch19() {test_select_and_append_ch1917,661
    fn test_deflate_and_filter_ch19() {test_deflate_and_filter_ch1938,1529
    fn test_iter_inorder_after_pipeline_ch19() {test_iter_inorder_after_pipeline_ch1956,2259

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap37/Test52BSTParaStEph.rs,322
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,98
fn para_basic_insert_find() {para_basic_insert_find14,259
fn para_split_and_join_pair() {para_split_and_join_pair24,573
fn para_union_and_delete() {para_union_and_delete36,980
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip50,1399

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36Mt.rs,131
fn gen_data(n: usize) -> ArraySeqMtEphS<i32> {gen_data7,206
fn bench_quicksort_mt(c: &mut Criterion) {bench_quicksort_mt17,541

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph.rs,86
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch199,311

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch188,262

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch198,262

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer18.rs,90
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch187,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStEph18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map9,311

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqMtPer.rs,90
fn bench_tabulate_map_mtper_ch19(c: &mut Criterion) {bench_tabulate_map_mtper_ch197,233

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchDirGraphStEph.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build8,283

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabUnDirGraphStEph.rs,980
fn bench_labelled_undir_graph_creation(c: &mut Criterion) {bench_labelled_undir_graph_creation7,282
fn bench_labelled_undir_graph_add_vertex(c: &mut Criterion) {bench_labelled_undir_graph_add_vertex34,1226
fn bench_labelled_undir_graph_add_labeled_edge(c: &mut Criterion) {bench_labelled_undir_graph_add_labeled_edge52,1770
fn bench_labelled_undir_graph_has_edge(c: &mut Criterion) {bench_labelled_undir_graph_has_edge70,2384
fn bench_labelled_undir_graph_get_edge_label(c: &mut Criterion) {bench_labelled_undir_graph_get_edge_label94,3148
fn bench_labelled_undir_graph_neighbors(c: &mut Criterion) {bench_labelled_undir_graph_neighbors120,4033
fn bench_labelled_undir_graph_edges(c: &mut Criterion) {bench_labelled_undir_graph_edges150,5111
fn bench_labelled_undir_graph_macro(c: &mut Criterion) {bench_labelled_undir_graph_macro168,5659
fn bench_labelled_undir_graph_edge_normalization(c: &mut Criterion) {bench_labelled_undir_graph_edge_normalization190,6263

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabDirGraphStEph.rs,932
fn bench_labelled_dir_graph_creation(c: &mut Criterion) {bench_labelled_dir_graph_creation7,276
fn bench_labelled_dir_graph_add_vertex(c: &mut Criterion) {bench_labelled_dir_graph_add_vertex34,1208
fn bench_labelled_dir_graph_add_labeled_arc(c: &mut Criterion) {bench_labelled_dir_graph_add_labeled_arc52,1746
fn bench_labelled_dir_graph_has_arc(c: &mut Criterion) {bench_labelled_dir_graph_has_arc70,2350
fn bench_labelled_dir_graph_get_arc_label(c: &mut Criterion) {bench_labelled_dir_graph_get_arc_label94,3102
fn bench_labelled_dir_graph_out_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_out_neighbors120,3975
fn bench_labelled_dir_graph_in_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_in_neighbors147,4903
fn bench_labelled_dir_graph_arcs(c: &mut Criterion) {bench_labelled_dir_graph_arcs174,5827
fn bench_labelled_dir_graph_macro(c: &mut Criterion) {bench_labelled_dir_graph_macro192,6363

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchUnDirGraphStEph.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build8,287

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPer.rs,80
fn bench_sll_persistent_ops(c: &mut Criterion) {bench_sll_persistent_ops9,263

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTTreapStEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree7,215
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap15,412

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTTreapMtEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree7,215
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap15,410

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap03/BenchInsertionSortSt.rs,128
fn build_vec(len: usize) -> Vec<i32> {build_vec6,207
fn bench_insertion_sort(c: &mut Criterion) {bench_insertion_sort10,285

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36St.rs,131
fn gen_data(n: usize) -> ArraySeqStEphS<i32> {gen_data7,206
fn bench_quicksort_st(c: &mut Criterion) {bench_quicksort_st17,538

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter26Mt.rs,138
fn gen_sequence(n: usize) -> ArrayMtPerS<usize> {gen_sequence7,217
fn bench_chapter26_mt(c: &mut Criterion) {bench_chapter26_mt11,297

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchMappingStEph.rs,70
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build9,362

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchRelationStEph.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range8,297

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchFibonacciMt.rs,68
fn bench_fibonacci_mt(c: &mut Criterion) {bench_fibonacci_mt6,192

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch197,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchMathSeq.rs,72
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics9,239

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer.rs,66
fn bench_ll_per_ch18(c: &mut Criterion) {bench_ll_per_ch187,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch188,274

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSetMtEph.rs,3716
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
    fn empty() -> Self {empty89,2979
    fn insert_value(&mut self, value: i32) {insert_value93,3042
    fn union_with(&self, other: &Self) -> Self {union_with97,3122
    fn difference_with(&self, other: &Self) -> Self {difference_with101,3204
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by105,3296
    fn reduce_sum(&self) -> i32 {reduce_sum109,3412
impl BenchSet for AVLSet<i32> {AVLSet114,3504
    fn empty() -> Self {empty115,3536
    fn insert_value(&mut self, value: i32) {insert_value119,3597
    fn union_with(&self, other: &Self) -> Self {union_with123,3677
    fn difference_with(&self, other: &Self) -> Self {difference_with127,3759
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by131,3851
    fn reduce_sum(&self) -> i32 {reduce_sum135,3967
impl BenchSet for RBSet<i32> {RBSet140,4059
    fn empty() -> Self {empty141,4090
    fn insert_value(&mut self, value: i32) {insert_value145,4150
    fn union_with(&self, other: &Self) -> Self {union_with149,4230
    fn difference_with(&self, other: &Self) -> Self {difference_with153,4312
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by157,4404
    fn reduce_sum(&self) -> i32 {reduce_sum161,4520
impl BenchSet for BBAlphaSet<i32> {BBAlphaSet166,4612
    fn empty() -> Self {empty167,4648
    fn insert_value(&mut self, value: i32) {insert_value171,4713
    fn union_with(&self, other: &Self) -> Self {union_with175,4793
    fn difference_with(&self, other: &Self) -> Self {difference_with179,4875
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by183,4967
    fn reduce_sum(&self) -> i32 {reduce_sum187,5083
impl BenchSet for TreapSet<i32> {TreapSet192,5175
    fn empty() -> Self {empty193,5209
    fn insert_value(&mut self, value: i32) {insert_value197,5272
    fn union_with(&self, other: &Self) -> Self {union_with201,5352
    fn difference_with(&self, other: &Self) -> Self {difference_with205,5434
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by209,5526
    fn reduce_sum(&self) -> i32 {reduce_sum213,5642
impl BenchSet for SplaySet<i32> {SplaySet218,5734
    fn empty() -> Self {empty219,5768
    fn insert_value(&mut self, value: i32) {insert_value223,5831
    fn union_with(&self, other: &Self) -> Self {union_with227,5911
    fn difference_with(&self, other: &Self) -> Self {difference_with231,5993
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by235,6085
    fn reduce_sum(&self) -> i32 {reduce_sum239,6201
fn bench_plain_set(c: &mut Criterion) {bench_plain_set244,6293
fn bench_avl_set(c: &mut Criterion) {bench_avl_set248,6400
fn bench_rb_set(c: &mut Criterion) {bench_rb_set252,6501
fn bench_bbalpha_set(c: &mut Criterion) {bench_bbalpha_set256,6599
fn bench_treap_set(c: &mut Criterion) {bench_treap_set260,6712
fn bench_splay_set(c: &mut Criterion) {bench_splay_set264,6819

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap03/BenchInsertionSortSt.rs,128
fn build_vec(len: usize) -> Vec<i32> {build_vec6,207
fn bench_insertion_sort(c: &mut Criterion) {bench_insertion_sort10,285

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchMappingStEph.rs,70
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build9,362

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap05/BenchRelationStEph.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range8,297

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchDirGraphStEph.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build8,283

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabUnDirGraphStEph.rs,980
fn bench_labelled_undir_graph_creation(c: &mut Criterion) {bench_labelled_undir_graph_creation7,282
fn bench_labelled_undir_graph_add_vertex(c: &mut Criterion) {bench_labelled_undir_graph_add_vertex34,1226
fn bench_labelled_undir_graph_add_labeled_edge(c: &mut Criterion) {bench_labelled_undir_graph_add_labeled_edge52,1770
fn bench_labelled_undir_graph_has_edge(c: &mut Criterion) {bench_labelled_undir_graph_has_edge70,2384
fn bench_labelled_undir_graph_get_edge_label(c: &mut Criterion) {bench_labelled_undir_graph_get_edge_label94,3148
fn bench_labelled_undir_graph_neighbors(c: &mut Criterion) {bench_labelled_undir_graph_neighbors120,4033
fn bench_labelled_undir_graph_edges(c: &mut Criterion) {bench_labelled_undir_graph_edges150,5111
fn bench_labelled_undir_graph_macro(c: &mut Criterion) {bench_labelled_undir_graph_macro168,5659
fn bench_labelled_undir_graph_edge_normalization(c: &mut Criterion) {bench_labelled_undir_graph_edge_normalization190,6263

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchLabDirGraphStEph.rs,932
fn bench_labelled_dir_graph_creation(c: &mut Criterion) {bench_labelled_dir_graph_creation7,276
fn bench_labelled_dir_graph_add_vertex(c: &mut Criterion) {bench_labelled_dir_graph_add_vertex34,1208
fn bench_labelled_dir_graph_add_labeled_arc(c: &mut Criterion) {bench_labelled_dir_graph_add_labeled_arc52,1746
fn bench_labelled_dir_graph_has_arc(c: &mut Criterion) {bench_labelled_dir_graph_has_arc70,2350
fn bench_labelled_dir_graph_get_arc_label(c: &mut Criterion) {bench_labelled_dir_graph_get_arc_label94,3102
fn bench_labelled_dir_graph_out_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_out_neighbors120,3975
fn bench_labelled_dir_graph_in_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_in_neighbors147,4903
fn bench_labelled_dir_graph_arcs(c: &mut Criterion) {bench_labelled_dir_graph_arcs174,5827
fn bench_labelled_dir_graph_macro(c: &mut Criterion) {bench_labelled_dir_graph_macro192,6363

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap06/BenchUnDirGraphStEph.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build8,287

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStEph19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch197,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchMathSeq.rs,72
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics9,239

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchLinkedListStPer.rs,66
fn bench_ll_per_ch18(c: &mut Criterion) {bench_ll_per_ch187,226

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap18/BenchArraySeqStPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch188,274

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
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch188,262

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap19/BenchArraySeqStPer.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch198,262

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap37/BenchBSTSetMtEph.rs,3716
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
    fn empty() -> Self {empty89,2979
    fn insert_value(&mut self, value: i32) {insert_value93,3042
    fn union_with(&self, other: &Self) -> Self {union_with97,3122
    fn difference_with(&self, other: &Self) -> Self {difference_with101,3204
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by105,3296
    fn reduce_sum(&self) -> i32 {reduce_sum109,3412
impl BenchSet for AVLSet<i32> {AVLSet114,3504
    fn empty() -> Self {empty115,3536
    fn insert_value(&mut self, value: i32) {insert_value119,3597
    fn union_with(&self, other: &Self) -> Self {union_with123,3677
    fn difference_with(&self, other: &Self) -> Self {difference_with127,3759
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by131,3851
    fn reduce_sum(&self) -> i32 {reduce_sum135,3967
impl BenchSet for RBSet<i32> {RBSet140,4059
    fn empty() -> Self {empty141,4090
    fn insert_value(&mut self, value: i32) {insert_value145,4150
    fn union_with(&self, other: &Self) -> Self {union_with149,4230
    fn difference_with(&self, other: &Self) -> Self {difference_with153,4312
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by157,4404
    fn reduce_sum(&self) -> i32 {reduce_sum161,4520
impl BenchSet for BBAlphaSet<i32> {BBAlphaSet166,4612
    fn empty() -> Self {empty167,4648
    fn insert_value(&mut self, value: i32) {insert_value171,4713
    fn union_with(&self, other: &Self) -> Self {union_with175,4793
    fn difference_with(&self, other: &Self) -> Self {difference_with179,4875
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by183,4967
    fn reduce_sum(&self) -> i32 {reduce_sum187,5083
impl BenchSet for TreapSet<i32> {TreapSet192,5175
    fn empty() -> Self {empty193,5209
    fn insert_value(&mut self, value: i32) {insert_value197,5272
    fn union_with(&self, other: &Self) -> Self {union_with201,5352
    fn difference_with(&self, other: &Self) -> Self {difference_with205,5434
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by209,5526
    fn reduce_sum(&self) -> i32 {reduce_sum213,5642
impl BenchSet for SplaySet<i32> {SplaySet218,5734
    fn empty() -> Self {empty219,5768
    fn insert_value(&mut self, value: i32) {insert_value223,5831
    fn union_with(&self, other: &Self) -> Self {union_with227,5911
    fn difference_with(&self, other: &Self) -> Self {difference_with231,5993
    fn filter_divisible_by(&self, divisor: i32) -> Self {filter_divisible_by235,6085
    fn reduce_sum(&self) -> i32 {reduce_sum239,6201
fn bench_plain_set(c: &mut Criterion) {bench_plain_set244,6293
fn bench_avl_set(c: &mut Criterion) {bench_avl_set248,6400
fn bench_rb_set(c: &mut Criterion) {bench_rb_set252,6501
fn bench_bbalpha_set(c: &mut Criterion) {bench_bbalpha_set256,6599
fn bench_treap_set(c: &mut Criterion) {bench_treap_set260,6712
fn bench_splay_set(c: &mut Criterion) {bench_splay_set264,6819

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
