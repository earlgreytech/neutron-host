# Nuke all existing build folders before running the build script. 

# This script was created because added functions in neutron-star-rt doesn't always link correctly unless a hard rebuild is made

cd ./contracts
for dir in */ ; do
    cd ./$dir
    echo "Removing previously compiled binaries (target directory) in /contracts/$dir..."
    rm -r ./target
    cd ../
done
cd ../

./build.sh
