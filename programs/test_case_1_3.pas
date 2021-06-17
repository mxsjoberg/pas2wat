{ expression: ((22 MOD (2 * 4)) * (21 DIV 3)) }
PROGRAM test_case_1_3;
VAR
	number: REAL;
BEGIN
	number := ((22 MOD (2 * 4)) * (21 DIV 3));

	WRITELN(number);
END.