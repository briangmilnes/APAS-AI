
/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36MtSlice.rs,1381
pub mod Chapter36MtSlice {Chapter36MtSlice3,98
    pub trait Chapter36MtSliceTrait<T: StT + Ord + Send> {Chapter36MtSliceTrait13,298
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first14,357
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median315,410
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random16,465
        fn quick_sort_mt_first(&self);quick_sort_mt_first18,520
        fn quick_sort_mt_median3(&self);quick_sort_mt_median319,559
        fn quick_sort_mt_random(&self);quick_sort_mt_random20,600
    impl<T: StT + Ord + Send> Chapter36MtSliceTrait<T> for ArraySeqMtEphSliceS<T> {ArraySeqMtEphSliceS23,647
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first24,731
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median326,809
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random40,1280
        fn quick_sort_mt_first(&self) {quick_sort_mt_first46,1456
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort51,1611
        fn quick_sort_mt_median3(&self) {quick_sort_mt_median388,2974
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort93,3131
        fn quick_sort_mt_random(&self) {quick_sort_mt_random143,5059
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {sort148,5215

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap6/UnDirGraphStEph.rs,2933
pub mod UnDirGraphStEph {UnDirGraphStEph3,80
    pub struct UnDirGraphStEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {UnDirGraphStEph10,267
        V: Set<V>,V11,360
        E: Set<Edge<V>>,E12,379
    pub trait UnDirGraphStEphTrait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {UnDirGraphStEphTrait15,411
        fn empty() -> UnDirGraphStEph<V>;empty18,600
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V>;FromSets21,750
        fn vertices(&self) -> &Set<V>;vertices24,913
        fn edges(&self) -> &Set<Edge<V>>;edges27,1044
        fn sizeV(&self) -> N;sizeV30,1178
        fn sizeE(&self) -> N;sizeE33,1300
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor36,1422
        fn NG(&self, v: &V) -> Set<V>;NG39,1565
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices42,1722
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;Incident45,1872
        fn Degree(&self, v: &V) -> N;Degree48,2021
    impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> UnDirGraphStEphTrait<V> forUnDirGraphStEph51,2066
        fn empty() -> UnDirGraphStEph<V> {empty52,2184
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E }FromSets58,2341
        fn vertices(&self) -> &Set<V> { &self.V }vertices59,2440
        fn edges(&self) -> &Set<Edge<V>> { &self.E }edges60,2490
        fn sizeV(&self) -> N { self.V.size() }sizeV61,2543
        fn sizeE(&self) -> N { self.E.size() }sizeE62,2590
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor64,2638
        fn NG(&self, v: &V) -> Set<V> {NG74,2962
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices86,3324
        fn Incident(&self, e: &Edge<V>, v: &V) -> B {Incident95,3596
        fn Degree(&self, v: &V) -> N { self.NG(v).size() }Degree103,3785
    impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Debug for UnDirGrUnDirGraphStEph106,3851
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt107,3961
    impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for UnDirUnDirGraphStEph115,4201
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} E={fmt116,4313
    impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for UnDirGraphStEUnDirGraphStEph119,4440
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }eq120,4544
    impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for UnDirGraphStEph<V> {UnDirGraphStEph122,4636
    macro_rules! UnDirGraphStEphLit {UnDirGraphStEphLit125,4755
    fn _UnDirGraphStEphLit_type_checks() {_UnDirGraphStEphLit_type_checks143,5941
    pub fn __undirgraph_macro_typecheck_exercise() {__undirgraph_macro_typecheck_exercise149,6204

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap6/LabDirGraphStEph.rs,2553
pub mod LabDirGraphStEph {LabDirGraphStEph3,91
    pub struct LabDirGraphStEph<V, L>LabDirGraphStEph11,334
        vertices: Set<V>,vertices16,459
        labeled_arcs: Set<LabEdge<V, L>>,labeled_arcs17,485
    pub trait LabDirGraphStEphTrait<V, L>LabDirGraphStEphTrait20,534
        fn empty() -> Self;empty25,663
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs26,691
        fn vertices(&self) -> &Set<V>;vertices27,794
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;labeled_arcs28,833
        fn arcs(&self) -> Set<Edge<V>>;arcs29,888
        fn add_vertex(&mut self, v: V);add_vertex30,928
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);add_labeled_arc31,968
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;get_arc_label32,1033
        fn has_arc(&self, from: &V, to: &V) -> bool;has_arc33,1098
        fn out_neighbors(&self, v: &V) -> Set<V>;out_neighbors34,1151
        fn in_neighbors(&self, v: &V) -> Set<V>;in_neighbors35,1201
    impl<V, L> LabDirGraphStEphTrait<V, L> for LabDirGraphStEph<V, L>LabDirGraphStEph38,1257
        fn empty() -> Self {empty43,1414
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) ->from_vertices_and_labeled_arcs50,1583
        fn vertices(&self) -> &Set<V> {vertices54,1754
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> {labeled_arcs58,1832
        fn arcs(&self) -> Set<Edge<V>> {arcs62,1930
        fn add_vertex(&mut self, v: V) {add_vertex70,2193
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {add_labeled_arc74,2282
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {get_arc_label80,2517
        fn has_arc(&self, from: &V, to: &V) -> bool {has_arc89,2818
        fn out_neighbors(&self, v: &V) -> Set<V> {out_neighbors98,3092
        fn in_neighbors(&self, v: &V) -> Set<V> {in_neighbors108,3414
    impl<V, L> Display for LabDirGraphStEph<V, L>LabDirGraphStEph119,3741
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt124,3878
    impl<V, L> Debug for LabDirGraphStEph<V, L>LabDirGraphStEph129,4037
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt134,4172
    macro_rules! LabDirGraphStEphLit {LabDirGraphStEphLit141,4398
    fn _LabDirGraphStEphLit_type_checks() {_LabDirGraphStEphLit_type_checks153,5186
    pub fn __lab_dir_graph_macro_typecheck_exercise() {__lab_dir_graph_macro_typecheck_exercise159,5457

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap6/DirGraphStEph.rs,3775
pub mod DirGraphStEph {DirGraphStEph3,77
    pub struct DirGraphStEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {DirGraphStEph10,262
        V: Set<V>,V11,353
        A: Set<Edge<V>>,A12,372
    pub trait DirGraphStEphTrait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {DirGraphStEphTrait15,404
        fn empty() -> DirGraphStEph<V>;empty18,591
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V>;FromSets21,739
        fn vertices(&self) -> &Set<V>;vertices24,900
        fn arcs(&self) -> &Set<Edge<V>>;arcs27,1031
        fn sizeV(&self) -> N;sizeV30,1164
        fn sizeA(&self) -> N;sizeA33,1286
        fn Neighbor(&self, u: &V, v: &V) -> B;Neighbor36,1408
        fn NG(&self, v: &V) -> Set<V>;NG39,1551
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;NGOfVertices42,1708
        fn NPlus(&self, v: &V) -> Set<V>;NPlus45,1862
        fn NMinus(&self, v: &V) -> Set<V>;NMinus48,2000
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NPlusOfVertices51,2161
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;NMinusOfVertices54,2340
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;Incident57,2494
        fn Degree(&self, v: &V) -> N;Degree60,2646
        fn InDegree(&self, v: &V) -> N;InDegree63,2780
        fn OutDegree(&self, v: &V) -> N;OutDegree66,2916
    impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> DirGraphStEphTrait<V> for DDirGraphStEph69,2964
        fn empty() -> DirGraphStEph<V> {empty70,3078
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }FromSets76,3231
        fn vertices(&self) -> &Set<V> { &self.V }vertices77,3326
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }arcs78,3376
        fn sizeV(&self) -> N { self.V.size() }sizeV79,3428
        fn sizeA(&self) -> N { self.A.size() }sizeA80,3475
        fn Neighbor(&self, u: &V, v: &V) -> B {Neighbor82,3523
        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }NG92,3800
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {NGOfVertices94,3857
        fn NPlus(&self, v: &V) -> Set<V> {NPlus103,4129
        fn NMinus(&self, v: &V) -> Set<V> {NMinus113,4411
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NPlusOfVertices123,4694
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {NMinusOfVertices132,4976
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B {Incident141,5262
        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }Degree149,5454
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }InDegree150,5516
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }OutDegree151,5581
    impl<V: Eq + Hash + Clone + std::fmt::Debug + std::fmt::Display> std::fmt::Debug for DirGrapDirGraphStEph154,5653
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt155,5761
    impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for DirGrDirGraphStEph163,5999
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} A={fmt164,6109
    impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for DirGraphStEphDirGraphStEph167,6236
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }eq168,6338
    impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for DirGraphStEph<V> {}DirGraphStEph170,6430
    macro_rules! DirGraphStEphLit {DirGraphStEphLit173,6547
    fn _DirGraphStEphLit_type_checks() {_DirGraphStEphLit_type_checks190,7701
    pub fn __dirgraph_macro_typecheck_exercise() {__dirgraph_macro_typecheck_exercise196,7956

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap6/LabUnDirGraphStEph.rs,2612
pub mod LabUnDirGraphStEph {LabUnDirGraphStEph3,94
    pub struct LabUnDirGraphStEph<V, L>LabUnDirGraphStEph11,339
        vertices: Set<V>,vertices16,472
        labeled_edges: Set<LabEdge<V, L>>,labeled_edges17,498
    pub trait LabUnDirGraphStEphTrait<V, L>LabUnDirGraphStEphTrait20,548
        fn empty() -> Self;empty25,685
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges26,713
        fn vertices(&self) -> &Set<V>;vertices27,818
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;labeled_edges28,857
        fn edges(&self) -> Set<Edge<V>>;edges29,913
        fn add_vertex(&mut self, v: V);add_vertex30,954
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);add_labeled_edge31,994
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;get_edge_label32,1058
        fn has_edge(&self, v1: &V, v2: &V) -> bool;has_edge33,1122
        fn neighbors(&self, v: &V) -> Set<V>;neighbors34,1174
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;normalize_edge35,1220
    impl<V, L> LabUnDirGraphStEphTrait<V, L> for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph38,1285
        fn empty() -> Self {empty43,1452
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) from_vertices_and_labeled_edges50,1624
        fn vertices(&self) -> &Set<V> {vertices54,1800
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> {labeled_edges58,1878
        fn edges(&self) -> Set<Edge<V>> {edges62,1978
        fn add_vertex(&mut self, v: V) {add_vertex70,2249
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {add_labeled_edge74,2338
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {get_edge_label85,2724
        fn has_edge(&self, v1: &V, v2: &V) -> bool {has_edge96,3165
        fn neighbors(&self, v: &V) -> Set<V> {neighbors107,3578
        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {normalize_edge119,4011
    impl<V, L> Display for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph127,4431
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt132,4576
    impl<V, L> Debug for LabUnDirGraphStEph<V, L>LabUnDirGraphStEph137,4738
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt142,4881
    macro_rules! LabUnDirGraphStEphLit {LabUnDirGraphStEphLit149,5111
    fn _LabUnDirGraphStEphLit_type_checks() {_LabUnDirGraphStEphLit_type_checks172,6326
    pub fn __lab_undir_graph_macro_typecheck_exercise() {__lab_undir_graph_macro_typecheck_exercise178,6605

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtPerChap19.rs,3793
pub mod ArraySeqMtPerChap19 {ArraySeqMtPerChap193,129
    pub trait ArraySeqMtPerChap19Trait<T: MtT> {ArraySeqMtPerChap19Trait11,363
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;tabulate15,625
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;map18,862
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;append21,1057
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;filter24,1340
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;update27,1521
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;ninject30,1723
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate32,1893
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes34,2022
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce36,2215
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan38,2339
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten40,2502
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect42,2653
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;inject45,2811
        fn atomicWrite(atomicWrite47,2949
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arrayinject_parallel253,3192
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins54,3299
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arraninject_parallel260,3567
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins61,3675
    impl<T: MtT + StT + Send + Sync> ArraySeqMtPerChap19Trait<T> for ArrayMtPerS<T> {ArrayMtPerS68,3888
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate69,3974
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map73,4127
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append77,4293
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter81,4456
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T> {update85,4626
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject89,4796
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate93,4982
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes97,5156
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce101,5364
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan105,5531
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten109,5712
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect120,6114
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> inject154,7439
        fn atomicWrite(atomicWrite158,7633
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arrayinject_parallel2166,7926
        fn AtomicWriteLowestChangeNumberWins(AtomicWriteLowestChangeNumberWins171,8206
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> Arraninject_parallel2192,8976
        fn AtomicWriteHighestChangeNumberWins(AtomicWriteHighestChangeNumberWins197,9259

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTSetAVLMtEph.rs,5580
pub mod BSTSetAVLMtEph {BSTSetAVLMtEph3,73
    pub struct BSTSetAVLMtEph<T: StTinMtT + Ord> {BSTSetAVLMtEph10,280
        tree: BSTAVLMtEph<T>,tree11,331
    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;BSTSetAVLMt14,368
    pub trait BSTSetAVLMtEphTrait<T: StTinMtT + Ord>: Sized {BSTSetAVLMtEphTrait16,418
        fn empty() -> Self;empty17,480
        fn singleton(value: T) -> Self;singleton18,508
        fn size(&self) -> N;size19,548
        fn is_empty(&self) -> B;is_empty20,577
        fn find(&self, value: &T) -> Option<T>;find21,610
        fn contains(&self, value: &T) -> B;contains22,658
        fn minimum(&self) -> Option<T>;minimum23,702
        fn maximum(&self) -> Option<T>;maximum24,742
        fn insert(&mut self, value: T);insert25,782
        fn delete(&mut self, target: &T);delete26,822
        fn union(&self, other: &Self) -> Self;union27,864
        fn intersection(&self, other: &Self) -> Self;intersection28,911
        fn difference(&self, other: &Self) -> Self;difference29,965
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1017
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1072
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1127
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1189
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1259
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1327
        fn as_tree(&self) -> &BSTAVLMtEph<T>;as_tree36,1378
    impl<T: StTinMtT + Ord> BSTSetAVLMtEph<T> {BSTSetAVLMtEph39,1431
        pub fn empty() -> Self {empty40,1479
        pub fn singleton(value: T) -> Self {singleton46,1598
        pub fn size(&self) -> N { self.tree.size() }size52,1755
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1809
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1871
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1949
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2027
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2095
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2163
        pub fn delete(&mut self, target: &T) {delete66,2236
        pub fn union(&self, other: &Self) -> Self {union74,2525
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2824
        pub fn difference(&self, other: &Self) -> Self {difference99,3402
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,3979
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4670
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,4983
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5339
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5748
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6012
        pub fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree178,6092
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6157
        fn rebuild_from_vec(values: Vec<T>) -> BSTAVLMtEph<T> {rebuild_from_vec182,6248
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6467
    impl<T: StTinMtT + Ord> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {BSTSetAVLMtEph202,6750
        fn empty() -> Self { Self::empty() }empty203,6825
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6871
        fn size(&self) -> N { self.tree.size() }size207,6938
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,6988
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7046
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7120
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7194
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7258
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7322
        fn delete(&mut self, target: &T) { BSTSetAVLMtEph::delete(self, target) }delete221,7391
        fn union(&self, other: &Self) -> Self { BSTSetAVLMtEph::union(self, other) }union223,7474
        fn intersection(&self, other: &Self) -> Self { BSTSetAVLMtEph::intersection(self, other)intersection225,7560
        fn difference(&self, other: &Self) -> Self { BSTSetAVLMtEph::difference(self, other) }difference227,7660
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetAVLMtEph::split(self, pivot) }split229,7756
        fn join_pair(left: Self, right: Self) -> Self { BSTSetAVLMtEph::join_pair(left, right) }join_pair231,7850
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetAVLMtEph::join_m(left, pivojoin_m233,7948
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetAVLMtEph::filter(sefilter235,8057
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetAVLMtEph::reduce(selfreduce237,8171
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8282
        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }as_tree241,8358
    macro_rules! BSTSetAVLMtEphLit {BSTSetAVLMtEphLit245,8445
    fn _BSTSetAVLMtEphLit_type_checks() {_BSTSetAVLMtEphLit_type_checks257,8960

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPerChap19.rs,2725
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
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter78,3791
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate87,4214
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce99,4695
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan114,5348
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten157,7082
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject161,7237
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject165,7421

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStEphChap18.rs,2838
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
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes30,1644
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce31,1760
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan32,1840
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten33,1939
        fn collect<A: StT, Bv: StT>(collect34,2020
    impl<T: StT> ArraySeqStEphChap18Trait<T> for ArraySeqStEphS<T> {ArraySeqStEphS40,2206
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate41,2275
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map48,2521
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append61,3055
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter78,3753
        fn update(a: &mut ArraySeqStEphS<T>, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update90,4178
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphSinject93,4320
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphninject101,4646
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,4796
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArriteratePrefixes111,5031
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce124,5590
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan131,5819
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten145,6372
        fn collect<A: StT, Bv: StT>(collect173,7458

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStPer.rs,4388
pub mod AVLTreeSeqStPer {AVLTreeSeqStPer3,85
    type Link<T> = Option<Rc<Node<T>>>;Link9,238
    struct Node<T: StT> {Node11,279
        value: T,value12,305
        height: N,height13,323
        size: N,size14,342
        left: Link<T>,left15,359
        right: Link<T>,right16,382
    fn height<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.height) }height19,413
    fn size<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.size) }size20,492
    fn mk<T: StT>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {mk22,568
    fn rotate_right<T: StT>(y: Rc<Node<T>>) -> Rc<Node<T>> {rotate_right35,913
    fn rotate_left<T: StT>(x: Rc<Node<T>>) -> Rc<Node<T>> {rotate_left42,1220
    fn rebalance<T: StT>(n: Rc<Node<T>>) -> Rc<Node<T>> {rebalance49,1526
    fn nth_ref<'a, T: StT>(mut cur: &'a Link<T>, mut index: N) -> &'a T {nth_ref71,2379
    fn set_rec<T: StT>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> {set_rec86,2828
    fn inorder_collect<T: StT>(cur: &Link<T>, out: &mut Vec<T>) {inorder_collect104,3600
    fn build_balanced_from_slice<T: StT>(a: &[T]) -> Link<T> {build_balanced_from_slice112,3840
        fn rec<T: StT>(a: &[T]) -> Link<T> {rec113,3903
    pub struct AVLTreeSeqStPerS<T: StT> {AVLTreeSeqStPerS125,4221
        root: Link<T>,root126,4263
    pub trait AVLTreeSeqStPerTrait<T: StT> {AVLTreeSeqStPerTrait129,4293
        fn empty() -> Self;empty132,4430
        fn new() -> Self;new135,4550
        fn length(&self) -> N;length138,4668
        fn nth(&self, index: N) -> &T;nth141,4807
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set144,5013
        fn singleton(item: T) -> Self;singleton149,5215
        fn isEmpty(&self) -> B;isEmpty152,5346
        fn isSingleton(&self) -> B;isSingleton155,5470
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy158,5622
        fn from_vec(values: Vec<T>) -> Self;from_vec160,5745
        fn values_in_order(&self) -> Vec<T>;values_in_order162,5834
    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS165,5886
        fn empty() -> Self { AVLTreeSeqStPerS { root: None } }empty166,5953
        fn new() -> Self { Self::empty() }new167,6016
        fn length(&self) -> N { size(&self.root) }length168,6059
        fn nth(&self, index: N) -> &T { nth_ref(&self.root, index) }nth169,6110
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set170,6179
        fn singleton(item: T) -> Self {singleton175,6368
        fn isEmpty(&self) -> B {isEmpty180,6513
        fn isSingleton(&self) -> B {isSingleton187,6676
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy194,6843
        fn from_vec(values: Vec<T>) -> Self {from_vec208,7351
        fn values_in_order(&self) -> Vec<T> {values_in_order213,7514
    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS220,7705
        fn eq(&self, other: &Self) -> bool {eq221,7758
    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}AVLTreeSeqStPerS233,8084
    impl<T: StT> std::fmt::Debug for AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS235,8132
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt236,8191
    impl<T: StT> AVLTreeSeqStPerS<T> {AVLTreeSeqStPerS242,8383
        pub fn to_arrayseq(&self) -> ArrayStPerS<T> {to_arrayseq243,8422
        pub fn iter<'a>(&'a self) -> AVLTreeSeqStPerIter<'a, T> {iter248,8568
    pub struct AVLTreeSeqStPerIter<'a, T: StT> {AVLTreeSeqStPerIter256,8781
        stack: Vec<&'a Node<T>>,stack257,8830
        current: Option<&'a Node<T>>,current258,8863
    impl<'a, T: StT> AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter261,8908
        fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {push_left262,8958
    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {AVLTreeSeqStPerIter270,9168
        type Item = &'a T;Item271,9231
        fn next(&mut self) -> Option<Self::Item> {next272,9258
    macro_rules! AVLTreeSeqStPerIterLit {AVLTreeSeqStPerIterLit285,9650
    fn _AVLTreeSeqStPer_struct_macro_checks() {_AVLTreeSeqStPer_struct_macro_checks295,9952
    macro_rules! AVLTreeSeqStPerSLit {AVLTreeSeqStPerSLit303,10328
    fn _AVLTreeSeqStPerSLit_type_checks() {_AVLTreeSeqStPerSLit_type_checks316,11062

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTSetPlainMtEph.rs,5266
pub mod BSTSetPlainMtEph {BSTSetPlainMtEph3,75
    pub struct BSTSetPlainMtEph<T: StTinMtT + Ord> {BSTSetPlainMtEph10,290
        tree: BSTPlainMtEph<T>,tree11,343
    pub type BSTSetPlainMt<T> = BSTSetPlainMtEph<T>;BSTSetPlainMt14,382
    pub trait BSTSetPlainMtEphTrait<T: StTinMtT + Ord>: Sized {BSTSetPlainMtEphTrait16,436
        fn empty() -> Self;empty17,500
        fn singleton(value: T) -> Self;singleton18,528
        fn size(&self) -> N;size19,568
        fn is_empty(&self) -> B;is_empty20,597
        fn find(&self, value: &T) -> Option<T>;find21,630
        fn contains(&self, value: &T) -> B;contains22,678
        fn minimum(&self) -> Option<T>;minimum23,722
        fn maximum(&self) -> Option<T>;maximum24,762
        fn insert(&mut self, value: T);insert25,802
        fn delete(&mut self, target: &T);delete26,842
        fn union(&self, other: &Self) -> Self;union27,884
        fn intersection(&self, other: &Self) -> Self;intersection28,931
        fn difference(&self, other: &Self) -> Self;difference29,985
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1037
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1092
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1147
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1209
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1279
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1347
        fn as_tree(&self) -> &BSTPlainMtEph<T>;as_tree36,1398
    impl<T: StTinMtT + Ord> BSTSetPlainMtEph<T> {BSTSetPlainMtEph39,1453
        pub fn empty() -> Self {empty40,1503
        pub fn singleton(value: T) -> Self {singleton46,1624
        pub fn size(&self) -> N { self.tree.size() }size52,1783
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1837
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1899
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1977
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2055
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2123
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2191
        pub fn delete(&mut self, target: &T) {delete66,2264
        pub fn union(&self, other: &Self) -> Self {union74,2553
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2852
        pub fn difference(&self, other: &Self) -> Self {difference99,3430
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4007
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4698
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5011
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5367
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5776
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6040
        pub fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree178,6120
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6187
        fn rebuild_from_vec(values: Vec<T>) -> BSTPlainMtEph<T> {rebuild_from_vec182,6278
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6501
    impl<T: StTinMtT + Ord> BSTSetPlainMtEphTrait<T> for BSTSetPlainMtEph<T> {BSTSetPlainMtEph202,6786
        fn empty() -> Self { Self::empty() }empty203,6865
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6911
        fn size(&self) -> N { self.tree.size() }size207,6978
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7028
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7086
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7160
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7234
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7298
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7362
        fn delete(&mut self, target: &T) {delete221,7431
        fn union(&self, other: &Self) -> Self {union229,7716
        fn intersection(&self, other: &Self) -> Self {intersection237,8011
        fn difference(&self, other: &Self) -> Self {difference254,8585
        fn split(&self, pivot: &T) -> (Self, B, Self) {split271,9158
        fn join_pair(left: Self, right: Self) -> Self {join_pair291,9845
        fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m299,10154
        fn filter<F>(&self, predicate: F) -> Selffilter308,10506
        fn reduce<F>(&self, op: F, base: T) -> Treduce315,10667
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order322,10825
        fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }as_tree324,10901
    macro_rules! BSTSetPlainMtEphLit {BSTSetPlainMtEphLit328,10990
    fn _BSTSetPlainMtEphLit_type_checks() {_BSTSetPlainMtEphLit_type_checks340,11531

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/MathSeq.rs,4286
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
    fn _MathSeqSLit_type_checks() {_MathSeqSLit_type_checks297,10408

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTRBStEph.rs,4614
pub mod BSTRBStEph {BSTRBStEph3,93
    enum Color {Color9,297
        Red,Red10,314
        Black,Black11,327
    type Link<T> = Option<Box<Node<T>>>;Link14,349
    struct Node<T: StT + Ord> {Node17,412
        key: T,key18,444
        color: Color,color19,460
        size: N,size20,482
        left: Link<T>,left21,499
        right: Link<T>,right22,522
    impl<T: StT + Ord> Node<T> {Node25,553
        fn new(key: T) -> Self {new26,586
    pub struct BSTRBStEph<T: StT + Ord> {BSTRBStEph37,807
        root: Link<T>,root38,849
    pub type BSTreeRB<T> = BSTRBStEph<T>;BSTreeRB41,879
    pub trait BSTRBStEphTrait<T: StT + Ord> {BSTRBStEphTrait43,922
        fn new() -> Self;new44,968
        fn size(&self) -> N;size45,994
        fn is_empty(&self) -> B;is_empty46,1023
        fn height(&self) -> N;height47,1056
        fn insert(&mut self, value: T);insert48,1087
        fn find(&self, target: &T) -> Option<&T>;find49,1127
        fn contains(&self, target: &T) -> B;contains50,1177
        fn minimum(&self) -> Option<&T>;minimum51,1222
        fn maximum(&self) -> Option<&T>;maximum52,1263
        fn in_order(&self) -> ArrayStPerS<T>;in_order53,1304
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order54,1350
    impl<T: StT + Ord> Default for BSTRBStEph<T> {BSTRBStEph57,1404
        fn default() -> Self { Self::new() }default58,1455
    impl<T: StT + Ord> BSTRBStEph<T> {BSTRBStEph61,1507
        pub fn new() -> Self { BSTRBStEph { root: None } }new62,1546
        pub fn size(&self) -> N { Self::size_link(&self.root) }size64,1606
        pub fn is_empty(&self) -> B {is_empty66,1671
        pub fn height(&self) -> N {height74,1838
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec75,1874
        pub fn insert(&mut self, value: T) {insert84,2170
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find91,2390
        pub fn contains(&self, target: &T) -> B {contains93,2484
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum101,2674
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum103,2750
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order105,2826
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order111,3044
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red117,3264
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link119,3366
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate121,3449
        fn rotate_left(link: &mut Link<T>) {rotate_left123,3568
        fn rotate_right(link: &mut Link<T>) {rotate_right140,4164
        fn flip_colors(link: &mut Link<T>) {flip_colors157,4762
        fn fix_up(link: &mut Link<T>) {fix_up178,5567
        fn insert_link(link: &mut Link<T>, value: T) {insert_link195,6254
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link211,6796
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link226,7313
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link236,7634
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect246,7957
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect254,8244
    impl<T: StT + Ord> BSTRBStEphTrait<T> for BSTRBStEph<T> {BSTRBStEph263,8540
        fn new() -> Self { BSTRBStEph::new() }new264,8602
        fn size(&self) -> N { BSTRBStEph::size(self) }size266,8650
        fn is_empty(&self) -> B { BSTRBStEph::is_empty(self) }is_empty268,8706
        fn height(&self) -> N { BSTRBStEph::height(self) }height270,8770
        fn insert(&mut self, value: T) { BSTRBStEph::insert(self, value) }insert272,8830
        fn find(&self, target: &T) -> Option<&T> { BSTRBStEph::find(self, target) }find274,8906
        fn contains(&self, target: &T) -> B { BSTRBStEph::contains(self, target) }contains276,8991
        fn minimum(&self) -> Option<&T> { BSTRBStEph::minimum(self) }minimum278,9075
        fn maximum(&self) -> Option<&T> { BSTRBStEph::maximum(self) }maximum280,9146
        fn in_order(&self) -> ArrayStPerS<T> { BSTRBStEph::in_order(self) }in_order282,9217
        fn pre_order(&self) -> ArrayStPerS<T> { BSTRBStEph::pre_order(self) }pre_order284,9294
    macro_rules! BSTRBStEphLit {BSTRBStEphLit288,9399
    fn _BSTRBStEphLit_type_checks() {_BSTRBStEphLit_type_checks300,9861

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36Mt.rs,1405
pub mod Chapter36Mt {Chapter36Mt3,94
    pub trait Chapter36MtTrait<T: StT + Ord + Send> {Chapter36MtTrait25,1177
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;pivot_mt_first26,1231
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;pivot_mt_median327,1284
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;pivot_mt_random28,1339
        fn quick_sort_mt_first(&mut self);quick_sort_mt_first30,1394
        fn quick_sort_mt_median3(&mut self);quick_sort_mt_median331,1437
        fn quick_sort_mt_random(&mut self);quick_sort_mt_random32,1482
    impl<T: StT + Ord + Send> Chapter36MtTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS35,1533
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }pivot_mt_first36,1607
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {pivot_mt_median337,1684
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {pivot_mt_random50,2154
        fn quick_sort_mt_first(&mut self) {quick_sort_mt_first56,2330
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort61,2449
        fn quick_sort_mt_median3(&mut self) {quick_sort_mt_median399,3795
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort104,3916
        fn quick_sort_mt_random(&mut self) {quick_sort_mt_random154,5729
            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {quick_sort159,5849

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStPer.rs,3146
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
        fn empty() -> Self { LinkedListStPerS { head: None, len: 0 } }empty74,2553
        fn new(length: N, init_value: T) -> Self {new75,2624
        fn length(&self) -> N { self.len }length94,3305
        fn nth(&self, index: N) -> &T {nth95,3348
        fn isEmpty(&self) -> B {isEmpty107,3715
        fn isSingleton(&self) -> B {isSingleton114,3873
        fn singleton(item: T) -> Self {singleton121,4035
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set130,4284
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy155,5283
    impl<T: StT> std::fmt::Debug for LinkedListStPerS<T> {LinkedListStPerS190,6424
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt191,6483
    pub struct LinkedListStPerIter<'a, T: StT> {LinkedListStPerIter202,6849
        cursor: Option<&'a NodeP<T>>,cursor203,6898
    impl<'a, T: StT> Iterator for LinkedListStPerIter<'a, T> {LinkedListStPerIter206,6943
        type Item = &'a T;Item207,7006
        fn next(&mut self) -> Option<Self::Item> {next208,7033
    impl<T: StT> PartialEq for LinkedListStPerS<T> {LinkedListStPerS218,7280
        fn eq(&self, other: &Self) -> bool {eq219,7333
    impl<T: StT> Eq for LinkedListStPerS<T> {}LinkedListStPerS236,7822
    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {LinkedListStPerS238,7870
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt239,7931
    macro_rules! NodePLit {NodePLit257,8459
    macro_rules! LinkedListStPerIterLit {LinkedListStPerIterLit267,8717
    fn _LinkedListStPer_struct_macro_checks() {_LinkedListStPer_struct_macro_checks276,8964
    macro_rules! LinkedListStPerSLit {LinkedListStPerSLit287,9328
    fn _LinkedListStPerSLit_type_checks() {_LinkedListStPerSLit_type_checks296,9824

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtEphSlice.rs,3453
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
    fn _ArraySeqMtEphSliceSLit_type_checks() {_ArraySeqMtEphSliceSLit_type_checks217,7378

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
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {map25,1171
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append38,1677
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter55,2335
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {update67,2760
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphSinject71,2920
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphninject79,3245

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTSetSplayMtEph.rs,5634
pub mod BSTSetSplayMtEph {BSTSetSplayMtEph3,75
    pub struct BSTSetSplayMtEph<T: StTinMtT + Ord> {BSTSetSplayMtEph10,290
        tree: BSTSplayMtEph<T>,tree11,343
    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;BSTSetSplayMt14,382
    pub trait BSTSetSplayMtEphTrait<T: StTinMtT + Ord>: Sized {BSTSetSplayMtEphTrait16,436
        fn empty() -> Self;empty17,500
        fn singleton(value: T) -> Self;singleton18,528
        fn size(&self) -> N;size19,568
        fn is_empty(&self) -> B;is_empty20,597
        fn find(&self, value: &T) -> Option<T>;find21,630
        fn contains(&self, value: &T) -> B;contains22,678
        fn minimum(&self) -> Option<T>;minimum23,722
        fn maximum(&self) -> Option<T>;maximum24,762
        fn insert(&mut self, value: T);insert25,802
        fn delete(&mut self, target: &T);delete26,842
        fn union(&self, other: &Self) -> Self;union27,884
        fn intersection(&self, other: &Self) -> Self;intersection28,931
        fn difference(&self, other: &Self) -> Self;difference29,985
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1037
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1092
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1147
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1209
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1279
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1347
        fn as_tree(&self) -> &BSTSplayMtEph<T>;as_tree36,1398
    impl<T: StTinMtT + Ord> BSTSetSplayMtEph<T> {BSTSetSplayMtEph39,1453
        pub fn empty() -> Self {empty40,1503
        pub fn singleton(value: T) -> Self {singleton46,1624
        pub fn size(&self) -> N { self.tree.size() }size52,1783
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1837
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1899
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1977
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2055
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2123
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2191
        pub fn delete(&mut self, target: &T) {delete66,2264
        pub fn union(&self, other: &Self) -> Self {union74,2553
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2852
        pub fn difference(&self, other: &Self) -> Self {difference99,3430
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4007
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4698
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5011
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5367
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5776
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6040
        pub fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree178,6120
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6187
        fn rebuild_from_vec(values: Vec<T>) -> BSTSplayMtEph<T> {rebuild_from_vec182,6278
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6501
    impl<T: StTinMtT + Ord> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {BSTSetSplayMtEph202,6786
        fn empty() -> Self { Self::empty() }empty203,6865
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6911
        fn size(&self) -> N { self.tree.size() }size207,6978
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7028
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7086
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7160
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7234
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7298
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7362
        fn delete(&mut self, target: &T) { BSTSetSplayMtEph::delete(self, target) }delete221,7431
        fn union(&self, other: &Self) -> Self { BSTSetSplayMtEph::union(self, other) }union223,7516
        fn intersection(&self, other: &Self) -> Self { BSTSetSplayMtEph::intersection(self, otheintersection225,7604
        fn difference(&self, other: &Self) -> Self { BSTSetSplayMtEph::difference(self, other) }difference227,7706
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetSplayMtEph::split(self, pivot) }split229,7804
        fn join_pair(left: Self, right: Self) -> Self { BSTSetSplayMtEph::join_pair(left, right)join_pair231,7900
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetSplayMtEph::join_m(left, pijoin_m233,8000
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetSplayMtEph::filter(filter235,8111
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetSplayMtEph::reduce(sereduce237,8227
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8340
        fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }as_tree241,8416
    macro_rules! BSTSetSplayMtEphLit {BSTSetSplayMtEphLit245,8505
    fn _BSTSetSplayMtEphLit_type_checks() {_BSTSetSplayMtEphLit_type_checks257,9046

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEphChap18.rs,2912
pub mod LinkedListStEphChap18 {LinkedListStEphChap183,60
    pub trait LinkedListStEphChap18Trait<T: StT> {LinkedListStEphChap18Trait8,212
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;tabulate11,355
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;map14,521
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;append17,722
        fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T>;filter20,910
        fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T>;update23,1109
        fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListinject26,1326
        fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedLisninject29,1553
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1661
        fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes31,1750
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce32,1870
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan33,1952
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;flatten34,2055
        fn collect<A: StT, Bv: StT>(collect35,2142
    impl<T: StT> LinkedListStEphChap18Trait<T> for LinkedListStEphS<T> {LinkedListStEphS41,2334
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate42,2407
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map49,2657
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append56,3104
        fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter68,3746
        fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T> update78,4216
        fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListinject81,4405
        fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedLisninject99,5330
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate113,6089
        fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes120,6426
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce130,6950
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan145,7768
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten164,8624
        fn collect<A: StT, Bv: StT>(collect176,9316

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTBBAlphaMtEph.rs,4514
pub mod BSTBBAlphaMtEph {BSTBBAlphaMtEph3,108
    type Link<T> = Option<Box<Node<T>>>;Link12,340
    struct Node<T: StTinMtT + Ord> {Node15,403
        key: T,key16,440
        size: N,size17,456
        left: Link<T>,left18,473
        right: Link<T>,right19,496
    impl<T: StTinMtT + Ord> Node<T> {Node22,527
        fn new(key: T) -> Self {new23,565
    pub struct BSTBBAlphaMtEph<T: StTinMtT + Ord> {BSTBBAlphaMtEph34,772
        root: Arc<RwLock<Link<T>>>,root35,824
    pub type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;BSTreeBBAlpha38,867
    pub trait BSTBBAlphaMtEphTrait<T: StTinMtT + Ord>: Sized {BSTBBAlphaMtEphTrait40,920
        fn new() -> Self;new41,983
        fn insert(&self, value: T);insert42,1009
        fn find(&self, target: &T) -> Option<T>;find43,1045
        fn contains(&self, target: &T) -> B;contains44,1094
        fn size(&self) -> N;size45,1139
        fn is_empty(&self) -> B;is_empty46,1168
        fn height(&self) -> N;height47,1201
        fn minimum(&self) -> Option<T>;minimum48,1232
        fn maximum(&self) -> Option<T>;maximum49,1272
        fn in_order(&self) -> ArrayStPerS<T>;in_order50,1312
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order51,1358
    impl<T: StTinMtT + Ord> Default for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph54,1412
        fn default() -> Self { Self::new() }default55,1473
    impl<T: StTinMtT + Ord> BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph58,1525
        pub fn new() -> Self {new59,1574
        pub fn size(&self) -> N {size65,1711
        pub fn is_empty(&self) -> B {is_empty70,1844
        pub fn height(&self) -> N {height78,2011
            fn height_rec<T: StTinMtT + Ord>(link: &Link<T>) -> N {height_rec79,2047
        pub fn insert(&self, value: T) {insert90,2397
        pub fn find(&self, target: &T) -> Option<T> {find99,2728
        pub fn contains(&self, target: &T) -> B {contains104,2898
        pub fn minimum(&self) -> Option<T> {minimum112,3088
        pub fn maximum(&self) -> Option<T> {maximum117,3240
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order122,3392
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order129,3671
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link136,3952
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate138,4035
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link140,4154
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild162,4900
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed169,5182
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values179,5592
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced187,5873
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link199,6313
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link214,6830
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link224,7151
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect234,7474
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect242,7761
    impl<T: StTinMtT + Ord> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {BSTBBAlphaMtEph251,8057
        fn new() -> Self { BSTBBAlphaMtEph::new() }new252,8134
        fn insert(&self, value: T) { BSTBBAlphaMtEph::insert(self, value) }insert254,8187
        fn find(&self, target: &T) -> Option<T> { BSTBBAlphaMtEph::find(self, target) }find256,8264
        fn contains(&self, target: &T) -> B { BSTBBAlphaMtEph::contains(self, target) }contains258,8353
        fn size(&self) -> N { BSTBBAlphaMtEph::size(self) }size260,8442
        fn is_empty(&self) -> B { BSTBBAlphaMtEph::is_empty(self) }is_empty262,8503
        fn height(&self) -> N { BSTBBAlphaMtEph::height(self) }height264,8572
        fn minimum(&self) -> Option<T> { BSTBBAlphaMtEph::minimum(self) }minimum266,8637
        fn maximum(&self) -> Option<T> { BSTBBAlphaMtEph::maximum(self) }maximum268,8712
        fn in_order(&self) -> ArrayStPerS<T> { BSTBBAlphaMtEph::in_order(self) }in_order270,8787
        fn pre_order(&self) -> ArrayStPerS<T> { BSTBBAlphaMtEph::pre_order(self) }pre_order272,8869
    macro_rules! BSTBBAlphaMtEphLit {BSTBBAlphaMtEphLit276,8979
    fn _BSTBBAlphaMtEphLit_type_checks() {_BSTBBAlphaMtEphLit_type_checks288,9502

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Types.rs,5980
pub mod Types {Types4,45
    pub type N = usize;N10,181
    pub enum B {B14,324
        True,True15,341
        False,False16,355
    impl std::fmt::Display for B {B23,588
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt24,623
    pub trait StT: Eq + Clone + Display + Debug + Sized {}StT34,968
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}T35,1027
    pub trait StTinMtT: StT + Send + Sync {}StTinMtT38,1191
    impl<T> StTinMtT for T where T: StT + Send + Sync {}T39,1236
    pub trait MtT: Sized + Send + Sync {MtT43,1430
        type Inner: StT;Inner44,1471
        fn clone_mt(&self) -> Self;clone_mt45,1496
        fn new_mt(inner: Self::Inner) -> Self;new_mt46,1532
    impl<T: StT + Send> MtT for std::sync::Mutex<T> {Mutex49,1586
        type Inner = T;Inner50,1640
        fn clone_mt(&self) -> Self {clone_mt51,1664
        fn new_mt(inner: Self::Inner) -> Self { std::sync::Mutex::new(inner) }new_mt55,1806
    impl<A: StT + Send + Sync, B: StT + Send + Sync> MtT for Pair<A, B> {Pair58,1892
        type Inner = Pair<A, B>;Inner59,1966
        fn clone_mt(&self) -> Self { self.clone() }clone_mt60,1999
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt61,2051
    impl MtT for usize {usize65,2192
        type Inner = usize;Inner66,2217
        fn clone_mt(&self) -> Self { *self }clone_mt67,2245
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt68,2290
    impl MtT for isize {isize71,2353
        type Inner = isize;Inner72,2378
        fn clone_mt(&self) -> Self { *self }clone_mt73,2406
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt74,2451
    impl MtT for i32 {i3277,2514
        type Inner = i32;Inner78,2537
        fn clone_mt(&self) -> Self { *self }clone_mt79,2563
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt80,2608
    impl MtT for u32 {u3283,2671
        type Inner = u32;Inner84,2694
        fn clone_mt(&self) -> Self { *self }clone_mt85,2720
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt86,2765
    impl MtT for i64 {i6489,2828
        type Inner = i64;Inner90,2851
        fn clone_mt(&self) -> Self { *self }clone_mt91,2877
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt92,2922
    impl MtT for u64 {u6495,2985
        type Inner = u64;Inner96,3008
        fn clone_mt(&self) -> Self { *self }clone_mt97,3034
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt98,3079
    impl MtT for bool {bool101,3142
        type Inner = bool;Inner102,3166
        fn clone_mt(&self) -> Self { *self }clone_mt103,3193
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt104,3238
    impl MtT for char {char107,3301
        type Inner = char;Inner108,3325
        fn clone_mt(&self) -> Self { *self }clone_mt109,3352
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt110,3397
    impl MtT for String {String114,3514
        type Inner = String;Inner115,3540
        fn clone_mt(&self) -> Self { self.clone() }clone_mt116,3569
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt117,3621
    impl<'a> MtT for &'a str {str121,3719
        type Inner = &'a str;Inner122,3750
        fn clone_mt(&self) -> Self { *self }clone_mt123,3780
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt124,3825
    impl MtT for B {B128,3930
        type Inner = B;Inner129,3951
        fn clone_mt(&self) -> Self { *self }clone_mt130,3975
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt131,4020
    pub struct Edge<V: StT>(pub V, pub V);Edge136,4222
    impl<V: StT> std::fmt::Display for Edge<V> {Edge138,4266
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})fmt139,4315
    impl<V: StT> From<(V, V)> for Edge<V> {Edge142,4439
        fn from(t: (V, V)) -> Self { Edge(t.0, t.1) }from143,4483
    impl<V: StT> From<Edge<V>> for (V, V) {V146,4544
        fn from(e: Edge<V>) -> (V, V) { (e.0, e.1) }from147,4588
    pub struct LabEdge<V: StT, L: Clone + Debug + Display + Eq + Hash>(pub V, pub V, pub L);LabEdge152,4761
    impl<V: StT, L: Clone + Debug + Display + Eq + Hash> std::fmt::Display for LabEdge<V, L> {LabEdge154,4855
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { fmt155,4950
    impl<V: StT, L: Clone + Debug + Display + Eq + Hash> From<(V, V, L)> for LabEdge<V, L> {LabEdge160,5108
        fn from(t: (V, V, L)) -> Self { LabEdge(t.0, t.1, t.2) }from161,5201
    impl<V: StT, L: Clone + Debug + Display + Eq + Hash> From<LabEdge<V, L>> for (V, V, L) {L164,5273
        fn from(e: LabEdge<V, L>) -> (V, V, L) { (e.0, e.1, e.2) }from165,5366
    pub struct Pair<A, B>(pub A, pub B);Pair170,5594
    impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {Pair172,5636
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})fmt173,5724
    impl<A, B> From<(A, B)> for Pair<A, B> {Pair176,5848
        fn from(t: (A, B)) -> Self { Pair(t.0, t.1) }from177,5893
    impl<A, B> From<Pair<A, B>> for (A, B) {B180,5954
        fn from(p: Pair<A, B>) -> Self { (p.0, p.1) }from181,5999
    macro_rules! ParaPair {ParaPair185,6080
    fn _ParaPair_type_checks() {_ParaPair_type_checks196,6554
    pub fn ArraySeqSetEq<T: PartialEq>(a_len: N, a_nth: impl Fn(N) -> T, b_len: N, b_nth: impl FArraySeqSetEq205,6974
    macro_rules! EdgeLit {EdgeLit244,8032
    macro_rules! PairLit {PairLit251,8176
    macro_rules! EdgeList {EdgeList258,8320
    macro_rules! PairList {PairList268,8537
    fn _EdgeLit_type_checks() {_EdgeLit_type_checks278,8758
    fn _PairLit_type_checks() {_PairLit_type_checks284,8945
    fn _EdgeList_type_checks() {_EdgeList_type_checks290,9137
    fn _PairList_type_checks() {_PairList_type_checks296,9346

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStEph.rs,2678
pub mod ArraySeqStEph {ArraySeqStEph3,93
    pub struct ArraySeqStEphS<T: StT> {ArraySeqStEphS13,336
        pub data: Box<[T]>,data14,376
    pub trait ArraySeqStEphTrait<T: StT> {ArraySeqStEphTrait18,459
        fn new(length: N, init_value: T) -> Self;new21,604
        fn length(&self) -> N;length24,746
        fn nth(&self, index: N) -> &T;nth27,869
        fn empty() -> Self;empty30,1000
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set33,1120
        fn singleton(item: T) -> Self;singleton36,1293
        fn isEmpty(&self) -> B;isEmpty39,1424
        fn isSingleton(&self) -> B;isSingleton42,1548
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy45,1686
    impl<T: StT> ArraySeqStEphS<T> {ArraySeqStEphS48,1753
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq51,1882
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy59,2211
        pub fn from_vec(v: Vec<T>) -> Self {from_vec77,2957
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {update84,3191
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter93,3491
    impl<T: StT> PartialEq for ArraySeqStEphS<T> {ArraySeqStEphS96,3561
        fn eq(&self, other: &Self) -> bool {eq97,3612
    impl<T: StT> Eq for ArraySeqStEphS<T> {}ArraySeqStEphS110,3939
    impl<T: StT> Debug for ArraySeqStEphS<T> {ArraySeqStEphS112,3985
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt113,4032
    impl<T: StT> Display for ArraySeqStEphS<T> {ArraySeqStEphS119,4225
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt120,4274
    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {ArraySeqStEphS132,4593
        fn new(length: N, init_value: T) -> Self { ArraySeqStEphS::from_vec(vec![init_value; lennew135,4758
        fn length(&self) -> N { self.data.len() }length138,4954
        fn nth(&self, index: N) -> &T { &self.data[index] }nth141,5096
        fn empty() -> Self { ArraySeqStEphS::from_vec(Vec::new()) }empty144,5248
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set147,5408
        fn singleton(item: T) -> Self { ArraySeqStEphS::from_vec(vec![item]) }singleton157,5777
        fn isEmpty(&self) -> B {isEmpty160,5948
        fn isSingleton(&self) -> B {isSingleton169,6205
        fn subseq_copy(&self, start: N, length: N) -> Self { self.subseq_copy(start, length) }subseq_copy178,6476
    macro_rules! ArraySeqStEphSLit {ArraySeqStEphSLit182,6598
    fn _ArraySeqStEphSLit_type_checks() {_ArraySeqStEphSLit_type_checks189,6987

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
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append41,2020
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {append244,2194
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T> {deflate47,2369
        fn filter(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {filter54,2628
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, mut x: A) -> A {iterate57,2800
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> T {reduce63,3012
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> (ArraySeqMtEphS<Tscan69,3220
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {flatten82,3714

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStEphChap19.rs,2407
pub mod ArraySeqStEphChap19 {ArraySeqStEphChap193,51
    pub trait ArraySeqStEphChap19Trait<T: StT> {ArraySeqStEphChap19Trait8,222
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;tabulate11,363
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;map14,527
        fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T>;select17,708
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append20,910
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;append223,1104
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T>;deflate26,1283
        fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;filter29,1448
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate30,1532
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce31,1619
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan32,1699
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;flatten33,1798
    impl<T: StT> ArraySeqStEphChap19Trait<T> for ArraySeqStEphS<T> {ArraySeqStEphS36,1885
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {tabulate37,1954
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {map40,2112
        fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T> {select43,2286
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append55,2690
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {append258,2864
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T> {deflate61,3039
        fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {filter68,3340
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate71,3512
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce74,3691
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, Tscan77,3863
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {flatten80,4052

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTAVLStEph.rs,4434
pub mod BSTAVLStEph {BSTAVLStEph3,97
    type Link<T> = Option<Box<Node<T>>>;Link8,260
    struct Node<T: StT + Ord> {Node11,323
        key: T,key12,355
        height: i32,height13,371
        size: N,size14,392
        left: Link<T>,left15,409
        right: Link<T>,right16,432
    impl<T: StT + Ord> Node<T> {Node19,463
        fn new(key: T) -> Self {new20,496
    pub struct BSTAVLStEph<T: StT + Ord> {BSTAVLStEph31,709
        root: Link<T>,root32,752
    pub type BSTreeAVL<T> = BSTAVLStEph<T>;BSTreeAVL35,782
    pub trait BSTAVLStEphTrait<T: StT + Ord> {BSTAVLStEphTrait37,827
        fn new() -> Self;new38,874
        fn size(&self) -> N;size39,900
        fn is_empty(&self) -> B;is_empty40,929
        fn height(&self) -> N;height41,962
        fn insert(&mut self, value: T);insert42,993
        fn find(&self, target: &T) -> Option<&T>;find43,1033
        fn contains(&self, target: &T) -> B;contains44,1083
        fn minimum(&self) -> Option<&T>;minimum45,1128
        fn maximum(&self) -> Option<&T>;maximum46,1169
        fn in_order(&self) -> ArrayStPerS<T>;in_order47,1210
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order48,1256
    impl<T: StT + Ord> Default for BSTAVLStEph<T> {BSTAVLStEph51,1310
        fn default() -> Self { Self::new() }default52,1362
    impl<T: StT + Ord> BSTAVLStEph<T> {BSTAVLStEph55,1414
        pub fn new() -> Self { BSTAVLStEph { root: None } }new56,1454
        pub fn size(&self) -> N { Self::size_link(&self.root) }size58,1515
        pub fn is_empty(&self) -> B {is_empty60,1580
        pub fn height(&self) -> N { Self::height_link(&self.root) as N }height68,1747
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert70,1821
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find72,1911
        pub fn contains(&self, target: &T) -> B {contains74,2005
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum82,2195
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum84,2271
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order86,2347
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order92,2565
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link98,2785
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link100,2874
        fn update(node: &mut Node<T>) {update102,2957
        fn rotate_right(link: &mut Link<T>) {rotate_right107,3193
        fn rotate_left(link: &mut Link<T>) {rotate_left121,3649
        fn rebalance(link: &mut Link<T>) {rebalance135,4104
        fn insert_link(link: &mut Link<T>, value: T) {insert_link160,5150
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link179,5785
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link194,6302
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link204,6623
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect214,6946
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect222,7233
    impl<T: StT + Ord> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {BSTAVLStEph231,7529
        fn new() -> Self { BSTAVLStEph::new() }new232,7593
        fn size(&self) -> N { BSTAVLStEph::size(self) }size234,7642
        fn is_empty(&self) -> B { BSTAVLStEph::is_empty(self) }is_empty236,7699
        fn height(&self) -> N { BSTAVLStEph::height(self) }height238,7764
        fn insert(&mut self, value: T) { BSTAVLStEph::insert(self, value) }insert240,7825
        fn find(&self, target: &T) -> Option<&T> { BSTAVLStEph::find(self, target) }find242,7902
        fn contains(&self, target: &T) -> B { BSTAVLStEph::contains(self, target) }contains244,7988
        fn minimum(&self) -> Option<&T> { BSTAVLStEph::minimum(self) }minimum246,8073
        fn maximum(&self) -> Option<&T> { BSTAVLStEph::maximum(self) }maximum248,8145
        fn in_order(&self) -> ArrayStPerS<T> { BSTAVLStEph::in_order(self) }in_order250,8217
        fn pre_order(&self) -> ArrayStPerS<T> { BSTAVLStEph::pre_order(self) }pre_order252,8295
    macro_rules! BSTAVLStEphLit {BSTAVLStEphLit256,8401
    fn _BSTAVLStEphLit_type_checks() {_BSTAVLStEphLit_type_checks268,8876

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPerChap18.rs,3029
pub mod ArraySeqStPerChap18 {ArraySeqStPerChap183,46
    pub trait ArraySeqStPerChap18Trait<T: StT> {ArraySeqStPerChap18Trait7,157
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;tabulate10,388
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;map14,626
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;append18,830
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T>;filter22,1114
        fn update(a: &ArrayStPerS<T>, item_at: Pair<N, T>) -> ArrayStPerS<T>;update26,1296
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;inject30,1535
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;ninject34,1752
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate37,1923
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes40,2053
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce43,2255
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);scan46,2380
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;flatten49,2544
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect52,2696
    impl<T: StT> ArraySeqStPerChap18Trait<T> for ArrayStPerS<T> {ArrayStPerS55,2819
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {tabulate56,2885
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {map60,3064
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {append72,3560
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T> {filter86,4084
        fn update(a: &ArrayStPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayStPerS<T> {update95,4419
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {inject101,4652
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {ninject113,5216
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate123,5658
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) ->iteratePrefixes130,5890
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce139,6293
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec140,6371
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {scan155,6871
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {rec156,6965
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {flatten177,7725
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<Tcollect187,8093

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTPlainMtEph.rs,3702
pub mod BSTPlainMtEph {BSTPlainMtEph3,90
    type Link<T> = Arc<RwLock<Option<Node<T>>>>;Link9,230
    struct Node<T: StTinMtT + Ord> {Node12,301
        key: T,key13,338
        height: i32,height14,354
        size: N,size15,375
        left: Link<T>,left16,392
        right: Link<T>,right17,415
    impl<T: StTinMtT + Ord> Node<T> {Node20,446
        fn new(key: T) -> Self {new21,484
        fn update(&mut self) {update31,737
    pub struct BSTPlainMtEph<T: StTinMtT + Ord> {BSTPlainMtEph39,1020
        root: Link<T>,root40,1070
    pub type BSTree<T> = BSTPlainMtEph<T>;BSTree43,1100
    pub trait BSTPlainMtEphTrait<T: StTinMtT + Ord>: Sized {BSTPlainMtEphTrait45,1144
        fn new() -> Self;new46,1205
        fn insert(&self, value: T);insert47,1231
        fn find(&self, target: &T) -> Option<T>;find48,1267
        fn contains(&self, target: &T) -> B;contains49,1316
        fn size(&self) -> N;size50,1361
        fn is_empty(&self) -> B;is_empty51,1390
        fn height(&self) -> N;height52,1423
        fn minimum(&self) -> Option<T>;minimum53,1454
        fn maximum(&self) -> Option<T>;maximum54,1494
        fn in_order(&self) -> ArrayStPerS<T>;in_order55,1534
    impl<T: StTinMtT + Ord> BSTPlainMtEph<T> {BSTPlainMtEph58,1587
        pub fn new() -> Self {new59,1634
        pub fn insert(&self, value: T) {insert65,1760
            fn descend<T: StTinMtT + Ord>(link: &Link<T>, value: T) -> bool {descend66,1801
        pub fn find(&self, target: &T) -> Option<T> {find100,2993
            fn find_rec<T: StTinMtT + Ord>(link: &Link<T>, target: &T) -> Option<T> {find_rec101,3047
        pub fn contains(&self, target: &T) -> B {contains120,3786
        pub fn size(&self) -> N {size127,3975
        pub fn is_empty(&self) -> B {is_empty132,4126
        pub fn height(&self) -> N {height140,4293
        pub fn minimum(&self) -> Option<T> {minimum145,4453
            fn leftmost<T: StTinMtT + Ord>(link: &Link<T>) -> Option<T> {leftmost146,4498
        pub fn maximum(&self) -> Option<T> {maximum167,5208
            fn rightmost<T: StTinMtT + Ord>(link: &Link<T>) -> Option<T> {rightmost168,5253
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order189,5970
            fn traverse<T: StTinMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {traverse190,6021
    fn height_of<T: StTinMtT + Ord>(link: &Option<Node<T>>) -> i32 { link.as_ref().map_or(0, |n|height_of209,6686
    fn size_of<T: StTinMtT + Ord>(link: &Option<Node<T>>) -> N { link.as_ref().map_or(0, |n| n.ssize_of211,6796
    impl<T: StTinMtT + Ord> BSTPlainMtEphTrait<T> for BSTPlainMtEph<T> {BSTPlainMtEph213,6900
        fn new() -> Self { BSTPlainMtEph::new() }new214,6973
        fn insert(&self, value: T) { BSTPlainMtEph::insert(self, value) }insert215,7023
        fn find(&self, target: &T) -> Option<T> { BSTPlainMtEph::find(self, target) }find216,7097
        fn contains(&self, target: &T) -> B { BSTPlainMtEph::contains(self, target) }contains217,7183
        fn size(&self) -> N { BSTPlainMtEph::size(self) }size218,7269
        fn is_empty(&self) -> B { BSTPlainMtEph::is_empty(self) }is_empty219,7327
        fn height(&self) -> N { BSTPlainMtEph::height(self) }height220,7393
        fn minimum(&self) -> Option<T> { BSTPlainMtEph::minimum(self) }minimum221,7455
        fn maximum(&self) -> Option<T> { BSTPlainMtEph::maximum(self) }maximum222,7527
        fn in_order(&self) -> ArrayStPerS<T> { BSTPlainMtEph::in_order(self) }in_order223,7599
    macro_rules! BSTPlainMtEphLit {BSTPlainMtEphLit227,7705
    fn _BSTPlainMtEphLit_type_checks() {_BSTPlainMtEphLit_type_checks242,8245

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTBBAlphaStEph.rs,4626
pub mod BSTBBAlphaStEph {BSTBBAlphaStEph3,80
    type Link<T> = Option<Box<Node<T>>>;Link10,277
    struct Node<T: StT + Ord> {Node13,340
        key: T,key14,372
        size: N,size15,388
        left: Link<T>,left16,405
        right: Link<T>,right17,428
    impl<T: StT + Ord> Node<T> {Node20,459
        fn new(key: T) -> Self {new21,492
    pub struct BSTBBAlphaStEph<T: StT + Ord> {BSTBBAlphaStEph31,678
        root: Link<T>,root32,725
    pub type BSTreeBBAlpha<T> = BSTBBAlphaStEph<T>;BSTreeBBAlpha35,755
    pub trait BSTBBAlphaStEphTrait<T: StT + Ord> {BSTBBAlphaStEphTrait37,808
        fn new() -> Self;new38,859
        fn size(&self) -> N;size39,885
        fn is_empty(&self) -> B;is_empty40,914
        fn height(&self) -> N;height41,947
        fn insert(&mut self, value: T);insert42,978
        fn find(&self, target: &T) -> Option<&T>;find43,1018
        fn contains(&self, target: &T) -> B;contains44,1068
        fn minimum(&self) -> Option<&T>;minimum45,1113
        fn maximum(&self) -> Option<&T>;maximum46,1154
        fn in_order(&self) -> ArrayStPerS<T>;in_order47,1195
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order48,1241
    impl<T: StT + Ord> Default for BSTBBAlphaStEph<T> {BSTBBAlphaStEph51,1295
        fn default() -> Self { Self::new() }default52,1351
    impl<T: StT + Ord> BSTBBAlphaStEph<T> {BSTBBAlphaStEph55,1403
        pub fn new() -> Self { BSTBBAlphaStEph { root: None } }new56,1447
        pub fn size(&self) -> N { Self::size_link(&self.root) }size58,1512
        pub fn is_empty(&self) -> B {is_empty60,1577
        pub fn height(&self) -> N {height68,1744
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec69,1780
        pub fn insert(&mut self, value: T) {insert78,2076
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find86,2364
        pub fn contains(&self, target: &T) -> B {contains88,2458
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum96,2648
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum98,2724
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order100,2800
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order106,3018
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link112,3238
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate114,3321
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link116,3440
        fn needs_rebuild(node: &Node<T>) -> bool {needs_rebuild138,4186
        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {rebalance_if_needed145,4468
        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {collect_values155,4878
        fn build_balanced(values: &[T]) -> Link<T> {build_balanced163,5159
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link175,5599
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link190,6116
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link200,6437
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect210,6760
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect218,7047
    impl<T: StT + Ord> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {BSTBBAlphaStEph227,7343
        fn new() -> Self { BSTBBAlphaStEph::new() }new228,7415
        fn size(&self) -> N { BSTBBAlphaStEph::size(self) }size230,7468
        fn is_empty(&self) -> B { BSTBBAlphaStEph::is_empty(self) }is_empty232,7529
        fn height(&self) -> N { BSTBBAlphaStEph::height(self) }height234,7598
        fn insert(&mut self, value: T) { BSTBBAlphaStEph::insert(self, value) }insert236,7663
        fn find(&self, target: &T) -> Option<&T> { BSTBBAlphaStEph::find(self, target) }find238,7744
        fn contains(&self, target: &T) -> B { BSTBBAlphaStEph::contains(self, target) }contains240,7834
        fn minimum(&self) -> Option<&T> { BSTBBAlphaStEph::minimum(self) }minimum242,7923
        fn maximum(&self) -> Option<&T> { BSTBBAlphaStEph::maximum(self) }maximum244,7999
        fn in_order(&self) -> ArrayStPerS<T> { BSTBBAlphaStEph::in_order(self) }in_order246,8075
        fn pre_order(&self) -> ArrayStPerS<T> { BSTBBAlphaStEph::pre_order(self) }pre_order248,8157
    macro_rules! BSTBBAlphaStEphLit {BSTBBAlphaStEphLit252,8267
    fn _BSTBBAlphaStEphLit_type_checks() {_BSTBBAlphaStEphLit_type_checks264,8794

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTTreapMtEph.rs,4319
pub mod BSTTreapMtEph {BSTTreapMtEph3,100
    type Link<T> = Option<Box<Node<T>>>;Link11,326
    struct Node<T: StTinMtT + Ord> {Node14,389
        key: T,key15,426
        priority: u64,priority16,442
        size: N,size17,465
        left: Link<T>,left18,482
        right: Link<T>,right19,505
    impl<T: StTinMtT + Ord> Node<T> {Node22,536
        fn new(key: T, priority: u64) -> Self {new23,574
    pub struct BSTTreapMtEph<T: StTinMtT + Ord> {BSTTreapMtEph35,822
        root: Arc<RwLock<Link<T>>>,root36,872
    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;BSTreeTreap39,915
    pub trait BSTTreapMtEphTrait<T: StTinMtT + Ord>: Sized {BSTTreapMtEphTrait41,964
        fn new() -> Self;new42,1025
        fn insert(&self, value: T);insert43,1051
        fn find(&self, target: &T) -> Option<T>;find44,1087
        fn contains(&self, target: &T) -> B;contains45,1136
        fn size(&self) -> N;size46,1181
        fn is_empty(&self) -> B;is_empty47,1210
        fn height(&self) -> N;height48,1243
        fn minimum(&self) -> Option<T>;minimum49,1274
        fn maximum(&self) -> Option<T>;maximum50,1314
        fn in_order(&self) -> ArrayStPerS<T>;in_order51,1354
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order52,1400
    impl<T: StTinMtT + Ord> Default for BSTTreapMtEph<T> {BSTTreapMtEph55,1454
        fn default() -> Self { Self::new() }default56,1513
    impl<T: StTinMtT + Ord> BSTTreapMtEph<T> {BSTTreapMtEph59,1565
        pub fn new() -> Self {new60,1612
        pub fn size(&self) -> N {size66,1747
        pub fn is_empty(&self) -> B {is_empty71,1880
        pub fn height(&self) -> N {height79,2047
            fn height_rec<T: StTinMtT + Ord>(link: &Link<T>) -> N {height_rec80,2083
        pub fn insert(&self, value: T) {insert91,2433
        pub fn find(&self, target: &T) -> Option<T> {find97,2635
        pub fn contains(&self, target: &T) -> B {contains102,2805
        pub fn minimum(&self) -> Option<T> {minimum110,2995
        pub fn maximum(&self) -> Option<T> {maximum115,3147
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order120,3299
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order127,3578
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link134,3859
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate136,3942
        fn rotate_left(link: &mut Link<T>) {rotate_left138,4061
        fn rotate_right(link: &mut Link<T>) {rotate_right152,4516
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link166,4972
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link191,5992
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link206,6509
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link216,6830
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect226,7153
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect234,7440
    impl<T: StTinMtT + Ord> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {BSTTreapMtEph243,7736
        fn new() -> Self { BSTTreapMtEph::new() }new244,7809
        fn insert(&self, value: T) { BSTTreapMtEph::insert(self, value) }insert246,7860
        fn find(&self, target: &T) -> Option<T> { BSTTreapMtEph::find(self, target) }find248,7935
        fn contains(&self, target: &T) -> B { BSTTreapMtEph::contains(self, target) }contains250,8022
        fn size(&self) -> N { BSTTreapMtEph::size(self) }size252,8109
        fn is_empty(&self) -> B { BSTTreapMtEph::is_empty(self) }is_empty254,8168
        fn height(&self) -> N { BSTTreapMtEph::height(self) }height256,8235
        fn minimum(&self) -> Option<T> { BSTTreapMtEph::minimum(self) }minimum258,8298
        fn maximum(&self) -> Option<T> { BSTTreapMtEph::maximum(self) }maximum260,8371
        fn in_order(&self) -> ArrayStPerS<T> { BSTTreapMtEph::in_order(self) }in_order262,8444
        fn pre_order(&self) -> ArrayStPerS<T> { BSTTreapMtEph::pre_order(self) }pre_order264,8524
    macro_rules! BSTTreapMtEphLit {BSTTreapMtEphLit268,8632
    fn _BSTTreapMtEphLit_type_checks() {_BSTTreapMtEphLit_type_checks280,9129

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/main.rs,23
fn main() {main7,236

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtPerChap18.rs,2833
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
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes32,2073
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce34,2266
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);scan36,2390
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;flatten38,2553
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect40,2704
    impl<T: MtT> ArraySeqMtPerChap18Trait<T> for ArrayMtPerS<T> {ArrayMtPerS43,2827
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {tabulate44,2893
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {map49,3073
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {append53,3267
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {filter66,3672
        fn update(a: &ArrayMtPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayMtPerS<T> {update76,4029
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {inject80,4190
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {ninject93,4839
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate104,5274
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMiteratePrefixes112,5507
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce123,5946
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {scan134,6276
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {flatten144,6660
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<Tcollect155,7062

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTParaStEph.rs,2986
pub mod BSTParaStEph {BSTParaStEph3,70
    pub enum Exposed<T: StT + Ord> {Exposed11,245
        Leaf,Leaf12,282
        Node(ParamBST<T>, T, ParamBST<T>),Node13,296
    struct NodeInner<T: StT + Ord> {NodeInner17,367
        key: T,key18,404
        size: N,size19,420
        left: ParamBST<T>,left20,437
        right: ParamBST<T>,right21,464
    pub struct ParamBST<T: StT + Ord> {ParamBST25,520
        root: Rc<RefCell<Option<Box<NodeInner<T>>>>>,root26,560
    pub trait ParamBSTTrait<T: StT + Ord>: Sized {ParamBSTTrait29,621
        fn new() -> Self;new30,672
        fn expose(&self) -> Exposed<T>;expose31,698
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid32,738
        fn size(&self) -> N;size33,788
        fn is_empty(&self) -> B;is_empty34,817
        fn insert(&self, key: T);insert35,850
        fn delete(&self, key: &T);delete36,884
        fn find(&self, key: &T) -> Option<T>;find37,919
        fn split(&self, key: &T) -> (Self, B, Self);split38,965
        fn join_pair(&self, other: Self) -> Self;join_pair39,1018
        fn union(&self, other: &Self) -> Self;union40,1068
        fn in_order(&self) -> ArrayStPerS<T>;in_order41,1115
    impl<T: StT + Ord> ParamBST<T> {ParamBST44,1168
        fn expose_internal(&self) -> Exposed<T> {expose_internal45,1205
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid53,1497
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner65,1949
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m84,2934
        fn min_key(tree: &Self) -> Option<T> {min_key86,3050
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner96,3394
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner107,3864
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order119,4371
    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {ParamBST131,4773
        fn new() -> Self {new132,4831
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose138,4957
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid140,5024
        fn size(&self) -> N {size142,5106
        fn is_empty(&self) -> B {is_empty146,5215
        fn insert(&self, key: T) {insert154,5378
        fn delete(&self, key: &T) {delete161,5668
        fn find(&self, key: &T) -> Option<T> {find168,5960
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split179,6457
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair181,6547
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union183,6648
        fn in_order(&self) -> ArrayStPerS<T> {in_order185,6734
    macro_rules! ParamBSTLit {ParamBSTLit193,6972
    fn _ParamBSTLit_type_checks() {_ParamBSTLit_type_checks205,7436

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTSplayMtEph.rs,4114
pub mod BSTSplayMtEph {BSTSplayMtEph3,99
    type Link<T> = Option<Box<Node<T>>>;Link10,299
    struct Node<T: StTinMtT + Ord> {Node13,362
        key: T,key14,399
        size: N,size15,415
        left: Link<T>,left16,432
        right: Link<T>,right17,455
    impl<T: StTinMtT + Ord> Node<T> {Node20,486
        fn new(key: T) -> Self {new21,524
    pub struct BSTSplayMtEph<T: StTinMtT + Ord> {BSTSplayMtEph32,731
        root: Arc<RwLock<Link<T>>>,root33,781
    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;BSTreeSplay36,824
    pub trait BSTSplayMtEphTrait<T: StTinMtT + Ord>: Sized {BSTSplayMtEphTrait38,873
        fn new() -> Self;new39,934
        fn insert(&self, value: T);insert40,960
        fn find(&self, target: &T) -> Option<T>;find41,996
        fn contains(&self, target: &T) -> B;contains42,1045
        fn size(&self) -> N;size43,1090
        fn is_empty(&self) -> B;is_empty44,1119
        fn height(&self) -> N;height45,1152
        fn minimum(&self) -> Option<T>;minimum46,1183
        fn maximum(&self) -> Option<T>;maximum47,1223
        fn in_order(&self) -> ArrayStPerS<T>;in_order48,1263
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order49,1309
    impl<T: StTinMtT + Ord> Default for BSTSplayMtEph<T> {BSTSplayMtEph52,1363
        fn default() -> Self { Self::new() }default53,1422
    impl<T: StTinMtT + Ord> BSTSplayMtEph<T> {BSTSplayMtEph56,1474
        pub fn new() -> Self {new57,1521
        pub fn size(&self) -> N {size63,1656
        pub fn is_empty(&self) -> B {is_empty68,1789
        pub fn height(&self) -> N {height76,1956
            fn height_rec<T: StTinMtT + Ord>(link: &Link<T>) -> N {height_rec77,1992
        pub fn insert(&self, value: T) {insert88,2342
        pub fn find(&self, target: &T) -> Option<T> {find93,2501
        pub fn contains(&self, target: &T) -> B {contains98,2671
        pub fn minimum(&self) -> Option<T> {minimum106,2861
        pub fn maximum(&self) -> Option<T> {maximum111,3013
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order116,3165
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order123,3444
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link130,3725
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate132,3808
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link134,3927
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link156,4673
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link171,5190
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link181,5511
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect191,5834
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect199,6121
    impl<T: StTinMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {BSTSplayMtEph208,6417
        fn new() -> Self { BSTSplayMtEph::new() }new209,6490
        fn insert(&self, value: T) { BSTSplayMtEph::insert(self, value) }insert211,6541
        fn find(&self, target: &T) -> Option<T> { BSTSplayMtEph::find(self, target) }find213,6616
        fn contains(&self, target: &T) -> B { BSTSplayMtEph::contains(self, target) }contains215,6703
        fn size(&self) -> N { BSTSplayMtEph::size(self) }size217,6790
        fn is_empty(&self) -> B { BSTSplayMtEph::is_empty(self) }is_empty219,6849
        fn height(&self) -> N { BSTSplayMtEph::height(self) }height221,6916
        fn minimum(&self) -> Option<T> { BSTSplayMtEph::minimum(self) }minimum223,6979
        fn maximum(&self) -> Option<T> { BSTSplayMtEph::maximum(self) }maximum225,7052
        fn in_order(&self) -> ArrayStPerS<T> { BSTSplayMtEph::in_order(self) }in_order227,7125
        fn pre_order(&self) -> ArrayStPerS<T> { BSTSplayMtEph::pre_order(self) }pre_order229,7205
    macro_rules! BSTSplayMtEphLit {BSTSplayMtEphLit233,7313
    fn _BSTSplayMtEphLit_type_checks() {_BSTSplayMtEphLit_type_checks245,7810

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap5/RelationStEphChap5_2.rs,2219
pub mod RelationStEphChap5_2 {RelationStEphChap5_23,63
    pub struct Relation<A, B> {Relation14,354
        pairs: Set<Pair<A, B>>,pairs15,386
    pub trait RelationStEphChap5_2Trait<RelationStEphChap5_2Trait18,425
        fn empty() -> Relation<X, Y>;empty25,682
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y>;FromSet29,825
        fn size(&self) -> N;size33,980
        fn domain(&self) -> Set<X>domain37,1106
        fn range(&self) -> Set<Y>range43,1274
        fn mem(&self, a: &X, b: &Y) -> Bmem49,1437
        fn iter(&self) -> Iter<'_, Pair<X, Y>>;iter54,1537
    impl<A, B> Relation<A, B> {Relation57,1592
        pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B>FromVec58,1624
    impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> PartialEq for Relation<Relation67,1878
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }eq68,1982
    impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> Eq for Relation<A, B> {Relation71,2062
    impl<A: Debug + Eq + Hash, B: Debug + Eq + Hash> Debug for Relation<A, B> {Relation73,2161
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { self.pairs.fmt(f) }fmt74,2241
    impl<A: Display + Eq + Hash, B: Display + Eq + Hash> Display for Relation<A, B> {Relation77,2325
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {fmt78,2411
    impl<X: Eq + Hash + Display + Debug + Clone + Sized, Y: Eq + Hash + Display + Debug + Clone Relation93,2831
        fn empty() -> Relation<X, Y> { Relation { pairs: SetLit![] } }empty96,3001
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y> { Relation { pairs } }FromSet98,3073
        fn size(&self) -> N { self.pairs.size() }size100,3158
        fn domain(&self) -> Set<X>domain102,3209
        fn range(&self) -> Set<Y>range113,3476
        fn mem(&self, a: &X, b: &Y) -> Bmem124,3742
        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }iter136,4018
    macro_rules! RelationLit {RelationLit140,4114
    fn _RelationLit_type_checks() {_RelationLit_type_checks156,5096
    pub fn __relation_macro_typecheck_exercise() {__relation_macro_typecheck_exercise162,5331

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap5/MappingStEphChap5_5.rs,2485
pub mod MappingStEphChap5_5 {MappingStEphChap5_53,72
    pub struct Mapping<A, B> {Mapping12,367
        rel: Relation<A, B>,rel13,398
    pub trait MappingStEphChap5_5Trait<MappingStEphChap5_5Trait16,434
        fn empty() -> Mapping<X, Y>;empty23,730
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y>;FromVec27,864
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;FromRelation31,1018
        fn size(&self) -> N;size35,1173
        fn domain(&self) -> Set<X>;domain39,1299
        fn range(&self) -> Set<Y>;range43,1432
        fn mem(&self, a: &X, b: &Y) -> B;mem47,1560
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>>;iter49,1603
    impl<A: Eq + Hash, B: Eq + Hash> Mapping<A, B> {Mapping52,1686
        fn unique_pairs_from_iter<I: IntoIterator<Item = Pair<A, B>>>(iter: I) -> Set<Pair<A, B>unique_pairs_from_iter53,1739
    impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + Mapping63,2117
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }eq66,2272
    impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + Mapping68,2347
    impl<A: std::fmt::Debug + Eq + Hash, B: std::fmt::Debug + Eq + Hash> std::fmt::Debug for MapMapping73,2502
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }fmt74,2611
    impl<A: std::fmt::Display + Eq + Hash, B: std::fmt::Display + Eq + Hash> std::fmt::Display fMapping76,2712
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }fmt77,2827
    impl<Mapping80,2929
        fn empty() -> Mapping<X, Y> {empty85,3164
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y> {FromVec91,3332
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {FromRelation98,3584
        fn size(&self) -> N { self.rel.size() }size105,3857
        fn domain(&self) -> Set<X> { self.rel.domain() }domain107,3906
        fn range(&self) -> Set<Y> { self.rel.range() }range109,3964
        fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }mem111,4020
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.rel.iter() }iter113,4085
    macro_rules! MappingLit {MappingLit117,4207
    fn _MappingLit_type_checks() {_MappingLit_type_checks128,4838
    pub fn __mapping_macro_typecheck_exercise() {__mapping_macro_typecheck_exercise134,5069

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap5/SetStEphChap5_1.rs,3609
pub mod SetStEphChap5_1 {SetStEphChap5_13,69
    pub struct Set<T> {Set11,256
        data: HashSet<T>,data12,280
    pub trait SetStEphChap5_1Trait<T: Eq + Hash + Clone + Display + Debug + Sized> {SetStEphChap5_1Trait15,313
        fn empty() -> Set<T>;empty18,490
        fn singleton(x: T) -> Set<T>;singleton21,612
        fn size(&self) -> N;size24,742
        fn mem(&self, x: &T) -> B;mem27,863
        fn union(&self, other: &Set<T>) -> Set<T>;union30,1006
        fn intersection(&self, other: &Set<T>) -> Set<T>;intersection33,1165
        fn partition(&self, parts: &Set<Set<T>>) -> B;partition36,1345
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>;CartesianProduct40,1511
        fn insert(&mut self, x: T) -> &mut Self;insert44,1690
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;iter48,1832
        fn FromVec(v: Vec<T>) -> Set<T>;FromVec51,1995
    impl<T: Eq + Hash> PartialEq for Set<T> {Set54,2043
        fn eq(&self, other: &Self) -> bool { self.data == other.data }eq55,2089
    impl<T: Eq + Hash> Eq for Set<T> {}Set58,2167
    impl<T: Eq + Hash> std::fmt::Debug for Set<T>Set60,2208
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt64,2302
    impl<T: Eq + Hash> std::fmt::Display for Set<T>Set69,2457
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt73,2555
    impl<T: Eq + Hash> Hash for Set<T> {Set89,3061
        fn hash<H: Hasher>(&self, state: &mut H) {hash90,3102
    impl<T: Eq + Hash> Set<T> {Set106,3666
        pub fn empty() -> Set<T> { Set { data: HashSet::new() } }empty107,3698
        pub fn singleton(x: T) -> Set<T> {singleton109,3765
        pub fn size(&self) -> N { self.data.len() }size115,3931
        pub fn mem(&self, x: &T) -> B {mem117,3984
        pub fn union(&self, other: &Set<T>) -> Set<T>union125,4158
        pub fn intersection(&self, other: &Set<T>) -> Set<T>intersection136,4432
        pub fn partition(&self, parts: &Set<Set<T>>) -> B {partition147,4789
        pub fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct165,5344
        pub fn insert(&mut self, x: T) -> &mut Self {insert179,5790
        pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter184,5913
        pub fn FromVec(v: Vec<T>) -> Set<T> {FromVec186,6005
    impl<T: Eq + Hash + Clone + Display + Debug + Sized> SetStEphChap5_1Trait<T> for Set<T> {Set195,6229
        fn empty() -> Set<T> { Set { data: HashSet::new() } }empty196,6323
        fn singleton(x: T) -> Set<T> {singleton198,6386
        fn size(&self) -> N { self.data.len() }size204,6548
        fn mem(&self, x: &T) -> B {mem206,6597
        fn union(&self, other: &Set<T>) -> Set<T>union214,6767
        fn intersection(&self, other: &Set<T>) -> Set<T>intersection225,7037
        fn partition(&self, parts: &Set<Set<T>>) -> B {partition236,7390
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>CartesianProduct254,7941
        fn insert(&mut self, x: T) -> &mut Self {insert268,8383
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }iter273,8502
        fn FromVec(v: Vec<T>) -> Set<T> {FromVec275,8590
    macro_rules! SetLit {SetLit285,8830
    fn _SetLit_type_checks() {_SetLit_type_checks297,9198
    pub fn __set_macro_typecheck_exercise() {__set_macro_typecheck_exercise303,9394
        let _s0: Set<&'static str> = SetLit![];str304,9440

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtPer.rs,3531
pub mod ArraySeqMtPer {ArraySeqMtPer8,390
    pub struct ArrayMtPerS<T> {ArrayMtPerS16,633
        data: Box<[T]>,data17,665
    pub trait ArraySeqMtPerTrait<T: MtT> {ArraySeqMtPerTrait21,767
        fn new(length: N, init_value: T) -> Self;new24,912
        fn length(&self) -> N;length27,1054
        fn nth(&self, index: N) -> &T;nth30,1177
        fn empty() -> Self;empty33,1308
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set36,1497
        fn singleton(item: T) -> Self;singleton41,1699
        fn isEmpty(&self) -> B;isEmpty44,1830
        fn isSingleton(&self) -> B;isSingleton47,1954
        fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy50,2092
    impl<T> ArrayMtPerS<T> {ArrayMtPerS55,2197
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq57,2267
        pub fn from_vec(v: Vec<T>) -> Self {from_vec66,2630
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter72,2770
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }iter_mut73,2833
        pub fn empty() -> Self {empty75,2912
        pub fn singleton(item: T) -> Self {singleton80,3048
        pub fn new(length: N, init_value: T) -> Selfnew85,3195
        pub fn length(&self) -> N { self.data.len() }length91,3357
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth92,3411
    impl<T: MtT> ArrayMtPerS<T> {ArrayMtPerS95,3482
        pub fn new_mt(length: N, init_value: T) -> Self {new_mt96,3516
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set104,3761
        pub fn subseq_copy_mt(&self, start: N, length: N) -> Self {subseq_copy_mt113,4101
    impl<T: MtT> Clone for ArrayMtPerS<T> {ArrayMtPerS122,4428
        fn clone(&self) -> Self {clone123,4472
    impl<T: MtT + StT + Send + Sync> MtT for ArrayMtPerS<T> {ArrayMtPerS131,4711
        type Inner = ArrayMtPerS<T>;Inner132,4773
        fn clone_mt(&self) -> Self { self.clone() }clone_mt133,4810
        fn new_mt(inner: Self::Inner) -> Self { inner }new_mt134,4862
    impl<T: Eq> PartialEq for ArrayMtPerS<T> {ArrayMtPerS137,4925
        fn eq(&self, other: &Self) -> bool {eq138,4972
    impl<T: Eq> Eq for ArrayMtPerS<T> {}ArrayMtPerS150,5298
    impl<T: Debug> Debug for ArrayMtPerS<T> {ArrayMtPerS152,5340
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt153,5386
    impl<T: Display> Display for ArrayMtPerS<T> {ArrayMtPerS159,5579
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt160,5629
    impl<T: MtT> ArraySeqMtPerTrait<T> for ArrayMtPerS<T> {ArrayMtPerS172,5950
        fn new(length: N, init_value: T) -> Self { ArrayMtPerS::new_mt(length, init_value) }new173,6010
        fn length(&self) -> N { self.data.len() }length174,6103
        fn nth(&self, index: N) -> &T { &self.data[index] }nth175,6153
        fn empty() -> Self { Self::from_vec(Vec::new()) }empty176,6213
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> { self.set(index, item) }set177,6271
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton178,6368
        fn isEmpty(&self) -> B {isEmpty179,6437
        fn isSingleton(&self) -> B {isSingleton186,6602
        fn subseq_copy(&self, start: N, length: N) -> Self { self.subseq_copy_mt(start, length) subseq_copy193,6771
    macro_rules! ArrayMtPerSLit {ArrayMtPerSLit197,6896
    fn _ArrayMtPerSLit_type_checks() {_ArrayMtPerSLit_type_checks204,7249

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTSetRBMtEph.rs,5552
pub mod BSTSetRBMtEph {BSTSetRBMtEph3,79
    pub struct BSTSetRBMtEph<T: StTinMtT + Ord> {BSTSetRBMtEph10,282
        tree: BSTRBMtEph<T>,tree11,332
    pub type BSTSetRBMt<T> = BSTSetRBMtEph<T>;BSTSetRBMt14,368
    pub trait BSTSetRBMtEphTrait<T: StTinMtT + Ord>: Sized {BSTSetRBMtEphTrait16,416
        fn empty() -> Self;empty17,477
        fn singleton(value: T) -> Self;singleton18,505
        fn size(&self) -> N;size19,545
        fn is_empty(&self) -> B;is_empty20,574
        fn find(&self, value: &T) -> Option<T>;find21,607
        fn contains(&self, value: &T) -> B;contains22,655
        fn minimum(&self) -> Option<T>;minimum23,699
        fn maximum(&self) -> Option<T>;maximum24,739
        fn insert(&mut self, value: T);insert25,779
        fn delete(&mut self, target: &T);delete26,819
        fn union(&self, other: &Self) -> Self;union27,861
        fn intersection(&self, other: &Self) -> Self;intersection28,908
        fn difference(&self, other: &Self) -> Self;difference29,962
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1014
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1069
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1124
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1186
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1256
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1324
        fn as_tree(&self) -> &BSTRBMtEph<T>;as_tree36,1375
    impl<T: StTinMtT + Ord> BSTSetRBMtEph<T> {BSTSetRBMtEph39,1427
        pub fn empty() -> Self {empty40,1474
        pub fn singleton(value: T) -> Self {singleton46,1592
        pub fn size(&self) -> N { self.tree.size() }size52,1748
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1802
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1864
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1942
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2020
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2088
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2156
        pub fn delete(&mut self, target: &T) {delete66,2229
        pub fn union(&self, other: &Self) -> Self {union74,2518
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2817
        pub fn difference(&self, other: &Self) -> Self {difference99,3395
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,3972
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4663
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,4976
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5332
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5741
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6005
        pub fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree178,6085
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6149
        fn rebuild_from_vec(values: Vec<T>) -> BSTRBMtEph<T> {rebuild_from_vec182,6240
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6457
    impl<T: StTinMtT + Ord> BSTSetRBMtEphTrait<T> for BSTSetRBMtEph<T> {BSTSetRBMtEph202,6739
        fn empty() -> Self { Self::empty() }empty203,6812
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6858
        fn size(&self) -> N { self.tree.size() }size207,6925
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,6975
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7033
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7107
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7181
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7245
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7309
        fn delete(&mut self, target: &T) { BSTSetRBMtEph::delete(self, target) }delete221,7378
        fn union(&self, other: &Self) -> Self { BSTSetRBMtEph::union(self, other) }union223,7460
        fn intersection(&self, other: &Self) -> Self { BSTSetRBMtEph::intersection(self, other) intersection225,7545
        fn difference(&self, other: &Self) -> Self { BSTSetRBMtEph::difference(self, other) }difference227,7644
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetRBMtEph::split(self, pivot) }split229,7739
        fn join_pair(left: Self, right: Self) -> Self { BSTSetRBMtEph::join_pair(left, right) }join_pair231,7832
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetRBMtEph::join_m(left, pivotjoin_m233,7929
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetRBMtEph::filter(selfilter235,8037
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetRBMtEph::reduce(self,reduce237,8150
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8260
        fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }as_tree241,8336
    macro_rules! BSTSetRBMtEphLit {BSTSetRBMtEphLit245,8422
    fn _BSTSetRBMtEphLit_type_checks() {_BSTSetRBMtEphLit_type_checks257,8924

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/AVLTreeSeqStEph.rs,4940
pub mod AVLTreeSeqStEph {AVLTreeSeqStEph3,61
    type Link<T> = Option<Box<AVLTreeNode<T>>>;Link8,193
    pub struct AVLTreeNode<T: StT> {AVLTreeNode10,242
        pub value: T,value11,279
        pub height: N,height12,301
        pub left_size: N,left_size13,324
        pub right_size: N,right_size14,350
        pub left: Link<T>,left15,377
        pub right: Link<T>,right16,404
        pub index: N,index17,432
    impl<T: StT> AVLTreeNode<T> {AVLTreeNode20,461
        fn new(value: T, index: N) -> Self {new21,495
    pub struct AVLTreeSeqStEphS<T: StT> {AVLTreeSeqStEphS34,788
        pub root: Link<T>,root35,830
        pub next_key: N,next_key36,857
    pub trait AVLTreeSeqStEphTrait<T: StT> {AVLTreeSeqStEphTrait39,889
        fn empty() -> Self;empty42,1026
        fn new() -> Self;new45,1146
        fn length(&self) -> N;length48,1264
        fn nth(&self, index: N) -> &T;nth51,1403
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set54,1550
        fn singleton(item: T) -> Self;singleton57,1723
        fn isEmpty(&self) -> B;isEmpty60,1854
        fn isSingleton(&self) -> B;isSingleton63,1978
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy66,2146
    impl<T: StT> AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS69,2213
        pub fn new_root() -> Self {new_root70,2252
        pub fn new() -> Self { Self::new_root() }new76,2400
        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqStEphS<T> {update77,2450
        pub fn from_vec(values: Vec<T>) -> AVLTreeSeqStEphS<T> {from_vec81,2657
        pub fn to_arrayseq(&self) -> ArraySeqStEphS<T> {to_arrayseq90,3040
        pub fn iter<'a>(&'a self) -> AVLTreeSeqIterStEph<'a, T> { AVLTreeSeqIterStEph::new(&selfiter106,3688
        pub fn push_back(&mut self, value: T) {push_back107,3793
        pub fn contains_value(&self, target: &T) -> B {contains_value112,4007
        pub fn insert_value(&mut self, value: T) { self.push_back(value); }insert_value120,4230
        pub fn delete_value(&mut self, target: &T) -> bool {delete_value121,4306
    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS146,5124
        fn empty() -> Self { AVLTreeSeqStEphS::new_root() }empty147,5191
        fn new() -> Self { AVLTreeSeqStEphS::new_root() }new149,5252
        fn length(&self) -> N { size_link(&self.root) }length151,5311
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }nth153,5368
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set155,5439
        fn singleton(item: T) -> Self {singleton160,5605
        fn isEmpty(&self) -> B {isEmpty166,5802
        fn isSingleton(&self) -> B {isSingleton174,5966
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy182,6134
    pub struct AVLTreeSeqIterStEph<'a, T: StT> {AVLTreeSeqIterStEph197,6655
        stack: Vec<&'a AVLTreeNode<T>>,stack198,6704
        current: Option<&'a AVLTreeNode<T>>,current199,6744
    impl<'a, T: StT> AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph202,6796
        fn new(root: &'a Link<T>) -> Self {new203,6846
        fn push_left(&mut self, link: &'a Link<T>) {push_left211,7075
    impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {AVLTreeSeqIterStEph220,7327
        type Item = &'a T;Item221,7390
        fn next(&mut self) -> Option<Self::Item> {next222,7417
    fn h<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }h231,7697
    fn size_link<T: StT>(n: &Link<T>) -> N {size_link233,7772
    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>) {update_meta241,7937
    fn rotate_right<T: StT>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_right249,8182
    fn rotate_left<T: StT>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rotate_left260,8545
    fn rebalance<T: StT>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {rebalance271,8906
    fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> &'a T {nth_link292,9671
    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str> {set_link304,10051
    pub(crate) fn insert_at_link<T: StT>(node: Link<T>, index: N, value: T, next_key: &mut N) ->insert_at_link321,10641
    macro_rules! AVLTreeNodeLit {AVLTreeNodeLit342,11457
    macro_rules! AVLTreeSeqIterStEphLit {AVLTreeSeqIterStEphLit365,12114
    fn _AVLTreeSeqStEph_struct_macro_checks() {_AVLTreeSeqStEph_struct_macro_checks375,12416
    macro_rules! AVLTreeSeqStEphSLit {AVLTreeSeqStEphSLit397,13044
    fn _AVLTreeSeqStEphSLit_type_checks() {_AVLTreeSeqStEphSLit_type_checks412,13633
    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {AVLTreeSeqStEphS417,13857
        fn eq(&self, other: &Self) -> bool {eq418,13910
    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}AVLTreeSeqStEphS431,14237

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chapter36St.rs,1495
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
        fn pivot_st_random(&self, lo: N, hi: N) -> T {pivot_st_random34,1192
        fn quick_sort_st_first(&mut self) {quick_sort_st_first40,1369
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort41,1413
        fn quick_sort_st_median3(&mut self) {quick_sort_st_median375,2596
            fn median3<T: StT + Ord>(a: &ArraySeqStEphS<T>, lo: N, hi: N) -> T {median376,2642
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort89,3179
        fn quick_sort_st_random(&mut self) {quick_sort_st_random123,4363
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {sort124,4408

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap3/InsertionSortSt.rs.saved,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/Chap3/InsertionSortSt.rs,390
pub mod InsertionSortSt {InsertionSortSt3,51
    pub trait InsertionSortStTrait {InsertionSortStTrait5,78
        fn insSort<T: Ord + Clone>(&self, slice: &mut [T]);insSort8,214
    pub struct InsertionSortSt;InsertionSortSt12,317
    impl InsertionSortStTrait for InsertionSortSt {InsertionSortSt14,350
        fn insSort<T: Ord + Clone>(&self, slice: &mut [T]) {insSort15,402

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTSetTreapMtEph.rs,5634
pub mod BSTSetTreapMtEph {BSTSetTreapMtEph3,75
    pub struct BSTSetTreapMtEph<T: StTinMtT + Ord> {BSTSetTreapMtEph10,290
        tree: BSTTreapMtEph<T>,tree11,343
    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;BSTSetTreapMt14,382
    pub trait BSTSetTreapMtEphTrait<T: StTinMtT + Ord>: Sized {BSTSetTreapMtEphTrait16,436
        fn empty() -> Self;empty17,500
        fn singleton(value: T) -> Self;singleton18,528
        fn size(&self) -> N;size19,568
        fn is_empty(&self) -> B;is_empty20,597
        fn find(&self, value: &T) -> Option<T>;find21,630
        fn contains(&self, value: &T) -> B;contains22,678
        fn minimum(&self) -> Option<T>;minimum23,722
        fn maximum(&self) -> Option<T>;maximum24,762
        fn insert(&mut self, value: T);insert25,802
        fn delete(&mut self, target: &T);delete26,842
        fn union(&self, other: &Self) -> Self;union27,884
        fn intersection(&self, other: &Self) -> Self;intersection28,931
        fn difference(&self, other: &Self) -> Self;difference29,985
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1037
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1092
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1147
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1209
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1279
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1347
        fn as_tree(&self) -> &BSTTreapMtEph<T>;as_tree36,1398
    impl<T: StTinMtT + Ord> BSTSetTreapMtEph<T> {BSTSetTreapMtEph39,1453
        pub fn empty() -> Self {empty40,1503
        pub fn singleton(value: T) -> Self {singleton46,1624
        pub fn size(&self) -> N { self.tree.size() }size52,1783
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1837
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1899
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,1977
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2055
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2123
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2191
        pub fn delete(&mut self, target: &T) {delete66,2264
        pub fn union(&self, other: &Self) -> Self {union74,2553
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2852
        pub fn difference(&self, other: &Self) -> Self {difference99,3430
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4007
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4698
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5011
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5367
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5776
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6040
        pub fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree178,6120
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6187
        fn rebuild_from_vec(values: Vec<T>) -> BSTTreapMtEph<T> {rebuild_from_vec182,6278
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6501
    impl<T: StTinMtT + Ord> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {BSTSetTreapMtEph202,6786
        fn empty() -> Self { Self::empty() }empty203,6865
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6911
        fn size(&self) -> N { self.tree.size() }size207,6978
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7028
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7086
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7160
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7234
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7298
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7362
        fn delete(&mut self, target: &T) { BSTSetTreapMtEph::delete(self, target) }delete221,7431
        fn union(&self, other: &Self) -> Self { BSTSetTreapMtEph::union(self, other) }union223,7516
        fn intersection(&self, other: &Self) -> Self { BSTSetTreapMtEph::intersection(self, otheintersection225,7604
        fn difference(&self, other: &Self) -> Self { BSTSetTreapMtEph::difference(self, other) }difference227,7706
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetTreapMtEph::split(self, pivot) }split229,7804
        fn join_pair(left: Self, right: Self) -> Self { BSTSetTreapMtEph::join_pair(left, right)join_pair231,7900
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetTreapMtEph::join_m(left, pijoin_m233,8000
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetTreapMtEph::filter(filter235,8111
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetTreapMtEph::reduce(sereduce237,8227
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8340
        fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }as_tree241,8416
    macro_rules! BSTSetTreapMtEphLit {BSTSetTreapMtEphLit245,8505
    fn _BSTSetTreapMtEphLit_type_checks() {_BSTSetTreapMtEphLit_type_checks257,9046

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStPerChap18.rs,2907
pub mod LinkedListStPerChap18 {LinkedListStPerChap183,48
    pub trait LinkedListStPerChap18Trait<T: StT> {LinkedListStPerChap18Trait7,165
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;tabulate10,384
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;map14,627
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;append18,833
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T>;filter22,1132
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;update26,1324
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject29,1490
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject32,1657
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;iterate35,1843
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes38,1978
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;reduce41,2182
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan44,2312
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;flatten47,2486
        fn collect<A: StT, Bv: StT>(collect50,2653
    impl<T: StT> LinkedListStPerChap18Trait<T> for LinkedListStPerS<T> {LinkedListStPerS56,2845
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {tabulate57,2918
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {map65,3169
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {append72,3616
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T> {filter84,4258
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T>update94,4728
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListinject102,5256
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedLisninject120,6223
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate137,7159
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LiteratePrefixes144,7496
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce153,8055
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<Tscan168,8873
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {flatten185,9790
        fn collect<A: StT, Bv: StT>(collect197,10482

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTParaTreapMtEph.rs,4821
pub mod BSTParaTreapMtEph {BSTParaTreapMtEph3,96
    pub enum Exposed<T: StTinMtT + Ord> {Exposed12,321
        Leaf,Leaf13,363
        Node(ParamTreap<T>, T, ParamTreap<T>),Node14,377
    struct NodeInner<T: StTinMtT + Ord> {NodeInner18,452
        key: T,key19,494
        priority: i64,priority20,510
        size: N,size21,533
        left: ParamTreap<T>,left22,550
        right: ParamTreap<T>,right23,579
    pub struct ParamTreap<T: StTinMtT + Ord> {ParamTreap27,637
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root28,684
    fn priority_for<T: StTinMtT + Ord>(key: &T) -> i64 {priority_for31,745
    fn tree_priority<T: StTinMtT + Ord>(tree: &ParamTreap<T>) -> i64 {tree_priority39,1047
    fn tree_size<T: StTinMtT + Ord>(tree: &ParamTreap<T>) -> N {tree_size44,1234
    fn make_node<T: StTinMtT + Ord>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreamake_node49,1404
    impl<T: StTinMtT + Ord + 'static> ParamTreap<T> {ParamTreap56,1729
        fn expose_internal(&self) -> Exposed<T> {expose_internal59,1874
        pub fn expose_with_priority(&self) -> Option<(ParamTreap<T>, T, i64, ParamTreap<T>)> {expose_with_priority69,2260
        fn join_with_priority(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) join_with_priority76,2700
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid99,3877
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner111,4349
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner132,5470
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner146,6220
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner162,6950
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner182,7866
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner202,8765
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel226,9864
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner236,10228
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel260,11297
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order270,11638
    pub trait ParamTreapTrait<T: StTinMtT + Ord + 'static>: Sized {ParamTreapTrait282,12040
        fn new() -> Self;new285,12199
        fn expose(&self) -> Exposed<T>;expose288,12316
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid291,12447
        fn size(&self) -> N;size294,12588
        fn is_empty(&self) -> B;is_empty297,12708
        fn insert(&self, key: T);insert300,12852
        fn delete(&self, key: &T);delete303,12997
        fn find(&self, key: &T) -> Option<T>;find306,13143
        fn split(&self, key: &T) -> (Self, B, Self);split309,13300
        fn join_pair(&self, other: Self) -> Self;join_pair312,13512
        fn union(&self, other: &Self) -> Self;union315,13687
        fn intersect(&self, other: &Self) -> Self;intersect318,13859
        fn difference(&self, other: &Self) -> Self;difference321,14035
        fn filter<F>(&self, predicate: F) -> Selffilter324,14192
        fn reduce<F>(&self, op: F, base: T) -> Treduce329,14416
        fn in_order(&self) -> ArrayStPerS<T>;in_order334,14632
    impl<T: StTinMtT + Ord + 'static> ParamTreapTrait<T> for ParamTreap<T> {ParamTreap337,14685
        fn new() -> Self {new340,14853
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose348,15072
        fn join_mid(exposed: Exposed<T>) -> Self { ParamTreap::join_mid(exposed) }join_mid352,15230
        fn size(&self) -> N { tree_size(self) }size356,15405
        fn is_empty(&self) -> B {is_empty360,15545
        fn insert(&self, key: T) {insert370,15819
        fn delete(&self, key: &T) {delete381,16335
        fn find(&self, key: &T) -> Option<T> {find391,16784
        fn split(&self, key: &T) -> (Self, B, Self) { ParamTreap::split_inner(self, key) }split404,17386
        fn join_pair(&self, other: Self) -> Self { ParamTreap::join_pair_inner(self.clone(), othjoin_pair408,17637
        fn union(&self, other: &Self) -> Self { ParamTreap::union_inner(self, other) }union412,17865
        fn intersect(&self, other: &Self) -> Self { ParamTreap::intersect_inner(self, other) }intersect416,18078
        fn difference(&self, other: &Self) -> Self { ParamTreap::difference_inner(self, other) }difference420,18299
        fn filter<F>(&self, predicate: F) -> Selffilter424,18502
        fn reduce<F>(&self, op: F, base: T) -> Treduce433,18804
        fn in_order(&self) -> ArrayStPerS<T> {in_order442,19097
    macro_rules! ParamTreapLit {ParamTreapLit450,19337
    fn _ParamTreapLit_type_checks() {_ParamTreapLit_type_checks462,19851

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/lib.rs,3116
pub mod Types;Types9,256
pub mod MathSeq;MathSeq12,304
pub mod Chap3 {Chap315,381
    pub mod InsertionSortSt;InsertionSortSt16,397
pub mod Chap5 {Chap520,461
    pub mod SetStEphChap5_1;SetStEphChap5_121,477
    pub mod RelationStEphChap5_2;RelationStEphChap5_222,506
    pub mod MappingStEphChap5_5;MappingStEphChap5_523,540
pub mod Chap6 {Chap629,771
    pub mod DirGraphStEph;DirGraphStEph30,787
    pub mod UnDirGraphStEph;UnDirGraphStEph31,814
    pub mod LabDirGraphStEph;LabDirGraphStEph32,843
    pub mod LabUnDirGraphStEph;LabUnDirGraphStEph33,873
pub mod LinkedListStPer;LinkedListStPer40,1148
pub mod LinkedListStPerChap18;LinkedListStPerChap1842,1225
pub mod LinkedListStPerChap19;LinkedListStPerChap1944,1320
pub mod LinkedListStEph;LinkedListStEph47,1416
pub mod LinkedListStEphChap18;LinkedListStEphChap1849,1493
pub mod LinkedListStEphChap19;LinkedListStEphChap1951,1588
pub mod ArraySeqStPer;ArraySeqStPer54,1684
pub mod ArraySeqStPerChap18;ArraySeqStPerChap1857,1756
pub mod ArraySeqStPerChap19;ArraySeqStPerChap1960,1846
pub mod ArraySeqMtPer;ArraySeqMtPer64,1971
pub mod ArraySeqMtPerChap18;ArraySeqMtPerChap1866,2042
pub mod ArraySeqMtPerChap19;ArraySeqMtPerChap1968,2131
pub mod ArraySeqStEph;ArraySeqStEph71,2221
pub mod ArraySeqStEphChap18;ArraySeqStEphChap1874,2293
pub mod ArraySeqStEphChap19;ArraySeqStEphChap1976,2382
pub mod ArraySeqMtEph;ArraySeqMtEph80,2506
pub mod ArraySeqMtEphSlice;ArraySeqMtEphSlice82,2577
pub mod ArraySeqMtEphChap18;ArraySeqMtEphChap1884,2663
pub mod ArraySeqMtEphChap19;ArraySeqMtEphChap1986,2752
pub mod AVLTreeSeqStPer;AVLTreeSeqStPer89,2842
pub mod AVLTreeSeqStPerChap18;AVLTreeSeqStPerChap1891,2919
pub mod AVLTreeSeqStPerChap19;AVLTreeSeqStPerChap1993,3014
pub mod AVLTreeSeqStEph;AVLTreeSeqStEph96,3110
pub mod AVLTreeSeqStEphChap18;AVLTreeSeqStEphChap1898,3187
pub mod AVLTreeSeqStEphChap19;AVLTreeSeqStEphChap19100,3282
pub mod Chapter36St;Chapter36St104,3403
pub mod Chapter36Mt;Chapter36Mt106,3468
pub mod Chapter36MtSlice;Chapter36MtSlice108,3533
pub mod BBTEph;BBTEph112,3641
pub mod BSTPlainStEph;BSTPlainStEph115,3707
pub mod BSTAVLStEph;BSTAVLStEph117,3830
pub mod BSTRBStEph;BSTRBStEph119,3941
pub mod BSTBBAlphaStEph;BSTBBAlphaStEph121,4046
pub mod BSTTreapStEph;BSTTreapStEph123,4181
pub mod BSTSplayStEph;BSTSplayStEph125,4304
pub mod BSTPlainMtEph;BSTPlainMtEph128,4428
pub mod BSTAVLMtEph;BSTAVLMtEph130,4551
pub mod BSTRBMtEph;BSTRBMtEph132,4662
pub mod BSTBBAlphaMtEph;BSTBBAlphaMtEph134,4767
pub mod BSTTreapMtEph;BSTTreapMtEph136,4902
pub mod BSTSplayMtEph;BSTSplayMtEph138,5025
pub mod BSTParaMtEph;BSTParaMtEph140,5148
pub mod BSTParaTreapMtEph;BSTParaTreapMtEph142,5275
pub mod BSTParaStEph;BSTParaStEph144,5431
pub mod BSTSetAVLMtEph;BSTSetAVLMtEph147,5559
pub mod BSTSetBBAlphaMtEph;BSTSetBBAlphaMtEph148,5583
pub mod BSTSetPlainMtEph;BSTSetPlainMtEph149,5611
pub mod BSTSetRBMtEph;BSTSetRBMtEph150,5637
pub mod BSTSetSplayMtEph;BSTSetSplayMtEph151,5660
pub mod BSTSetTreapMtEph;BSTSetTreapMtEph152,5686

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/#ArraySeqStPer.rs#,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTRBMtEph.rs,4517
pub mod BSTRBMtEph {BSTRBMtEph3,102
    enum Color {Color11,341
        Red,Red12,358
        Black,Black13,371
    type Link<T> = Option<Box<Node<T>>>;Link16,393
    struct Node<T: StTinMtT + Ord> {Node19,456
        key: T,key20,493
        color: Color,color21,509
        size: N,size22,531
        left: Link<T>,left23,548
        right: Link<T>,right24,571
    impl<T: StTinMtT + Ord> Node<T> {Node27,602
        fn new(key: T) -> Self {new28,640
    pub struct BSTRBMtEph<T: StTinMtT + Ord> {BSTRBMtEph40,882
        root: Arc<RwLock<Link<T>>>,root41,929
    pub type BSTreeRB<T> = BSTRBMtEph<T>;BSTreeRB44,972
    pub trait BSTRBMtEphTrait<T: StTinMtT + Ord>: Sized {BSTRBMtEphTrait46,1015
        fn new() -> Self;new47,1073
        fn insert(&self, value: T);insert48,1099
        fn find(&self, target: &T) -> Option<T>;find49,1135
        fn contains(&self, target: &T) -> B;contains50,1184
        fn size(&self) -> N;size51,1229
        fn is_empty(&self) -> B;is_empty52,1258
        fn height(&self) -> N;height53,1291
        fn minimum(&self) -> Option<T>;minimum54,1322
        fn maximum(&self) -> Option<T>;maximum55,1362
        fn in_order(&self) -> ArrayStPerS<T>;in_order56,1402
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order57,1448
    impl<T: StTinMtT + Ord> Default for BSTRBMtEph<T> {BSTRBMtEph60,1502
        fn default() -> Self { Self::new() }default61,1558
    impl<T: StTinMtT + Ord> BSTRBMtEph<T> {BSTRBMtEph64,1610
        pub fn new() -> Self {new65,1654
        pub fn size(&self) -> N {size71,1786
        pub fn is_empty(&self) -> B {is_empty76,1919
        pub fn height(&self) -> N {height84,2086
            fn height_rec<T: StTinMtT + Ord>(link: &Link<T>) -> N {height_rec85,2122
        pub fn insert(&self, value: T) {insert96,2472
        pub fn find(&self, target: &T) -> Option<T> {find104,2737
        pub fn contains(&self, target: &T) -> B {contains109,2907
        pub fn minimum(&self) -> Option<T> {minimum117,3097
        pub fn maximum(&self) -> Option<T> {maximum122,3249
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order127,3401
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order134,3680
        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Reis_red141,3961
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link143,4063
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate145,4146
        fn rotate_left(link: &mut Link<T>) {rotate_left147,4265
        fn rotate_right(link: &mut Link<T>) {rotate_right166,4925
        fn flip_colors(link: &mut Link<T>) {flip_colors185,5589
        fn fix_up(link: &mut Link<T>) {fix_up206,6394
        fn insert_link(link: &mut Link<T>, value: T) {insert_link242,7548
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link258,8090
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link273,8607
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link283,8928
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect293,9251
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect301,9538
    impl<T: StTinMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {BSTRBMtEph310,9834
        fn new() -> Self { BSTRBMtEph::new() }new311,9901
        fn insert(&self, value: T) { BSTRBMtEph::insert(self, value) }insert313,9949
        fn find(&self, target: &T) -> Option<T> { BSTRBMtEph::find(self, target) }find315,10021
        fn contains(&self, target: &T) -> B { BSTRBMtEph::contains(self, target) }contains317,10105
        fn size(&self) -> N { BSTRBMtEph::size(self) }size319,10189
        fn is_empty(&self) -> B { BSTRBMtEph::is_empty(self) }is_empty321,10245
        fn height(&self) -> N { BSTRBMtEph::height(self) }height323,10309
        fn minimum(&self) -> Option<T> { BSTRBMtEph::minimum(self) }minimum325,10369
        fn maximum(&self) -> Option<T> { BSTRBMtEph::maximum(self) }maximum327,10439
        fn in_order(&self) -> ArrayStPerS<T> { BSTRBMtEph::in_order(self) }in_order329,10509
        fn pre_order(&self) -> ArrayStPerS<T> { BSTRBMtEph::pre_order(self) }pre_order331,10586
    macro_rules! BSTRBMtEphLit {BSTRBMtEphLit335,10691
    fn _BSTRBMtEphLit_type_checks() {_BSTRBMtEphLit_type_checks347,11149

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEphChap19.rs,2712
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
        fn inject(values: &LinkedListStEphS<T>, changes: &LinkedListStEphS<Pair<N, T>>) -> Linkeinject20,1257
    impl<T: StT> LinkedListStEphChap19Trait<T> for LinkedListStEphS<T> {LinkedListStEphS23,1376
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {tabulate24,1449
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {map27,1613
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>select30,1795
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append44,2443
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {append247,2627
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T> {deflate50,2812
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T> {filter57,3123
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {iterate60,3303
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {reduce63,3488
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<Tscan66,3666
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {flatten69,3863
        fn inject(values: &LinkedListStEphS<T>, changes: &LinkedListStEphS<Pair<N, T>>) -> Linkeinject72,4039

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTParaMtEph.rs,4111
pub mod BSTParaMtEph {BSTParaMtEph3,69
    pub enum Exposed<T: StTinMtT + Ord> {Exposed10,229
        Leaf,Leaf11,271
        Node(ParamBST<T>, T, ParamBST<T>),Node12,285
    struct NodeInner<T: StTinMtT + Ord> {NodeInner16,356
        key: T,key17,398
        size: N,size18,414
        left: ParamBST<T>,left19,431
        right: ParamBST<T>,right20,458
    pub struct ParamBST<T: StTinMtT + Ord> {ParamBST24,514
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,root25,559
    pub trait ParamBSTTrait<T: StTinMtT + Ord + 'static>: Sized {ParamBSTTrait28,620
        fn new() -> Self;new31,777
        fn expose(&self) -> Exposed<T>;expose34,894
        fn join_mid(exposed: Exposed<T>) -> Self;join_mid37,1025
        fn size(&self) -> N;size40,1166
        fn is_empty(&self) -> B;is_empty43,1286
        fn insert(&self, key: T);insert46,1430
        fn delete(&self, key: &T);delete49,1575
        fn find(&self, key: &T) -> Option<T>;find52,1721
        fn split(&self, key: &T) -> (Self, B, Self);split55,1878
        fn join_pair(&self, other: Self) -> Self;join_pair58,2090
        fn union(&self, other: &Self) -> Self;union61,2265
        fn intersect(&self, other: &Self) -> Self;intersect64,2437
        fn difference(&self, other: &Self) -> Self;difference67,2613
        fn filter<F>(&self, predicate: F) -> Selffilter70,2770
        fn reduce<F>(&self, op: F, base: T) -> Treduce75,2994
        fn in_order(&self) -> ArrayStPerS<T>;in_order80,3210
    impl<T: StTinMtT + Ord + 'static> ParamBST<T> {ParamBST83,3263
        fn expose_internal(&self) -> Exposed<T> {expose_internal86,3406
        fn join_mid(exposed: Exposed<T>) -> Self {join_mid96,3796
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {split_inner110,4359
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(lejoin_m131,5435
        fn min_key(tree: &Self) -> Option<T> {min_key135,5662
        fn join_pair_inner(left: Self, right: Self) -> Self {join_pair_inner147,6177
        fn union_inner(a: &Self, b: &Self) -> Self {union_inner160,6772
        fn intersect_inner(a: &Self, b: &Self) -> Self {intersect_inner176,7489
        fn difference_inner(a: &Self, b: &Self) -> Self {difference_inner196,8442
        fn filter_inner<F>(tree: &Self, predicate: &Arc<F>) -> Selffilter_inner217,9408
        fn filter_parallel<F>(tree: &Self, predicate: F) -> Selffilter_parallel241,10480
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> Treduce_inner251,10842
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> Treduce_parallel275,11911
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {collect_in_order285,12250
    impl<T: StTinMtT + Ord + 'static> ParamBSTTrait<T> for ParamBST<T> {ParamBST297,12652
        fn new() -> Self {new300,12816
        fn expose(&self) -> Exposed<T> { self.expose_internal() }expose308,13033
        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }join_mid312,13191
        fn size(&self) -> N {size316,13364
        fn is_empty(&self) -> B {is_empty323,13602
        fn insert(&self, key: T) {insert333,13876
        fn delete(&self, key: &T) {delete343,14319
        fn find(&self, key: &T) -> Option<T> {find353,14764
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }split366,15372
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), otherjoin_pair370,15621
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }union374,15847
        fn intersect(&self, other: &Self) -> Self { ParamBST::intersect_inner(self, other) }intersect378,16058
        fn difference(&self, other: &Self) -> Self { ParamBST::difference_inner(self, other) }difference382,16277
        fn filter<F>(&self, predicate: F) -> Selffilter386,16478
        fn reduce<F>(&self, op: F, base: T) -> Treduce395,16778
        fn in_order(&self) -> ArrayStPerS<T> {in_order404,17069

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqMtEph.rs,2790
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
        pub fn length(&self) -> N {length53,1966
        pub fn nth_cloned(&self, index: N) -> T {nth_cloned59,2129
        pub fn iter_snapshot(&self) -> Iter<'_, T>iter_snapshot66,2398
        pub fn to_vec(&self) -> Vec<T> {to_vec79,2896
        pub fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy85,3089
    impl<T: StT> Clone for ArraySeqMtEphS<T> {ArraySeqMtEphS95,3453
        fn clone(&self) -> Self { ArraySeqMtEphS::from_vec(self.to_vec()) }clone96,3500
    impl<T: StT> PartialEq for ArraySeqMtEphS<T> {ArraySeqMtEphS99,3583
        fn eq(&self, other: &Self) -> bool {eq100,3634
    impl<T: StT> Eq for ArraySeqMtEphS<T> {}ArraySeqMtEphS106,3785
    impl<T: StT> Debug for ArraySeqMtEphS<T> {ArraySeqMtEphS108,3831
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt109,3878
    impl<T: StT> Display for ArraySeqMtEphS<T> {ArraySeqMtEphS115,4046
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt116,4095
    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {ArraySeqMtEphS129,4448
        fn new(length: N, init_value: T) -> Self { ArraySeqMtEphS::from_vec(vec![init_value; lennew130,4511
        fn length(&self) -> N { self.length() }length131,4615
        fn nth_cloned(&self, index: N) -> T { self.nth_cloned(index) }nth_cloned132,4663
        fn empty() -> Self { ArraySeqMtEphS::from_vec(Vec::new()) }empty133,4734
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set134,4802
        fn singleton(item: T) -> Self { ArraySeqMtEphS::from_vec(vec![item]) }singleton145,5182
        fn isEmpty(&self) -> B {isEmpty146,5261
        fn isSingleton(&self) -> B {isSingleton153,5424
        fn subseq_copy(&self, start: N, length: N) -> Self { self.subseq_copy(start, length) }subseq_copy160,5591
    macro_rules! ArraySeqMtEphSLit {ArraySeqMtEphSLit164,5713

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTSplayStEph.rs,4268
pub mod BSTSplayStEph {BSTSplayStEph3,84
    type Link<T> = Option<Box<Node<T>>>;Link8,249
    struct Node<T: StT + Ord> {Node11,312
        key: T,key12,344
        size: N,size13,360
        left: Link<T>,left14,377
        right: Link<T>,right15,400
    impl<T: StT + Ord> Node<T> {Node18,431
        fn new(key: T) -> Self {new19,464
    pub struct BSTSplayStEph<T: StT + Ord> {BSTSplayStEph29,650
        root: Link<T>,root30,695
    pub type BSTreeSplay<T> = BSTSplayStEph<T>;BSTreeSplay33,725
    pub trait BSTSplayStEphTrait<T: StT + Ord> {BSTSplayStEphTrait35,774
        fn new() -> Self;new36,823
        fn size(&self) -> N;size37,849
        fn is_empty(&self) -> B;is_empty38,878
        fn height(&self) -> N;height39,911
        fn insert(&mut self, value: T);insert40,942
        fn find(&self, target: &T) -> Option<&T>;find41,982
        fn contains(&self, target: &T) -> B;contains42,1032
        fn minimum(&self) -> Option<&T>;minimum43,1077
        fn maximum(&self) -> Option<&T>;maximum44,1118
        fn in_order(&self) -> ArrayStPerS<T>;in_order45,1159
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order46,1205
    impl<T: StT + Ord> Default for BSTSplayStEph<T> {BSTSplayStEph49,1259
        fn default() -> Self { Self::new() }default50,1313
    impl<T: StT + Ord> BSTSplayStEph<T> {BSTSplayStEph53,1365
        pub fn new() -> Self { BSTSplayStEph { root: None } }new54,1407
        pub fn size(&self) -> N { Self::size_link(&self.root) }size56,1470
        pub fn is_empty(&self) -> B {is_empty58,1535
        pub fn height(&self) -> N {height66,1702
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec67,1738
        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }insert76,2034
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find78,2124
        pub fn contains(&self, target: &T) -> B {contains80,2218
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum88,2408
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum90,2484
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order92,2560
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order98,2778
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link104,2998
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate106,3081
        fn insert_link(link: &mut Link<T>, value: T) -> bool {insert_link108,3200
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link130,3946
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link145,4463
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link155,4784
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect165,5107
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect173,5394
    impl<T: StT + Ord> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {BSTSplayStEph182,5690
        fn new() -> Self { BSTSplayStEph::new() }new183,5758
        fn size(&self) -> N { BSTSplayStEph::size(self) }size185,5809
        fn is_empty(&self) -> B { BSTSplayStEph::is_empty(self) }is_empty187,5868
        fn height(&self) -> N { BSTSplayStEph::height(self) }height189,5935
        fn insert(&mut self, value: T) { BSTSplayStEph::insert(self, value) }insert191,5998
        fn find(&self, target: &T) -> Option<&T> { BSTSplayStEph::find(self, target) }find193,6077
        fn contains(&self, target: &T) -> B { BSTSplayStEph::contains(self, target) }contains195,6165
        fn minimum(&self) -> Option<&T> { BSTSplayStEph::minimum(self) }minimum197,6252
        fn maximum(&self) -> Option<&T> { BSTSplayStEph::maximum(self) }maximum199,6326
        fn in_order(&self) -> ArrayStPerS<T> { BSTSplayStEph::in_order(self) }in_order201,6400
        fn pre_order(&self) -> ArrayStPerS<T> { BSTSplayStEph::pre_order(self) }pre_order203,6480
    macro_rules! BSTSplayStEphLit {BSTSplayStEphLit207,6588
    fn _BSTSplayStEphLit_type_checks() {_BSTSplayStEphLit_type_checks219,7089

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTAVLMtEph.rs,4241
pub mod BSTAVLMtEph {BSTAVLMtEph3,96
    type Link<T> = Option<Box<Node<T>>>;Link10,294
    struct Node<T: StTinMtT + Ord> {Node13,357
        key: T,key14,394
        height: i32,height15,410
        size: N,size16,431
        left: Link<T>,left17,448
        right: Link<T>,right18,471
    impl<T: StTinMtT + Ord> Node<T> {Node21,502
        fn new(key: T) -> Self {new22,540
    pub struct BSTAVLMtEph<T: StTinMtT + Ord> {BSTAVLMtEph34,774
        root: Arc<RwLock<Link<T>>>,root35,822
    pub type BSTreeAVL<T> = BSTAVLMtEph<T>;BSTreeAVL38,865
    pub trait BSTAVLMtEphTrait<T: StTinMtT + Ord>: Sized {BSTAVLMtEphTrait40,910
        fn new() -> Self;new41,969
        fn insert(&self, value: T);insert42,995
        fn find(&self, target: &T) -> Option<T>;find43,1031
        fn contains(&self, target: &T) -> B;contains44,1080
        fn size(&self) -> N;size45,1125
        fn is_empty(&self) -> B;is_empty46,1154
        fn height(&self) -> N;height47,1187
        fn minimum(&self) -> Option<T>;minimum48,1218
        fn maximum(&self) -> Option<T>;maximum49,1258
        fn in_order(&self) -> ArrayStPerS<T>;in_order50,1298
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order51,1344
    impl<T: StTinMtT + Ord> Default for BSTAVLMtEph<T> {BSTAVLMtEph54,1398
        fn default() -> Self { Self::new() }default55,1455
    impl<T: StTinMtT + Ord> BSTAVLMtEph<T> {BSTAVLMtEph58,1507
        pub fn new() -> Self {new59,1552
        pub fn size(&self) -> N {size65,1685
        pub fn is_empty(&self) -> B {is_empty70,1818
        pub fn height(&self) -> N {height78,1985
        pub fn insert(&self, value: T) {insert83,2127
        pub fn find(&self, target: &T) -> Option<T> {find88,2286
        pub fn contains(&self, target: &T) -> B {contains93,2456
        pub fn minimum(&self) -> Option<T> {minimum101,2646
        pub fn maximum(&self) -> Option<T> {maximum106,2798
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order111,2950
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order118,3229
        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }height_link125,3510
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link127,3599
        fn update(node: &mut Node<T>) {update129,3682
        fn rotate_right(link: &mut Link<T>) {rotate_right134,3918
        fn rotate_left(link: &mut Link<T>) {rotate_left148,4374
        fn rebalance(link: &mut Link<T>) {rebalance162,4829
        fn insert_link(link: &mut Link<T>, value: T) {insert_link187,5875
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link206,6510
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link221,7027
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link231,7349
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect241,7673
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect249,7960
    impl<T: StTinMtT + Ord> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {BSTAVLMtEph258,8256
        fn new() -> Self { BSTAVLMtEph::new() }new259,8325
        fn insert(&self, value: T) { BSTAVLMtEph::insert(self, value) }insert261,8374
        fn find(&self, target: &T) -> Option<T> { BSTAVLMtEph::find(self, target) }find263,8447
        fn contains(&self, target: &T) -> B { BSTAVLMtEph::contains(self, target) }contains265,8532
        fn size(&self) -> N { BSTAVLMtEph::size(self) }size267,8617
        fn is_empty(&self) -> B { BSTAVLMtEph::is_empty(self) }is_empty269,8674
        fn height(&self) -> N { BSTAVLMtEph::height(self) }height271,8739
        fn minimum(&self) -> Option<T> { BSTAVLMtEph::minimum(self) }minimum273,8800
        fn maximum(&self) -> Option<T> { BSTAVLMtEph::maximum(self) }maximum275,8871
        fn in_order(&self) -> ArrayStPerS<T> { BSTAVLMtEph::in_order(self) }in_order277,8942
        fn pre_order(&self) -> ArrayStPerS<T> { BSTAVLMtEph::pre_order(self) }pre_order279,9020
    macro_rules! BSTAVLMtEphLit {BSTAVLMtEphLit283,9126
    fn _BSTAVLMtEphLit_type_checks() {_BSTAVLMtEphLit_type_checks295,9597

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/LinkedListStEph.rs,3356
pub mod LinkedListStEph {LinkedListStEph3,90
    pub struct NodeE<T: StT> {NodeE7,170
        pub value: T,value8,201
        pub next: Option<Box<NodeE<T>>>,next9,223
    pub struct LinkedListStEphS<T: StT> {LinkedListStEphS13,292
        head: Option<Box<NodeE<T>>>,head14,334
        len: N,len15,371
    pub trait LinkedListStEphTrait<T: StT> {LinkedListStEphTrait18,394
        fn empty() -> LinkedListStEphS<T>;empty21,531
        fn new(length: N, init_value: T) -> Self;new24,676
        fn length(&self) -> N;length27,818
        fn nth(&self, index: N) -> &T;nth30,957
        fn isEmpty(&self) -> B;isEmpty33,1088
        fn isSingleton(&self) -> B;isSingleton36,1212
        fn singleton(item: T) -> Self;singleton39,1340
        fn update(&mut self, item_at: Pair<N, T>) -> &mut Self;update42,1487
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;set45,1659
        fn subseq_copy(&self, start: N, length: N) -> Self;subseq_copy48,1884
    impl<T: StT> LinkedListStEphS<T> {LinkedListStEphS51,1951
        fn push_front_node(&mut self, node: Box<NodeE<T>>) {push_front_node54,2082
        pub fn from_vec(v: Vec<T>) -> Self {from_vec63,2379
        pub fn iter<'a>(&'a self) -> LinkedListStEphIter<'a, T> {iter73,2736
    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {LinkedListStEphS80,2913
        fn empty() -> Self { LinkedListStEphS { head: None, len: 0 } }empty83,3072
        fn new(length: N, init_value: T) -> Self {new86,3245
        fn length(&self) -> N { self.len }length107,4018
        fn nth(&self, index: N) -> &T {nth110,4169
        fn isEmpty(&self) -> B {isEmpty124,4628
        fn isSingleton(&self) -> B {isSingleton133,4878
        fn singleton(item: T) -> Self {singleton142,5132
        fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {update153,5489
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {set168,6000
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy183,6586
    impl<T: StT> std::fmt::Debug for LinkedListStEphS<T> {LinkedListStEphS219,7768
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt220,7827
    pub struct LinkedListStEphIter<'a, T: StT> {LinkedListStEphIter231,8193
        cursor: Option<&'a NodeE<T>>,cursor232,8242
    impl<'a, T: StT> Iterator for LinkedListStEphIter<'a, T> {LinkedListStEphIter235,8287
        type Item = &'a T;Item236,8350
        fn next(&mut self) -> Option<Self::Item> {next237,8377
    impl<T: StT> PartialEq for LinkedListStEphS<T> {LinkedListStEphS247,8624
        fn eq(&self, other: &Self) -> bool {eq248,8677
    impl<T: StT> Eq for LinkedListStEphS<T> {}LinkedListStEphS265,9166
    impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {LinkedListStEphS267,9214
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {fmt268,9275
    macro_rules! NodeELit {NodeELit286,9803
    macro_rules! LinkedListStEphIterLit {LinkedListStEphIterLit296,10061
    fn _LinkedListStEph_struct_macro_checks() {_LinkedListStEph_struct_macro_checks305,10308
    macro_rules! LinkedListStEphSLit {LinkedListStEphSLit316,10672
    fn _LinkedListStEphSLit_type_checks() {_LinkedListStEphSLit_type_checks338,11970

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/ArraySeqStPer.rs,2710
pub mod ArraySeqStPer {ArraySeqStPer8,354
    pub struct ArrayStPerS<T: StT> {ArrayStPerS17,650
        data: Box<[T]>,data18,687
    pub trait ArraySeqStPerTrait<T: StT + Clone> {ArraySeqStPerTrait22,789
        fn new(length: N, init_value: T) -> Self;new25,942
        fn length(&self) -> N;length28,1084
        fn nth(&self, index: N) -> &T;nth31,1207
        fn empty() -> Self;empty34,1338
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>set37,1527
        fn singleton(item: T) -> Self;singleton42,1729
        fn isEmpty(&self) -> B;isEmpty45,1860
        fn isSingleton(&self) -> B;isSingleton48,1984
        fn subseq_copy(&self, start: N, length: N) -> Selfsubseq_copy51,2122
    impl<T: StT> ArrayStPerS<T> {ArrayStPerS56,2227
        pub fn subseq(&self, start: N, length: N) -> &[T] {subseq59,2353
        pub fn from_vec(v: Vec<T>) -> Self {from_vec68,2716
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }iter74,2856
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }iter_mut75,2919
        pub fn empty() -> Self {empty77,2998
        pub fn singleton(item: T) -> Self {singleton82,3134
        pub fn new(length: N, init_value: T) -> Selfnew87,3281
        pub fn length(&self) -> N { self.data.len() }length93,3443
        pub fn nth(&self, index: N) -> &T { &self.data[index] }nth94,3497
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str>set95,3561
    impl<T: StT + Debug> Debug for ArrayStPerS<T> {ArrayStPerS108,3921
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt109,3973
    impl<T: StT + Display> Display for ArrayStPerS<T> {ArrayStPerS115,4166
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {fmt116,4222
    impl<T: StT + Clone> ArraySeqStPerTrait<T> for ArrayStPerS<T> {ArrayStPerS128,4543
        fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) }new129,4611
        fn length(&self) -> N { self.data.len() }length130,4705
        fn nth(&self, index: N) -> &T { &self.data[index] }nth131,4755
        fn empty() -> Self { Self::from_vec(Vec::new()) }empty132,4815
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {set133,4873
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }singleton141,5178
        fn isEmpty(&self) -> B {isEmpty142,5247
        fn isSingleton(&self) -> B {isSingleton149,5412
        fn subseq_copy(&self, start: N, length: N) -> Self {subseq_copy156,5581
    macro_rules! ArrayStPerSLit {ArrayStPerSLit169,5968
    fn _ArrayStPerSLit_type_checks() {_ArrayStPerSLit_type_checks176,6321

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTPlainStEph.rs,3550
pub mod BSTPlainStEph {BSTPlainStEph3,64
    pub struct BSTPlainStEph<T: StT + Ord> {BSTPlainStEph8,208
        root: BBTree<T>,root9,253
    pub type BSTree<T> = BSTPlainStEph<T>;BSTree12,285
    pub trait BSTPlainStEphTrait<T: StT + Ord> {BSTPlainStEphTrait14,329
        fn new() -> Self;new15,378
        fn size(&self) -> N;size16,404
        fn is_empty(&self) -> B;is_empty17,433
        fn height(&self) -> N;height18,466
        fn insert(&mut self, value: T);insert19,497
        fn find(&self, target: &T) -> Option<&T>;find20,537
        fn contains(&self, target: &T) -> B;contains21,587
        fn minimum(&self) -> Option<&T>;minimum22,632
        fn maximum(&self) -> Option<&T>;maximum23,673
        fn in_order(&self) -> ArrayStPerS<T>;in_order24,714
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order25,760
    impl<T: StT + Ord> BSTPlainStEph<T> {BSTPlainStEph28,814
        pub fn new() -> Self { BSTPlainStEph { root: BBTree::leaf() } }new29,856
        pub fn size(&self) -> N { self.root.size() }size31,929
        pub fn is_empty(&self) -> B { self.root.is_leaf() }is_empty33,983
        pub fn height(&self) -> N { self.root.height() }height35,1044
        pub fn insert(&mut self, value: T) { insert_node(&mut self.root, value); }insert37,1102
        pub fn find(&self, target: &T) -> Option<&T> { find_node(&self.root, target) }find39,1186
        pub fn contains(&self, target: &T) -> B { contains_node(&self.root, target) }contains41,1274
        pub fn minimum(&self) -> Option<&T> { min_node(&self.root) }minimum43,1361
        pub fn maximum(&self) -> Option<&T> { max_node(&self.root) }maximum45,1431
        pub fn in_order(&self) -> ArrayStPerS<T> { self.root.in_order() }in_order47,1501
        pub fn pre_order(&self) -> ArrayStPerS<T> { self.root.pre_order() }pre_order49,1576
    impl<T: StT + Ord> BSTPlainStEphTrait<T> for BSTPlainStEph<T> {BSTPlainStEph52,1659
        fn new() -> Self { BSTPlainStEph::new() }new53,1727
        fn size(&self) -> N { BSTPlainStEph::size(self) }size55,1778
        fn is_empty(&self) -> B { BSTPlainStEph::is_empty(self) }is_empty57,1837
        fn height(&self) -> N { BSTPlainStEph::height(self) }height59,1904
        fn insert(&mut self, value: T) { BSTPlainStEph::insert(self, value) }insert61,1967
        fn find(&self, target: &T) -> Option<&T> { BSTPlainStEph::find(self, target) }find63,2046
        fn contains(&self, target: &T) -> B { BSTPlainStEph::contains(self, target) }contains65,2134
        fn minimum(&self) -> Option<&T> { BSTPlainStEph::minimum(self) }minimum67,2221
        fn maximum(&self) -> Option<&T> { BSTPlainStEph::maximum(self) }maximum69,2295
        fn in_order(&self) -> ArrayStPerS<T> { BSTPlainStEph::in_order(self) }in_order71,2369
        fn pre_order(&self) -> ArrayStPerS<T> { BSTPlainStEph::pre_order(self) }pre_order73,2449
    fn insert_node<T: StT + Ord>(node: &mut BBTree<T>, value: T) {insert_node76,2537
    fn contains_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> B {contains_node91,3040
    fn find_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> Option<&'a T> {find_node106,3526
    fn min_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {min_node121,4019
    fn max_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {max_node131,4339
    macro_rules! BSTPlainStEphLit {BSTPlainStEphLit142,4681
    fn _BSTPlainStEphLit_type_checks() {_BSTPlainStEphLit_type_checks157,5229

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTSetBBAlphaMtEph.rs,5687
pub mod BSTSetBBAlphaMtEph {BSTSetBBAlphaMtEph3,78
    pub struct BSTSetBBAlphaMtEph<T: StTinMtT + Ord> {BSTSetBBAlphaMtEph10,301
        tree: BSTBBAlphaMtEph<T>,tree11,356
    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;BSTSetBBAlphaMt14,397
    pub trait BSTSetBBAlphaMtEphTrait<T: StTinMtT + Ord>: Sized {BSTSetBBAlphaMtEphTrait16,455
        fn empty() -> Self;empty17,521
        fn singleton(value: T) -> Self;singleton18,549
        fn size(&self) -> N;size19,589
        fn is_empty(&self) -> B;is_empty20,618
        fn find(&self, value: &T) -> Option<T>;find21,651
        fn contains(&self, value: &T) -> B;contains22,699
        fn minimum(&self) -> Option<T>;minimum23,743
        fn maximum(&self) -> Option<T>;maximum24,783
        fn insert(&mut self, value: T);insert25,823
        fn delete(&mut self, target: &T);delete26,863
        fn union(&self, other: &Self) -> Self;union27,905
        fn intersection(&self, other: &Self) -> Self;intersection28,952
        fn difference(&self, other: &Self) -> Self;difference29,1006
        fn split(&self, pivot: &T) -> (Self, B, Self);split30,1058
        fn join_pair(left: Self, right: Self) -> Self;join_pair31,1113
        fn join_m(left: Self, pivot: T, right: Self) -> Self;join_m32,1168
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;filter33,1230
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;reduce34,1300
        fn iter_in_order(&self) -> ArrayStPerS<T>;iter_in_order35,1368
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T>;as_tree36,1419
    impl<T: StTinMtT + Ord> BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph39,1476
        pub fn empty() -> Self {empty40,1528
        pub fn singleton(value: T) -> Self {singleton46,1651
        pub fn size(&self) -> N { self.tree.size() }size52,1812
        pub fn is_empty(&self) -> B { self.tree.is_empty() }is_empty54,1866
        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find56,1928
        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains58,2006
        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum60,2084
        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum62,2152
        pub fn insert(&mut self, value: T) { self.tree.insert(value); }insert64,2220
        pub fn delete(&mut self, target: &T) {delete66,2293
        pub fn union(&self, other: &Self) -> Self {union74,2582
        pub fn intersection(&self, other: &Self) -> Self {intersection82,2881
        pub fn difference(&self, other: &Self) -> Self {difference99,3459
        pub fn split(&self, pivot: &T) -> (Self, B, Self) {split116,4036
        pub fn join_pair(left: Self, right: Self) -> Self {join_pair136,4727
        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {join_m144,5040
        pub fn filter<F>(&self, mut predicate: F) -> Selffilter153,5396
        pub fn reduce<F>(&self, mut op: F, base: T) -> Treduce166,5805
        pub fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order176,6069
        pub fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree178,6149
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }values_vec180,6218
        fn rebuild_from_vec(values: Vec<T>) -> BSTBBAlphaMtEph<T> {rebuild_from_vec182,6309
        fn from_sorted_iter<I>(values: I) -> Selffrom_sorted_iter190,6536
    impl<T: StTinMtT + Ord> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {BSTSetBBAlphaMtEph202,6823
        fn empty() -> Self { Self::empty() }empty203,6906
        fn singleton(value: T) -> Self { Self::singleton(value) }singleton205,6952
        fn size(&self) -> N { self.tree.size() }size207,7019
        fn is_empty(&self) -> B { self.tree.is_empty() }is_empty209,7069
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }find211,7127
        fn contains(&self, value: &T) -> B { self.tree.contains(value) }contains213,7201
        fn minimum(&self) -> Option<T> { self.tree.minimum() }minimum215,7275
        fn maximum(&self) -> Option<T> { self.tree.maximum() }maximum217,7339
        fn insert(&mut self, value: T) { self.tree.insert(value); }insert219,7403
        fn delete(&mut self, target: &T) { BSTSetBBAlphaMtEph::delete(self, target) }delete221,7472
        fn union(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::union(self, other) }union223,7559
        fn intersection(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::intersection(self, otintersection225,7649
        fn difference(&self, other: &Self) -> Self { BSTSetBBAlphaMtEph::difference(self, other)difference227,7753
        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetBBAlphaMtEph::split(self, pivot) }split229,7853
        fn join_pair(left: Self, right: Self) -> Self { BSTSetBBAlphaMtEph::join_pair(left, righjoin_pair231,7951
        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetBBAlphaMtEph::join_m(left, join_m233,8053
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetBBAlphaMtEph::filtefilter235,8166
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetBBAlphaMtEph::reduce(reduce237,8284
        fn iter_in_order(&self) -> ArrayStPerS<T> { self.tree.in_order() }iter_in_order239,8399
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }as_tree241,8475
    macro_rules! BSTSetBBAlphaMtEphLit {BSTSetBBAlphaMtEphLit245,8566
    fn _BSTSetBBAlphaMtEphLit_type_checks() {_BSTSetBBAlphaMtEphLit_type_checks257,9133

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BSTTreapStEph.rs,4427
pub mod BSTTreapStEph {BSTTreapStEph3,72
    type Link<T> = Option<Box<Node<T>>>;Link9,263
    struct Node<T: StT + Ord> {Node12,326
        key: T,key13,358
        priority: u64,priority14,374
        size: N,size15,397
        left: Link<T>,left16,414
        right: Link<T>,right17,437
    impl<T: StT + Ord> Node<T> {Node20,468
        fn new(key: T, priority: u64) -> Self {new21,501
    pub struct BSTTreapStEph<T: StT + Ord> {BSTTreapStEph32,728
        root: Link<T>,root33,773
    pub type BSTreeTreap<T> = BSTTreapStEph<T>;BSTreeTreap36,803
    pub trait BSTTreapStEphTrait<T: StT + Ord> {BSTTreapStEphTrait38,852
        fn new() -> Self;new39,901
        fn size(&self) -> N;size40,927
        fn is_empty(&self) -> B;is_empty41,956
        fn height(&self) -> N;height42,989
        fn insert(&mut self, value: T);insert43,1020
        fn find(&self, target: &T) -> Option<&T>;find44,1060
        fn contains(&self, target: &T) -> B;contains45,1110
        fn minimum(&self) -> Option<&T>;minimum46,1155
        fn maximum(&self) -> Option<&T>;maximum47,1196
        fn in_order(&self) -> ArrayStPerS<T>;in_order48,1237
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order49,1283
    impl<T: StT + Ord> Default for BSTTreapStEph<T> {BSTTreapStEph52,1337
        fn default() -> Self { Self::new() }default53,1391
    impl<T: StT + Ord> BSTTreapStEph<T> {BSTTreapStEph56,1443
        pub fn new() -> Self { BSTTreapStEph { root: None } }new57,1485
        pub fn size(&self) -> N { Self::size_link(&self.root) }size59,1548
        pub fn is_empty(&self) -> B {is_empty61,1613
        pub fn height(&self) -> N {height69,1780
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {height_rec70,1816
        pub fn insert(&mut self, value: T) {insert79,2112
        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }find84,2261
        pub fn contains(&self, target: &T) -> B {contains86,2355
        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }minimum94,2545
        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }maximum96,2621
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order98,2697
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order104,2915
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }size_link110,3135
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::sizeupdate112,3218
        fn rotate_left(link: &mut Link<T>) {rotate_left114,3337
        fn rotate_right(link: &mut Link<T>) {rotate_right128,3792
        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {insert_link142,4248
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {find_link167,5268
        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {min_link182,5785
        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {max_link192,6106
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {in_order_collect202,6429
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {pre_order_collect210,6716
    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {BSTTreapStEph219,7012
        fn new() -> Self { BSTTreapStEph::new() }new220,7080
        fn size(&self) -> N { BSTTreapStEph::size(self) }size222,7131
        fn is_empty(&self) -> B { BSTTreapStEph::is_empty(self) }is_empty224,7190
        fn height(&self) -> N { BSTTreapStEph::height(self) }height226,7257
        fn insert(&mut self, value: T) { BSTTreapStEph::insert(self, value) }insert228,7320
        fn find(&self, target: &T) -> Option<&T> { BSTTreapStEph::find(self, target) }find230,7399
        fn contains(&self, target: &T) -> B { BSTTreapStEph::contains(self, target) }contains232,7487
        fn minimum(&self) -> Option<&T> { BSTTreapStEph::minimum(self) }minimum234,7574
        fn maximum(&self) -> Option<&T> { BSTTreapStEph::maximum(self) }maximum236,7648
        fn in_order(&self) -> ArrayStPerS<T> { BSTTreapStEph::in_order(self) }in_order238,7722
        fn pre_order(&self) -> ArrayStPerS<T> { BSTTreapStEph::pre_order(self) }pre_order240,7802
    macro_rules! BSTTreapStEphLit {BSTTreapStEphLit244,7910
    fn _BSTTreapStEphLit_type_checks() {_BSTTreapStEphLit_type_checks256,8411

/home/milnes/APASVERUS/APAS-AI/apas-ai/src/BBTEph.rs,1944
pub mod BBTEph {BBTEph3,56
    pub enum BBTree<T: StT> {BBTree10,318
        Leaf,Leaf11,348
        Node(Box<BBNode<T>>),Node12,362
    pub struct BBNode<T: StT> {BBNode16,442
        pub left: BBTree<T>,left17,474
        pub value: T,value18,503
        pub right: BBTree<T>,right19,525
    pub trait BBTEphTrait<T: StT> {BBTEphTrait22,562
        fn leaf() -> Self;leaf23,598
        fn node(left: Self, value: T, right: Self) -> Self;node24,625
        fn is_leaf(&self) -> B;is_leaf25,685
        fn in_order(&self) -> ArrayStPerS<T>;in_order26,717
        fn pre_order(&self) -> ArrayStPerS<T>;pre_order27,763
        fn height(&self) -> N;height28,810
        fn size(&self) -> N;size29,841
    impl<T: StT> BBTree<T> {BBTree32,877
        pub fn leaf() -> Self { BBTree::Leaf }leaf33,906
        pub fn node(left: BBTree<T>, value: T, right: BBTree<T>) -> Self {node35,954
        pub fn is_leaf(&self) -> B {is_leaf39,1106
        pub fn in_order(&self) -> ArrayStPerS<T> {in_order46,1283
        pub fn pre_order(&self) -> ArrayStPerS<T> {pre_order59,1948
        pub fn height(&self) -> N {height72,2614
        pub fn size(&self) -> N {size83,2949
    impl<T: StT> BBTEphTrait<T> for BBTree<T> {BBTree91,3158
        fn leaf() -> Self { BBTree::leaf() }leaf92,3206
        fn node(left: Self, value: T, right: Self) -> Self { BBTree::node(left, value, right) }node94,3252
        fn is_leaf(&self) -> B { BBTree::is_leaf(self) }is_leaf96,3349
        fn in_order(&self) -> ArrayStPerS<T> { BBTree::in_order(self) }in_order98,3407
        fn pre_order(&self) -> ArrayStPerS<T> { BBTree::pre_order(self) }pre_order100,3480
        fn height(&self) -> N { BBTree::height(self) }height102,3555
        fn size(&self) -> N { BBTree::size(self) }size104,3611
    macro_rules! BBNodeLit {BBNodeLit108,3689
    fn _BBNodeLit_type_checks() {_BBNodeLit_type_checks119,3986

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap6/Test24DirGraphStEph.rs,129
pub mod TestDirGraphStEph {TestDirGraphStEph1,0
    fn test_digraph_vertices_and_arcs() {test_digraph_vertices_and_arcs8,218

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap6/Test26LabDirGraphStEph.rs,1037
pub mod TestLabDirGraphStEph {TestLabDirGraphStEph1,0
    fn test_labelled_dir_graph_empty() {test_labelled_dir_graph_empty8,250
    fn test_labelled_dir_graph_add_vertex() {test_labelled_dir_graph_add_vertex16,547
    fn test_labelled_dir_graph_add_labeled_arc() {test_labelled_dir_graph_add_labeled_arc28,939
    fn test_labelled_dir_graph_neighbors() {test_labelled_dir_graph_neighbors45,1565
    fn test_labelled_dir_graph_arcs() {test_labelled_dir_graph_arcs69,2451
    fn test_labelled_dir_graph_macro_empty() {test_labelled_dir_graph_macro_empty81,2848
    fn test_labelled_dir_graph_macro_with_data() {test_labelled_dir_graph_macro_with_data88,3075
    fn test_labelled_dir_graph_different_label_types() {test_labelled_dir_graph_different_label_types103,3567
    fn test_labelled_dir_graph_display() {test_labelled_dir_graph_display120,4061
    fn test_labelled_dir_graph_debug() {test_labelled_dir_graph_debug132,4415
    fn test_labelled_dir_graph_self_loop() {test_labelled_dir_graph_self_loop144,4774

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap6/Test27LabUnDirGraphStEph.rs,1329
pub mod TestLabUnDirGraphStEph {TestLabUnDirGraphStEph1,0
    fn test_labelled_undir_graph_empty() {test_labelled_undir_graph_empty8,258
    fn test_labelled_undir_graph_add_vertex() {test_labelled_undir_graph_add_vertex16,564
    fn test_labelled_undir_graph_add_labeled_edge() {test_labelled_undir_graph_add_labeled_edge28,963
    fn test_labelled_undir_graph_neighbors() {test_labelled_undir_graph_neighbors49,1897
    fn test_labelled_undir_graph_edges() {test_labelled_undir_graph_edges72,2739
    fn test_labelled_undir_graph_macro_empty() {test_labelled_undir_graph_macro_empty86,3264
    fn test_labelled_undir_graph_macro_with_data() {test_labelled_undir_graph_macro_with_data93,3498
    fn test_labelled_undir_graph_edge_normalization() {test_labelled_undir_graph_edge_normalization110,4101
    fn test_labelled_undir_graph_different_label_types() {test_labelled_undir_graph_different_label_types124,4626
    fn test_labelled_undir_graph_display() {test_labelled_undir_graph_display143,5281
    fn test_labelled_undir_graph_debug() {test_labelled_undir_graph_debug155,5641
    fn test_labelled_undir_graph_self_loop() {test_labelled_undir_graph_self_loop167,6007
    fn test_labelled_undir_graph_multiple_edges_same_vertices() {test_labelled_undir_graph_multiple_edges_same_vertices180,6450

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap6/Test25UnDirGraphStEph.rs,139
pub mod TestUnDirGraphStEph {TestUnDirGraphStEph1,0
    fn test_undigraph_vertices_and_edges() {test_undigraph_vertices_and_edges8,224

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test42BBTEph.rs,189
fn inorder_and_preorder_traversals_match_definitions() {inorder_and_preorder_traversals_match_definitions7,136
fn bst_insert_and_search_behavior() {bst_insert_and_search_behavior22,606

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test51BSTSetMtEph.rs,11099
trait TestSet: Sized {TestSet4,50
    fn empty() -> Self;empty5,73
    fn insert(&mut self, value: i32);insert6,97
    fn delete(&mut self, value: &i32);delete7,135
    fn size(&self) -> usize;size8,174
    fn is_empty(&self) -> B;is_empty9,203
    fn contains(&self, value: &i32) -> B;contains10,232
    fn minimum(&self) -> Option<i32>;minimum11,274
    fn maximum(&self) -> Option<i32>;maximum12,312
    fn union(&self, other: &Self) -> Self;union13,350
    fn intersection(&self, other: &Self) -> Self;intersection14,393
    fn difference(&self, other: &Self) -> Self;difference15,443
    fn split(&self, pivot: &i32) -> (Self, B, Self);split16,491
    fn join_pair(left: Self, right: Self) -> Self;join_pair17,544
    fn join_m(left: Self, pivot: i32, right: Self) -> Self;join_m18,595
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self;filter19,655
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32;reduce20,723
    fn iter_seq(&self) -> ArrayStPerS<i32>;iter_seq21,797
impl TestSet for apas_ai::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32> {BSTSetPlainMt24,844
    fn empty() -> Self { Self::empty() }empty25,927
    fn insert(&mut self, value: i32) { self.insert(value); }insert27,969
    fn delete(&mut self, value: &i32) { self.delete(value); }delete29,1031
    fn size(&self) -> usize { self.size() }size31,1094
    fn is_empty(&self) -> B { self.is_empty() }is_empty33,1139
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains35,1188
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum37,1255
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum39,1312
    fn union(&self, other: &Self) -> Self { self.union(other) }union41,1369
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection43,1434
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference45,1513
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split47,1588
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair49,1663
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m51,1747
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter53,1844
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce55,1939
    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }iter_seq57,2039
impl TestSet for apas_ai::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32> {BSTSetAVLMt60,2110
    fn empty() -> Self { Self::empty() }empty61,2187
    fn insert(&mut self, value: i32) { self.insert(value); }insert63,2229
    fn delete(&mut self, value: &i32) { self.delete(value); }delete65,2291
    fn size(&self) -> usize { self.size() }size67,2354
    fn is_empty(&self) -> B { self.is_empty() }is_empty69,2399
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains71,2448
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum73,2515
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum75,2572
    fn union(&self, other: &Self) -> Self { self.union(other) }union77,2629
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection79,2694
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference81,2773
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split83,2848
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair85,2923
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m87,3007
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter89,3104
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce91,3199
    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }iter_seq93,3299
impl TestSet for apas_ai::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32> {BSTSetRBMt96,3370
    fn empty() -> Self { Self::empty() }empty97,3444
    fn insert(&mut self, value: i32) { self.insert(value); }insert99,3486
    fn delete(&mut self, value: &i32) { self.delete(value); }delete101,3548
    fn size(&self) -> usize { self.size() }size103,3611
    fn is_empty(&self) -> B { self.is_empty() }is_empty105,3656
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains107,3705
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum109,3772
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum111,3829
    fn union(&self, other: &Self) -> Self { self.union(other) }union113,3886
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection115,3951
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference117,4030
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split119,4105
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair121,4180
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m123,4264
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter125,4361
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce127,4456
    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }iter_seq129,4556
impl TestSet for apas_ai::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32> {BSTSetBBAlphaMt132,4627
    fn empty() -> Self { Self::empty() }empty133,4716
    fn insert(&mut self, value: i32) { self.insert(value); }insert135,4758
    fn delete(&mut self, value: &i32) { self.delete(value); }delete137,4820
    fn size(&self) -> usize { self.size() }size139,4883
    fn is_empty(&self) -> B { self.is_empty() }is_empty141,4928
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains143,4977
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum145,5044
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum147,5101
    fn union(&self, other: &Self) -> Self { self.union(other) }union149,5158
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection151,5223
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference153,5302
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split155,5377
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair157,5452
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m159,5536
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter161,5633
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce163,5728
    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }iter_seq165,5828
impl TestSet for apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32> {BSTSetTreapMt168,5899
    fn empty() -> Self { Self::empty() }empty169,5982
    fn insert(&mut self, value: i32) { self.insert(value); }insert171,6024
    fn delete(&mut self, value: &i32) { self.delete(value); }delete173,6086
    fn size(&self) -> usize { self.size() }size175,6149
    fn is_empty(&self) -> B { self.is_empty() }is_empty177,6194
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains179,6243
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum181,6310
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum183,6367
    fn union(&self, other: &Self) -> Self { self.union(other) }union185,6424
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection187,6489
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference189,6568
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split191,6643
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair193,6718
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m195,6802
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter197,6899
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce199,6994
    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }iter_seq201,7094
impl TestSet for apas_ai::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32> {BSTSetSplayMt204,7165
    fn empty() -> Self { Self::empty() }empty205,7248
    fn insert(&mut self, value: i32) { self.insert(value); }insert207,7290
    fn delete(&mut self, value: &i32) { self.delete(value); }delete209,7352
    fn size(&self) -> usize { self.size() }size211,7415
    fn is_empty(&self) -> B { self.is_empty() }is_empty213,7460
    fn contains(&self, value: &i32) -> B { self.contains(value) }contains215,7509
    fn minimum(&self) -> Option<i32> { self.minimum() }minimum217,7576
    fn maximum(&self) -> Option<i32> { self.maximum() }maximum219,7633
    fn union(&self, other: &Self) -> Self { self.union(other) }union221,7690
    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }intersection223,7755
    fn difference(&self, other: &Self) -> Self { self.difference(other) }difference225,7834
    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }split227,7909
    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }join_pair229,7984
    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }join_m231,8068
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }filter233,8165
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base)reduce235,8260
    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }iter_seq237,8360
fn exercise_set<S: TestSet>() {exercise_set240,8431
fn test_plain_bst_set_ops() { exercise_set::<apas_ai::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSettest_plain_bst_set_ops300,10186
fn test_avl_bst_set_ops() { exercise_set::<apas_ai::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<test_avl_bst_set_ops303,10310
fn test_rb_bst_set_ops() { exercise_set::<apas_ai::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32>test_rb_bst_set_ops306,10426
fn test_bbalpha_bst_set_ops() {test_bbalpha_bst_set_ops309,10538
fn test_treap_bst_set_ops() { exercise_set::<apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSettest_treap_bst_set_ops314,10674
fn test_splay_bst_set_ops() { exercise_set::<apas_ai::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSettest_splay_bst_set_ops317,10798

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test29Algorithm_21_1.rs,428
fn points2d_tab_flat(n: N) -> ArrayStPerS<Pair<N, N>> {points2d_tab_flat13,497
fn test_points2d_n3_example() {test_points2d_n3_example26,1022
fn test_points2d_n1_empty() {test_points2d_n1_empty33,1256
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values39,1362
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order47,1561
fn test_points2d_debug_shape() {test_points2d_debug_shape68,2064

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test12ArraySeqStEph.rs,283
pub mod TestArraySeqEph {TestArraySeqEph1,0
    fn test_ephemeral_arrayseq_basic() {test_ephemeral_arrayseq_basic11,332
    fn test_ephemeral_ch18_map_append_filter() {test_ephemeral_ch18_map_append_filter20,598
    fn test_iterators_collect() {test_iterators_collect36,1213

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test18AVLTreeSeqStEph.rs,117
pub mod TestAVLTreeSeqEph {TestAVLTreeSeqEph1,0
    fn test_avl_ephemeral_basic() {test_avl_ephemeral_basic7,234

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test47BSTBBAlphaStEph.rs,162
fn bbalpha_insert_find_balances() {bbalpha_insert_find_balances5,75
fn bbalpha_duplicate_insert_is_idempotent() {bbalpha_duplicate_insert_is_idempotent25,695

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test07LinkedListStEphChap18.rs,939
pub mod TestLinkedListStEphChap18 {TestLinkedListStEphChap181,0
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list8,279
    fn test_construct_eph_from_vec() {test_construct_eph_from_vec19,648
    fn test_eph_is_empty_and_singleton() {test_eph_is_empty_and_singleton25,809
    fn test_eph_set_and_subseq_copy() {test_eph_set_and_subseq_copy33,1096
    fn test_iter_inorder_collect_eph_ch18() {test_iter_inorder_collect_eph_ch1842,1363
    fn test_tabulate_and_map_ch18() {test_tabulate_and_map_ch1848,1533
    fn test_append_ch18() {test_append_ch1855,1859
    fn test_filter_ch18() {test_filter_ch1863,2185
    fn test_update_ch18() {test_update_ch1876,2551
    fn test_inject_and_ninject_ch18() {test_inject_and_ninject_ch1883,2782
    fn test_iterate_reduce_scan_ch18() {test_iterate_reduce_scan_ch1893,3289
    fn test_flatten_and_collect_ch18() {test_flatten_and_collect_ch18105,3877

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test10ArraySeqStPerChap18.rs,949
pub mod TestArraySeqStPerChap18 {TestArraySeqStPerChap181,0
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,354
        fn fib(n: N) -> N {fib12,389
    fn test_map_increment() {test_map_increment37,1046
    fn test_subseq() {test_subseq44,1287
    fn test_append() {test_append55,1624
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,1893
    fn test_filter_even() {test_filter_even76,2522
    fn test_flatten() {test_flatten99,3322
    fn test_update_sequence() {test_update_sequence113,4092
    fn test_inject_and_ninject() {test_inject_and_ninject123,4601
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan148,5957
    fn test_iterate_sum_basic() {test_iterate_sum_basic167,6931
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum175,7203
    fn test_collect_groups_by_key() {test_collect_groups_by_key187,7664

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test01Types.rs,677
pub mod TestTypes {TestTypes1,0
    fn test_boolean_eq_neq_and_ordering() {test_boolean_eq_neq_and_ordering5,67
    fn test_ordering_on_n_naturals() {test_ordering_on_n_naturals15,324
    fn test_cmp_on_b_returns_expected_ordering_variants() {test_cmp_on_b_returns_expected_ordering_variants24,571
    fn test_btree_set_orders_b_true_before_false() {test_btree_set_orders_b_true_before_false32,895
    fn test_n_aliases_usize_and_cmp_examples() {test_n_aliases_usize_and_cmp_examples44,1344
    fn test_debug_format_for_b_variants() {test_debug_format_for_b_variants59,1797
    fn test_display_format_for_b_variants() {test_display_format_for_b_variants65,1970

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test53BSTParaTreapMtEph.rs,612
fn make_tree(values: &[i32]) -> ParamTreap<i32> {make_tree5,100
fn make_range_tree(start: i32, end: i32) -> ParamTreap<i32> {make_range_tree13,257
fn treap_basic_insert_find() {treap_basic_insert_find22,437
fn treap_split_join_pair() {treap_split_join_pair32,752
fn treap_union_intersect_difference() {treap_union_intersect_difference44,1150
fn treap_filter_reduce() {treap_filter_reduce62,1911
fn treap_join_mid_roundtrip() {treap_join_mid_roundtrip75,2307
fn treap_invariants_priority_heap() {treap_invariants_priority_heap94,2897
    fn check_heap(tree: &ParamTreap<i32>) {check_heap95,2935

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test46BSTRBStEph.rs,146
fn rb_insert_find_and_bounds() {rb_insert_find_and_bounds5,65
fn rb_duplicate_insert_is_idempotent() {rb_duplicate_insert_is_idempotent25,683

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test15AVLTreeSeqStPer.rs,215
pub mod TestAVLTreeSeqPer {TestAVLTreeSeqPer1,0
    fn test_persistent_set_does_not_mutate() {test_persistent_set_does_not_mutate8,233
    fn test_iterator_inorder_values() {test_iterator_inorder_values17,563

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test39Chapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test45BSTAVLStEph.rs,150
fn avl_insert_find_and_bounds() {avl_insert_find_and_bounds5,67
fn avl_duplicate_insert_is_idempotent() {avl_duplicate_insert_is_idempotent27,774

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test49BSTSplayStEph.rs,144
fn splay_basic_behaviour() {splay_basic_behaviour5,71
fn splay_duplicate_insert_is_idempotent() {splay_duplicate_insert_is_idempotent24,643

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test13ArraySeqStEphChap18.rs,1052
pub mod TestArraySeqStEphChap18 {TestArraySeqStEphChap183,93
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci11,370
        fn fib(n: N) -> N {fib12,405
    fn test_map_increment() {test_map_increment37,1091
    fn test_subseq() {test_subseq44,1337
    fn test_append() {test_append55,1827
    fn test_sequence_literals_and_append() {test_sequence_literals_and_append63,2104
    fn test_filter_even() {test_filter_even76,2789
    fn test_flatten() {test_flatten98,3515
    fn test_update_sequence() {test_update_sequence112,4248
    fn test_inject_conflicts_last_wins() {test_inject_conflicts_last_wins124,4763
    fn test_ninject_conflicts_last_wins() {test_ninject_conflicts_last_wins137,5533
    fn test_iterate_and_prefixes_and_reduce_and_scan() {test_iterate_and_prefixes_and_reduce_and_scan153,6297
    fn test_iterate_sum_basic() {test_iterate_sum_basic172,7325
    fn test_iterate_prefixes_sum() {test_iterate_prefixes_sum180,7603
    fn test_collect_groups_by_key() {test_collect_groups_by_key193,8082

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test17AVLTreeSeqStPerChap19.rs,383
pub mod TestAVLTreeSeqStPerChap19 {TestAVLTreeSeqStPerChap191,0
    fn test_tabulate_and_map_ch19() {test_tabulate_and_map_ch199,291
    fn test_select_and_append_ch19() {test_select_and_append_ch1917,667
    fn test_deflate_and_filter_ch19() {test_deflate_and_filter_ch1938,1571
    fn test_iter_inorder_after_pipeline_ch19() {test_iter_inorder_after_pipeline_ch1956,2319

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test35Exercsise_21_9.rs,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test38Problem21_1.rs,425
fn points2d(n: N) -> ArrayStPerS<Pair<N, N>> {points2d10,326
fn test_points2d_n3_example() {test_points2d_n3_example25,646
fn test_points2d_n1_empty() {test_points2d_n1_empty33,892
fn test_points2d_n2_basic_values() {test_points2d_n2_basic_values39,989
fn test_points2d_iterator_in_order() {test_points2d_iterator_in_order47,1171
fn test_points2d_debug_display_shapes() {test_points2d_debug_display_shapes68,1617

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test14ArraySeqStEphChap19.rs,1431
pub mod TestArraySeqStEphChap19 {TestArraySeqStEphChap193,93
    fn test_empty() {test_empty9,338
    fn test_singleton() {test_singleton15,474
    fn test_map() {test_map21,632
    fn test_append() {test_append28,876
    fn test_append2() {test_append236,1193
    fn test_deflate_true() {test_deflate_true44,1512
    fn test_deflate_false() {test_deflate_false50,1724
    fn test_filter_even_numbers() {test_filter_even_numbers56,1926
    fn test_filter_none() {test_filter_none63,2237
    fn test_update_in_bounds() {test_update_in_bounds70,2533
    fn test_update_out_of_bounds() {test_update_out_of_bounds77,2769
    fn test_isEmpty() {test_isEmpty84,3002
    fn test_isSingleton() {test_isSingleton94,3369
    fn test_iterate_sum() {test_iterate_sum104,3752
    fn test_iterate_concat() {test_iterate_concat111,3982
    fn test_map_empty() {test_map_empty125,4399
    fn test_append_with_empty() {test_append_with_empty132,4621
    fn test_append2_equivalence() {test_append2_equivalence142,5072
    fn test_filter_empty_sequence() {test_filter_empty_sequence151,5442
    fn test_select_boundary() {test_select_boundary158,5681
    fn test_subseq_basic() {test_subseq_basic169,6199
    fn test_reduce_sum_basic_ch19() {test_reduce_sum_basic_ch19176,6421
    fn test_scan_sum_basic_ch19() {test_scan_sum_basic_ch19190,7000
    fn test_flatten_ch19() {test_flatten_ch19201,7399

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test19AVLTreeSeqStEphChap18.rs,292
pub mod TestAVLTreeSeqStEphChap18 {TestAVLTreeSeqStEphChap183,79
    fn test_tabulate_inorder() {test_tabulate_inorder13,453
    fn test_map_increment() {test_map_increment19,651
    fn test_append_union() {test_append_union27,998
    fn test_filter_even() {test_filter_even41,1494

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test41ArraySeqMtEph.rs,156
fn test_arrayseq_mteph_basic_ops() {test_arrayseq_mteph_basic_ops5,61
fn test_arrayseq_mteph_append_and_map() {test_arrayseq_mteph_append_and_map22,522

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test39Chapter36St.rs,610
trait ToVec<T: StT> {ToVec5,110
    fn to_vec(&self) -> Vec<T>;to_vec6,132
impl<T: StT> ToVec<T> for ArraySeqStEphS<T> {ArraySeqStEphS8,166
    fn to_vec(&self) -> Vec<T> { (0..self.length()).map(|i| self.nth(i).clone()).collect() }to_vec9,212
fn quick_sort_variants_produce_sorted_output() {quick_sort_variants_produce_sorted_output13,316
fn quick_sort_handles_edge_cases() {quick_sort_handles_edge_cases28,840
fn pivot_strategies_match_expectations() {pivot_strategies_match_expectations51,1615
fn quick_sort_small_inputs_use_shared_pivots() {quick_sort_small_inputs_use_shared_pivots76,2390

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test26ArraySeqMtPer.rs,910
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test08LinkedListStEphChap19.rs,626
pub mod TestLinkedListStEphChap19 {TestLinkedListStEphChap191,0
    fn expect_list(list: &LinkedListStEphS<N>, expected: &[N]) {expect_list8,279
    fn test_eph_set_and_nth() {test_eph_set_and_nth17,540
    fn test_eph_subseq_copy_and_display_debug() {test_eph_subseq_copy_and_display_debug24,723
    fn test_iter_inorder_collect_eph_ch19() {test_iter_inorder_collect_eph_ch1934,1084
    fn test_tabulate_map_select_append_ch19() {test_tabulate_map_select_append_ch1940,1254
    fn test_deflate_filter_iterate_reduce_scan_flatten_inject_ch19() {test_deflate_filter_iterate_reduce_scan_flatten_inject_ch1953,1875

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test30Algorithm_21_2.rs,527
fn points3d_tab_flat(n: N) -> ArrayStPerS<Pair<N, Pair<N, N>>> {points3d_tab_flat13,554
fn test_points3d_tab_flat_n0_empty() {test_points3d_tab_flat_n0_empty36,1764
fn test_points3d_tab_flat_n1_single() {test_points3d_tab_flat_n1_single42,1879
fn test_points3d_tab_flat_n2_values_and_order() {test_points3d_tab_flat_n2_values_and_order49,2046
fn test_points3d_tab_flat_iterator_order() {test_points3d_tab_flat_iterator_order66,2472
fn test_points3d_tab_flat_debug_shape() {test_points3d_tab_flat_debug_shape85,2966

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test34Exercise_21_8_and_Algorithm_21_5.rs,347
fn is_divisible(n: N, i: N) -> B {is_divisible8,272
fn is_prime(n: N) -> B {is_prime19,543
fn primes_bf(n: N) -> ArrayStPerS<N> {primes_bf37,1170
fn test_is_prime_small_values() {test_is_prime_small_values47,1528
fn test_primes_bf_small() {test_primes_bf_small57,1805
fn test_primes_bf_debug_shape() {test_primes_bf_debug_shape64,1957

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Exercise_21_2.txt,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test50BSTMtEph.rs,309
fn mt_plain_basic_ops() {mt_plain_basic_ops10,295
fn mt_avl_basic_ops() {mt_avl_basic_ops23,636
fn mt_rb_basic_ops() {mt_rb_basic_ops34,903
fn mt_bbalpha_basic_ops() {mt_bbalpha_basic_ops44,1104
fn mt_treap_basic_ops() {mt_treap_basic_ops54,1317
fn mt_splay_basic_ops() {mt_splay_basic_ops64,1526

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test11ArraySeqStPerChap19.rs,358
pub mod TestArraySeqPerChap19 {TestArraySeqPerChap191,0
    fn test_map_and_select_and_append() {test_map_and_select_and_append11,322
    fn test_deflate_and_filter() {test_deflate_and_filter24,877
    fn test_iterate_reduce_scan_flatten() {test_iterate_reduce_scan_flatten40,1481
    fn test_inject_and_parallel() {test_inject_and_parallel59,2495

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test02MathSeq.rs,1347
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
fn is_even(x: &N) -> B {is_even10,352
fn is_vowel(c: &char) -> B {is_vowel17,452
fn pair_even_with_vowels(a: &ArrayStPerS<N>, b: &ArrayStPerS<char>) -> ArrayStPerS<Pair<N, char>pair_even_with_vowels26,741
fn test_pair_even_with_vowels_basic() {test_pair_even_with_vowels_basic44,1570
fn test_pair_even_with_vowels_debug_shape() {test_pair_even_with_vowels_debug_shape53,1866

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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap5/Test22RelationStEphChap5_2.rs,151
pub mod TestRelationStEphChap5_2 {TestRelationStEphChap5_21,0
    fn test_relation_domain_range_and_mem() {test_relation_domain_range_and_mem9,296

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap5/Test21SetStEphChap5_1.rs,575
pub mod TestSetStEphChap5_1 {TestSetStEphChap5_11,0
    fn macro_typecheck_exercise() {macro_typecheck_exercise9,220
        let _empty: Set<&'static str> = SetLit![];str10,256
    fn test_cartesian_product_example_5_1() {test_cartesian_product_example_5_116,406
    fn test_partition_example_5_2_true() {test_partition_example_5_2_true32,928
    fn test_partition_example_5_2_false_due_to_overlap() {test_partition_example_5_2_false_due_to_overlap41,1225
    fn test_partition_false_due_to_missing_element() {test_partition_false_due_to_missing_element50,1570

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap5/Test23MappingStEphChap5_5.rs,562
pub mod Test23MappingStEphChap5_5 {Test23MappingStEphChap5_53,55
    fn test_empty_mapping() {test_empty_mapping12,404
    fn test_from_vec_basic() {test_from_vec_basic20,618
    fn test_from_vec_duplicate_keys() {test_from_vec_duplicate_keys32,1125
    fn test_from_relation() {test_from_relation43,1683
    fn test_domain_and_range() {test_domain_and_range57,2476
    fn test_iter() {test_iter76,3191
    fn test_mem_comprehensive() {test_mem_comprehensive89,3675
    fn test_empty_mapping_operations() {test_empty_mapping_operations108,4344

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test37Problem_21_4.rs,1303
fn cartesian_loops(a: &ArrayStPerS<N>, b: &ArrayStPerS<&'static str>) -> ArrayStPerS<Pair<N, &'scartesian_loops12,412
    let mut v: Vec<Pair<N, &'static str>> = Vec::with_capacity(a.length() * b.length());str13,522
fn cartesian_tab_flat(a: &ArrayStPerS<N>, b: &ArrayStPerS<&'static str>) -> ArrayStPerS<Pair<N, cartesian_tab_flat24,902
    let nested: ArrayStPerS<ArrayStPerS<Pair<N, &'static str>>> =str25,1015
        <ArrayStPerS<ArrayStPerS<Pair<N, &'static str>>> as ArraySeqStPerChap19Trait<str26,1081
            ArrayStPerS<Pair<N, &'static str>>,str27,1167
                <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerChap19Trait<Pair<N, &'staticstr30,1255
                <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerChap19Trait<Pair<N, &'staticstr30,1255
    <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerChap18Trait<Pair<N, &'static str>>>::flastr37,1522
    <ArrayStPerS<Pair<N, &'static str>> as ArraySeqStPerChap18Trait<Pair<N, &'static str>>>::flastr37,1522
fn test_cartesian_loops_basic() {test_cartesian_loops_basic41,1643
fn test_cartesian_tab_flat_basic() {test_cartesian_tab_flat_basic57,2021
fn test_cartesian_iterator_order() {test_cartesian_iterator_order73,2405
fn test_cartesian_debug_shape() {test_cartesian_debug_shape82,2709

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test20AVLTreeSeqStEphChap19.rs,260
pub mod TestAVLTreeSeqStEphChap19 {TestAVLTreeSeqStEphChap193,80
    fn test_tabulate_and_map() {test_tabulate_and_map13,439
    fn test_select_and_append() {test_select_and_append21,779
    fn test_deflate_and_filter() {test_deflate_and_filter51,1839

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test27ArraySeqMtPerChap18.rs,597
pub mod Test27ArraySeqMtPerChap18 {Test27ArraySeqMtPerChap183,61
    fn test_tabulate_basic() {test_tabulate_basic10,304
    fn test_tabulate_fibonacci() {test_tabulate_fibonacci19,589
        fn fib(n: N) -> N {fib20,624
    fn test_tabulate_empty() {test_tabulate_empty51,1448
    fn test_tabulate_single() {test_tabulate_single58,1656
    fn test_tabulate_string() {test_tabulate_string65,1858
    fn test_tabulate_boolean() {test_tabulate_boolean80,2371
    fn test_tabulate_squares() {test_tabulate_squares95,2875
    fn test_tabulate_large() {test_tabulate_large110,3285

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test52BSTParaMtEph.rs,782
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,90
fn make_range_tree(start: i32, end: i32) -> ParamBST<i32> {make_range_tree13,243
fn para_basic_insert_find() {para_basic_insert_find22,419
fn para_split_and_join_pair() {para_split_and_join_pair32,733
fn para_union_and_delete() {para_union_and_delete44,1140
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip58,1559
fn para_intersect_and_difference() {para_intersect_and_difference80,2225
fn para_filter_and_reduce() {para_filter_and_reduce92,2571
fn para_union_large_balanced() {para_union_large_balanced106,2934
fn para_intersect_and_difference_large() {para_intersect_and_difference_large117,3234
fn para_filter_and_reduce_edge_cases() {para_filter_and_reduce_edge_cases133,3837

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Chap3/TestInsertionSortSt.rs,517
fn sort_and_assert(mut data: Vec<i32>, expected: &[i32]) {sort_and_assert3,41
fn insertion_sort_handles_empty() {insertion_sort_handles_empty9,194
fn insertion_sort_single_element() {insertion_sort_single_element16,363
fn insertion_sort_already_sorted() {insertion_sort_already_sorted21,449
fn insertion_sort_reverse_order() {insertion_sort_reverse_order26,557
fn insertion_sort_with_duplicates() {insertion_sort_with_duplicates31,664
fn insertion_sort_random_slice() {insertion_sort_random_slice36,773

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test05LinkedListStPerChap19.rs,439
pub mod TestLinkedListPerChap19 {TestLinkedListPerChap191,0
    fn test_select() {test_select9,289
    fn test_append_variants() {test_append_variants22,807
    fn test_deflate() {test_deflate32,1248
    fn test_map() {test_map46,1731
    fn test_iterate_and_reduce() {test_iterate_and_reduce53,1963
    fn test_scan() {test_scan62,2332
    fn test_flatten() {test_flatten70,2626
    fn test_inject() {test_inject77,2915

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test06LinkedListStEph.rs,547
pub mod TestLinkedListEph {TestLinkedListEph2,56
    fn test_empty_singleton_and_predicates() {test_empty_singleton_and_predicates10,274
    fn test_new_and_nth_set() {test_new_and_nth_set19,569
    fn test_subseq_copy() {test_subseq_copy28,808
    fn test_linkedlisteph_basic() {test_linkedlisteph_basic37,1047
    fn test_debug_format_for_eph() {test_debug_format_for_eph46,1291
    fn test_display_format_for_eph() {test_display_format_for_eph52,1447
    fn test_iter_inorder_collect_eph() {test_iter_inorder_collect_eph58,1603

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test04LinkedListStPerChap18.rs,580
pub mod TestLinkedListStPerChap18 {TestLinkedListStPerChap181,0
    fn test_tabulate() {test_tabulate9,291
    fn test_map() {test_map16,500
    fn test_filter() {test_filter24,795
    fn test_append() {test_append37,1201
    fn test_update() {test_update45,1512
    fn test_inject() {test_inject52,1767
    fn test_ninject() {test_ninject60,2088
    fn test_iterate() {test_iterate68,2413
    fn test_reduce() {test_reduce75,2645
    fn test_scan() {test_scan82,2872
    fn test_flatten() {test_flatten90,3178
    fn test_collect() {test_collect97,3502

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test09ArraySeqStPer.rs,2133
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

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test52BSTParaStEph.rs,322
fn make_tree(values: &[i32]) -> ParamBST<i32> {make_tree5,90
fn para_basic_insert_find() {para_basic_insert_find14,251
fn para_split_and_join_pair() {para_split_and_join_pair24,565
fn para_union_and_delete() {para_union_and_delete36,972
fn para_join_mid_expose_roundtrip() {para_join_mid_expose_roundtrip50,1391

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test40Chapter36Mt.rs,654
fn to_vec<T: StT>(a: &ArraySeqMtEphS<T>) -> Vec<T> { (0..a.length()).map(|i| a.nth_cloned(i)).coto_vec7,133
fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }is_sorted9,240
fn quick_sort_mt_variants_produce_sorted_output() {quick_sort_mt_variants_produce_sorted_output12,342
fn quick_sort_mt_edge_cases() {quick_sort_mt_edge_cases30,849
fn pivot_mt_strategies_match_expectations() {pivot_mt_strategies_match_expectations53,1573
fn quick_sort_mt_large_inputs() {quick_sort_mt_large_inputs74,2280
fn quick_sort_mt_small_inputs_use_shared_pivots() {quick_sort_mt_small_inputs_use_shared_pivots87,2760

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test32Exercise_21_5_and_21_6.rs,380
fn all_contiguous_subseqs<T: StT>(a: &ArrayStPerS<T>) -> ArrayStPerS<ArrayStPerS<T>> {all_contiguous_subseqs13,491
fn test_all_contiguous_subseqs_n0() {test_all_contiguous_subseqs_n032,1245
fn test_all_contiguous_subseqs_n3_values() {test_all_contiguous_subseqs_n3_values39,1416
fn test_all_contiguous_subseqs_debug_shape() {test_all_contiguous_subseqs_debug_shape48,1752

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test36Problem_21_3.rs,489
fn points3d_loops(n: N) -> ArrayStPerS<Pair<N, Pair<N, N>>> {points3d_loops10,381
fn test_points3d_loops_n0_empty() {test_points3d_loops_n0_empty27,784
fn test_points3d_loops_n1_single() {test_points3d_loops_n1_single33,893
fn test_points3d_loops_n2_values_and_order() {test_points3d_loops_n2_values_and_order40,1054
fn test_points3d_loops_iterator_order() {test_points3d_loops_iterator_order57,1474
fn test_points3d_loops_debug_shape() {test_points3d_loops_debug_shape76,1962

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test28ArraySeqMtPerChap19.rs,586
pub mod Test28ArraySeqMtPerChap19 {Test28ArraySeqMtPerChap193,61
    fn test_inject_basic() {test_inject_basic13,389
    fn test_inject_conflicting_updates() {test_inject_conflicting_updates29,1002
    fn test_inject_out_of_bounds() {test_inject_out_of_bounds45,1674
    fn test_inject_empty_changes() {test_inject_empty_changes57,2190
    fn test_inject_empty_values() {test_inject_empty_values69,2614
    fn test_atomic_write_migrated_from_st_test() {test_atomic_write_migrated_from_st_test83,3258
    fn test_inject_string_values() {test_inject_string_values108,4466

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test31Algorithm_21_6.rs,261
fn prime_sieve(n: N) -> ArrayStPerS<N> {prime_sieve13,533
fn test_prime_sieve_small() {test_prime_sieve_small61,2482
fn test_prime_sieve_n2_empty() {test_prime_sieve_n2_empty67,2600
fn test_prime_sieve_debug_shape() {test_prime_sieve_debug_shape73,2703

/home/milnes/APASVERUS/APAS-AI/apas-ai/tests/Test16AVLTreeSeqStPerChap18.rs,293
pub mod TestAVLTreeSeqStPerChap18 {TestAVLTreeSeqStPerChap183,49
    fn test_tabulate_inorder() {test_tabulate_inorder13,413
    fn test_map_increment() {test_map_increment19,647
    fn test_append_union() {test_append_union26,1004
    fn test_filter_even() {test_filter_even34,1479

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap6/BenchDirGraphStEph.rs,72
fn bench_dirgraph_build(c: &mut Criterion) {bench_dirgraph_build8,283

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap6/BenchLabUnDirGraphStEph.rs,980
fn bench_labelled_undir_graph_creation(c: &mut Criterion) {bench_labelled_undir_graph_creation7,282
fn bench_labelled_undir_graph_add_vertex(c: &mut Criterion) {bench_labelled_undir_graph_add_vertex35,1232
fn bench_labelled_undir_graph_add_labeled_edge(c: &mut Criterion) {bench_labelled_undir_graph_add_labeled_edge53,1784
fn bench_labelled_undir_graph_has_edge(c: &mut Criterion) {bench_labelled_undir_graph_has_edge71,2406
fn bench_labelled_undir_graph_get_edge_label(c: &mut Criterion) {bench_labelled_undir_graph_get_edge_label95,3190
fn bench_labelled_undir_graph_neighbors(c: &mut Criterion) {bench_labelled_undir_graph_neighbors121,4095
fn bench_labelled_undir_graph_edges(c: &mut Criterion) {bench_labelled_undir_graph_edges150,5169
fn bench_labelled_undir_graph_macro(c: &mut Criterion) {bench_labelled_undir_graph_macro170,5769
fn bench_labelled_undir_graph_edge_normalization(c: &mut Criterion) {bench_labelled_undir_graph_edge_normalization192,6385

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap6/BenchLabDirGraphStEph.rs,932
fn bench_labelled_dir_graph_creation(c: &mut Criterion) {bench_labelled_dir_graph_creation7,276
fn bench_labelled_dir_graph_add_vertex(c: &mut Criterion) {bench_labelled_dir_graph_add_vertex35,1214
fn bench_labelled_dir_graph_add_labeled_arc(c: &mut Criterion) {bench_labelled_dir_graph_add_labeled_arc53,1760
fn bench_labelled_dir_graph_has_arc(c: &mut Criterion) {bench_labelled_dir_graph_has_arc71,2372
fn bench_labelled_dir_graph_get_arc_label(c: &mut Criterion) {bench_labelled_dir_graph_get_arc_label95,3144
fn bench_labelled_dir_graph_out_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_out_neighbors121,4037
fn bench_labelled_dir_graph_in_neighbors(c: &mut Criterion) {bench_labelled_dir_graph_in_neighbors148,4985
fn bench_labelled_dir_graph_arcs(c: &mut Criterion) {bench_labelled_dir_graph_arcs175,5929
fn bench_labelled_dir_graph_macro(c: &mut Criterion) {bench_labelled_dir_graph_macro195,6517

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap6/BenchUnDirGraphStEph.rs,76
fn bench_undirgraph_build(c: &mut Criterion) {bench_undirgraph_build8,287

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStEphChap19.rs,68
fn bench_avl_eph_ch19(c: &mut Criterion) {bench_avl_eph_ch197,225

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTAVLStEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree7,207
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl15,400

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTSplayMtEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree7,215
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay15,410

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36.rs.claude,0

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36Mt.rs,131
fn gen_data(n: usize) -> ArraySeqMtEphS<i32> {gen_data7,206
fn bench_quicksort_mt(c: &mut Criterion) {bench_quicksort_mt17,541

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchMathSeq.rs,72
fn bench_mathseq_basics(c: &mut Criterion) {bench_mathseq_basics9,239

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStEphChap19.rs,86
fn bench_tabulate_map_eph_ch19(c: &mut Criterion) {bench_tabulate_map_eph_ch199,317

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPer.rs,80
fn bench_sll_persistent_ops(c: &mut Criterion) {bench_sll_persistent_ops9,263

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTTreapStEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree7,215
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap15,412

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStEph.rs,406
struct LinearCongruentialGenerator32 {LinearCongruentialGenerator3212,409
    state: u32,state13,448
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3216,467
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new17,504
    fn next_N(&mut self) -> N {next_N19,623
fn bench_build_random_s(c: &mut Criterion) {bench_build_random_s28,860

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTTreapMtEph.rs,134
fn build_tree(len: usize) -> BSTreeTreap<i32> {build_tree7,215
fn bench_bsteph_treap(c: &mut Criterion) {bench_bsteph_treap15,410

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEphChap19.rs,66
fn bench_ll_eph_ch19(c: &mut Criterion) {bench_ll_eph_ch197,232

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStPerChap18.rs,86
fn bench_tabulate_map_per_ch18(c: &mut Criterion) {bench_tabulate_map_per_ch188,274

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTBBAlphaMtEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree7,223
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha15,422

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTAVLMtEph.rs,128
fn build_tree(len: usize) -> BSTreeAVL<i32> {build_tree7,207
fn bench_bsteph_avl(c: &mut Criterion) {bench_bsteph_avl15,398

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStPer.rs,428
struct LinearCongruentialGenerator32 {LinearCongruentialGenerator3212,465
    state: u32,state13,504
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3216,523
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new17,560
    fn next_N(&mut self) -> N {next_N19,679
fn bench_build_random_s_persistent(c: &mut Criterion) {bench_build_random_s_persistent28,916

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEphChap18.rs,66
fn bench_ll_eph_ch18(c: &mut Criterion) {bench_ll_eph_ch187,232

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStEph.rs,58
fn bench_avl_eph(c: &mut Criterion) {bench_avl_eph7,225

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqMtPerChap18.rs,90
fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {bench_tabulate_map_mtper_ch187,245

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTRBMtEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree7,203
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb15,392

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter36St.rs,131
fn gen_data(n: usize) -> ArraySeqStEphS<i32> {gen_data7,206
fn bench_quicksort_st(c: &mut Criterion) {bench_quicksort_st17,538

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchChapter26Mt.rs,162
fn gen_sequence(n: usize) -> ArrayMtPerS<usize> { ArrayMtPerS::new(n, 0) }gen_sequence7,217
fn bench_chapter26_mt(c: &mut Criterion) {bench_chapter26_mt9,293

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStEphChap18.rs,68
fn bench_tabulate_map(c: &mut Criterion) {bench_tabulate_map9,317

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTSetMtEph.rs,5091
trait BenchSet: Sized {BenchSet12,705
    fn empty() -> Self;empty13,729
    fn insert_value(&mut self, value: i32);insert_value14,753
    fn union_with(&self, other: &Self) -> Self;union_with15,797
    fn difference_with(&self, other: &Self) -> Self;difference_with16,845
    fn filter_divisible_by(&self, divisor: i32) -> Self;filter_divisible_by17,898
    fn reduce_sum(&self) -> i32;reduce_sum18,955
fn build_pair<S: BenchSet>(len: usize) -> (S, S) {build_pair21,991
fn build_single<S: BenchSet>(len: usize) -> S {build_single34,1270
fn bench_set_variants<S: BenchSet>(c: &mut Criterion, label: &str) {bench_set_variants42,1431
impl BenchSet for PlainSet<i32> {PlainSet85,2894
    fn empty() -> Self { BSTSetPlainMtEphLit![] }empty86,2928
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value88,2979
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with90,3047
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with92,3117
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by94,3197
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum96,3301
impl BenchSet for AVLSet<i32> {AVLSet99,3381
    fn empty() -> Self { BSTSetAVLMtEphLit![] }empty100,3413
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value102,3462
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with104,3530
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with106,3600
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by108,3680
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum110,3784
impl BenchSet for RBSet<i32> {RBSet113,3864
    fn empty() -> Self { BSTSetRBMtEphLit![] }empty114,3895
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value116,3943
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with118,4011
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with120,4081
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by122,4161
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum124,4265
impl BenchSet for BBAlphaSet<i32> {BBAlphaSet127,4345
    fn empty() -> Self { BSTSetBBAlphaMtEphLit![] }empty128,4381
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value130,4434
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with132,4502
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with134,4572
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by136,4652
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum138,4756
impl BenchSet for TreapSet<i32> {TreapSet141,4836
    fn empty() -> Self { BSTSetTreapMtEphLit![] }empty142,4870
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value144,4921
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with146,4989
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with148,5059
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by150,5139
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum152,5243
impl BenchSet for SplaySet<i32> {SplaySet155,5323
    fn empty() -> Self { BSTSetSplayMtEphLit![] }empty156,5357
    fn insert_value(&mut self, value: i32) { self.insert(value); }insert_value158,5408
    fn union_with(&self, other: &Self) -> Self { self.union(other) }union_with160,5476
    fn difference_with(&self, other: &Self) -> Self { self.difference(other) }difference_with162,5546
    fn filter_divisible_by(&self, divisor: i32) -> Self { self.filter(|value| *value % divisor =filter_divisible_by164,5626
    fn reduce_sum(&self) -> i32 { self.reduce(|acc, value| acc + value, 0) }reduce_sum166,5730
fn bench_plain_set(c: &mut Criterion) { bench_set_variants::<PlainSet<i32>>(c, "BSTSetPlainMtEphbench_plain_set169,5810
fn bench_avl_set(c: &mut Criterion) { bench_set_variants::<AVLSet<i32>>(c, "BSTSetAVLMtEph"); }bench_avl_set171,5913
fn bench_rb_set(c: &mut Criterion) { bench_set_variants::<RBSet<i32>>(c, "BSTSetRBMtEph"); }bench_rb_set173,6010
fn bench_bbalpha_set(c: &mut Criterion) { bench_set_variants::<BBAlphaSet<i32>>(c, "BSTSetBBAlphbench_bbalpha_set175,6104
fn bench_treap_set(c: &mut Criterion) { bench_set_variants::<TreapSet<i32>>(c, "BSTSetTreapMtEphbench_treap_set177,6213
fn bench_splay_set(c: &mut Criterion) { bench_set_variants::<SplaySet<i32>>(c, "BSTSetSplayMtEphbench_splay_set179,6316

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqStPerChap19.rs,86
fn bench_tabulate_map_per_ch19(c: &mut Criterion) {bench_tabulate_map_per_ch198,274

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTParaStEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree7,205
fn bench_para_bst(c: &mut Criterion) {bench_para_bst15,392

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStPerChap19.rs,95
fn bench_build_and_read_persistent(c: &mut Criterion) {bench_build_and_read_persistent10,335

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTRBStEph.rs,125
fn build_tree(len: usize) -> BSTreeRB<i32> {build_tree7,203
fn bench_bsteph_rb(c: &mut Criterion) {bench_bsteph_rb15,394

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTParaMtEph.rs,123
fn build_tree(len: usize) -> ParamBST<i32> {build_tree7,190
fn bench_para_bst(c: &mut Criterion) {bench_para_bst15,346

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStPer.rs,80
fn bench_build_and_contains(c: &mut Criterion) {bench_build_and_contains9,270

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap5/BenchRelationStEphChap5_2.rs,106
fn bench_relation_build_and_domain_range(c: &mut Criterion) {bench_relation_build_and_domain_range8,297

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap5/BenchMappingStEphChap5_5.rs,70
fn bench_mapping_build(c: &mut Criterion) {bench_mapping_build9,362

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTPlainMtEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree7,210
fn bench_bsteph(c: &mut Criterion) {bench_bsteph17,446

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqMtPerChap19.rs,90
fn bench_tabulate_map_mtper_ch19(c: &mut Criterion) {bench_tabulate_map_mtper_ch197,245

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTBBAlphaStEph.rs,140
fn build_tree(len: usize) -> BSTreeBBAlpha<i32> {build_tree7,223
fn bench_bsteph_bbalpha(c: &mut Criterion) {bench_bsteph_bbalpha15,424

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPerChap19.rs,66
fn bench_ll_per_ch19(c: &mut Criterion) {bench_ll_per_ch198,297

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/Chap3/BenchInsertionSortSt.rs,128
fn build_vec(len: usize) -> Vec<i32> {build_vec6,207
fn bench_insertion_sort(c: &mut Criterion) {bench_insertion_sort10,285

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStPerChap18.rs,68
fn bench_avl_per_ch18(c: &mut Criterion) {bench_avl_per_ch187,232

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchAVLTreeSeqStEphChap18.rs,68
fn bench_avl_eph_ch18(c: &mut Criterion) {bench_avl_eph_ch187,225

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTParaTreapMtEph.rs,129
fn build_tree(len: usize) -> ParamTreap<i32> {build_tree7,202
fn bench_para_treap(c: &mut Criterion) {bench_para_treap15,362

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTPlainStEph.rs,117
fn build_tree(len: usize) -> BSTree<i32> {build_tree7,210
fn bench_bsteph(c: &mut Criterion) {bench_bsteph17,448

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchBSTSplayStEph.rs,134
fn build_tree(len: usize) -> BSTreeSplay<i32> {build_tree7,215
fn bench_bsteph_splay(c: &mut Criterion) {bench_bsteph_splay15,412

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStPerChap18.rs,66
fn bench_ll_per_ch18(c: &mut Criterion) {bench_ll_per_ch187,232

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchArraySeqMtPer.rs,456
struct LinearCongruentialGenerator32 {LinearCongruentialGenerator3212,465
    state: u32,state13,504
impl LinearCongruentialGenerator32 {LinearCongruentialGenerator3216,523
    fn new(seed: u32) -> LinearCongruentialGenerator32 { LinearCongruentialGenerator32 { state: new17,560
    fn next_N(&mut self) -> N {next_N19,679
fn bench_build_random_s_multithreaded_persistent(c: &mut Criterion) {bench_build_random_s_multithreaded_persistent28,916

/home/milnes/APASVERUS/APAS-AI/apas-ai/benches/BenchLinkedListStEph.rs,56
fn bench_ll_eph(c: &mut Criterion) {bench_ll_eph7,225
