var pw_old = "I had to modify the return statement, because it was introducing a bug for me";

var ffi = require('ffi');

var lib = ffi.Library('target/release/randomizer', {
    randomize: ['string', ['string', 'int']]
});

function randomizer(pw, num) {
    console.log(pw.length);
    var new_pw = '';

    for (var i=0; i < pw.length; i++) {
        var oldChar = pw.charCodeAt(i);
        var newChar = oldChar + num;
        new_pw += String.fromCharCode(newChar);
    }
    return result;
}

console.time("randomizer");
//console.log(randomizer(pw_old, 3));
console.log(lib.randomize(pw_old, 3));
console.timeEnd("randomizer");