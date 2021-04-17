# Nuke all existing build folders before running the build. 

# This was created because added functions in neutron-star-rt doesn't always link correctly unless a hard rebuild is made here

for dir in */ ; do
    cd ./$dir
    for subdir in */ ; do
        cd ./$subdir
        echo "removing target directory in $dir$subdir"
        rm -r ./target
        cd ../
    done
    cd ../
done

./build.sh
