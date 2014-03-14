#!/bin/bash
atest="test"
benchmark="benchmark"
function execute_normal {
   if [ -f *-tests ]; then
	    for thing in *-tests
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
}
function compile_normal {
   if [ -f *-tests.rs ]; then
        for v in *-tests.rs; do
                echo $v
                rustc $v &> /dev/null
                if [[ $? -ne 0 ]]; then
                    echo "$v failed to compile"
                fi
        done
        else
            echo "No tests to compile"
    fi
}
if [[ $1 = $atest ]]; then
    #If file exists
    cd test-files/
    compile_normal
    execute_normal
else 
    if [[ $1 = $benchmark ]]; then
        echo "Entering benchmark-files/"
        cd benchmark-files/
        #If file exists
        compile_normal
        execute_normal
        if [ -f *-tests.rs ]; then
		    echo "Compiling rust test ..."
            for v in *-tests.rs; do
                rustc --test $v &> /dev/null
                #If error of any kind
                if [[ $? -ne 0 ]]; then
                    echo "$v failed to compile.. Error: $?"
                fi
            done
            else
                echo "No tests to compile"
        fi
        if [ -f *-tests ]; then
		    echo "Running test"
		    for v in *-tests; do
                var=$(./$v --bench | grep -o "bench:[ \t]* [0-9]*" | awk '{print $2}')
                echo $var >> output-files/$v
                if [[ $? -ne 0 ]]; then
                    echo "$v program has an error"
                    exit 1
                fi
            done
            else
                echo "No test to run"
        fi
        echo "Exiting benchmark-files/"
        cd ../
		latexfigure_start='\begin{figure}\includegraphics[scale=0.5]{'
		latexfigure_end='}\end{figure}'
	
		#Outputs a .*-test-graph.png file for each test output
        if [ -f benchmark-files/output-files/*-tests ]; then
		    echo "Compiling graphs with python script..."
            for file in benchmark-files/output-files/*-tests 
                do
		            python graphs.py $file
            done
            else
                echo "No output files to graph"
        fi
		echo "Starting latex editing"
        #Copy latex-template to edit
		cp latex-template.tex graphs.tex
	
		#appends each include graphics statment to the tex file
		for graph in graph-pictures/*-graph.png 
            do
		        echo $latexfigure_start$graph$latexfigure_end>>graphs.tex
		done
	
		#Ends latex file
		echo "Ending latex file"
		echo '\end{document}'>>graphs.tex
		echo "Compiling latex..."
		pdflatex graphs.tex 1> /dev/null
		echo "Cleaning up"
		rm graphs.tex graphs.aux graphs.log
        rm benchmark-files/y_fetch1 benchmark-files/y_fetch2 2> /dev/null
		echo "Opening pdf for your pleasure!"
		evince graphs.pdf
	fi
fi
