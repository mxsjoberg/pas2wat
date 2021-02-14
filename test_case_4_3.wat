;; this file is generated
(module
    (func
        ;; signature
        (export "test_case_4_3")
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
                ;; boolean
                (i32.const 1)
            )
            (then
                ;; assignment statement
                (set_local $n
                    ;; number
                    (f64.const 1)
                )
            )
            (else
                ;; assignment statement
                (set_local $n
                    ;; number
                    (f64.const 0)
                )
            )
        )
        ;; variable reference
        (get_local $n)
    )
)