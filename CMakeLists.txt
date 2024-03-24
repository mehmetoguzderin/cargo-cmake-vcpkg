cmake_minimum_required(VERSION 3.27)
set(CMAKE_TOOLCHAIN_FILE
    "${CMAKE_CURRENT_SOURCE_DIR}/vcpkg/scripts/buildsystems/vcpkg.cmake")
project(ProjectCxx)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# set(CMAKE_CXX_CLANG_TIDY clang-tidy)

find_package(TIFF REQUIRED)

add_library(LibCxx STATIC
    lib.cpp
)

add_library(LibSharedCxx SHARED
    lib-shared.cpp
)

target_link_libraries(LibSharedCxx PRIVATE
    TIFF::TIFF
)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG 366066c9caed822a4cbfc2fafbf7d0116d59b832
)

FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

corrosion_link_libraries(MainRs LibCxx)

# corrosion_link_libraries(MainRs LibSharedCxx)
