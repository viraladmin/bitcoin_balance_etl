# Install script for directory: /home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/libleveldb.a")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib" TYPE STATIC_LIBRARY FILES "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/build/libleveldb.a")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/leveldb" TYPE FILE FILES
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/c.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/cache.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/comparator.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/db.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/dumpfile.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/env.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/export.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/filter_policy.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/iterator.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/options.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/slice.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/status.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/table_builder.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/table.h"
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/include/leveldb/write_batch.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb/leveldbTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb/leveldbTargets.cmake"
         "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/build/CMakeFiles/Export/b8f7c4f5ff3a4e54f173a153f249a621/leveldbTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb/leveldbTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb/leveldbTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb/leveldbTargets.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb" TYPE FILE FILES "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/build/CMakeFiles/Export/b8f7c4f5ff3a4e54f173a153f249a621/leveldbTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb/leveldbTargets-release.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    file(INSTALL DESTINATION "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb" TYPE FILE FILES "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/build/CMakeFiles/Export/b8f7c4f5ff3a4e54f173a153f249a621/leveldbTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb/leveldbConfig.cmake;/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb/leveldbConfigVersion.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/lib/cmake/leveldb" TYPE FILE FILES
    "/home/viraladmin/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leveldb-sys-2.0.9/deps/leveldb-1.22/cmake/leveldbConfig.cmake"
    "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/build/leveldbConfigVersion.cmake"
    )
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/home/viraladmin/bitcoin/custom_tools/target/release/build/leveldb-sys-811598238a6aaa23/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
