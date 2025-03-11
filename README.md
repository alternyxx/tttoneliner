# tttoneliner
A project to make a Tic-Tac-Toe AI with a single simple line of code.  
<br>
<img 
    src="/assets/tttol.gif" 
    width="800" 
    height="450"
/>

# Explanation
```math
\text{Let } E, X, O \text{ be distinct values such that} \\
E, X, O \in \{ x \mid 0 \leq x \leq 9 \}
```
  

```math
\text{Board state is represented as} \\
B = \sum_{n=0}^8 C_n \times 10^n \\
\text{where } C_n \in \{E, X, O\} \text{ and represents the value at position } n
```

```math
\text{B is also such that } \\
n(E) > 0 \text{ and } n(X) = n(O) \text{ if its player 1's turn or} \\
n(E) > 0 \text{ and } n(X) = n(O)+1 \text{ if its player 2's turn}
```


```math
\text{We try to find a function } f(B) \text{ that returns the optimal move}
\\[1ex]
\text{Currently trying } f(B) = \left(N \times x\right) mod 9
```

# Project Structure
The three directories contain a README, going over their details but here's an overview for the purpose of each directory.  
  
/dataset is where the dataset used for the project is generated.  
/nyxs_tac is where the magic number N is generated.  
/tttoneliner is the one liner for Tic-Tac-Toe.