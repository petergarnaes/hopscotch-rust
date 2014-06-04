#/bin/bash
/usr/bin/gnuplot <<\__EOF
set term png
set output "plot-insert-lookup-same-bucket.png"
set title "Insert and lookup into same bucket benchmark"
set xlabel "Nr. of operations"
set ylabel "Time (ns)"
plot "bench-insert-lookup-same-bucket" u 1:2 t 'hopscotch insert' w linespoints, "bench-insert-lookup-same-bucket" u 1:3 w linespoints t 'robin hood insert', "bench-insert-lookup-same-bucket" u 1:4 t 'hopscotch lookup' w linespoints, "bench-insert-lookup-same-bucket" u 1:5 t 'robin hood lookup' w linespoints
__EOF
