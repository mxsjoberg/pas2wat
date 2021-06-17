PROGRAM test_case_3_1;
VAR
    v: PACKED ARRAY[0..2] OF INTEGER;
BEGIN   
    v[0] := ((40) + (4 DIV (1 + 1)));
    
    WRITELN(v[0]);
END.