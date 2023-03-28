
use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::Array;
use std::io::BufRead;
use ndarray::prelude::*;
use ndarray::ViewRepr;

#[derive(Debug)]
pub struct Data{
	pub rows:usize, // the amount of rows
	pub cols:usize, // the amount of cols
	pub rownames: Vec::<String>, //rge rownames of the data - we will cluster them
	pub data: ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>>,
}


impl Data {

	pub fn new( rows:usize, cols:usize, data: Vec::<f64>, rownames: Vec::<String> ) -> Self {
		//let ret = &data as &[f64]; 
		let ret = Array::from_iter(&mut data.iter().cloned());
		let data = ret.into_shape([rows, cols]).unwrap();
		Self {
			rows, 
			cols,
			rownames,
			data
		}
	}

	pub fn read_file( file:&std::string::String, sep:char ) -> Self {

	    let mut cols = 0;
	    let mut rows = 0;
	    // get the data dimensions
	    {
	        let fi = std::fs::File::open( file ).unwrap();
	        let reader = std::io::BufReader::new(fi);

	        for line in reader.lines() {
	            match line {
	                Ok(line) => {
	                    cols = line.split( sep ).count() -1;
	                    rows +=1;
	                },
	                Err(err) => {
	                    panic!("Unexpected error reading the csv file: {err:?}");
	                }
	            }
	        }
	    }
	    rows -=1;
	    println!("I got {rows} rows and {cols} cols in this data"  );

	    let mut arr  = Vec::<f64>::with_capacity( cols * rows );
	    let mut names = Vec::<String>::with_capacity( cols );

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
	                for mut val in line.split( sep ).collect::<Vec<&str>>(){
	                    if header{
	                        names.push( val.to_string() );
	                        header = false;
	                    }else {
	                    	val = val.trim();
	                        let v = match val.parse::<f64>() {
	                            Ok( v ) => v,
	                            Err(_err) => {
	                                match val.parse::<usize>(){
	                                    Ok(v) =>  { 
	                                        v as f64
	                                    },
	                                    Err(err) => {
	                                        eprintln!("I could not parse '{val}' to usize or f64 {err:?}");
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

	    Self::new( rows, cols, arr,  names )
	}

	fn sum ( data: &ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 1]>> ) -> f64 {
		let mut sum = 0.0;
		for val in data{
			sum += val;
		}
		sum
	}


	fn min ( data: &ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 1]>> ) -> f64 {
		let mut min:f64 = 1000000000.0;
		for val in data{
			if val < &min {
				min = *val;
			}
		}
		min
	}

	fn max ( data: &ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 1]>> ) -> f64 {
		let mut min:f64 = -1000000000.0;
		for val in data{
			if val > &min {
				min = *val;
			}
		}
		min
	}
	pub fn scale ( &mut self ){
		for mut row in self.data.rows_mut() {
			row -= Self::min( &row );
			row /= Self::max( &row );
		}
	}

	pub fn dist( &self, ids:&Vec<usize> ) -> f64{

		let mut sum:f64 = 0.0;

	    for i in 0..ids.len() {
	        for j in i+1..ids.len() {
	            let dist = Self::euclidean_distance(self.data.index_axis(Axis(0), ids[i]), 
	            	self.data.index_axis(Axis(0), ids[j]));
	            sum += dist;
	            //println!("{}: {}\n{}: {}", self.rownames[ids[i]], self.data.index_axis(Axis(0),ids[i]),
	            //	self.rownames[ids[j]], self.data.index_axis(Axis(0), ids[j]) ) ;
	            //println!("Euclidean distance between {} and {} is {}", ids[i], ids[j], dist);
	        }
	    }
	    sum // / ids.len() as f64
	}

	pub fn euclidean_distance(p1: ArrayView1<f64>, p2: ArrayView1<f64>) -> f64 {
		(p1.iter().zip(p2.iter()).map(|(x, y)| (x - y).powf(2.0)).sum::<f64>()).sqrt()
	}

	pub fn print ( &self ){
		println!("{}", self.data);
	}

}

#[cfg(test)]
mod tests {

    use crate::data::Data;
    use std::path::Path;

     #[test]
    fn check_dist() {

    	let mut data = Data::read_file( &"testData/Spellman_Yeast_Cell_Cycle.tsv".to_string(), '\t' );
    	data.scale();
    	let mut ids =Vec::<usize>::with_capacity(10);
    	for i in 0..10{
    		ids.push(i);
    	}
    	assert_eq!( data.dist( &ids ), 58.69013053176867 );
    }

}