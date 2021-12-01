console.log(require('fs').readFileSync("./input.txt").toString().split("\n").reduce((acc, c, i, arr)=>(parseInt(c) < parseInt(arr[i+1]))?acc+1:acc,0))
