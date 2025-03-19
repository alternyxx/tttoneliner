# tttoneliner
A project to make a Tic-Tac-Toe AI with a single simple line of code.  
A supposed overnight project turned disastrous.
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

This is important to establish in case anyone wants to find a better function than I have.
Read below for more info.
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
The below is the function I'm using in the one-liner and the approach I ended up going for
and below that is the functions I've tried but failed.
<br>
$$o(B) = \forall_{j=1}^9 max(0, \sum_{i=0}^8 (B_i w_i,j) + b_j) $$  
<br>
In this case, we'd map o(b) so that $2 \times 10^max(o(B))$ can correctly progress the board with the optimal move.
You may notice that's just an equation for a forward pass and that's exactly what I ended up having to resort to.  
For example, below is a function that I first tried,
<br>
$$o(B) = w \times B mod 9$$
<br>
but this ended up not working because of the $n(X) = n(O) + 1$  

# Project Structure
The three directories contain a README, going over their details but here's an overview for the purpose of each directory.  
  
[/dataset](/dataset/) is where the dataset used for the project is generated.  
[/neuralnet](/neuralnet/) is where the neural network is written.  
[/tttoneliner](/tttoneliner/) is the one liner for Tic-Tac-Toe.
