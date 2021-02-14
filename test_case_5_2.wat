;; this file is generated
(module
    (func
        ;; signature
        (export "test_case_5_2")
        (param $c f64)
        (param $n f64)
        (param $F0 f64)
        (param $F1 f64)
        (param $Fn f64)
        (result f64)
        ;; body
        ;; assignment statement
        (set_local $c
            ;; number
            (f64.const 0)
        )
        ;; assignment statement
        (set_local $n
            ;; number
            (f64.const 42)
        )
        ;; assignment statement
        (set_local $Fn
            ;; number
            (f64.const 0)
        )
        ;; assignment statement
        (set_local $F0
            ;; number
            (f64.const 0)
        )
        ;; assignment statement
        (set_local $F1
            ;; number
            (f64.const 1)
        )
        ;; if statement
        (if
            (block
                (result i32)
                ;; variable reference
                (get_local $n)
                ;; number
                (f64.const 0)
                ;; binary operator
                (f64.le)
            )
            (then
                ;; assignment statement
                (set_local $Fn
                    ;; variable reference
                    (get_local $F0)
                )
            )
            (else
                ;; if statement
                (if
                    (block
                        (result i32)
                        ;; variable reference
                        (get_local $n)
                        ;; number
                        (f64.const 1)
                        ;; binary operator
                        (f64.eq)
                    )
                    (then
                        ;; assignment statement
                        (set_local $Fn
                            ;; variable reference
                            (get_local $F1)
                        )
                    )
                    (else
                        ;; while statement
                        (block
                            (loop
                                ;; assignment statement
                                (set_local $Fn
                                    ;; variable reference
                                    (get_local $F0)
                                    ;; variable reference
                                    (get_local $F1)
                                    ;; binary operator
                                    (f64.add)
                                )
                                ;; assignment statement
                                (set_local $F0
                                    ;; variable reference
                                    (get_local $F1)
                                )
                                ;; assignment statement
                                (set_local $F1
                                    ;; variable reference
                                    (get_local $Fn)
                                )
                                ;; assignment statement
                                (set_local $c
                                    ;; variable reference
                                    (get_local $c)
                                    ;; number
                                    (f64.const 1)
                                    ;; binary operator
                                    (f64.add)
                                )
                                ;; conditional statement
                                (br_if 1
                                    ;; variable reference
                                    (get_local $c)
                                    ;; variable reference
                                    (get_local $n)
                                    ;; number
                                    (f64.const 2)
                                    ;; binary operator
                                    (f64.sub)
                                    ;; binary operator
                                    (f64.gt)
                                )
                                (br 0)
                            )
                        )
                    )
                )
            )
        )
        ;; variable reference
        (get_local $Fn)
    )
)