w = inp
x = 1
y = inp + 11
z = z * 26 + inp + 11

if the number added to x (after putting z in) is positive, it's >9
  so it can't == w
positive x means we put a number in
  by z *= 26 + inp + y_offset
negative x (assuming we match w/inp to equal x) means we pop a number off
  by z /= 26

|  x  |  y  |
-------------
|  14 | 16  |
|  11 |  3  |
|  12 |  2  |
|  11 |  7  |
| -10 | 13  |
|  15 |  6  |
| -14 | 10  |
|  10 | 11  |
|  -4 |  6  |
|  -3 |  5  |
|  13 | 11  |
|  -3 |  4  |
|  -9 |  4  |
| -12 |  6  |

push 16 + inp0
push  3 + inp1
push  2 + inp2
push  7 + inp3
pop (inp4 must equal previous input (i[3]) - 10)
push 6 + inp5
pop (inp6 must equal previous input (i[5]) - 14)
push 11 + inp7
pop (inp8 must equal previous input (i[7]) - 4)
pop (inp9 must equal i[2] - 3)
push 11 + inp10
pop (inp11 must equal previous input (i[10]) - 3)
pop (inp12 must equal i[1] - 9)
pop (inp13 must equal i[0] - 12)

inputs:
i[0]  => 
i[1]  => 
i[2]  => 
i[3]  => 
i[4]  => i[3] - 3
i[5]  => 
i[6]  => i[5] - 8
i[7]  => 
i[8]  => i[7] + 7
i[9]  => i[2] - 1
i[10] => 
i[11] => i[10] + 8
i[12] => i[1] - 6
i[13] => i[0] + 4

0 1 2 3 4 5 6 7 8 9 10 11 12 13
5 9 9 9 6 9 1 2 9 8  1  9  3  9
59596989399939 => too low
59596899399939 => too low
59996893689939 => too low
59996912981939 => correct!

# Part 2 (smallest):
0 1 2 3 4 5 6 7 8 9 10 11 12 13
1 7 2 4 1 9 1 1 8 1  1  9  1  5
17241911811915 => correct!
