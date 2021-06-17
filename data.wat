(module
  (import "console" "log" (func $log (param i32 i32)))
  (import "js" "mem" (memory 1))
  (data (i32.const 0) "hello wasm")
  (func (export "main")
  	;; pass offset 0 to log
    i32.const 0
    ;; pass length 10 to log
    i32.const 10
    ;; call log
    call $log
  )
)