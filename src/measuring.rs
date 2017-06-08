
///DEBUG DEBUG DEBUG WEEOO WEEOO WEEOO

use modes::Mode;
use super::*;
use nanos;
use verification::{verify_all};
use std::time::{Instant, Duration};
use verification::verify;
use structs::run_config::{Config, Maps};
use structs::solutions::{Candidate, Solution};
use bio::data_structures::fmindex::Interval;
use bio::data_structures::suffix_array::RawSuffixArray;
use bio::data_structures::fmindex::FMIndexable;
use search::*;


///DEBUG DEBUG DEBUG WEEOO WEEOO WEEOO
pub fn measure_solve(config : &Config, maps : &Maps, mode : Mode){
    let min_suff_len = mode.get_fewest_suff_blocks();
    let alphabet = Alphabet::new(config.alphabet());
    if config.verbosity >= 2 {
        println!("OK index alphabet set to '{}'",
                 String::from_utf8_lossy(config.alphabet()));
    }
    let sa = suffix_array(&maps.text);
    let bwt = bwt(&maps.text, &sa);
    let less = less(&bwt, &alphabet);
    let occ = Occ::new(&bwt, 3, &alphabet);
    let fm = FMIndex::new(&bwt, &less, &occ);
    if config.verbosity >= 2 {println!("OK index ready.");};

    let f = File::create(&config.output)
        .expect("Couldn't open output file.");
    let mut wrt_buf = BufWriter::new(f);

    if config.verbosity >= 2 {println!("OK output writer ready.");}

    let id_iterator = 0..maps.num_ids();
    let mut complete_solution_list : Vec<Solution> = Vec::new(); //only used in event output sorting is desired

    let config_task_completion_clone = config.track_progress;
    let num_tasks = maps.num_ids();
    let progress_tracker = thread::spawn(move || {
        track_progress(config_task_completion_clone, num_tasks);
    });
    if config.track_progress {
        if config.verbosity >= 2 {println!("OK spawning progress tracker thread.");}
    }else{
        if config.verbosity >= 2 {println!("OK suppressing progress tracker thread.");}
    }
    if config.verbosity >= 2 {println!("OK spawning {} worker threads.", config.worker_threads);}


    if config.verbosity >= 1{
        println!("OK working.");
    }
    let work_start = Instant::now();

    let mut pattern_measurements : Vec<Vec<Measurement>> = Vec::new();
    {
        let computation = |id_a|   measure_an_id(config, maps, id_a, &sa, &fm, &mode);
        let aggregator = |filter_measurements| {
            pattern_measurements.push(filter_measurements);
            if config.track_progress { ATOMIC_TASKS_DONE.fetch_add(1, Ordering::SeqCst);}
        };
        // SINGLE-THREADED mode
        //        for id_a in id_iterator{
        //            aggregator(computation(id_a));
        //        }
        cue::pipeline(
            "overlap_pipeline",          // name of the pipeline for logging
            config.worker_threads,      // number of worker threads
            id_iterator,                // iterator with work items
            computation,
            aggregator,
        );
    } // borrow of solution now returned

    if config.track_progress {
        ATOMIC_TASKS_DONE.store(num_tasks, Ordering::Relaxed);
        progress_tracker.join().is_ok();
    }
    if config.verbosity >= 2 {println!("OK output file {} written.", config.output);};

    if config.verbosity >= 1{
        println!("OK completed in {}.", approx_elapsed_string(&work_start));
    }

    let averages = measure_agglutinate(pattern_measurements);
    write_measurements(&mut wrt_buf, averages, min_suff_len);
}


///DEBUG DEBUG DEBUG WEEOO WEEOO WEEOO
#[inline]
fn measure_an_id<DBWT: DerefBWT + Clone, DLess: DerefLess + Clone, DOcc: DerefOcc + Clone>
(config : &Config, maps : &Maps, id_a : usize, sa : &RawSuffixArray,
 fm : &FMIndex<DBWT, DLess, DOcc>, mode : &Mode)
 -> Vec<Measurement>{
    fm.measures(maps.get_string(id_a), config, maps, id_a, sa, mode)
}

