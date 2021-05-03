# Build all smart contract cargo projects located in subrepos under the contracts folder

cd ./contracts
for dir in */ ; do
    cd ./$dir
    echo "Building contracts in /contracts/$dir..."
    cargo build --target thumbv6m-none-eabi
    cargo build --target thumbv6m-none-eabi --release
    cd ../
done
cd ../
