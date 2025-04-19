# tttoneliner
A project to make a Tic-Tac-Toe AI with a single simple line of code.  
A supposed overnight project turned disastrous.  

Provided below is the one liner which you can just run in a Python interpreter!
```py
{(B:=111111111,{print('{}│{}│{}\n───┼───┼───\n{}│{}│{}\n───┼───┼───\n{}│{}│{}'.format(*['   'if i=='1'else' X 'if i=='5'else' O 'for i in str(B)]))for _ in range(5)if(B:=B+4*10**abs(9-int(input())))and(y:=[2.7182**(sum(w[i][j]/100*max(0,[sum(v[k][l]/100*list(map(int, str(B)))[l]for l in range(9))+a[k]/100 for k in range(15)][j])for j in range(15))+b[i]/100)for i in range(9)])and(B:=B+8*10**y.index(max(y,key=lambda x:x/sum(y))))})for v,w,a,b in[([[-7,-30,-3,-37,-2,13,-11,28,-95],[120,160,-34,-41,145,16,-31,218,-19],[-120,-5,-130,195,-269,86,-166,-24,109],[14,-120,49,15,-130,15,-39,142,64],[23,129,-81,257,100,-48,-28,7,67],[-46,-46,73,-89,291,0,178,61,143],[-23,-23,-260,34,-158,16,212,-41,-278],[-2,86,169,98,47,-164,49,-245,250],[-95,-59,160,194,-80,99,-108,-154,-143],[12,2,68,55,-50,-71,216,216,-167],[47,242,166,-1,4,-81,199,-47,0],[32,40,-160,-167,189,122,199,22,22],[108,-184,41,67,172,21,47,115,-85],[-51,287,-344,62,-134,42,-244,26,-50],[-94,-59,-30,43,10,-40,-43,33,-102]],[[1,-4,7,-107,24,4,-11,-154,73,-12,123,-8,74,77,-156],[-3,-72,6,-343,38,-49,-4,6,85,-109,106,88,57,-96,-28],[0,202,257,0,-12,45,-21,-83,79,-243,8,-224,15,206,2],[40,88,-134,-76,2,-37,-24,126,-261,161,-92,31,-14,-382,36],[-7,-1,-208,265,30,-194,156,-24,41,32,131,82,-167,-177,4],[-30,153,-28,-23,-202,-5,74,80,170,54,-48,85,-40,75,-33],[-3,56,225,7,48,-99,-265,85,-223,131,-103,166,-23,-42,18],[23,-100,-131,187,-133,-44,284,195,-57,139,-250,125,287,-6,28],[4,-196,72,-11,160,348,-250,-307,67,3,85,-192,-275,233,-27]],[0,-166,159,438,-19,153,149,-46,-211,-103,36,-59,-330,67,1],[-269,40,-42,-89,169,92,26,-78,159])]}
```

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
<!--ive been writing docs for two days, js lemme skimp over ts fn-->
$$o(B) = \forall_{j=1}^9 max(0, \sum_{i=0}^8 (B_i w_i,j) + b_j) $$  
<br>
In this case, we'd map o(b) so that $2 \times 10^(max(o(B)))$ can correctly progress the board with the optimal move.
You may notice that's just an equation for a forward pass and that's exactly what I ended up having to resort to.  
For example, below is a function that I first tried,
<br>
$$o(B) = w \times B\ mod\ 9$$
<br>
but this ended up not working because of the $n(X) = n(O) + 1$  
To this extent, I also tried transformations of $B$, for example, incorporating the digit position of $B_i$, but
it didn't end up working. 

# Project Structure
The three directories contain a README, going over their details but here's an overview for the purpose of each directory.  
  
[/dataset](/dataset/) is where the dataset used for the project is generated.  
[/neuralnet](/neuralnet/) is where the neural network is written.  
[/tttoneliner](/tttoneliner/) is the one liner for Tic-Tac-Toe.
