TODO dead code goes here that I might want to use later


RUST BIO TEST
fn doz(config : &Config){
    let text = b"ACAGCTATCGGTA";
    let filename = "data/basic.fasta";

    instantiate an alphabet
    let alphabet = alphabets::dna::iupac_alphabet();
    calculate a suffix array
    let pos = suffix_array(text);
    calculate BWT
    let bwt = bwt(text, &pos);
    calculate less and Occ
    let less = less(&bwt, &alphabet);
    let occ = Occ::new(&bwt, 3, &alphabet);
    setup FMIndex
    let fmindex = FMIndex::new(&bwt, &less, &occ);

    Iterate over a FASTQ file, use the alphabet to validate read
    sequences and search for exact matches in the FM-Index.

        obtain reader or fail with error (via the unwrap method)
    let f = File::open(&filename)
        .expect(&format!("Failed to open input file at {:?}\n", &filename));
    let reader = fasta::Reader::new(f);
    for result in reader.records() {
        obtain record or fail with error
        let record = result.unwrap();
        obtain sequence
        let seq = record.seq();
        if alphabet.is_word(seq) {
            let interval = fmindex.backward_search(seq.iter());
            let positions = interval.occ(&pos);
        }
    }
}







pub struct Filter{
    blocks : i32,
}

impl IntoIterator for Filter {
    type Item = i32;
    type IntoIter = FilterIterator;

    fn into_iter(self) -> Self::IntoIter {
        FilterIterator { blocks: self.blocks, next_index: 0 }
    }
}

pub struct FilterIterator{
    blocks : i32,
    next_index : i32,
}

impl Iterator for FilterIterator {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.next_index == self.blocks{
            None
        } else{
            self.next_index += 1;
            Some(if self.next_index == self.blocks {self.next_index-2} else {self.next_index-1})
        }
    }
}




fn iterative_candidates(&self,
                        cand_set : &mut HashSet<Candidate>,
                        cns : &SearchConstants,
                        errors : i32,
                        p_i : i32,
                        indel_balance : i32,
                        a_match_len : i32,
                        b_match_len : i32,
                        matches : &Interval,
                        debug : &str){


    let (mut l, mut r) = (0, self.bwt().len() - 1);
    let mut s = String::new();
    for &a in cns.pattern.iter().rev() {
        let less = self.less(a);
        l = less + if l > 0 { self.occ(l - 1, a) } else { 0 };
        r = less + self.occ(r, a) - 1;
        s.push(a as char);
        println!("   [{},\t{})\t({} element)::\t{}", l, r, r-l, &s);
    }

    let a = b'$';
    s.push(a as char);
    let less = self.less(a);
    l = less + if l > 0 { self.occ(l - 1, a) } else { 0 };
    r = less + self.occ(r, a);
    println!("   [{},\t{})\t({} element)::\t{}", l, r, r-l, &s);

    let x = Interval {
        lower: l,
        upper: r,
    };
    let positions = x.occ(cns.sa);
    println!("positions {:?}", &positions);
    for p in positions {
        let fetch_index = &(p as i32 + 1);
        println!("FETCH INDEX {}", fetch_index);
        let id_b = *cns.maps.bdmap_index_id.get_by_first(fetch_index).expect("DOLLAR MAP BAD");
        if id_b == cns.id_a{
            skip obvious self-matches
            println!("  !!!!!! SELF CAND {:?}, {:?}", p, debug);
            continue
        }
        let c = Candidate {
            id_b: id_b,
            overlap_a: a_match_len,
            overlap_b: b_match_len,
            overhang_right_b: 0,
        };
        println!("  !!!!!! ~~~  adding FOUND CANDIDATE AT {} with {}", p, debug);
        cand_set.insert(c);
        println!("cand set size is now {}", cand_set.len());
    }
}


for id_a in id_iterator{
aggregator(computation(id_a));
}
multithreaded part
cue::pipeline(
"overlap_pipeline",           name of the pipeline for logging
config.worker_threads,       number of worker threads
id_iterator,                 iterator with work items
computation,
aggregator,
);


let max_b_len =
if config.reversals {patt_len} else {(patt_len as f32 / (1.0 - config.err_rate)).floor() as usize};

pub mod string_walk{
    pub trait Walkable<'a>{
        fn read(&self) -> u8;
        fn can_read(&self) -> bool;
        fn advance(&mut self);
        fn new(&'a [u8]) -> Self;
    }

    #[derive (Clone)]
    pub struct ForwardWalker<'a>{
        src : &'a [u8],
        next_position : usize,
    }

    #[derive (Clone)]
    pub struct BackwardWalker<'a>{
        src : &'a [u8],
        next_position : usize,
    }

    impl<'a> Walkable<'a> for ForwardWalker<'a>{
        fn new(src : &'a [u8]) -> ForwardWalker<'a>{
            ForwardWalker{src:src, next_position:0}
        }

        fn read(&self) -> u8{
            self.src[self.next_position]
        }
        fn can_read(&self) -> bool{
            self.next_position < self.src.len()
        }
        fn advance(&mut self){
            self.next_position += 1;
        }
    }

    impl<'a> Walkable<'a> for BackwardWalker<'a>{
        fn new(src : &'a [u8]) -> BackwardWalker<'a>{
            BackwardWalker{src:src, next_position:src.len()-1}
        }
        fn read(&self) -> u8{
            self.src[self.next_position]
        }
        fn can_read(&self) -> bool{
            self.next_position < self.src.len() && self.next_position > 0
        }
        fn advance(&mut self){
            self.next_position -= 1;
        }
    }
}




let mut id_b = c.id_b;

