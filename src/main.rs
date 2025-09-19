use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::time::SystemTime;

use simulated_annealing::data::Data;
use simulated_annealing::simulation::Simulation;

use rand::Rng;


/// Run a simulated anealing clustering over the rows of the provided data. 
/// The software is a demo project for the Lund Stem Cell Center - Bioinformatics Rust course.

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the data (text file)
    #[clap(default_value= "testData/Spellman_Yeast_Cell_Cycle.tsv",short, long)]
    data: String,
    /// the column separator for the file
    #[clap(default_value= "\\t",short, long)]
    sep: String,
    /// the target cluster count
    #[clap( default_value_t=  10,short, long)]
    clusters: usize,
    /// the outpath
    #[clap(default_value=  "testData/TestClustering",short, long)]
    outpath: String,
    /// the starting temperature
    #[clap( default_value_t= 20.0, long)]
    t1: f64,
    /// the cooling factor
    #[clap( default_value_t= 0.9995, long)]
    cf: f64,
    /// the number of iterations
    #[clap( default_value_t= 25000, long)]
    it: usize,
    /// a starting grouping
    #[clap( default_value= "testData/RFclustered.txt", long)]
    start: String,
}





fn main() {
    let now = SystemTime::now();
    
    let opts: Opts = Opts::parse();
    let mut sep = '\t';
    if &opts.sep != "\\t"{
        println!("I set sep to {}", opts.sep );
        sep = opts.sep.chars().next().unwrap(); 
    }

    let mut data = Data::read_file( &opts.data , sep );

    let mut rng = rand::thread_rng();
    let mut t:f64 = opts.t1;

    //data.print();
    data.scale();
    //data.print();

    let mut sim = Simulation::new( data, &opts.clusters, opts.start );
    let mut old_energy = sim.calc_energy( ) / opts.clusters as f64;
    let mut new_energy = 0.0;
    let mut shifts = 0;
    let mut rand:f64;// = rng.gen::<f64>();
    println!( "Starting energy is {old_energy:0.2}");
    let mut doit:bool;
    for _i in 0..opts.it {
        sim.switch_row();
        new_energy = sim.calc_energy( ) / opts.clusters as f64;
        doit = false;
        rand = rng.gen::<f64>();
        
        if new_energy < old_energy || libm::exp( -( (new_energy - old_energy) / t ) ) > rand {
            sim.fixate();
            shifts += 1;
            old_energy = new_energy;
        }else {
            sim.rinse();
            new_energy = old_energy;
        }
        
        t *= opts.cf;
    }


    fs::create_dir_all(&opts.outpath).expect("AlreadyExists");
    let mut fp = PathBuf::from(&opts.outpath);
    fp.push( format!("SimulatedAnealing_k_{}_t1_{}_cf_{}_it_{}.tsv", opts.clusters, opts.t1, opts.cf, opts.it ) );
    
    sim.write( &fp );
    sim.plot(&opts.outpath );

    match now.elapsed() {
        Ok(elapsed) => {
            let mut milli = elapsed.as_millis();

            let mil = milli % 1000;
            milli= (milli - mil) /1000;

            let sec = milli % 60;
            milli= (milli -sec) /60;

            let min = milli % 60;
            milli= (milli -min) /60;

            println!("finished in {milli} h {min} min {sec} sec {mil} milli sec - end energy was {new_energy:0.2} with {shifts} gene shifts ({:0.2}%) and end t1 = {t:0.2}\n", shifts as f32 / opts.it as f32 );},
       Err(e) => {println!("Error: {e:?}");}
    }

    let fname = fp.to_str().unwrap();
    println!( "Clustering written to {fname}");
}
