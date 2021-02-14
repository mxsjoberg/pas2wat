;; this file is generated
(module
    (func
        ;; signature
        (export "test_case_4_1")
        (param $n f64)
        (result f64)
        ;; body
        ;; assignment statement
        (set_local $n
            ;; number
            (f64.const 0)
        )
        ;; if statement
        (if
            (block
                (result i32)
                ;; variable reference
                (get_local $n)
                ;; number
                (f64.const 5)
                ;; binary operator
                (f64.lt)
            )
            (then
                ;; assignment statement
                (set_local $n
                    ;; variable reference
                    (get_local $n)
                    ;; number
                    (f64.const 1)
                    ;; binary operator
                    (f64.add)
                )
            )
            (else
                ;; assignment statement
                (set_local $n
                    ;; variable reference
                    (get_local $n)
                    ;; number
                    (f64.const 1)
                    ;; binary operator
                    (f64.sub)
                )
            )
        )
        ;; variable reference
        (get_local $n)
    )
)