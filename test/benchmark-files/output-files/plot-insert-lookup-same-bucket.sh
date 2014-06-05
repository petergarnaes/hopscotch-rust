#/bin/bash
/usr/bin/gnuplot <<\__EOF
set term png
set output "plot-insert-lookup-same-bucket.png"
set title "Insert and lookup into same bucket benchmark"
set xlabel "Nr. of operations"
set ylabel "Time (ns)"
set mxtics 1
set mytics 4
set grid mxtics mytics
plot "bench-insert-lookup-same-bucket" u 1:2 t 'hopscotch insert' w linespoints lw 2, "bench-insert-lookup-same-bucket" u 1:3 w linespoints t 'robin hood insert' lw 2, "bench-insert-lookup-same-bucket" u 1:4 t 'hopscotch lookup' w linespoints lw 2, "bench-insert-lookup-same-bucket" u 1:5 t 'robin hood lookup' w linespoints lw 2
__EOF
