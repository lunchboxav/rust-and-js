var ffi = require('ffi');

var lib = ffi.Library('target/release/fibonacci', {
    fibonacci: ['int', ['int']]
});

function fibonacci(num) {
    if (num <= 1) {
        return 1;
    } else {
        return fibonacci(num - 1) + fibonacci(num - 2);
    }   
}

function printFibonacci() {
    for (i = 0; i < 46; i++) {
        console.log(i + " - " + lib.fibonacci(i));
    }
}

console.time("fibonacci");
printFibonacci();
console.timeEnd("fibonacci");
