PROGRAM test_case_3_2;
VAR
    v: PACKED ARRAY[0..2] OF INTEGER;
    x, y: INTEGER;
BEGIN   
    v[0] := 42;
    x := 1;
    y := 1;
    
    WRITELN(v[x - y]);
END.