{ expression: (((6 * 24) - 6) + (4 * 32 * 3)) / (4 * 2 + 4) - 1.5 }
PROGRAM test_case_1_1;
VAR
    number: REAL;
BEGIN
    number := (((6 * 24) - 6) + (4 * 32 * 3)) / (4 * 2 + 4) - 1.5;
    
    WRITELN(number);
END.