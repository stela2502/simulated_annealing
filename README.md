# simulated_annealing

This will in the end implement the final project ffrom this R course:

https://sccbioinformatics.github.io/R_programming_1/#The_Final_Project


The main steps are these:

1. load the data into Rust
2. scale the data to lie between 0 and 1
3. randomly group the genes into k clusters
4. Calculate the eucledian distances between the genes and sum all these values per cluster 
5. calculate the mean 'energy' in the system as mean cluster energy

Now the fun starts - randomly asign one gene to a different cluster and re-run 4 and 5.
Compare the two energies and keep the clusters if the new energy is lower than the old or
if the new energy is higher than the old only keep this change if this term is true:

$$ e^{-{(E_{new} - E_{old}) \over T}} > random(0,1) $$

With T being the temperature of the system.

7. Then scale the temperatue by the cooling factor.

# Install

You need the Rust compiler: https://www.rust-lang.org/tools/install

Then you can clone this repo and complie the code:

```
git clone git@github.com:stela2502/simulated_annealing.git
cd simulated_annealing
cargo build -r
```

# Testing

```
target/release/simulated_annealing -c 12 --it 100000 --t1 20 --cf 0.9995
Rscript testData/Visualize.R
```

# Usage

```
simulated_annealing  -h
simulated_annealing 1.0.0
Stefan L. <stefan.lang@med.lu.se>
Run a simulated anealing clustering over the rows of the provided data. The software is a demo
project for the Lund Stem Cell Center - Bioinformatics Rust workshop.

USAGE:
    simulated_annealing [OPTIONS]

OPTIONS:
    -c, --clusters <CLUSTERS>    the target cluster count [default: 10]
        --cf <CF>                the cooling factor [default: 0.9995]
    -d, --data <DATA>            the data (text file) [default:
                                 testData/Spellman_Yeast_Cell_Cycle.tsv]
    -h, --help                   Print help information
        --it <IT>                the number of iterations [default: 25000]
    -o, --outpath <OUTPATH>      the outpath [default: testData/TestClustering]
    -s, --sep <SEP>              the column separator for the file [default: \t]
        --start <START>          a starting grouping [default: testData/RFclustered.txt]
        --t1 <T1>                the starting temperature [default: 20]
    -V, --version                Print version information
```


## Output

The main output is the cluster information;
a two column tab separated table connecting cell/sample names with the cluster id.

You can try the tool on our test data:


```
target/release/simulated_annealing  --start "notThis" -c 10
I got 256 rows and 16 cols in this data (testData/Spellman_Yeast_Cell_Cycle.tsv)
precalculate the distances between genes
Finished
randomly assigning groups
Starting energy is 498.77
finished in 0 h 0 min 0 sec 32 milli sec - end energy was 293.01 with 7520 gene shifts (0.30%) and end t1 = 0.00

Clustering written to testData/TestClustering/SimulatedAnealing_k_10_t1_20_cf_0.9995_it_25000.tsv
````