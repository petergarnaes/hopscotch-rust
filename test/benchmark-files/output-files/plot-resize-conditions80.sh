#/bin/bash
/usr/bin/gnuplot <<\__EOF
set term png
set output "plot-resize-conditions80.png"
set title "Resize % with different add ranges and table sizes"
set xlabel "Add range"
set ylabel "Resize %"
set style data histogram gap 1
set style fill solid border -1
set boxwidth 0.9
plot 'bench-resize-conditions80' using 2:xtic(1) title 'size 4096', 'bench-resize-conditions80' using 3:xtic(1) title 'size 8192', 'bench-resize-conditions80' using 4:xtic(1) title 'size 16384', 'bench-resize-conditions80' using 5:xtic(1) title 'size 32768'
