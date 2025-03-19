# neuralnet
A neural network from scratch.  

### Why?
idk- i shouldve prob js used a framework- ;-; this did NOT need any gpu operations :sob:

# Explanation
The neural network is divided into two parts, neuralnet.rs and neuralnet.wgsl  
The forward pass is done in neuralnet.wgsl  
and we retrieve back the data and do the backward propagation in the rust side.

During forward pass, we send batches of 64 to the gpu

