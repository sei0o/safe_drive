export LD_LIBRARY_PATH=/safe_drive/supplements/ros2/install/example_msg/lib:$LD_LIBRARY_PATH
export SAFE_DRIVE_TEST=1
export CARGO_TARGET_DIR=target_docker
export RCUTILS_LOGGING_USE_STDOUT=1
cargo test $* --features humble -- --nocapture --exact 


