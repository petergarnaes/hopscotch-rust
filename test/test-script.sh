#!/bin/bash
atest="test"
benchmark="benchmark"
if [[ $1 = $atest ]]; then
    #If file exists
    if [ -f test-files/*-tests.rs ]; then
        for v in test-files/*-tests.rs; do
                echo $v
                rustc $v
                if [[ $? -ne 0 ]]; then
                    echo "$v failed to compile"
                fi
                mv $v "${$v}.ignore"
        done
        else
            echo "No tests to compile"
    fi
    if [ -f test-files/*-tests ]; then
	    for thing in test-files/*-tests
            do
		        ./$thing
		        if [[ $? -ne 0 ]]; then
			        echo "$thing failed to complete"
			        exit 1
		        fi
	    done
        else
            echo "No tests to run"
    fi
else 
    if [[ $1 = $benchmark ]]; then
        #If file exists
        if [ -f benchmark-files/*-tests.rs ]; then
		    echo "Compiling rust test ..."
            for v in benchmark-files/*-tests.rs; do
                rustc $v
                #If error of any kind
                if [[ $? -ne 0 ]]; then
                    echo "$v failed to compile.. Error: $?"
                fi
                mv $v "${v}.ignore"
            done
            else
                echo "No tests to compile"
        fi

        if [ -f benchmark-files/*-tests ]; then
		    echo "Running test"
		    for v in benchmark-files/*-tests; do
                ./$v
                if [[ $? -ne 0 ]]; then
                    echo "$v program has an error"
                    exit 1
                fi
            done
            else
                echo "No test to run"
        fi

		latexfigure_start='\begin{figure}\includegraphics[scale=0.5]{'
		latexfigure_end='}\end{figure}'
	
		#Outputs a .*-test-graph.png file for each test output
        if [ -f benchmark-files/output-files/*-output ]; then
		    echo "Compiling graphs with python script..."
            for file in benchmark-files/output-files/*-output 
                do
		            python graphs.py $file
            done
            else
                echo "No output files to graph"
        fi
		#Deletes last latex line containing \end{document}
		echo "Starting latex editing"
		cp latex-template.tex graphs.tex
        #Doesn't work so just removed the line manually
		sed '$d' < graphs.tex 1> /dev/null
	
		#appends each include graphics statment to the tex file
		for graph in graph-pictures/*-graph.png 
            do
                echo "Do we have a matchDo we have a match??"
		        echo $latexfigure_start$graph$latexfigure_end>>graphs.tex
		done
	
		#Ends latex file
		echo "Ending latex file"
		echo '\end{document}'>>graphs.tex
		echo "Compiling latex..."
		pdflatex graphs.tex 1> /dev/null
		echo "Cleaning up"
		rm graphs.tex graphs.aux graphs.log
		echo "Opening pdf for your pleasure!"
		evince graphs.pdf
	fi
fi
