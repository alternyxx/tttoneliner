# neuralnet
Originally, the neural network was just a one time thing that I did here. 
Again, when I was initially working on this, I thought it would be an overnight 
project but oh did I underestimate the function that I was tackling. And so, I 
eventually migrated the repository for the neural network to a seperate repo, 
neuralnyx, where I made it a full library.

# Cost
There are a lot of problems with the library and so, might require a couple retries.  
Personally, I reached a cost of 0.17~ but since it's my own implementation, I can't trust 
it myself :P. And ofc, even if someone else did get it, the weights and biases probably 
won't be the same.

# Issues
neuralnyx currently doesn't train the last batch during an epoch (due to gpu funkiness) 
so that's where I presume the mismatched training is from.