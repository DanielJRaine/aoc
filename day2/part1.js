const fs = require('fs');
const util = require('util')

let data = fs.readFileSync('./data.txt', {
    encoding: "utf8",
})

data = data.split('\n');

console.log(util.inspect(data, { maxArrayLength: null }))

