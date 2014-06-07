#/bin/bash
/usr/bin/gnuplot <<\__EOF
set term png
set output "plot-average-lookup.png"
set title "Average lookup time"
set xlabel "Nr. of lookups"
set ylabel "time in ns"
set mxtics 1
set mytics 5
set grid mxtics mytics
plot "bench-lookups" u 1:2 t 'hopscotch' w linespoints lw 2, "bench-lookups" u 1:3 w linespoints t 'robin hood' lw 2, "thorup_data/lookupdata" u 1:2 w linespoints t 'hopscotch C++' lw 2
__EOF
