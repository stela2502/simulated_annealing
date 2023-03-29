use crate::data::Data;
use rand::Rng;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct Simulation{
	pub data:Data, // the data
	pub last:Vec<usize>, // the 'old' clusters
	pub new:Vec<usize>, // the 'new' clusters
	pub energy: f64, // the current energy
	pub energy_array: Vec<f64>,
	pub changed: [usize;2],
	pub k: usize,
	row_c: usize,
	old_energy: [f64;2],
}

impl Simulation{
	pub fn new( data:Data, k:&usize, start:String ) -> Self {
		let mut last = Vec::<usize>::with_capacity( data.rows );
		let mut new = Vec::<usize>::with_capacity( data.rows );
		let mut energy_array = Vec::<f64>::with_capacity( *k  );
		let mut rng = rand::thread_rng();

		let grouping: Data;

		if Path::new( &start ).exists(){
			grouping = Data::read_file( &start, '\t' );
			if data.rows == grouping.rows {
				for i in 0..data.rows {
					let r = grouping.data[[i,0]].round() as usize - 1;
					new.push( r );
					last.push ( r );
				}
			}
		}

		if new.is_empty(){
			println!("randomly assigning groups");
			for _i in 0..data.rows {
				let r = rng.gen_range(0..*k);
				new.push( r );
				last.push ( r );
			}
		}
		
		for _i in 0..*k{
			energy_array.push(0.0);
		}

		let energy = 0.0;
		let changed = [0,0];
		let row_c = 0;
		let old_energy = [0.0 as f64;2];

		Self {
			data,
			last,
			new,
			energy,
			energy_array,
			changed,
			k:*k,
			row_c,
			old_energy,
		}
	}

	pub fn in_cluster(&self, group:usize ) -> Vec<usize> {
		let mut ret = Vec::<usize>::with_capacity( self.k );
		for i in 0..self.new.len(){
			if self.new[i] == group {
				ret.push(i);
			}
		}
		ret
	}

	pub fn calc_energy( &mut self ) -> f64 {
		let mut ret:f64 = 0.0;

		if self.changed[0] != self.changed[1]{
			// only calculate the missing ones
			//println!("Calculating the missing ones only!");
			self.old_energy[0] = self.energy_array[self.changed[0]];
			self.old_energy[1] = self.energy_array[self.changed[1]];
			for d in self.changed{
				self.energy_array[d] = self.data.dist( &self.in_cluster(d) );
			}
		}
		else {
			//println!("Calculate all energy values");
			for d in 0..self.k{
				self.energy_array[d] = self.data.dist( &self.in_cluster(d) );
			}
		}

		for val in &self.energy_array{
			ret += val;
		}
		ret
	}

	pub fn switch_row( &mut self ) {
		let mut rng = rand::thread_rng();
		//let k = rng.gen_range(0..self.k);
		// let mut r:usize = 0; 
		// let mut max:f64 = 0.0;
		// for i in self.in_cluster( k ){
		// 	if max < self.data.total_energy[i]{
		// 		max = self.data.total_energy[i];
		// 		r = i;
		// 	}
		// }

		let r:usize = rng.gen_range(0..self.data.rows);
		//println! ("we are looking to change gene {r}");
		while self.new[r] == self.last[r]{
			self.new[r] = rng.gen_range( 0..self.k );
		}
		//println!("switched gene {r} from {} to {}", self.last[r], self.new[r] );
		self.changed = [ self.last[r], self.new[r] ];
		self.row_c = r;
	}

	pub fn fixate( &mut self ){
		self.last[self.row_c] = self.new[self.row_c];
	}

	pub fn rinse( &mut self ){
		self.new[self.row_c] = self.last[self.row_c];
		self.energy_array[self.changed[0]] = self.old_energy[0];
		self.energy_array[self.changed[1]] = self.old_energy[1];
	}

	pub fn write( &mut self, fp:&PathBuf,  ){
		let f = match File::create( fp){
        	Ok(file) => file,
        	Err(err) => panic!("The file {} cound not be created: {err}", fp.file_name().unwrap().to_str().unwrap() )
    	};
    	let mut buff1 = BufWriter::new( f );

	    for i in 0..self.data.rows{
    	    match writeln!( buff1, "{}\t{}",  &self.data.rownames[i].to_string(), self.new[i] ){
        	    Ok(_) => () ,
            	Err(err) => {
                	eprintln!("write error: {err}");
            	}	   
        	}
    	}
	}
	pub fn print_change( &self ){
		println!("gene{} moved from cluster {} to {}", self.row_c, self.changed[0], self.changed[1] );

	}
}


#[cfg(test)]
mod tests {

    use crate::data::Data;
    use crate::simulation::Simulation;
    use std::path::Path;

     #[test]
    fn check_dist() {

    	let mut data = Data::read_file( &"testData/Spellman_Yeast_Cell_Cycle.tsv".to_string(), '\t' );
    	data.scale();
    	let mut sim = Simulation::new( data, &10, "testData/RFclustered.txt".to_string() );
    	let ids = sim.in_cluster( 0 );
    	assert_eq!( ids, [0, 4, 16, 17, 32, 33, 56, 63, 88, 89, 91, 
    						92, 99, 106, 108, 121, 131, 133, 137, 144, 149,
    	 					163, 172, 173, 174, 180, 185, 187, 200,
    	  					211, 225, 237, 242, 243, 253]);

    	assert_eq!( sim.data.dist( &sim.in_cluster( 0 ) ), 521.4210862193595  );

    	let new_energy = sim.calc_energy( ) / 10.0;

    }

}