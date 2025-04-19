# Dataset
Concerns with generating the dataset.json for mapping.  
The dataset has all reachable board positions as keys and their respective 
optimal moves as values. It's worth noting that a position such as 
```
   Board           Array            
 O │ X │ O       [
───┼───┼───       2, 1, 2,
   │ X │          0, 1, 0,
───┼───┼───       0, 0, 0,
   │   │                  ]
```
while very much possible, wouldn't be included in the dataset because it isn't 
reachable when player 2 plays perfectly. It's also noting that while the optimal 
move was originally the index to play, it's now $10^{|8 - i|}$ to condense more 
in the oneliner. The i is the index to play ofc :P.
<br><br>
  
Currently, i, j and k values are being used as 1, 2 and 3 but this happens only
during outputting the dataset. Within tictactoe.rs, which is where every
function relating to tictactoe is, an empty cell is represented as 0, an X is
represented as 1 and an O is represented as a 2. This also means that the 
function to represent a tictactoe board as an integer is in dataset.rs.
<br><br>
  
There are some concerns I have with dataset generation.  
  
First off, since I'm too lazy, the dataset is a hashmap of string and i8's. This means that the values are not sorted and updates everytime a new dataset.json is generated since hashmaps are unsorted.  
  
Secondly, tictactoe.rs is actually broken and there's an issue specifically in
this position
```
   │   │ X
───┼───┼───
   │ O │ X
───┼───┼───
   │   │
```
whereby, our minimax function returns 1, resulting in
```
   │ O │ X
───┼───┼───
   │ O │ X
───┼───┼───
   │   │
```  
ion hv a clue on y this happens tho-  
<br>
Aside from that, this was my first ever rust code soooo yeah!