;; this file is generated
(module
  (import "console" "log" (func $log (param f64)))
  (func
    ;; signature
    (export "test")
    (param $int f64)
    ;; assignment statement
    (set_local $int
      ;; number
      (f64.const 10)
    )
    ;; if statement
    (if
      (block
        (result i32)
        ;; boolean
        (i32.const 1)
      )
      (then
        ;; variable reference
        (get_local $int)
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