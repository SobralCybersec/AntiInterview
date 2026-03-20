cmake_minimum_required(VERSION 3.15)
project(hook)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Detours
find_path(DETOURS_INCLUDE_DIRS "detours/detours.h")
find_library(DETOURS_LIBRARY detours REQUIRED)

# Function to create hook DLL
function(add_hook_dll TARGET_NAME SOURCE_FILE OUTPUT_NAME)
    add_library(${TARGET_NAME} SHARED ${SOURCE_FILE} pch.cpp)
    target_precompile_headers(${TARGET_NAME} PRIVATE pch.h)
    target_include_directories(${TARGET_NAME} PRIVATE ${DETOURS_INCLUDE_DIRS})
    target_link_libraries(${TARGET_NAME} PRIVATE ${DETOURS_LIBRARY} ntdll shlwapi)
    set_target_properties(${TARGET_NAME} PROPERTIES 
        OUTPUT_NAME "${OUTPUT_NAME}"
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}"
        LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}"
    )
endfunction()

# Build all hook DLLs
add_hook_dll(hook_notepad hook_notepad.cpp "hook_notepad")
add_hook_dll(hook_firefox hook_firefox.cpp "hook_firefox")
add_hook_dll(hook_edge hook_edge.cpp "hook_edge")
add_hook_dll(hook_chrome hook_chrome.cpp "hook_chrome")
add_hook_dll(hook_vscode hook_vscode.cpp "hook_vscode")
add_hook_dll(hook_visualstudio hook_visualstudio.cpp "hook_visualstudio")
add_hook_dll(hook_antiinterview hook_antiinterview.cpp "hook_antiinterview")
