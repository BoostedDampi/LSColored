#!/bin/fish
set total 0

for i in (seq 1 1000)
	set start_time (date +%s%3N)
	#lsc /
	./lsc /
	set end_time (date +%s%3N)
	set duration (math $end_time - $start_time)
	set total (math $total + $duration)
end

set avarage (math $total / 1000)

echo $avarage
