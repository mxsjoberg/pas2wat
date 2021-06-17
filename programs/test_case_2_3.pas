{ expression: ((22 MOD (2 * 4)) * (21 DIV 3)) }
PROGRAM test_case_2_3;
VAR
	a: INTEGER;
	b, c: INTEGER;
	d: REAL;
	x: REAL;
BEGIN
	BEGIN
		a := (2 * 4);
		b := 22 MOD a;
		c := 21 DIV 3;
		d := b * c;
	END;

	x := d;

	WRITELN(x);
END.