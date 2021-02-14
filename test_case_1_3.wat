;; this file is generated
(module
    (func
        ;; signature
        (export "test_case_1_3")
        (param $number f64)
        (result f64)
        ;; body
        ;; assignment statement
        (set_local $number
            ;; number
            (f64.const 22)
            (i32.trunc_f64_s)
            ;; number
            (f64.const 2)
            ;; number
            (f64.const 4)
            ;; binary operator
            (f64.mul)
            (i32.trunc_f64_s)
            ;; binary operator
            (i32.rem_s)
            (f64.convert_i32_s)
            ;; number
            (f64.const 21)
            (i32.trunc_f64_s)
            ;; number
            (f64.const 3)
            (i32.trunc_f64_s)
            ;; binary operator
            (i32.div_s)
            (f64.convert_i32_s)
            ;; binary operator
            (f64.mul)
        )
        ;; variable reference
        (get_local $number)
    )
)