cmake_minimum_required(VERSION 3.10)
project(MyProject)
set(SUBDIRS 
  sample 
)


foreach(SUBDIR ${SUBDIRS})
    if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/${SUBDIR}/CMakeLists.txt)
        message(STATUS "Adding subdirectory: ${SUBDIR}")
        add_subdirectory(${SUBDIR})
    else()
        message(WARNING "Skipping ${SUBDIR}, CMakeLists.txt not found.")
    endif()
endforeach()


add_custom_target(run_script
    COMMAND ./run_script.sh
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Running build..."
)
