cmake_minimum_required(VERSION 3.12)
project(xpsimport)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_CMD cargo build)
  set(TARGET_DIR "debug")
else ()
  set(CARGO_CMD cargo build --release)
  set(TARGET_DIR "release")
endif ()

set(XPSIMPORT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/xpsimport.lib")

add_custom_target(xpsimport ALL
  COMMENT "Compiling xpsimport"
  COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
  COMMAND ${CMAKE_COMMAND} -E copy ${XPSIMPORT_SO} ${CMAKE_CURRENT_BINARY_DIR}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

include_directories(include)

set_target_properties(xpsimport PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})
