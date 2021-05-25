#  Hello, World!

This is a simple example which uses an NVTX "thread range" to time an expensive operation (sleeping 1 second). The thread range is labelled with "Hello, World!".

![Screenshot from 2021-05-24 19-54-07](https://user-images.githubusercontent.com/48108917/119422236-a310bc80-bcce-11eb-960a-ea6e4f681dd8.png)

# Running the example (NVIDIA NSight Systems)

 1. Open NVIDIA NSight Systems
 2. Start a new project
 3. Select a target host for profiling (local computer is fine)
 4. Set the target application information to:
    * Command line with arguments: `cargo run --example hello_world`
    * Set the working directory to the nvtx-rs directory path
 5. Collect NVTX trace
 6. Start the project (wait for completion)
 7. Open the report
 8. Right click the NVTX trace layer and "Show in Events View"
