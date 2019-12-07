<?php
// runtime: missing file -> Warning
// missing error check
// 	-> runtime error
// error message is values are ignored.

// data is invalid -> ignored, Warning
// 	-> runtime error

//undefined * is OK
//// unhandled swtich case

function x(){
	echo("X was called");
}

if(rand()%2000000 > 1000000){
	never_happens();
}else{
	x();

}
for($x=0;$x<1000;$x++){
	if(!file_exists("input.txt")){ die("invalid file");}
	$a = file("input.txt");
	$sum = 0;
	foreach($a as $v){
		$sum += $v;
	}
	echo($sum);
}
