if [ -z "$1" ]; then 
    target_project="hello_rust"
else 
    target_project=$1
fi

if [ -z "$2" ]; then
    target_file="input.txt"
else
    target_file=$2
fi

docker build -t advent-of-code:${target_project} --build-arg TARGET_PROJECT=${target_project} . && \
docker run --rm advent-of-code:${target_project} ./input/${target_file}
