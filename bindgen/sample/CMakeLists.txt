cmake_minimum_required(VERSION 3.10)

file(GLOB SAMPLE_SOURCES src/*.cpp)
# ライブラリ名
add_library(sample SHARED ${SAMPLE_SOURCES})

# インクルードディレクトリを追加（ヘッダファイルがある場合）
target_include_directories(sample PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/inc)


