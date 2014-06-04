#/bin/bash
/usr/bin/gnuplot <<\__EOF
set term png
set output "plot-60lookup-20insert-20remove.png"
set title "60% lookup 20% insert 20% remove benchmark"
set xlabel "Load factor"
set ylabel "Time (ns)"
plot "bench-60lookup-20insert-20remove" u 1:2 t 'hopscotch' w linespoints, "bench-60lookup-20insert-20remove" u 1:3 w linespoints t 'robin hood'
__EOF
