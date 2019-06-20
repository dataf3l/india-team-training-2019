f2 () {
	echo f2 says $1
}

f1 () {
	echo f1
	f2 9
}
