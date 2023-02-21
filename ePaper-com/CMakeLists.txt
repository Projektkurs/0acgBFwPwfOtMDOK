# Specify the minimum version.
set(LIBPATH "IT8951-ePaper/Raspberry/lib")
cmake_minimum_required(VERSION 3.9)
# Specify the project info.
project(IT8951 VERSION 1.0.0 DESCRIPTION "IT8951-ePaper")
# Declare the library target.
add_library(${PROJECT_NAME} STATIC
    ${LIBPATH}/e-Paper/EPD_IT8951.c
    ${LIBPATH}/e-Paper/EPD_IT8951.h
    ${LIBPATH}/Config/DEV_Config.c
    ${LIBPATH}/Config/DEV_Config.h
    ${LIBPATH}/Config/Debug.h
    ${SOURCES}
    #lib/
)
target_link_libraries(${PROJECT_NAME} bcm2835)
include_directories(${LIBPATH})
# Configure the directories to search for header files.
target_include_directories(${PROJECT_NAME} PRIVATE source)

set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE CXX)

# Set the version property.
set_target_properties(${PROJECT_NAME} PROPERTIES VERSION ${PROJECT_VERSION})
# Set the shared object version property to the project's major version.
set_target_properties(${PROJECT_NAME} PROPERTIES SOVERSION ${PROJECT_VERSION_MAJOR})
# Set the public header property to the one with the actual API.
set_target_properties(${PROJECT_NAME} PROPERTIES PUBLIC_HEADER ${LIBPATH}/e-Paper/EPD_IT8951.h)

install()
