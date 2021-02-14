{ factorial of 10 : 3628800 }
PROGRAM test_case_5_1;
VAR
    n, result: LONGINT;
BEGIN
    n := 10;
    result := 1;
    
    IF n = 0 THEN
    ELSE
        WHILE n > 0 DO
        BEGIN
            result := n * result;
            n := n - 1;
        END;
    
    WRITELN(result);
END.