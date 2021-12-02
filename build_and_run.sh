if [ -z "$1" ]; then 
    target_project="hello_rust"
else 
    target_project=$1
fi

docker build -t advent-of-code:${target_project} --build-arg TARGET_PROJECT=${target_project} . && \
docker run --rm advent-of-code:${target_project}
