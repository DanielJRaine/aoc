const fs = require('fs');

const depths = fs.readFileSync('./depths.txt')

let depthIncreasedCount = 0;
let previousDepth;
for (const currentDepth of depths) {
    console.log({currentDepth});
    if (parseInt(currentDepth) > previousDepth) depthIncreasedCount++    
    previousDepth = currentDepth;
    console.log({previousDepth});
}
