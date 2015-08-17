rust-jq - rust bindinds for libjq


https://github.com/stedolan/jq

https://github.com/stedolan/jq/wiki/C-API:-jq-program-invocation

Hint: the API in jv.h is for dealing with parsed JSON values, while
the API in jq.h is for evaluating jq programs. Use jq_init() to
allocate a jq_state * value, then use jq_compile_args() to compile a
jq program, then use jq_start() to get ready to evaluate it on one
input value, and then repeatedly call jq_next() to extract all the
output values that the program produces. Termination is indicated by
jq_next() outputting an invalid value (without a message), while
errors are indicated by jq_next() outputting an invalid value with a
message. When done, use jq_teardown() to release the jq_state * value.



Hint: the API in jv.h is for dealing with parsed JSON values, while
the API in jq.h is for evaluating jq programs.
* Use jq_init() to allocate a jq_state * value,
* then use jq_compile_args() to compile a jq program,
* then use jq_start() to get ready to evaluate it on one input value,
* and then repeatedly call jq_next() to extract all the output values that the program produces.
* Termination is indicated by jq_next() outputting an invalid value (without a message),
* while errors are indicated by jq_next() outputting an invalid value with a message.
* When done, use jq_teardown() to release the jq_state * value.


export DYLD_LIBRARY_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/:$DYLD_LIBRARY_PATH
from the jq source directory:
../rust-bindgen/target/release/bindgen -builtins -o ffi_jq.rs -l jq jq.h
../rust-bindgen/target/release/bindgen -builtins -o ffi_jv.rs -l jq jv.h
