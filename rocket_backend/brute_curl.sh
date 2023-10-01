for run in {1..400}
do
	for childrun in {1..400}
	do
		curl -sL localhost:8080/link/create &
	done	
done

