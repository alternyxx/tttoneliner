# neuralnet
Originally, the neural network was just a one time thing that I did here. 
Again, when I was initially working on this, I thought it would be an overnight 
project but oh did I underestimate the function that I was tackling. And so, I 
eventually migrated the repository for the neural network to a seperate repo, 
neuralnyx, where I made it a full library.

# Number of neurons
The amount of neurons used is currently 15 for the first layer and then 9 for the 
output later. Trying to use only the output layer results in converging at around 0.8. 
While this does get a somewhat decent AI (one that can recognise at least which moves NOT 
to play somehow), it hands games after one or 2 moves. Trying to use less than 15 neurons 
result in much higher convergeance as opposed to 15. IIRC, 12 results in a convergeance around 
0.5~ whereas for some reason, 9 neurons can't even converge below 1.0.

# Cost
There are a lot of problems with the library and so, might require a couple retries.  
Personally, I reached a cost of 0.18~ but since it's my own implementation, I can't trust 
it myself :P. And ofc, even if someone else did get it, the weights and biases probably 
won't be the same.