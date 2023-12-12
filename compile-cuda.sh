# Compute version 8.6
nvcc plugins/cuda/pyrin-cuda-native/src/pyrin-cuda.cu -std=c++17 -O3 --restrict --ptx --gpu-architecture=compute_86 --gpu-code=sm_86 -o plugins/cuda/resources/pyrin-cuda-sm86.ptx -Xptxas -O3 -Xcompiler -O3
# Compute version 7.5
nvcc plugins/cuda/pyrin-cuda-native/src/pyrin-cuda.cu -std=c++17 -O3 --restrict --ptx --gpu-architecture=compute_75 --gpu-code=sm_75 -o plugins/cuda/resources/pyrin-cuda-sm75.ptx -Xptxas -O3 -Xcompiler -O3
# Compute version 6.1
nvcc plugins/cuda/pyrin-cuda-native/src/pyrin-cuda.cu -std=c++17 -O3 --restrict --ptx --gpu-architecture=compute_61 --gpu-code=sm_61 -o plugins/cuda/resources/pyrin-cuda-sm61.ptx -Xptxas -O3 -Xcompiler -O3


#nvcc plugins/cuda/pyrin-cuda-native/src/pyrin-cuda.cu -std=c++17 -rdc=true -O3 --restrict --ptx --gpu-architecture=compute_86 --gpu-code=sm_86 -o plugins/cuda/resources/pyrin-cuda-sm86.ptx -Xptxas -O3 -Xcompiler -O3
## Compute version 7.5
#nvcc plugins/cuda/pyrin-cuda-native/src/pyrin-cuda.cu -std=c++17 -rdc=true -O3 --restrict --ptx --gpu-architecture=compute_75 --gpu-code=sm_75 -o plugins/cuda/resources/pyrin-cuda-sm75.ptx -Xptxas -O3 -Xcompiler -O3
## Compute version 6.1
#nvcc plugins/cuda/pyrin-cuda-native/src/pyrin-cuda.cu -std=c++17 -rdc=true -O3 --restrict --ptx --gpu-architecture=compute_61 --gpu-code=sm_61 -o plugins/cuda/resources/pyrin-cuda-sm61.ptx -Xptxas -O3 -Xcompiler -O3


# Compute version 3.0
#/usr/local/cuda-9.2/bin/nvcc plugins/cuda/pyrin-cuda-native/src/pyrin-cuda.cu -ccbin=gcc-7 -std=c++11 -O3 --restrict --ptx --gpu-architecture=compute_30 --gpu-code=sm_30 -o plugins/cuda/resources/pyrin-cuda-sm30.ptx
# Compute version 2.0
#/usr/local/cuda-8.0/bin/nvcc plugins/cuda/pyrin-cuda-native/src/pyrin-cuda.cu -ccbin=gcc-5 -std=c++11 -O3 --restrict --ptx --gpu-architecture=compute_20 --gpu-code=sm_20 -o plugins/cuda/resources/pyrin-cuda-sm20.ptx

