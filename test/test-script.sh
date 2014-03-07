#!/bin/bash
atest="test"
benchmark="benchmark"
if [[ $1 = $atest ]];
    then
    for v in test-files/*-test.rs; do
            echo $v
            rustc $v
    done
	for thing in test-files/*-test 
        do
		    ./$thing
		    if [[ $? -ne 0 ]]; then
			    echo "$thing failed to complete"
			    exit 1
		    fi
	done
else 
    if [[ $1 = $benchmark ]]; then
		#Compiling test files
		echo "Compiling rust test ..."
        for v in benchmark-files/*-test.rs; do
            rustc $v
        done

		echo "Running test"
		for v in benchmark-files/*-test; do
            ./$v
            if [[ $? -ne 0 ]]; then
                echo "$v program has an error"
                exit 1
            fi
        done

		latexfigure_start = '\begin{figure}\includegraphics[scale=1]{graph-pictures/'
		latexfigure_end = '}\end{figure}'
	
		#Outputs a .*-test-graph.png file for each test output
		echo "Compiling graphs with python script..."
        for file in benchmark-files/output-files/*-output 
            do
		        python graphs.py $file
        done
		#Deletes last latex line containing \end{document}
		echo "Starting latex editing"
		cp latex-template.tex graphs.tex
		sed '$d' < graphs.tex
	
		#appends each include graphics statment to the tex file
		for graph in graph-pictures/*-graph.png 
            do
		        echo $latexfigure_start$graph$latexfigure_end>>graphs.tex
		done
	
		#Ends latex file
		echo "Ending latex file"
		echo "\end{document}">>graphs.tex
		echo "Compiling latex..."
		pdflatex graphs.tex
		echo "Cleaning up"
		#rm graphs.tex
		echo "Opening pdf for your pleasure!"
		evince graphs.pdf
	fi
fi
