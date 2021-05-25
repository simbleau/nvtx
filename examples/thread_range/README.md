# Using thread ranges

Thread ranges are used to describe events over a time span during the execution of a respective application.

They are visible in the events of the NVTX trace in programs such as NVIDIA NSight Systems.

![Screenshot from 2021-05-24 19-55-37](https://user-images.githubusercontent.com/48108917/119422353-e834ee80-bcce-11eb-9b4b-005e53f72ed2.png)

# Running the example (NVIDIA NSight Systems)

 1. Open NVIDIA NSight Systems
 2. Start a new project
 3. Select a target host for profiling (local computer is fine)
 4. Set the target application information to:
    * Command line with arguments: `cargo run --example thread_range`
    * Set the working directory to the nvtx-rs directory path
 5. Collect NVTX trace
 6. Start the project (wait for completion)
 7. Open the report
 8. Right click the NVTX trace layer and "Show in Events View"
