#/bin/bash
/usr/bin/gnuplot <<\__EOF
set term png
set output "plot-90lookup-5insert-5remove.png"
set title "90% lookup 5% insert 5% remove benchmark"
set xlabel "Load factor"
set ylabel "ops / ms"
set xtics 0.25,.05,0.85
set xrange [0.25:0.85]
set mxtics 1
set mytics 4
set grid mxtics mytics
plot "bench-90lookup-5insert-5remove" u 1:2 t 'hopscotch' w linespoints lw 2, "bench-90lookup-5insert-5remove" u 1:3 w linespoints t 'robin hood' lw 2
__EOF
