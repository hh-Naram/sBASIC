10 PRINT "First 10 Fibonacci Numbers:"
20 LET N = 10
30 LET I = 0
40 LET VAL1 = 0
50 LET VAL2 = 1
60 PRINT VAL1
70 PRINT VAL2
80 LET NEXTVAL = VAL1 + VAL2
90 PRINT NEXTVAL
100 VAL1 = VAL2
110 VAL2 = NEXTVAL
120 NEXTVAL = VAL1 + VAL2
130 IF I < 10 THEN 90
