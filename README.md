# simulated_annealing

This will in the end implement the final project ffrom this R course:

https://sccbioinformatics.github.io/R_programming_1/#The_Final_Project


The main steps are these:

1. load the data into Rust
2. scale the data to lie between 0 and 1
3. randomly group the genes into k clusters
5. sum all these values per cluster 
6. calculate the mean 'enegery' in the system as mean cluster energy

Now the fun starts - randomly asigne one gene to a different cluster and re-run this.
Compare the two energies and keep the clusters if the new energy is lower than the old.
I the enw energy is higher than the old only change the energy if this term is true:

e^-((V_new - V_old)/T) > random(0,1)

With T being the temperature of the system.
Then scale the temperatue by the cooling factor.

1. 