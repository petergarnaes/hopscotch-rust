#/bin/bash
/usr/bin/gnuplot <<\__EOF
set term png
set output "plot-90lookup-5insert-5remove.png"
set title "90% lookup 5% insert 5% remove benchmark"
set xlabel "Load factor"
set ylabel "Time (ns)"
plot "bench-90lookup-5insert-5remove" u 1:2 t 'hopscotch' w linespoints, "bench-90lookup-5insert-5remove" u 1:3 w linespoints t 'robin hood'
__EOF
