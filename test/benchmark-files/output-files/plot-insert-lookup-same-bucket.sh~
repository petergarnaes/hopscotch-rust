#/bin/bash
/usr/bin/gnuplot <<\__EOF
set term png
set output "plot-insert-same-bucket.png"
set title "Insert into same bucket benchmark"
set xlabel "Nr. of inserts"
set ylabel "Time (ns)"
plot "bench-insert-same-bucket" u 1:2 t 'hopscotch' w linespoints, "bench-insert-same-bucket" u 1:3 w linespoints t 'robin hood'
__EOF
