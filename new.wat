;; this file is generated
(module
    (import "console" "log" (func $log (param f64)))
    (func
        ;; signature
        (export "test")
        ;; body
        ;; if statement
        (if
            (block
                (result i32)
                ;; boolean
                (i32.const 1)
            )
            (then
                ;; number
                (f64.const 42)
                ;; write
                (call $log)
            )
            (else
                ;; number
                (f64.const 0)
                ;; write
                (call $log)
            )
        )
    )
)