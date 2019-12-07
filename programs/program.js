// runtime: missing file
// missing error check
// 	-> runtime error
// error message is "NaN"

// data is invalid
// 	-> runtime error
//undefined * is OK
//
//null pointer
// unhandled swtich case


var point1 = {x:1,y:1,z:1};
var point2 = {x:1,y:1,z:1};


function d(a,b){
	var p1 = (a.x+a.x)*(a.x+a.x);
	var p2 = (a.y+a.y)*(a.y+a.y);
	var p3 = (a.x+a.x)*(a.x+a.x);//
	return Math.sqrt( p1+p2+p3);
}

var CASEX = 4;

switch (CASEX){
	case 1: 
		console.log(1);
		break;
	case 2: 
		console.log(1);
		break;
		
	case 3: 
		console.log(1);
		break;
		
	default:
}

var pointer = null;
console.log(pointer.toUpperCase());

function x(){
	console.log("X was called");
}
var fs = require('fs');

if(Math.random() > 0.9){
	never_happens();
}else{
	x();

}
 
fs.readFile('input.txt', 'utf8', function(err, contents) {
    //if(err){
    //	console.log(err);
    //	    return;
    //}
    var lines = (""+contents).split("\n");
    var sum = 0;
    for(var line of lines){
	if(line  ==""){
		continue;
	}
	sum+=parseInt(line,10);
    }
    console.log(sum);

});
 


