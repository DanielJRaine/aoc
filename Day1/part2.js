const fs = require('fs');

let depths = fs.readFileSync('./depths.txt', {
    encoding: "utf8",
    flag: 'r'
})

depths = depths.split('\n').map(Number)

let depthIncreasedCount = 0;
let previousDepth;

// todo: 
