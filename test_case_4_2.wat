;; this file is generated
(module
    (func
        ;; signature
        (export "test_case_4_2")
        (param $n f64)
        (result f64)
        ;; body
        ;; assignment statement
        (set_local $n
            ;; number
            (f64.const 0)
        )
        ;; while statement
        (block
            (loop
                ;; assignment statement
                (set_local $n
                    ;; variable reference
                    (get_local $n)
                    ;; number
                    (f64.const 1)
                    ;; binary operator
                    (f64.add)
                )
                ;; conditional statement
                (br_if 1
                    ;; variable reference
                    (get_local $n)
                    ;; number
                    (f64.const 5)
                    ;; binary operator
                    (f64.ge)
                )
                (br 0)
            )
        )
        ;; variable reference
        (get_local $n)
    )
)