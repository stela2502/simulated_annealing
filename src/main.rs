use clap::Parser;
use ndarray::prelude::*;


#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the data (text file)
    #[clap(default_value= "testData/data.csv",short, long)]
    data: String,
    /// the column separator for the file
    #[clap(default_value= ",",short, long)]
    sep: String,
    /// the target cluster count
    #[clap( default_value=  "testData/",short, long)]
    clusters: usize,
    /// the outpath
    #[clap(default_value=  "testData/TestClustering",short, long)]
    outpath: String,
    /// the starting temperature
    #[clap( default_value= 5.0, long)]
    t1: f32,
    /// the cooling factor
    #[clap( default_value= 0.995, long)]
    t1: f32,
    /// the number of iterations
    #[clap( default_value= 25000, long)]
    it: usize,
}



fn process_file( file:&PathBuf, sep:char, mut names:[String] ) -> ndarray {

    let mut cols = 0;
    let mut rows = 0;
    // get the data dimensions
    {
        let fi = std::fs::File::open( file ).unwrap();
        let reader = std::io::BufReader::new(fi);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    rows = line.len() -1;
                    cols +=1;
                },
                Err(err) => {
                    panic!("Unexpected error reading the csv file: {err:?}");
                }
            }
        }
    }
    let arr = Vec::<f32>::with_capacity( cols * rows );
    names = Vec::<String>::with_capacity( cols );

    let fi = std::fs::File::open( file ).unwrap();
    let reader = std::io::BufReader::new(fi);

    let mut header = true;

    for line in reader.lines() {
        if header{
            // just drop the header
            header = false;
            continue;
        }
        match line {
            Ok(line) => {
                header =true;
                for val in line.split( sep ).collect(){
                    if header{
                        names.push( val.to_string() );
                        header = false;
                    }else {
                        let v = match x.parse::<f32>() {
                            Ok( v ) => v,
                            Err(_err) => {
                                match x.parse::<usize>(){
                                    Ok(v) =>  { 
                                        r as f32
                                    },
                                    Err(_err) => {
                                        //eprintln!("I could not parse '{x}' to usize or f32 {err:?}");
                                        0.0
                                    },
                                }
                            },
                        };
                        arr.push( v );
                    }
                }
            },
            Err(err) => {
                panic!("Unexpected error reading the csv file: {err:?}");
            }
        }
    }
    array![ arr ]
}


fn main() {

    println!("Hello, world!");
}
