#/bin/bash
cd benchmark-files/
#Compiling
echo "Compiling..."
for file in *
do
    [ -f "$file" ] || continue
    case "$file" in
        *.rs)
            rustc $file
        ;;
        *)
            #Not to be compiled
        ;;
    esac
done
#Running
echo "Running tests..."
for file in *
do
    [ -f "$file" ] || continue
    case "$file" in
        *.rs)
            #Not to be run
        ;;
        *)
            ./$file
        ;;
    esac
done
#Plotting
echo "Plotting..."
cd output-files/
for file in *
do
    [ -f "$file" ] || continue
    case "$file" in
        *.sh)
            ./$file
        ;;
        *)
            #Not to be plotted
        ;;
    esac
done

