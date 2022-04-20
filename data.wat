(module
  (import "console" "log" (func $log (param f64)))
  (import "console" "logString" (func $logString (param i32 i32)))
  (import "js" "mem" (memory 1))
  (data (i32.const 0) "Hi")
  (func (export "test")
    i32.const 0  ;; pass offset 0 to log
    i32.const 2  ;; pass length 2 to log
    call $logString
    f64.const 1
    call $log
  )
)