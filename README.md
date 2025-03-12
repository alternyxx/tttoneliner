# tttoneliner
A project to make a Tic-Tac-Toe AI with a single simple line of code.  
Now somehow ended up with me writing a neural network....
<br>
![Examples of Tic-Tac-Toe being played](/assets/tttol.gif)

# Explanation
## Board
<!-- We're not using ```math ``` because I can't get it to render linebreaks, same w/ $$ around the whole thing -->
$$\text{Let } E, X, O \text{ be distinct values such that}$$  
$$E, X, O \in \\{ x \mid 0 \leq x \leq 9 \\}$$  
  

$$\text{Board state is represented as}$$  
$$B = \sum_{n=0}^8 C_n \times 10^n$$  
$$\text{where } C_n \in \\{E, X, O\\}$$  
$$\text{ and represents the value at position } n$$  
  
  
$$\text{B is also such that}$$  
$$n(E) > 0 \text{ and } n(X) = n(O) \text{ if its player 1's turn or}$$  
$$n(E) > 0 \text{ and } n(X) = n(O)+1 \text{ if its player 2's turn}$$  
  
## Function
<p align="center">
    <img
        src="/assets/graph.png"
        alt="Graph of the function we're trying to achieve"
        width="70%"
    />
</p>

The above is a subset of possible board positions (with i, j and k 
values being 1, 2 and 3 respectively) and we try to map a function
$O(B)$ to receive a board position and return the optimal move.  
$$O(B) = \sum_{i=0}^8 B_i W_i$$  
Whereby $B_i$ , the $i^{th}$ digit of B, into $W_i$ which is its respective
weight value. 
> [!Note]  
> Getting the weights is pretty much training a neural network, 
> which I'm not going to be doing from scratch so... I guess the project will
> be on a halt for a while... 

# Project Structure
The three directories contain a README, going over their details but here's an overview for the purpose of each directory.  
  
/dataset is where the dataset used for the project is generated.  
/neuralnet is where the weights are generated.  
/tttoneliner is the one liner for Tic-Tac-Toe.