///DEBUG DEBUG DEBUG WEEOO WEEOO WEEOO
use std::ops::{Add,Div};
//for one filter
///DEBUG DEBUG DEBUG WEEOO WEEOO WEEOO
pub struct Measurement{
    // times
    pub search_nanos : u64,
    pub veri_true_nanos : u64,
    pub veri_false_nanos : u64,

    //counts
    pub sol_true_count : u64,
    pub sol_false_count : u64,
}

///DEBUG DEBUG DEBUG WEEOO WEEOO WEEOO
impl Add for Measurement{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Measurement {
            search_nanos : self.search_nanos + other.search_nanos,
            veri_true_nanos :  self.veri_true_nanos + other.veri_true_nanos,
            veri_false_nanos :  self.veri_false_nanos + other.veri_false_nanos,
            sol_true_count :  self.sol_true_count + other.sol_true_count,
            sol_false_count :  self.sol_false_count + other.sol_false_count,
        }
    }
}


impl Measurement{
    fn divz(self, rhs : u64) -> Measurement{
        Measurement{
            search_nanos : self.search_nanos / rhs,
            veri_true_nanos: self.veri_true_nanos / rhs,
            veri_false_nanos: self.veri_false_nanos / rhs,
            sol_true_count: self.sol_true_count / rhs,
            sol_false_count: self.sol_false_count / rhs,
        }
    }

    fn null() -> Measurement{
        Measurement{
            search_nanos : 0,
            veri_true_nanos: 0,
            veri_false_nanos: 0,
            sol_true_count: 0,
            sol_false_count: 0,
        }
    }
}



///DEBUG DEBUG DEBUG WEEOO WEEOO WEEOO
fn measure_agglutinate(pattern_measurements : Vec<Vec<Measurement>>) -> Vec<Measurement>{
    let mut agglutinated_measurements : Vec<Vec<Measurement>> = Vec::new();
    for pat in pattern_measurements {
        for (i, filter_measurements) in pat.into_iter().enumerate(){
            while agglutinated_measurements.len() <= i{
                let a : Vec<Measurement> = Vec::new();
                agglutinated_measurements.push(a)
            }
            agglutinated_measurements[i].push(filter_measurements)
        }
    }
    //now filter measurements contains measurements clumped by length
    let mut averages : Vec<Measurement> = Vec::new();
    for filter_measurements in agglutinated_measurements{
        let f_len = filter_measurements.len() as u64;
        let mut sums = Measurement{
            search_nanos : 0,
            veri_true_nanos: 0,
            veri_false_nanos: 0,
            sol_true_count: 0,
            sol_false_count: 0,
        };
        for m in filter_measurements{
            sums = sums + m;
        }
        averages.push(sums);
        //        averages.push(sums.divz(f_len))
    }
    averages
}

///DEBUG DEBUG DEBUG WEEOO WEEOO WEEOO
fn write_measurements(buf : &mut BufWriter<File>, averages : Vec<Measurement>, min_suff_len : i32){

    buf.write_all("SUFFLEN\tsrchNanos\tvTNanos\tvFNanos\tcndCount\tsTCount\tsFCount\n".as_bytes())
        .expect("couldn't write header line to output");
    for (i, m) in averages.iter().enumerate(){
        let formatted = format!("{}\t{}\t{}\t{}\t{}\t{}\n",
                                i as i32 + min_suff_len,
                                m.search_nanos,
                                m.veri_true_nanos,
                                m.veri_false_nanos,
                                m.sol_true_count,
                                m.sol_false_count,
        );
        buf.write(formatted.as_bytes()).is_ok();
    }
}



///DEBUG DEBUG DEBUG WEEOO WEEOO WEEOO
pub fn wheat_from_chaff(id_a : usize, candidates : HashSet<Candidate>, config : &Config, maps : &Maps)
                        -> (HashSet<Candidate>, HashSet<Candidate>) {

    let mut wheat : HashSet<Candidate> = HashSet::new();
    let mut chaff : HashSet<Candidate> = HashSet::new();
    for c in candidates {
        match verify(id_a, c.clone(), config, maps){
            Some(_) => wheat.insert(c),
            None => chaff.insert(c),
        };
    }
    (wheat, chaff)
}