PROGRAM test_case_4_3;
VAR
    n: INTEGER;
BEGIN
    n := 0;
    
    IF true THEN 
        n := 1
    ELSE
        n := 0;
    
    WRITELN(n);
END.