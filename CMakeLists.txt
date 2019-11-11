cmake_minimum_required(VERSION 3.12)
project(xpsimport)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_CMD cargo build)
  set(TARGET_DIR "debug")
else ()
  set(CARGO_CMD cargo build --release)
  set(TARGET_DIR "release")
endif ()

if(WIN32)
  set(XPSIMPORT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/xpsimport.lib")
elseif (APPLE) 
  set(XPSIMPORT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libxpsimport.a")
else() 
  set(XPSIMPORT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libxpsimport.so")
endif()

add_custom_target(xpsimport ALL
  COMMENT "Compiling xpsimport"
  COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
  COMMAND ${CMAKE_COMMAND} -E copy ${XPSIMPORT_SO} ${CMAKE_CURRENT_BINARY_DIR}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

set_target_properties(xpsimport PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})
set_target_properties(xpsimport PROPERTIES SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})

function (target_link_xpsimport_library t)
  get_target_property(XPS_DIR xpsimport LOCATION)
  get_target_property(XPS_SRC xpsimport SOURCE_DIR)
  if (WIN32)
    target_link_libraries(${t} PUBLIC ${XPS_DIR}/xpsimport.lib WS2_32 userenv)
  elseif(APPLE)
    target_link_libraries(${t} PUBLIC ${XPS_DIR}/libxpsimport.a)
  elseif(LINUX)
    target_link_libraries(${t} PUBLIC ${XPS_DIR}/libxpsimport.so)
  endif()
  target_include_directories(${t} PUBLIC ${XPS_SRC}/include)
  add_dependencies(${t} xpsimport)
endfunction()
