const fs = require('fs')
const util = require('util')

let depths = fs.readFileSync('./depths.txt', {
    encoding: "utf8",
    flag: 'r'
})

depths = depths.split('\n').map(Number)

let depthIncreasedCount = 0;
let previousDepth;
const windowLength = 3;
const windows = [];

// todo: group depths into sets of 3
console.log(depths);

for (let i=0; i<depths.length - windowLength; i++) {
    windows[i] = [depths[i], depths[i+1], depths[i+2]] 
}

// process.stdout.write(windows);
console.log(util.inspect(windows, { maxArrayLength: null }))
