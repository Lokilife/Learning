mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Debug -A x64 -DCMAKE_TOOLCHAIN_FILE=$VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake
cmake --build . --config Debug
