PROGRAM test_case_4_1;
VAR
    n: INTEGER;
BEGIN
    n := 0;
    
    IF n < 5 THEN 
        n := n + 1
    ELSE
        n := n - 1;
    
    WRITELN(n);
END.