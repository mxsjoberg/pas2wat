;; this file is generated
(module
    (func
        ;; signature
        (export "test_case_3_1")
        (param $v_0 f64)
        (param $v_1 f64)
        (param $v_2 f64)
        (result f64)
        ;; body
        ;; assignment statement
        (set_local $v_0
            ;; number
            (f64.const 40)
            ;; number
            (f64.const 4)
            (i32.trunc_f64_s)
            ;; number
            (f64.const 1)
            ;; number
            (f64.const 1)
            ;; binary operator
            (f64.add)
            (i32.trunc_f64_s)
            ;; binary operator
            (i32.div_s)
            (f64.convert_i32_s)
            ;; binary operator
            (f64.add)
        )
        ;; variable reference
        (get_local $v_0)
    )
)