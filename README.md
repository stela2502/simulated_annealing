# simulated_annealing

This will in the end implement the final project ffrom this R course:

https://sccbioinformatics.github.io/R_programming_1/#The_Final_Project


The main steps are these:

1. load the data into Rust
2. scale the data to lie between 0 and 1
3. randomly group the genes into k clusters
5. sum all these values per cluster 
6. calculate the mean 'energy' in the system as mean cluster energy

Now the fun starts - randomly asign one gene to a different cluster and re-run 5 and 6.
Compare the two energies and keep the clusters if the new energy is lower than the old or
if the new energy is higher than the old only keep this change if this term is true:

$$ e^-{(V_{new} - V_{old}) \over T} > random(0,1) $$

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
