#!/bin/bash

#Compiling test files
echo "Compiling rust test ..."
rustc rust-test.rs

latexfigure_start = '\begin{figure}\includegraphics[scale=1]{'
latexfigure_end = '}\end{figure}'

#Outputs a .*-test-graph.png file for each test output
echo "Compiling graphs with python script..."
python graph.py .*-test-output

#Deletes last latex line containing \end{document}
echo "Starting latex editing"
sed '$d' graphs.tex

#appends each include graphics statment to the tex file
for graph in .*-test-graph.png do
    echo "$latexfigure_start$graph$latexfigure_end">>graphs.tex
done

#Ends latex file
echo '\end{document}'>>graphs.tex
echo "Latex finished..."
echo "Compiling latex..."
pdflatex graphs.tex
echo "Mobing pdf out of test dir..."
mv graphs.pdf ../graphs.pdf
echo "Moving out of test dir..."
cd ../
echo "Opening pdf for your pleasure!"
evince graphs.pdf
