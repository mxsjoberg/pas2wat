{ fibonacci number for 42 : 267914296 }
{ sequence starting at n=0 : 0,1,1,2,3,5,8,13,21,34,55,89,144,... }
PROGRAM test_case_5_2;
VAR
    c, n, F0, F1, Fn: LONGINT;
BEGIN
    c := 0;
    n := 42;
    Fn := 0;
    
    F0 := 0;
    F1 := 1;
    
    IF n <= 0 THEN
        Fn := F0
    ELSE
    IF n = 1 THEN
        Fn := F1
    ELSE
    WHILE c <= (n - 2) DO
    BEGIN
        Fn := F0 + F1;
        F0 := F1;
        F1 := Fn;
        c := c + 1;
    END;

    WRITELN(Fn);
END.