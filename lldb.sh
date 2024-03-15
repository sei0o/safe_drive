LD_LIBRARY_PATH=supplements/ros2/install/example_msg/lib:$LD_LIBRARY_PATH SAFE_DRIVE_TEST=1 rust-lldb --local-lldbinit $1 -- r $2 --exact --nocapture

