const fs = require('fs')
const util = require('util')
const _ = require('lodash');

let depths = fs.readFileSync('./depths.txt', {
    encoding: "utf8",
    flag: 'r'
})

depths = depths.split('\n').map(Number)

const windowLength = 3;
const windows = [];

// todo: group depths into sets of 3

let currentWindowSum;
let previousWindowSum;
let depthIncreasedCount = 0;

for (let i=0; i<depths.length - windowLength; i++) {
    windows[i] = [depths[i], depths[i+1], depths[i+2]] 
    currentWindowSum = _.sum(windows[i]);
    if (currentWindowSum > previousWindowSum) {
        depthIncreasedCount++
    }
    previousWindowSum = currentWindowSum;
}

console.log(depthIncreasedCount);

// process.stdout.write(windows);
// console.log(util.inspect(windows, { maxArrayLength: null }))
