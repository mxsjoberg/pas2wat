;; this file is generated
(module
    (func
        ;; signature
        (export "test_case_1_2")
        (param $number f64)
        (result f64)
        ;; body
        ;; assignment statement
        (set_local $number
            ;; number
            (f64.const 6)
            ;; number
            (f64.const 24)
            ;; binary operator
            (f64.mul)
            ;; number
            (f64.const 6)
            ;; unary operator
            (f64.neg)
            ;; binary operator
            (f64.add)
            ;; number
            (f64.const 4)
            ;; number
            (f64.const 32)
            ;; binary operator
            (f64.mul)
            ;; number
            (f64.const 3)
            ;; binary operator
            (f64.mul)
            ;; unary operator
            (f64.neg)
            ;; binary operator
            (f64.sub)
            ;; number
            (f64.const 4)
            ;; number
            (f64.const 2)
            ;; binary operator
            (f64.mul)
            ;; number
            (f64.const 4)
            ;; binary operator
            (f64.add)
            ;; binary operator
            (f64.div)
            ;; number
            (f64.const 1.5)
            ;; unary operator
            (f64.neg)
            ;; binary operator
            (f64.add)
        )
        ;; variable reference
        (get_local $number)
    )
)