;; this file is generated
(module
    (func
        ;; signature
        (export "test_case_2_3")
        (param $a f64)
        (param $b f64)
        (param $c f64)
        (param $d f64)
        (param $x f64)
        (result f64)
        ;; body
        ;; assignment statement
        (set_local $a
            ;; number
            (f64.const 2)
            ;; number
            (f64.const 4)
            ;; binary operator
            (f64.mul)
        )
        ;; assignment statement
        (set_local $b
            ;; number
            (f64.const 22)
            (i32.trunc_f64_s)
            ;; variable reference
            (get_local $a)
            (i32.trunc_f64_s)
            ;; binary operator
            (i32.rem_s)
            (f64.convert_i32_s)
        )
        ;; assignment statement
        (set_local $c
            ;; number
            (f64.const 21)
            (i32.trunc_f64_s)
            ;; number
            (f64.const 3)
            (i32.trunc_f64_s)
            ;; binary operator
            (i32.div_s)
            (f64.convert_i32_s)
        )
        ;; assignment statement
        (set_local $d
            ;; variable reference
            (get_local $b)
            ;; variable reference
            (get_local $c)
            ;; binary operator
            (f64.mul)
        )
        ;; assignment statement
        (set_local $x
            ;; variable reference
            (get_local $d)
        )
        ;; variable reference
        (get_local $x)
    )
)