# tttoneliner
A one liner for an AI for Tic-Tac-Toe, with ~300 bytes.  
<!--All the spaces for newlines are intended-->
<!--The actual version isnt updated despite all this yap soz :P-->
<br>    
![Examples of Tic-Tac-Toe being played](../assets/tttol.gif)  
<sub>plz some1 provide better quality ;-;</sub>

# pip
Make sure Python (> 3.10) is installed and added to path, as well as pip.  
To try it out for yourself,
```
pip install tttoneliner
```

As always, you can also
```
git clone https://github.com/alternyxx/tttoneliner
cd \tttoneliner\tttoneliner
pip install .
```

# Trying out
```
tttol
```
Runs the script and waits for an input.  
A board is like this-
```  
 1 │ 2 │ 3 
───┼───┼───  
 4 │ 5 │ 6 
───┼───┼───  
 7 │ 8 │ 9 
```
and you can type 5 to play in the middle, which should then
print out
```
 O │   │   
───┼───┼───
   │ X │
───┼───┼───
   │   │
```
The X is a move made by you and the O is made by the AI.  
For your fairness, there is a single way to beat it (at 
least from what I've discovered) and is totally not a bug.

# Explanation
It is worth noting that in the actual tttoneliner.py, instead of the outside comprehension to set variables, 
it is actually a main function using default parameters to set the variables. This means that technically, its 
actually two lines of code (just cramped in one line). This is because a function is necessary to set an entry 
point.  
<br>
Current versions
### Most condensed version (wip)
```python
def main(B=111111111):{print(B)for _ in range(5)if(B:=B+1*10**abs(9-int(input())))and(B:=B+2*10**boards[B])}
```

### Readable (And Playable) version (plays random moves)
```python
{{
    print('{}│{}│{}\n───┼───┼───\n{}│{}│{}\n───┼───┼───\n{}│{}│{}'
        .format(*['   ' if i == '1' else ' X ' if i == '2' else ' O 'for i in str(B)])
    ) for _ in range(5)
        if (B := B + 1 * 10 ** abs(9-int(input())))
            and (t := [2.71 ** (
                sum(w[i][j] * list(map(int,str(B)))[j] for j in range(9)) + b[i]
            ) for i in range(9)])
            and (B := B + 8 * 10 ** t.index(max(t, key = lambda x: x / sum(t))))
 } for B, w, b in [(
    111111111,
    [
        [0.5324228, 0.76355624, 1.0098922, 1.0587181, 0.47961304, 0.8581823, 0.045538172, 1.0410924, -1.579585],
        [0.4228034, 0.9537834, 0.9404552, 1.0115612, 0.5452481, 0.8414384, 0.0972775, -1.0017858, 0.40512067],
        [0.7085032, 0.955647, 1.0234874, 1.03152, 0.36106178, 0.82706773, -1.0083874, 0.76845086, 0.19899876],
        [0.46637133, 0.65566856, 1.0476296, 1.3649647, 0.5284521, -1.0649983, 0.18261205, 0.78231066, 0.3543514],
        [0.6555538, 0.993636, 1.0712171, 1.2289686, -1.1268336, 0.7060021, 0.121178925, 0.5562301, 0.011001009],
        [0.6181679, 0.48797947, 1.063911, -0.8666658, 0.55554706, 0.98548925, 0.23587868, 0.90056294, 0.30139807],
        [0.7447117, 0.6618681, -0.7941452, 1.040301, 0.39376765, 0.8027111, 0.14813231, 0.800058, 0.38237593],
        [0.6764701, -1.1199214, 1.1118764, 0.79409623, 0.48408085, 0.83786356, 0.18026906, 1.0840094, 0.28564036],
        [-0.9352118, 0.70592344, 1.1290987, 1.057092, 0.47734314, 0.8022352, 0.11734288, 0.7755984, 0.34827393],
    ],
    [-0.27312347, -0.35538667, -0.7696111, -0.9394737, 0.61957735, -0.09319963, 0.28401002, 0.48452064, 0.626548],
)]}
```