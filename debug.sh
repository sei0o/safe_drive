CARGO_TARGET_DIR=target_docker LD_LIBRARY_PATH=supplements/ros2/install/example_msg/lib:$LD_LIBRARY_PATH SAFE_DRIVE_TEST=1 rust-gdb -x test.gdb -q $@
