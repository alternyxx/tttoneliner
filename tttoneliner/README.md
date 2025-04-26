# tttoneliner
A one liner for an AI for Tic-Tac-Toe, with around 1600~ bytes. 
Well, nearly a perfect AI anyways since you can probably find a lot of positions 
in which the AI won't play a good move and just... hand you the game.

# Try It Out!
Make sure Python 3.10 or newer is installed for any of the below.  

### Interpreter
Provided below is the one liner which you can simply copy paste and run in the interpreter!
```py
{(B:=111111111,{print('{}│{}│{}\n───┼───┼───\n{}│{}│{}\n───┼───┼───\n{}│{}│{}'.format(*['   'if i=='1'else' X 'if i=='5'else' O 'for i in str(B)]))for _ in range(5)if(B:=B+4*10**abs(9-int(input())))and(y:=[2.7182**(sum(w[i][j]/100*max(0,[sum(v[k][l]/100*list(map(int, str(B)))[l]for l in range(9))+a[k]/100 for k in range(15)][j])for j in range(15))+b[i]/100)for i in range(9)])and(B:=B+8*10**y.index(max(y,key=lambda x:x/sum(y))))})for v,w,a,b in[([[-7,-30,-3,-37,-2,13,-11,28,-95],[120,160,-34,-41,145,16,-31,218,-19],[-120,-5,-130,195,-269,86,-166,-24,109],[14,-120,49,15,-130,15,-39,142,64],[23,129,-81,257,100,-48,-28,7,67],[-46,-46,73,-89,291,0,178,61,143],[-23,-23,-260,34,-158,16,212,-41,-278],[-2,86,169,98,47,-164,49,-245,250],[-95,-59,160,194,-80,99,-108,-154,-143],[12,2,68,55,-50,-71,216,216,-167],[47,242,166,-1,4,-81,199,-47,0],[32,40,-160,-167,189,122,199,22,22],[108,-184,41,67,172,21,47,115,-85],[-51,287,-344,62,-134,42,-244,26,-50],[-94,-59,-30,43,10,-40,-43,33,-102]],[[1,-4,7,-107,24,4,-11,-154,73,-12,123,-8,74,77,-156],[-3,-72,6,-343,38,-49,-4,6,85,-109,106,88,57,-96,-28],[0,202,257,0,-12,45,-21,-83,79,-243,8,-224,15,206,2],[40,88,-134,-76,2,-37,-24,126,-261,161,-92,31,-14,-382,36],[-7,-1,-208,265,30,-194,156,-24,41,32,131,82,-167,-177,4],[-30,153,-28,-23,-202,-5,74,80,170,54,-48,85,-40,75,-33],[-3,56,225,7,48,-99,-265,85,-223,131,-103,166,-23,-42,18],[23,-100,-131,187,-133,-44,284,195,-57,139,-250,125,287,-6,28],[4,-196,72,-11,160,348,-250,-307,67,3,85,-192,-275,233,-27]],[0,-166,159,438,-19,153,149,-46,-211,-103,36,-59,-330,67,1],[-269,40,-42,-89,169,92,26,-78,159])]}
```

### Pip
You can also install via pip which conveniently provides an entry point!
```console
pip install tttoneliner
```

After which you can simply run the command tttol
```console
$ tttol
5
 O │   │   
───┼───┼───
   │ X │
───┼───┼───
   │   │
```

### Build
As always, you can also just clone the repository.
```
git clone https://github.com/alternyxx/tttoneliner
cd tttoneliner/tttoneliner
pip install .
```

After which, you'll have the tttol command :D

# Dev Notes
### Overview Explanation
As you may have guessed, it is a neural network, specfically one with two layers. The 
neural network was trained using my own implementation from scratch which is why this project 
(which was supposed to be an overnight one) took so long.  

### Other
It is worth noting that in the actual tttoneliner.py, instead of the outside comprehension to set variables, 
it is actually a main function using default parameters to set the variables. This means that technically, its 
actually two lines of code (just cramped in one line). This is because a function is necessary to set an entry 
point.  

### Readable version
<sub>idt its pep compliant but the format is more readable to me</sub>

```py
{(B := 111111111, {
	print('{}│{}│{}\n───┼───┼───\n{}│{}│{}\n───┼───┼───\n{}│{}│{}'
		.format(*['   ' if i == '1' else ' X ' if i == '5' else ' O ' for i in str(B)])
	) for _ in range(5)
		if (B := B + 4 * 10 ** abs(9 - int(input())))
			and (y := [2.7182 ** (
				sum(w[i][j] / 100 * max(0,
                    [sum(
                        v[k][l] / 100 * list(map(int, str(B))
                    )[l] for l in range(9)) + a[k] / 100 for k in range(15)][j]
                ) for j in range(15)) + b[i] / 100
			) for i in range(9)])
			and (B := B + 8 * 10 ** y.index(max(y, key = lambda x: x / sum(y))))
	}) for v, w, a, b in [(
	[
        [153, -41, -93, -77, -7, 400, 81, 155, -128],
        [108, 108, 74, 146, -70, 33, -264, -148, -18],
        [-187, -71, 25, -69, 85, 136, 27, 121, -171],
        [57, -211, 47, -82, 196, 19, 
        -22, 137, 66],
        [185, -61, -177, -112, -363, 53, -124, -85, 138],
        [98, 66, -127, 13, -317, -53, -5, 57, 117],
        [-200, 71, -184, 164, -185, -34, -250, 101, 55],
        [67, 532, 473, -62, 28, -202, -14, 30, 61],
        [257, 9, -18, 5, -262, 10, -93, 32, 17],
        [-74, -60, -39, -35, -13, -45, 55, -23, -10],
        [-30, 64, 46, 89, 23, 317, -18, -113, -110],
        [7, 
        32, -40, 31, -300, 61, 115, 151, 122],
        [25, 125, 677, 34, -25, -106, -82, -19, -139],
        [215, -350, -37, -208, 209, 1, 94, -21, 237],
        [7, 25, -18, 189, -314, 31, 117, 190, 90],
    ],
	[
        [169, 112, 229, -172, -291, 220, -1367, 18, -473, -102, -204, -247, 32, 10, 144],
        [-155, -23, -210, -454, -8702, -43, -2205, 71, 164, -40, 265, -273, -50, 260, 37],
        [-106, 154, -6044, 434, 404, -329, 771, 105, 426, -21, 125, -152, -133, -204, -91],
        [-56, 41, -165, -90, -5784, -39, -12314, 35, -187, -37, -382, -282, -28, -14, 282],
        [90, -68, -7006, -767, 475, 476, -1321, -238, -328, -21, -107, 167, 332, -293, -43],
        [61, -30, 55, -66, -6326, -101, -5569, 24, -35, -23, -92, 730, 12, -24, -663],
        [100, 172, -224, -120, 40, -13, -556, 72, -134, -19, -200, -179, -279, -42, 102],
        [-166, -148, 124, 195, -3945, -225, 357, -802, 243, -23, -204, -37, 574, 321, 211],
        [-425, -603, -56, 184, 3553, -741, 763, -24, -8832, 18, 388, 643, -32, -270, -215],
    ],
	[-8, 69, -298, -426, 16, 990, 758, 98, 42, -11, -451, 949, 283, -15, 961],
    [-917, -1089, -2146, 2085, -869, 863, 1964, -524, 5630],
)]}
```