let mut overlap_a = c.overlap_a;
let mut overlap_b = c.overlap_b;
let mut overhang_left_a = c.overhang_left_a;
let mut overhang_right_b = if overhang_left_a == 0{
(b_len as i32) - (overlap_b as i32)
(b_len as i32) - (a_len as i32) - overhang_left_a;
} else {
(b_len as i32) - (overlap_b as i32) - overhang_left_a
};
println!("!??!?  {}", overhang_right_b);


GUARANTEE 1/2: id_a <= id_b
REMEDY: vertical flip. a becomes b, b becomes a.
if id_a > id_b {

println!("VFLIP");
overhang_left_a *= -1;
overhang_right_b *= -1;
swap(&mut a_len, &mut b_len);
swap(&mut id_a, &mut id_b);
swap(&mut overlap_a, &mut overlap_b);
cigar = cigar.vflip();   I->D, D->I
}

GUARANTEE 2/2: A is a string from the input (not a flipped string)
REMEDY: horizontal flip. a becomes b, b becomes a.
if config.reversals && id_a % 2 == 1 {
println!("HFLIP");
swap(&mut overhang_left_a, &mut overhang_right_b);
overhang_left_a *= -1;
overhang_right_b *= -1;
id_a = companion_id(id_a);
id_b = companion_id(id_b);
cigar.h_flip();  XYZ -> ZYX
}
let orientation = if !config.reversals || id_b%2==0{
Orientation::Normal
}else{
Orientation::Reversed
};

INTERNAL --> EXTERNAL REPRESENTATION
swap(&mut overhang_left_a, &mut overhang_right_b);
overhang_left_a *= -1;
overhang_right_b *= -1;


Solution{
id_a : id_a,
id_b : id_b,
orientation : orientation,
overlap_a : overlap_a,
overlap_b : overlap_b,
overhang_left_a : overhang_left_a,
overhang_right_b : overhang_right_b,
errors : errors,
cigar : cigar,
}

the search guarantees this; Thus omitted from the candidate struct.
let overhang_left_a = c.overhang_left_a;
let overlap_a_start : usize = max(0, overhang_left_a) as usize;
let overlap_a_end : usize = overlap_a_start + c.overlap_a;

println!("id_a : {}", id_a);
println!("{}", maps.push_string(&maps.formatted(id_a), " ", max(0, -overhang_left_a) as usize));
println!("{}", maps.push_string(&maps.formatted(c.id_b), " ",  max(0, overhang_left_a) as usize));
stdout().flush();
let overlap_b_start : usize  = max(0, -overhang_left_a) as usize;
let overlap_b_end : usize  = overlap_b_start + c.overlap_b;
println!(" alen {} blen {}", a_len, b_len);

assert!(overlap_a_end <= a_len);
assert!(overlap_b_end <= b_len);
let a_part : &[u8] = &maps.get_string(id_a)  [overlap_a_start..overlap_a_end];
let b_part : &[u8] = &maps.get_string(c.id_b)[overlap_b_start..overlap_b_end];

println!("VERIFYING <{}> <{}>", String::from_utf8_lossy(a_part), String::from_utf8_lossy(b_part));


let overlap_a = a_match_len + cns.blind_a_chars;
let overhang_left_a : i32 = if inclusion{
[aaaa|aa]
[bbbbbbbbb?????
^    ^
<--->
- (position as i32 - index_b as i32)
} else {
[aaaaaa|aa]
<--->[b?????????
a_len as i32 - (overlap_a as i32)
};
let blind_b_chars = cns.blind_a_chars; TODO major assumption!
let overlap_b : i32 = if inclusion {
inclusion, overhang_left_a <= a
min(
(b_match_len + blind_b_chars) as i32 + overhang_left_a,
case where there is space
b_len as i32 + overhang_left_a
case where the overlap shrinks to fit
)
} else {
suff-prefix overlap, overhang_left_a >= a
min(
(b_match_len + cns.blind_a_chars) as i32,   case where there is space
(b_len - b_match_len) as i32                case where the overlap shrinks to fit
)
};
if overlap_b < 0 {
no way to overlap at all
continue;
}
let overlap_b = overlap_b as usize;

let a1 = max(0, overhang_left_a);
let a2 = overlap_a as i32;
let a3 = a_len as i32 - a1 - a2;
let b1 = max(0, -overhang_left_a);
let b2 = overlap_b as i32;
let b3 = a_len as i32 - b1 - b2;

if a3 < 0 || b3 < 0{
println!("PROBLEM! this would crash the verifier");
println!("\n\nincl: {}", inclusion);
cns.maps.print_text_debug();
println!("{}", &cns.maps.push_string(debug, " ", position));
println!("{}", &cns.maps.push_string(&"^", " ", index_b));
println!("alen {}, blen {}", a_len, b_len);
println!("{} | {} / {} | ?", overhang_left_a, a_match_len, cns.blind_a_chars);
println!("{} | {} / {} | ?", overhang_left_a, b_match_len, blind_b_chars);
println!("aovr {}, bover {}", overlap_a, overlap_b);
println!("[{}/{}/{}]", a1,a2,a3);
println!("[{}/{}/{}]", b1,b2,b3);
continue;
}


if overlap_a == a_len && overlap_b == b_len && cns.id_a > id_b {
perfect complete overlap. this one is deemed to be redundant
println!("redundant perfect overlap");
continue;
}


println!("LOHA {}", overhang_left_a);

let mut new_debug = debug.to_owned();
new_debug.push_str(&format!(" incl {} blind {}", inclusion, cns.blind_a_chars));
let c = Candidate {
id_b: id_b,
overlap_a: overlap_a,
overlap_b: overlap_b,
overhang_left_a: overhang_left_a,
debug_str : new_debug,
};
if cand_set.contains(&c){
println!("OLD {:#?}\nNEW {:#?}", c, c);
}
cand_set.insert(c);