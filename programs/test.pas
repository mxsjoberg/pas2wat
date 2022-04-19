PROGRAM test;
{ BEGIN
	IF ODD(1) THEN
    	WRITELN(42)
    ELSE
    	WRITELN(0);
END. }
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