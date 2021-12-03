const https = require('https');

module.exports = (id) => {
    // each line is a measurement of the sea floor depth as the sweep looks further and further away from the submarine.
    // The first order of business is to figure out how quickly the depth increases
    
    // TODO - count the number of times a depth measurement increases from the previous measurement
    
    const options = {
        hostname: 'adventofcode.com',
        path: `/2021/day/${id}/input`,
        method: 'GET',
        headers: {
            cookie: '_ga=GA1.2.115346750.1638380518; _gid=GA1.2.2114843456.1638380518; ru=53616c7465645f5f059cf41ab1176b2530ef06411ee5452d12358948fc91b1a4dce8dd50dd008b507ea4a65e1514637b; session=53616c7465645f5f983d89798ee666d9149e833e99589039ec91110c4edd01d4a04e2343a64a412d92387e71f3deb861',
        }
    };
    
    const req = https.get(options, res => {
        
        let rawData = ''; 
        res.on('data', chunk => {
            if (chunk) rawData += chunk;
        })
        
        res.on('end', () => {
            const depths = rawData.split('\n');
            console.log(depths)
            
            let depthIncreasedCount = 0;
            let previousDepth;
            for (const currentDepth of depths ) {
                if (currentDepth > previousDepth) depthIncreasedCount++    
                previousDepth = currentDepth;
            } 
            
            process.stdout.write(`Depth increased ${depthIncreasedCount} times`);    
        })
    });
    
    req.on('error', error => {
        console.error(error)
    })
    
    req.end();
}
