10 PRINT "How many numbers of the Fibonacci sequence to print?"
20 INPUT N
30 LET I = 0
40 LET NUM1 = 0
50 LET NUM2 = 1
60 PRINT NUM1
70 PRINT NUM2
80 LET NNUM = NUM1 + NUM2
90 PRINT NNUM
100 NUM1 = NUM2
110 NUM2 = NNUM
120 NNUM = NUM1 + NUM2
130 I = I + 1
140 IF I < N THEN 90
